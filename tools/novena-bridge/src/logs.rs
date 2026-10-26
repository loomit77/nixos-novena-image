//! Read-only runner evidence. Hash equality never grants technical acceptance.
use crate::{status, BridgeError, BridgeResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};

pub const MAX_LOG_BYTES: usize = 32 * 1024 * 1024;
const MAX_SIDECAR_BYTES: usize = 8192;

/// One bounded read; final symlinks and special files are rejected. Parent path
/// trust and concurrent replacement still require an external filesystem policy.
fn read_regular(path: &Path, limit: usize) -> BridgeResult<Vec<u8>> {
    let meta = fs::symlink_metadata(path).map_err(|_| BridgeError("LOG_READ_ERROR"))?;
    if !meta.is_file() || meta.len() > limit as u64 {
        return Err(BridgeError("LOG_NOT_REGULAR_OR_TOO_LARGE"));
    }
    let file = fs::File::open(path).map_err(|_| BridgeError("LOG_READ_ERROR"))?;
    if !file
        .metadata()
        .map_err(|_| BridgeError("LOG_READ_ERROR"))?
        .is_file()
    {
        return Err(BridgeError("LOG_NOT_REGULAR"));
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| BridgeError("LOG_READ_ERROR"))?;
    if bytes.len() > limit {
        return Err(BridgeError("LOG_TOO_LARGE"));
    }
    Ok(bytes)
}

/// Verify against the exact bytes inspected, never a second read of the log.
pub fn verify_sidecar(source: &str, bytes: &[u8], sidecar: Option<&[u8]>) -> &'static str {
    let Some(sidecar) = sidecar else {
        return "MISSING";
    };
    if sidecar.len() > MAX_SIDECAR_BYTES {
        return "INVALID";
    }
    let Ok(text) = std::str::from_utf8(sidecar) else {
        return "INVALID";
    };
    let lines: Vec<_> = text.lines().collect();
    if lines.len() != 1 {
        return "INVALID";
    }
    let line = lines[0];
    let Some(split) = line.find([' ', '\t']) else {
        return "INVALID";
    };
    let (hash, name) = line.split_at(split);
    let name = name
        .trim_start_matches([' ', '\t'])
        .strip_prefix('*')
        .unwrap_or_else(|| name.trim_start_matches([' ', '\t']));
    if hash.len() != 64
        || !hash.bytes().all(|c| c.is_ascii_hexdigit())
        || Path::new(name).file_name().and_then(|n| n.to_str()) != Some(source)
    {
        return "INVALID";
    }
    if format!("{:x}", Sha256::digest(bytes)).eq_ignore_ascii_case(hash) {
        "PASS"
    } else {
        "FAIL"
    }
}

/// Pure parser. Claimed BLOCK_RESULT is retained separately; neither runner nor
/// upload output is independent evidence of technical or professional approval.
pub fn inspect_bytes(source: &str, bytes: &[u8], sidecar: Option<&[u8]>) -> BridgeResult<Value> {
    if bytes.len() > MAX_LOG_BYTES {
        return Err(BridgeError("LOG_TOO_LARGE"));
    }
    let integrity = verify_sidecar(source, bytes, sidecar);
    let raw = String::from_utf8_lossy(bytes);
    let mut reasons = Vec::new();
    if std::str::from_utf8(bytes).is_err() {
        reasons.push("INVALID_UTF8".to_owned());
    }
    let keys = [
        "BLOCK_ID",
        "BLOCK_RESULT",
        "COMMAND_EXIT_STATUS",
        "LOG_COMPLETE",
        "UPLOAD_RESULT",
    ];
    let fields: Vec<Vec<&str>> = keys
        .iter()
        .map(|key| {
            let prefix = format!("{key}=");
            raw.lines()
                .filter_map(|line| line.strip_prefix(&prefix).map(str::trim))
                .collect()
        })
        .collect();
    for (key, values) in keys.iter().zip(&fields) {
        if values.iter().any(|value| Some(value) != values.first()) {
            reasons.push(format!("CONFLICTING_{key}"));
        }
    }
    let get = |i: usize| fields[i].last().copied();
    let complete = get(3) == Some("YES") && fields[3].iter().all(|v| *v == "YES");
    if !complete {
        reasons.push("LOG_INCOMPLETE".into());
    }
    if get(0).is_none_or(str::is_empty) || get(1).is_none() || get(2).is_none() {
        reasons.push("MISSING_RESULT_FIELDS".into());
    }
    if !matches!(get(1), Some("PASS" | "FAIL")) {
        reasons.push("INVALID_BLOCK_RESULT".into());
    }
    let exit = get(2)
        .filter(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|v| v.parse::<u8>().ok());
    if exit.is_none() {
        reasons.push("INVALID_EXIT_STATUS".into());
    }
    if matches!(
        (get(1), exit),
        (Some("PASS"), Some(1..=255)) | (Some("FAIL"), Some(0))
    ) {
        reasons.push("CONTRADICTORY_RESULT_EXIT".into());
    }
    if integrity != "PASS" {
        reasons.push(format!("LOG_INTEGRITY_{integrity}"));
    }
    reasons.sort();
    reasons.dedup();
    let trusted = reasons.is_empty();
    Ok(
        json!({"source":source, "block_id":get(0), "complete":complete,
        "state":if trusted {"COMPLETE"} else {"REVIEW_REQUIRED"}, "integrity":integrity,
        "process_result":if trusted {if exit == Some(0) {"PASS"} else {"FAIL"}} else {"UNKNOWN"},
        "claimed_block_result":get(1), "technical_result":"UNKNOWN",
        "professional_acceptance":"UNKNOWN", "upload_result":"UNKNOWN", "reasons":reasons}),
    )
}

pub fn inspect_file(path: &Path) -> BridgeResult<Value> {
    let source = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or(BridgeError("INVALID_LOG_NAME"))?;
    let bytes = read_regular(path, MAX_LOG_BYTES)?;
    let mut sidepath = path.as_os_str().to_os_string();
    sidepath.push(".sha256");
    let sidepath = Path::new(&sidepath);
    let side = match fs::symlink_metadata(sidepath) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        _ => Some(read_regular(sidepath, MAX_SIDECAR_BYTES).unwrap_or_default()),
    };
    inspect_bytes(source, &bytes, side.as_deref())
}

/// Reparse original bytes; callers cannot forge an inspection report to advance
/// status. Existing checkpoint and current observation remain byte-value equal.
pub fn prepare_from_log(
    previous: &status::Status,
    source: &str,
    bytes: &[u8],
    sidecar: Option<&[u8]>,
) -> BridgeResult<status::PreparedStatus> {
    // Only local documents: remote publication-field semantics remain undecided.
    status::Status::from_value(previous.document().clone(), status::PreviousKind::Local)?;
    let report = inspect_bytes(source, bytes, sidecar)?;
    let block = report["block_id"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or(BridgeError("MISSING_BLOCK_ID"))?;
    let mut candidate = previous.document().clone();
    candidate["latest_block"] = json!({"block_id":block, "runner":report["process_result"],
        "upload":"UNKNOWN", "technical_result":"UNKNOWN", "professional_acceptance":"UNKNOWN"});
    status::prepare(
        previous,
        &status::Status::from_value(candidate, status::PreviousKind::Local)?,
    )
}
