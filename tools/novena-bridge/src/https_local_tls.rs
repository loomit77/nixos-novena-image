//! Lokaler TLS-Gegenpart ist das vorhandene OpenSSL-Programm, keine zweite
//! Clientimplementierung. Zertifikate und Schlüssel entstehen nur im Test.
use super::*;
use crate::objects::*;
use crate::protocol::*;
use crate::status::*;
use serde_json::{json, Value};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Certificates(PathBuf);

impl Certificates {
    fn new(host: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "novena-local-tls-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        // Kein Wiederverwenden vorhandener Verzeichnisse oder Symlinks.
        fs::create_dir(&directory).unwrap();
        let certificates = Self(directory);
        for name in ["ca", "other"] {
            certificates.openssl(&[
                "req",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-noenc",
                "-days",
                "2",
                "-subj",
                "/CN=Novena synthetic test CA",
                "-addext",
                "basicConstraints=critical,CA:TRUE",
                "-addext",
                "keyUsage=critical,keyCertSign,cRLSign",
                "-keyout",
                &format!("{name}.key"),
                "-out",
                &format!("{name}.pem"),
            ]);
            certificates.openssl(&[
                "x509",
                "-in",
                &format!("{name}.pem"),
                "-outform",
                "DER",
                "-out",
                &format!("{name}.der"),
            ]);
        }
        certificates.openssl(&[
            "req",
            "-new",
            "-newkey",
            "rsa:2048",
            "-noenc",
            "-subj",
            "/CN=Novena synthetic server",
            "-keyout",
            "server.key",
            "-out",
            "server.csr",
        ]);
        fs::write(
            certificates.0.join("server.ext"),
            format!(
                "basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:{host}\n"
            ),
        )
        .unwrap();
        certificates.openssl(&[
            "x509",
            "-req",
            "-in",
            "server.csr",
            "-CA",
            "ca.pem",
            "-CAkey",
            "ca.key",
            "-set_serial",
            "2",
            "-days",
            "2",
            "-extfile",
            "server.ext",
            "-out",
            "server.pem",
        ]);
        certificates
    }

    fn openssl(&self, arguments: &[&str]) {
        let status = Command::new("openssl")
            .args(arguments)
            .current_dir(&self.0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Lokales OpenSSL wird für TLS-Tests benötigt");
        assert!(status.success(), "Testzertifikaterzeugung fehlgeschlagen");
    }
}

impl Drop for Certificates {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Server {
    child: Child,
    reader: Option<JoinHandle<()>>,
    requests: Arc<Mutex<Vec<Vec<u8>>>>,
    port: u16,
    stop: Option<mpsc::Sender<()>>,
}

type Responder = Box<dyn FnMut(&[u8]) -> Vec<u8> + Send>;

impl Server {
    fn start(certificates: &Certificates, reply: &'static [u8]) -> Self {
        Self::start_reply(certificates, reply.to_vec(), false)
    }

    fn start_reply(certificates: &Certificates, reply: Vec<u8>, close: bool) -> Self {
        Self::start_script(certificates, vec![(Duration::ZERO, reply)], close, false)
    }

    fn start_script(
        certificates: &Certificates,
        script: Vec<(Duration, Vec<u8>)>,
        close: bool,
        bounded_hold: bool,
    ) -> Self {
        Self::start_responder(certificates, script, close, bounded_hold, None)
    }

    fn start_responder(
        certificates: &Certificates,
        script: Vec<(Duration, Vec<u8>)>,
        close: bool,
        bounded_hold: bool,
        mut responder: Option<Responder>,
    ) -> Self {
        // Port wird vom OS gewählt. Die kurze Übergabelücke ist kein Probe-
        // Connect: Serverstartfehler führen zum Testfehler, nie zu Fallback/DNS.
        let reservation = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = reservation.local_addr().unwrap().port();
        drop(reservation);
        let mut child = Command::new("openssl")
            .args([
                "s_server",
                "-4",
                "-accept",
                &format!("127.0.0.1:{port}"),
                "-cert",
                "server.pem",
                "-key",
                "server.key",
                "-no_ign_eof",
                "-no-CAfile",
                "-no-CApath",
                "-no-CAstore",
            ])
            .current_dir(&certificates.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Lokaler OpenSSL-Testserver fehlt");
        let stdout = child.stdout.take().unwrap();
        let mut stdin = child.stdin.take().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let observed = requests.clone();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut input = BufReader::new(stdout);
            let mut line = Vec::new();
            let mut request = Vec::new();
            loop {
                line.clear();
                if !matches!(input.read_until(b'\n', &mut line), Ok(n) if n > 0) {
                    break;
                }
                if line == b"ACCEPT\n" || line == b"ACCEPT\r\n" {
                    let _ = ready_tx.send(());
                }
                if line.starts_with(b"GET ") || !request.is_empty() {
                    request.extend_from_slice(&line);
                    if request.len() > 64 * 1024 {
                        break;
                    }
                    if line == b"\r\n" || line == b"\n" {
                        let dynamic = responder
                            .as_mut()
                            .map(|reply| vec![(Duration::ZERO, reply(&request))]);
                        observed.lock().unwrap().push(std::mem::take(&mut request));
                        for (delay, bytes) in dynamic.as_ref().unwrap_or(&script) {
                            if !delay.is_zero()
                                && !matches!(
                                    stop_rx.recv_timeout(*delay),
                                    Err(mpsc::RecvTimeoutError::Timeout)
                                )
                            {
                                return;
                            }
                            if stdin.write_all(bytes).and_then(|()| stdin.flush()).is_err() {
                                return;
                            }
                        }
                        if bounded_hold {
                            // Fallback beendet auch bei einem defekten Client-
                            // Timeout den Server. Ein später EOF ist kein PASS:
                            // die Zeitassertion muss vorher erfüllt sein.
                            let _ = stop_rx.recv_timeout(Duration::from_secs(3));
                            return;
                        }
                        if close {
                            // EOF beendet s_server mit -no_ign_eof. Damit erhält
                            // der Client einen echten TLS-Abschluss statt eines
                            // erfundenen Read-Fehlers oder einer langen Wartezeit.
                            drop(stdin);
                            let _ = std::io::copy(&mut input, &mut std::io::sink());
                            return;
                        }
                    }
                }
            }
        });
        let server = Self {
            child,
            reader: Some(reader),
            requests,
            port,
            stop: Some(stop_tx),
        };
        ready_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("Loopback-TLS-Testserver nicht bereit");
        server
    }

    fn observed_requests(&self) -> Vec<Vec<u8>> {
        // Endliches Beobachtungsfenster, kein universeller Retrybeweis.
        thread::sleep(Duration::from_millis(250));
        self.requests.lock().unwrap().clone()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}

const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}";

#[path = "readback_fixture.rs"]
mod readback_fixture;

// Integration im Unit-Testziel: Nur hier ist die cfg(test)-Loopbackauflösung
// verfügbar. Sämtliche REST-Antworten durchlaufen den echten HTTPS-Client.
fn full_readback(case: usize) {
    use crate::offline_readback::{verify_complete_readback, RunBudget, RunScope};
    use crate::raw_commit_files::RawCommitFiles;
    use readback_fixture::{payload, Fixture};

    let certificates = Certificates::new("api.github.com");
    let mut f = Fixture::new(false);
    let side = object_oid(ObjectKind::Blob, b"preserved");
    if case >= 7 {
        f.records.insert(
            format!("{PREFIX}/git/blobs/{side}"),
            json!({"sha":side.to_string(),"encoding":"base64","size":9,"content":encode_base64(b"preserved")}),
        );
    }
    assert_eq!(f.head, f.after);
    let expected = payload();
    let original = expected.bytes().to_vec();
    // Separates privates Rohcommitverzeichnis, fertig vor Quellenkonstruktion.
    let directory = certificates.0.join("raw");
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&directory)
            .unwrap();
    }
    #[cfg(not(unix))]
    fs::create_dir(&directory).unwrap();
    for (id, bytes) in &f.raw {
        fs::write(directory.join(id.to_string()), bytes).unwrap();
    }
    let error = match case {
        0 => None,
        1 => {
            let mut wrong = f.raw[&f.after].clone();
            wrong.push(b'\n');
            fs::write(directory.join(f.after.to_string()), wrong).unwrap();
            Some("COMMIT_HASH_MISMATCH")
        }
        2 => {
            fs::remove_file(directory.join(f.before.to_string())).unwrap();
            Some("RAW_COMMIT_PATH_MISSING")
        }
        3 => {
            f.records
                .get_mut(&format!("{PREFIX}/git/commits/{}", f.after))
                .unwrap()["tree"]["sha"] = json!(f.leaf.to_string());
            Some("COMMIT_METADATA_MISMATCH")
        }
        4 => {
            let mut wrong = original.clone();
            wrong[0] ^= 1;
            f.records
                .get_mut(&format!("{PREFIX}/git/blobs/{}", f.blob))
                .unwrap()["content"] = json!(encode_base64(&wrong));
            Some("BLOB_HASH_MISMATCH")
        }
        5 => Some("FINAL_HEAD_MISMATCH"),
        6 => Some("HTTP_STATUS_REJECTED"),
        7 | 10 => None,
        8 => {
            f.records
                .get_mut(&format!("{PREFIX}/git/blobs/{side}"))
                .unwrap()["content"] = json!(encode_base64(b"tampered!"));
            Some("BLOB_HASH_MISMATCH")
        }
        9 => {
            f.records.remove(&format!("{PREFIX}/git/blobs/{side}"));
            Some("HTTP_STATUS_REJECTED")
        }
        11 => Some("RUN_BUDGET_EXCEEDED"),
        12 => Some("FINAL_HEAD_MISMATCH"),
        13 => {
            f.records
                .get_mut(&format!("{PREFIX}/git/trees/{}", f.leaf))
                .unwrap()["truncated"] = json!(true);
            Some("INCOMPLETE_OR_WRONG_TREE")
        }
        _ => unreachable!(),
    };
    f.raw.clear(); // Kein Speicherfallback für Originalcommitbytes.
    let (before, after, blob) = (f.before, f.after, f.blob);
    f.head_sequence.push_back(after);
    f.head_sequence.push_back(if case == 5 || case == 12 {
        before
    } else {
        after
    });
    let head_path = format!("{PREFIX}/git/ref/heads/main");
    let ref_path = head_path.clone();
    let server = Server::start_responder(
        &certificates,
        Vec::new(),
        false,
        false,
        Some(Box::new(move |request| {
            let request = std::str::from_utf8(request).unwrap();
            assert!(!request.to_ascii_lowercase().contains("authorization:"));
            let path = request
                .lines()
                .next()
                .unwrap()
                .strip_prefix("GET /")
                .unwrap()
                .strip_suffix(" HTTP/1.1")
                .unwrap();
            allowed_endpoint(path).unwrap();
            if case == 6 {
                return b"HTTP/1.1 503 Unavailable\r\nContent-Length: 2\r\n\r\n{}".to_vec();
            }
            let value = if path == ref_path {
                let head = f.head_sequence.pop_front().expect("Nur zwei Refabfragen");
                json!({"ref":REF,"object":{"type":"commit","sha":head.to_string()}})
            } else {
                match f.records.get(path) {
                    Some(value) => value.clone(),
                    None => return b"HTTP/1.1 404 Missing\r\nContent-Length: 2\r\n\r\n{}".to_vec(),
                }
            };
            let mut body = serde_json::to_vec(&value).unwrap();
            if case == 11 {
                body.resize(MAX_BODY, b' ');
            }
            let mut reply =
                format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
            reply.extend(body);
            reply
        })),
    );
    let root = fs::read(certificates.0.join("ca.der")).unwrap();
    let mut source = HttpsGetSource::local_test(&root, server.port);
    source.authorization = None;
    source.budget = ReceiveBudget::new(256, MAX_BODY).unwrap();
    if case >= 7 {
        if case == 11 {
            // ReceiveBudget verlangt eine freie EOF-Probe. Ein zusätzliches
            // Testbyte isoliert hier die strengere gemeinsame RunBudget-Grenze;
            // die Produktionskonfiguration bleibt unverändert.
            source.budget.remaining = TOTAL_BYTES + 1;
        }
        let budget = RunBudget::new();
        let mut responses = budget.responses(source);
        if case == 11 {
            // Vier echte GETs schöpfen denselben Kontext aus. Ein neuer Reader
            // darf weder den Gesamtzähler noch die Fehlersperre zurücksetzen.
            for _ in 0..4 {
                assert_eq!(responses.get(PREFIX).unwrap().len(), MAX_BODY);
            }
        }
        let mut reader = RestReader::new(responses, 256).unwrap();
        let files = RawCommitFiles::from_trusted_immutable_directory(&directory).unwrap();
        let mut raw = budget.commits(files);
        let result = verify_complete_readback(
            &mut reader,
            &mut raw,
            "SYNTHETIC_REPOSITORY_ID",
            before,
            after,
            &expected,
            None,
            &budget,
        );
        if let Some(error) = error {
            assert_eq!(result.unwrap_err(), BridgeError(error), "Fall {case}");
            let count = server.observed_requests().len();
            assert_eq!(reader.repository(), Err(BridgeError("RUN_ALREADY_FAILED")));
            assert_eq!(server.observed_requests().len(), count);
            if case == 11 {
                assert_eq!(count, 4);
            }
        } else {
            let result = result.unwrap();
            let replay = crate::offline_package::verify_package(result.package().bytes()).unwrap();
            assert_eq!(replay.evidence().bytes(), result.evidence().bytes());
            if case == 0 {
                use crate::object_evidence::EvidenceKind;
                use crate::offline_readback::{
                    AcquisitionManifest, ExportClaim, ReadbackMode, ReadbackSpec,
                };
                use crate::offline_run::{compare_replays, OfflineRun};
                use sha2::{Digest, Sha256};
                let transfer = certificates.0.join("offline-297");
                let mut builder = fs::DirBuilder::new();
                #[cfg(unix)]
                {
                    use std::os::unix::fs::DirBuilderExt;
                    builder.mode(0o700);
                }
                builder.create(&transfer).unwrap();
                crate::offline_package::export_private(result.package(), &transfer).unwrap();
                let imported = crate::offline_package::import_private(&transfer).unwrap();
                let package = imported.package().bytes();
                let digest: [u8; 32] = Sha256::digest(package).into();
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
                let budgets = [RunBudget::new(), RunBudget::new(), RunBudget::new()];
                let runs: Vec<_> = budgets
                    .iter()
                    .map(|budget| {
                        let spec = ReadbackSpec::new(
                            "SYNTHETIC_REPOSITORY_ID",
                            1,
                            after,
                            ReadbackMode::Snapshot {
                                expected: expected.bytes(),
                            },
                        )
                        .unwrap();
                        OfflineRun::new(spec, before, "tls-297", digest, budget).unwrap()
                    })
                    .collect();
                let bytes = runs[0].manifest_bytes(&result).unwrap();
                let manifest = AcquisitionManifest::untrusted(&bytes, &claims).unwrap();
                let a = runs[0].verify(&manifest, package).unwrap();
                let b = runs[1].verify(&manifest, package).unwrap();
                assert_eq!(
                    compare_replays(&a, &b).unwrap(),
                    "REPLAY_CONSISTENT_OFFLINE"
                );
                let mut wrong = crate::parse_exchange_v1(&bytes).unwrap();
                wrong["reference"] = json!("refs/heads/wrong");
                let wrong = crate::canonical_json(&serde_json::to_vec(&wrong).unwrap()).unwrap();
                let wrong = AcquisitionManifest::untrusted(&wrong, &claims).unwrap();
                assert!(runs[2].verify(&wrong, package).is_err());
                assert!(!a.complete().report().publication_performed);
            }
            assert!(!replay.report().publication_performed);
            assert_eq!(result.scope(), RunScope::Offline);
            assert_eq!(result.complete().blobs().len(), 2);
            assert_eq!(result.complete().paths(), 3);
            assert_eq!(result.complete().blobs()[&side].length, 9);
            assert_eq!(result.report().commit, after);
            assert!(!result.report().publication_performed);
            assert_eq!(result.report().scope, "OFFLINE_SUPPLIED_RESPONSES");
            let requests = server.observed_requests();
            let side_get = format!("GET /{PREFIX}/git/blobs/{side} HTTP/1.1\r\n");
            assert_eq!(
                requests
                    .iter()
                    .filter(|r| r.starts_with(side_get.as_bytes()))
                    .count(),
                1
            );
            let target_get = format!("GET /{PREFIX}/git/blobs/{blob} HTTP/1.1\r\n");
            // Gewöhnlicher Readback und Zusatzdurchlauf sind getrennt;
            // innerhalb der vollständigen Prüfung nur ein GET je Blob-OID.
            assert_eq!(
                requests
                    .iter()
                    .filter(|r| r.starts_with(target_get.as_bytes()))
                    .count(),
                2
            );
            assert_eq!(requests.len(), reader.requests());
            assert!(requests.iter().all(|r| !String::from_utf8_lossy(r)
                .to_ascii_lowercase()
                .contains("authorization:")));
        }
        assert_eq!(expected.bytes(), original);
        let status: Value = serde_json::from_slice(expected.bytes()).unwrap();
        assert_eq!(status["last_verified"], Value::Null);
        assert_eq!(status["publication"]["state"], "NOT_PUBLISHED");
        assert_eq!(
            production_publish(),
            Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
        );
        return;
    }
    let mut reader = RestReader::new(source, 256).unwrap();
    let mut raw = RawCommitFiles::from_trusted_immutable_directory(&directory).unwrap();
    let result = verify_initial_index(
        &mut reader,
        &mut raw,
        "SYNTHETIC_REPOSITORY_ID",
        before,
        after,
        &expected,
    );
    if let Some(error) = error {
        assert_eq!(result.unwrap_err(), BridgeError(error), "Fall {case}");
    } else {
        let report = result.unwrap();
        assert_eq!(report.commit, after);
        assert_eq!(report.blob, blob);
        assert!(report.commit_hash_verified);
        assert!(!report.publication_performed);
        assert_eq!(report.scope, "OFFLINE_SUPPLIED_RESPONSES");
        assert_eq!(report.as_json()["bridge_publication"], "BLOCKED");
    }
    if case == 6 {
        assert_eq!(reader.repository(), Err(BridgeError("HTTPS_SOURCE_CLOSED")));
    }
    let requests = server.observed_requests();
    assert!(!requests.is_empty());
    if case == 6 {
        assert_eq!(requests.len(), 1); // Auch expliziter Folgeaufruf sendet nichts.
    } else {
        assert_eq!(requests.len(), reader.requests());
    }
    for request in &requests {
        let text = String::from_utf8_lossy(request).to_ascii_lowercase();
        assert!(text.starts_with("get /"));
        assert!(text.contains(&format!("\r\nhost: api.github.com:{}\r\n", server.port)));
        assert!(!text.contains("authorization:"));
    }
    if case == 0 || case == 5 {
        let head_get = format!("GET /{head_path} HTTP/1.1\r\n");
        assert_eq!(
            requests
                .iter()
                .filter(|r| r.starts_with(head_get.as_bytes()))
                .count(),
            2
        );
        for id in [before, after] {
            let get = format!("GET /{PREFIX}/git/commits/{id} HTTP/1.1\r\n");
            assert!(requests.iter().any(|r| r.starts_with(get.as_bytes())));
        }
        let get = format!("GET /{PREFIX}/git/blobs/{blob} HTTP/1.1\r\n");
        assert!(requests.iter().any(|r| r.starts_with(get.as_bytes())));
    }
    assert_eq!(raw.provenance(), "LOCAL_EXPLICIT_RAW_COMMIT_FILES");
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
fn local_tls_full_readback_raw_files() {
    full_readback(0);
}
#[test]
fn local_tls_complete_readback_multiple_blobs_raw_files() {
    full_readback(7);
}
#[test]
fn local_tls_complete_readback_tampered_side_blob() {
    full_readback(8);
}
#[test]
fn local_tls_complete_readback_missing_side_blob() {
    full_readback(9);
}
#[test]
fn local_tls_complete_readback_repeated_oid_single_side_get() {
    full_readback(10);
}
#[test]
fn local_tls_shared_budget_exhaustion_prevents_next_get() {
    full_readback(11);
}
#[test]
fn local_tls_complete_readback_final_ref_changed() {
    full_readback(12);
}
#[test]
fn local_tls_complete_readback_incomplete_tree_no_evidence() {
    full_readback(13);
}
#[test]
fn local_tls_readback_wrong_raw_commit() {
    full_readback(1);
}
#[test]
fn local_tls_readback_missing_raw_commit() {
    full_readback(2);
}
#[test]
fn local_tls_readback_wrong_tree_metadata() {
    full_readback(3);
}
#[test]
fn local_tls_readback_wrong_blob() {
    full_readback(4);
}
#[test]
fn local_tls_readback_ref_changed() {
    full_readback(5);
}
#[test]
fn local_tls_readback_failed_get_no_retry() {
    full_readback(6);
}

fn rejected(host: &str, root_name: &str, reply: &'static [u8], code: &'static str, count: usize) {
    let certificates = Certificates::new(host);
    let server = Server::start(&certificates, reply);
    let root = fs::read(certificates.0.join(root_name)).unwrap();
    let mut source = HttpsGetSource::local_test(&root, server.port);
    let error = source.get(PREFIX).unwrap_err();
    assert_eq!(error, BridgeError(code));
    let rendered = format!("{error} {error:?}");
    for secret in ["SYNTHETIC_SECRET", "RESPONSE_SECRET", "LOCATION_SECRET"] {
        assert!(!rendered.contains(secret));
    }
    assert_eq!(source.get(PREFIX), Err(BridgeError("HTTPS_SOURCE_CLOSED")));
    assert_eq!(server.observed_requests().len(), count);
}

#[test]
fn local_tls_trusted_ca_handshake_and_single_get() {
    let certificates = Certificates::new("api.github.com");
    let server = Server::start(&certificates, OK);
    let root = fs::read(certificates.0.join("ca.der")).unwrap();
    let mut source = HttpsGetSource::local_test(&root, server.port);
    assert_eq!(source.get(PREFIX).unwrap(), b"{}");
    assert_eq!(
        source.get(PREFIX),
        Err(BridgeError("REQUEST_LIMIT_EXCEEDED"))
    );
    let requests = server.observed_requests();
    assert_eq!(requests.len(), 1);
    let request = String::from_utf8_lossy(&requests[0]);
    assert!(request.starts_with(&format!("GET /{PREFIX} HTTP/1.1\r\n")));
    assert!(request
        .to_ascii_lowercase()
        .contains(&format!("\r\nhost: api.github.com:{}\r\n", server.port)));
    assert!(request.contains("Bearer SYNTHETIC_SECRET"));
}

#[test]
fn local_tls_unknown_ca_rejected_before_http() {
    rejected("api.github.com", "other.der", OK, "HTTPS_REQUEST_FAILED", 0);
}

#[test]
fn local_tls_wrong_hostname_rejected_before_http() {
    rejected("wrong.invalid", "ca.der", OK, "HTTPS_REQUEST_FAILED", 0);
}

#[test]
fn local_tls_http_redirect_rejected_without_followup_or_secret_leak() {
    rejected(
        "api.github.com",
        "ca.der",
        b"HTTP/1.1 302 Found\r\nLocation: http://api.github.com/LOCATION_SECRET\r\nContent-Length: 15\r\nConnection: close\r\n\r\nRESPONSE_SECRET",
        "HTTP_REDIRECT_REJECTED",
        1,
    );
}

#[test]
fn local_tls_malformed_http_rejected_without_retry_or_secret_leak() {
    rejected(
        "api.github.com",
        "ca.der",
        b"HTTP/1.1 INVALID_RESPONSE_SECRET\r\n\r\n",
        "HTTPS_REQUEST_FAILED",
        1,
    );
}

// Integrationsfälle im Unit-Testmodul: Zugriff auf den ausschließlich unter
// cfg(test) vorhandenen Loopback-Konstruktor, echter unveränderter fetch-Pfad.
// Vier erlaubte Versuche verhindern, dass das Requestlimit die Fehlersperre
// beim zweiten Aufruf lediglich verdeckt.
fn receive_case(reply: Vec<u8>, close: bool, expected: BridgeResult<Vec<u8>>) {
    let certificates = Certificates::new("api.github.com");
    let server = Server::start_reply(&certificates, reply, close);
    let root = fs::read(certificates.0.join("ca.der")).unwrap();
    let mut source = HttpsGetSource::local_test(&root, server.port);
    source.budget = ReceiveBudget::new(4, 16).unwrap();
    let result = source.get(PREFIX);
    assert_eq!(result, expected);
    if let Err(error) = result {
        let rendered = format!("{error} {error:?}");
        for secret in ["SYNTHETIC_SECRET", "RESPONSE_SECRET", "HEADER_SECRET"] {
            assert!(!rendered.contains(secret));
        }
        assert_eq!(source.budget.attempts, 1);
        assert_eq!(source.get(PREFIX), Err(BridgeError("HTTPS_SOURCE_CLOSED")));
        assert_eq!(source.budget.attempts, 1);
    }
    assert_eq!(server.observed_requests().len(), 1);
}

#[test]
fn local_tls_body_exact_limit_with_content_length() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nContent-Length: 16\r\n\r\n0123456789abcdef".to_vec(),
        false,
        Ok(b"0123456789abcdef".to_vec()),
    );
}

#[test]
fn local_tls_chunked_body_exact_limit() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n8\r\n01234567\r\n8\r\n89abcdef\r\n0\r\n\r\n".to_vec(),
        false,
        Ok(b"0123456789abcdef".to_vec()),
    );
}

#[test]
fn local_tls_declared_body_above_limit() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nContent-Length: 17\r\n\r\nRESPONSE_SECRETxxx".to_vec(),
        false,
        Err(BridgeError("RESPONSE_TOO_LARGE")),
    );
}

#[test]
fn local_tls_streamed_chunked_body_above_limit() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n11\r\nRESPONSE_SECRETxxx\r\n0\r\n\r\n".to_vec(),
        false,
        Err(BridgeError("RESPONSE_TOO_LARGE")),
    );
}

#[test]
fn local_tls_incomplete_content_length_body() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nContent-Length: 16\r\n\r\nRESPONSE_SECRET".to_vec(),
        true,
        Err(BridgeError("HTTPS_BODY_READ_FAILED")),
    );
}

#[test]
fn local_tls_connection_ends_before_chunk_terminator() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\ne\r\nRESPONSE_SECRET\r\n".to_vec(),
        true,
        Err(BridgeError("HTTPS_BODY_READ_FAILED")),
    );
}

#[test]
fn local_tls_unexpected_content_encoding() {
    receive_case(
        b"HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 14\r\n\r\nRESPONSE_SECRET"
            .to_vec(),
        false,
        Err(BridgeError("HTTP_ENCODING_REJECTED")),
    );
}

#[test]
fn local_tls_http_error_status() {
    receive_case(
        b"HTTP/1.1 503 Unavailable\r\nContent-Length: 14\r\n\r\nRESPONSE_SECRET".to_vec(),
        false,
        Err(BridgeError("HTTP_STATUS_REJECTED")),
    );
}

#[test]
fn local_tls_larger_header_with_small_body() {
    let reply = format!(
        "HTTP/1.1 200 OK\r\nX-Synthetic: HEADER_SECRET{}\r\nContent-Length: 2\r\n\r\n{{}}",
        "x".repeat(8192)
    );
    receive_case(reply.into_bytes(), false, Ok(b"{}".to_vec()));
}

#[test]
fn local_tls_excessive_header_count() {
    let mut reply = String::from("HTTP/1.1 200 OK\r\n");
    for index in 0..101 {
        reply.push_str(&format!("X-Synthetic-{index}: HEADER_SECRET\r\n"));
    }
    reply.push_str("Content-Length: 2\r\n\r\n{}");
    receive_case(
        reply.into_bytes(),
        false,
        Err(BridgeError("HTTPS_REQUEST_FAILED")),
    );
}

// Charakterisierung der API-Lücke, keine Sicherheitsabnahme einer Bytegrenze.
// Explizit gezählte Wirebytes: Statuszeile, Headerzeilen und Abschluss-CRLF;
// der zweibytegroße Body zählt nicht mit. Alle Eingaben bleiben endlich.
fn sized_header_reply(head_bytes: usize, long_status: bool) -> Vec<u8> {
    assert!((128..=512 * 1024).contains(&head_bytes));
    let (prefix, suffix) = if long_status {
        ("HTTP/1.1 200 ", "\r\nContent-Length: 2\r\n\r\n")
    } else {
        (
            "HTTP/1.1 200 OK\r\nX-Unknown: HEADER_SECRET",
            "\r\nContent-Length: 2\r\n\r\n",
        )
    };
    let mut reply = prefix.as_bytes().to_vec();
    reply.resize(head_bytes - suffix.len(), b'x');
    reply.extend_from_slice(suffix.as_bytes());
    assert_eq!(reply.len(), head_bytes);
    reply.extend_from_slice(b"{}");
    reply
}

#[test]
fn local_tls_header_just_below_32k_accepted() {
    receive_case(
        sized_header_reply(32 * 1024 - 1, false),
        false,
        Ok(b"{}".to_vec()),
    );
}

#[test]
fn local_tls_header_above_32k_currently_accepted() {
    receive_case(
        sized_header_reply(32 * 1024 + 1, false),
        false,
        Ok(b"{}".to_vec()),
    );
}

#[test]
fn local_tls_single_64k_header_currently_accepted() {
    receive_case(
        sized_header_reply(64 * 1024, false),
        false,
        Ok(b"{}".to_vec()),
    );
}

#[test]
fn local_tls_long_status_line_above_32k_currently_accepted() {
    receive_case(
        sized_header_reply(40 * 1024, true),
        false,
        Ok(b"{}".to_vec()),
    );
}

#[test]
fn local_tls_100_headers_above_32k_currently_accepted() {
    let mut reply = String::from("HTTP/1.1 200 OK\r\n");
    for index in 0..99 {
        reply.push_str(&format!("X-Unknown-{index}: {}\r\n", "x".repeat(400)));
    }
    reply.push_str("Content-Length: 2\r\n\r\n");
    assert!(reply.len() > 32 * 1024);
    reply.push_str("{}");
    receive_case(reply.into_bytes(), false, Ok(b"{}".to_vec()));
}

#[test]
fn local_tls_101_small_headers_rejected_by_count() {
    let mut reply = String::from("HTTP/1.1 200 OK\r\n");
    for _ in 0..100 {
        reply.push_str("X-Unknown: x\r\n");
    }
    reply.push_str("Content-Length: 2\r\n\r\n{}");
    assert!(reply.len() < 32 * 1024);
    receive_case(
        reply.into_bytes(),
        false,
        Err(BridgeError("HTTPS_REQUEST_FAILED")),
    );
}

#[test]
fn local_tls_incomplete_large_header_eof_rejected() {
    let mut reply = b"HTTP/1.1 200 OK\r\nX-Unknown: HEADER_SECRET".to_vec();
    reply.resize(64 * 1024, b'x');
    receive_case(reply, true, Err(BridgeError("HTTPS_REQUEST_FAILED")));
}

#[test]
fn local_tls_incomplete_header_above_default_buffer_rejected() {
    let mut reply = b"HTTP/1.1 200 OK\r\nX-Unknown: HEADER_SECRET".to_vec();
    reply.resize(512 * 1024, b'x');
    // EOF bleibt als Fallback erhalten. Dieser Test allein unterscheidet
    // Pufferablehnung nicht von EOF und misst keine Heap-Höchstbelegung.
    receive_case(reply, true, Err(BridgeError("HTTPS_REQUEST_FAILED")));
}

#[test]
fn local_tls_header_slow_drip_does_not_restart_deadline() {
    let mut script = vec![(
        Duration::ZERO,
        b"HTTP/1.1 200 OK\r\nX-Unknown: HEADER_SECRET".to_vec(),
    )];
    script.extend((0..50).map(|_| (Duration::from_millis(50), vec![b'x'])));
    timed_receive_case(script, true, false, false);
}

// Echte HTTPS-Zeitfälle: kein synthetisches Read und kein Client-Workerthread.
// Der Server schließt selbst nach endlicher Frist; Drop unterbricht seine
// Verzögerungen und wartet auf den Leserthread. Der Server-Fallback liegt
// außerhalb der erlaubten Clientlaufzeit und kann keinen Scheinerfolg liefern.
fn timed_receive_case(
    script: Vec<(Duration, Vec<u8>)>,
    waiting_for_headers: bool,
    expect_body_progress: bool,
    operation_limit: bool,
) {
    let certificates = Certificates::new("api.github.com");
    let server = Server::start_script(&certificates, script, false, true);
    let root = fs::read(certificates.0.join("ca.der")).unwrap();
    let mut source = HttpsGetSource::local_test(&root, server.port);
    source.budget = ReceiveBudget::new(4, 1024).unwrap();
    let timeout = Duration::from_millis(600);
    source.budget.request_timeout = if operation_limit {
        Duration::from_secs(2)
    } else {
        timeout
    };
    let started = Instant::now();
    if operation_limit {
        source.budget.started = Some(started - (OPERATION_TIMEOUT - timeout));
    }
    let error = source.get(PREFIX).unwrap_err();
    let elapsed = started.elapsed();
    // 1,2 s Jittertoleranz, aber deutlich vor dem 3-s-Server-Fallback.
    assert!(elapsed < Duration::from_millis(1800), "{elapsed:?}");
    assert!(elapsed >= Duration::from_millis(300), "{elapsed:?}");
    if waiting_for_headers {
        assert_eq!(error, BridgeError("HTTPS_REQUEST_FAILED"));
    } else {
        assert!(
            error == BridgeError("HTTPS_BODY_READ_FAILED") || error == BridgeError("HTTP_TIMEOUT"),
            "{error:?}"
        );
    }
    if expect_body_progress {
        assert!(source.budget.remaining < TOTAL_BYTES);
    }
    assert_eq!(source.budget.attempts, 1);
    assert_eq!(source.get(PREFIX), Err(BridgeError("HTTPS_SOURCE_CLOSED")));
    assert_eq!(source.budget.attempts, 1);
    assert_eq!(server.observed_requests().len(), 1);
}

#[test]
fn local_tls_headers_then_delayed_body_aborts_while_waiting() {
    timed_receive_case(
        vec![
            (
                Duration::ZERO,
                b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n".to_vec(),
            ),
            (Duration::from_secs(3), b"{}".to_vec()),
        ],
        false,
        false,
        false,
    );
}

#[test]
fn local_tls_slow_drip_does_not_restart_request_deadline() {
    let mut script = vec![(
        Duration::ZERO,
        b"HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\nx".to_vec(),
    )];
    script.extend((0..50).map(|_| (Duration::from_millis(50), vec![b'x'])));
    timed_receive_case(script, false, true, false);
}

#[test]
fn local_tls_unterminated_chunked_response_aborts_while_waiting() {
    timed_receive_case(
        vec![(
            Duration::ZERO,
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1\r\nx\r\n".to_vec(),
        )],
        false,
        true,
        false,
    );
}

#[test]
fn local_tls_operation_remainder_limits_blocking_body_read() {
    timed_receive_case(
        vec![(
            Duration::ZERO,
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nx".to_vec(),
        )],
        false,
        true,
        true,
    );
}

#[test]
fn local_tls_missing_complete_headers_aborts_while_waiting() {
    timed_receive_case(
        vec![(
            Duration::ZERO,
            b"HTTP/1.1 200 OK\r\nX-Synthetic: incomplete".to_vec(),
        )],
        true,
        false,
        false,
    );
}
