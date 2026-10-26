//! Deterministic in-memory initial Git object preparation; never writes objects.
use crate::objects::*;
use crate::protocol::{
    check_commit, decode_base64, encode_base64, update_refs_request, RestReader,
};
use crate::status::{self, PreviousKind, Status, REF, TARGET};
use crate::{parse_exchange_v1, BridgeError, BridgeResult};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PlannedObject {
    pub kind: ObjectKind,
    pub oid: Oid,
    pub bytes: Vec<u8>,
}
#[derive(Debug)]
pub struct InitialObjectPlan {
    pub before: Oid,
    pub after: Oid,
    pub tree: Oid,
    pub blob: Oid,
    pub objects: Vec<PlannedObject>,
}
impl InitialObjectPlan {
    pub fn as_json(&self, repository_id: &str) -> BridgeResult<Value> {
        let objects:Vec<_>=self.objects.iter().map(|o|json!({"kind":match o.kind {ObjectKind::Blob=>"blob",ObjectKind::Tree=>"tree",ObjectKind::Commit=>"commit"},"oid":o.oid.to_string(),"bytes_base64":encode_base64(&o.bytes)})).collect();
        Ok(
            json!({"target":TARGET,"before":self.before.to_string(),"after":self.after.to_string(),"tree":self.tree.to_string(),"blob":self.blob.to_string(),"objects":objects,"cas_request":update_refs_request(repository_id,REF,self.before,self.after)?,"write_performed":false,"bridge_publication":"BLOCKED","scope":"OFFLINE_OBJECT_PLAN"}),
        )
    }
}

fn direct(reader: &mut impl TreeReader, id: Oid) -> BridgeResult<Vec<TreeEntry>> {
    let record = reader.tree(id)?;
    if record.oid != id || record.truncated {
        return Err(BridgeError("INCOMPLETE_OR_WRONG_TREE"));
    }
    if tree_oid(&record.entries)? != id {
        return Err(BridgeError("TREE_HASH_MISMATCH"));
    }
    Ok(record.entries)
}

pub fn prepare_initial_commit(
    reader: &mut impl TreeReader,
    parent_bytes: &[u8],
    before: Oid,
    candidate: &Status,
    identity: &str,
    message: &[u8],
) -> BridgeResult<InitialObjectPlan> {
    prepare_commit(
        reader,
        parent_bytes,
        before,
        None,
        candidate,
        identity,
        message,
    )
}

/// Variant A continuation: old bytes must hash to the target in the verified baseline.
/// No publication field is rewritten and no source claim is promoted.
pub fn prepare_update_commit(
    reader: &mut impl TreeReader,
    parent_bytes: &[u8],
    before: Oid,
    baseline: &[u8],
    candidate: &Status,
    identity: &str,
    message: &[u8],
) -> BridgeResult<InitialObjectPlan> {
    prepare_commit(
        reader,
        parent_bytes,
        before,
        Some(baseline),
        candidate,
        identity,
        message,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepare_commit(
    reader: &mut impl TreeReader,
    parent_bytes: &[u8],
    before: Oid,
    baseline: Option<&[u8]>,
    candidate: &Status,
    identity: &str,
    message: &[u8],
) -> BridgeResult<InitialObjectPlan> {
    if identity.contains(['\r', '\n', '\0']) {
        return Err(BridgeError("INVALID_IDENTITY_HEADER"));
    }
    let payload = match baseline {
        Some(bytes) => status::prepare(&Status::parse(bytes, PreviousKind::Local)?, candidate)?,
        None => status::prepare_initial(candidate)?,
    };
    let parent = inspect_commit(parent_bytes)?;
    if object_oid(ObjectKind::Commit, parent_bytes) != before {
        return Err(BridgeError("PARENT_COMMIT_HASH_MISMATCH"));
    }
    let old = flatten_verified(reader, parent.tree)?;
    if let Some(bytes) = baseline {
        if old.files.get(TARGET) != Some(&(Mode::File, object_oid(ObjectKind::Blob, bytes))) {
            return Err(BridgeError("STALE_BASE_CONFLICT"));
        }
    }
    let blob = object_oid(ObjectKind::Blob, payload.bytes());
    let mut created = BTreeMap::new();
    fn insert(
        reader: &mut impl TreeReader,
        current: Option<Oid>,
        parts: &[&str],
        blob: Oid,
        created: &mut BTreeMap<Oid, Vec<TreeEntry>>,
        replacing: bool,
    ) -> BridgeResult<Oid> {
        let mut entries = if let Some(id) = current {
            direct(reader, id)?
        } else {
            Vec::new()
        };
        let index = entries.iter().position(|e| e.name == parts[0]);
        let next = if parts.len() == 1 {
            if index.is_some() && !replacing {
                return Err(BridgeError("TARGET_ALREADY_EXISTS"));
            }
            TreeEntry {
                name: parts[0].to_owned(),
                mode: Mode::File,
                oid: blob,
            }
        } else {
            let old = if let Some(i) = index {
                if entries[i].mode != Mode::Directory {
                    return Err(BridgeError("TARGET_PARENT_NOT_DIRECTORY"));
                }
                Some(entries[i].oid)
            } else {
                None
            };
            let child = insert(reader, old, &parts[1..], blob, created, replacing)?;
            TreeEntry {
                name: parts[0].to_owned(),
                mode: Mode::Directory,
                oid: child,
            }
        };
        if let Some(i) = index {
            entries[i] = next;
        } else {
            entries.push(next);
        }
        let id = tree_oid(&entries)?;
        created.insert(id, entries);
        Ok(id)
    }
    let tree = insert(
        reader,
        Some(parent.tree),
        &["novena", "status", "NOVENA-CURRENT-STATE.json"],
        blob,
        &mut created,
        baseline.is_some(),
    )?;
    struct Overlay<'a, R> {
        source: &'a mut R,
        created: &'a BTreeMap<Oid, Vec<TreeEntry>>,
    }
    impl<R: TreeReader> TreeReader for Overlay<'_, R> {
        fn tree(&mut self, id: Oid) -> BridgeResult<TreeRecord> {
            if let Some(entries) = self.created.get(&id) {
                Ok(TreeRecord {
                    oid: id,
                    truncated: false,
                    entries: entries.clone(),
                })
            } else {
                self.source.tree(id)
            }
        }
    }
    let new = flatten_verified(
        &mut Overlay {
            source: reader,
            created: &created,
        },
        tree,
    )?;
    if let Some(bytes) = baseline {
        verify_single_replacement(&old, &new, object_oid(ObjectKind::Blob, bytes), blob)?;
    } else {
        verify_single_addition(&old, &new, TARGET, blob)?;
    }
    // inspect_commit validates the exact final bytes, including header injection.
    let mut raw =
        format!("tree {tree}\nparent {before}\nauthor {identity}\ncommitter {identity}\n\n")
            .into_bytes();
    raw.extend(message);
    let after = object_oid(ObjectKind::Commit, &raw);
    verify_commit(&raw, after, Some(before), tree)?;
    let mut objects = vec![PlannedObject {
        kind: ObjectKind::Blob,
        oid: blob,
        bytes: payload.bytes().to_vec(),
    }];
    for (id, entries) in created {
        objects.push(PlannedObject {
            kind: ObjectKind::Tree,
            oid: id,
            bytes: tree_bytes(&entries)?,
        });
    }
    objects.push(PlannedObject {
        kind: ObjectKind::Commit,
        oid: after,
        bytes: raw,
    });
    Ok(InitialObjectPlan {
        before,
        after,
        tree,
        blob,
        objects,
    })
}

/// Input format documented in the crate README. Entirely local; no fallback.
pub fn plan_offline_bundle(bytes: &[u8]) -> BridgeResult<Value> {
    plan_bundle(bytes, false)
}
/// Same bundle plus baseline_base64, bound to the verified old target blob.
pub fn plan_update_bundle(bytes: &[u8]) -> BridgeResult<Value> {
    plan_bundle(bytes, true)
}
fn plan_bundle(bytes: &[u8], update: bool) -> BridgeResult<Value> {
    if bytes.len() > 32 * 1024 * 1024 {
        return Err(BridgeError("BUNDLE_TOO_LARGE"));
    }
    let v = parse_exchange_v1(bytes).map_err(|_| BridgeError("INVALID_BUNDLE_JSON"))?;
    let fields = v.as_object().ok_or(BridgeError("INVALID_BUNDLE_FIELDS"))?;
    let mut expected = vec![
        "repository_id",
        "before",
        "candidate",
        "responses",
        "raw_commits",
        "identity",
        "message",
    ];
    if update {
        expected.push("baseline_base64");
    }
    if fields.len() != expected.len() || expected.iter().any(|k| !fields.contains_key(*k)) {
        return Err(BridgeError("INVALID_BUNDLE_FIELDS"));
    }
    let text = |k: &str| v[k].as_str().ok_or(BridgeError("EXPECTED_STRING"));
    let repository_id = text("repository_id")?;
    let before = Oid::parse(text("before")?)?;
    let candidate = Status::from_value(v["candidate"].clone(), PreviousKind::Local)?;
    let responses = v["responses"]
        .as_object()
        .ok_or(BridgeError("INVALID_BUNDLE_RESPONSES"))?;
    let mut reader = RestReader::new(
        |path: &str| {
            serde_json::to_vec(responses.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                .map_err(|_| BridgeError("INVALID_RESPONSE"))
        },
        256,
    )?;
    if reader.repository()? != repository_id || repository_id.trim().is_empty() {
        return Err(BridgeError("REPOSITORY_ID_MISMATCH"));
    }
    if reader.head()? != before {
        return Err(BridgeError("STALE_BASE_CONFLICT"));
    }
    let parent = decode_base64(
        v["raw_commits"][before.to_string()]
            .as_str()
            .ok_or(BridgeError("MISSING_RAW_COMMIT"))?,
    )?;
    // Bind the pinned raw baseline to its supplied REST metadata, using the
    // same identity/link checks as independent readback. Never reconstruct it.
    check_commit(&mut reader, &mut |_: Oid| Ok(parent.clone()), before)?;
    let plan = if update {
        let baseline = decode_base64(text("baseline_base64")?)?;
        let oid = object_oid(ObjectKind::Blob, &baseline);
        if reader.blob(oid)? != baseline {
            return Err(BridgeError("PAYLOAD_MISMATCH"));
        }
        prepare_update_commit(
            &mut reader,
            &parent,
            before,
            &baseline,
            &candidate,
            text("identity")?,
            text("message")?.as_bytes(),
        )?
    } else {
        prepare_initial_commit(
            &mut reader,
            &parent,
            before,
            &candidate,
            text("identity")?,
            text("message")?.as_bytes(),
        )?
    };
    if reader.head()? != before {
        return Err(BridgeError("LATE_HEAD_CHANGE"));
    }
    plan.as_json(repository_id)
}
