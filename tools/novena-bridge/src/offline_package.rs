//! Versionierter Offline-Replayvertrag. Keine Authentifizierung, kein Statusindex.
//! REST-Bodies sind Originalantwortbytes; Tree-Nutzbytes werden ausschließlich
//! mit dem vorhandenen Tree-Verifier rekonstruiert. Kein Archiv-/Pfadparser.
use crate::object_evidence::OfflineObjectEvidence;
use crate::objects::Oid;
use crate::offline_readback::{verify_complete_readback, CompleteReadback, RunBudget};
use crate::protocol::{decode_base64_bounded, encode_base64, RestReader};
use crate::status::{prepare, prepare_initial, PreviousKind, Status};
use crate::{BridgeError, BridgeResult};
use serde_json::{json, Value};
use std::collections::{BTreeMap, VecDeque};
use std::io::{Read, Write};
use std::path::Path;

const MIB: usize = 1024 * 1024;
pub const MAX_PACKAGE_BYTES: usize = 64 * MIB;
const MAX_TRANSPORT_BYTES: usize = 32 * MIB;
const CONTRACT: &str = "NOVENA_OFFLINE_REPLAY_V1";
const FILE: &str = "novena-offline-replay-v1.json";

#[derive(Debug)]
pub struct OfflinePackage {
    bytes: Vec<u8>,
}
impl OfflinePackage {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_readback(
        repository_id: &str,
        before: Oid,
        after: Oid,
        expected: &[u8],
        baseline: Option<&[u8]>,
        responses: &[(String, Vec<u8>)],
        commits: &BTreeMap<Oid, Vec<u8>>,
        inventory: &OfflineObjectEvidence,
    ) -> BridgeResult<Self> {
        // Vor Base64-/JSON-Allokation: feste Anzahl und konservative obere
        // Serialisierungsgrenze inklusive JSON-Escaping und Inventarduplikat.
        if responses.len() > 256
            || commits.len() != 2
            || expected.len() > MIB
            || baseline.is_some_and(|b| b.len() > MIB)
            || inventory.objects().len() > 4354
        {
            return Err(BridgeError("PACKAGE_LIMIT"));
        }
        let mut identities = std::collections::BTreeSet::new();
        if inventory
            .objects()
            .iter()
            .any(|o| !identities.insert(o.oid()))
        {
            return Err(BridgeError("PACKAGE_AMBIGUOUS_IDENTITY"));
        }
        let mut transport = 0usize;
        let mut bound = inventory.bytes().len() + 16384;
        for (path, body) in responses {
            crate::protocol::allowed_endpoint(path)?;
            if body.len() > 8 * MIB || path.len() > 256 {
                return Err(BridgeError("PACKAGE_LIMIT"));
            }
            transport = transport
                .checked_add(body.len())
                .ok_or(BridgeError("PACKAGE_LIMIT"))?;
            bound += body.len().div_ceil(3) * 4 + path.len() * 6 + 128;
        }
        for body in commits.values() {
            if body.len() > MIB {
                return Err(BridgeError("PACKAGE_LIMIT"));
            }
            transport += body.len();
            bound += body.len().div_ceil(3) * 4 + 128;
        }
        bound += (expected.len() + baseline.map_or(0, <[u8]>::len)).div_ceil(3) * 4 + 16;
        if transport > MAX_TRANSPORT_BYTES || bound > MAX_PACKAGE_BYTES {
            return Err(BridgeError("PACKAGE_LIMIT"));
        }
        let responses: Vec<_> = responses
            .iter()
            .map(|(path, body)| json!({"endpoint":path,"body_base64":encode_base64(body)}))
            .collect();
        let commits: Vec<_> = commits.iter().map(|(oid, body)|
            json!({"git_oid":oid.to_string(),"payload_base64":encode_base64(body)})).collect();
        let inventory = crate::parse_exchange_v1(inventory.bytes())
            .map_err(|_| BridgeError("PACKAGE_INVENTORY"))?;
        let document = json!({"contract":CONTRACT,"scope":"OFFLINE_ONLY",
            "repository_id":repository_id,"before":before.to_string(),"after":after.to_string(),
            "expected_base64":encode_base64(expected),
            "baseline_base64":baseline.map(encode_base64),
            "responses":responses,"original_commits":commits,"inventory":inventory});
        Ok(Self {
            bytes: canonical(&document)?,
        })
    }
}
fn canonical(value: &Value) -> BridgeResult<Vec<u8>> {
    let bytes = serde_json::to_vec(value).map_err(|_| BridgeError("PACKAGE_JSON"))?;
    crate::canonical_json(&bytes).map_err(|_| BridgeError("PACKAGE_JSON"))
}
fn text(v: &Value) -> BridgeResult<&str> {
    v.as_str().ok_or(BridgeError("PACKAGE_FIELD"))
}
fn exact(v: &Value, keys: &[&str]) -> BridgeResult<()> {
    let map = v.as_object().ok_or(BridgeError("PACKAGE_FIELDS"))?;
    if map.len() != keys.len() || keys.iter().any(|k| !map.contains_key(*k)) {
        return Err(BridgeError("PACKAGE_FIELDS"));
    }
    Ok(())
}
fn payload(v: &Value, maximum: usize, total: &mut usize) -> BridgeResult<Vec<u8>> {
    let s = text(v)?;
    if s.len() > maximum.div_ceil(3) * 4 {
        return Err(BridgeError("PACKAGE_LIMIT"));
    }
    // Länge vor dem Decoder berechnen; kanonisches Base64 wird nachgerechnet.
    if !s.len().is_multiple_of(4) {
        return Err(BridgeError("PACKAGE_BASE64"));
    }
    let padding = s
        .as_bytes()
        .iter()
        .rev()
        .take_while(|b| **b == b'=')
        .count();
    if padding > 2 {
        return Err(BridgeError("PACKAGE_BASE64"));
    }
    let length = (s.len() / 4 * 3)
        .checked_sub(padding)
        .ok_or(BridgeError("PACKAGE_BASE64"))?;
    if length > maximum || length > MAX_TRANSPORT_BYTES.saturating_sub(*total) {
        return Err(BridgeError("PACKAGE_LIMIT"));
    }
    let bytes = decode_base64_bounded(s, maximum)?;
    if bytes.len() != length || encode_base64(&bytes) != s {
        return Err(BridgeError("PACKAGE_BASE64"));
    }
    *total += length;
    Ok(bytes)
}

/// Unabhängige Neuberechnung mit frischem, gemeinsamem Budget. Weder das
/// Inventar noch darin enthaltene Erfolgs-/Herkunftsbehauptungen werden vertraut.
/// Ein reines V1-Inventar ohne Replaybytes wird geschlossen abgelehnt.
pub fn verify_package(bytes: &[u8]) -> BridgeResult<CompleteReadback> {
    verify_package_in_run(bytes, &RunBudget::new())
}

/// Der Aufrufer kann denselben Budgetkontext über die gesamte Laufbindung halten.
pub(crate) fn verify_package_in_run(
    bytes: &[u8],
    budget: &RunBudget,
) -> BridgeResult<CompleteReadback> {
    budget.check()?;
    budget.fail(replay(bytes, budget))
}

fn replay(bytes: &[u8], budget: &RunBudget) -> BridgeResult<CompleteReadback> {
    if bytes.len() > MAX_PACKAGE_BYTES {
        return Err(BridgeError("PACKAGE_LIMIT"));
    }
    let v = crate::parse_exchange_v1(bytes).map_err(|_| BridgeError("PACKAGE_JSON"))?;
    exact(
        &v,
        &[
            "contract",
            "scope",
            "repository_id",
            "before",
            "after",
            "expected_base64",
            "baseline_base64",
            "responses",
            "original_commits",
            "inventory",
        ],
    )?;
    if v["contract"] != CONTRACT || v["scope"] != "OFFLINE_ONLY" {
        return Err(BridgeError("PACKAGE_CONTRACT"));
    }
    if canonical(&v)? != bytes {
        return Err(BridgeError("PACKAGE_NONCANONICAL"));
    }
    let repository_id = text(&v["repository_id"])?;
    if repository_id.len() > 1024 {
        return Err(BridgeError("PACKAGE_LIMIT"));
    }
    let before = Oid::parse(text(&v["before"])?)?;
    let after = Oid::parse(text(&v["after"])?)?;
    let mut status_total = 0;
    let expected = payload(&v["expected_base64"], MIB, &mut status_total)?;
    let baseline = if v["baseline_base64"].is_null() {
        None
    } else {
        Some(payload(&v["baseline_base64"], MIB, &mut status_total)?)
    };
    let candidate = Status::parse(&expected, PreviousKind::Local)?;
    let prepared = if let Some(old) = baseline.as_deref() {
        prepare(&Status::parse(old, PreviousKind::Local)?, &candidate)?
    } else {
        prepare_initial(&candidate)?
    };
    if prepared.bytes() != expected {
        return Err(BridgeError("PACKAGE_STATUS_BYTES"));
    }
    let responses = v["responses"]
        .as_array()
        .ok_or(BridgeError("PACKAGE_FIELD"))?;
    let commits = v["original_commits"]
        .as_array()
        .ok_or(BridgeError("PACKAGE_FIELD"))?;
    if responses.len() > 256 || commits.len() != 2 {
        return Err(BridgeError("PACKAGE_LIMIT"));
    }
    let mut total = 0;
    let mut queue = VecDeque::new();
    for r in responses {
        exact(r, &["endpoint", "body_base64"])?;
        let endpoint = text(&r["endpoint"])?;
        if endpoint.len() > 256 {
            return Err(BridgeError("PACKAGE_LIMIT"));
        }
        crate::protocol::allowed_endpoint(endpoint)?;
        queue.push_back((
            endpoint.to_owned(),
            payload(&r["body_base64"], 8 * MIB, &mut total)?,
        ));
    }
    let mut raw_map = BTreeMap::new();
    let mut previous = None;
    for c in commits {
        exact(c, &["git_oid", "payload_base64"])?;
        let oid = Oid::parse(text(&c["git_oid"])?)?;
        if previous.is_some_and(|p| p >= oid) {
            return Err(BridgeError("PACKAGE_OBJECT_ORDER"));
        }
        previous = Some(oid);
        raw_map.insert(oid, payload(&c["payload_base64"], MIB, &mut total)?);
    }
    let result = {
        let mut reader = RestReader::new(
            budget.responses(|path: &str| {
                let (endpoint, body) = queue
                    .pop_front()
                    .ok_or(BridgeError("PACKAGE_MISSING_RESPONSE"))?;
                if path != endpoint {
                    return Err(BridgeError("PACKAGE_RESPONSE_ORDER"));
                }
                Ok(body)
            }),
            256,
        )?;
        let mut raw = budget.commits(|oid: Oid| {
            raw_map
                .remove(&oid)
                .ok_or(BridgeError("PACKAGE_MISSING_COMMIT"))
        });
        verify_complete_readback(
            &mut reader,
            &mut raw,
            repository_id,
            before,
            after,
            &prepared,
            baseline.as_deref(),
            budget,
        )?
    };
    if !queue.is_empty() || !raw_map.is_empty() {
        return Err(BridgeError("PACKAGE_UNUSED_OBJECTS"));
    }
    // Exakter Vergleich des neu berechneten Inventars und Vertrags schließt
    // unbekannte Felder, falsche Hashes/Größen, Inventarlücken und Zusätze aus.
    if result.package().bytes() != bytes {
        return Err(BridgeError("PACKAGE_RECOMPUTATION_MISMATCH"));
    }
    Ok(result)
}

/// Nur explizit gewählte private, während der Operation unveränderliche
/// Verzeichnisse einschließlich aller Eltern. Kein Schutz gegen konkurrierende
/// Pfad-/Mountwechsel oder offene Schreibhandles; TOCTOU bleibt beim Aufrufer.
fn directory(path: &Path) -> BridgeResult<()> {
    if !path.is_absolute()
        || path.components().any(|c| {
            !matches!(
                c,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
    {
        return Err(BridgeError("PACKAGE_DIRECTORY"));
    }
    for ancestor in path.ancestors() {
        let m =
            std::fs::symlink_metadata(ancestor).map_err(|_| BridgeError("PACKAGE_DIRECTORY"))?;
        if !m.is_dir() || m.file_type().is_symlink() {
            return Err(BridgeError("PACKAGE_DIRECTORY"));
        }
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if std::fs::metadata(path)
            .map_err(|_| BridgeError("PACKAGE_DIRECTORY"))?
            .permissions()
            .mode()
            & 0o077
            != 0
        {
            return Err(BridgeError("PACKAGE_DIRECTORY_NOT_PRIVATE"));
        }
    }
    Ok(())
}
/// Export nur in leeres privates Verzeichnis; create_new verhindert Überschreiben.
/// Fehler können eine unvollständige Datei hinterlassen, niemals einen Erfolg.
pub fn export_private(package: &OfflinePackage, path: &Path) -> BridgeResult<()> {
    directory(path)?;
    if std::fs::read_dir(path)
        .map_err(|_| BridgeError("PACKAGE_DIRECTORY"))?
        .next()
        .is_some()
    {
        return Err(BridgeError("PACKAGE_DIRECTORY_NOT_EMPTY"));
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path.join(FILE))
        .map_err(|_| BridgeError("PACKAGE_CREATE"))?;
    file.write_all(package.bytes())
        .and_then(|()| file.sync_all())
        .map_err(|_| BridgeError("PACKAGE_WRITE"))
}
/// Genau eine feste reguläre Datei; keine Pfade aus JSON, kein Git-Repository.
pub fn import_private(path: &Path) -> BridgeResult<CompleteReadback> {
    directory(path)?;
    let mut entries = std::fs::read_dir(path).map_err(|_| BridgeError("PACKAGE_DIRECTORY"))?;
    let entry = entries
        .next()
        .ok_or(BridgeError("PACKAGE_MISSING_FILE"))?
        .map_err(|_| BridgeError("PACKAGE_DIRECTORY"))?;
    if entry.file_name() != FILE || entries.next().is_some() {
        return Err(BridgeError("PACKAGE_DIRECTORY_CONTENTS"));
    }
    let name = path.join(FILE);
    let meta = std::fs::symlink_metadata(&name).map_err(|_| BridgeError("PACKAGE_FILE"))?;
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > MAX_PACKAGE_BYTES as u64 {
        return Err(BridgeError("PACKAGE_FILE"));
    }
    let mut file = std::fs::File::open(name).map_err(|_| BridgeError("PACKAGE_FILE"))?;
    let opened = file.metadata().map_err(|_| BridgeError("PACKAGE_FILE"))?;
    if !opened.is_file() || opened.len() != meta.len() {
        return Err(BridgeError("PACKAGE_FILE"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if meta.dev() != opened.dev() || meta.ino() != opened.ino() || opened.nlink() != 1 {
            return Err(BridgeError("PACKAGE_FILE"));
        }
    }
    let mut bytes = Vec::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|_| BridgeError("PACKAGE_READ"))?;
        if n == 0 {
            break;
        }
        if n > MAX_PACKAGE_BYTES - bytes.len() {
            return Err(BridgeError("PACKAGE_LIMIT"));
        }
        bytes.extend_from_slice(&buffer[..n]);
    }
    if bytes.len() as u64 != opened.len() {
        return Err(BridgeError("PACKAGE_FILE"));
    }
    verify_package(&bytes)
}
