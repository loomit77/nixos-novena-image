use novena_bridge::objects::*;
use novena_bridge::protocol::*;
use novena_bridge::status::*;
use novena_bridge::{BridgeError, BridgeResult};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

#[cfg(unix)]
mod protocol_raw_files {
    use super::*;
    use novena_bridge::raw_commit_files::RawCommitFiles;
    use std::fs;
    use std::os::unix::fs::DirBuilderExt;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    type ObservedCalls = Rc<RefCell<Vec<String>>>;

    // Ausschließlich eigene synthetische Dateien; keine Git-/HTTP-Beschaffung.
    // Die Quelle wird erst nach abgeschlossener Einrichtung konstruiert.
    struct PrivateFiles(PathBuf);
    impl PrivateFiles {
        fn new(f: &Fixture) -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = PathBuf::from("/tmp").join(format!(
                "novena-readback-289-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            // Existierende Pfade niemals übernehmen, ändern oder bereinigen.
            fs::DirBuilder::new().mode(0o700).create(&path).unwrap();
            let files = Self(path);
            for (id, bytes) in &f.raw {
                fs::write(files.0.join(id.to_string()), bytes).unwrap();
            }
            files
        }

        fn reader(&self, f: &Fixture) -> (RestReader<impl ResponseSource>, ObservedCalls) {
            let mut records = f.records.clone();
            records.insert(
                format!("{PREFIX}/git/ref/heads/main"),
                json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}}),
            );
            let mut paths = BTreeMap::new();
            for (i, (endpoint, value)) in records.into_iter().enumerate() {
                let path = self.0.join(format!("rest-{i}.json"));
                fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
                paths.insert(endpoint, path);
            }
            let calls = Rc::new(RefCell::new(Vec::new()));
            let observed = calls.clone();
            let reader = RestReader::new(
                move |endpoint: &str| {
                    observed.borrow_mut().push(endpoint.to_owned());
                    let path = paths.get(endpoint).ok_or(BridgeError("MISSING_RESPONSE"))?;
                    fs::read(path).map_err(|_| BridgeError("SYNTHETIC_REST_IO"))
                },
                256,
            )
            .unwrap();
            (reader, calls)
        }

        fn source(&self) -> RawCommitFiles {
            RawCommitFiles::from_trusted_immutable_directory(&self.0).unwrap()
        }
    }
    impl Drop for PrivateFiles {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn assert_gates(expected: &PreparedStatus, original: &[u8]) {
        assert_eq!(expected.bytes(), original);
        let status: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert_eq!(status["last_verified"], Value::Null);
        assert_eq!(status["current"]["state"], "CURRENT_UNKNOWN");
        assert_eq!(status["latest_block"]["upload"], "UNKNOWN");
        assert_eq!(status["latest_block"]["professional_acceptance"], "UNKNOWN");
        assert_eq!(status["build"]["state"], "UNKNOWN");
        assert!(!status["review_required"].as_array().unwrap().is_empty());
        assert_eq!(status["publication"]["state"], "NOT_PUBLISHED");
        assert_eq!(status["publication"]["commit"], Value::Null);
        assert_eq!(
            production_publish(),
            Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
        );
    }

    #[test]
    fn raw_files_full_initial_readback_and_failures() {
        // Jede Variante erhält einen getrennten statischen Dateibaum.
        // Keine Mutation während Konstruktion/Nutzung von RawCommitFiles.
        for case in 0..8 {
            let mut f = Fixture::new(false);
            let expected = payload();
            let original = expected.bytes().to_vec();
            let files = PrivateFiles::new(&f);
            let commit = format!("{PREFIX}/git/commits/{}", f.after);
            let tree = format!("{PREFIX}/git/trees/{}", f.leaf);
            let blob = format!("{PREFIX}/git/blobs/{}", f.blob);
            let error = match case {
                0 => None,
                1 => {
                    let mut wrong = f.raw[&f.after].clone();
                    wrong.push(b'\n');
                    fs::write(files.0.join(f.after.to_string()), wrong).unwrap();
                    Some("COMMIT_HASH_MISMATCH")
                }
                2 => {
                    fs::remove_file(files.0.join(f.before.to_string())).unwrap();
                    Some("RAW_COMMIT_PATH_MISSING")
                }
                3 => {
                    f.records.get_mut(&commit).unwrap()["tree"]["sha"] = json!(f.leaf.to_string());
                    Some("COMMIT_METADATA_MISMATCH")
                }
                4 => {
                    f.records.get_mut(&commit).unwrap()["parents"] = json!([]);
                    Some("COMMIT_METADATA_MISMATCH")
                }
                5 => {
                    let mut changed = original.clone();
                    changed[0] ^= 1;
                    f.records.get_mut(&blob).unwrap()["content"] = json!(encode_base64(&changed));
                    Some("BLOB_HASH_MISMATCH")
                }
                6 => {
                    f.records.get_mut(&tree).unwrap()["tree"][0]["path"] = json!("different.json");
                    Some("TREE_HASH_MISMATCH")
                }
                7 => {
                    f.records.remove(&blob);
                    Some("MISSING_RESPONSE")
                }
                _ => unreachable!(),
            };
            let (mut reader, calls) = files.reader(&f);
            // Der Verifier hat keinen Zugriff mehr auf die Speicherrohbytes.
            f.raw.clear();
            let mut source = files.source();
            let result = verify_initial_index(
                &mut reader,
                &mut source,
                "SYNTHETIC_REPOSITORY_ID",
                f.before,
                f.after,
                &expected,
            );
            if let Some(error) = error {
                assert_eq!(result.unwrap_err(), BridgeError(error), "case {case}");
            } else {
                let report = result.unwrap();
                assert_eq!(report.commit, f.after);
                assert_eq!(report.blob, f.blob);
                assert!(report.commit_hash_verified);
                assert!(!report.publication_performed);
                assert_eq!(report.scope, "OFFLINE_SUPPLIED_RESPONSES");
                assert_eq!(report.as_json()["bridge_publication"], "BLOCKED");
                let calls = calls.borrow();
                assert_eq!(calls.first().unwrap(), PREFIX);
                let head = format!("{PREFIX}/git/ref/heads/main");
                assert_eq!(calls.last().unwrap(), &head);
                assert_eq!(calls.iter().filter(|p| *p == &head).count(), 2);
                for id in [f.before, f.after] {
                    assert!(calls.contains(&format!("{PREFIX}/git/commits/{id}")));
                }
                assert!(calls.contains(&blob));
            }
            assert_eq!(source.provenance(), "LOCAL_EXPLICIT_RAW_COMMIT_FILES");
            assert_gates(&expected, &original);
        }
    }

    #[test]
    fn raw_files_full_update_readback_preserves_unknown_and_checkpoint() {
        let (mut f, baseline, expected, before) = update_fixture();
        let original = expected.bytes().to_vec();
        let files = PrivateFiles::new(&f);
        let (mut reader, calls) = files.reader(&f);
        f.raw.clear();
        let report = verify_update_index(
            &mut reader,
            &mut files.source(),
            "SYNTHETIC_REPOSITORY_ID",
            before,
            f.after,
            &expected,
            &baseline,
        )
        .unwrap();
        assert!(report.commit_hash_verified);
        assert_eq!(report.commit, f.after);
        assert_eq!(report.blob, object_oid(ObjectKind::Blob, expected.bytes()));
        assert_eq!(report.scope, "OFFLINE_SUPPLIED_RESPONSES");
        assert!(!report.publication_performed);
        let calls = calls.borrow();
        for bytes in [baseline.as_slice(), expected.bytes()] {
            let id = object_oid(ObjectKind::Blob, bytes);
            assert!(calls.contains(&format!("{PREFIX}/git/blobs/{id}")));
        }
        let head = format!("{PREFIX}/git/ref/heads/main");
        assert_eq!(calls.last().unwrap(), &head);
        assert_eq!(calls.iter().filter(|p| *p == &head).count(), 2);
        assert_gates(&expected, &original);
    }
}

fn http_response(path: &str, body: &[u8]) -> OfflineHttpResponse {
    OfflineHttpResponse {
        method: "GET".into(),
        url: format!("https://api.github.com/{path}"),
        status: Some(200),
        location: None,
        elapsed: Some(std::time::Duration::from_millis(1)),
        body: Some(body.to_vec()),
    }
}
fn http_source(responses: Vec<OfflineHttpResponse>, limit: usize) -> OfflineHttpSource {
    OfflineHttpSource::new(
        responses,
        limit,
        8 * 1024 * 1024,
        std::time::Duration::from_secs(5),
    )
    .unwrap()
}

#[test]
fn rest_response_budget_accepts_exact_boundary_and_stops_before_next_source_call() {
    let calls = Rc::new(RefCell::new(0usize));
    let observed = calls.clone();
    let mut reader = RestReader::new(
        move |_: &str| {
            *observed.borrow_mut() += 1;
            let mut body = vec![b' '; 8 * 1024 * 1024];
            body[..2].copy_from_slice(b"{}");
            Ok(body)
        },
        256,
    )
    .unwrap();
    assert!(reader.get("not-allowlisted").is_err());
    assert_eq!(*calls.borrow(), 0);
    for path in [
        PREFIX,
        &format!("{PREFIX}/git/ref/heads/main"),
        PREFIX,
        PREFIX,
    ] {
        assert_eq!(reader.get(path).unwrap(), json!({}));
    }
    assert_eq!(reader.requests(), 4);
    for _ in 0..2 {
        assert_eq!(
            reader.get(PREFIX).unwrap_err(),
            BridgeError("RESPONSE_BUDGET_EXCEEDED")
        );
    }
    assert_eq!(*calls.borrow(), 4);
    assert_eq!(reader.requests(), 4);
}

#[test]
fn rest_response_budget_rejects_one_byte_over_and_remains_exhausted() {
    let calls = Rc::new(RefCell::new(0usize));
    let observed = calls.clone();
    let mut reader = RestReader::new(
        move |_: &str| {
            let mut count = observed.borrow_mut();
            *count += 1;
            let size = match *count {
                1..=3 => 8 * 1024 * 1024,
                4 => 8 * 1024 * 1024 - 1,
                _ => 2,
            };
            let mut body = vec![b' '; size];
            body[..2].copy_from_slice(b"{}");
            Ok(body)
        },
        256,
    )
    .unwrap();
    for _ in 0..4 {
        assert_eq!(reader.get(PREFIX).unwrap(), json!({}));
    }
    for _ in 0..2 {
        assert_eq!(
            reader.get(PREFIX).unwrap_err(),
            BridgeError("RESPONSE_BUDGET_EXCEEDED")
        );
    }
    assert_eq!(*calls.borrow(), 5);
    assert_eq!(reader.requests(), 5);
}

#[test]
fn rest_response_budget_charges_rejected_bodies() {
    for (size, prefix, expected, attempts) in [
        (8 * 1024 * 1024, b"xx", "INVALID_RESPONSE_JSON", 4),
        (8 * 1024 * 1024, b"[]", "INVALID_RESPONSE_SHAPE", 4),
        (8 * 1024 * 1024 + 1, b"{}", "RESPONSE_TOO_LARGE", 3),
    ] {
        let mut reader = RestReader::new(
            move |_: &str| {
                let mut body = vec![b' '; size];
                body[..2].copy_from_slice(prefix);
                Ok(body)
            },
            256,
        )
        .unwrap();
        for _ in 0..attempts {
            assert_eq!(reader.get(PREFIX).unwrap_err(), BridgeError(expected));
        }
        assert_eq!(
            reader.get(PREFIX).unwrap_err(),
            BridgeError("RESPONSE_BUDGET_EXCEEDED")
        );
        let requests = reader.requests();
        assert_eq!(
            reader.get(PREFIX).unwrap_err(),
            BridgeError("RESPONSE_BUDGET_EXCEEDED")
        );
        assert_eq!(reader.requests(), requests);
    }
}

#[test]
fn offline_http_envelope_policy_and_attempt_budget() {
    let good = br#"{"full_name":"loomit77/project-block-logs","node_id":"SYNTHETIC_ID"}"#;
    let mut reader = RestReader::new(http_source(vec![http_response(PREFIX, good)], 1), 2).unwrap();
    // An arbitrary URL is rejected before consuming the only accepted response.
    assert!(reader
        .get(&format!("https://api.github.com/{PREFIX}"))
        .is_err());
    assert_eq!(reader.requests(), 0);
    assert_eq!(reader.repository().unwrap(), "SYNTHETIC_ID");
    // Adapter budget is independent of the outer reader's budget.
    assert_eq!(
        reader.repository().unwrap_err(),
        BridgeError("REQUEST_LIMIT_EXCEEDED")
    );

    for method in ["POST", "PATCH", "PUT", "DELETE", "HEAD", "get"] {
        let mut response = http_response(PREFIX, good);
        response.method = method.into();
        assert_eq!(
            http_source(vec![response], 1).get(PREFIX).unwrap_err(),
            BridgeError("HTTP_METHOD_NOT_ALLOWED")
        );
    }
    for url in [
        format!("http://api.github.com/{PREFIX}"),
        format!("https://api.github.com.evil.invalid/{PREFIX}"),
        format!("https://secret@api.github.com/{PREFIX}"),
        format!("https://api.github.com:443/{PREFIX}"),
        format!("https://api.github.com/{PREFIX}?extra=1"),
        format!("https://example.invalid/{PREFIX}"),
    ] {
        let mut response = http_response(PREFIX, good);
        response.url = url;
        assert_eq!(
            http_source(vec![response], 1).get(PREFIX).unwrap_err(),
            BridgeError("HTTP_URL_NOT_ALLOWED")
        );
    }
    for (status, location, expected) in [
        (Some(302), None, "HTTP_REDIRECT_REJECTED"),
        (
            Some(200),
            Some("https://evil.invalid/secret"),
            "HTTP_REDIRECT_REJECTED",
        ),
        (
            Some(301),
            Some("https://api.github.com/elsewhere"),
            "HTTP_REDIRECT_REJECTED",
        ),
        (None, None, "HTTP_STATUS_REJECTED"),
        (Some(401), None, "HTTP_STATUS_REJECTED"),
        (Some(500), None, "HTTP_STATUS_REJECTED"),
        (Some(204), None, "HTTP_STATUS_REJECTED"),
        (Some(206), None, "HTTP_STATUS_REJECTED"),
    ] {
        let mut response = http_response(PREFIX, good);
        response.status = status;
        response.location = location.map(str::to_owned);
        let mut source = http_source(vec![response], 1);
        assert_eq!(source.get(PREFIX).unwrap_err(), BridgeError(expected));
        assert_eq!(
            source.get(PREFIX).unwrap_err(),
            BridgeError("REQUEST_LIMIT_EXCEEDED")
        );
    }
    let mut source = http_source(vec![], 1);
    assert_eq!(
        source.get(PREFIX).unwrap_err(),
        BridgeError("MISSING_RESPONSE")
    );
    assert_eq!(
        source.get(PREFIX).unwrap_err(),
        BridgeError("REQUEST_LIMIT_EXCEEDED")
    );
}

#[test]
fn offline_http_missing_malformed_size_and_recorded_timeout() {
    for body in [
        None,
        Some(vec![]),
        Some(b"invalid".to_vec()),
        Some(b"[]".to_vec()),
        Some(br#"{"a":1,"a":2}"#.to_vec()),
    ] {
        let mut response = http_response(PREFIX, b"{}");
        response.body = body;
        let mut reader = RestReader::new(http_source(vec![response], 1), 1).unwrap();
        assert!(reader.get(PREFIX).is_err());
    }
    for elapsed in [
        None,
        Some(std::time::Duration::from_secs(5)),
        Some(std::time::Duration::from_secs(6)),
    ] {
        let mut response = http_response(PREFIX, b"{}");
        response.elapsed = elapsed;
        assert!(http_source(vec![response], 1).get(PREFIX).is_err());
    }
    let timeout = std::time::Duration::from_secs(5);
    assert!(OfflineHttpSource::new(vec![http_response(PREFIX, b"{} ")], 1, 2, timeout).is_err());
    assert_eq!(
        OfflineHttpSource::new(vec![http_response(PREFIX, b"{}")], 1, 2, timeout)
            .unwrap()
            .get(PREFIX)
            .unwrap(),
        b"{}"
    );
    for (requests, bytes, seconds) in [
        (0, 1, 1),
        (257, 1, 1),
        (1, 0, 1),
        (1, 8 * 1024 * 1024 + 1, 1),
        (1, 1, 0),
        (1, 1, 61),
    ] {
        assert!(OfflineHttpSource::new(
            vec![],
            requests,
            bytes,
            std::time::Duration::from_secs(seconds)
        )
        .is_err());
    }
    assert!(OfflineHttpSource::new(
        vec![http_response(PREFIX, b"{}"), http_response(PREFIX, b"{}")],
        1,
        2,
        timeout
    )
    .is_err());
}

#[test]
fn offline_http_reuses_full_readback_without_state_promotion() {
    let f = Fixture::new(false);
    let expected = payload();
    let original = expected.bytes().to_vec();
    // Derive the transcript from the existing verifier, using only synthetic
    // fixture data. No second implementation of tree traversal or object checks.
    let mut transcript = Vec::new();
    {
        let mut recording = RestReader::new(
            |path: &str| {
                let value = if path == format!("{PREFIX}/git/ref/heads/main") {
                    json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}})
                } else {
                    f.records
                        .get(path)
                        .cloned()
                        .ok_or(BridgeError("MISSING_RESPONSE"))?
                };
                let body = serde_json::to_vec(&value).unwrap();
                transcript.push(http_response(path, &body));
                Ok(body)
            },
            256,
        )
        .unwrap();
        verify_initial_index(
            &mut recording,
            &mut raw(Rc::new(RefCell::new(f.clone()))),
            "SYNTHETIC_REPOSITORY_ID",
            f.before,
            f.after,
            &expected,
        )
        .unwrap();
    }
    // Run both the complete graph and a same-length blob content substitution
    // through the new boundary and the unchanged object verifier.
    for tamper in [false, true] {
        let responses = transcript
            .iter()
            .map(|r| {
                let body = r.body.as_ref().unwrap();
                let mut response =
                    http_response(r.url.strip_prefix("https://api.github.com/").unwrap(), body);
                if tamper
                    && r.url == format!("https://api.github.com/{PREFIX}/git/blobs/{}", f.blob)
                {
                    let mut v: Value = serde_json::from_slice(body).unwrap();
                    let mut changed = original.clone();
                    changed[0] ^= 1;
                    v["content"] = json!(encode_base64(&changed));
                    response.body = Some(serde_json::to_vec(&v).unwrap());
                }
                response
            })
            .collect();
        let result = verify_initial_index(
            &mut RestReader::new(http_source(responses, 256), 256).unwrap(),
            &mut raw(Rc::new(RefCell::new(f.clone()))),
            "SYNTHETIC_REPOSITORY_ID",
            f.before,
            f.after,
            &expected,
        );
        if tamper {
            assert_eq!(result.unwrap_err(), BridgeError("BLOB_HASH_MISMATCH"));
        } else {
            let report = result.unwrap().as_json();
            assert_eq!(report["publication_performed"], false);
            assert_eq!(report["bridge_publication"], "BLOCKED");
            assert_eq!(report["scope"], "OFFLINE_SUPPLIED_RESPONSES");
        }
        assert_eq!(expected.bytes(), original);
        let status: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert_eq!(status["last_verified"], Value::Null);
        assert_eq!(status["publication"]["state"], "NOT_PUBLISHED");
        assert_eq!(status["publication"]["commit"], Value::Null);
        assert_eq!(
            production_publish().unwrap_err(),
            BridgeError("PRODUCTION_PUBLICATION_DISABLED")
        );
    }
}

#[path = "../src/readback_fixture.rs"]
mod readback_fixture;
use readback_fixture::*;

mod object_evidence_295 {
    use super::*;
    use novena_bridge::object_evidence::{ByteOrigin, EvidenceKind};
    use novena_bridge::offline_readback::*;
    use sha2::{Digest, Sha256};

    fn fixture(message: &[u8]) -> Fixture {
        let mut f = Fixture::new(false);
        let side = object_oid(ObjectKind::Blob, b"preserved");
        f.records.insert(
            format!("{PREFIX}/git/blobs/{side}"),
            json!({
                "sha":side.to_string(),"size":9,"encoding":"base64",
                "content":encode_base64(b"preserved")
            }),
        );
        let middle = f.records[&format!(
            "{PREFIX}/git/trees/{}",
            inspect_commit(&f.raw[&f.after]).unwrap().tree
        )]["tree"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["path"] == "novena")
            .map(|e| Oid::parse(e["sha"].as_str().unwrap()).unwrap())
            .unwrap();
        let shared = vec![
            e("README.txt", Mode::File, side),
            e("copy", Mode::File, side),
        ];
        let old_tree = f.put_tree(shared.clone());
        f.before = f.put_commit(old_tree, None, "Baseline");
        let mut entries = shared;
        entries.push(e("novena", Mode::Directory, middle));
        let tree = f.put_tree(entries);
        let previous = f.put_commit(tree, Some(f.before), "replace");
        let mut bytes = f.raw[&previous].clone();
        let start = bytes.windows(2).position(|b| b == b"\n\n").unwrap() + 2;
        bytes.truncate(start);
        bytes.extend_from_slice(message);
        f.after = object_oid(ObjectKind::Commit, &bytes);
        let mut meta = f.records[&format!("{PREFIX}/git/commits/{previous}")].clone();
        meta["sha"] = json!(f.after.to_string());
        f.records
            .insert(format!("{PREFIX}/git/commits/{}", f.after), meta);
        f.raw.insert(f.after, bytes);
        f.head = f.after;
        f
    }
    fn run(f: Fixture, exhaust: bool) -> BridgeResult<CompleteReadback> {
        let before = f.before;
        let after = f.after;
        let f = Rc::new(RefCell::new(f));
        let budget = RunBudget::new();
        let source = f.clone();
        let mut rest = RestReader::new(
            budget.responses(move |path: &str| {
                let f = source.borrow();
                if path == format!("{PREFIX}/git/ref/heads/main") {
                    return Ok(serde_json::to_vec(&json!({"ref":REF,"object":{
                    "type":"commit","sha":f.head.to_string()}}))
                    .unwrap());
                }
                serde_json::to_vec(f.records.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                    .map_err(|_| BridgeError("SERIALIZATION"))
            }),
            256,
        )?;
        if exhaust {
            for _ in 0..256 {
                rest.repository()?;
            }
        }
        let mut raw = budget.commits(super::raw(f));
        let expected = payload();
        let original = expected.bytes().to_vec();
        let result = verify_complete_readback(
            &mut rest,
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            before,
            after,
            &expected,
            None,
            &budget,
        );
        assert_eq!(expected.bytes(), original);
        let document: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert!(document["last_verified"].is_null());
        assert_eq!(document["publication"]["state"], "NOT_PUBLISHED");
        assert_eq!(
            production_publish(),
            Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
        );
        if result.is_err() {
            assert_eq!(
                verify_complete_readback(
                    &mut rest,
                    &mut raw,
                    "SYNTHETIC_REPOSITORY_ID",
                    before,
                    after,
                    &expected,
                    None,
                    &budget
                )
                .unwrap_err(),
                BridgeError("RUN_ALREADY_FAILED")
            );
        }
        result
    }

    #[test]
    fn vollstaendiger_readback_deterministisches_eindeutiges_objektinventar() {
        let f = fixture(b"scope=LIVE_PROVENANCE\n\xff\n\n");
        let first = run(f.clone(), false).unwrap();
        let second = run(f.clone(), false).unwrap();
        assert_eq!(first.evidence().bytes(), second.evidence().bytes());
        assert_eq!(first.evidence().scope(), RunScope::Offline);
        assert!(!first.report().publication_performed);
        assert_eq!(first.complete().paths(), 5);
        let items = first.evidence().objects();
        assert_eq!(items.len(), 8); // zwei Commits, vier Trees, zwei Blobs
        assert_eq!(
            items
                .iter()
                .filter(|o| o.kind() == EvidenceKind::Blob)
                .count(),
            2
        );
        assert!(items
            .windows(2)
            .all(|w| (w[0].kind(), w[0].oid()) < (w[1].kind(), w[1].oid())));
        for item in items {
            let body = match item.kind() {
                EvidenceKind::Commit => {
                    assert_eq!(item.byte_origin(), ByteOrigin::OriginalCommitPayload);
                    f.raw[&item.oid()].clone()
                }
                EvidenceKind::Tree => {
                    assert_eq!(item.byte_origin(), ByteOrigin::CanonicallyReconstructedTree);
                    let record = &f.records[&format!("{PREFIX}/git/trees/{}", item.oid())];
                    let entries: Vec<_> = record["tree"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| TreeEntry {
                            name: v["path"].as_str().unwrap().into(),
                            mode: Mode::from_api(
                                v["type"].as_str().unwrap(),
                                v["mode"].as_str().unwrap(),
                            )
                            .unwrap(),
                            oid: Oid::parse(v["sha"].as_str().unwrap()).unwrap(),
                        })
                        .collect();
                    tree_bytes(&entries).unwrap()
                }
                EvidenceKind::Blob => {
                    assert_eq!(item.byte_origin(), ByteOrigin::DecodedBlobPayload);
                    if item.oid() == f.blob {
                        payload().bytes().to_vec()
                    } else {
                        b"preserved".to_vec()
                    }
                }
            };
            assert_eq!(item.length(), body.len());
            assert_eq!(
                *item.payload_sha256(),
                <[u8; 32]>::from(Sha256::digest(&body))
            );
        }
        let value: Value = serde_json::from_slice(first.evidence().bytes()).unwrap();
        assert_eq!(value["scope"], "OFFLINE_SUPPLIED_RESPONSES");
        assert_eq!(value["object_count"], items.len());
        assert_eq!(value["transport_provenance"], "NOT_AUTHENTICATED");
        let entry = value["original_commits"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["git_oid"] == f.after.to_string())
            .unwrap();
        assert_eq!(entry["payload_base64"], encode_base64(&f.raw[&f.after]));
    }

    #[test]
    fn geaenderte_commitbytes_aendern_sha256_und_git_oid() {
        let a = fixture(b"message\n");
        let b = fixture(b"message\n\n");
        let ra = run(a.clone(), false).unwrap();
        let rb = run(b.clone(), false).unwrap();
        let ca = ra
            .evidence()
            .objects()
            .iter()
            .find(|o| o.oid() == a.after)
            .unwrap();
        let cb = rb
            .evidence()
            .objects()
            .iter()
            .find(|o| o.oid() == b.after)
            .unwrap();
        assert_ne!(ca.oid(), cb.oid());
        assert_ne!(ca.payload_sha256(), cb.payload_sha256());
        assert_ne!(ra.evidence().bytes(), rb.evidence().bytes());
    }

    fn encoded(v: &Value) -> Vec<u8> {
        novena_bridge::canonical_json(&serde_json::to_vec(v).unwrap()).unwrap()
    }

    struct HandoffDir(std::path::PathBuf);
    impl HandoffDir {
        fn new() -> Self {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "novena-299-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            use std::os::unix::fs::DirBuilderExt;
            std::fs::DirBuilder::new()
                .mode(0o700)
                .create(&path)
                .unwrap();
            Self(path)
        }
    }
    impl Drop for HandoffDir {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    fn file_replay(
        original: &CompleteReadback,
        case: usize,
    ) -> BridgeResult<novena_bridge::offline_run::BoundOfflineReplay> {
        use novena_bridge::offline_run::OfflineRun;
        let dir = HandoffDir::new();
        let value: Value = serde_json::from_slice(original.package().bytes()).unwrap();
        let before = Oid::parse(value["before"].as_str().unwrap()).unwrap();
        let after = Oid::parse(value["after"].as_str().unwrap()).unwrap();
        let expected = payload();
        let mut claims: Vec<_> = original
            .evidence()
            .objects()
            .iter()
            .filter(|o| o.kind() == EvidenceKind::Commit)
            .map(|o| ExportClaim {
                oid: o.oid(),
                length: o.length(),
                payload_sha256: *o.payload_sha256(),
            })
            .collect();
        for c in value["original_commits"].as_array().unwrap() {
            std::fs::write(
                dir.0.join(c["git_oid"].as_str().unwrap()),
                novena_bridge::protocol::decode_base64(c["payload_base64"].as_str().unwrap())
                    .unwrap(),
            )
            .unwrap();
        }
        let path = dir.0.join(claims[0].oid.to_string());
        let budget = RunBudget::new();
        let spec = ReadbackSpec::new(
            "SYNTHETIC_REPOSITORY_ID",
            1,
            after,
            ReadbackMode::Snapshot {
                expected: expected.bytes(),
            },
        )
        .unwrap();
        let digest = Sha256::digest(original.package().bytes()).into();
        let run = OfflineRun::new(spec, before, "synthetic-299", digest, &budget).unwrap();
        let mut manifest: Value =
            serde_json::from_slice(&run.manifest_bytes(original).unwrap()).unwrap();
        let mut package = value.clone();
        let mut selected = dir.0.clone();
        match case {
            0 => {}
            1 => {
                claims[0].payload_sha256 = [0; 32];
            }
            2 => {
                std::fs::remove_file(&path).unwrap();
            }
            3 => {
                std::fs::write(dir.0.join("unexpected"), b"x").unwrap();
            }
            4 => {
                let mut b = std::fs::read(&path).unwrap();
                *b.last_mut().unwrap() = b'x';
                std::fs::write(&path, b).unwrap();
            }
            5 => {
                let mut b = std::fs::read(&path).unwrap();
                b.pop();
                std::fs::write(&path, b).unwrap();
            }
            6 => {
                std::fs::remove_file(&path).unwrap();
                std::os::unix::fs::symlink("/etc/passwd", &path).unwrap();
            }
            7 => {
                selected = dir.0.join("..");
            }
            8 => {
                std::fs::write(&path, vec![b'x'; 1024 * 1024 + 1]).unwrap();
            }
            9 => {
                manifest["repository"] = json!("other/repo");
            }
            10 => {
                manifest["reference"] = json!("refs/heads/other");
            }
            11 => {
                manifest["offline_run_id"] = json!("other-run");
            }
            12 => {
                manifest["contract"] = json!("NOVENA_OFFLINE_RUN_V2");
            }
            13 => {
                manifest["provenance"] = json!("AUTHENTICATED_GITHUB");
            }
            14 => {
                claims[1].oid = claims[0].oid;
            }
            15 => {
                let mut b = std::fs::read(&path).unwrap();
                *b.last_mut().unwrap() = b'x';
                claims[0].payload_sha256 = Sha256::digest(&b).into();
                std::fs::write(&path, b).unwrap();
            }
            16 => {
                std::fs::remove_file(&path).unwrap();
                std::fs::create_dir(&path).unwrap();
            }
            17 => {
                std::fs::hard_link(&path, dir.0.join("alias")).unwrap();
            }
            18 => {
                for response in package["responses"].as_array_mut().unwrap() {
                    if response["endpoint"]
                        .as_str()
                        .unwrap()
                        .contains("/git/blobs/")
                    {
                        let mut body: Value = serde_json::from_slice(
                            &novena_bridge::protocol::decode_base64(
                                response["body_base64"].as_str().unwrap(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                        if body["content"] == encode_base64(b"preserved") {
                            body["content"] = json!(encode_base64(b"tampered!"));
                            response["body_base64"] =
                                json!(encode_base64(&serde_json::to_vec(&body).unwrap()));
                        }
                    }
                }
            }
            19 => {
                for response in package["responses"].as_array_mut().unwrap() {
                    if response["endpoint"]
                        .as_str()
                        .unwrap()
                        .contains("/git/trees/")
                    {
                        let mut body: Value = serde_json::from_slice(
                            &novena_bridge::protocol::decode_base64(
                                response["body_base64"].as_str().unwrap(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                        body["truncated"] = json!(true);
                        response["body_base64"] =
                            json!(encode_base64(&serde_json::to_vec(&body).unwrap()));
                    }
                }
            }
            20 => {
                let bytes = std::fs::read(&path).unwrap();
                let mut source = budget.commits(|_: Oid| Ok(bytes.clone()));
                source.raw_commit(claims[0].oid).unwrap();
                source.raw_commit(claims[0].oid).unwrap();
                assert_eq!(
                    source.raw_commit(claims[0].oid).unwrap_err(),
                    BridgeError("RUN_BUDGET_EXCEEDED")
                );
            }
            21 => {
                manifest["before"] = json!("0".repeat(40));
            }
            22 => {
                manifest["after"] = json!("0".repeat(40));
            }
            23 => {
                claims[0].length = 1024 * 1024 + 1;
            }
            27 => {
                let bytes = std::fs::read(&path).unwrap();
                let mut source = budget.commits(|_: Oid| Ok(bytes.clone()));
                source.raw_commit(claims[0].oid).unwrap();
            }
            26 => {
                let mut reader = RestReader::new(
                    budget.responses(|_: &str| {
                        Ok(serde_json::to_vec(
                            &json!({"full_name":ARCHIVE,"node_id":"SYNTHETIC_REPOSITORY_ID"}),
                        )
                        .unwrap())
                    }),
                    256,
                )
                .unwrap();
                reader.repository().unwrap();
            }
            25 => {}
            24 => {
                std::fs::rename(&dir.0, dir.0.with_extension("real")).unwrap();
                std::os::unix::fs::symlink(dir.0.with_extension("real"), &dir.0).unwrap();
            }
            _ => unreachable!(),
        }
        if matches!(case, 1 | 15) {
            manifest["original_commits"][0]["payload_sha256"] = json!(claims[0]
                .payload_sha256
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>());
        }
        let bytes = encoded(&package);
        let spec = ReadbackSpec::new(
            "SYNTHETIC_REPOSITORY_ID",
            1,
            after,
            ReadbackMode::Snapshot {
                expected: expected.bytes(),
            },
        )
        .unwrap();
        // Anpassung des äußeren Digests erreicht bei 18/19 den Objektprüfer.
        let run = OfflineRun::new(
            spec,
            before,
            "synthetic-299",
            Sha256::digest(&bytes).into(),
            &budget,
        );
        let result = run.and_then(|run| {
            manifest["package_sha256"] = json!(format!("{:x}", Sha256::digest(&bytes)));
            let mut original_manifest = encoded(&manifest);
            if case == 25 {
                original_manifest.push(b' ');
            }
            let manifest = AcquisitionManifest::untrusted(&original_manifest, &claims)?;
            let result = run.verify_files(&manifest, &bytes, &selected);
            if result.is_err() {
                assert_eq!(
                    run.verify_files(&manifest, &bytes, &selected).unwrap_err(),
                    BridgeError("RUN_ALREADY_FAILED")
                );
            }
            result
        });
        if case == 24 {
            std::fs::remove_file(&dir.0).unwrap();
            std::fs::rename(dir.0.with_extension("real"), &dir.0).unwrap();
        }
        assert_eq!(expected.bytes(), payload().bytes());
        assert!(
            serde_json::from_slice::<Value>(expected.bytes()).unwrap()["last_verified"].is_null()
        );
        assert_eq!(
            production_publish(),
            Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
        );
        result
    }
    #[test]
    fn offline_299_file_handoff_negative_matrix() {
        let original = run(fixture(b"299 original\n"), false).unwrap();
        for case in 1..26 {
            let error = file_replay(&original, case).unwrap_err();
            let expected = match case {
                1 | 4 => Some("RAW_HANDOFF_DIGEST"),
                5 | 8 | 16 => Some("RAW_HANDOFF_FILE"),
                15 => Some("COMMIT_HASH_MISMATCH"),
                18 => Some("BLOB_HASH_MISMATCH"),
                19 => Some("INCOMPLETE_OR_WRONG_TREE"),
                9..=13 | 21 | 22 | 25 => Some("RUN_MANIFEST_MISMATCH"),
                _ => None,
            };
            if let Some(expected) = expected {
                assert_eq!(error, BridgeError(expected), "case {case}");
            }
        }
        assert_eq!(
            file_replay(&original, 27).unwrap_err(),
            BridgeError("RUN_BUDGET_EXCEEDED")
        );
    }
    #[test]
    fn offline_299_export_import_independent_file_replays() {
        let original = run(fixture(b"299 original\n"), false).unwrap();
        let dir = HandoffDir::new();
        novena_bridge::offline_package::export_private(original.package(), &dir.0).unwrap();
        let imported = novena_bridge::offline_package::import_private(&dir.0).unwrap();
        let a = file_replay(&imported, 0).unwrap();
        let b = file_replay(&imported, 0).unwrap();
        assert_eq!(
            novena_bridge::offline_run::compare_replays(&a, &b).unwrap(),
            "REPLAY_CONSISTENT_OFFLINE"
        );
        assert!(!a.complete().report().publication_performed);
        let different_usage = file_replay(&imported, 26).unwrap();
        assert!(novena_bridge::offline_run::compare_replays(&a, &different_usage).is_err());
        assert!(file_replay(&imported, 18).is_err());
        assert!(file_replay(&imported, 4).is_err());
        let different = run(fixture(b"299 different\n"), false).unwrap();
        let c = file_replay(&different, 0).unwrap();
        assert!(novena_bridge::offline_run::compare_replays(&a, &c).is_err());
    }
    fn bound_replay(
        original: &CompleteReadback,
        package: &[u8],
        mutation: impl FnOnce(&mut Value, &mut Vec<ExportClaim>),
        exhaust: bool,
    ) -> BridgeResult<novena_bridge::offline_run::BoundOfflineReplay> {
        use novena_bridge::offline_run::OfflineRun;
        let p: Value = serde_json::from_slice(original.package().bytes()).unwrap();
        let before = Oid::parse(p["before"].as_str().unwrap()).unwrap();
        let after = Oid::parse(p["after"].as_str().unwrap()).unwrap();
        let expected = payload();
        let budget = RunBudget::new();
        let spec = ReadbackSpec::new(
            "SYNTHETIC_REPOSITORY_ID",
            1,
            after,
            ReadbackMode::Snapshot {
                expected: expected.bytes(),
            },
        )
        .unwrap();
        let run = OfflineRun::new(
            spec,
            before,
            "synthetic-297",
            Sha256::digest(original.package().bytes()).into(),
            &budget,
        )
        .unwrap();
        let mut manifest: Value =
            serde_json::from_slice(&run.manifest_bytes(original).unwrap()).unwrap();
        // Geänderte Paketbytes dürfen den Digest passieren, damit die Tests
        // tatsächlich den bestehenden Objektverifier und nicht nur SHA256 prüfen.
        let digest: [u8; 32] = Sha256::digest(package).into();
        manifest["package_sha256"] = json!(digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>());
        let mut claims: Vec<_> = original
            .evidence()
            .objects()
            .iter()
            .filter(|o| o.kind() == EvidenceKind::Commit)
            .map(|o| ExportClaim {
                oid: o.oid(),
                length: o.length(),
                payload_sha256: *o.payload_sha256(),
            })
            .collect();
        mutation(&mut manifest, &mut claims);
        let spec = ReadbackSpec::new(
            "SYNTHETIC_REPOSITORY_ID",
            1,
            after,
            ReadbackMode::Snapshot {
                expected: expected.bytes(),
            },
        )
        .unwrap();
        let run = OfflineRun::new(spec, before, "synthetic-297", digest, &budget).unwrap();
        if exhaust {
            let mut reader = RestReader::new(
                budget.responses(|_: &str| {
                    Ok(serde_json::to_vec(
                        &json!({"full_name":ARCHIVE,"node_id":"SYNTHETIC_REPOSITORY_ID"}),
                    )
                    .unwrap())
                }),
                256,
            )
            .unwrap();
            // Vollständige zulässige Abrufe verbrauchen den gemeinsamen Kontext.
            for _ in 0..256 {
                reader.repository()?;
            }
        }
        let bytes = encoded(&manifest);
        let manifest = AcquisitionManifest::untrusted(&bytes, &claims)?;
        let result = run.verify(&manifest, package);
        if result.is_err() {
            assert_eq!(
                run.verify(&manifest, package).unwrap_err(),
                BridgeError("RUN_ALREADY_FAILED")
            );
        }
        result
    }

    #[test]
    fn offline_297_laufbindung_manifest_negativmatrix() {
        let original = run(fixture(b"297 deterministic\n"), false).unwrap();
        let bytes = original.package().bytes();
        let first = bound_replay(&original, bytes, |_, _| {}, false).unwrap();
        let second = bound_replay(&original, bytes, |_, _| {}, false).unwrap();
        assert_eq!(
            novena_bridge::offline_run::compare_replays(&first, &second).unwrap(),
            "REPLAY_CONSISTENT_OFFLINE"
        );
        assert!(novena_bridge::offline_run::compare_replays(&first, &first).is_err());
        assert_eq!(first.scope(), RunScope::Offline);
        for case in 0..21 {
            let result = bound_replay(
                &original,
                bytes,
                |m, claims| match case {
                    0 => m["repository"] = json!("other/repo"),
                    1 => m["reference"] = json!("refs/heads/other"),
                    2 => m["before"] = json!("0".repeat(40)),
                    3 => m["after"] = json!("0".repeat(40)),
                    4 => m["observations"].as_array_mut().unwrap().swap(0, 1),
                    5 => m["package_sha256"] = json!("0".repeat(64)),
                    6 => m["contract"] = json!("NOVENA_OFFLINE_RUN_V2"),
                    7 => m["provenance"] = json!("AUTHENTICATED_GITHUB"),
                    8 => m["scope"] = json!("LIVE"),
                    9 => m["original_commits"][0]["length"] = json!(0),
                    10 => {
                        m["original_commits"].as_array_mut().unwrap().pop();
                    }
                    11 => {
                        let mut o = m["original_commits"][0].clone();
                        o["length"] = json!(0);
                        m["original_commits"].as_array_mut().unwrap().push(o);
                    }
                    12 => claims[0].length += 1,
                    13 => claims[0].payload_sha256[0] ^= 1,
                    14 => m["offline_run_id"] = json!("other-run"),
                    15 => m["mode"] = json!("update"),
                    16 => m["repository_node_id"] = json!("WRONG_ID"),
                    17 => m["expected_numeric_id"] = json!(2),
                    18 => m["budget_contract"] = json!("UNLIMITED"),
                    19 => m["package_contract"] = json!("NOVENA_OFFLINE_REPLAY_V2"),
                    20 => claims.swap(0, 1),
                    _ => unreachable!(),
                },
                false,
            );
            assert!(result.is_err(), "Manifestmutation {case}");
        }
        assert_eq!(
            bound_replay(&original, bytes, |_, _| {}, true).unwrap_err(),
            BridgeError("RUN_BUDGET_EXCEEDED")
        );
        let other = run(fixture(b"297 different original bytes\n"), false).unwrap();
        let other = bound_replay(&other, other.package().bytes(), |_, _| {}, false).unwrap();
        assert!(novena_bridge::offline_run::compare_replays(&first, &other).is_err());
        assert_eq!(first.complete().package().bytes(), bytes);
        assert!(!first.complete().report().publication_performed);
        let status: Value = serde_json::from_slice(payload().bytes()).unwrap();
        assert_eq!(status["last_verified"], Value::Null);
        assert_eq!(status["publication"]["state"], "NOT_PUBLISHED");
        assert!(production_publish().is_err());
    }

    #[test]
    fn offline_297_objektfehler_geben_keinen_gebundenen_teilbeleg() {
        let original = run(fixture(b"297 object verification\n"), false).unwrap();
        let pristine: Value = serde_json::from_slice(original.package().bytes()).unwrap();
        let side = object_oid(ObjectKind::Blob, b"preserved").to_string();
        for case in 0..11 {
            let mut p = pristine.clone();
            match case {
                0 => {
                    let mut raw =
                        decode_base64(p["original_commits"][0]["payload_base64"].as_str().unwrap())
                            .unwrap();
                    raw.push(b'\n');
                    p["original_commits"][0]["payload_base64"] = json!(encode_base64(&raw));
                }
                1 => {
                    p["original_commits"].as_array_mut().unwrap().pop();
                }
                2 | 3 => {
                    let r = p["responses"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|r| {
                            let path = r["endpoint"].as_str().unwrap();
                            if case == 2 {
                                path.ends_with(&side)
                            } else {
                                path.contains("/git/trees/")
                            }
                        })
                        .unwrap();
                    let raw = decode_base64(r["body_base64"].as_str().unwrap()).unwrap();
                    let mut body: Value = serde_json::from_slice(&raw).unwrap();
                    if case == 2 {
                        body["content"] = json!(encode_base64(b"tampered!"));
                    } else {
                        body["tree"].as_array_mut().unwrap().pop();
                    }
                    r["body_base64"] = json!(encode_base64(&serde_json::to_vec(&body).unwrap()));
                }
                4 => p["inventory"]["objects"][0]["payload_sha256"] = json!("0".repeat(64)),
                5 => p["contract"] = json!("NOVENA_OFFLINE_REPLAY_V2"),
                6 => {
                    let mut o = p["inventory"]["objects"][0].clone();
                    o["length"] = json!(0);
                    p["inventory"]["objects"].as_array_mut().unwrap().push(o);
                }
                7 => p["scope"] = json!("LIVE"),
                8 => {
                    let mut extra = p["original_commits"][0].clone();
                    extra["git_oid"] = json!("0".repeat(40));
                    p["original_commits"].as_array_mut().unwrap().push(extra);
                }
                9 => p["inventory"]["objects"][0]["length"] = json!(0),
                10 => p["responses"].as_array_mut().unwrap().swap(0, 1),
                _ => unreachable!(),
            }
            assert!(
                bound_replay(&original, &encoded(&p), |_, _| {}, false).is_err(),
                "Objektmutation {case}"
            );
        }
    }

    #[test]
    fn offline_update_paket_mit_vollstaendigen_nebenblobs() {
        use novena_bridge::offline_package::verify_package;
        let (mut f, baseline, expected, before) = update_fixture();
        let side = object_oid(ObjectKind::Blob, b"preserved");
        f.records.insert(format!("{PREFIX}/git/blobs/{side}"), json!({
            "sha":side.to_string(),"size":9,"encoding":"base64","content":encode_base64(b"preserved")
        }));
        let after = f.after;
        let shared = Rc::new(RefCell::new(f));
        let source = shared.clone();
        let budget = RunBudget::new();
        let mut reader = RestReader::new(
            budget.responses(move |p: &str| {
                if p.ends_with("/git/ref/heads/main") {
                    return Ok(serde_json::to_vec(
                        &json!({"ref":REF,"object":{"type":"commit","sha":after.to_string()}}),
                    )
                    .unwrap());
                }
                Ok(serde_json::to_vec(
                    source
                        .borrow()
                        .records
                        .get(p)
                        .ok_or(BridgeError("MISSING_RESPONSE"))?,
                )
                .unwrap())
            }),
            256,
        )
        .unwrap();
        let mut raw = budget.commits(super::raw(shared));
        // Vorherige Abrufe belasten denselben Kontext; das Paket enthält
        // trotzdem genau die nachfolgende vollständige Readback-Sequenz.
        reader.repository().unwrap();
        let result = verify_complete_readback(
            &mut reader,
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            before,
            after,
            &expected,
            Some(&baseline),
            &budget,
        )
        .unwrap();
        let replay = verify_package(result.package().bytes()).unwrap();
        assert_eq!(result.evidence().bytes(), replay.evidence().bytes());
        assert_eq!(replay.complete().blobs().len(), 3);
        use novena_bridge::offline_run::{compare_replays, OfflineRun};
        let claims: Vec<_> = result
            .evidence()
            .objects()
            .iter()
            .filter(|o| o.kind() == EvidenceKind::Commit)
            .map(|o| ExportClaim {
                oid: o.oid(),
                length: o.length(),
                payload_sha256: *o.payload_sha256(),
            })
            .collect();
        let budgets = [RunBudget::new(), RunBudget::new()];
        let runs: Vec<_> = budgets
            .iter()
            .map(|budget| {
                let spec = ReadbackSpec::new(
                    "SYNTHETIC_REPOSITORY_ID",
                    1,
                    after,
                    ReadbackMode::Transition {
                        baseline: before,
                        old: &baseline,
                        new: expected.bytes(),
                    },
                )
                .unwrap();
                OfflineRun::new(
                    spec,
                    before,
                    "update-297",
                    Sha256::digest(result.package().bytes()).into(),
                    budget,
                )
                .unwrap()
            })
            .collect();
        let manifest = runs[0].manifest_bytes(&result).unwrap();
        assert_eq!(manifest, runs[1].manifest_bytes(&result).unwrap());
        let manifest = AcquisitionManifest::untrusted(&manifest, &claims).unwrap();
        let a = runs[0].verify(&manifest, result.package().bytes()).unwrap();
        let b = runs[1].verify(&manifest, result.package().bytes()).unwrap();
        assert_eq!(
            compare_replays(&a, &b).unwrap(),
            "REPLAY_CONSISTENT_OFFLINE"
        );
        let old: Value = serde_json::from_slice(&baseline).unwrap();
        let new: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert_eq!(old["last_verified"], new["last_verified"]);
        assert_eq!(new["publication"]["state"], "NOT_PUBLISHED");
        assert!(production_publish().is_err());
    }

    #[test]
    fn offline_paket_wiederpruefung_und_fail_closed_matrix() {
        use novena_bridge::offline_package::verify_package;
        let first = run(fixture(b"OFFLINE\n\xff\nLIVE_PROVENANCE\n"), false).unwrap();
        let second = run(fixture(b"OFFLINE\n\xff\nLIVE_PROVENANCE\n"), false).unwrap();
        let bytes = first.package().bytes();
        assert_eq!(bytes, second.package().bytes());
        let replay = verify_package(bytes).unwrap();
        assert_eq!(replay.package().bytes(), bytes);
        assert_eq!(replay.evidence().bytes(), first.evidence().bytes());
        assert!(!replay.report().publication_performed);
        assert_eq!(replay.scope(), RunScope::Offline);
        assert!(production_publish().is_err());
        assert!(verify_package(first.evidence().bytes()).is_err());
        let original: Value = serde_json::from_slice(bytes).unwrap();
        // REST-Bodies dürfen größer als ein Blob sein. Exakte originale
        // JSON-Whitespacebytes bleiben auch über die gemeinsame Decoder-API erhalten.
        let mut large_body_package = original.clone();
        let mut body = decode_base64(
            large_body_package["responses"][0]["body_base64"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        body.resize(1024 * 1024 + 1, b' ');
        large_body_package["responses"][0]["body_base64"] = json!(encode_base64(&body));
        assert!(verify_package(&encoded(&large_body_package)).is_ok());
        let mut large_commit_package = original.clone();
        large_commit_package["original_commits"][0]["payload_base64"] =
            json!("A".repeat(1_398_108));
        assert!(verify_package(&encoded(&large_commit_package)).is_err());
        let side = object_oid(ObjectKind::Blob, b"preserved").to_string();
        // Jede Mutation wird einzeln gegen dasselbe vollständige Original geprüft.
        for case in 0..23 {
            let mut v = original.clone();
            match case {
                0 => v["contract"] = json!("NOVENA_OFFLINE_REPLAY_V2"),
                1 => v["scope"] = json!("LIVE"),
                2 => v["verified"] = json!(true),
                3 => v["inventory"]["objects"][0]["length"] = json!(0),
                4 => v["inventory"]["objects"][0]["payload_sha256"] = json!("0".repeat(64)),
                5 => v["inventory"]["objects"][0]["git_oid"] = json!("0".repeat(40)),
                6 => v["inventory"]["objects"][0]["type"] = json!("blob"),
                7 => {
                    v["inventory"]["objects"].as_array_mut().unwrap().pop();
                }
                8 => {
                    let duplicate = v["inventory"]["objects"][0].clone();
                    v["inventory"]["objects"]
                        .as_array_mut()
                        .unwrap()
                        .push(duplicate);
                }
                9 => v["inventory"]["objects"].as_array_mut().unwrap().swap(0, 1),
                10 => v["inventory"]["transport_provenance"] = json!("AUTHENTICATED_GITHUB"),
                11 => {
                    let raw =
                        decode_base64(v["original_commits"][0]["payload_base64"].as_str().unwrap())
                            .unwrap();
                    let mut raw = raw;
                    raw.push(b'\n');
                    v["original_commits"][0]["payload_base64"] = json!(encode_base64(&raw));
                }
                12 => {
                    v["original_commits"].as_array_mut().unwrap().pop();
                }
                13 => v["original_commits"][1] = v["original_commits"][0].clone(),
                14 => v["original_commits"].as_array_mut().unwrap().swap(0, 1),
                15 => {
                    v["responses"].as_array_mut().unwrap().pop();
                }
                16 => v["responses"].as_array_mut().unwrap().swap(0, 1),
                17 => v["inventory"]["object_count"] = json!(1),
                18..=21 => {
                    let responses = v["responses"].as_array_mut().unwrap();
                    let r = responses
                        .iter_mut()
                        .rev()
                        .find(|r| {
                            let p = r["endpoint"].as_str().unwrap();
                            match case {
                                18 => p.contains("/git/trees/"),
                                19 => p.ends_with(&side),
                                _ => p.ends_with("/git/ref/heads/main"),
                            }
                        })
                        .unwrap();
                    let body = decode_base64(r["body_base64"].as_str().unwrap()).unwrap();
                    let mut body: Value = serde_json::from_slice(&body).unwrap();
                    match case {
                        18 => body["tree"][0]["sha"] = json!("0".repeat(40)),
                        19 => body["content"] = json!(encode_base64(b"tampered!")),
                        20 => body["object"]["sha"] = original["before"].clone(),
                        _ => body["object"]["type"] = json!("blob"),
                    }
                    r["body_base64"] = json!(encode_base64(&serde_json::to_vec(&body).unwrap()));
                }
                22 => v["inventory"]["unknown_required_field"] = json!(1),
                _ => unreachable!(),
            }
            assert!(verify_package(&encoded(&v)).is_err(), "Mutation {case}");
            assert_eq!(first.package().bytes(), bytes);
        }
        let duplicate = String::from_utf8(bytes.to_vec()).unwrap().replacen(
            '{',
            "{\"scope\":\"OFFLINE_ONLY\",",
            1,
        );
        assert!(verify_package(duplicate.as_bytes()).is_err());
        let mut whitespace = bytes.to_vec();
        whitespace.push(b' ');
        assert!(verify_package(&whitespace).is_err());
        let mut depth = vec![b'['; 130];
        depth.extend(std::iter::repeat_n(b']', 130));
        assert!(verify_package(&depth).is_err());
        let oversized = vec![b' '; novena_bridge::offline_package::MAX_PACKAGE_BYTES + 1];
        assert_eq!(
            verify_package(&oversized).unwrap_err(),
            BridgeError("PACKAGE_LIMIT")
        );
        let mut count = original;
        let response = count["responses"][0].clone();
        count["responses"] = json!(vec![response; 257]);
        assert_eq!(
            verify_package(&encoded(&count)).unwrap_err(),
            BridgeError("PACKAGE_LIMIT")
        );
        assert!(payload()
            .bytes()
            .windows(b"\"last_verified\":null".len())
            .any(|s| s == b"\"last_verified\":null"));
    }

    #[cfg(unix)]
    #[test]
    fn private_dateiuebergabe_ohne_ueberschreiben_und_fremdpfade() {
        use novena_bridge::offline_package::{export_private, import_private};
        use std::os::unix::fs::{symlink, DirBuilderExt};
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::path::PathBuf::from("/tmp").join(format!(
            "novena-evidence-296-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::DirBuilder::new()
            .mode(0o700)
            .create(&path)
            .unwrap();
        let first = run(fixture(b"private\n"), false).unwrap();
        assert!(import_private(&path).is_err());
        export_private(first.package(), &path).unwrap();
        let replay = import_private(&path).unwrap();
        assert_eq!(first.package().bytes(), replay.package().bytes());
        assert!(export_private(first.package(), &path).is_err());
        let file = path.join("novena-offline-replay-v1.json");
        std::fs::write(path.join("unknown"), b"x").unwrap();
        assert!(import_private(&path).is_err());
        std::fs::remove_file(path.join("unknown")).unwrap();
        std::fs::remove_file(&file).unwrap();
        std::fs::create_dir(&file).unwrap();
        assert!(import_private(&path).is_err());
        std::fs::remove_dir(&file).unwrap();
        symlink("missing", &file).unwrap();
        assert!(import_private(&path).is_err());
        assert!(export_private(first.package(), &path).is_err());
        std::fs::remove_file(&file).unwrap();
        std::fs::create_dir(path.join(".git")).unwrap();
        assert!(export_private(first.package(), &path).is_err());
        assert!(import_private(&path).is_err());
        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn sha256_bekannte_vektoren_und_unabhaengiges_lokales_werkzeug() {
        use std::io::Write;
        use std::process::{Command, Stdio};
        for (body, expected) in [
            (
                b"".as_slice(),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                b"abc".as_slice(),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
        ] {
            assert_eq!(format!("{:x}", Sha256::digest(body)), expected);
        }
        // Nur synthetische Testobjekte, kein produktiver Prozess/Netz/Git/SSH.
        let f = fixture(b"checksum\n\xff\n");
        let result = run(f.clone(), false).unwrap();
        for item in result.evidence().objects() {
            let bytes = match item.kind() {
                EvidenceKind::Commit => f.raw[&item.oid()].clone(),
                EvidenceKind::Blob => {
                    if item.oid() == f.blob {
                        payload().bytes().to_vec()
                    } else {
                        b"preserved".to_vec()
                    }
                }
                EvidenceKind::Tree => {
                    let mut reader = RestReader::new(
                        |p: &str| Ok(serde_json::to_vec(&f.records[p]).unwrap()),
                        256,
                    )
                    .unwrap();
                    tree_bytes(&reader.tree(item.oid()).unwrap().entries).unwrap()
                }
            };
            let mut child = Command::new("sha256sum")
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Lokales sha256sum für unabhängige Testgegenprüfung erforderlich");
            child.stdin.take().unwrap().write_all(&bytes).unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(output.status.success());
            let expected: String = item
                .payload_sha256()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            assert_eq!(
                std::str::from_utf8(&output.stdout)
                    .unwrap()
                    .split_whitespace()
                    .next(),
                Some(expected.as_str())
            );
        }
    }

    #[test]
    fn fehler_und_budget_liefern_keine_partielle_erfolgsevidenz() {
        for failure in 0..4 {
            let mut f = fixture(b"valid\n");
            let side = object_oid(ObjectKind::Blob, b"preserved");
            let path = format!("{PREFIX}/git/blobs/{side}");
            match failure {
                0 => {
                    f.records.get_mut(&path).unwrap()["content"] =
                        json!(encode_base64(b"tampered!"));
                }
                1 => {
                    f.records.remove(&path);
                }
                2 => {
                    f.raw.remove(&f.after);
                }
                _ => {}
            }
            assert!(run(f, failure == 3).is_err());
        }
    }
}

// Vollständigkeitsprofil 293: dieselben synthetischen Originalobjekte und
// REST-Pfade wie in den bestehenden Initial-/Update-Integrationstests.
mod complete_readback_293 {
    use super::*;
    use novena_bridge::offline_readback::*;
    use sha2::{Digest, Sha256};

    fn put_blob(f: &mut Fixture, body: &[u8]) -> Oid {
        let id = object_oid(ObjectKind::Blob, body);
        f.records.insert(
            format!("{PREFIX}/git/blobs/{id}"),
            json!({
                "sha":id.to_string(), "encoding":"base64", "size":body.len(),
                "content":encode_base64(body)
            }),
        );
        id
    }
    fn source(f: Fixture) -> impl ResponseSource {
        move |path: &str| {
            serde_json::to_vec(f.records.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                .map_err(|_| BridgeError("SERIALIZATION"))
        }
    }
    fn root(f: &Fixture) -> Oid {
        inspect_commit(&f.raw[&f.after]).unwrap().tree
    }
    fn enriched() -> Fixture {
        let mut f = Fixture::new(false);
        put_blob(&mut f, b"preserved");
        f
    }

    #[test]
    fn vollstaendiger_initial_readback_mit_nebenblob_und_unveraenderten_sperren() {
        let f = enriched();
        let roots = [inspect_commit(&f.raw[&f.before]).unwrap().tree, root(&f)];
        let budget = RunBudget::new();
        let shared = Rc::new(RefCell::new(f.clone()));
        let mut rest = RestReader::new(budget.responses(source(f.clone())), 256).unwrap();
        // Refantwort bleibt synthetisch; kein HTTP-/Git-Prozess.
        let mut records = f.clone();
        records.records.insert(
            format!("{PREFIX}/git/ref/heads/main"),
            json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}}),
        );
        // Readerwechsel erhält denselben laufweiten Kontext.
        assert_eq!(rest.repository().unwrap(), "SYNTHETIC_REPOSITORY_ID");
        let mut rest = RestReader::new(budget.responses(source(records)), 256).unwrap();
        let mut raw = budget.commits(super::raw(shared));
        let expected = payload();
        let original = expected.bytes().to_vec();
        let report = verify_initial_index(
            &mut rest,
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            f.before,
            f.after,
            &expected,
        )
        .unwrap();
        assert!(!report.publication_performed);
        assert_eq!(report.scope, "OFFLINE_SUPPLIED_RESPONSES");
        let complete = verify_complete_blobs(&mut rest, &roots, &budget).unwrap();
        assert_eq!(complete.scope(), RunScope::Offline);
        assert_eq!(complete.paths(), 3);
        assert_eq!(complete.blobs().len(), 2);
        let side = object_oid(ObjectKind::Blob, b"preserved");
        assert_eq!(complete.blobs()[&side].length, 9);
        assert_eq!(
            complete.blobs()[&side].payload_sha256,
            <[u8; 32]>::from(Sha256::digest(b"preserved"))
        );
        assert_eq!(expected.bytes(), original);
        let document: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert_eq!(document["last_verified"], Value::Null);
        assert_eq!(document["publication"]["state"], "NOT_PUBLISHED");
        assert_eq!(
            production_publish(),
            Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
        );
    }

    #[test]
    fn nebenblob_fehlt_oder_ist_manipuliert_kein_teilerfolg() {
        for missing in [false, true] {
            let mut f = enriched();
            let tree = root(&f);
            let side = object_oid(ObjectKind::Blob, b"preserved");
            let endpoint = format!("{PREFIX}/git/blobs/{side}");
            if missing {
                f.records.remove(&endpoint);
            } else {
                f.records.get_mut(&endpoint).unwrap()["content"] =
                    json!(encode_base64(b"tampered!"));
            }
            let budget = RunBudget::new();
            let mut rest = RestReader::new(budget.responses(source(f)), 256).unwrap();
            let error = verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err();
            assert_eq!(
                error,
                BridgeError(if missing {
                    "MISSING_RESPONSE"
                } else {
                    "BLOB_HASH_MISMATCH"
                })
            );
            assert_eq!(
                verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
                BridgeError("RUN_ALREADY_FAILED")
            );
        }
    }

    #[test]
    fn mehrfach_referenzierte_oid_ausfuehrbar_und_symlink_nur_ein_blobabruf() {
        let mut f = enriched();
        let id = put_blob(&mut f, b"../nicht-verfolgen");
        let entries = vec![
            e("a", Mode::File, id),
            e("b", Mode::Executable, id),
            e("c", Mode::Symlink, id),
        ];
        let tree = tree_oid(&entries).unwrap();
        f.records.insert(
            format!("{PREFIX}/git/trees/{tree}"),
            json!({"sha":tree.to_string(), "truncated":false,
            "tree":[
                {"path":"a","mode":"100644","type":"blob","sha":id.to_string()},
                {"path":"b","mode":"100755","type":"blob","sha":id.to_string()},
                {"path":"c","mode":"120000","type":"blob","sha":id.to_string()}
            ]}),
        );
        let budget = RunBudget::new();
        let mut rest = RestReader::new(budget.responses(source(f)), 256).unwrap();
        let report = verify_complete_blobs(&mut rest, &[tree], &budget).unwrap();
        assert_eq!(report.paths(), 3);
        assert_eq!(report.blobs().len(), 1);
        assert_eq!(rest.requests(), 2);
    }

    #[test]
    fn falscher_typ_gitlink_duplikat_und_unvollstaendige_trees_abgewiesen() {
        for case in 0..7 {
            let mut f = enriched();
            let mut tree = root(&f);
            if case == 6 {
                let id = f.before;
                tree = tree_oid(&[e("sub", Mode::Gitlink, id)]).unwrap();
                f.records.insert(
                    format!("{PREFIX}/git/trees/{tree}"),
                    json!({"sha":tree.to_string(),"truncated":false,
                    "tree":[{"path":"sub","mode":"160000","type":"commit","sha":id.to_string()}]}),
                );
            } else {
                let endpoint = format!("{PREFIX}/git/trees/{tree}");
                let v = f.records.get_mut(&endpoint).unwrap();
                match case {
                    0 => v["tree"][0]["type"] = json!("tree"),
                    1 => v["truncated"] = json!(true),
                    2 => {
                        v["tree"].as_array_mut().unwrap().pop();
                    }
                    3 => {
                        let duplicate = v["tree"][0].clone();
                        v["tree"].as_array_mut().unwrap().push(duplicate);
                    }
                    4 => v["tree"][0]["mode"] = json!("999999"),
                    5 => {
                        f.records.remove(&endpoint);
                    }
                    _ => unreachable!(),
                }
            }
            let budget = RunBudget::new();
            let mut rest = RestReader::new(budget.responses(source(f)), 256).unwrap();
            let error = verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err();
            assert_eq!(
                error.0,
                [
                    "INVALID_TYPE_OR_MODE",
                    "INCOMPLETE_OR_WRONG_TREE",
                    "TREE_HASH_MISMATCH",
                    "INVALID_OR_DUPLICATE_NAME",
                    "INVALID_TYPE_OR_MODE",
                    "MISSING_RESPONSE",
                    "GITLINK_UNSUPPORTED"
                ][case]
            );
        }
    }

    #[test]
    fn gemeinsames_rest_und_raw_budget_ueber_readerwechsel() {
        const MIB: usize = 1024 * 1024;
        let budget = RunBudget::new();
        for size in [8 * MIB, 8 * MIB, 8 * MIB, 7 * MIB] {
            let mut rest = RestReader::new(
                budget.responses(move |_: &str| {
                    let mut body = vec![b' '; size];
                    body[..2].copy_from_slice(b"{}");
                    Ok(body)
                }),
                256,
            )
            .unwrap();
            rest.get(PREFIX).unwrap();
        }
        let mut bytes = format!("tree {}\nauthor Test <test@localhost.invalid> 1 +0000\ncommitter Test <test@localhost.invalid> 1 +0000\n\nx", "a".repeat(40)).into_bytes();
        bytes.resize(MIB, b'x');
        let id = object_oid(ObjectKind::Commit, &bytes);
        let mut raw = budget.commits(move |_: Oid| Ok(bytes.clone()));
        assert_eq!(raw.raw_commit(id).unwrap().len(), MIB);
        assert_eq!(
            raw.raw_commit(id).unwrap_err(),
            BridgeError("RUN_BUDGET_EXCEEDED")
        );
        let mut rest = RestReader::new(
            budget.responses(|_: &str| -> BridgeResult<Vec<u8>> { panic!("keine weitere Quelle") }),
            256,
        )
        .unwrap();
        assert_eq!(
            rest.get(PREFIX).unwrap_err(),
            BridgeError("RUN_ALREADY_FAILED")
        );
    }

    #[test]
    fn anfragebudget_ohne_neustart_und_kein_vollstaendigkeitsbeleg() {
        let budget = RunBudget::new();
        for _ in 0..256 {
            let mut rest =
                RestReader::new(budget.responses(|_: &str| Ok(b"{}".to_vec())), 256).unwrap();
            rest.get(PREFIX).unwrap();
        }
        let f = enriched();
        let tree = root(&f);
        let mut rest = RestReader::new(budget.responses(source(f)), 256).unwrap();
        assert_eq!(
            verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
            BridgeError("RUN_BUDGET_EXCEEDED")
        );
    }

    #[test]
    fn gesamtbodybudget_waehrend_echter_tree_und_blobpruefung() {
        let f = enriched();
        let tree = root(&f);
        let budget = RunBudget::new();
        let mut rest = RestReader::new(
            budget.responses(move |path: &str| {
                let mut body =
                    serde_json::to_vec(f.records.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                        .map_err(|_| BridgeError("SERIALIZATION"))?;
                body.resize(8 * 1024 * 1024, b' ');
                Ok(body)
            }),
            256,
        )
        .unwrap();
        assert_eq!(
            verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
            BridgeError("RESPONSE_BUDGET_EXCEEDED")
        );
        assert_eq!(rest.requests(), 4);
        assert_eq!(
            verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
            BridgeError("RUN_ALREADY_FAILED")
        );
    }

    #[test]
    fn abgeschnittenes_json_und_budgetkontextwechsel_verhindern_beleg() {
        let tree = root(&enriched());
        let budget = RunBudget::new();
        let mut rest = RestReader::new(
            budget.responses(|_: &str| Ok(br#"{"tree":["#.to_vec())),
            256,
        )
        .unwrap();
        assert_eq!(
            verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
            BridgeError("INVALID_RESPONSE_JSON")
        );
        let first = RunBudget::new();
        let other = RunBudget::new();
        let mut rest = RestReader::new(
            first.responses(|_: &str| -> BridgeResult<Vec<u8>> {
                panic!("keine Quelle bei Kontextwechsel")
            }),
            256,
        )
        .unwrap();
        assert_eq!(
            verify_complete_blobs(&mut rest, &[tree], &other).unwrap_err(),
            BridgeError("RUN_BUDGET_CONTEXT_MISMATCH")
        );
        assert_eq!(rest.requests(), 0);
    }

    #[test]
    fn zuteilungsgrenzen_fuer_treeeintraege_treebytes_und_blobgroesse() {
        let f = enriched();
        let tree = root(&f);
        let mut rest = RestReader::new(source(f.clone()), 256).unwrap();
        assert_eq!(
            rest.tree_bounded(tree, 1, 32 * 1024 * 1024).unwrap_err(),
            BridgeError("TREE_LIMIT_EXCEEDED")
        );
        assert_eq!(
            rest.tree_bounded(tree, 100_000, 1).unwrap_err(),
            BridgeError("TREE_BYTE_LIMIT_EXCEEDED")
        );
        let side = object_oid(ObjectKind::Blob, b"preserved");
        assert_eq!(
            rest.blob_bounded(side, 8).unwrap_err(),
            BridgeError("BLOB_TOO_LARGE")
        );
        assert_eq!(rest.blob_bounded(side, 9).unwrap(), b"preserved");
    }

    #[test]
    fn maximale_treetiefe_und_pfadsumme_werden_vor_pfadzuteilung_geprueft() {
        for long_path in [false, true] {
            let mut f = enriched();
            let mut tree = f.put_tree(vec![]);
            let name = if long_path {
                "x".repeat(3000)
            } else {
                "d".into()
            };
            for _ in 0..if long_path { 2 } else { 130 } {
                tree = f.put_tree(vec![e(&name, Mode::Directory, tree)]);
            }
            let budget = RunBudget::new();
            let mut rest = RestReader::new(budget.responses(source(f)), 256).unwrap();
            assert_eq!(
                verify_complete_blobs(&mut rest, &[tree], &budget).unwrap_err(),
                BridgeError(if long_path {
                    "TREE_PATH_TOO_LONG"
                } else {
                    "TREE_LIMIT_EXCEEDED"
                })
            );
        }
    }

    #[test]
    fn offline_vertraege_koennen_keine_live_herkunft_behaupten() {
        let id = Oid::parse(&"a".repeat(40)).unwrap();
        assert!(Oid::parse(&"A".repeat(40)).is_err());
        let spec = ReadbackSpec::new(
            "EXPLICIT_SYNTHETIC_PIN",
            1,
            id,
            ReadbackMode::Snapshot {
                expected: b"synthetic",
            },
        )
        .unwrap();
        assert_eq!(spec.scope(), RunScope::Offline);
        assert_eq!(spec.repository(), ARCHIVE);
        assert_eq!(spec.reference(), REF);
        assert!(spec.endpoint("https://evil.invalid/").is_err());
        let claims = [ExportClaim {
            oid: id,
            length: 5,
            payload_sha256: [0; 32],
        }];
        let manifest = AcquisitionManifest::untrusted(br#"{"scope":"LIVE"}"#, &claims).unwrap();
        assert_eq!(manifest.provenance(), "UNTRUSTED_ACQUISITION_CLAIM");
        assert_eq!(spec.scope(), RunScope::Offline);
        assert!(ReadbackSpec::new("", 1, id, ReadbackMode::Snapshot { expected: b"" }).is_err());
        assert!(AcquisitionManifest::untrusted(b"", &[]).is_err());
    }
}

fn reader(f: Rc<RefCell<Fixture>>, limit: usize) -> RestReader<impl ResponseSource> {
    RestReader::new(
        move |path: &str| {
            let mut f = f.borrow_mut();
            if path == format!("{PREFIX}/git/ref/heads/main") {
                let head = f.head_sequence.pop_front().unwrap_or(f.head);
                return Ok(serde_json::to_vec(
                    &json!({"ref":REF,"object":{"type":"commit","sha":head.to_string()}}),
                )
                .unwrap());
            }
            serde_json::to_vec(f.records.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                .map_err(|_| BridgeError("SERIALIZATION"))
        },
        limit,
    )
    .unwrap()
}
fn raw(f: Rc<RefCell<Fixture>>) -> impl RawCommitSource {
    move |id: Oid| {
        f.borrow()
            .raw
            .get(&id)
            .cloned()
            .ok_or(BridgeError("MISSING_RAW_COMMIT"))
    }
}
fn verify(f: Fixture) -> BridgeResult<ReadbackReport> {
    let (before, after) = (f.before, f.after);
    let shared = Rc::new(RefCell::new(f));
    verify_initial_index(
        &mut reader(shared.clone(), 256),
        &mut raw(shared),
        "SYNTHETIC_REPOSITORY_ID",
        before,
        after,
        &payload(),
    )
}
#[test]
fn integrated_independent_readback_and_python_negative_vectors() {
    let f = Fixture::new(false);
    let report = verify(f.clone()).unwrap();
    assert!(report.commit_hash_verified);
    assert!(!report.publication_performed);
    assert_eq!(report.scope, "OFFLINE_SUPPLIED_RESPONSES");
    // A continuation payload cannot bypass the initial checkpoint restriction.
    let mut v: Value = serde_json::from_slice(payload().bytes()).unwrap();
    v["last_verified"] = json!({"block_id":"BASE","git_head":"a".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["original.txt"]});
    let status = Status::from_value(v, PreviousKind::Local).unwrap();
    let continuation = prepare(&status, &status).unwrap();
    let shared = Rc::new(RefCell::new(f.clone()));
    assert_eq!(
        verify_initial_index(
            &mut reader(shared.clone(), 256),
            &mut raw(shared),
            "SYNTHETIC_REPOSITORY_ID",
            f.before,
            f.after,
            &continuation
        )
        .unwrap_err(),
        BridgeError("INITIAL_CHECKPOINT_NOT_NULL")
    );
    let tree = format!("{PREFIX}/git/trees/{}", f.leaf);
    let blob = format!("{PREFIX}/git/blobs/{}", f.blob);
    let commit = format!("{PREFIX}/git/commits/{}", f.after);
    let mut cases = Vec::new();
    let mut x = f.clone();
    x.records.get_mut(PREFIX).unwrap()["node_id"] = json!("OTHER");
    cases.push(x);
    let mut x = f.clone();
    x.records.get_mut(PREFIX).unwrap()["full_name"] = json!("other/repo");
    cases.push(x);
    let mut x = f.clone();
    x.head = x.before;
    cases.push(x);
    let mut x = f.clone();
    x.head_sequence = VecDeque::from([f.after, f.before]);
    cases.push(x);
    let mut x = f.clone();
    x.records.get_mut(&commit).unwrap()["parents"] = json!([]);
    cases.push(x);
    let mut x = f.clone();
    x.raw.get_mut(&f.after).unwrap().extend(b"tampered");
    cases.push(x);
    let mut x = f.clone();
    x.raw.remove(&f.after);
    cases.push(x);
    let mut x = f.clone();
    x.records.remove(&tree);
    cases.push(x);
    for value in [json!(true), Value::Null, json!("false")] {
        let mut x = f.clone();
        x.records.get_mut(&tree).unwrap()["truncated"] = value;
        cases.push(x);
    }
    let mut x = f.clone();
    x.records.get_mut(&tree).unwrap()["tree"] = json!([]);
    cases.push(x);
    let mut x = f.clone();
    let v = x.records.get_mut(&tree).unwrap();
    let duplicate = v["tree"][0].clone();
    v["tree"].as_array_mut().unwrap().push(duplicate);
    cases.push(x);
    for (field, value) in [
        ("mode", json!("100755")),
        ("path", json!("../escape")),
        ("sha", json!("f".repeat(40))),
        ("unknown", json!(1)),
        ("size", json!(-1)),
        ("size", json!(true)),
        ("url", json!(0)),
    ] {
        let mut x = f.clone();
        x.records.get_mut(&tree).unwrap()["tree"][0][field] = value;
        cases.push(x);
    }
    for (field, value) in [
        ("content", json!("!!!=")),
        ("content", json!(base64(b"tampered"))),
        ("size", json!(0)),
        ("encoding", json!("utf8")),
        ("sha", json!("f".repeat(40))),
    ] {
        let mut x = f.clone();
        x.records.get_mut(&blob).unwrap()[field] = value;
        cases.push(x);
    }
    cases.push(Fixture::new(true));
    for (i, case) in cases.into_iter().enumerate() {
        assert!(verify(case).is_err(), "negative {i}");
    }
}

#[test]
fn allowlist_budget_and_strict_transport_input() {
    let f = Rc::new(RefCell::new(Fixture::new(false)));
    let mut r = reader(f, 1);
    r.repository().unwrap();
    assert_eq!(
        r.repository().unwrap_err(),
        BridgeError("REQUEST_LIMIT_EXCEEDED")
    );
    assert_eq!(r.requests(), 1);
    for path in [
        format!("{PREFIX}/issues"),
        format!("{PREFIX}/git/trees/{}?recursive=1", "a".repeat(40)),
        "https://api.github.com/repos/x".into(),
        format!("{PREFIX}/git/trees/{}", "A".repeat(40)),
    ] {
        assert!(allowed_endpoint(&path).is_err());
    }
    for bytes in [
        b"invalid".to_vec(),
        b"[]".to_vec(),
        b"{\"a\":1,\"a\":2}".to_vec(),
        vec![0xff],
        vec![b' '; 8 * 1024 * 1024 + 1],
    ] {
        let mut r = RestReader::new(move |_: &str| Ok(bytes.clone()), 1).unwrap();
        assert!(r.get(PREFIX).is_err());
    }
    for (encoded, expected) in [
        ("", b"".as_slice()),
        ("Zg==", b"f"),
        ("Zm8=", b"fo"),
        (" Zm9v\n", b"foo"),
    ] {
        assert_eq!(decode_base64(encoded).unwrap(), expected);
    }
    for bad in [
        "!!!=",
        "Zg=",
        "=g==",
        "Zg==AAAA",
        "Zh==",
        "Zm9=",
        "Zg===",
        "Zg==\u{a0}",
    ] {
        assert!(decode_base64(bad).is_err(), "{bad}");
    }
    assert!(production_publish().is_err());
}

#[test]
fn graphql_exact_cas_conflict_and_ambiguity_never_retry() {
    let success = br#"{"data":{"updateRefs":{"clientMutationId":null}}}"#;
    let replies: Vec<BridgeResult<Vec<u8>>> = vec![
        Ok(success.to_vec()),
        Err(BridgeError("TIMEOUT")),
        Ok(b"{}".to_vec()),
        Ok(br#"{"errors":[{"type":"STALE_DATA"}]}"#.to_vec()),
        Ok(br#"{"data":{"updateRefs":{"clientMutationId":null}},"errors":[]}"#.to_vec()),
    ];
    for (i, reply) in replies.into_iter().enumerate() {
        let f = Rc::new(RefCell::new(Fixture::new(false)));
        let (before, after) = (f.borrow().before, f.borrow().after);
        f.borrow_mut().head = before;
        let mut calls = 0;
        let shared = f.clone();
        let outcome = simulate_graphql(
            &mut reader(f.clone(), 256),
            &mut raw(f),
            |request| {
                calls += 1;
                let update = &request["variables"]["input"]["refUpdates"][0];
                assert_eq!(update["beforeOid"], before.to_string());
                assert_eq!(update["afterOid"], after.to_string());
                assert_eq!(update["force"], false);
                shared.borrow_mut().head = after;
                reply.clone()
            },
            "SYNTHETIC_REPOSITORY_ID",
            before,
            after,
            &payload(),
        )
        .unwrap();
        assert_eq!(calls, 1);
        if i == 0 {
            assert!(matches!(outcome, SyntheticOutcome::Verified(_)));
        } else {
            assert_eq!(outcome, SyntheticOutcome::MutationUnknown);
        }
        assert_eq!(shared.borrow().head, after); // Never roll back even on ambiguous reply.
    }
    let f = Rc::new(RefCell::new(Fixture::new(false)));
    let (before, after) = (f.borrow().before, f.borrow().after);
    let mut calls = 0;
    let result = simulate_graphql(
        &mut reader(f.clone(), 256),
        &mut raw(f),
        |_| {
            calls += 1;
            Ok(success.to_vec())
        },
        "SYNTHETIC_REPOSITORY_ID",
        before,
        after,
        &payload(),
    )
    .unwrap();
    assert_eq!(result, SyntheticOutcome::Conflict);
    assert_eq!(calls, 0);
    for (repo, reference, b, a) in [
        ("", REF, before, after),
        ("ID", "refs/heads/other", before, after),
        ("ID", REF, before, before),
    ] {
        assert!(update_refs_request(repo, reference, b, a).is_err());
    }
    // Valid response without independently observed update is still ambiguous.
    let f = Rc::new(RefCell::new(Fixture::new(false)));
    f.borrow_mut().head = before;
    assert_eq!(
        simulate_graphql(
            &mut reader(f.clone(), 256),
            &mut raw(f),
            |_| Ok(success.to_vec()),
            "SYNTHETIC_REPOSITORY_ID",
            before,
            after,
            &payload()
        )
        .unwrap(),
        SyntheticOutcome::MutationUnknown
    );
}

#[test]
fn offline_bundle_cli_end_to_end() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut f = Fixture::new(false);
    f.records.insert(
        format!("{PREFIX}/git/ref/heads/main"),
        json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}}),
    );
    let commits: serde_json::Map<String, Value> = f
        .raw
        .iter()
        .map(|(id, bytes)| (id.to_string(), json!(base64(bytes))))
        .collect();
    let bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","before":f.before.to_string(),"after":f.after.to_string(),"candidate":serde_json::from_slice::<Value>(payload().bytes()).unwrap(),"responses":f.records,"raw_commits":commits});
    let bytes = serde_json::to_vec(&bundle).unwrap();
    let report = verify_offline_bundle(&bytes).unwrap();
    assert!(report.commit_hash_verified);
    assert!(!report.publication_performed);
    let mut child = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args(["verify-offline", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap(),
        report.as_json()
    );
    let mut broken = bundle;
    broken["raw_commits"] = json!({});
    assert!(verify_offline_bundle(&serde_json::to_vec(&broken).unwrap()).is_err());
}

#[test]
fn offline_planning_binds_baseline_commit_metadata() {
    use novena_bridge::planning::{plan_offline_bundle, plan_update_bundle};
    for update in [false, true] {
        let f = Fixture::new(false);
        let before = if update { f.after } else { f.before };
        let route = format!("{PREFIX}/git/commits/{before}");
        let mut bundle = json!({
            "repository_id":"SYNTHETIC_REPOSITORY_ID",
            "before":before.to_string(),
            "candidate":Status::parse(payload().bytes(), PreviousKind::Local).unwrap().document(),
            "responses":f.records,
            "raw_commits":{before.to_string():base64(&f.raw[&before])},
            "identity":"Novena Test <test@localhost.invalid> 1760000000 +0000",
            "message":"Synthetic plan\n"
        });
        bundle["responses"][format!("{PREFIX}/git/ref/heads/main")] =
            json!({"ref":REF,"object":{"type":"commit","sha":before.to_string()}});
        if update {
            bundle["baseline_base64"] = json!(base64(payload().bytes()));
        }
        let plan = |v: &Value| {
            let bytes = serde_json::to_vec(v).unwrap();
            if update {
                plan_update_bundle(&bytes)
            } else {
                plan_offline_bundle(&bytes)
            }
        };
        let accepted = plan(&bundle).unwrap();
        assert_eq!(accepted["before"], before.to_string());
        assert_eq!(accepted["write_performed"], false);
        assert_eq!(accepted["bridge_publication"], "BLOCKED");

        for (field, value, error) in [
            ("sha", json!("0".repeat(40)), "COMMIT_ID_MISMATCH"),
            (
                "tree",
                json!({"sha":"0".repeat(40)}),
                "COMMIT_METADATA_MISMATCH",
            ),
            ("parents", json!([]), "COMMIT_METADATA_MISMATCH"),
        ] {
            let mut bad = bundle.clone();
            // Root baselines need a fabricated parent to contradict the raw bytes.
            bad["responses"][route.as_str()][field] = if field == "parents" && !update {
                json!([{"sha":"0".repeat(40)}])
            } else {
                value
            };
            assert_eq!(plan(&bad).unwrap_err(), BridgeError(error));
        }
        let mut missing = bundle.clone();
        missing["responses"].as_object_mut().unwrap().remove(&route);
        assert_eq!(plan(&missing).unwrap_err(), BridgeError("MISSING_RESPONSE"));
        let mut bad_raw = bundle.clone();
        let mut changed = f.raw[&before].clone();
        changed.extend_from_slice(b"changed");
        bad_raw["raw_commits"][before.to_string()] = json!(base64(&changed));
        assert_eq!(
            plan(&bad_raw).unwrap_err(),
            BridgeError("COMMIT_HASH_MISMATCH")
        );
    }
}

#[test]
fn object_plan_matches_independent_graph_and_cli() {
    use novena_bridge::planning::{plan_offline_bundle, prepare_initial_commit};
    let mut f = Fixture::new(false);
    let before = f.before;
    let after = f.after;
    f.records.insert(
        format!("{PREFIX}/git/ref/heads/main"),
        json!({"ref":REF,"object":{"type":"commit","sha":before.to_string()}}),
    );
    let parent = f.raw[&before].clone();
    let shared = Rc::new(RefCell::new(f.clone()));
    let status = Status::parse(payload().bytes(), PreviousKind::Local).unwrap();
    let identity = "Novena Test <test@localhost.invalid> 1760000000 +0000";
    let plan = prepare_initial_commit(
        &mut reader(shared.clone(), 256),
        &parent,
        before,
        &status,
        identity,
        b"Synthetic Bridge commit\n",
    )
    .unwrap();
    assert_eq!(plan.after, after);
    assert_eq!(plan.blob, f.blob);
    assert_eq!(plan.objects.len(), 5);
    for object in &plan.objects {
        assert_eq!(object_oid(object.kind, &object.bytes), object.oid);
    }
    let bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","before":before.to_string(),"candidate":status.document(),"responses":f.records,"raw_commits":{before.to_string():base64(&parent)},"identity":identity,"message":"Synthetic Bridge commit\n"});
    let result = plan_offline_bundle(&serde_json::to_vec(&bundle).unwrap()).unwrap();
    assert_eq!(result, plan.as_json("SYNTHETIC_REPOSITORY_ID").unwrap());
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut child = Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args(["prepare-objects", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&bundle).unwrap())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap(),
        result
    );
    for identity in [
        "malformed",
        "A <a> 1 +0000\ncommitter A <a> 1 +0000\n\ninjection",
    ] {
        assert!(prepare_initial_commit(
            &mut reader(shared.clone(), 256),
            &parent,
            before,
            &status,
            identity,
            b"message"
        )
        .is_err());
    }
    assert!(prepare_initial_commit(
        &mut reader(shared.clone(), 256),
        &parent,
        before,
        &status,
        identity,
        b""
    )
    .is_err());
    assert!(prepare_initial_commit(
        &mut reader(shared.clone(), 256),
        b"wrong",
        before,
        &status,
        identity,
        b"message"
    )
    .is_err());
    assert!(prepare_initial_commit(
        &mut reader(shared, 256),
        &f.raw[&after],
        after,
        &status,
        identity,
        b"message"
    )
    .is_err());
}

#[test]
fn planning_preserves_existing_subtrees_and_rejects_parent_collisions() {
    use novena_bridge::planning::prepare_initial_commit;
    let identity = "Novena Test <test@localhost.invalid> 1760000000 +0000";
    for collision in [false, true] {
        let mut f = Fixture::new(false);
        let existing = object_oid(ObjectKind::Blob, b"existing sibling");
        let old_status = f.put_tree(vec![e("sibling.json", Mode::File, existing)]);
        let old_novena = f.put_tree(vec![e(
            "status",
            if collision {
                Mode::File
            } else {
                Mode::Directory
            },
            if collision { existing } else { old_status },
        )]);
        let root = f.put_tree(vec![e("novena", Mode::Directory, old_novena)]);
        let before = f.put_commit(root, None, "Existing baseline");
        let parent = f.raw[&before].clone();
        let shared = Rc::new(RefCell::new(f));
        let status = Status::parse(payload().bytes(), PreviousKind::Local).unwrap();
        let plan = prepare_initial_commit(
            &mut reader(shared, 256),
            &parent,
            before,
            &status,
            identity,
            b"message",
        );
        if collision {
            assert!(plan.is_err());
        } else {
            assert!(plan.is_ok());
        }
    }
}

#[test]
fn python_planner_and_graphql_contract_differential() {
    use std::process::Command;
    let script = r#"
import importlib.util,json,sys,copy
from pathlib import Path
root=Path(sys.argv[1])
def load(name):
    spec=importlib.util.spec_from_file_location('reference_'+name,root/('novena-bridge-'+name+'.py'))
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
p=load('publisher');g=load('publish-gate');t=load('rest-transport')
pilot=p.read_json(p.PILOT);candidate=copy.deepcopy(pilot)
candidate['latest_block']={'block_id':'SYNTHETIC-FAIL','runner':'FAIL','upload':'PASS','technical_result':'FAIL','professional_acceptance':'UNKNOWN'}
prepared=p.prepare(pilot,candidate,p.schema_validator())
paths=[t.PREFIX,t.PREFIX+'/git/ref/heads/main',t.PREFIX+'/git/trees/'+'a'*40,t.PREFIX+'/git/trees/'+'a'*40+'?recursive=1',t.PREFIX+'/issues']
allowed=[]
for path in paths:
    try: t.allowed_endpoint(path);allowed.append(True)
    except t.TransportError: allowed.append(False)
json.dump({'cas':g.update_refs_request('SYNTHETIC_ID','refs/heads/main','a'*40,'b'*40),'candidate':candidate,'prepared':p.canonical(prepared).decode('utf-8'),'paths':paths,'allowed':allowed},sys.stdout)
"#;
    let out = Command::new("python3")
        .args(["-B", "-c", script])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts"))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(
        update_refs_request(
            "SYNTHETIC_ID",
            REF,
            Oid::parse(&"a".repeat(40)).unwrap(),
            Oid::parse(&"b".repeat(40)).unwrap()
        )
        .unwrap(),
        v["cas"]
    );
    let previous = Status::parse(
        include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json"),
        PreviousKind::Local,
    )
    .unwrap();
    let candidate = Status::from_value(v["candidate"].clone(), PreviousKind::Local).unwrap();
    assert_eq!(
        prepare(&previous, &candidate).unwrap().bytes(),
        v["prepared"].as_str().unwrap().as_bytes()
    );
    for (path, allowed) in v["paths"]
        .as_array()
        .unwrap()
        .iter()
        .zip(v["allowed"].as_array().unwrap())
    {
        assert_eq!(
            allowed_endpoint(path.as_str().unwrap()).is_ok(),
            allowed.as_bool().unwrap()
        );
    }
}

#[test]
fn variant_a_attests_exact_bytes_without_promoting_claims() {
    use novena_bridge::attestation::*;
    let f = Fixture::new(false);
    let bytes = payload().bytes().to_vec();
    let mut raw = |id| f.raw.get(&id).cloned().ok_or(BridgeError("MISSING_RAW"));
    let report = verify_snapshot(
        &mut reader(Rc::new(RefCell::new(f.clone())), 256),
        &mut raw,
        "SYNTHETIC_REPOSITORY_ID",
        f.after,
        &bytes,
        OfflineObservation {
            recorded_at_utc: "2026-10-26T12:00:00Z",
            conditions: "synthetic independent responses",
        },
    )
    .unwrap();
    assert_eq!(report.bytes(), bytes);
    let j = report.as_json();
    assert_eq!(j["professional_acceptance"], "NOT_ATTESTED");
    assert_eq!(j["bridge_publication"], "BLOCKED");
    let mut child = std::process::Command::new("python3")
        .args([
            "-B",
            "-c",
            "import sys,hashlib; print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert_eq!(j["sha256"], String::from_utf8(out.stdout).unwrap().trim());
    assert_eq!(
        Status::parse(report.bytes(), PreviousKind::Local)
            .unwrap()
            .document()["publication"]["state"],
        "NOT_PUBLISHED"
    );
    for variant in 0..6 {
        let mut bad = f.clone();
        let mut expected = bytes.clone();
        let mut time = "2026-10-26T12:00:00Z";
        let mut conditions = "synthetic";
        match variant {
            0 => {
                expected.push(b' ');
            }
            1 => {
                bad.head_sequence = VecDeque::from([f.after, f.before]);
            }
            2 => {
                bad.raw.get_mut(&f.after).unwrap().push(b' ');
            }
            3 => {
                time = "invalid";
            }
            4 => {
                conditions = "";
            }
            _ => {
                bad.records.get_mut(PREFIX).unwrap()["node_id"] = json!("wrong");
            }
        }
        let mut raw = |id| bad.raw.get(&id).cloned().ok_or(BridgeError("MISSING_RAW"));
        assert!(verify_snapshot(
            &mut reader(Rc::new(RefCell::new(bad.clone())), 256),
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            f.after,
            &expected,
            OfflineObservation {
                recorded_at_utc: time,
                conditions
            }
        )
        .is_err());
    }
}

#[test]
fn continuation_preserves_siblings_and_binds_original_baseline() {
    use novena_bridge::planning::*;
    let f = Fixture::new(false);
    let baseline = payload().bytes().to_vec();
    let mut candidate = Status::parse(&baseline, PreviousKind::Local)
        .unwrap()
        .document()
        .clone();
    candidate["generated_at_utc"] = json!("2026-10-26T12:00:00Z");
    let candidate = Status::from_value(candidate, PreviousKind::Local).unwrap();
    let parent = &f.raw[&f.after];
    let identity = "Novena Test <test@localhost.invalid> 1760000000 +0000";
    let mut r = reader(Rc::new(RefCell::new(f.clone())), 256);
    let plan = prepare_update_commit(
        &mut r,
        parent,
        f.after,
        &baseline,
        &candidate,
        identity,
        b"Update\n",
    )
    .unwrap();
    for o in &plan.objects {
        assert_eq!(object_oid(o.kind, &o.bytes), o.oid);
    }
    assert_eq!(
        inspect_commit(&plan.objects.last().unwrap().bytes)
            .unwrap()
            .parents,
        vec![f.after]
    );
    let mut wrong = baseline.clone();
    wrong.push(b' ');
    assert!(prepare_update_commit(
        &mut r,
        parent,
        f.after,
        &wrong,
        &candidate,
        identity,
        b"Update\n"
    )
    .is_err());
    assert!(prepare_update_commit(
        &mut r,
        parent,
        f.before,
        &baseline,
        &candidate,
        identity,
        b"Update\n"
    )
    .is_err());
    let mut bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","before":f.after.to_string(),
        "candidate":candidate.document(),"responses":f.records,"raw_commits":{},
        "identity":identity,"message":"Update\n","baseline_base64":base64(&baseline)});
    bundle["responses"][format!("{PREFIX}/git/ref/heads/main")] =
        json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}});
    bundle["raw_commits"][f.after.to_string()] = json!(base64(parent));
    let bytes = serde_json::to_vec(&bundle).unwrap();
    assert_eq!(
        plan_update_bundle(&bytes).unwrap()["after"],
        plan.after.to_string()
    );
    bundle["before"] = json!(f.before.to_string());
    assert!(plan_update_bundle(&serde_json::to_vec(&bundle).unwrap()).is_err());
}

fn update_fixture() -> (Fixture, Vec<u8>, PreparedStatus, Oid) {
    let mut f = Fixture::new(false);
    let baseline = payload().bytes().to_vec();
    let previous = Status::parse(&baseline, PreviousKind::Local).unwrap();
    let mut candidate = previous.document().clone();
    candidate["generated_at_utc"] = json!("2026-10-26T12:00:00Z");
    let expected = prepare(
        &previous,
        &Status::from_value(candidate, PreviousKind::Local).unwrap(),
    )
    .unwrap();
    let before = f.after;
    let blob = object_oid(ObjectKind::Blob, expected.bytes());
    let leaf = f.put_tree(vec![e("NOVENA-CURRENT-STATE.json", Mode::File, blob)]);
    let middle = f.put_tree(vec![e("status", Mode::Directory, leaf)]);
    let root = f.put_tree(vec![
        e(
            "README.txt",
            Mode::File,
            object_oid(ObjectKind::Blob, b"preserved"),
        ),
        e("novena", Mode::Directory, middle),
    ]);
    f.after = f.put_commit(root, Some(before), "Update");
    f.head = f.after;
    f.records.insert(format!("{PREFIX}/git/blobs/{blob}"),json!({"sha":blob.to_string(),"encoding":"base64","size":expected.bytes().len(),"content":base64(expected.bytes())}));
    (f, baseline, expected, before)
}

#[test]
fn update_readback_and_cas_races_never_retry() {
    let (f, baseline, expected, before) = update_fixture();
    for variant in 0..5 {
        let mut changed = f.clone();
        match variant {
            1 => changed.head_sequence = VecDeque::from([f.after, before]),
            2 => changed.head = before,
            3 => {
                changed.raw.get_mut(&f.after).unwrap().push(b' ');
            }
            4 => {
                changed.records.remove(&format!(
                    "{PREFIX}/git/blobs/{}",
                    object_oid(ObjectKind::Blob, &baseline)
                ));
            }
            _ => (),
        }
        let mut raw = |id| {
            changed
                .raw
                .get(&id)
                .cloned()
                .ok_or(BridgeError("MISSING_RAW"))
        };
        let result = verify_update_index(
            &mut reader(Rc::new(RefCell::new(changed.clone())), 256),
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            before,
            f.after,
            &expected,
            &baseline,
        );
        assert_eq!(result.is_ok(), variant == 0);
    }
    for variant in 0..5 {
        let mut changed = f.clone();
        changed.head_sequence = match variant {
            0 => VecDeque::from([before, f.after, f.after]),
            1 => VecDeque::from([f.after]), // preflight CAS conflict: zero attempts
            2 | 3 => VecDeque::from([before]), // timeout or malformed reply
            _ => VecDeque::from([before, f.after, before]), // race during readback
        };
        let calls = Rc::new(RefCell::new(0));
        let count = calls.clone();
        let mut raw = |id| {
            changed
                .raw
                .get(&id)
                .cloned()
                .ok_or(BridgeError("MISSING_RAW"))
        };
        let result = simulate_update_graphql(
            &mut reader(Rc::new(RefCell::new(changed.clone())), 256),
            &mut raw,
            move |_| {
                *count.borrow_mut() += 1;
                match variant {
                    2 => Err(BridgeError("TIMEOUT")),
                    3 => Ok(b"{}".to_vec()),
                    _ => Ok(serde_json::to_vec(
                        &json!({"data":{"updateRefs":{"clientMutationId":null}}}),
                    )
                    .unwrap()),
                }
            },
            "SYNTHETIC_REPOSITORY_ID",
            before,
            f.after,
            &expected,
            &baseline,
        )
        .unwrap();
        assert_eq!(*calls.borrow(), if variant == 1 { 0 } else { 1 });
        match variant {
            0 => assert!(matches!(result, SyntheticOutcome::Verified(_))),
            1 => assert_eq!(result, SyntheticOutcome::Conflict),
            _ => assert_eq!(result, SyntheticOutcome::MutationUnknown),
        }
    }
}

#[test]
fn replacement_policy_rejects_siblings_directories_modes_and_baseline() {
    let (f, baseline, expected, before) = update_fixture();
    let mut r = reader(Rc::new(RefCell::new(f.clone())), 256);
    let old = flatten_verified(&mut r, inspect_commit(&f.raw[&before]).unwrap().tree).unwrap();
    let new = flatten_verified(&mut r, inspect_commit(&f.raw[&f.after]).unwrap().tree).unwrap();
    let base = object_oid(ObjectKind::Blob, &baseline);
    let blob = object_oid(ObjectKind::Blob, expected.bytes());
    assert!(verify_single_replacement(&old, &new, base, blob).is_ok());
    for variant in 0..4 {
        let mut altered = FlatTree {
            files: new.files.clone(),
            directories: new.directories.clone(),
        };
        match variant {
            0 => {
                altered.files.remove("README.txt");
            }
            1 => {
                altered
                    .files
                    .insert("README.txt".into(), (Mode::File, blob));
            }
            2 => {
                altered.directories.insert("unexpected-empty".into());
            }
            _ => {
                altered
                    .files
                    .insert(TARGET.into(), (Mode::Executable, blob));
            }
        }
        assert!(verify_single_replacement(&old, &altered, base, blob).is_err());
    }
    assert!(verify_single_replacement(&old, &new, blob, blob).is_err());
}

fn run_bundle_cli(command: &str, bytes: &[u8]) -> std::process::Output {
    use std::io::Write;
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_novena-bridge"))
        .args([command, "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn variant_a_and_continuation_cli_end_to_end() {
    let (mut f, baseline, expected, before) = update_fixture();
    let commits: serde_json::Map<String, Value> = f
        .raw
        .iter()
        .map(|(id, b)| (id.to_string(), json!(base64(b))))
        .collect();
    let mut bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","commit":f.after.to_string(),
        "expected_bytes_base64":base64(expected.bytes()),"responses":f.records,
        "raw_commits":commits,"recorded_at_utc":"2026-10-26T12:00:00Z","conditions":"synthetic CLI fixture"});
    bundle["responses"][format!("{PREFIX}/git/ref/heads/main")] =
        json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}});
    let out = run_bundle_cli("verify-attestation", &serde_json::to_vec(&bundle).unwrap());
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["contract"], "VARIANT_A_OFFLINE_V1");
    assert_eq!(report["bytes_base64"], base64(expected.bytes()));
    bundle["unexpected"] = json!(true);
    let out = run_bundle_cli("verify-attestation", &serde_json::to_vec(&bundle).unwrap());
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    f.head = before;
    let identity = "Novena Test <test@localhost.invalid> 1760000000 +0000";
    let mut bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","before":before.to_string(),
        "candidate":Status::parse(expected.bytes(),PreviousKind::Local).unwrap().document(),
        "responses":f.records,"raw_commits":commits,"identity":identity,"message":"Update\n",
        "baseline_base64":base64(&baseline)});
    bundle["responses"][format!("{PREFIX}/git/ref/heads/main")] =
        json!({"ref":REF,"object":{"type":"commit","sha":before.to_string()}});
    let out = run_bundle_cli(
        "prepare-update-objects",
        &serde_json::to_vec(&bundle).unwrap(),
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let plan: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(plan["write_performed"], false);
    // All newly generated update objects checked by independent git hash-object,
    // without -w, repository creation, or object/index mutation.
    use std::io::Write;
    for object in plan["objects"].as_array().unwrap() {
        let mut child = std::process::Command::new("git")
            .args([
                "--no-optional-locks",
                "hash-object",
                "-t",
                object["kind"].as_str().unwrap(),
                "--stdin",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&decode_base64(object["bytes_base64"].as_str().unwrap()).unwrap())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        assert_eq!(object["oid"], String::from_utf8(out.stdout).unwrap().trim());
    }
    bundle["baseline_base64"] = json!(base64(b"{}"));
    let out = run_bundle_cli(
        "prepare-update-objects",
        &serde_json::to_vec(&bundle).unwrap(),
    );
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
}

#[test]
fn continuation_keeps_non_null_checkpoint_and_rejects_promotion() {
    use novena_bridge::planning::prepare_update_commit;
    let mut f = Fixture::new(false);
    let mut v = Status::parse(payload().bytes(), PreviousKind::Local)
        .unwrap()
        .document()
        .clone();
    v["last_verified"] = json!({"block_id":"BASE","git_head":"a".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["original.txt"]});
    let previous = Status::from_value(v.clone(), PreviousKind::Local).unwrap();
    let baseline = previous.canonical_bytes();
    let blob = object_oid(ObjectKind::Blob, &baseline);
    let leaf = f.put_tree(vec![e("NOVENA-CURRENT-STATE.json", Mode::File, blob)]);
    let middle = f.put_tree(vec![e("status", Mode::Directory, leaf)]);
    let root = f.put_tree(vec![e("novena", Mode::Directory, middle)]);
    let before = f.put_commit(root, None, "Existing checkpoint");
    let identity = "Novena Test <test@localhost.invalid> 1760000000 +0000";
    v["generated_at_utc"] = json!("2026-10-26T12:00:00Z");
    let candidate = Status::from_value(v.clone(), PreviousKind::Local).unwrap();
    let mut r = reader(Rc::new(RefCell::new(f.clone())), 256);
    let plan = prepare_update_commit(
        &mut r,
        &f.raw[&before],
        before,
        &baseline,
        &candidate,
        identity,
        b"Update\n",
    )
    .unwrap();
    let payload = &plan
        .objects
        .iter()
        .find(|o| matches!(o.kind, ObjectKind::Blob))
        .unwrap()
        .bytes;
    assert_eq!(
        Status::parse(payload, PreviousKind::Local)
            .unwrap()
            .document()["last_verified"],
        previous.document()["last_verified"]
    );
    for checkpoint in [
        Value::Null,
        json!({"block_id":"OTHER","git_head":"b".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["new.txt"]}),
    ] {
        v["last_verified"] = checkpoint;
        let candidate = Status::from_value(v.clone(), PreviousKind::Local).unwrap();
        assert_eq!(
            prepare_update_commit(
                &mut r,
                &f.raw[&before],
                before,
                &baseline,
                &candidate,
                identity,
                b"Update\n"
            )
            .unwrap_err(),
            BridgeError("CHECKPOINT_CHANGE_FORBIDDEN")
        );
    }
}

#[test]
fn update_transition_bundle_cli_strict_negative_vectors() {
    let (f, baseline, expected, before) = update_fixture();
    let commits: serde_json::Map<String, Value> = f
        .raw
        .iter()
        .map(|(id, b)| (id.to_string(), json!(base64(b))))
        .collect();
    let mut bundle = json!({"repository_id":"SYNTHETIC_REPOSITORY_ID","before":before.to_string(),
        "after":f.after.to_string(),"candidate":Status::parse(expected.bytes(),PreviousKind::Local).unwrap().document(),
        "baseline_base64":base64(&baseline),"responses":f.records,"raw_commits":commits});
    bundle["responses"][format!("{PREFIX}/git/ref/heads/main")] =
        json!({"ref":REF,"object":{"type":"commit","sha":f.after.to_string()}});
    let good = serde_json::to_vec(&bundle).unwrap();
    assert!(verify_update_bundle(&good).is_ok());
    let out = run_bundle_cli("verify-update-offline", &good);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["publication_performed"], false);
    assert_eq!(report["commit"], f.after.to_string());
    for variant in 0..10 {
        let mut bad = bundle.clone();
        match variant {
            0 => {
                bad["unexpected"] = json!(true);
            }
            1 => {
                bad.as_object_mut().unwrap().remove("baseline_base64");
            }
            2 => {
                bad["baseline_base64"] = json!(base64(b"{}"));
            }
            3 => {
                let mut bytes = baseline.clone();
                bytes.push(b'\n');
                bad["baseline_base64"] = json!(base64(&bytes));
            }
            4 => {
                bad["responses"][format!("{PREFIX}/git/ref/heads/main")]["object"]["sha"] =
                    json!(before.to_string());
            }
            5 => {
                bad["raw_commits"]
                    .as_object_mut()
                    .unwrap()
                    .remove(&before.to_string());
            }
            6 => {
                let bytes = &f.raw[&f.after];
                bad["raw_commits"][f.after.to_string()] = json!(base64(&bytes[..bytes.len() - 1]));
            }
            7 => {
                let oid = object_oid(ObjectKind::Blob, &baseline);
                bad["responses"]
                    .as_object_mut()
                    .unwrap()
                    .remove(&format!("{PREFIX}/git/blobs/{oid}"));
            }
            8 => {
                bad["responses"][format!("{PREFIX}/git/trees/{}", f.leaf)]["truncated"] =
                    json!(true);
            }
            9 => {
                bad["candidate"]["last_verified"] = json!({"block_id":"NEW","git_head":"a".repeat(40),"qualified_at_utc":"2026-10-26T00:00:00Z","qualification":"PASS","evidence_files":["x.txt"]});
            }
            _ => unreachable!(),
        }
        let out = run_bundle_cli("verify-update-offline", &serde_json::to_vec(&bad).unwrap());
        assert!(!out.status.success(), "variant {variant}");
        assert!(out.stdout.is_empty(), "variant {variant}");
    }
    // Even fully rehashed commits/trees must not change a sibling object.
    let mut changed = f.clone();
    let root = inspect_commit(&f.raw[&f.after]).unwrap().tree;
    let middle = Oid::parse(
        f.records[&format!("{PREFIX}/git/trees/{root}")]["tree"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["path"] == "novena")
            .unwrap()["sha"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    let tree = changed.put_tree(vec![
        e(
            "README.txt",
            Mode::File,
            object_oid(ObjectKind::Blob, b"changed"),
        ),
        e("novena", Mode::Directory, middle),
    ]);
    let after = changed.put_commit(tree, Some(before), "Forbidden sibling update");
    let mut bad = bundle.clone();
    bad["after"] = json!(after.to_string());
    bad["responses"] = json!(changed.records);
    bad["responses"][format!("{PREFIX}/git/ref/heads/main")] =
        json!({"ref":REF,"object":{"type":"commit","sha":after.to_string()}});
    bad["raw_commits"] = json!(changed
        .raw
        .iter()
        .map(|(id, b)| (id.to_string(), json!(base64(b))))
        .collect::<serde_json::Map<String, Value>>());
    let out = run_bundle_cli("verify-update-offline", &serde_json::to_vec(&bad).unwrap());
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    // Initial and snapshot adapters cannot silently consume the transition bundle.
    assert!(verify_offline_bundle(&good).is_err());
    assert!(novena_bridge::attestation::verify_bundle(&good).is_err());
}
