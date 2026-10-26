//! Conservative status transitions. No function grants checkpoint acceptance.
use crate::{git_blob_oid, parse_exchange_v1, validate_exchange_v1, BridgeError, BridgeResult};
use serde_json::Value;

pub const ARCHIVE: &str = "loomit77/project-block-logs";
pub const TARGET: &str = "novena/status/NOVENA-CURRENT-STATE.json";
pub const REF: &str = "refs/heads/main";
pub const MAX_PAYLOAD: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultFlag {
    Pass,
    Fail,
    Unknown,
}
impl ResultFlag {
    fn read(v: &Value) -> Self {
        match v.as_str() {
            Some("PASS") => Self::Pass,
            Some("FAIL") => Self::Fail,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockResults {
    pub runner: ResultFlag,
    pub upload: ResultFlag,
    pub technical: ResultFlag,
    pub professional: ResultFlag,
}

/// Validated immutable document; construction validates syntax, schema AND policy.
#[derive(Clone, Debug)]
pub struct Status {
    document: Value,
}
#[derive(Clone, Copy, Debug)]
pub enum PreviousKind {
    Local,
    ConfirmedArchive,
}

fn strict_oid(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 40
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

impl Status {
    pub fn parse(bytes: &[u8], kind: PreviousKind) -> BridgeResult<Self> {
        if bytes.len() > MAX_PAYLOAD {
            return Err(BridgeError("PAYLOAD_TOO_LARGE"));
        }
        let document = parse_exchange_v1(bytes).map_err(|_| BridgeError("INVALID_STATUS_JSON"))?;
        Self::from_value(document, kind)
    }

    pub fn from_value(document: Value, kind: PreviousKind) -> BridgeResult<Self> {
        validate_exchange_v1(&document).map_err(|_| BridgeError("INVALID_STATUS_SCHEMA"))?;
        if serde_json::to_vec(&document)
            .map_err(|_| BridgeError("INVALID_STATUS_JSON"))?
            .len()
            > MAX_PAYLOAD
        {
            return Err(BridgeError("PAYLOAD_TOO_LARGE"));
        }
        let publication = &document["publication"];
        match kind {
            PreviousKind::Local
                if publication["state"] != "NOT_PUBLISHED" || !publication["commit"].is_null() =>
            {
                return Err(BridgeError("FALSE_PUBLICATION_CLAIM"))
            }
            PreviousKind::ConfirmedArchive
                if publication["state"] != "CONFIRMED" || !strict_oid(&publication["commit"]) =>
            {
                return Err(BridgeError("UNCONFIRMED_ARCHIVE"))
            }
            _ => (),
        }
        let checkpoint = &document["last_verified"];
        if !checkpoint.is_null() {
            if !strict_oid(&checkpoint["git_head"]) {
                return Err(BridgeError("INVALID_CHECKPOINT_OID"));
            }
            if checkpoint["evidence_files"]
                .as_array()
                .unwrap()
                .iter()
                .any(|s| s.as_str().unwrap().is_empty())
            {
                return Err(BridgeError("EMPTY_CHECKPOINT_EVIDENCE"));
            }
        }
        if document["current"]["state"] == "LAST_VERIFIED"
            && (checkpoint.is_null() || document["current"]["observed_at_utc"].is_null())
        {
            return Err(BridgeError("UNSUPPORTED_CURRENT_CLAIM"));
        }
        Ok(Self { document })
    }

    pub fn document(&self) -> &Value {
        &self.document
    }
    pub fn latest_results(&self) -> Option<BlockResults> {
        let b = &self.document["latest_block"];
        (!b.is_null()).then(|| BlockResults {
            runner: ResultFlag::read(&b["runner"]),
            upload: ResultFlag::read(&b["upload"]),
            technical: ResultFlag::read(&b["technical_result"]),
            professional: ResultFlag::read(&b["professional_acceptance"]),
        })
    }
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = serde_json::to_vec(&self.document).expect("validated integer JSON");
        bytes.push(b'\n');
        bytes
    }
}

/// Fully prepared local bytes. Does not attest to source truth or publication.
#[derive(Debug)]
pub struct PreparedStatus {
    bytes: Vec<u8>,
}
impl PreparedStatus {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn blob_oid(&self) -> String {
        git_blob_oid(&self.bytes)
    }
}

pub fn prepare(previous: &Status, candidate: &Status) -> BridgeResult<PreparedStatus> {
    // Recheck candidate's local publication policy even if constructed as remote.
    Status::from_value(candidate.document.clone(), PreviousKind::Local)?;
    if previous.document["last_verified"] != candidate.document["last_verified"] {
        return Err(BridgeError("CHECKPOINT_CHANGE_FORBIDDEN"));
    }
    // A historical checkpoint cannot establish a newly observed live state.
    if candidate.document["current"]["state"] == "LAST_VERIFIED"
        && previous.document["current"] != candidate.document["current"]
    {
        return Err(BridgeError("NEW_CURRENT_CLAIM_REQUIRES_REVIEW"));
    }
    Ok(PreparedStatus {
        bytes: candidate.canonical_bytes(),
    })
}

pub fn prepare_initial(candidate: &Status) -> BridgeResult<PreparedStatus> {
    Status::from_value(candidate.document.clone(), PreviousKind::Local)?;
    if !candidate.document["last_verified"].is_null() {
        return Err(BridgeError("INITIAL_CHECKPOINT_NOT_NULL"));
    }
    Ok(PreparedStatus {
        bytes: candidate.canonical_bytes(),
    })
}

/// Compare exact baseline bytes, independently of serialization or upload success.
pub fn compare_base(actual: Option<&[u8]>, expected: Option<&[u8]>) -> BridgeResult<()> {
    if actual != expected {
        return Err(BridgeError("STALE_BASE_CONFLICT"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn pilot() -> Value {
        parse_exchange_v1(include_bytes!(
            "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
        ))
        .unwrap()
    }
    fn local(v: Value) -> Status {
        Status::from_value(v, PreviousKind::Local).unwrap()
    }
    fn checkpoint(v: &mut Value) {
        v["last_verified"] = json!({"block_id":"BASE", "git_head":"a".repeat(40), "qualified_at_utc":"2026-10-26T00:00:00Z", "qualification":"PASS", "evidence_files":["original.txt"]});
    }
    #[test]
    fn failed_and_unqualified_blocks_never_promote() {
        for runner in ["PASS", "FAIL", "UNKNOWN"] {
            for upload in ["PASS", "FAIL", "UNKNOWN"] {
                for technical in ["PASS", "FAIL", "UNKNOWN"] {
                    for professional in ["PASS", "FAIL", "UNKNOWN"] {
                        let mut base = pilot();
                        checkpoint(&mut base);
                        let before = local(base.clone());
                        base["latest_block"] = json!({"block_id":"NEW", "runner":runner, "upload":upload, "technical_result":technical, "professional_acceptance":professional});
                        let after = local(base);
                        let prepared = prepare(&before, &after).unwrap();
                        assert_eq!(
                            parse_exchange_v1(prepared.bytes()).unwrap()["last_verified"],
                            before.document()["last_verified"]
                        );
                        let results = after.latest_results().unwrap();
                        assert_eq!(results.technical, ResultFlag::read(&json!(technical)));
                        assert_eq!(results.professional, ResultFlag::read(&json!(professional)));
                    }
                }
            }
        }
    }
    #[test]
    fn checkpoint_changes_and_false_publication_fail_closed() {
        let mut v = pilot();
        checkpoint(&mut v);
        let previous = local(v.clone());
        for replacement in [
            Value::Null,
            json!({"block_id":"OTHER", "git_head":"b".repeat(40), "qualified_at_utc":"2026-10-26T00:00:00Z", "qualification":"PASS", "evidence_files":["new.txt"]}),
        ] {
            let mut candidate = v.clone();
            candidate["last_verified"] = replacement;
            assert_eq!(
                prepare(&previous, &local(candidate)).unwrap_err(),
                BridgeError("CHECKPOINT_CHANGE_FORBIDDEN")
            );
        }
        assert!(prepare_initial(&previous).is_err());
        v["publication"]["state"] = json!("CONFIRMED");
        assert!(Status::from_value(v, PreviousKind::Local).is_err());
        let mut v = pilot();
        v["current"]["state"] = json!("LAST_VERIFIED");
        assert!(Status::from_value(v, PreviousKind::Local).is_err());
        let mut v = pilot();
        checkpoint(&mut v);
        v["last_verified"]["git_head"] = json!(format!("{}\n", "a".repeat(40)));
        assert!(validate_exchange_v1(&v).is_ok());
        assert!(Status::from_value(v, PreviousKind::Local).is_err());
    }
    #[test]
    fn initial_pilot_and_exact_baseline_conflicts() {
        let status = local(pilot());
        assert_eq!(
            prepare_initial(&status).unwrap().bytes(),
            status.canonical_bytes()
        );
        assert!(compare_base(None, None).is_ok());
        assert!(compare_base(Some(b"x"), Some(b"x")).is_ok());
        assert!(compare_base(None, Some(b"")).is_err());
        assert!(compare_base(Some(b"x\n"), Some(b"x")).is_err());
        let p = prepare(&status, &status).unwrap();
        assert_eq!(p.bytes(), status.canonical_bytes());
        let mut oversized = pilot();
        oversized["latest_block"]["block_id"] = json!("x".repeat(MAX_PAYLOAD));
        assert!(validate_exchange_v1(&oversized).is_ok());
        assert_eq!(
            Status::from_value(oversized, PreviousKind::Local).unwrap_err(),
            BridgeError("PAYLOAD_TOO_LARGE")
        );
    }
}
