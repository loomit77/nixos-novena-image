//! Explicit local file selection. No discovery, writes or timestamp authority.
use crate::{logs, parse_exchange_v1, sources, status, BridgeError, BridgeResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Read,
    path::{Component, Path},
};

fn keys(v: &Value, expected: &[&str]) -> BridgeResult<()> {
    let o = v.as_object().ok_or(BridgeError("INVALID_SOURCE_BUNDLE"))?;
    if o.len() != expected.len() || expected.iter().any(|k| !o.contains_key(*k)) {
        return Err(BridgeError("INVALID_SOURCE_BUNDLE_FIELDS"));
    }
    Ok(())
}
fn string<'a>(v: &'a Value, key: &str) -> BridgeResult<&'a str> {
    v[key]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or(BridgeError("INVALID_SOURCE_FIELD"))
}
/// Absolute paths only; reject symlinks in every component. On Linux the final
/// open also refuses symlinks and cannot block on a concurrently substituted FIFO.
/// Concurrent ancestor replacement remains outside this filesystem trust model.
fn read(path: &str, limit: usize) -> BridgeResult<Vec<u8>> {
    let path = Path::new(path);
    if !path.is_absolute() {
        return Err(BridgeError("SOURCE_PATH_NOT_ABSOLUTE"));
    }
    let mut prefix = std::path::PathBuf::new();
    for c in path.components() {
        if !matches!(c, Component::RootDir | Component::Normal(_)) {
            return Err(BridgeError("INVALID_SOURCE_PATH"));
        }
        prefix.push(c);
        let m = fs::symlink_metadata(&prefix).map_err(|_| BridgeError("SOURCE_READ_ERROR"))?;
        if m.file_type().is_symlink() {
            return Err(BridgeError("SOURCE_SYMLINK"));
        }
    }
    let m = fs::symlink_metadata(path).map_err(|_| BridgeError("SOURCE_READ_ERROR"))?;
    if !m.is_file() || m.len() > limit as u64 {
        return Err(BridgeError("SOURCE_NOT_REGULAR_OR_TOO_LARGE"));
    }
    #[cfg(not(target_os = "linux"))]
    return Err(BridgeError("SAFE_LOCAL_ADAPTER_REQUIRES_LINUX"));
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(0x20000 | 0x800)
            .open(path)
            .map_err(|_| BridgeError("SOURCE_READ_ERROR"))?;
        if !file
            .metadata()
            .map_err(|_| BridgeError("SOURCE_READ_ERROR"))?
            .is_file()
        {
            return Err(BridgeError("SOURCE_NOT_REGULAR"));
        }
        let mut b = Vec::new();
        file.take(limit as u64 + 1)
            .read_to_end(&mut b)
            .map_err(|_| BridgeError("SOURCE_READ_ERROR"))?;
        if b.len() > limit {
            return Err(BridgeError("SOURCE_TOO_LARGE"));
        }
        Ok(b)
    }
}

/// Manifest is itself retained byte-for-byte in the separate provenance report.
/// Expected hashes/roles are caller assertions, never authenticated collection.
pub fn prepare_bundle(bytes: &[u8]) -> BridgeResult<Value> {
    if bytes.len() > status::MAX_PAYLOAD {
        return Err(BridgeError("SOURCE_MANIFEST_TOO_LARGE"));
    }
    let v = parse_exchange_v1(bytes).map_err(|_| BridgeError("INVALID_SOURCE_BUNDLE"))?;
    keys(&v, &["contract", "previous", "selected_log", "sources"])?;
    if v["contract"] != "LOCAL_SOURCE_FILES_V1" {
        return Err(BridgeError("INVALID_SOURCE_CONTRACT"));
    }
    let previous = status::Status::from_value(v["previous"].clone(), status::PreviousKind::Local)?;
    let selected = string(&v, "selected_log")?;
    let entries = v["sources"]
        .as_array()
        .filter(|a| !a.is_empty() && a.len() <= 256)
        .ok_or(BridgeError("INVALID_SOURCE_COUNT"))?;
    let mut owned = Vec::new();
    let mut ids = std::collections::BTreeSet::new();
    let mut roles = std::collections::BTreeSet::new();
    let mut remaining = logs::MAX_LOG_BYTES;
    for e in entries {
        keys(
            e,
            &["id", "kind", "path", "sha256", "sidecar_path", "block_id"],
        )?;
        let id = string(e, "id")?;
        if id.len() > 4096 || !ids.insert(id) {
            return Err(BridgeError("INVALID_OR_DUPLICATE_SOURCE_ID"));
        }
        let kind = match string(e, "kind")? {
            "RunnerLog" => sources::Kind::RunnerLog,
            "GitObservation" => sources::Kind::GitObservation,
            "ImageInventory" => sources::Kind::ImageInventory,
            _ => return Err(BridgeError("INVALID_SOURCE_KIND")),
        };
        roles.insert(kind);
        let path = string(e, "path")?;
        if kind == sources::Kind::RunnerLog
            && Path::new(path).file_name().and_then(|s| s.to_str())
                != Path::new(id).file_name().and_then(|s| s.to_str())
        {
            return Err(BridgeError("SOURCE_NAME_MISMATCH"));
        }
        let b = read(path, remaining)?;
        remaining -= b.len();
        let expected = string(e, "sha256")?;
        if expected != format!("{:x}", Sha256::digest(&b)) {
            return Err(BridgeError("SOURCE_HASH_MISMATCH"));
        }
        let side = if e["sidecar_path"].is_null() {
            None
        } else {
            let b = read(string(e, "sidecar_path")?, remaining.min(8192))?;
            remaining -= b.len();
            Some(b)
        };
        if kind == sources::Kind::RunnerLog {
            let report = logs::inspect_bytes(
                Path::new(id).file_name().unwrap().to_str().unwrap(),
                &b,
                side.as_deref(),
            )?;
            if report["block_id"] != Value::String(string(e, "block_id")?.to_owned()) {
                return Err(BridgeError("SOURCE_BLOCK_MISMATCH"));
            }
        } else if !e["block_id"].is_null() || side.is_some() {
            return Err(BridgeError("INVALID_OBSERVATION_ASSOCIATION"));
        }
        owned.push((id, kind, b, side));
    }
    if !ids.contains(selected)
        || roles.len() != 3
        || !owned
            .iter()
            .any(|(id, k, _, _)| *id == selected && *k == sources::Kind::RunnerLog)
    {
        return Err(BridgeError("INCOMPLETE_SOURCE_BUNDLE"));
    }
    let refs: Vec<_> = owned
        .iter()
        .map(|(id, kind, b, s)| sources::Source {
            id,
            kind: *kind,
            bytes: b,
            sidecar: s.as_deref(),
        })
        .collect();
    let merged = sources::merge(&previous, &refs, Some(selected))?;
    let mut provenance = merged.provenance;
    provenance["file_selection"] = json!({"contract":"LOCAL_SOURCE_FILES_V1","manifest_base64":crate::protocol::encode_base64(bytes),"manifest_sha256":format!("{:x}",Sha256::digest(bytes)),"association":"CALLER_SUPPLIED_NOT_AUTHENTICATED","expected_original_hashes":"PASS"});
    Ok(
        json!({"payload":parse_exchange_v1(merged.payload.bytes()).map_err(|_| BridgeError("INVALID_PREPARED_PAYLOAD"))?,"provenance":provenance}),
    )
}
