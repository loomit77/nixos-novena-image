use novena_bridge::{logs::*, status::*};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::process::{Command, Stdio};

const COMPLETE: &str =
    "BLOCK_ID=SYNTHETIC\nCOMMAND_EXIT_STATUS=0\nBLOCK_RESULT=PASS\nLOG_COMPLETE=YES\n";
fn side(bytes: &[u8]) -> Vec<u8> {
    format!("{:x}  /archive/SYNTHETIC.txt\n", Sha256::digest(bytes)).into_bytes()
}
fn inspect(text: &str) -> Value {
    inspect_bytes(
        "SYNTHETIC.txt",
        text.as_bytes(),
        Some(&side(text.as_bytes())),
    )
    .unwrap()
}

#[test]
fn python_selftest_vectors_and_independent_hash() {
    // Reuse the literal fixture and all five discriminating cases in the Python
    // selftest, without executing its tempfile-writing selftest function.
    let child = Command::new("python3").args(["-B", "-c", r#"
import ast, hashlib, json
from pathlib import Path
tree = ast.parse(Path('../../scripts/novena-bridge-lite.py').read_text())
func = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == 'selftest')
assignment = next(n for n in ast.walk(func) if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'text' for t in n.targets))
text = ast.literal_eval(assignment.value)
print(json.dumps({'text': text, 'sha256': hashlib.sha256(text.encode()).hexdigest()}))
"#]).stdin(Stdio::null()).stdout(Stdio::piped()).spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let oracle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(oracle["text"], COMPLETE);
    let sidecar = format!("{}  SYNTHETIC.txt\n", oracle["sha256"].as_str().unwrap());
    let report = inspect_bytes(
        "SYNTHETIC.txt",
        COMPLETE.as_bytes(),
        Some(sidecar.as_bytes()),
    )
    .unwrap();
    assert_eq!(report["state"], "COMPLETE");
    assert_eq!(report["integrity"], "PASS");
    assert_eq!(report["professional_acceptance"], "UNKNOWN");
    assert_eq!(
        inspect(&format!("{COMPLETE}BLOCK_RESULT=FAIL\n"))["state"],
        "REVIEW_REQUIRED"
    );
    assert_eq!(
        inspect_bytes("SYNTHETIC.txt", COMPLETE.as_bytes(), None).unwrap()["integrity"],
        "MISSING"
    );
    assert_eq!(
        inspect_bytes("SYNTHETIC.txt", b"BLOCK_ID=RUNNING\n", None).unwrap()["process_result"],
        "UNKNOWN"
    );
    assert_eq!(
        verify_sidecar(
            "SYNTHETIC.txt",
            COMPLETE.as_bytes(),
            Some(format!("{}  SYNTHETIC.txt\n", "0".repeat(64)).as_bytes())
        ),
        "FAIL"
    );
    assert_eq!(
        verify_sidecar(
            "abc.txt",
            b"abc",
            Some(b"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  abc.txt\n")
        ),
        "PASS"
    );
}

#[test]
fn contradictions_truncation_and_invalid_fields_never_pass() {
    for key_value in [
        "BLOCK_ID=OTHER",
        "BLOCK_RESULT=FAIL",
        "COMMAND_EXIT_STATUS=1",
        "LOG_COMPLETE=NO",
    ] {
        let report = inspect(&format!("{COMPLETE}{key_value}\n"));
        assert_eq!(report["state"], "REVIEW_REQUIRED");
        assert_eq!(report["process_result"], "UNKNOWN");
        assert!(report["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r.as_str().unwrap().starts_with("CONFLICTING_")));
    }
    for (from, to) in [
        ("BLOCK_RESULT=PASS", "BLOCK_RESULT=WHAT"),
        ("COMMAND_EXIT_STATUS=0", "COMMAND_EXIT_STATUS=-1"),
        ("COMMAND_EXIT_STATUS=0", "COMMAND_EXIT_STATUS=256"),
        ("COMMAND_EXIT_STATUS=0", "COMMAND_EXIT_STATUS=1"),
        ("BLOCK_ID=SYNTHETIC", "BLOCK_ID="),
        ("LOG_COMPLETE=YES\n", ""),
    ] {
        let report = inspect(&COMPLETE.replace(from, to));
        assert_eq!(report["process_result"], "UNKNOWN", "{from} -> {to}");
    }
    for line in COMPLETE.lines() {
        let report = inspect(&COMPLETE.replace(&format!("{line}\n"), ""));
        assert_eq!(report["process_result"], "UNKNOWN");
    }
    assert_eq!(
        inspect(&format!("{COMPLETE}BLOCK_RESULT=PASS\n"))["state"],
        "COMPLETE"
    );
    assert_eq!(
        inspect(
            &COMPLETE
                .replace("=PASS", "=FAIL")
                .replace("STATUS=0", "STATUS=2")
        )["process_result"],
        "FAIL"
    );
    let bytes = [COMPLETE.as_bytes(), b"\xff"].concat();
    assert_eq!(
        inspect_bytes("SYNTHETIC.txt", &bytes, Some(&side(&bytes))).unwrap()["process_result"],
        "UNKNOWN"
    );
    assert!(inspect_bytes("large.txt", &vec![b'x'; MAX_LOG_BYTES + 1], None).is_err());
}

#[test]
fn sidecars_are_separate_and_strict() {
    for sidecar in [
        b"".as_slice(),
        b"bad",
        b"\xff",
        b"abc  SYNTHETIC.txt\n",
        &vec![b'x'; 8193],
    ] {
        assert_eq!(
            verify_sidecar("SYNTHETIC.txt", COMPLETE.as_bytes(), Some(sidecar)),
            "INVALID"
        );
    }
    let good = String::from_utf8(side(COMPLETE.as_bytes())).unwrap();
    for bad in [
        good.replace("SYNTHETIC.txt", "wrong.txt"),
        format!("{good}\n"),
        format!("{good}{good}"),
    ] {
        assert_eq!(
            verify_sidecar("SYNTHETIC.txt", COMPLETE.as_bytes(), Some(bad.as_bytes())),
            "INVALID"
        );
    }
    assert_eq!(
        verify_sidecar(
            "SYNTHETIC.txt",
            COMPLETE.as_bytes(),
            Some(good.to_uppercase().as_bytes())
        ),
        "INVALID"
    );
    let binary = good.replace("  /archive/", "\t*/archive/");
    assert_eq!(
        verify_sidecar(
            "SYNTHETIC.txt",
            COMPLETE.as_bytes(),
            Some(binary.as_bytes())
        ),
        "PASS"
    );
}

#[test]
fn upload_and_runner_never_advance_checkpoint() {
    let mut v: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
    ))
    .unwrap();
    v["last_verified"] = json!({"block_id":"BASE", "git_head":"a".repeat(40), "qualified_at_utc":"2026-10-26T00:00:00Z", "qualification":"PASS", "evidence_files":["original.txt"]});
    let previous = Status::from_value(v.clone(), PreviousKind::Local).unwrap();
    for extra in [
        "UPLOAD_RESULT=PASS\n",
        "UPLOAD_RESULT=PASS\nUPLOAD_RESULT=FAIL\n",
        "BLOCK_RESULT=FAIL\n",
    ] {
        let text = format!("{COMPLETE}{extra}");
        let prepared = prepare_from_log(
            &previous,
            "SYNTHETIC.txt",
            text.as_bytes(),
            Some(&side(text.as_bytes())),
        )
        .unwrap();
        let after: Value = serde_json::from_slice(prepared.bytes()).unwrap();
        assert_eq!(after["last_verified"], v["last_verified"]);
        assert_eq!(after["current"], v["current"]);
        for key in ["upload", "technical_result", "professional_acceptance"] {
            assert_eq!(after["latest_block"][key], "UNKNOWN");
        }
    }
}

#[test]
fn existing_file_cli_is_read_only_and_missing_sidecar_is_unknown() {
    let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/SYNTHETIC.txt");
    let original = std::fs::read(fixture).unwrap();
    let sidepath = format!("{fixture}.sha256");
    let original_sidecar = std::fs::read(&sidepath).unwrap();
    assert_eq!(original, COMPLETE.as_bytes());
    let out = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args(["inspect-log", fixture])
        .output()
        .unwrap();
    assert!(out.status.success());
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["integrity"], "PASS");
    assert_eq!(report["state"], "COMPLETE");
    assert_eq!(report["technical_result"], "UNKNOWN");
    assert_eq!(std::fs::read(fixture).unwrap(), original);
    assert_eq!(std::fs::read(sidepath).unwrap(), original_sidecar);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/logs.rs");
    let before = std::fs::read(path).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args(["inspect-log", path])
        .output()
        .unwrap();
    assert!(out.status.success());
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["integrity"], "MISSING");
    assert_eq!(report["state"], "REVIEW_REQUIRED");
    assert_eq!(std::fs::read(path).unwrap(), before);
    for path in ["/nonexistent/runner.txt", env!("CARGO_MANIFEST_DIR")] {
        let out = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
            .args(["inspect-log", path])
            .output()
            .unwrap();
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
    }
}
