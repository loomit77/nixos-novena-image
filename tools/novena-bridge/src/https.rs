//! Explizite HTTPS-GET-Quelle, ohne CLI-Anbindung oder RawCommitSource.
//!
//! Keine Live-Qualifikation. Feste Headeraufnahme vor Hypers HTTP/1-Parser.
//! Eine monotone Tokio-Deadline umfasst Verbindung, TLS, Header und Body.
use crate::protocol::{allowed_endpoint, ResponseSource};
use crate::{BridgeError, BridgeResult};
use http_body_util::BodyExt;
use std::sync::Arc;
#[path = "https_header.rs"]
mod header;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_ENCODING, LOCATION};
#[cfg(test)]
use std::io::Read;
use std::time::{Duration, Instant};

const TOTAL_BYTES: usize = 32 * 1024 * 1024;
const MAX_BODY: usize = 8 * 1024 * 1024;
const OPERATION_TIMEOUT: Duration = Duration::from_secs(120);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

fn target(endpoint: &str) -> BridgeResult<String> {
    allowed_endpoint(endpoint)?;
    Ok(format!("https://api.github.com/{endpoint}"))
}

/// Kein Default/Debug/Serialize und kein übergebbarer Client oder freier URL.
/// Konstruktion sendet nichts; erst ResponseSource::get führt einen GET aus.
/// CA-DER-Bytes und optionales Token stammen ausschließlich vom Aufrufer.
/// Der Aufrufer muss die freigegebene CA-Provenienz und Leserechte sicherstellen.
/// Keine Umgebungs-, Datei-, gh-, Git-, Cookie- oder Credential-Erkennung.
pub struct HttpsGetSource {
    tls: Arc<rustls::ClientConfig>,
    authorization: Option<HeaderValue>,
    budget: ReceiveBudget,
    #[cfg(test)]
    local_port: Option<u16>,
}

impl HttpsGetSource {
    pub fn new(
        roots_der: &[&[u8]],
        token: Option<&str>,
        request_limit: usize,
        response_limit: usize,
    ) -> BridgeResult<Self> {
        let budget = ReceiveBudget::new(request_limit, response_limit)?;
        if roots_der.is_empty()
            || roots_der.len() > 128
            || roots_der
                .iter()
                .any(|der| der.is_empty() || der.len() > 64 * 1024)
        {
            return Err(BridgeError("INVALID_TLS_ROOTS"));
        }
        let authorization = authorization(token)?;
        let mut roots = rustls::RootCertStore::empty();
        for der in roots_der {
            roots
                .add(rustls::pki_types::CertificateDer::from(der.to_vec()))
                .map_err(|_| BridgeError("INVALID_TLS_ROOTS"))?;
        }
        let tls = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .map_err(|_| BridgeError("HTTPS_CLIENT_INIT_FAILED"))?
        .with_root_certificates(roots)
        .with_no_client_auth();
        Ok(Self {
            tls: Arc::new(tls),
            authorization,
            budget,
            #[cfg(test)]
            local_port: None,
        })
    }

    /// Nur Unit-Tests: fester API-Hostname/SNI, ausschließlich IPv4-Loopback.
    #[cfg(test)]
    fn local_test(root: &[u8], port: u16) -> Self {
        assert_ne!(port, 0);
        let mut source = Self::new(&[root], Some("SYNTHETIC_SECRET"), 1, 1024).unwrap();
        source.local_port = Some(port);
        source
    }

    fn fetch(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>> {
        // Prüfung vor Normalisierung, Versuch und jeder Netzwerkausführung.
        let url = target(endpoint)?;
        #[cfg(test)]
        let url = if let Some(port) = self.local_port {
            let mut url = reqwest::Url::parse(&url).unwrap();
            url.set_port(Some(port)).unwrap();
            url.to_string()
        } else {
            url
        };
        let deadline = self.budget.begin(Instant::now())?;
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(BridgeError("HTTP_TIMEOUT"));
        }
        // Runtime erst bei explizitem GET; Konstruktion hat keine Socketwirkung.
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| BridgeError("HTTPS_CLIENT_INIT_FAILED"))?;
        runtime.block_on(async {
            let deadline = tokio::time::Instant::from_std(deadline);
            let headers_received = std::cell::Cell::new(false);
            let work = async {
                let parsed =
                    reqwest::Url::parse(&url).map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                let port = parsed.port().unwrap_or(443);
                #[cfg(test)]
                let host = if self.local_port.is_some() {
                    "127.0.0.1"
                } else {
                    "api.github.com"
                };
                #[cfg(not(test))]
                let host = "api.github.com";
                let tcp = tokio::time::timeout(
                    Duration::from_secs(10),
                    tokio::net::TcpStream::connect((host, port)),
                )
                .await
                .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?
                .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                let name = rustls::pki_types::ServerName::try_from("api.github.com").unwrap();
                let tls = tokio_rustls::TlsConnector::from(self.tls.clone())
                    .connect(name, tcp)
                    .await
                    .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                let io = hyper_util::rt::TokioIo::new(header::HeaderGate::new(tls));
                let (mut sender, connection) = hyper::client::conn::http1::Builder::new()
                    .max_headers(100)
                    .max_buf_size(header::MAX_HEADER_BYTES)
                    .handshake(io)
                    .await
                    .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                let driver = tokio::spawn(connection);
                // Drop bricht den einzigen Connectiontask auch bei Fehler/Timeout ab.
                struct Abort(tokio::task::JoinHandle<Result<(), hyper::Error>>);
                impl Drop for Abort {
                    fn drop(&mut self) {
                        self.0.abort();
                    }
                }
                let _driver = Abort(driver);
                let mut request = hyper::Request::builder()
                    .method("GET")
                    .uri(parsed.path())
                    .header("Host", parsed.authority())
                    .header("Accept", "application/vnd.github+json")
                    .header("X-GitHub-Api-Version", "2022-11-28")
                    .header("User-Agent", "novena-bridge/0.1")
                    .header("Accept-Encoding", "identity")
                    .header("Connection", "close");
                if let Some(value) = &self.authorization {
                    request = request.header(AUTHORIZATION, value.clone());
                }
                let request = request
                    .body(http_body_util::Empty::<bytes::Bytes>::new())
                    .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                let response = sender
                    .send_request(request)
                    .await
                    .map_err(|_| BridgeError("HTTPS_REQUEST_FAILED"))?;
                headers_received.set(true);
                check_headers(response.status().as_u16(), response.headers())?;
                if response
                    .headers()
                    .get(reqwest::header::CONTENT_LENGTH)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .is_some_and(|n| {
                        n > self.budget.response_limit.min(self.budget.remaining) as u64
                    })
                {
                    return Err(BridgeError("RESPONSE_TOO_LARGE"));
                }
                // Dieselbe begrenzte Empfangsschleife wie in den Budgettests.
                let mut body = response.into_body();
                let mut pending = bytes::Bytes::new();
                let mut bytes = Vec::new();
                let mut buffer = [0u8; 8192];
                loop {
                    let size = self.budget.read_size(bytes.len(), buffer.len())?;
                    while pending.is_empty() {
                        match body.frame().await {
                            Some(Ok(frame)) => {
                                if let Ok(data) = frame.into_data() {
                                    pending = data;
                                }
                            }
                            Some(Err(_)) => return Err(BridgeError("HTTPS_BODY_READ_FAILED")),
                            None => return self.budget.finish(bytes),
                        }
                    }
                    let count = size.min(pending.len());
                    buffer[..count].copy_from_slice(&pending.split_to(count));
                    self.budget.append(&mut bytes, &buffer[..count])?;
                }
            };
            tokio::time::timeout_at(deadline, work).await.map_err(|_| {
                BridgeError(if headers_received.get() {
                    "HTTP_TIMEOUT"
                } else {
                    "HTTPS_REQUEST_FAILED"
                })
            })?
        })
    }
}

#[cfg(test)]
#[path = "https_local_tls.rs"]
mod local_tls;

impl ResponseSource for HttpsGetSource {
    fn get(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>> {
        let result = self.fetch(endpoint);
        // Nach jedem Fehler geschlossen: auch unvollständige/verworfene Bodies
        // erlauben keine weiteren Requests und kein Budget-Recycling.
        if result.is_err() {
            self.budget.failed = true;
        }
        result
    }
}

fn authorization(token: Option<&str>) -> BridgeResult<Option<HeaderValue>> {
    token
        .map(|token| {
            if token.is_empty()
                || token.len() > 4096
                || !token.bytes().all(|b| b.is_ascii_graphic())
            {
                return Err(BridgeError("INVALID_HTTP_CREDENTIAL"));
            }
            let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|_| BridgeError("INVALID_HTTP_CREDENTIAL"))?;
            value.set_sensitive(true);
            Ok(value)
        })
        .transpose()
}

fn check_headers(status: u16, headers: &HeaderMap) -> BridgeResult<()> {
    if (300..400).contains(&status) || headers.contains_key(LOCATION) {
        return Err(BridgeError("HTTP_REDIRECT_REJECTED"));
    }
    if status != 200 {
        return Err(BridgeError("HTTP_STATUS_REJECTED"));
    }
    if headers
        .get_all(CONTENT_ENCODING)
        .iter()
        .any(|v| v.as_bytes() != b"identity")
    {
        return Err(BridgeError("HTTP_ENCODING_REJECTED"));
    }
    Ok(())
}

struct ReceiveBudget {
    attempts: usize,
    request_limit: usize,
    response_limit: usize,
    remaining: usize,
    started: Option<Instant>,
    failed: bool,
    request_timeout: Duration,
}

impl ReceiveBudget {
    fn new(request_limit: usize, response_limit: usize) -> BridgeResult<Self> {
        if !(1..=256).contains(&request_limit) || !(1..=MAX_BODY).contains(&response_limit) {
            return Err(BridgeError("INVALID_HTTP_LIMITS"));
        }
        Ok(Self {
            attempts: 0,
            request_limit,
            response_limit,
            remaining: TOTAL_BYTES,
            started: None,
            failed: false,
            request_timeout: REQUEST_TIMEOUT,
        })
    }

    fn begin(&mut self, now: Instant) -> BridgeResult<Instant> {
        if self.failed {
            return Err(BridgeError("HTTPS_SOURCE_CLOSED"));
        }
        if self.attempts >= self.request_limit {
            return Err(BridgeError("REQUEST_LIMIT_EXCEEDED"));
        }
        if self.remaining == 0 {
            return Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"));
        }
        let operation = *self.started.get_or_insert(now) + OPERATION_TIMEOUT;
        if now >= operation {
            return Err(BridgeError("HTTP_TIMEOUT"));
        }
        self.attempts += 1;
        Ok(operation.min(now + self.request_timeout))
    }

    fn read_size(&self, length: usize, buffer: usize) -> BridgeResult<usize> {
        let size = buffer
            .min(self.response_limit - length + 1)
            .min(self.remaining);
        if size == 0 {
            return Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"));
        }
        Ok(size)
    }
    fn append(&mut self, body: &mut Vec<u8>, data: &[u8]) -> BridgeResult<()> {
        self.remaining -= data.len();
        if data.len() > self.response_limit - body.len() {
            return Err(BridgeError("RESPONSE_TOO_LARGE"));
        }
        body.extend_from_slice(data);
        Ok(())
    }
    fn finish(&self, body: Vec<u8>) -> BridgeResult<Vec<u8>> {
        if body.is_empty() {
            Err(BridgeError("INVALID_HTTP_BODY"))
        } else {
            Ok(body)
        }
    }
    #[cfg(test)]
    fn receive(
        &mut self,
        reader: &mut impl Read,
        deadline: Instant,
        mut now: impl FnMut() -> Instant,
    ) -> BridgeResult<Vec<u8>> {
        let mut body = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            if now() >= deadline {
                return Err(BridgeError("HTTP_TIMEOUT"));
            }
            // Höchstens eine zusätzliche Byteprobe pro Antwort. Auch die Probe
            // wird vom Gesamtbudget abgezogen, nie über dessen Grenze gelesen.
            let size = self.read_size(body.len(), buffer.len())?;
            let count = reader
                .read(&mut buffer[..size])
                .map_err(|_| BridgeError("HTTPS_BODY_READ_FAILED"))?;
            // Abrechnung vor dem nachgelagerten Deadlinefehler erhalten.
            self.append(&mut body, &buffer[..count])?;
            if now() >= deadline {
                return Err(BridgeError("HTTP_TIMEOUT"));
            }
            if count == 0 {
                return self.finish(body);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::PREFIX;
    use std::io::{self, Cursor};

    #[test]
    fn exact_targets_and_no_url_normalization() {
        for endpoint in [
            PREFIX.to_owned(),
            format!("{PREFIX}/git/ref/heads/main"),
            format!("{PREFIX}/git/commits/{}", "a".repeat(40)),
            format!("{PREFIX}/git/trees/{}", "b".repeat(40)),
            format!("{PREFIX}/git/blobs/{}", "c".repeat(40)),
        ] {
            let url = reqwest::Url::parse(&target(&endpoint).unwrap()).unwrap();
            assert_eq!(url.scheme(), "https");
            assert_eq!(url.host_str(), Some("api.github.com"));
            assert_eq!(url.port(), None);
            assert_eq!(url.path(), format!("/{endpoint}"));
            assert!(url.username().is_empty());
            assert!(url.query().is_none());
            assert!(url.fragment().is_none());
        }
        for endpoint in [
            format!("https://api.github.com/{PREFIX}"),
            format!("http://api.github.com/{PREFIX}"),
            format!("https://api.github.com.evil.invalid/{PREFIX}"),
            format!("{PREFIX}?secret=TOKEN"),
            format!("{PREFIX}#TOKEN"),
            format!("{PREFIX}/../project-block-logs"),
            format!("{PREFIX}/git/trees/{}?recursive=1", "a".repeat(40)),
            format!("{PREFIX}/git/commits/{}", "A".repeat(40)),
            format!("{PREFIX}/contents/file"),
            format!("/{PREFIX}"),
            "graphql".into(),
            "repos/other/repository".into(),
            format!("{PREFIX}/%67it/ref/heads/main"),
        ] {
            let error = target(&endpoint).unwrap_err();
            assert!(!format!("{error:?} {error}").contains("TOKEN"));
        }
    }

    #[test]
    fn transport_policy_has_no_implicit_roots_or_redirect_followup() {
        // Konstruktion ohne Runtime, DNS oder Socket. Explizite Roots sind Pflicht.
        assert!(HttpsGetSource::new(&[], None, 1, 2).is_err());
        assert!(HttpsGetSource::new(&[b"invalid"], None, 1, 2).is_err());
        let mut headers = HeaderMap::new();
        headers.insert(LOCATION, HeaderValue::from_static("https://other.invalid"));
        assert_eq!(
            check_headers(200, &headers),
            Err(BridgeError("HTTP_REDIRECT_REJECTED"))
        );
    }

    #[test]
    fn credentials_are_explicit_sensitive_and_errors_are_static() {
        assert!(authorization(None).unwrap().is_none());
        let value = authorization(Some("SYNTHETIC_SECRET")).unwrap().unwrap();
        assert!(value.is_sensitive());
        assert!(!format!("{value:?}").contains("SYNTHETIC_SECRET"));
        for token in ["", "SECRET\r\nInjected: yes", "SECRET TOKEN", "éSECRET"] {
            let error = authorization(Some(token)).unwrap_err();
            assert_eq!(error, BridgeError("INVALID_HTTP_CREDENTIAL"));
            assert!(!format!("{error:?} {error}").contains("SECRET"));
        }
    }

    #[test]
    fn statuses_locations_and_encodings_fail_closed() {
        let mut headers = HeaderMap::new();
        assert!(check_headers(200, &headers).is_ok());
        for status in [204, 206, 301, 302, 304, 307, 308, 401, 403, 429, 500] {
            assert!(check_headers(status, &headers).is_err());
        }
        headers.insert(
            LOCATION,
            HeaderValue::from_static("https://secret.invalid/TOKEN"),
        );
        let error = check_headers(200, &headers).unwrap_err();
        assert_eq!(error, BridgeError("HTTP_REDIRECT_REJECTED"));
        assert!(!format!("{error:?}").contains("TOKEN"));
        headers.clear();
        headers.insert(CONTENT_ENCODING, HeaderValue::from_static("identity"));
        assert!(check_headers(200, &headers).is_ok());
        headers.append(CONTENT_ENCODING, HeaderValue::from_static("gzip"));
        assert_eq!(
            check_headers(200, &headers),
            Err(BridgeError("HTTP_ENCODING_REJECTED"))
        );
    }

    #[test]
    fn finite_attempts_and_monotonic_deadline_checks() {
        for (requests, bytes) in [(0, 1), (257, 1), (1, 0), (1, MAX_BODY + 1)] {
            assert!(ReceiveBudget::new(requests, bytes).is_err());
        }
        let now = Instant::now();
        let mut budget = ReceiveBudget::new(1, 2).unwrap();
        assert_eq!(budget.begin(now).unwrap(), now + REQUEST_TIMEOUT);
        assert_eq!(
            budget.begin(now),
            Err(BridgeError("REQUEST_LIMIT_EXCEEDED"))
        );
        let mut budget = ReceiveBudget::new(256, 2).unwrap();
        budget.begin(now).unwrap();
        assert_eq!(
            budget.begin(now + OPERATION_TIMEOUT),
            Err(BridgeError("HTTP_TIMEOUT"))
        );
        budget.failed = true;
        assert_eq!(budget.begin(now), Err(BridgeError("HTTPS_SOURCE_CLOSED")));
    }

    #[test]
    fn streaming_limits_charge_probe_and_do_not_read_beyond_total_budget() {
        let now = Instant::now();
        let mut budget = ReceiveBudget::new(3, 2).unwrap();
        let deadline = budget.begin(now).unwrap();
        assert_eq!(
            budget
                .receive(&mut Cursor::new(b"{}"), deadline, || now)
                .unwrap(),
            b"{}"
        );
        assert_eq!(budget.remaining, TOTAL_BYTES - 2);
        let mut oversized = Cursor::new(b"SECRET");
        assert_eq!(
            budget.receive(&mut oversized, deadline, || now),
            Err(BridgeError("RESPONSE_TOO_LARGE"))
        );
        assert_eq!(oversized.position(), 3);
        assert_eq!(budget.remaining, TOTAL_BYTES - 5);
        budget.remaining = 2;
        let mut input = Cursor::new(b"{}extra");
        assert_eq!(
            budget.receive(&mut input, deadline, || now),
            Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"))
        );
        assert_eq!(input.position(), 2);
        assert_eq!(
            budget.begin(now),
            Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"))
        );
    }

    #[test]
    fn empty_bodies_read_errors_and_expired_reads_are_rejected() {
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("SYNTHETIC_SECRET"))
            }
        }
        let now = Instant::now();
        let mut budget = ReceiveBudget::new(1, 2).unwrap();
        let deadline = budget.begin(now).unwrap();
        assert_eq!(
            budget.receive(&mut Cursor::new(b""), deadline, || now),
            Err(BridgeError("INVALID_HTTP_BODY"))
        );
        let error = budget.receive(&mut Broken, deadline, || now).unwrap_err();
        assert_eq!(error, BridgeError("HTTPS_BODY_READ_FAILED"));
        assert!(!format!("{error:?} {error}").contains("SECRET"));
        let mut calls = 0;
        assert_eq!(
            budget.receive(&mut Cursor::new(b"{}"), deadline, || {
                calls += 1;
                if calls == 1 {
                    now
                } else {
                    deadline
                }
            }),
            Err(BridgeError("HTTP_TIMEOUT"))
        );
        assert_eq!(budget.remaining, TOTAL_BYTES - 2);
    }
}
