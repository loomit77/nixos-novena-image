//! Variant A evidence contract, explicitly separate from Python remote=True.
//! NOT_PUBLISHED/null are payload status values, not claims about later external
//! verification of the enclosing commit. Offline inputs cannot prove authenticity,
//! freshness, publication, or professional acceptance.
use crate::objects::*;
use crate::protocol::{check_commit, RawCommitSource, ResponseSource, RestReader};
use crate::status::{PreviousKind, Status, ARCHIVE, REF, TARGET};
use crate::{BridgeError, BridgeResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// A caller-supplied observation label; never presented as trusted wall-clock time.
pub struct OfflineObservation<'a> {
    pub recorded_at_utc: &'a str,
    pub conditions: &'a str,
}

/// Constructible only after independent object/readback verification.
/// Retains the actual checked bytes; serialization never upgrades status claims.
#[derive(Debug)]
pub struct OfflineAttestation {
    repository_id: String,
    commit: Oid,
    blob: Oid,
    bytes: Vec<u8>,
    sha256: String,
    recorded_at_utc: String,
    conditions: String,
}
impl OfflineAttestation {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn as_json(&self) -> Value {
        json!({"contract":"VARIANT_A_OFFLINE_V1", "repository":ARCHIVE,
            "repository_id":self.repository_id,"ref":REF,"target":TARGET,
            "commit":self.commit.to_string(),"blob_oid":self.blob.to_string(),
            "sha256":self.sha256,"bytes_base64":crate::protocol::encode_base64(&self.bytes),
            "recorded_at_utc":self.recorded_at_utc,"conditions":self.conditions,
            "observation_metadata":"CALLER_SUPPLIED", "scope":"OFFLINE_SUPPLIED_RESPONSES",
            "head_checked_before_and_after":true,"professional_acceptance":"NOT_ATTESTED",
            "bridge_publication":"BLOCKED","publication_performed":false})
    }
}

pub fn verify_snapshot<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    repository_id: &str,
    commit: Oid,
    expected_bytes: &[u8],
    observation: OfflineObservation<'_>,
) -> BridgeResult<OfflineAttestation> {
    // Reuse the full exchange timestamp validator, including RFC3339 boundaries.
    let mut probe = Status::parse(expected_bytes, PreviousKind::Local)?
        .document()
        .clone();
    probe["generated_at_utc"] = json!(observation.recorded_at_utc);
    crate::validate_exchange_v1(&probe).map_err(|_| BridgeError("INVALID_OBSERVATION_TIME"))?;
    if observation.conditions.trim().is_empty() || observation.conditions.len() > 4096 {
        return Err(BridgeError("INVALID_OBSERVATION_CONDITIONS"));
    }
    if repository_id.trim().is_empty() || reader.repository()? != repository_id {
        return Err(BridgeError("REPOSITORY_ID_MISMATCH"));
    }
    if reader.head()? != commit {
        return Err(BridgeError("INITIAL_HEAD_MISMATCH"));
    }
    let info = check_commit(reader, raw, commit)?;
    let flat = flatten_verified(reader, info.tree)?;
    let blob = object_oid(ObjectKind::Blob, expected_bytes);
    if flat.files.get(TARGET) != Some(&(Mode::File, blob)) {
        return Err(BridgeError("PAYLOAD_MISMATCH"));
    }
    if reader.blob(blob)? != expected_bytes {
        return Err(BridgeError("PAYLOAD_MISMATCH"));
    }
    if reader.head()? != commit {
        return Err(BridgeError("FINAL_HEAD_MISMATCH"));
    }
    Ok(OfflineAttestation {
        repository_id: repository_id.into(),
        commit,
        blob,
        bytes: expected_bytes.to_vec(),
        sha256: format!("{:x}", Sha256::digest(expected_bytes)),
        recorded_at_utc: observation.recorded_at_utc.into(),
        conditions: observation.conditions.into(),
    })
}

/// Strict local bundle; no filesystem, network, mutation response, or fallback.
pub fn verify_bundle(bytes: &[u8]) -> BridgeResult<OfflineAttestation> {
    if bytes.len() > 32 * 1024 * 1024 {
        return Err(BridgeError("BUNDLE_TOO_LARGE"));
    }
    let v = crate::parse_exchange_v1(bytes).map_err(|_| BridgeError("INVALID_BUNDLE_JSON"))?;
    let fields = v.as_object().ok_or(BridgeError("INVALID_BUNDLE_FIELDS"))?;
    let expected = [
        "repository_id",
        "commit",
        "expected_bytes_base64",
        "responses",
        "raw_commits",
        "recorded_at_utc",
        "conditions",
    ];
    if fields.len() != expected.len() || expected.iter().any(|k| !fields.contains_key(*k)) {
        return Err(BridgeError("INVALID_BUNDLE_FIELDS"));
    }
    let text = |k: &str| v[k].as_str().ok_or(BridgeError("EXPECTED_STRING"));
    let responses = v["responses"]
        .as_object()
        .ok_or(BridgeError("INVALID_BUNDLE_RESPONSES"))?;
    let commits = v["raw_commits"]
        .as_object()
        .ok_or(BridgeError("INVALID_BUNDLE_COMMITS"))?;
    let mut reader = RestReader::new(
        |path: &str| {
            serde_json::to_vec(responses.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                .map_err(|_| BridgeError("INVALID_RESPONSE"))
        },
        256,
    )?;
    let mut raw = |id: Oid| {
        crate::protocol::decode_base64(
            commits
                .get(&id.to_string())
                .and_then(Value::as_str)
                .ok_or(BridgeError("MISSING_RAW_COMMIT"))?,
        )
    };
    let expected = crate::protocol::decode_base64(text("expected_bytes_base64")?)?;
    verify_snapshot(
        &mut reader,
        &mut raw,
        text("repository_id")?,
        Oid::parse(text("commit")?)?,
        &expected,
        OfflineObservation {
            recorded_at_utc: text("recorded_at_utc")?,
            conditions: text("conditions")?,
        },
    )
}
