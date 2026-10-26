use serde_json::Value;
use std::{
    io::Write,
    process::{Command, Output, Stdio},
};
fn cli(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let bytes = input.to_vec();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&bytes);
    });
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    out
}
#[test]
fn strict_cli_negative_regression_and_no_partial_stdout() {
    for source in [
        br#"{"a":1,"a":2}"#.as_slice(),
        br#"{"a":1,"\u0061":2}"#,
        b"\xff",
        b"1.0",
        b"1e0",
        b"01",
        b"{} trailing",
        br#""\ud800""#,
        b"18446744073709551616",
        b"[1,]",
        b"\xef\xbb\xbf{}",
    ] {
        for command in [
            "exchange-v1-json",
            "validate-exchange-v1",
            "prepare-initial",
            "prepare-objects",
            "prepare-update-objects",
            "verify-attestation",
            "verify-offline",
        ] {
            let out = cli(&[command, "-"], source);
            assert!(!out.status.success(), "{command}");
            assert!(out.stdout.is_empty(), "{command}");
        }
    }
    for args in [
        vec![],
        vec!["unknown", "missing-file"],
        vec!["prepare-initial"],
        vec!["prepare-initial", "-", "extra"],
        vec!["prepare-status", "-", "-"],
        vec!["plan-cas", "ID", "bad", "bad"],
    ] {
        let out = cli(&args, b"");
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
    }
}
#[test]
fn production_disabled_before_any_input_is_opened() {
    for command in ["publish", "production-publish", "--publish"] {
        let out = cli(&[command, "nonexistent-input", "--force"], b"");
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(String::from_utf8_lossy(&out.stderr).contains("PRODUCTION_PUBLICATION_DISABLED"));
    }
}
#[test]
fn initial_payload_and_pure_cas_plan() {
    let source = include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json");
    let out = cli(&["prepare-initial", "-"], source);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stdout, novena_bridge::canonical_json(source).unwrap());
    let before = "a".repeat(40);
    let after = "b".repeat(40);
    let out = cli(&["plan-cas", "SYNTHETIC_ID", &before, &after], b"");
    assert!(out.status.success());
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["write_performed"], false);
    assert_eq!(v["bridge_publication"], "BLOCKED");
    assert_eq!(
        v["request"]["variables"]["input"]["refUpdates"][0]["beforeOid"],
        before
    );
    let mut v: Value = serde_json::from_slice(source).unwrap();
    v["publication"]["state"] = serde_json::json!("CONFIRMED");
    let out = cli(&["prepare-initial", "-"], &serde_json::to_vec(&v).unwrap());
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
}
