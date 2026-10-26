use novena_bridge::{sources::*, status::*};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
fn previous() -> Status {
    let mut v: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
    ))
    .unwrap();
    v["last_verified"] = json!({"block_id":"BASE","git_head":"a".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["base.txt"]});
    Status::from_value(v, PreviousKind::Local).unwrap()
}
const LOG: &[u8] = b"BLOCK_ID=TEST\nBLOCK_RESULT=PASS\nCOMMAND_EXIT_STATUS=0\nLOG_COMPLETE=YES\nUPLOAD_RESULT=PASS\n";
fn check(
    logs: &[(&str, &[u8])],
    git: &[u8],
    inventory: &[u8],
    selected: Option<&str>,
    side: bool,
) -> Merge {
    let sides: Vec<_> = logs
        .iter()
        .map(|(name, b)| format!("{:x}  {name}\n", Sha256::digest(b)))
        .collect();
    let mut sources: Vec<_> = logs
        .iter()
        .zip(&sides)
        .map(|((id, bytes), s)| Source {
            id,
            kind: Kind::RunnerLog,
            bytes,
            sidecar: side.then_some(s.as_bytes()),
        })
        .collect();
    sources.push(Source {
        id: "git",
        kind: Kind::GitObservation,
        bytes: git,
        sidecar: None,
    });
    sources.push(Source {
        id: "inventory",
        kind: Kind::ImageInventory,
        bytes: inventory,
        sidecar: None,
    });
    merge(&previous(), &sources, selected).unwrap()
}
fn git() -> Vec<u8> {
    serde_json::to_vec(&json!({"head":"a".repeat(40),"branch":"main","worktree":"MODIFIED","remote_sync":"UNKNOWN"})).unwrap()
}
const INVENTORY: &[u8] = br#"{"record_count":200,"unique_file_hashes":100,"host_counts":{"foobox":200},"scan_warnings":[],"path_conflicts":[],"opaque_original_field":"retained"}"#;
#[test]
fn originals_linked_and_only_latest_block_projected() {
    let m = check(
        &[("test.txt", LOG)],
        &git(),
        INVENTORY,
        Some("test.txt"),
        true,
    );
    assert_eq!(m.provenance["state"], "OBSERVATIONS_MERGED");
    let v: Value = serde_json::from_slice(m.payload.bytes()).unwrap();
    let mut expected = previous().document().clone();
    expected["latest_block"] = json!({"block_id":"TEST","runner":"PASS","upload":"UNKNOWN","technical_result":"UNKNOWN","professional_acceptance":"UNKNOWN"});
    assert_eq!(v, expected);
    let s = &m.provenance["sources"][0];
    assert_eq!(
        novena_bridge::protocol::decode_base64(s["original_base64"].as_str().unwrap()).unwrap(),
        LOG
    );
    assert_eq!(s["sha256"], format!("{:x}", Sha256::digest(LOG)));
    assert_eq!(s["inspection"]["integrity"], "PASS");
    assert_eq!(m.provenance["build"], "UNKNOWN");
    assert_eq!(m.provenance["hardware_image_identity"], "UNKNOWN");
    assert_eq!(
        m.provenance["sources"][2]["inspection"]["observation"]["opaque_original_field"],
        "retained"
    );
}
#[test]
fn conflicting_block_sources_and_missing_integrity_cannot_pass() {
    let bad = [LOG, b"BLOCK_RESULT=FAIL\n"].concat();
    for side in [true, false] {
        let m = check(
            &[("a.txt", LOG), ("b.txt", &bad)],
            &git(),
            INVENTORY,
            Some("a.txt"),
            side,
        );
        assert_eq!(m.provenance["state"], "REVIEW_REQUIRED");
        let v: Value = serde_json::from_slice(m.payload.bytes()).unwrap();
        assert_eq!(v["latest_block"]["runner"], "UNKNOWN");
        assert_eq!(v["last_verified"], previous().document()["last_verified"]);
    }
}
#[test]
fn selection_missing_invalid_and_duplicate_sources() {
    for selected in [None, Some("missing")] {
        let m = check(&[("a.txt", LOG)], b"{}", b"{}", selected, true);
        assert_eq!(m.provenance["state"], "REVIEW_REQUIRED");
        assert_eq!(
            serde_json::from_slice::<Value>(m.payload.bytes()).unwrap(),
            *previous().document()
        );
    }
    let s = || Source {
        id: "duplicate",
        kind: Kind::ImageInventory,
        bytes: INVENTORY,
        sidecar: None,
    };
    assert!(merge(&previous(), &[s(), s()], None).is_err());
    let m = merge(&previous(), &[], None).unwrap();
    assert_eq!(m.provenance["state"], "REVIEW_REQUIRED");
}
#[test]
fn git_and_inventory_conflicts_retained_without_cross_domain_promotion() {
    let g = git();
    let other = String::from_utf8(g.clone())
        .unwrap()
        .replace("main", "other");
    let inv = String::from_utf8(INVENTORY.to_vec())
        .unwrap()
        .replace("\"path_conflicts\":[]", "\"path_conflicts\":[{}]");
    let m = merge(
        &previous(),
        &[
            Source {
                id: "git1",
                kind: Kind::GitObservation,
                bytes: &g,
                sidecar: None,
            },
            Source {
                id: "git2",
                kind: Kind::GitObservation,
                bytes: other.as_bytes(),
                sidecar: None,
            },
            Source {
                id: "inv",
                kind: Kind::ImageInventory,
                bytes: inv.as_bytes(),
                sidecar: None,
            },
        ],
        None,
    )
    .unwrap();
    let reasons = m.provenance["reasons"].as_array().unwrap();
    assert!(reasons.contains(&json!("CONFLICTING_OBSERVATIONS")));
    assert!(reasons.contains(&json!("INVENTORY_REQUIRES_REVIEW")));
    assert_eq!(
        serde_json::from_slice::<Value>(m.payload.bytes()).unwrap(),
        *previous().document()
    );
}

#[test]
fn malformed_observations_and_inconsistent_counters_require_review() {
    for bytes in [
        b"\xff".as_slice(),
        b"{\"record_count\":1,\"record_count\":2}",
        b"null",
    ] {
        let m = check(&[("a.txt", LOG)], bytes, bytes, Some("a.txt"), true);
        assert_eq!(m.provenance["state"], "REVIEW_REQUIRED");
        assert_eq!(m.provenance["build"], "UNKNOWN");
    }
    let bad = String::from_utf8(INVENTORY.to_vec())
        .unwrap()
        .replace("\"record_count\":200", "\"record_count\":1");
    let m = check(
        &[("a.txt", LOG)],
        &git(),
        bad.as_bytes(),
        Some("a.txt"),
        true,
    );
    assert!(m.provenance["reasons"]
        .as_array()
        .unwrap()
        .contains(&json!("INCONSISTENT_INVENTORY_COUNTS")));
    let m = check(&[("a.txt", LOG)], &git(), INVENTORY, Some("a.txt"), false);
    assert_eq!(m.provenance["sources"][0]["sha256_integrity"], "MISSING");
    assert_eq!(
        serde_json::from_slice::<Value>(m.payload.bytes()).unwrap()["latest_block"]["runner"],
        "UNKNOWN"
    );
    assert_eq!(m.provenance["sources"][1]["sha256_integrity"], "UNKNOWN");
}
