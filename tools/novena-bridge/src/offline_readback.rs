//! Strikt lesende Offline-Verträge nach Architekturentscheid 292.
//! Keine Prozess-, Credential-, Dateipfad- oder Netzwerkbeschaffung.
use crate::objects::*;
use crate::protocol::{allowed_endpoint, RawCommitSource, ResponseSource, RestReader};
use crate::{BridgeError, BridgeResult};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

const MIB: usize = 1024 * 1024;

/// Nur Offline ist konstruierbar. LIVE erfordert künftig einen getrennten,
/// authentifizierten Erwerbstyp; Eingangsmanifeste können ihn nicht erzeugen.
#[derive(Debug, PartialEq, Eq)]
pub enum RunScope {
    Offline,
}

#[derive(Debug)]
pub enum ReadbackMode<'a> {
    Snapshot {
        expected: &'a [u8],
    },
    Transition {
        baseline: Oid,
        old: &'a [u8],
        new: &'a [u8],
    },
}

/// Pins sind explizite Erwartungen des Aufrufers, kein Herkunftsnachweis.
pub struct ReadbackSpec<'a> {
    commit: Oid,
    mode: ReadbackMode<'a>,
    node_id: &'a str,
    numeric_id: u64,
}
impl<'a> ReadbackSpec<'a> {
    pub fn new(
        node_id: &'a str,
        numeric_id: u64,
        commit: Oid,
        mode: ReadbackMode<'a>,
    ) -> BridgeResult<Self> {
        if node_id.trim().is_empty() || node_id.len() > 1024 || numeric_id == 0 {
            return Err(BridgeError("INVALID_REPOSITORY_PIN"));
        }
        let valid = match &mode {
            ReadbackMode::Snapshot { expected } => expected.len() <= MIB,
            ReadbackMode::Transition { baseline, old, new } => {
                *baseline != commit && old.len() <= MIB && new.len() <= MIB
            }
        };
        if !valid {
            return Err(BridgeError("INVALID_READBACK_SPEC"));
        }
        Ok(Self {
            commit,
            mode,
            node_id,
            numeric_id,
        })
    }
    pub fn repository(&self) -> &'static str {
        crate::status::ARCHIVE
    }
    pub fn reference(&self) -> &'static str {
        crate::status::REF
    }
    pub fn rest_origin(&self) -> &'static str {
        "https://api.github.com:443"
    }
    /// Nur fester zukünftiger Zielvertrag, kein Erwerbsadapter.
    pub fn planned_git_target(&self) -> &'static str {
        "git@github.com:loomit77/project-block-logs.git"
    }
    pub fn pins(&self) -> (&str, u64) {
        (self.node_id, self.numeric_id)
    }
    pub fn commit(&self) -> Oid {
        self.commit
    }
    pub fn mode(&self) -> &ReadbackMode<'a> {
        &self.mode
    }
    pub fn scope(&self) -> RunScope {
        RunScope::Offline
    }
    pub fn endpoint(&self, path: &str) -> BridgeResult<()> {
        allowed_endpoint(path)
    }
}

/// Unbeglaubigte Erwerbsbehauptung, ausdrücklich kein LIVE-Beleg.
/// Hashdomäne: unveränderte Commit-Nutzbytes ohne Git-Header.
pub struct ExportClaim {
    pub oid: Oid,
    pub length: usize,
    pub payload_sha256: [u8; 32],
}
pub struct AcquisitionManifest<'a> {
    original: &'a [u8],
    exports: &'a [ExportClaim],
}
impl<'a> AcquisitionManifest<'a> {
    pub fn untrusted(original: &'a [u8], exports: &'a [ExportClaim]) -> BridgeResult<Self> {
        if original.len() > 64 * MIB || exports.is_empty() || exports.len() > 2 {
            return Err(BridgeError("MANIFEST_LIMIT_EXCEEDED"));
        }
        for (i, item) in exports.iter().enumerate() {
            if item.length > MIB || exports[..i].iter().any(|old| old.oid == item.oid) {
                return Err(BridgeError("INVALID_EXPORT_CLAIM"));
            }
        }
        Ok(Self { original, exports })
    }
    pub fn original(&self) -> &[u8] {
        self.original
    }
    pub fn exports(&self) -> &[ExportClaim] {
        self.exports
    }
    pub fn provenance(&self) -> &'static str {
        "UNTRUSTED_ACQUISITION_CLAIM"
    }
}

/// Fester Typvertrag, keine Behauptung einer OS-/Deadline-Durchsetzung.
pub struct SupervisorLimits;
impl SupervisorLimits {
    pub const COMMITS: usize = 2;
    pub const SSH_SESSIONS: usize = 1;
    pub const SSH_WIRE_BYTES: usize = 64 * MIB;
    pub const ACQUISITION_DISK_BYTES: usize = 128 * MIB;
    pub const EVIDENCE_BYTES: usize = 64 * MIB;
    pub const RUN_SECONDS: u64 = 120;
    pub const OPERATION_SECONDS: u64 = 30;
    pub const CONNECT_SECONDS: u64 = 10;
    pub const HEADERS_SECONDS: u64 = 10;
    pub const EXPORT_SECONDS: u64 = 5;
    pub const RAM_BYTES: usize = 512 * MIB;
    pub const SWAP_BYTES: usize = 0;
    pub const CPU_SECONDS: u64 = 60;
    pub const PROCESSES: usize = 16;
    pub const HEADER_BYTES_BLOCKED: usize = 32 * 1024;
}

#[derive(Default)]
struct Usage {
    failed: bool,
    requests: usize,
    bodies: usize,
    raw: usize,
    commits: usize,
    trees: usize,
    entries: usize,
    tree_bytes: usize,
    blob_bytes: usize,
    transcript: Vec<(String, Vec<u8>)>,
}
/// Ein gemeinsamer Kontext kann über mehrere Reader/Quellen hinweg verwendet
/// werden. Quellen-/Budgetfehler und Fehler des Vollständigkeitsprüfers
/// vergiften ihn; keine Fortsetzung mit Teil-Erfolg. Fehler eines separat
/// aufgerufenen alten Verifiers muss der Aufrufer weiterhin als Abbruch behandeln.
#[derive(Default)]
pub struct RunBudget(RefCell<Usage>);
impl RunBudget {
    pub fn new() -> Self {
        Self::default()
    }
    /// Vor Dateizugriff prüfen; die tatsächliche Abrechnung erfolgt genau
    /// einmal durch BudgetedCommits im bestehenden vollständigen Replay.
    pub(crate) fn handoff_capacity(&self, claims: &[ExportClaim]) -> BridgeResult<()> {
        self.check()?;
        let u = self.0.borrow();
        let bytes = claims
            .iter()
            .try_fold(0usize, |total, c| total.checked_add(c.length))
            .ok_or(BridgeError("RUN_BUDGET_EXCEEDED"))?;
        if claims.len() > 2usize.saturating_sub(u.commits)
            || bytes > (2 * MIB).saturating_sub(u.raw)
            || bytes > (32 * MIB).saturating_sub(u.bodies + u.raw)
        {
            return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
        }
        Ok(())
    }
    pub(crate) fn consumption(&self) -> [usize; 8] {
        let u = self.0.borrow();
        [
            u.requests,
            u.bodies,
            u.raw,
            u.commits,
            u.trees,
            u.entries,
            u.tree_bytes,
            u.blob_bytes,
        ]
    }
    pub(crate) fn check(&self) -> BridgeResult<()> {
        if self.0.borrow().failed {
            Err(BridgeError("RUN_ALREADY_FAILED"))
        } else {
            Ok(())
        }
    }
    pub(crate) fn fail<T>(&self, result: BridgeResult<T>) -> BridgeResult<T> {
        if result.is_err() {
            self.0.borrow_mut().failed = true;
        }
        result
    }
    pub fn responses<S: ResponseSource>(&self, source: S) -> BudgetedResponses<'_, S> {
        BudgetedResponses {
            budget: self,
            source,
        }
    }
    pub fn commits<S: RawCommitSource>(&self, source: S) -> BudgetedCommits<'_, S> {
        BudgetedCommits {
            budget: self,
            source,
            verified: BTreeMap::new(),
        }
    }
}
pub struct BudgetedResponses<'a, S> {
    budget: &'a RunBudget,
    source: S,
}
impl<S: ResponseSource> ResponseSource for BudgetedResponses<'_, S> {
    fn get(&mut self, endpoint: &str) -> BridgeResult<Vec<u8>> {
        let result = (|| {
            self.budget.check()?;
            allowed_endpoint(endpoint)?;
            {
                let mut u = self.budget.0.borrow_mut();
                if u.requests >= 256 || u.bodies + u.raw >= 32 * MIB {
                    return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
                }
                u.requests += 1;
            }
            // Die Quelle allokiert selbst: Grenze gilt vor Parserverarbeitung,
            // nicht als Empfangs-/Allocatorgarantie einer fremden Quelle.
            let bytes = self.source.get(endpoint)?;
            let mut u = self.budget.0.borrow_mut();
            if bytes.len() > 8 * MIB || bytes.len() > 32 * MIB - u.bodies - u.raw {
                return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
            }
            u.bodies += bytes.len();
            u.transcript.push((endpoint.to_owned(), bytes.clone()));
            Ok(bytes)
        })();
        self.budget.fail(result)
    }
}
pub struct BudgetedCommits<'a, S> {
    budget: &'a RunBudget,
    source: S,
    verified: BTreeMap<Oid, Vec<u8>>,
}
impl<S: RawCommitSource> RawCommitSource for BudgetedCommits<'_, S> {
    fn raw_commit(&mut self, id: Oid) -> BridgeResult<Vec<u8>> {
        let result = (|| {
            self.budget.check()?;
            {
                let mut u = self.budget.0.borrow_mut();
                if u.commits >= 2 || u.bodies + u.raw >= 32 * MIB {
                    return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
                }
                u.commits += 1;
            }
            let bytes = self.source.raw_commit(id)?;
            let mut u = self.budget.0.borrow_mut();
            if bytes.len() > MIB
                || bytes.len() > 2 * MIB - u.raw
                || bytes.len() > 32 * MIB - u.bodies - u.raw
            {
                return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
            }
            inspect_commit(&bytes)?;
            if object_oid(ObjectKind::Commit, &bytes) != id {
                return Err(BridgeError("COMMIT_HASH_MISMATCH"));
            }
            u.raw += bytes.len();
            self.verified.insert(id, bytes.clone());
            Ok(bytes)
        })();
        self.budget.fail(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobEvidence {
    pub length: usize,
    pub payload_sha256: [u8; 32],
}
/// Separater Vollständigkeitsbeleg. Kein Status, keine Publikationsfähigkeit.
#[derive(Debug)]
pub struct CompleteBlobs {
    roots: Vec<Oid>,
    blobs: BTreeMap<Oid, BlobEvidence>,
    paths: usize,
    trees: BTreeMap<Oid, BlobEvidence>,
}
impl CompleteBlobs {
    pub fn scope(&self) -> RunScope {
        RunScope::Offline
    }
    pub fn blobs(&self) -> &BTreeMap<Oid, BlobEvidence> {
        &self.blobs
    }
    pub fn roots(&self) -> &[Oid] {
        &self.roots
    }
    pub fn paths(&self) -> usize {
        self.paths
    }
}

/// Nur nach gewöhnlicher Transitionsprüfung UND vollständiger Blobprüfung.
/// Fehler liefern keinen Teilbeleg. Kein authentifizierter Herkunftstyp.
#[derive(Debug)]
pub struct CompleteReadback {
    report: crate::protocol::ReadbackReport,
    complete: CompleteBlobs,
    evidence: crate::object_evidence::OfflineObjectEvidence,
    package: crate::offline_package::OfflinePackage,
}
impl CompleteReadback {
    pub fn package(&self) -> &crate::offline_package::OfflinePackage {
        &self.package
    }
    pub fn evidence(&self) -> &crate::object_evidence::OfflineObjectEvidence {
        &self.evidence
    }
    pub fn report(&self) -> &crate::protocol::ReadbackReport {
        &self.report
    }
    pub fn complete(&self) -> &CompleteBlobs {
        &self.complete
    }
    pub fn scope(&self) -> RunScope {
        RunScope::Offline
    }
}

/// Bestehende Initial-/Update-Semantik (baseline=None/Some) mit zusätzlicher
/// vollständiger Blobprüfung vor der finalen Refprüfung. Beide Quellen müssen
/// denselben Kontext verwenden; Roots stammen aus geprüften Originalcommits.
/// Die alten flatten-Traversalzähler bleiben separat, REST/Rawbytes gemeinsam.
#[allow(clippy::too_many_arguments)]
pub fn verify_complete_readback<S: ResponseSource, R: RawCommitSource>(
    reader: &mut RestReader<BudgetedResponses<'_, S>>,
    raw: &mut BudgetedCommits<'_, R>,
    repository_id: &str,
    before: Oid,
    after: Oid,
    expected: &crate::status::PreparedStatus,
    baseline: Option<&[u8]>,
    budget: &RunBudget,
) -> BridgeResult<CompleteReadback> {
    // Nur die vollständige Readback-Sequenz exportieren. Bereits belastete
    // Laufzähler bleiben erhalten; dies ist ausdrücklich kein Budgetneustart.
    let transcript_start = budget.0.borrow().transcript.len();
    let result = (|| {
        budget.check()?;
        if !std::ptr::eq(reader.source_ref().budget, budget) || !std::ptr::eq(raw.budget, budget) {
            return Err(BridgeError("RUN_BUDGET_CONTEXT_MISMATCH"));
        }
        if reader.repository()? != repository_id || repository_id.trim().is_empty() {
            return Err(BridgeError("REPOSITORY_ID_MISMATCH"));
        }
        if reader.head()? != after {
            return Err(BridgeError("INITIAL_HEAD_MISMATCH"));
        }
        let (blob, roots) = crate::protocol::verify_transition_objects(
            reader, raw, before, after, expected, baseline,
        )?;
        let complete = verify_complete_blobs(reader, &roots, budget)?;
        if reader.head()? != after {
            return Err(BridgeError("FINAL_HEAD_MISMATCH"));
        }
        let evidence = crate::object_evidence::OfflineObjectEvidence::from_verified(
            repository_id,
            before,
            after,
            &raw.verified,
            &complete.trees,
            &complete.blobs,
        )?;
        let package = crate::offline_package::OfflinePackage::from_readback(
            repository_id,
            before,
            after,
            expected.bytes(),
            baseline,
            &budget.0.borrow().transcript[transcript_start..],
            &raw.verified,
            &evidence,
        )?;
        Ok(CompleteReadback {
            report: crate::protocol::ReadbackReport {
                commit: after,
                blob,
                commit_hash_verified: true,
                publication_performed: false,
                scope: "OFFLINE_SUPPLIED_RESPONSES",
            },
            complete,
            evidence,
            package,
        })
    })();
    budget.fail(result)
}

/// Alle referenzierten Blobs aus ein/zwei vollständig geprüften Trees.
/// Die Quellenhülle muss denselben Kontext verwenden; damit belasten auch
/// Vor-/Nachprüfungen und Readerwechsel das gemeinsame REST-Budget.
/// Symlinks werden ausschließlich als Bytes gelesen, Gitlinks abgewiesen.
pub fn verify_complete_blobs<S: ResponseSource>(
    reader: &mut RestReader<BudgetedResponses<'_, S>>,
    roots: &[Oid],
    budget: &RunBudget,
) -> BridgeResult<CompleteBlobs> {
    struct Walk {
        active: BTreeSet<Oid>,
        evidence: CompleteBlobs,
    }
    fn walk<S: ResponseSource>(
        reader: &mut RestReader<S>,
        oid: Oid,
        prefix: &str,
        depth: usize,
        budget: &RunBudget,
        w: &mut Walk,
    ) -> BridgeResult<()> {
        budget.check()?;
        let (remaining_entries, remaining_bytes) = {
            let mut u = budget.0.borrow_mut();
            if depth > 128 || u.trees >= 4096 {
                return Err(BridgeError("TREE_LIMIT_EXCEEDED"));
            }
            u.trees += 1;
            (100_000 - u.entries, 32 * MIB - u.tree_bytes)
        };
        if !w.active.insert(oid) {
            return Err(BridgeError("CYCLIC_TREE"));
        }
        let record = reader.tree_bounded(oid, remaining_entries, remaining_bytes)?;
        // tree_bounded prüft Zuteilungsgrenzen, tree_bytes prüft Namen,
        // Duplikate und kanonische Git-Sortierung über den bestehenden Pfad.
        let bytes = tree_bytes(&record.entries)?;
        if record.truncated || record.oid != oid {
            return Err(BridgeError("INCOMPLETE_OR_WRONG_TREE"));
        }
        if object_oid(ObjectKind::Tree, &bytes) != oid {
            return Err(BridgeError("TREE_HASH_MISMATCH"));
        }
        {
            let mut u = budget.0.borrow_mut();
            u.entries += record.entries.len();
            u.tree_bytes += bytes.len();
        }
        w.evidence.trees.insert(
            oid,
            BlobEvidence {
                length: bytes.len(),
                payload_sha256: Sha256::digest(&bytes).into(),
            },
        );
        drop(bytes);
        for entry in record.entries {
            if prefix.len() + entry.name.len() > 4096 {
                return Err(BridgeError("TREE_PATH_TOO_LONG"));
            }
            match entry.mode {
                Mode::Directory => {
                    if prefix.len() + entry.name.len() + 1 > 4096 {
                        return Err(BridgeError("TREE_PATH_TOO_LONG"));
                    }
                    let path = format!("{prefix}{}/", entry.name);
                    walk(reader, entry.oid, &path, depth + 1, budget, w)?;
                }
                Mode::Gitlink => return Err(BridgeError("GITLINK_UNSUPPORTED")),
                Mode::File | Mode::Executable | Mode::Symlink => {
                    w.evidence.paths += 1;
                    let inventory_full = w.evidence.blobs.len() >= 256;
                    if let std::collections::btree_map::Entry::Vacant(slot) =
                        w.evidence.blobs.entry(entry.oid)
                    {
                        // Vor der Inventarzuteilung zusätzlich begrenzen;
                        // maximal 256 eindeutige Blobnachweise.
                        if inventory_full {
                            return Err(BridgeError("RUN_BUDGET_EXCEEDED"));
                        }
                        let remaining = 32 * MIB - budget.0.borrow().blob_bytes;
                        let body = reader.blob_bounded(entry.oid, MIB.min(remaining))?;
                        budget.0.borrow_mut().blob_bytes += body.len();
                        slot.insert(BlobEvidence {
                            length: body.len(),
                            payload_sha256: Sha256::digest(&body).into(),
                        });
                    }
                }
            }
        }
        w.active.remove(&oid);
        Ok(())
    }
    let result = (|| {
        budget.check()?;
        if !std::ptr::eq(reader.source_ref().budget, budget) {
            return Err(BridgeError("RUN_BUDGET_CONTEXT_MISMATCH"));
        }
        if roots.is_empty() || roots.len() > 2 {
            return Err(BridgeError("INVALID_ROOT_INVENTORY"));
        }
        let mut w = Walk {
            active: BTreeSet::new(),
            evidence: CompleteBlobs {
                roots: roots.to_vec(),
                blobs: BTreeMap::new(),
                paths: 0,
                trees: BTreeMap::new(),
            },
        };
        for root in roots {
            walk(reader, *root, "", 0, budget, &mut w)?;
        }
        Ok(w.evidence)
    })();
    budget.fail(result)
}
