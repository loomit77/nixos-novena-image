//! Getrennte Offline-Evidenz aus dem erfolgreichen vollständigen Readback.
//! SHA256 betrifft Nutzbytes ohne Git-Header; OIDs bleiben Git-SHA1-Identitäten.
use crate::objects::Oid;
use crate::offline_readback::{BlobEvidence, RunScope, SupervisorLimits};
use crate::{BridgeError, BridgeResult};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceKind {
    Commit,
    Tree,
    Blob,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrigin {
    OriginalCommitPayload,
    CanonicallyReconstructedTree,
    DecodedBlobPayload,
}
#[derive(Debug)]
pub struct ObjectEvidence {
    kind: EvidenceKind,
    oid: Oid,
    length: usize,
    sha256: [u8; 32],
    origin: ByteOrigin,
}
impl ObjectEvidence {
    pub fn kind(&self) -> EvidenceKind {
        self.kind
    }
    pub fn oid(&self) -> Oid {
        self.oid
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn payload_sha256(&self) -> &[u8; 32] {
        &self.sha256
    }
    pub fn byte_origin(&self) -> ByteOrigin {
        self.origin
    }
}

/// Keine öffentliche Konstruktion/Deserialisierung, keine Promotion oder Mutation.
/// Inventar nach (Commit, Tree, Blob), dann numerisch/lexikalisch nach OID.
#[derive(Debug)]
pub struct OfflineObjectEvidence {
    objects: Vec<ObjectEvidence>,
    bytes: Vec<u8>,
}
impl OfflineObjectEvidence {
    pub fn scope(&self) -> RunScope {
        RunScope::Offline
    }
    pub fn objects(&self) -> &[ObjectEvidence] {
        &self.objects
    }
    /// V1: kompaktes JSON, rekursiv lexikalische Schlüssel, abschließendes LF.
    /// Keine Uhrzeit, Zufallswerte oder behauptete Transportauthentifizierung.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn from_verified(
        repository_id: &str,
        before: Oid,
        after: Oid,
        commits: &BTreeMap<Oid, Vec<u8>>,
        trees: &BTreeMap<Oid, BlobEvidence>,
        blobs: &BTreeMap<Oid, BlobEvidence>,
    ) -> BridgeResult<Self> {
        if repository_id.len() > 1024
            || commits.len() != 2
            || trees.len() > 4096
            || blobs.len() > 256
            || commits.values().any(|b| b.len() > 1024 * 1024)
            || !commits.contains_key(&before)
            || !commits.contains_key(&after)
        {
            return Err(BridgeError("EVIDENCE_COMMIT_INVENTORY_MISMATCH"));
        }
        // Konservative Obergrenze vor den JSON-/Base64-Zwischenpuffern.
        let upper = 16384
            + repository_id.len() * 6
            + (commits.len() + trees.len() + blobs.len()) * 512
            + commits
                .values()
                .map(|b| b.len().div_ceil(3) * 4 + 128)
                .sum::<usize>();
        if upper > SupervisorLimits::EVIDENCE_BYTES {
            return Err(BridgeError("EVIDENCE_BUDGET_EXCEEDED"));
        }
        let mut objects = Vec::new();
        for (oid, bytes) in commits {
            objects.push(ObjectEvidence {
                kind: EvidenceKind::Commit,
                oid: *oid,
                length: bytes.len(),
                sha256: Sha256::digest(bytes).into(),
                origin: ByteOrigin::OriginalCommitPayload,
            });
        }
        for (kind, origin, inventory) in [
            (
                EvidenceKind::Tree,
                ByteOrigin::CanonicallyReconstructedTree,
                trees,
            ),
            (EvidenceKind::Blob, ByteOrigin::DecodedBlobPayload, blobs),
        ] {
            for (oid, item) in inventory {
                objects.push(ObjectEvidence {
                    kind,
                    oid: *oid,
                    length: item.length,
                    sha256: item.payload_sha256,
                    origin,
                });
            }
        }
        let records: Vec<_> = objects
            .iter()
            .map(|o| {
                let (kind, origin) = match o.kind {
                    EvidenceKind::Commit => ("commit", "ORIGINAL_COMMIT_PAYLOAD"),
                    EvidenceKind::Tree => ("tree", "CANONICALLY_RECONSTRUCTED_TREE"),
                    EvidenceKind::Blob => ("blob", "DECODED_BLOB_PAYLOAD"),
                };
                json!({"git_oid":o.oid.to_string(), "type":kind, "length":o.length,
                "payload_sha256":hex(&o.sha256), "byte_origin":origin})
            })
            .collect();
        let raw: Vec<_> = commits.iter().map(|(oid, bytes)| json!({
            "git_oid":oid.to_string(), "payload_base64":crate::protocol::encode_base64(bytes)
        })).collect();
        let document = json!({
            "contract":"NOVENA_OFFLINE_OBJECT_EVIDENCE_V1",
            "scope":"OFFLINE_SUPPLIED_RESPONSES",
            "repository":crate::status::ARCHIVE, "repository_node_id":repository_id,
            "reference":crate::status::REF, "baseline_commit":before.to_string(),
            "initial_observed_head":after.to_string(), "final_observed_head":after.to_string(),
            "object_count":objects.len(), "objects":records, "original_commits":raw,
            "sha256_domain":"PAYLOAD_WITHOUT_GIT_HEADER",
            "transport_provenance":"NOT_AUTHENTICATED",
            "publication":"BLOCKED", "last_verified_promotion":"BLOCKED",
            "limitations":["NO_AUTHENTICATED_GITHUB_ORIGIN", "NO_ATOMIC_REMOTE_SNAPSHOT",
                "NO_FUTURE_REF_REACHABILITY", "NO_SECURE_SSH_ACQUISITION",
                "NO_IMMUTABLE_FILESYSTEM_HANDOFF", "NO_HTTP_HEADER_MEMORY_BOUND",
                "NO_COMPLETE_PREEMPTIVE_DEADLINE", "NO_ORIGINAL_TREE_TRANSPORT_BYTES",
                "NO_ANCESTOR_HISTORY_VERIFICATION", "NO_INDEPENDENT_REPOSITORY_PIN_PROVENANCE"]
        });
        let encoded =
            serde_json::to_vec(&document).map_err(|_| BridgeError("EVIDENCE_SERIALIZATION"))?;
        let bytes =
            crate::canonical_json(&encoded).map_err(|_| BridgeError("EVIDENCE_SERIALIZATION"))?;
        if bytes.len() > SupervisorLimits::EVIDENCE_BYTES {
            return Err(BridgeError("EVIDENCE_BUDGET_EXCEEDED"));
        }
        Ok(Self { objects, bytes })
    }
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
