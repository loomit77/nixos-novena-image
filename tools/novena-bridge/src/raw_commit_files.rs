//! Explizite lokale Originaldateien, kein Git-Objektspeicheradapter.
use crate::objects::{inspect_commit, object_oid, ObjectKind, Oid};
use crate::protocol::RawCommitSource;
use crate::{BridgeError, BridgeResult};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

pub const MAX_RAW_COMMIT_BYTES: usize = 1024 * 1024;

/// Liest ausschließlich `<Verzeichnis>/<40-stellige-kleine-SHA1>` ohne Suffix.
/// Dateien enthalten originale Commitbytes ohne Git-Objektheader oder JSON.
///
/// Sicherheitsvoraussetzung: Verzeichnis, Dateien und sämtliche Elternpfade
/// sind vertrauenswürdig, privat kontrolliert und während Konstruktion und
/// Nutzung unveränderlich (auch keine Schreibzugriffe über Hardlinks/Mounts).
/// Der Aufrufer muss dies extern gewährleisten. Metadatenprüfungen sind kein
/// atomarer Schutz gegen TOCTOU, Symlink- oder Mountaustausch. Für feindlich
/// veränderbare Dateisysteme ist diese Quelle ausdrücklich ungeeignet.
/// Reguläre lokale Dateien vorausgesetzt; keine harte I/O-Deadline zugesagt.
/// Kein Default, keine Discovery, Expansion, canonicalize oder Schreibaktion.
pub struct RawCommitFiles {
    directory: PathBuf,
}

impl RawCommitFiles {
    /// Explizite Bestätigung des oben beschriebenen Vertrauensvertrags.
    /// Absolute Pfade ohne `..` sind erforderlich; Symlinks werden abgewiesen.
    pub fn from_trusted_immutable_directory(directory: &Path) -> BridgeResult<Self> {
        check_directory(directory)?;
        Ok(Self {
            directory: directory.to_owned(),
        })
    }

    /// Klassifikation, keine GitHub-Herkunfts- oder Repositoryattestation.
    pub fn provenance(&self) -> &'static str {
        "LOCAL_EXPLICIT_RAW_COMMIT_FILES"
    }

    /// Texteingang nutzt ausschließlich den vorhandenen strikten OID-Parser.
    pub fn read_oid(&mut self, requested: &str) -> BridgeResult<Vec<u8>> {
        self.raw_commit(Oid::parse(requested)?)
    }
}

fn io_error(error: std::io::Error) -> BridgeError {
    if error.kind() == std::io::ErrorKind::NotFound {
        BridgeError("RAW_COMMIT_PATH_MISSING")
    } else {
        BridgeError("RAW_COMMIT_IO_ERROR")
    }
}

fn check_directory(directory: &Path) -> BridgeResult<()> {
    if !directory.is_absolute() {
        return Err(BridgeError("RAW_COMMIT_ABSOLUTE_DIRECTORY_REQUIRED"));
    }
    let mut prefix = PathBuf::new();
    for component in directory.components() {
        if !matches!(component, Component::RootDir | Component::Normal(_)) {
            return Err(BridgeError("RAW_COMMIT_INVALID_DIRECTORY"));
        }
        prefix.push(component.as_os_str());
        // Einschließlich aller Eltern: keine Symlinkauflösung akzeptieren.
        let meta = fs::symlink_metadata(&prefix).map_err(io_error)?;
        if !meta.file_type().is_dir() {
            return Err(BridgeError("RAW_COMMIT_DIRECTORY_TYPE_REJECTED"));
        }
    }
    Ok(())
}

impl RawCommitSource for RawCommitFiles {
    fn raw_commit(&mut self, id: Oid) -> BridgeResult<Vec<u8>> {
        // Oid ist privat konstruiert; Display liefert genau 40 kleine Hexziffern.
        check_directory(&self.directory)?;
        let path = self.directory.join(id.to_string());
        let before = fs::symlink_metadata(&path).map_err(io_error)?;
        if !before.file_type().is_file() {
            return Err(BridgeError("RAW_COMMIT_FILE_TYPE_REJECTED"));
        }
        if before.len() > MAX_RAW_COMMIT_BYTES as u64 {
            return Err(BridgeError("RAW_COMMIT_TOO_LARGE"));
        }
        // Kein Race-Schutz behauptet: zwischen Vorprüfung und open müssen die
        // extern garantierten unveränderlichen Pfade unverändert bleiben.
        let file = File::open(&path).map_err(io_error)?;
        let opened = file.metadata().map_err(io_error)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if before.dev() != opened.dev() || before.ino() != opened.ino() {
                return Err(BridgeError("RAW_COMMIT_FILE_CHANGED"));
            }
        }
        if !opened.is_file() {
            return Err(BridgeError("RAW_COMMIT_FILE_TYPE_REJECTED"));
        }
        if opened.len() > MAX_RAW_COMMIT_BYTES as u64 {
            return Err(BridgeError("RAW_COMMIT_TOO_LARGE"));
        }
        let mut bytes = Vec::new();
        file.take(MAX_RAW_COMMIT_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(io_error)?;
        if bytes.len() > MAX_RAW_COMMIT_BYTES {
            return Err(BridgeError("RAW_COMMIT_TOO_LARGE"));
        }
        if before.len() != opened.len() || bytes.len() as u64 != opened.len() {
            return Err(BridgeError("RAW_COMMIT_SIZE_CHANGED"));
        }
        inspect_commit(&bytes)?;
        if object_oid(ObjectKind::Commit, &bytes) != id {
            return Err(BridgeError("COMMIT_HASH_MISMATCH"));
        }
        // Erst nach vollständigem Lesen und allen Prüfungen Bytes herausgeben.
        Ok(bytes)
    }
}

/// Deskriptorverankerte Linux-Übergabe, genau zwei Dateien ohne Manifestpfade.
/// Einmal gelesene Bytes werden gehasht und an den bestehenden Replay gegeben.
/// Keine atomare Verzeichnisaufnahme oder Garantie gegen Mountadministration.
pub(crate) fn read_handoff(
    directory: &Path,
    claims: &[crate::offline_readback::ExportClaim],
) -> BridgeResult<std::collections::BTreeMap<Oid, Vec<u8>>> {
    read_handoff_observed(directory, claims, |_, _| Ok(()))
}

// Deterministische Testeingriffe an tatsächlichen Deskriptorgrenzen, kein Threadrace.
fn read_handoff_observed(
    directory: &Path,
    claims: &[crate::offline_readback::ExportClaim],
    mut observe: impl FnMut(&str, &Path) -> BridgeResult<()>,
) -> BridgeResult<std::collections::BTreeMap<Oid, Vec<u8>>> {
    if claims.len() != 2
        || claims[0].oid >= claims[1].oid
        || claims.iter().any(|c| c.length > MAX_RAW_COMMIT_BYTES)
        || !directory.is_absolute()
        || directory
            .components()
            .any(|c| !matches!(c, Component::RootDir | Component::Normal(_)))
    {
        return Err(BridgeError("RAW_HANDOFF_SELECTION"));
    }
    #[cfg(not(target_os = "linux"))]
    return Err(BridgeError("RAW_HANDOFF_REQUIRES_LINUX"));
    #[cfg(target_os = "linux")]
    {
        use std::os::fd::AsRawFd;
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        // O_NOFOLLOW/O_NONBLOCK wie im lokalen Quellenadapter; zusätzlich
        // O_DIRECTORY für Eltern. Nur eigene Kernelreferenzen via procfs.
        fn open(path: &Path, directory: bool) -> BridgeResult<File> {
            fs::OpenOptions::new()
                .read(true)
                .custom_flags(0x20000 | 0x800 | if directory { 0x10000 } else { 0 })
                .open(path)
                .map_err(io_error)
        }
        fn anchor(file: &File) -> PathBuf {
            PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()))
        }
        let mut dir = open(Path::new("/"), true)?;
        for component in directory.components() {
            if let Component::Normal(name) = component {
                dir = open(&anchor(&dir).join(name), true)?;
            }
        }
        observe("anchored", &anchor(&dir))?;
        let expected: std::collections::BTreeSet<_> = claims
            .iter()
            .map(|c| std::ffi::OsString::from(c.oid.to_string()))
            .collect();
        let inventory = || -> BridgeResult<()> {
            let mut actual = std::collections::BTreeSet::new();
            for entry in fs::read_dir(anchor(&dir)).map_err(io_error)? {
                let name = entry.map_err(io_error)?.file_name();
                if !expected.contains(&name) || !actual.insert(name) {
                    return Err(BridgeError("RAW_HANDOFF_CONTENTS"));
                }
            }
            if actual != expected {
                return Err(BridgeError("RAW_HANDOFF_CONTENTS"));
            }
            Ok(())
        };
        inventory()?;
        let mut result = std::collections::BTreeMap::new();
        for claim in claims {
            observe("before_open", &anchor(&dir).join(claim.oid.to_string()))?;
            let file = open(&anchor(&dir).join(claim.oid.to_string()), false)?;
            let meta = file.metadata().map_err(io_error)?;
            if !meta.is_file() || meta.nlink() != 1 || meta.len() != claim.length as u64 {
                return Err(BridgeError("RAW_HANDOFF_FILE"));
            }
            observe("opened", &anchor(&file))?;
            let mut bytes = Vec::new();
            (&file)
                .take(claim.length as u64 + 1)
                .read_to_end(&mut bytes)
                .map_err(io_error)?;
            observe("read", &anchor(&file))?;
            let after = file.metadata().map_err(io_error)?;
            if after.nlink() != 1 || after.len() != meta.len() {
                return Err(BridgeError("RAW_HANDOFF_FILE"));
            }
            use sha2::{Digest, Sha256};
            if bytes.len() != claim.length
                || <[u8; 32]>::from(Sha256::digest(&bytes)) != claim.payload_sha256
            {
                return Err(BridgeError("RAW_HANDOFF_DIGEST"));
            }
            inspect_commit(&bytes)?;
            if object_oid(ObjectKind::Commit, &bytes) != claim.oid {
                return Err(BridgeError("COMMIT_HASH_MISMATCH"));
            }
            result.insert(claim.oid, bytes);
        }
        inventory()?;
        Ok(result)
    }
}

#[cfg(all(test, target_os = "linux"))]
#[path = "handoff_race_tests.rs"]
mod race_tests;
