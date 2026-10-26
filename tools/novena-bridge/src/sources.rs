//! Local provenance contract, separate from Exchange-v1 and from attestations.
//! No filesystem discovery, timestamp ordering, remote trust or acceptance inference.
use crate::{logs, parse_exchange_v1, protocol::encode_base64, status, BridgeError, BridgeResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    RunnerLog,
    GitObservation,
    ImageInventory,
}
/// Identity and role are caller supplied. Bytes are retained, not reconstructed.
pub struct Source<'a> {
    pub id: &'a str,
    pub kind: Kind,
    pub bytes: &'a [u8],
    pub sidecar: Option<&'a [u8]>,
}
#[derive(Debug)]
pub struct Merge {
    pub payload: status::PreparedStatus,
    pub provenance: Value,
}

/// Explicit selection replaces Lite's mtime-based "latest" heuristic. No report
/// supplied by a caller can masquerade as a freshly inspected original log.
/// Git observations use exactly snapshot.git's keys; inventory is the original
/// JSON (unknown fields retained only as opaque source data, never projected).
pub fn merge(
    previous: &status::Status,
    sources: &[Source<'_>],
    selected_log: Option<&str>,
) -> BridgeResult<Merge> {
    status::Status::from_value(previous.document().clone(), status::PreviousKind::Local)?;
    if sources.len() > 256
        || sources
            .iter()
            .map(|s| {
                s.bytes
                    .len()
                    .saturating_add(s.sidecar.map_or(0, <[u8]>::len))
            })
            .sum::<usize>()
            > logs::MAX_LOG_BYTES
    {
        return Err(BridgeError("SOURCE_BUDGET_EXCEEDED"));
    }
    let mut ids = BTreeSet::new();
    let mut reports = Vec::new();
    let mut reasons = BTreeSet::new();
    let mut observations = BTreeMap::new();
    let mut blocks: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut selected = None;
    for source in sources {
        if source.id.is_empty() || source.id.len() > 4096 || !ids.insert(source.id) {
            return Err(BridgeError("INVALID_OR_DUPLICATE_SOURCE_ID"));
        }
        let hash = format!("{:x}", Sha256::digest(source.bytes));
        let inspection = match source.kind {
            Kind::RunnerLog => {
                let name = std::path::Path::new(source.id)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or(BridgeError("INVALID_LOG_NAME"))?;
                let report = logs::inspect_bytes(name, source.bytes, source.sidecar)?;
                if report["state"] != "COMPLETE" {
                    reasons.insert("LOG_REVIEW_REQUIRED");
                }
                if let Some(block) = report["block_id"].as_str() {
                    blocks
                        .entry(block.to_owned())
                        .or_default()
                        .push(reports.len());
                }
                if selected_log == Some(source.id) {
                    selected = Some(reports.len());
                }
                report
            }
            kind => {
                // A computed digest identifies bytes, but is no independent integrity claim.
                let parsed = parse_exchange_v1(source.bytes).ok();
                let valid = parsed.as_ref().is_some_and(|v| match kind {
                    Kind::GitObservation => {
                        v.as_object().is_some_and(|o| o.len() == 4)
                            && v["head"]
                                .as_str()
                                .is_some_and(|s| crate::objects::Oid::parse(s).is_ok())
                            && v["branch"].is_string()
                            && matches!(v["worktree"].as_str(), Some("CLEAN" | "MODIFIED"))
                            && v["remote_sync"] == "UNKNOWN"
                    }
                    Kind::ImageInventory => {
                        v.is_object()
                            && v["record_count"].as_u64().is_some()
                            && v["unique_file_hashes"].as_u64().is_some()
                            && v["host_counts"]
                                .as_object()
                                .is_some_and(|o| o.values().all(|n| n.as_u64().is_some()))
                            && v["scan_warnings"].is_array()
                            && v["path_conflicts"].is_array()
                    }
                    _ => false,
                });
                if !valid {
                    reasons.insert("MISSING_OR_INVALID_OBSERVATION");
                }
                if let Some(old) = observations.insert(kind, parsed.clone()) {
                    if old != parsed {
                        reasons.insert("CONFLICTING_OBSERVATIONS");
                    }
                }
                if kind == Kind::ImageInventory && valid {
                    let v = parsed.as_ref().unwrap();
                    let records = v["record_count"].as_u64().unwrap();
                    let total = v["host_counts"]
                        .as_object()
                        .unwrap()
                        .values()
                        .try_fold(0u64, |sum, n| sum.checked_add(n.as_u64().unwrap()));
                    if v["unique_file_hashes"].as_u64().unwrap() > records || total != Some(records)
                    {
                        reasons.insert("INCONSISTENT_INVENTORY_COUNTS");
                    }
                    if !v["scan_warnings"].as_array().unwrap().is_empty()
                        || !v["path_conflicts"].as_array().unwrap().is_empty()
                    {
                        reasons.insert("INVENTORY_REQUIRES_REVIEW");
                    }
                }
                json!({"state":if valid {"OBSERVATION_ONLY"} else {"REVIEW_REQUIRED"}, "observation":parsed})
            }
        };
        reports.push(json!({"id":source.id,"kind":format!("{:?}",source.kind),"sha256":hash,
            "original_base64":encode_base64(source.bytes),"sidecar_base64":source.sidecar.map(encode_base64),
            "sha256_integrity":if source.kind == Kind::RunnerLog {inspection["integrity"].clone()} else {json!("UNKNOWN")},
            "inspection":inspection,"source_authenticity":"UNKNOWN"}));
    }
    if !observations.contains_key(&Kind::GitObservation)
        || !observations.contains_key(&Kind::ImageInventory)
    {
        reasons.insert("MISSING_OBSERVATION");
    }
    let mut conflicting_logs = BTreeSet::new();
    for indexes in blocks.values() {
        if indexes
            .iter()
            .any(|i| reports[*i]["sha256"] != reports[indexes[0]]["sha256"])
        {
            reasons.insert("CONFLICTING_BLOCK_SOURCES");
            conflicting_logs.extend(indexes.iter().copied());
        }
    }
    let mut candidate = previous.document().clone();
    if let Some(index) = selected {
        let report = &reports[index]["inspection"];
        if let Some(block) = report["block_id"].as_str().filter(|s| !s.is_empty()) {
            candidate["latest_block"] = json!({"block_id":block,
                "runner":if conflicting_logs.contains(&index) {json!("UNKNOWN")} else {report["process_result"].clone()},
                "upload":"UNKNOWN","technical_result":"UNKNOWN","professional_acceptance":"UNKNOWN"});
        } else {
            reasons.insert("SELECTED_LOG_MISSING_BLOCK_ID");
        }
    } else {
        reasons.insert("NO_SELECTED_LOG");
    }
    let payload = status::prepare(
        previous,
        &status::Status::from_value(candidate, status::PreviousKind::Local)?,
    )?;
    Ok(Merge {
        payload,
        provenance: json!({"contract":"LOCAL_SOURCE_MERGE_V1",
        "scope":"CALLER_SUPPLIED_ORIGINALS","exchange_v1_projection":"LATEST_BLOCK_ONLY",
        "selected_log":selected_log,"sources":reports,"reasons":reasons,
        "state":if reasons.is_empty() {"OBSERVATIONS_MERGED"} else {"REVIEW_REQUIRED"},
        "build":"UNKNOWN","hardware_image_identity":"UNKNOWN","remote_sync":"UNKNOWN",
        "checkpoint_approval":"NOT_GRANTED","publication_performed":false}),
    })
}
