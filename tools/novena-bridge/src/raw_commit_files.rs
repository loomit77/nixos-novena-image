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
