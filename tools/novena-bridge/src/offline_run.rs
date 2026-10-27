//! Explizite Offline-Laufbindung; niemals ein authentifizierter Erwerbsbeleg.
use crate::object_evidence::EvidenceKind;
use crate::objects::Oid;
use crate::offline_readback::{
    AcquisitionManifest, CompleteReadback, ReadbackMode, ReadbackSpec, RunBudget, RunScope,
};
use crate::{BridgeError, BridgeResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CONTRACT: &str = "NOVENA_OFFLINE_RUN_V1";
const MAX_MANIFEST_BYTES: usize = 16 * 1024;

/// Vor dem Import vom Aufrufer vorgegeben. IDs sind reproduzierbare lokale
/// Bezeichnungen; ihre globale Eindeutigkeit ist Verantwortung des Aufrufers.
/// Numerischer Repositorypin bleibt eine Erwartung, kein geprüfter REST-Beleg.
pub struct OfflineRun<'a> {
    spec: ReadbackSpec<'a>,
    before: Oid,
    id: &'a str,
    package_sha256: [u8; 32],
    budget: &'a RunBudget,
}

impl<'a> OfflineRun<'a> {
    pub fn new(
        spec: ReadbackSpec<'a>,
        before: Oid,
        id: &'a str,
        package_sha256: [u8; 32],
        budget: &'a RunBudget,
    ) -> BridgeResult<Self> {
        if id.is_empty()
            || id.len() > 128
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
            || before == spec.commit()
            || matches!(spec.mode(), ReadbackMode::Transition { baseline, .. } if *baseline != before)
        {
            return Err(BridgeError("INVALID_OFFLINE_RUN"));
        }
        budget.check()?;
        Ok(Self {
            spec,
            before,
            id,
            package_sha256,
            budget,
        })
    }

    /// Vorlage aus bereits lokal geprüften Bytes. Auch diese Vorlage ist beim
    /// späteren Import nur eine Behauptung und wird vollständig erneut geprüft.
    pub fn manifest_bytes(&self, complete: &CompleteReadback) -> BridgeResult<Vec<u8>> {
        let package = complete.package().bytes();
        if <[u8; 32]>::from(Sha256::digest(package)) != self.package_sha256 {
            return Err(BridgeError("RUN_PACKAGE_DIGEST"));
        }
        let p = crate::parse_exchange_v1(package).map_err(|_| BridgeError("RUN_PACKAGE"))?;
        let (_, old, new) = match self.spec.mode() {
            ReadbackMode::Snapshot { expected } => ("initial", None, *expected),
            ReadbackMode::Transition { old, new, .. } => ("update", Some(*old), *new),
        };
        if p["repository_id"] != self.spec.pins().0
            || p["before"] != self.before.to_string()
            || p["after"] != self.spec.commit().to_string()
            || p["expected_base64"] != crate::protocol::encode_base64(new)
            || p["baseline_base64"]
                != old
                    .map(crate::protocol::encode_base64)
                    .map_or(Value::Null, Value::String)
        {
            return Err(BridgeError("RUN_EXPECTATION_MISMATCH"));
        }
        let commits: Vec<_> = complete
            .evidence()
            .objects()
            .iter()
            .filter(|o| o.kind() == EvidenceKind::Commit)
            .map(|o| {
                json!({"git_oid":o.oid().to_string(),"length":o.length(),
                "payload_sha256":hex(o.payload_sha256())})
            })
            .collect();
        self.binding_bytes(commits)
    }

    fn binding_bytes(&self, commits: Vec<Value>) -> BridgeResult<Vec<u8>> {
        let mode = match self.spec.mode() {
            ReadbackMode::Snapshot { .. } => "initial",
            ReadbackMode::Transition { .. } => "update",
        };
        let v = json!({"contract":CONTRACT,"scope":"OFFLINE_ONLY",
            "provenance":"UNTRUSTED_ACQUISITION_CLAIM",
            "repository":self.spec.repository(),"reference":self.spec.reference(),
            "repository_node_id":self.spec.pins().0,"expected_numeric_id":self.spec.pins().1,
            "before":self.before.to_string(),"after":self.spec.commit().to_string(),
            "mode":mode,"offline_run_id":self.id,
            "observations":[{"phase":"initial","oid":self.spec.commit().to_string()},
                {"phase":"final","oid":self.spec.commit().to_string()}],
            "original_commits":commits,"package_contract":"NOVENA_OFFLINE_REPLAY_V1",
            "package_sha256":hex(&self.package_sha256),"budget_contract":"EXISTING_RUN_BUDGET_V1"});
        crate::canonical_json(&serde_json::to_vec(&v).map_err(|_| BridgeError("RUN_MANIFEST"))?)
            .map_err(|_| BridgeError("RUN_MANIFEST"))
    }

    /// Kein Teilbeleg bei Fehler. Derselbe Kontext umfasst Paketprüfung und
    /// Manifestvergleich; jeder Fehler vergiftet ihn. Kein Budgetreset im Replay.
    pub fn verify(
        &self,
        manifest: &AcquisitionManifest<'_>,
        package: &[u8],
    ) -> BridgeResult<BoundOfflineReplay> {
        self.verify_inner(manifest, package, None)
    }
    /// Explizite Dateiauswahl; Manifestoriginal als unveränderlicher Slice.
    /// Jeder Aufruf öffnet und prüft beide Originaldateien unabhängig neu.
    pub fn verify_files(
        &self,
        manifest: &AcquisitionManifest<'_>,
        package: &[u8],
        directory: &std::path::Path,
    ) -> BridgeResult<BoundOfflineReplay> {
        self.budget.check()?;
        self.budget.fail((|| {
            if manifest.original().len() > MAX_MANIFEST_BYTES {
                return Err(BridgeError("RUN_MANIFEST_LIMIT"));
            }
            if package.len() > crate::offline_package::MAX_PACKAGE_BYTES
                || <[u8; 32]>::from(Sha256::digest(package)) != self.package_sha256
            {
                return Err(BridgeError("RUN_PACKAGE_DIGEST"));
            }
            let claims = manifest
                .exports()
                .iter()
                .map(|c| {
                    json!({
                        "git_oid":c.oid.to_string(),"length":c.length,
                        "payload_sha256":hex(&c.payload_sha256)
                    })
                })
                .collect();
            if self.binding_bytes(claims)? != manifest.original() {
                return Err(BridgeError("RUN_MANIFEST_MISMATCH"));
            }
            self.budget.handoff_capacity(manifest.exports())?;
            let commits = crate::raw_commit_files::read_handoff(directory, manifest.exports())?;
            self.verify_inner(manifest, package, Some(commits))
        })())
    }
    fn verify_inner(
        &self,
        manifest: &AcquisitionManifest<'_>,
        package: &[u8],
        commits: Option<std::collections::BTreeMap<Oid, Vec<u8>>>,
    ) -> BridgeResult<BoundOfflineReplay> {
        self.budget.check()?;
        self.budget.fail((|| {
            if manifest.original().len() > MAX_MANIFEST_BYTES {
                return Err(BridgeError("RUN_MANIFEST_LIMIT"));
            }
            crate::parse_exchange_v1(manifest.original())
                .map_err(|_| BridgeError("RUN_MANIFEST_JSON"))?;
            if package.len() > crate::offline_package::MAX_PACKAGE_BYTES
                || <[u8; 32]>::from(Sha256::digest(package)) != self.package_sha256
            {
                return Err(BridgeError("RUN_PACKAGE_DIGEST"));
            }
            let complete = if let Some(commits) = commits {
                crate::offline_package::verify_handoff_in_run(package, self.budget, commits)?
            } else {
                crate::offline_package::verify_package_in_run(package, self.budget)?
            };
            let binding = self.manifest_bytes(&complete)?;
            if binding != manifest.original() {
                return Err(BridgeError("RUN_MANIFEST_MISMATCH"));
            }
            let commits: Vec<_> = complete
                .evidence()
                .objects()
                .iter()
                .filter(|o| o.kind() == EvidenceKind::Commit)
                .collect();
            if manifest.exports().len() != commits.len()
                || manifest
                    .exports()
                    .iter()
                    .zip(&commits)
                    .any(|(claim, object)| {
                        claim.oid != object.oid()
                            || claim.length != object.length()
                            || &claim.payload_sha256 != object.payload_sha256()
                    })
            {
                return Err(BridgeError("RUN_EXPORT_MISMATCH"));
            }
            Ok(BoundOfflineReplay {
                complete,
                binding,
                usage: self.budget.consumption(),
            })
        })())
    }
}

/// Nur durch vollständige Wiederprüfung konstruierbar, nicht aus JSON ladbar.
#[derive(Debug)]
pub struct BoundOfflineReplay {
    complete: CompleteReadback,
    binding: Vec<u8>,
    usage: [usize; 8],
}
impl BoundOfflineReplay {
    pub fn complete(&self) -> &CompleteReadback {
        &self.complete
    }
    pub fn scope(&self) -> RunScope {
        RunScope::Offline
    }
}

/// Beide Werte müssen durch separate verify-Aufrufe entstanden sein. Selbst
/// Gleichheit authentifiziert weder Transport noch zwei Remote-Erhebungen.
pub fn compare_replays(
    a: &BoundOfflineReplay,
    b: &BoundOfflineReplay,
) -> BridgeResult<&'static str> {
    if std::ptr::eq(a, b)
        || a.binding != b.binding
        || a.usage != b.usage
        || a.complete.package().bytes() != b.complete.package().bytes()
        || a.complete.evidence().bytes() != b.complete.evidence().bytes()
    {
        return Err(BridgeError("OFFLINE_REPLAY_CONFLICT"));
    }
    Ok("REPLAY_CONSISTENT_OFFLINE")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
