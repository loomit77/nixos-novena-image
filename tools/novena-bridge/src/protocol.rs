//! REST-/Offlineverträge; optionaler expliziter HTTPS-Transport in crate::https.
use crate::objects::*;
use crate::status::{PreparedStatus, ARCHIVE, MAX_PAYLOAD, REF, TARGET};
use crate::{parse_exchange_v1, BridgeError, BridgeResult};
use serde_json::{json, Value};

pub const PREFIX: &str = "repos/loomit77/project-block-logs";

pub fn allowed_endpoint(path: &str) -> BridgeResult<()> {
    if path == PREFIX || path == format!("{PREFIX}/git/ref/heads/main") {
        return Ok(());
    }
    for kind in ["commits", "trees", "blobs"] {
        if let Some(id) = path.strip_prefix(&format!("{PREFIX}/git/{kind}/")) {
            Oid::parse(id)?;
            return Ok(());
        }
    }
    Err(BridgeError("ENDPOINT_NOT_ALLOWLISTED"))
}

/// Explizit übergebene Antwortquelle. Es gibt keinen Netzwerkdefault.
pub trait ResponseSource {
    fn get(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>>;
}
impl<F: FnMut(&str) -> BridgeResult<Vec<u8>>> ResponseSource for F {
    fn get(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>> {
        self(endpoint)
    }
}

/// Synthetic HTTP envelope, never evidence of authenticated acquisition.
/// No Debug/serialization implementation: URLs, headers and bodies must not
/// become diagnostics. Credentials are neither accepted separately nor read.
pub struct OfflineHttpResponse {
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
    pub location: Option<String>,
    pub elapsed: Option<std::time::Duration>,
    pub body: Option<Vec<u8>>,
}

/// Offline-only HTTP-to-ResponseSource boundary. No socket, process, credential,
/// retry, redirect following or CLI integration. Construction is explicit.
///
/// Recorded elapsed time is caller supplied; this checks a synthetic timeout
/// policy, NOT a real deadline. A future TLS client must enforce deadlines over
/// DNS/connect/TLS/headers/body and bound bytes while receiving them. It must
/// not treat these envelopes as proof of HTTPS, hostname or source authenticity.
pub struct OfflineHttpSource {
    responses: std::collections::VecDeque<OfflineHttpResponse>,
    requests: usize,
    request_limit: usize,
    response_limit: usize,
    timeout: std::time::Duration,
}
impl OfflineHttpSource {
    pub fn new(
        responses: Vec<OfflineHttpResponse>,
        request_limit: usize,
        response_limit: usize,
        timeout: std::time::Duration,
    ) -> BridgeResult<Self> {
        if !(1..=256).contains(&request_limit)
            || !(1..=8 * 1024 * 1024).contains(&response_limit)
            || timeout.is_zero()
            || timeout > std::time::Duration::from_secs(60)
        {
            return Err(BridgeError("INVALID_HTTP_LIMITS"));
        }
        // Inputs already reside in memory. Bound retained transcript size, not
        // network allocation; there is deliberately no network receiver here.
        let mut total = 0usize;
        if responses.len() > request_limit {
            return Err(BridgeError("HTTP_TRANSCRIPT_TOO_LARGE"));
        }
        for response in &responses {
            if response.method.len() > 16
                || response.url.len() > 256
                || response.location.as_ref().is_some_and(|s| s.len() > 4096)
            {
                return Err(BridgeError("HTTP_TRANSCRIPT_TOO_LARGE"));
            }
            let size = response.body.as_ref().map_or(0, Vec::len);
            if size > response_limit {
                return Err(BridgeError("RESPONSE_TOO_LARGE"));
            }
            total += size; // At most 256 * 8 MiB, including on 32-bit hosts.
            if total > 32 * 1024 * 1024 {
                return Err(BridgeError("HTTP_TRANSCRIPT_TOO_LARGE"));
            }
        }
        Ok(Self {
            responses: responses.into(),
            requests: 0,
            request_limit,
            response_limit,
            timeout,
        })
    }
}
impl ResponseSource for OfflineHttpSource {
    fn get(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>> {
        allowed_endpoint(endpoint)?;
        if self.requests >= self.request_limit {
            return Err(BridgeError("REQUEST_LIMIT_EXCEEDED"));
        }
        // Failed and missing responses consume an attempt; no hidden retries.
        self.requests += 1;
        let response = self
            .responses
            .pop_front()
            .ok_or(BridgeError("MISSING_RESPONSE"))?;
        if response.method != "GET" {
            return Err(BridgeError("HTTP_METHOD_NOT_ALLOWED"));
        }
        // Exact construction excludes userinfo, ports, queries, fragments,
        // encoded paths, HTTP downgrade and lookalike/alternative hosts.
        if response.url != format!("https://api.github.com/{endpoint}") {
            return Err(BridgeError("HTTP_URL_NOT_ALLOWED"));
        }
        if response.location.is_some() || response.status.is_some_and(|s| (300..400).contains(&s)) {
            return Err(BridgeError("HTTP_REDIRECT_REJECTED"));
        }
        // GitHub's contracted object GETs require a complete 200 response;
        // 204, 206 and 304 cannot stand in for complete object bytes.
        if response.status != Some(200) {
            return Err(BridgeError("HTTP_STATUS_REJECTED"));
        }
        let elapsed = response
            .elapsed
            .ok_or(BridgeError("MISSING_HTTP_ELAPSED"))?;
        if elapsed >= self.timeout {
            return Err(BridgeError("HTTP_TIMEOUT"));
        }
        let body = response.body.ok_or(BridgeError("MISSING_RESPONSE"))?;
        if body.is_empty() || body.len() > self.response_limit {
            return Err(BridgeError("INVALID_HTTP_BODY"));
        }
        // JSON/shape and all Git checks remain exclusively in existing readers.
        Ok(body)
    }
}

pub struct RestReader<S> {
    source: S,
    requests: usize,
    limit: usize,
    remaining_response_bytes: usize,
}
impl<S: ResponseSource> RestReader<S> {
    pub fn new(source: S, limit: usize) -> BridgeResult<Self> {
        if !(1..=256).contains(&limit) {
            return Err(BridgeError("INVALID_REQUEST_LIMIT"));
        }
        Ok(Self {
            source,
            requests: 0,
            limit,
            remaining_response_bytes: 32 * 1024 * 1024,
        })
    }
    pub fn requests(&self) -> usize {
        self.requests
    }
    pub(crate) fn source_ref(&self) -> &S {
        &self.source
    }
    pub fn get(&mut self, path: &str) -> BridgeResult<Value> {
        allowed_endpoint(path)?;
        if self.requests >= self.limit {
            return Err(BridgeError("REQUEST_LIMIT_EXCEEDED"));
        }
        if self.remaining_response_bytes == 0 {
            return Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"));
        }
        self.requests += 1;
        let bytes = self.source.get(path)?;
        // Charge every returned body before size/JSON/shape validation. This
        // bounds downstream processing across calls, not source allocation or
        // network reception. RawCommitSource is outside this reader's budget.
        let Some(remaining) = self.remaining_response_bytes.checked_sub(bytes.len()) else {
            self.remaining_response_bytes = 0;
            return Err(BridgeError("RESPONSE_BUDGET_EXCEEDED"));
        };
        self.remaining_response_bytes = remaining;
        if bytes.len() > 8 * 1024 * 1024 {
            return Err(BridgeError("RESPONSE_TOO_LARGE"));
        }
        let v = parse_exchange_v1(&bytes).map_err(|_| BridgeError("INVALID_RESPONSE_JSON"))?;
        if !v.is_object() {
            return Err(BridgeError("INVALID_RESPONSE_SHAPE"));
        }
        Ok(v)
    }
    pub fn repository(&mut self) -> BridgeResult<String> {
        let v = self.get(PREFIX)?;
        if v["full_name"] != ARCHIVE {
            return Err(BridgeError("WRONG_REPOSITORY"));
        }
        let id = text(&v["node_id"])?;
        if id.trim().is_empty() {
            return Err(BridgeError("MISSING_REPOSITORY_ID"));
        }
        Ok(id.to_owned())
    }
    pub fn head(&mut self) -> BridgeResult<Oid> {
        let v = self.get(&format!("{PREFIX}/git/ref/heads/main"))?;
        if v["ref"] != REF || v["object"]["type"] != "commit" {
            return Err(BridgeError("WRONG_REF_OBJECT"));
        }
        value_oid(&v["object"]["sha"])
    }
    pub fn commit(&mut self, id: Oid) -> BridgeResult<CommitInfo> {
        let v = self.get(&format!("{PREFIX}/git/commits/{id}"))?;
        if value_oid(&v["sha"])? != id {
            return Err(BridgeError("COMMIT_ID_MISMATCH"));
        }
        let parents = v["parents"]
            .as_array()
            .ok_or(BridgeError("INVALID_PARENTS"))?
            .iter()
            .map(|p| value_oid(&p["sha"]))
            .collect::<BridgeResult<Vec<_>>>()?;
        Ok(CommitInfo {
            tree: value_oid(&v["tree"]["sha"])?,
            parents,
            message_size: 0,
        })
    }
    pub fn blob(&mut self, id: Oid) -> BridgeResult<Vec<u8>> {
        self.blob_bounded(id, MAX_PAYLOAD)
    }
    /// Zusätzliche Grenze vor Base64-Materialisierung; keine Empfangsgrenze.
    pub fn blob_bounded(&mut self, id: Oid, maximum: usize) -> BridgeResult<Vec<u8>> {
        let v = self.get(&format!("{PREFIX}/git/blobs/{id}"))?;
        if value_oid(&v["sha"])? != id || v["encoding"] != "base64" {
            return Err(BridgeError("INVALID_BLOB_RECORD"));
        }
        let size = v["size"].as_u64().ok_or(BridgeError("INVALID_BLOB_SIZE"))?;
        if size > MAX_PAYLOAD.min(maximum) as u64 {
            return Err(BridgeError("BLOB_TOO_LARGE"));
        }
        let encoded = text(&v["content"])?;
        let digits = encoded.bytes().filter(|b| !b.is_ascii_whitespace()).count();
        if digits > (size as usize).div_ceil(3) * 4 {
            return Err(BridgeError("BLOB_SIZE_MISMATCH"));
        }
        // Auch die letzte Base64-Gruppe vor der Decoderzuteilung abrechnen:
        // dieselbe Zeichenzahl kann je nach Padding bis zu drei Bytes tragen.
        let padding = encoded
            .bytes()
            .rev()
            .filter(|b| !b.is_ascii_whitespace())
            .take(2)
            .take_while(|b| *b == b'=')
            .count();
        if digits.is_multiple_of(4) && (digits / 4 * 3).checked_sub(padding) != Some(size as usize)
        {
            return Err(BridgeError("BLOB_SIZE_MISMATCH"));
        }
        let bytes = decode_base64(encoded)?;
        if bytes.len() as u64 != size {
            return Err(BridgeError("BLOB_SIZE_MISMATCH"));
        }
        verify_blob(&bytes, id)?;
        Ok(bytes)
    }
}
fn text(v: &Value) -> BridgeResult<&str> {
    v.as_str().ok_or(BridgeError("EXPECTED_STRING"))
}
fn value_oid(v: &Value) -> BridgeResult<Oid> {
    Oid::parse(text(v)?)
}

impl<S: ResponseSource> TreeReader for RestReader<S> {
    fn tree(&mut self, id: Oid) -> BridgeResult<TreeRecord> {
        self.tree_bounded(id, 100_000, 32 * 1024 * 1024)
    }
}
impl<S: ResponseSource> RestReader<S> {
    /// Prüft vor der zusätzlichen Eintrags-/Namensallokation. Der JSON-Baum
    /// wurde bereits unter den getrennten REST-Bodygrenzen materialisiert.
    pub fn tree_bounded(
        &mut self,
        id: Oid,
        maximum_entries: usize,
        maximum_bytes: usize,
    ) -> BridgeResult<TreeRecord> {
        let v = self.get(&format!("{PREFIX}/git/trees/{id}"))?;
        if value_oid(&v["sha"])? != id || v["truncated"] != false {
            return Err(BridgeError("INCOMPLETE_OR_WRONG_TREE"));
        }
        let input = v["tree"].as_array().ok_or(BridgeError("INVALID_ENTRIES"))?;
        if input.len() > maximum_entries {
            return Err(BridgeError("TREE_LIMIT_EXCEEDED"));
        }
        let mut total = 0usize;
        for entry in input {
            let name = text(&entry["path"])?;
            if name.len() > 4096 {
                return Err(BridgeError("TREE_PATH_TOO_LONG"));
            }
            let mode = Mode::from_api(text(&entry["type"])?, text(&entry["mode"])?)?;
            let overhead = if mode == Mode::Directory { 27 } else { 28 };
            total = total
                .checked_add(name.len() + overhead)
                .ok_or(BridgeError("TREE_BYTE_LIMIT_EXCEEDED"))?;
            if total > maximum_bytes {
                return Err(BridgeError("TREE_BYTE_LIMIT_EXCEEDED"));
            }
        }
        let mut entries = Vec::new();
        for entry in input {
            let fields = entry.as_object().ok_or(BridgeError("INVALID_ENTRY"))?;
            if ["path", "mode", "type", "sha"]
                .iter()
                .any(|k| !fields.contains_key(*k))
                || fields
                    .keys()
                    .any(|k| !["path", "mode", "type", "sha", "size", "url"].contains(&k.as_str()))
            {
                return Err(BridgeError("INVALID_ENTRY_FIELDS"));
            }
            if fields.get("size").is_some_and(|s| s.as_u64().is_none())
                || fields.get("url").is_some_and(|s| !s.is_string())
            {
                return Err(BridgeError("INVALID_ENTRY_METADATA"));
            }
            entries.push(TreeEntry {
                name: text(&entry["path"])?.to_owned(),
                mode: Mode::from_api(text(&entry["type"])?, text(&entry["mode"])?)?,
                oid: value_oid(&entry["sha"])?,
            });
        }
        // Verification and canonical name validation happen in flatten_verified.
        Ok(TreeRecord {
            oid: id,
            truncated: false,
            entries,
        })
    }
}

/// Strict padded base64; ASCII layout whitespace is allowed, noncanonical bits fail.
pub fn decode_base64(encoded: &str) -> BridgeResult<Vec<u8>> {
    decode_base64_bounded(encoded, MAX_PAYLOAD)
}

/// Derselbe Decoder für begrenzte Offline-Transportbytes. Der Blobvertrag
/// behält über decode_base64 unverändert seine bisherige 1-MiB-Grenze.
pub(crate) fn decode_base64_bounded(encoded: &str, maximum: usize) -> BridgeResult<Vec<u8>> {
    if maximum > 8 * 1024 * 1024
        || encoded.len() > (8usize * 1024 * 1024).max(maximum.div_ceil(3) * 4)
    {
        return Err(BridgeError("BASE64_TOO_LARGE"));
    }
    let data: Vec<u8> = encoded
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect();
    if !data.len().is_multiple_of(4) || data.len() / 4 * 3 > maximum + 2 {
        return Err(BridgeError("INVALID_BASE64_SIZE"));
    }
    fn digit(b: u8) -> BridgeResult<u8> {
        match b {
            b'A'..=b'Z' => Ok(b - b'A'),
            b'a'..=b'z' => Ok(b - b'a' + 26),
            b'0'..=b'9' => Ok(b - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(BridgeError("INVALID_BASE64")),
        }
    }
    let mut out = Vec::new();
    for (i, c) in data.as_chunks::<4>().0.iter().enumerate() {
        let a = digit(c[0])?;
        let b = digit(c[1])?;
        let last = i == data.len() / 4 - 1;
        out.push((a << 2) | (b >> 4));
        if c[2] == b'=' {
            if !last || c[3] != b'=' || b & 15 != 0 {
                return Err(BridgeError("INVALID_BASE64_PADDING"));
            }
        } else {
            let d = digit(c[2])?;
            out.push((b << 4) | (d >> 2));
            if c[3] == b'=' {
                if !last || d & 3 != 0 {
                    return Err(BridgeError("INVALID_BASE64_PADDING"));
                }
            } else {
                out.push((d << 6) | digit(c[3])?);
            }
        }
    }
    if out.len() > maximum {
        return Err(BridgeError("BLOB_TOO_LARGE"));
    }
    Ok(out)
}

pub fn encode_base64(bytes: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for c in bytes.chunks(3) {
        let a = c[0];
        let b = *c.get(1).unwrap_or(&0);
        let d = *c.get(2).unwrap_or(&0);
        out.push(alphabet[(a >> 2) as usize] as char);
        out.push(alphabet[(((a & 3) << 4) | (b >> 4)) as usize] as char);
        out.push(if c.len() > 1 {
            alphabet[(((b & 15) << 2) | (d >> 6)) as usize] as char
        } else {
            '='
        });
        out.push(if c.len() > 2 {
            alphabet[(d & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// Raw bytes must come from a separate immutable-object source, never synthesized
/// from REST timestamps/identity strings. GitHub raw acquisition remains unwired.
pub trait RawCommitSource {
    fn raw_commit(&mut self, id: Oid) -> BridgeResult<Vec<u8>>;
}

#[cfg(test)]
#[path = "raw_commit_tests.rs"]
mod raw_commit_tests;
impl<F: FnMut(Oid) -> BridgeResult<Vec<u8>>> RawCommitSource for F {
    fn raw_commit(&mut self, id: Oid) -> BridgeResult<Vec<u8>> {
        self(id)
    }
}

pub(crate) fn check_commit<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    id: Oid,
) -> BridgeResult<CommitInfo> {
    let meta = reader.commit(id)?;
    let bytes = raw.raw_commit(id)?;
    let info = inspect_commit(&bytes)?;
    if object_oid(ObjectKind::Commit, &bytes) != id {
        return Err(BridgeError("COMMIT_HASH_MISMATCH"));
    }
    if info.tree != meta.tree || info.parents != meta.parents {
        return Err(BridgeError("COMMIT_METADATA_MISMATCH"));
    }
    Ok(info)
}

pub(crate) fn verify_transition_objects<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
    baseline: Option<&[u8]>,
) -> BridgeResult<(Oid, [Oid; 2])> {
    // A PreparedStatus may also originate from document-level continuation.
    // This verifier's initial-addition contract still requires a null checkpoint.
    let candidate =
        crate::status::Status::parse(expected.bytes(), crate::status::PreviousKind::Local)?;
    if let Some(bytes) = baseline {
        let previous = crate::status::Status::parse(bytes, crate::status::PreviousKind::Local)?;
        crate::status::prepare(&previous, &candidate)?;
    } else {
        crate::status::prepare_initial(&candidate)?;
    }
    if before == after {
        return Err(BridgeError("UNCHANGED_COMMIT"));
    }
    let old = check_commit(reader, raw, before)?;
    let new = check_commit(reader, raw, after)?;
    if new.parents != [before] {
        return Err(BridgeError("COMMIT_PARENT_MISMATCH"));
    }
    let roots = [old.tree, new.tree];
    let old = flatten_verified(reader, old.tree)?;
    let new = flatten_verified(reader, new.tree)?;
    let blob = object_oid(ObjectKind::Blob, expected.bytes());
    if let Some(bytes) = baseline {
        let baseline_oid = object_oid(ObjectKind::Blob, bytes);
        verify_single_replacement(&old, &new, baseline_oid, blob)?;
        if reader.blob(baseline_oid)? != bytes {
            return Err(BridgeError("STALE_BASE_CONFLICT"));
        }
    } else {
        verify_single_addition(&old, &new, TARGET, blob)?;
    }
    if reader.blob(blob)? != expected.bytes() {
        return Err(BridgeError("PAYLOAD_MISMATCH"));
    }
    Ok((blob, roots))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadbackReport {
    pub commit: Oid,
    pub blob: Oid,
    pub commit_hash_verified: bool,
    pub publication_performed: bool,
    pub scope: &'static str,
}

pub fn verify_initial_index<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
) -> BridgeResult<ReadbackReport> {
    verify_transition_index(reader, raw, repository_id, before, after, expected, None)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_update_index<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
    baseline: &[u8],
) -> BridgeResult<ReadbackReport> {
    verify_transition_index(
        reader,
        raw,
        repository_id,
        before,
        after,
        expected,
        Some(baseline),
    )
}

fn verify_transition_index<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
    baseline: Option<&[u8]>,
) -> BridgeResult<ReadbackReport> {
    if reader.repository()? != repository_id || repository_id.trim().is_empty() {
        return Err(BridgeError("REPOSITORY_ID_MISMATCH"));
    }
    if reader.head()? != after {
        return Err(BridgeError("INITIAL_HEAD_MISMATCH"));
    }
    let (blob, _) = verify_transition_objects(reader, raw, before, after, expected, baseline)?;
    if reader.head()? != after {
        return Err(BridgeError("FINAL_HEAD_MISMATCH"));
    }
    Ok(ReadbackReport {
        commit: after,
        blob,
        commit_hash_verified: true,
        publication_performed: false,
        scope: "OFFLINE_SUPPLIED_RESPONSES",
    })
}

/// Pure GraphQL request builder. Never executes this request.
pub fn update_refs_request(
    repository_id: &str,
    ref_name: &str,
    before: Oid,
    after: Oid,
) -> BridgeResult<Value> {
    if repository_id.trim().is_empty() || repository_id.len() > 1024 || ref_name != REF {
        return Err(BridgeError("INVALID_REPOSITORY_OR_REF"));
    }
    if before == after {
        return Err(BridgeError("UNCHANGED_COMMIT"));
    }
    Ok(
        json!({"query":"mutation($input: UpdateRefsInput!) { updateRefs(input: $input) { clientMutationId } }", "variables":{"input":{"repositoryId":repository_id,"refUpdates":[{"name":REF,"beforeOid":before.to_string(),"afterOid":after.to_string(),"force":false}]}}}),
    )
}
fn exact(v: &Value, keys: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}
pub fn valid_cas_response(bytes: &[u8]) -> bool {
    if bytes.len() > 1024 * 1024 {
        return false;
    }
    parse_exchange_v1(bytes).is_ok_and(|v| {
        exact(&v, &["data"])
            && exact(&v["data"], &["updateRefs"])
            && exact(&v["data"]["updateRefs"], &["clientMutationId"])
            && v["data"]["updateRefs"]["clientMutationId"].is_null()
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyntheticOutcome {
    Conflict,
    Verified(ReadbackReport),
    MutationUnknown,
}

/// Synthetic injected transport only. Exactly one attempt; every error after
/// invocation is ambiguous, with no retry or rollback path in this library.
pub fn simulate_graphql<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    simulated_reply: impl FnMut(&Value) -> BridgeResult<Vec<u8>>,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
) -> BridgeResult<SyntheticOutcome> {
    simulate_transition(
        reader,
        raw,
        simulated_reply,
        repository_id,
        before,
        after,
        expected,
        None,
    )
}

/// Offline synthetic update CAS only. No retry or rollback, including ambiguous results.
#[allow(clippy::too_many_arguments)]
pub fn simulate_update_graphql<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    simulated_reply: impl FnMut(&Value) -> BridgeResult<Vec<u8>>,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
    baseline: &[u8],
) -> BridgeResult<SyntheticOutcome> {
    simulate_transition(
        reader,
        raw,
        simulated_reply,
        repository_id,
        before,
        after,
        expected,
        Some(baseline),
    )
}

#[allow(clippy::too_many_arguments)]
fn simulate_transition<S: ResponseSource>(
    reader: &mut RestReader<S>,
    raw: &mut impl RawCommitSource,
    mut simulated_reply: impl FnMut(&Value) -> BridgeResult<Vec<u8>>,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &PreparedStatus,
    baseline: Option<&[u8]>,
) -> BridgeResult<SyntheticOutcome> {
    let request = update_refs_request(repository_id, REF, before, after)?;
    if reader.repository()? != repository_id {
        return Err(BridgeError("REPOSITORY_ID_MISMATCH"));
    }
    verify_transition_objects(reader, raw, before, after, expected, baseline)?;
    if reader.head()? != before {
        return Ok(SyntheticOutcome::Conflict);
    }
    let reply = match simulated_reply(&request) {
        Ok(v) => v,
        Err(_) => return Ok(SyntheticOutcome::MutationUnknown),
    };
    if !valid_cas_response(&reply) {
        return Ok(SyntheticOutcome::MutationUnknown);
    }
    match verify_transition_index(
        reader,
        raw,
        repository_id,
        before,
        after,
        expected,
        baseline,
    ) {
        Ok(report) => Ok(SyntheticOutcome::Verified(report)),
        Err(_) => Ok(SyntheticOutcome::MutationUnknown),
    }
}

pub fn production_publish() -> BridgeResult<()> {
    Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
}

/// Verify a local self-contained bundle. It cannot establish remote freshness or
/// source authenticity: pinned IDs and supplied original bytes are the inputs.
pub fn verify_offline_bundle(bytes: &[u8]) -> BridgeResult<ReadbackReport> {
    verify_bundle(bytes, false)
}

/// Full update transition, deliberately separate from snapshot attestation.
pub fn verify_update_bundle(bytes: &[u8]) -> BridgeResult<ReadbackReport> {
    verify_bundle(bytes, true)
}

fn verify_bundle(bytes: &[u8], update: bool) -> BridgeResult<ReadbackReport> {
    if bytes.len() > 32 * 1024 * 1024 {
        return Err(BridgeError("BUNDLE_TOO_LARGE"));
    }
    let bundle = parse_exchange_v1(bytes).map_err(|_| BridgeError("INVALID_BUNDLE_JSON"))?;
    let mut fields = vec![
        "repository_id",
        "before",
        "after",
        "candidate",
        "responses",
        "raw_commits",
    ];
    if update {
        fields.push("baseline_base64");
    }
    if !exact(&bundle, &fields) {
        return Err(BridgeError("INVALID_BUNDLE_FIELDS"));
    }
    let repository_id = text(&bundle["repository_id"])?;
    let before = value_oid(&bundle["before"])?;
    let after = value_oid(&bundle["after"])?;
    let candidate = crate::status::Status::from_value(
        bundle["candidate"].clone(),
        crate::status::PreviousKind::Local,
    )?;
    let baseline = if update {
        Some(decode_base64(text(&bundle["baseline_base64"])?)?)
    } else {
        None
    };
    let expected = if let Some(bytes) = baseline.as_deref() {
        crate::status::prepare(
            &crate::status::Status::parse(bytes, crate::status::PreviousKind::Local)?,
            &candidate,
        )?
    } else {
        crate::status::prepare_initial(&candidate)?
    };
    let responses = bundle["responses"]
        .as_object()
        .ok_or(BridgeError("INVALID_BUNDLE_RESPONSES"))?;
    let commits = bundle["raw_commits"]
        .as_object()
        .ok_or(BridgeError("INVALID_BUNDLE_COMMITS"))?;
    // Invalid or missing route/data never falls back to filesystem or network.
    let mut reader = RestReader::new(
        |path: &str| {
            serde_json::to_vec(responses.get(path).ok_or(BridgeError("MISSING_RESPONSE"))?)
                .map_err(|_| BridgeError("INVALID_RESPONSE"))
        },
        256,
    )?;
    let mut raw = |id: Oid| {
        decode_base64(text(
            commits
                .get(&id.to_string())
                .ok_or(BridgeError("MISSING_RAW_COMMIT"))?,
        )?)
    };
    if let Some(baseline) = baseline.as_deref() {
        verify_update_index(
            &mut reader,
            &mut raw,
            repository_id,
            before,
            after,
            &expected,
            baseline,
        )
    } else {
        verify_initial_index(
            &mut reader,
            &mut raw,
            repository_id,
            before,
            after,
            &expected,
        )
    }
}

impl ReadbackReport {
    pub fn as_json(&self) -> Value {
        json!({"verification":"PASS","scope":self.scope,"target":TARGET,"commit":self.commit.to_string(),"blob_oid":self.blob.to_string(),"commit_hash_verified":self.commit_hash_verified,"publication_performed":self.publication_performed,"bridge_publication":"BLOCKED"})
    }
}
