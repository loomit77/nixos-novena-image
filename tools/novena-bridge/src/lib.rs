//! Novena Bridge: geprüfte Identitäten für Git-Objekte.
//!
//! Diese Bibliothek veröffentlicht nichts. HTTPS ist nur explizit als Quelle aktivierbar.

use sha1::{Digest, Sha1};

pub mod attestation;
pub mod https;
pub mod logs;
pub mod object_evidence;
pub mod objects;
pub mod offline_package;
pub mod offline_readback;
pub mod offline_run;
pub mod planning;
pub mod protocol;
pub mod raw_commit_files;
pub mod sources;
pub mod status;

/// Stable error codes; response bodies and credentials are never included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeError(pub &'static str);
impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for BridgeError {}
pub type BridgeResult<T> = Result<T, BridgeError>;

/// Fehler des strikten JSON-Eingangs (Byteposition, keine Schema-Prüfung).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeJsonError {
    pub offset: usize,
    pub reason: &'static str,
}

impl std::fmt::Display for ExchangeJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at byte {}", self.reason, self.offset)
    }
}

impl std::error::Error for ExchangeJsonError {}

/// Liest striktes Exchange-v1-JSON, ohne vollständige Schema-Validierung.
///
/// Doppelte dekodierte Schlüssel werden während des Einlesens abgewiesen.
/// Zahlen müssen Ganzzahlen in i64::MIN..=u64::MAX sein. Maximal 128
/// verschachtelte Container sind erlaubt. Unicode-Normalisierung erfolgt nicht.
pub fn parse_exchange_v1(input: &[u8]) -> Result<serde_json::Value, ExchangeJsonError> {
    if let Err(error) = std::str::from_utf8(input) {
        return Err(ExchangeJsonError {
            offset: error.valid_up_to(),
            reason: "invalid UTF-8",
        });
    }
    let mut parser = ExchangeParser { input, pos: 0 };
    let value = parser.value(0)?;
    parser.whitespace();
    if parser.pos != input.len() {
        return parser.fail("trailing characters");
    }
    Ok(value)
}

/// Struktureller Fehler des konkreten Exchange-Schemas v1 (JSON-Pointer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeSchemaError {
    pub path: String,
    pub reason: &'static str,
}

impl std::fmt::Display for ExchangeSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exchange-v1 schema: {} at {}", self.reason, self.path)
    }
}
impl std::error::Error for ExchangeSchemaError {}

type SchemaResult = Result<(), ExchangeSchemaError>;

fn schema_error(path: &str, reason: &'static str) -> ExchangeSchemaError {
    ExchangeSchemaError {
        path: path.into(),
        reason,
    }
}

fn pointer(parent: &str, key: &str) -> String {
    format!("{}/{}", parent, key.replace('~', "~0").replace('/', "~1"))
}

fn fields<'a>(
    v: &'a serde_json::Value,
    path: &str,
    keys: &[&str],
) -> Result<&'a serde_json::Map<String, serde_json::Value>, ExchangeSchemaError> {
    let obj = v
        .as_object()
        .ok_or_else(|| schema_error(path, "expected object"))?;
    for key in keys {
        if !obj.contains_key(*key) {
            return Err(schema_error(&pointer(path, key), "required field missing"));
        }
    }
    for key in obj.keys() {
        if !keys.contains(&key.as_str()) {
            return Err(schema_error(&pointer(path, key), "unknown field"));
        }
    }
    Ok(obj)
}

fn string(v: &serde_json::Value, path: &str, nonempty: bool) -> SchemaResult {
    match v.as_str() {
        Some(s) if !nonempty || !s.is_empty() => Ok(()),
        _ => Err(schema_error(
            path,
            "expected string with required minimum length",
        )),
    }
}

fn choice(v: &serde_json::Value, path: &str, choices: &[&str]) -> SchemaResult {
    if v.as_str().is_some_and(|s| choices.contains(&s)) {
        Ok(())
    } else {
        Err(schema_error(path, "enum or const mismatch"))
    }
}

fn oid(v: &serde_json::Value, path: &str) -> SchemaResult {
    if v.as_str().is_some_and(|s| {
        // Schema pattern uses `$`, which also matches before a terminal LF.
        // Protocol/policy object IDs use the stricter objects::Oid parser.
        let s = s.strip_suffix('\n').unwrap_or(s);
        s.len() == 40
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }) {
        Ok(())
    } else {
        Err(schema_error(
            path,
            "expected 40 lowercase hexadecimal characters",
        ))
    }
}

fn strings(v: &serde_json::Value, path: &str, minimum: usize) -> SchemaResult {
    let a = v
        .as_array()
        .ok_or_else(|| schema_error(path, "expected array"))?;
    if a.len() < minimum {
        return Err(schema_error(path, "too few array items"));
    }
    let mut seen = std::collections::HashSet::new();
    for (i, item) in a.iter().enumerate() {
        let path = pointer(path, &i.to_string());
        string(item, &path, false)?;
        if !seen.insert(item.as_str().expect("string checked")) {
            return Err(schema_error(&path, "duplicate array item"));
        }
    }
    Ok(())
}

// RFC 3339 full-date / full-time, including fractional seconds, lowercase
// t/z and numeric offsets. No UTC-only restriction is implied by field names.
fn date_time(v: &serde_json::Value, path: &str) -> SchemaResult {
    if v.as_str().is_some_and(valid_date_time) {
        Ok(())
    } else {
        Err(schema_error(path, "expected RFC 3339 date-time"))
    }
}

fn valid_date_time(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 20 || !b.is_ascii() {
        return false;
    }
    let number = |start: usize, end: usize| -> Option<i32> {
        let part = b.get(start..end)?;
        if !part.iter().all(u8::is_ascii_digit) {
            return None;
        }
        Some(part.iter().fold(0, |n, d| n * 10 + i32::from(d - b'0')))
    };
    if b[4] != b'-'
        || b[7] != b'-'
        || !matches!(b[10], b'T' | b't')
        || b[13] != b':'
        || b[16] != b':'
    {
        return false;
    }
    let (Some(y), Some(m), Some(d), Some(h), Some(min), Some(sec)) = (
        number(0, 4),
        number(5, 7),
        number(8, 10),
        number(11, 13),
        number(14, 16),
        number(17, 19),
    ) else {
        return false;
    };
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = match m {
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return false,
    };
    if d < 1 || d > days || h > 23 || min > 59 || sec > 60 {
        return false;
    }
    let mut end = 19;
    if b.get(end) == Some(&b'.') {
        end += 1;
        let start = end;
        while b.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if start == end {
            return false;
        }
    }
    let offset = match b.get(end..) {
        Some([b'Z' | b'z']) => 0,
        Some([sign @ (b'+' | b'-'), _, _, b':', _, _]) => {
            let (Some(oh), Some(om)) = (number(end + 1, end + 3), number(end + 4, end + 6)) else {
                return false;
            };
            if oh > 23 || om > 59 {
                return false;
            }
            (oh * 60 + om) * if *sign == b'+' { 1 } else { -1 }
        }
        _ => return false,
    };
    // A leap second is at the end of a UTC June/December day. Translate
    // the local minute and date boundary; do not hard-code a historical table.
    if sec == 60 {
        let utc = h * 60 + min - offset;
        let (um, ud) = if utc < 0 {
            if d > 1 {
                (m, d - 1)
            } else {
                (
                    if m == 1 { 12 } else { m - 1 },
                    if matches!(m, 5 | 7 | 10 | 12) {
                        30
                    } else if m == 3 {
                        if leap {
                            29
                        } else {
                            28
                        }
                    } else {
                        31
                    },
                )
            }
        } else if utc >= 1440 {
            if d == days {
                (if m == 12 { 1 } else { m + 1 }, 1)
            } else {
                (m, d + 1)
            }
        } else {
            (m, d)
        };
        return utc.rem_euclid(1440) == 1439 && ((um == 6 && ud == 30) || (um == 12 && ud == 31));
    }
    true
}

/// Validiert ausschließlich docs/bridge-lite/exchange-schema-v1.json.
///
/// Der Aufrufer liest Bytes zuerst mit `parse_exchange_v1` ein. Diese Prüfung
/// verändert keine Werte und bestätigt weder Evidenz noch LAST_VERIFIED oder
/// Publikationsberechtigung. Keine allgemeine JSON-Schema-Engine.
pub fn validate_exchange_v1(v: &serde_json::Value) -> SchemaResult {
    let top = fields(
        v,
        "",
        &[
            "schema_version",
            "project",
            "generated_at_utc",
            "last_verified",
            "current",
            "latest_block",
            "build",
            "evidence",
            "publication",
            "review_required",
        ],
    )?;
    if top["schema_version"] != serde_json::json!(1) {
        return Err(schema_error("/schema_version", "const mismatch"));
    }
    choice(&top["project"], "/project", &["novena"])?;
    date_time(&top["generated_at_utc"], "/generated_at_utc")?;
    let result = ["PASS", "FAIL", "UNKNOWN"];
    if !top["last_verified"].is_null() {
        let p = "/last_verified";
        let o = fields(
            &top["last_verified"],
            p,
            &[
                "block_id",
                "git_head",
                "qualified_at_utc",
                "qualification",
                "evidence_files",
            ],
        )?;
        string(&o["block_id"], &pointer(p, "block_id"), true)?;
        oid(&o["git_head"], &pointer(p, "git_head"))?;
        date_time(&o["qualified_at_utc"], &pointer(p, "qualified_at_utc"))?;
        choice(&o["qualification"], &pointer(p, "qualification"), &["PASS"])?;
        strings(&o["evidence_files"], &pointer(p, "evidence_files"), 1)?;
    }
    let o = fields(&top["current"], "/current", &["state", "observed_at_utc"])?;
    choice(
        &o["state"],
        "/current/state",
        &["LAST_VERIFIED", "CURRENT_UNKNOWN", "IN_PROGRESS"],
    )?;
    if !o["observed_at_utc"].is_null() {
        date_time(&o["observed_at_utc"], "/current/observed_at_utc")?;
    }
    if !top["latest_block"].is_null() {
        let p = "/latest_block";
        let o = fields(
            &top["latest_block"],
            p,
            &[
                "block_id",
                "runner",
                "upload",
                "technical_result",
                "professional_acceptance",
            ],
        )?;
        string(&o["block_id"], &pointer(p, "block_id"), true)?;
        for k in [
            "runner",
            "upload",
            "technical_result",
            "professional_acceptance",
        ] {
            choice(&o[k], &pointer(p, k), &result)?;
        }
    }
    let o = fields(&top["build"], "/build", &["state", "source"])?;
    choice(
        &o["state"],
        "/build/state",
        &["UNKNOWN", "NOT_STARTED", "RUNNING", "PASS", "FAIL"],
    )?;
    if !o["source"].is_null() {
        string(&o["source"], "/build/source", false)?;
    }
    let a = top["evidence"]
        .as_array()
        .ok_or_else(|| schema_error("/evidence", "expected array"))?;
    for (i, item) in a.iter().enumerate() {
        let p = pointer("/evidence", &i.to_string());
        let o = fields(
            item,
            &p,
            &[
                "log_file",
                "sha256_file",
                "local_integrity",
                "archive_upload",
            ],
        )?;
        for k in ["log_file", "sha256_file"] {
            string(&o[k], &pointer(&p, k), true)?;
        }
        for k in ["local_integrity", "archive_upload"] {
            choice(&o[k], &pointer(&p, k), &result)?;
        }
    }
    let o = fields(
        &top["publication"],
        "/publication",
        &["state", "archive", "commit"],
    )?;
    choice(
        &o["state"],
        "/publication/state",
        &["NOT_PUBLISHED", "UNKNOWN", "CONFIRMED"],
    )?;
    choice(
        &o["archive"],
        "/publication/archive",
        &["loomit77/project-block-logs"],
    )?;
    if !o["commit"].is_null() {
        oid(&o["commit"], "/publication/commit")?;
    }
    strings(&top["review_required"], "/review_required", 0)
}

struct ExchangeParser<'a> {
    input: &'a [u8],
    pos: usize,
}

#[cfg(test)]
mod exchange_tests {
    use super::{canonical_json, git_blob_oid, parse_exchange_v1};
    use serde_json::json;

    #[test]
    fn existing_exchange_documents_are_accepted() {
        for source in [
            include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json").as_slice(),
            include_bytes!("../../../docs/bridge-lite/EXCHANGE-CANDIDATE-149.json").as_slice(),
            include_bytes!("../../../docs/bridge-lite/current-state.json").as_slice(),
        ] {
            let parsed = parse_exchange_v1(source).unwrap();
            assert_eq!(
                parsed,
                serde_json::from_slice::<serde_json::Value>(source).unwrap()
            );
            let mut bytes = serde_json::to_vec(&parsed).unwrap();
            bytes.push(b'\n');
            assert_eq!(bytes, canonical_json(source).unwrap());
            assert_eq!(
                git_blob_oid(&bytes),
                git_blob_oid(&canonical_json(source).unwrap())
            );
        }
    }

    #[test]
    fn duplicate_keys_are_rejected_during_object_parsing() {
        for source in [
            r#"{"a":1,"a":2}"#,
            r#"{"outer":{"a":1,"a":2}}"#,
            r#"[{"a":null,"a":false}]"#,
            r#"{"a":1,"\u0061":2}"#,
            r#"{"\u0061":1,"a":2}"#,
            r#"{"ä":1,"\u00e4":2}"#,
            r#"{"😀":1,"\ud83d\ude00":2}"#,
            r#"{"":1,"":2}"#,
            r#"{"a":1,"a":INVALID}"#,
        ] {
            let error = parse_exchange_v1(source.as_bytes()).unwrap_err();
            assert_eq!(error.reason, "duplicate object key", "{source}");
        }
    }

    #[test]
    fn valid_values_and_independent_object_keys_are_accepted() {
        let source = br#" {"a":[{"x":1},{"x":2}],"s":"\"\\\n\u0061","v":[true,false,null,-0]} "#;
        assert_eq!(
            parse_exchange_v1(source).unwrap(),
            json!({"a":[{"x":1},{"x":2}],"s":"\"\\\na","v":[true,false,null,0]})
        );
        // JSON decoding equivalence does not imply Unicode normalization.
        assert!(parse_exchange_v1(r#"{"é":1,"e\u0301":2}"#.as_bytes()).is_ok());
    }

    #[test]
    fn integer_boundaries_are_exact() {
        for source in [
            "-9223372036854775808",
            "9223372036854775807",
            "18446744073709551615",
        ] {
            let value = parse_exchange_v1(source.as_bytes()).unwrap();
            assert_eq!(value.to_string(), source);
        }
        for source in [
            "-9223372036854775809",
            "18446744073709551616",
            "999999999999999999999999999999999999999999",
        ] {
            assert_eq!(
                parse_exchange_v1(source.as_bytes()).unwrap_err().reason,
                "integer overflow"
            );
        }
    }

    #[test]
    fn non_integer_number_syntax_is_rejected_everywhere() {
        for number in [
            "0.0",
            "1.5",
            "-0.0",
            "1e0",
            "1E+2",
            "1e-2",
            "1e999",
            "NaN",
            "Infinity",
            "-Infinity",
        ] {
            for source in [
                number.to_owned(),
                format!("{{\"n\":{number}}}"),
                format!("[{number}]"),
            ] {
                assert!(parse_exchange_v1(source.as_bytes()).is_err(), "{source}");
            }
        }
        assert!(parse_exchange_v1(br#"["NaN","Infinity","1.0","1e2"]"#).is_ok());
    }

    #[test]
    fn malformed_json_and_trailing_input_are_rejected() {
        for source in [
            "",
            " ",
            "{",
            "[",
            "{invalid}",
            "[1,]",
            "{\"a\":1,}",
            "{\"a\" 1}",
            "{\"a\":}",
            "[1 2]",
            "01",
            "-01",
            "-",
            "+1",
            "true false",
            "{}x",
            "{}{}",
            "null\0",
            "True",
            "/*x*/null",
            "\"\\x\"",
            "\"\\uD800\"",
            "\"\\uDC00\"",
            "\"unclosed",
            "\"line\nfeed\"",
            "\u{feff}{}",
            "null\u{a0}",
        ] {
            assert!(parse_exchange_v1(source.as_bytes()).is_err(), "{source:?}");
        }
        for source in [
            b"\xff".as_slice(),
            b"\"\xff\"",
            b"{\"\xff\":0}",
            b"null\xff",
        ] {
            assert_eq!(
                parse_exchange_v1(source).unwrap_err().reason,
                "invalid UTF-8"
            );
        }
        assert!(parse_exchange_v1(b" \t\r\n{} \t\r\n").is_ok());
    }

    #[test]
    fn nesting_is_bounded() {
        let allowed = format!("{}0{}", "[".repeat(128), "]".repeat(128));
        assert!(parse_exchange_v1(allowed.as_bytes()).is_ok());
        let excessive = format!("[{allowed}]");
        assert_eq!(
            parse_exchange_v1(excessive.as_bytes()).unwrap_err().reason,
            "nesting limit exceeded"
        );
    }

    #[test]
    fn canonical_json_retains_its_general_json_behavior() {
        assert!(canonical_json(br#"{"number":1.5}"#).is_ok());
        assert_eq!(canonical_json(br#"{"a":1,"a":2}"#).unwrap(), b"{\"a\":2}\n");
    }
}

impl ExchangeParser<'_> {
    fn fail<T>(&self, reason: &'static str) -> Result<T, ExchangeJsonError> {
        Err(ExchangeJsonError {
            offset: self.pos,
            reason,
        })
    }

    fn whitespace(&mut self) {
        while matches!(self.input.get(self.pos), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }

    fn take(&mut self, byte: u8) -> bool {
        if self.input.get(self.pos) == Some(&byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    // Keep the independently qualified A1 token-scanner control flow unchanged.
    #[allow(clippy::collapsible_match)]
    fn string(&mut self) -> Result<String, ExchangeJsonError> {
        let start = self.pos;
        if !self.take(b'"') {
            return self.fail("expected string");
        }
        while let Some(&byte) = self.input.get(self.pos) {
            self.pos += 1;
            match byte {
                b'"' => {
                    return serde_json::from_slice(&self.input[start..self.pos]).map_err(|_| {
                        ExchangeJsonError {
                            offset: start,
                            reason: "invalid JSON string",
                        }
                    });
                }
                // Skip the escaped byte; serde_json checks the escape and
                // surrogate pairs when the complete token is decoded.
                b'\\' => {
                    if self.pos < self.input.len() {
                        self.pos += 1;
                    }
                }
                _ => {}
            }
        }
        self.fail("unterminated string")
    }

    fn value(&mut self, depth: usize) -> Result<serde_json::Value, ExchangeJsonError> {
        use serde_json::Value;
        self.whitespace();
        match self.input.get(self.pos).copied() {
            Some(b'{' | b'[') if depth >= 128 => self.fail("nesting limit exceeded"),
            Some(b'{') => {
                self.pos += 1;
                let mut object = serde_json::Map::new();
                self.whitespace();
                if self.take(b'}') {
                    return Ok(Value::Object(object));
                }
                loop {
                    self.whitespace();
                    let key = self.string()?;
                    if object.contains_key(&key) {
                        return self.fail("duplicate object key");
                    }
                    self.whitespace();
                    if !self.take(b':') {
                        return self.fail("expected colon");
                    }
                    let value = self.value(depth + 1)?;
                    object.insert(key, value);
                    self.whitespace();
                    if self.take(b'}') {
                        break;
                    }
                    if !self.take(b',') {
                        return self.fail("expected comma or closing brace");
                    }
                }
                Ok(Value::Object(object))
            }
            Some(b'[') => {
                self.pos += 1;
                let mut array = Vec::new();
                self.whitespace();
                if self.take(b']') {
                    return Ok(Value::Array(array));
                }
                loop {
                    array.push(self.value(depth + 1)?);
                    self.whitespace();
                    if self.take(b']') {
                        break;
                    }
                    if !self.take(b',') {
                        return self.fail("expected comma or closing bracket");
                    }
                }
                Ok(Value::Array(array))
            }
            Some(b'"') => self.string().map(Value::String),
            Some(b'-' | b'0'..=b'9') => {
                let start = self.pos;
                self.take(b'-');
                if !self.take(b'0') {
                    if !matches!(self.input.get(self.pos), Some(b'1'..=b'9')) {
                        return self.fail("expected integer");
                    }
                    while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                        self.pos += 1;
                    }
                }
                if matches!(self.input.get(self.pos), Some(b'.' | b'e' | b'E')) {
                    return self.fail("decimal and exponent notation forbidden");
                }
                let token = std::str::from_utf8(&self.input[start..self.pos])
                    .expect("UTF-8 checked at entry");
                let number = if token.starts_with('-') {
                    token.parse::<i64>().map(serde_json::Number::from)
                } else {
                    token.parse::<u64>().map(serde_json::Number::from)
                };
                match number {
                    Ok(number) => Ok(Value::Number(number)),
                    Err(_) => self.fail("integer overflow"),
                }
            }
            Some(b'n' | b't' | b'f') => {
                let (token, value): (&[u8], Value) = match self.input[self.pos] {
                    b'n' => (b"null", Value::Null),
                    b't' => (b"true", Value::Bool(true)),
                    _ => (b"false", Value::Bool(false)),
                };
                if self.input[self.pos..].starts_with(token) {
                    self.pos += token.len();
                    Ok(value)
                } else {
                    self.fail("invalid literal")
                }
            }
            _ => self.fail("expected JSON value"),
        }
    }
}

/// Berechnet die kanonische Git-SHA-1-Identität eines Blobs.
///
/// Git hasht nicht nur den Dateiinhalt, sondern:
///
/// `blob <byteanzahl>\0<dateiinhalt>`
///
/// Die Byteanzahl ist die tatsächliche Länge, nicht die
/// Zahl der UTF-8-Zeichen.
pub fn git_blob_oid(content: &[u8]) -> String {
    let header = format!("blob {}\0", content.len());

    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);

    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::git_blob_oid;

    #[test]
    fn empty_blob_matches_git() {
        assert_eq!(
            git_blob_oid(b""),
            "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"
        );
    }

    #[test]
    fn text_blob_matches_git() {
        assert_eq!(
            git_blob_oid(b"hello\n"),
            "ce013625030ba8dba906f756967f9e9ca394464a"
        );
    }

    #[test]
    fn trailing_newline_changes_identity() {
        assert_ne!(git_blob_oid(b"hello"), git_blob_oid(b"hello\n"));
    }

    #[test]
    fn arbitrary_binary_bytes_supported() {
        let content = [0x00, 0xff, 0x80, 0x0a];
        let id = git_blob_oid(&content);

        assert_eq!(id.len(), 40);
        assert!(id.bytes().all(|b| b.is_ascii_hexdigit()));
    }
}

/// Serialisiert ein JSON-Dokument deterministisch.
///
/// Dies ist KEINE Schema- oder Sicherheitsvalidierung.
/// Insbesondere prüft diese Funktion noch keine doppelten
/// JSON-Schlüssel und keine Bridge-Veröffentlichungsregeln.
///
/// Die serde_json-Standardzuordnung sortiert Objekt-Schlüssel.
/// Die Ausgabe wird mit genau einem Newline abgeschlossen.
pub fn canonical_json(input: &[u8]) -> Result<Vec<u8>, serde_json::Error> {
    let document: serde_json::Value = serde_json::from_slice(input)?;

    let mut output = serde_json::to_vec(&document)?;
    output.push(b'\n');

    Ok(output)
}

#[cfg(test)]
mod json_tests {
    use super::{canonical_json, git_blob_oid};

    #[test]
    fn keys_are_sorted_and_whitespace_removed() {
        let source = br#"{ "z": 2, "a": 1 }"#;
        let expected = b"{\"a\":1,\"z\":2}\n";

        assert_eq!(canonical_json(source).unwrap(), expected);
    }

    #[test]
    fn unicode_is_kept_as_utf8() {
        let source = "{\"text\":\"Grüße\"}";
        let expected = "{\"text\":\"Grüße\"}\n";

        assert_eq!(
            canonical_json(source.as_bytes()).unwrap(),
            expected.as_bytes()
        );
    }

    #[test]
    fn invalid_json_is_rejected() {
        assert!(canonical_json(b"{invalid").is_err());
    }

    #[test]
    fn canonical_bytes_have_a_git_blob_identity() {
        let output = canonical_json(br#"{"z":2,"a":1}"#).unwrap();

        assert_eq!(git_blob_oid(&output), git_blob_oid(b"{\"a\":1,\"z\":2}\n"));
    }
}

#[cfg(test)]
mod schema_tests {
    use super::*;
    use serde_json::{json, Value};

    fn full() -> Value {
        let mut v = parse_exchange_v1(include_bytes!(
            "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
        ))
        .unwrap();
        v["last_verified"] = json!({
            "block_id":"synthetic", "git_head":"abcdef0123456789abcdef0123456789abcdef01",
            "qualified_at_utc":"2024-02-29T23:59:59Z", "qualification":"PASS",
            "evidence_files":["synthetic.txt"]
        });
        v["current"]["observed_at_utc"] = json!("2026-10-25T12:00:00+02:30");
        v["build"]["source"] = json!("synthetic");
        v["publication"]["commit"] = v["last_verified"]["git_head"].clone();
        v
    }

    fn changed(base: &Value, path: &str, value: Value) -> Value {
        let mut v = base.clone();
        *v.pointer_mut(path).unwrap() = value;
        v
    }

    fn reject(base: &Value, path: &str, value: Value) {
        let error = validate_exchange_v1(&changed(base, path, value.clone()))
            .expect_err(&format!("must reject {path}: {value}"));
        assert!(error.path.starts_with(path), "{path}: {value}: {error}");
    }

    #[test]
    fn pilot_and_existing_documents_preserve_canonical_bytes() {
        for source in [
            include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json").as_slice(),
            include_bytes!("../../../docs/bridge-lite/EXCHANGE-CANDIDATE-149.json").as_slice(),
        ] {
            let v = parse_exchange_v1(source).unwrap();
            let before = v.clone();
            validate_exchange_v1(&v).unwrap();
            assert_eq!(before, v);
            let mut output = serde_json::to_vec(&v).unwrap();
            output.push(b'\n');
            assert_eq!(output, canonical_json(source).unwrap());
        }
        validate_exchange_v1(&full()).unwrap();
    }

    // Inventory the actual schema in tests. Each reachable definition is
    // exercised using a non-null, nonempty representative, including every
    // required field, const, enum, type and additionalProperties constraint.
    // This is test traversal, not a runtime schema interpreter.
    fn inventory(root: &Value, schema: &Value, path: &str, base: &Value, count: &mut usize) {
        if let Some(reference) = schema["$ref"].as_str() {
            inventory(
                root,
                root.pointer(&reference[1..]).unwrap(),
                path,
                base,
                count,
            );
            return;
        }
        if let Some(alternatives) = schema["oneOf"].as_array() {
            assert!(validate_exchange_v1(&changed(base, path, Value::Null)).is_ok());
            inventory(
                root,
                alternatives
                    .iter()
                    .find(|s| s.get("$ref").is_some())
                    .unwrap(),
                path,
                base,
                count,
            );
            return;
        }
        *count += 1;
        let value = base.pointer(path).unwrap();
        if schema.get("const").is_some() {
            assert_eq!(value, &schema["const"]);
            for wrong in [
                Value::Null,
                json!(true),
                json!(2),
                json!("wrong"),
                json!([]),
                json!({}),
            ] {
                reject(base, path, wrong);
            }
        }
        if let Some(options) = schema["enum"].as_array() {
            for option in options {
                validate_exchange_v1(&changed(base, path, option.clone())).unwrap();
            }
            for wrong in [
                Value::Null,
                json!(true),
                json!(1),
                json!(""),
                json!("pass"),
                json!([]),
                json!({}),
            ] {
                reject(base, path, wrong);
            }
        }
        if let Some(t) = schema.get("type") {
            let types: Vec<&str> = if let Some(a) = t.as_array() {
                a.iter().map(|v| v.as_str().unwrap()).collect()
            } else {
                vec![t.as_str().unwrap()]
            };
            for (name, candidate) in [
                ("null", Value::Null),
                ("boolean", json!(true)),
                ("integer", json!(1)),
                ("string", json!("x")),
                ("array", json!([])),
                ("object", json!({})),
            ] {
                if !types.contains(&name)
                    && !(name == "null" && ["/last_verified", "/latest_block"].contains(&path))
                {
                    reject(base, path, candidate);
                }
            }
            if types.contains(&"null") {
                validate_exchange_v1(&changed(base, path, Value::Null)).unwrap();
            }
        }
        if schema["type"] == "object" {
            let required = schema["required"].as_array().unwrap();
            assert_eq!(
                required.len(),
                schema["properties"].as_object().unwrap().len()
            );
            assert_eq!(schema["additionalProperties"], json!(false));
            let mut extra = value.clone();
            extra["unknown/~field"] = json!(0);
            reject(base, path, extra);
            for key in required {
                let key = key.as_str().unwrap();
                let mut missing = value.clone();
                missing.as_object_mut().unwrap().remove(key);
                reject(base, path, missing);
                inventory(
                    root,
                    &schema["properties"][key],
                    &pointer(path, key),
                    base,
                    count,
                );
            }
        }
        if schema["type"] == "array" {
            if schema["minItems"] == 1 {
                reject(base, path, json!([]));
            } else {
                validate_exchange_v1(&changed(base, path, json!([]))).unwrap();
            }
            inventory(root, &schema["items"], &pointer(path, "0"), base, count);
            let item = value[0].clone();
            let duplicate = json!([item, item]);
            if schema["uniqueItems"] == true {
                reject(base, path, duplicate);
            } else {
                validate_exchange_v1(&changed(base, path, duplicate)).unwrap();
            }
        }
        if schema["minLength"] == 1 {
            reject(base, path, json!(""));
            validate_exchange_v1(&changed(base, path, json!("😀"))).unwrap();
        }
        if schema.get("pattern").is_some() {
            assert_eq!(schema["pattern"], json!("^[0-9a-f]{40}$"));
            for bad in [
                "a".repeat(39),
                "a".repeat(41),
                "A".repeat(40),
                "g".repeat(40),
                "é".repeat(40),
            ] {
                reject(base, path, json!(bad));
            }
            for valid in [
                "0".repeat(40),
                "f".repeat(40),
                format!("{}\n", "a".repeat(40)),
            ] {
                validate_exchange_v1(&changed(base, path, json!(valid))).unwrap();
            }
        }
        if schema["format"] == "date-time" {
            reject(base, path, json!("2026-10-25"));
        }
    }

    #[test]
    fn all_schema_fields_and_constraints_are_exercised() {
        let root: Value = serde_json::from_slice(include_bytes!(
            "../../../docs/bridge-lite/exchange-schema-v1.json"
        ))
        .unwrap();
        let mut count = 0;
        inventory(&root, &root, "", &full(), &mut count);
        assert_eq!(count, 35, "schema inventory changed: review coverage");
    }

    #[test]
    fn date_times_accept_offsets_fractions_and_calendar_boundaries() {
        let v = full();
        for path in [
            "/generated_at_utc",
            "/current/observed_at_utc",
            "/last_verified/qualified_at_utc",
        ] {
            for valid in [
                "2024-02-29T00:00:00Z",
                "2000-02-29t23:59:59z",
                "2026-10-25T11:59:37.780476+00:00",
                "2026-01-01T00:00:00-00:00",
                "2026-01-01T00:00:00+23:59",
                "2026-01-01T00:00:00-23:59",
                "2026-01-01T00:00:00.1Z",
                "2016-12-31T23:59:60Z",
                "2017-01-01T00:59:60+01:00",
                "2015-07-01T01:29:60+01:30",
                "2015-06-30T22:59:60-01:00",
            ] {
                validate_exchange_v1(&changed(&v, path, json!(valid)))
                    .unwrap_or_else(|e| panic!("{valid}: {e}"));
            }
            for bad in [
                "",
                "2026-02-29T00:00:00Z",
                "1900-02-29T00:00:00Z",
                "2026-04-31T00:00:00Z",
                "2026-00-01T00:00:00Z",
                "2026-13-01T00:00:00Z",
                "2026-01-00T00:00:00Z",
                "2026-01-32T00:00:00Z",
                "2026-01-01T24:00:00Z",
                "2026-01-01T00:60:00Z",
                "2026-01-01T00:00:61Z",
                "2026-01-01T00:00:60Z",
                "2026-01-01T00:00:00",
                "2026-01-01 00:00:00Z",
                "2026-1-01T00:00:00Z",
                "2026-01-01T00:00:00.Z",
                "2026-01-01T00:00:00+24:00",
                "2026-01-01T00:00:00+00:60",
                "2026-01-01T00:00:00+0000",
                "2026-01-01T00:00:00+0a:00",
                "2026-01-01T00:00:00Zx",
                "2026-01-01T00:00:00Z\n",
                "２０２６-01-01T00:00:00Z",
            ] {
                reject(&v, path, json!(bad));
            }
        }
    }

    #[test]
    fn schema_success_does_not_apply_publication_or_checkpoint_policy() {
        let mut v = full();
        v["publication"]["state"] = json!("CONFIRMED");
        v["current"]["state"] = json!("LAST_VERIFIED");
        v["last_verified"]["evidence_files"] = json!([""]);
        v["build"]["source"] = json!("");
        v["review_required"] = json!(["", "é", "e\u{301}"]);
        validate_exchange_v1(&v).unwrap();
        v["publication"]["commit"] = Value::Null;
        v["last_verified"] = Value::Null;
        v["latest_block"] = Value::Null;
        validate_exchange_v1(&v).unwrap();
    }

    #[test]
    fn unique_items_use_decoded_string_equality() {
        let v = full();
        for path in ["/review_required", "/last_verified/evidence_files"] {
            let duplicate = parse_exchange_v1(br#"["a","\u0061"]"#).unwrap();
            reject(&v, path, duplicate);
        }
    }
}

#[cfg(test)]
mod schema_boundary_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn old_snapshot_is_json_but_not_an_exchange_status_document() {
        let v = parse_exchange_v1(include_bytes!(
            "../../../docs/bridge-lite/current-state.json"
        ))
        .unwrap();
        assert_eq!(validate_exchange_v1(&v).unwrap_err().path, "/project");
    }

    #[test]
    fn no_unstated_string_or_array_maximum_is_imposed() {
        let mut v = parse_exchange_v1(include_bytes!(
            "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
        ))
        .unwrap();
        v["latest_block"]["block_id"] = json!("😀".repeat(4096));
        v["review_required"] = json!((0..1024).map(|i| i.to_string()).collect::<Vec<_>>());
        validate_exchange_v1(&v).unwrap();
    }
}

pub mod local_sources;
