use novena_bridge::{local_sources::prepare_bundle, protocol::decode_base64};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
struct Fixture {
    dir: PathBuf,
    manifest: Value,
}
impl Fixture {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "tests/.synthetic-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let log = include_bytes!("fixtures/SYNTHETIC.txt");
        let git = serde_json::to_vec(&json!({"head":"a".repeat(40),"branch":"main","worktree":"MODIFIED","remote_sync":"UNKNOWN"})).unwrap();
        let inv = br#"{"record_count":0,"unique_file_hashes":0,"host_counts":{},"scan_warnings":[],"path_conflicts":[],"opaque":"retained"}"#;
        let mut entries = Vec::new();
        for (id, kind, bytes) in [
            ("SYNTHETIC.txt", "RunnerLog", log.as_slice()),
            ("git.json", "GitObservation", git.as_slice()),
            ("inventory.json", "ImageInventory", inv.as_slice()),
        ] {
            let path = dir.join(id);
            fs::write(&path, bytes).unwrap();
            entries.push(json!({"id":id,"kind":kind,"path":path,"sha256":format!("{:x}",Sha256::digest(bytes)),"sidecar_path":null,"block_id":if kind=="RunnerLog" {json!("SYNTHETIC")} else {Value::Null}}));
        }
        let side = dir.join("SYNTHETIC.txt.sha256");
        fs::write(&side, include_bytes!("fixtures/SYNTHETIC.txt.sha256")).unwrap();
        entries[0]["sidecar_path"] = json!(side);
        let mut previous: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
        ))
        .unwrap();
        previous["last_verified"] = json!({"block_id":"BASE","git_head":"b".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["base.txt"]});
        Self {
            dir,
            manifest: json!({"contract":"LOCAL_SOURCE_FILES_V1","previous":previous,"selected_log":"SYNTHETIC.txt","sources":entries}),
        }
    }
    fn bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.manifest).unwrap()
    }
    fn cli(&self) -> std::process::Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
            .args(["prepare-local-sources", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&self.bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    }
    fn replace_log(&mut self, b: &[u8]) {
        fs::write(self.dir.join("SYNTHETIC.txt"), b).unwrap();
        self.manifest["sources"][0]["sha256"] = json!(format!("{:x}", Sha256::digest(b)));
        fs::write(
            self.dir.join("SYNTHETIC.txt.sha256"),
            format!("{:x}  SYNTHETIC.txt\n", Sha256::digest(b)),
        )
        .unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap();
    }
}
#[test]
fn files_to_prepared_payload_and_separate_byte_exact_provenance() {
    let f = Fixture::new();
    let output = f.cli();
    assert!(output.status.success(), "{:?}", output);
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["provenance"]["state"], "OBSERVATIONS_MERGED");
    let mut expected = f.manifest["previous"].clone();
    expected["latest_block"] = json!({"block_id":"SYNTHETIC","runner":"PASS","upload":"UNKNOWN","technical_result":"UNKNOWN","professional_acceptance":"UNKNOWN"});
    assert_eq!(v["payload"], expected);
    novena_bridge::validate_exchange_v1(&v["payload"]).unwrap();
    for (e, s) in f.manifest["sources"]
        .as_array()
        .unwrap()
        .iter()
        .zip(v["provenance"]["sources"].as_array().unwrap())
    {
        assert_eq!(s["id"], e["id"]);
        assert_eq!(
            decode_base64(s["original_base64"].as_str().unwrap()).unwrap(),
            fs::read(e["path"].as_str().unwrap()).unwrap()
        );
    }
    assert_eq!(
        decode_base64(
            v["provenance"]["sources"][0]["sidecar_base64"]
                .as_str()
                .unwrap()
        )
        .unwrap(),
        include_bytes!("fixtures/SYNTHETIC.txt.sha256")
    );
    assert_eq!(
        decode_base64(
            v["provenance"]["file_selection"]["manifest_base64"]
                .as_str()
                .unwrap()
        )
        .unwrap(),
        f.bytes()
    );
}
#[test]
fn hard_errors_never_emit_partial_stdout() {
    for case in 0..15 {
        let mut f = Fixture::new();
        match case {
            0 => {
                fs::remove_file(f.dir.join("SYNTHETIC.txt")).unwrap();
            }
            1 => f.manifest["sources"][0]["sha256"] = json!("0".repeat(64)),
            2 => f.manifest["sources"][0]["block_id"] = json!("WRONG"),
            3 => {
                f.manifest["sources"].as_array_mut().unwrap().pop();
            }
            4 => f.manifest["selected_log"] = json!("missing"),
            5 => f.manifest["last_verified"] = json!({"qualification":"PASS"}),
            6 => {
                let e = f.manifest["sources"][0].clone();
                f.manifest["sources"].as_array_mut().unwrap().push(e);
            }
            7 => f.manifest["sources"][1]["block_id"] = json!("SYNTHETIC"),
            8 => f.manifest["sources"][1]["sha256"] = f.manifest["sources"][2]["sha256"].clone(),
            9 => {
                fs::remove_file(f.dir.join("SYNTHETIC.txt.sha256")).unwrap();
            }
            10 => {
                fs::remove_file(f.dir.join("SYNTHETIC.txt")).unwrap();
                std::os::unix::fs::symlink("git.json", f.dir.join("SYNTHETIC.txt")).unwrap();
            }
            11 => {
                fs::remove_file(f.dir.join("SYNTHETIC.txt.sha256")).unwrap();
                std::os::unix::fs::symlink("git.json", f.dir.join("SYNTHETIC.txt.sha256")).unwrap();
            }
            12 => {
                fs::remove_file(f.dir.join("SYNTHETIC.txt")).unwrap();
                fs::create_dir(f.dir.join("SYNTHETIC.txt")).unwrap();
            }
            13 => {
                assert!(Command::new("mkfifo")
                    .arg(f.dir.join("socket"))
                    .status()
                    .unwrap()
                    .success());
                f.manifest["sources"][1]["path"] = json!(f.dir.join("socket"));
            }
            14 => {
                std::os::unix::fs::symlink(&f.dir, f.dir.join("alias")).unwrap();
                f.manifest["sources"][1]["path"] = json!(f.dir.join("alias/git.json"));
            }
            _ => unreachable!(),
        }
        let o = f.cli();
        assert!(!o.status.success(), "case {case}");
        assert!(o.stdout.is_empty(), "case {case}");
    }
}
#[test]
fn review_cases_preserve_status_dimensions_and_checkpoint() {
    for case in 0..7 {
        let mut f = Fixture::new();
        match case {
            0=>f.manifest["sources"][0]["sidecar_path"]=Value::Null,
            1=>fs::write(f.dir.join("SYNTHETIC.txt.sha256"),format!("{}  SYNTHETIC.txt\n","0".repeat(64))).unwrap(),
            2=>fs::write(f.dir.join("SYNTHETIC.txt.sha256"),"invalid").unwrap(),
            3=>f.replace_log(b"BLOCK_ID=SYNTHETIC\nBLOCK_RESULT=PASS\n"),
            4=>f.replace_log(b"BLOCK_ID=SYNTHETIC\nBLOCK_ID=OTHER\nBLOCK_RESULT=PASS\nCOMMAND_EXIT_STATUS=0\nLOG_COMPLETE=YES\n"),
            5=>{let mut e=f.manifest["sources"][0].clone();e["id"]=json!("duplicate/SYNTHETIC.txt");let b=b"BLOCK_ID=SYNTHETIC\nBLOCK_RESULT=FAIL\nCOMMAND_EXIT_STATUS=1\nLOG_COMPLETE=YES\n";let p=f.dir.join("other");fs::create_dir(&p).unwrap();fs::write(p.join("SYNTHETIC.txt"),b).unwrap(); e["path"]=json!(p.join("SYNTHETIC.txt"));e["sha256"]=json!(format!("{:x}",Sha256::digest(b)));e["sidecar_path"]=Value::Null;f.manifest["sources"].as_array_mut().unwrap().push(e);},
            6=>{let a=f.manifest["sources"][1]["path"].clone(); f.manifest["sources"][1]["path"]=f.manifest["sources"][2]["path"].clone();f.manifest["sources"][2]["path"]=a;let h=f.manifest["sources"][1]["sha256"].clone();f.manifest["sources"][1]["sha256"]=f.manifest["sources"][2]["sha256"].clone();f.manifest["sources"][2]["sha256"]=h;},
            _=>unreachable!()
        }
        if case == 4 {
            let o = f.cli();
            assert!(!o.status.success());
            assert!(o.stdout.is_empty());
            continue;
        }
        let m = prepare_bundle(&f.bytes()).unwrap();
        assert_eq!(m["provenance"]["state"], "REVIEW_REQUIRED", "case {case}");
        if case < 6 {
            assert_eq!(m["payload"]["latest_block"]["runner"], "UNKNOWN");
        }
        assert_eq!(
            m["payload"]["last_verified"],
            f.manifest["previous"]["last_verified"]
        );
        for field in ["build", "current", "publication", "evidence"] {
            assert_eq!(m["payload"][field], f.manifest["previous"][field]);
        }
    }
}
