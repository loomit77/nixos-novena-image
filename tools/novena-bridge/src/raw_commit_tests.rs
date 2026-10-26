//! Vertragstests mit Speicher- und temporären Dateifixtures, kein Git-Speicheradapter.
use super::*;
use crate::raw_commit_files::{RawCommitFiles, MAX_RAW_COMMIT_BYTES};
use std::{fs, path::PathBuf};

// Alle Schreibzugriffe dieser Dateitests bleiben in einem eigenen Tempbaum.
struct RawFilesFixture(PathBuf);
impl RawFilesFixture {
    fn new() -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "novena-raw-commit-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        // Keine Wiederverwendung oder Bereinigung bereits vorhandener Pfade.
        builder.create(&path).unwrap();
        Self(path)
    }
    fn put(&self, id: Oid, bytes: &[u8]) {
        fs::write(self.0.join(id.to_string()), bytes).unwrap();
    }
    fn source(&self) -> RawCommitFiles {
        RawCommitFiles::from_trusted_immutable_directory(&self.0).unwrap()
    }
}
impl Drop for RawFilesFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn check_file(source: &mut RawCommitFiles, id: Oid) -> BridgeResult<CommitInfo> {
    let meta = serde_json::to_vec(&json!({
        "sha": id.to_string(), "tree": {"sha": "a".repeat(40)}, "parents": []
    }))
    .unwrap();
    let mut reader = RestReader::new(|_: &str| Ok(meta.clone()), 1).unwrap();
    check_commit(&mut reader, source, id)
}

#[test]
fn raw_files_preserve_original_bytes_and_verify_commit_identity() {
    let f = RawFilesFixture::new();
    for message in [
        b"original\xff\xfe\n\n".as_slice(),
        b"no final newline",
        b"one\n",
    ] {
        let bytes = fixture(message);
        let id = object_oid(ObjectKind::Commit, &bytes);
        f.put(id, &bytes);
        let mut source = f.source();
        assert_eq!(source.read_oid(&id.to_string()).unwrap(), bytes);
        assert_eq!(
            check_file(&mut source, id).unwrap().message_size,
            message.len()
        );
        assert_eq!(source.provenance(), "LOCAL_EXPLICIT_RAW_COMMIT_FILES");
        assert_eq!(fs::read(f.0.join(id.to_string())).unwrap(), bytes);
    }
    assert_eq!(
        production_publish(),
        Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
    );
}

#[test]
fn raw_files_reject_added_or_missing_newline_and_wrong_hash_domain() {
    let f = RawFilesFixture::new();
    let bytes = fixture(b"original\n");
    let id = object_oid(ObjectKind::Commit, &bytes);
    for altered in [
        fixture(b"original"),
        fixture(b"original\n\n"),
        fixture(b"wrong\n"),
    ] {
        f.put(id, &altered);
        assert_eq!(
            check_file(&mut f.source(), id),
            Err(BridgeError("COMMIT_HASH_MISMATCH"))
        );
    }
    for kind in [ObjectKind::Blob, ObjectKind::Tree] {
        let other = object_oid(kind, &bytes);
        f.put(other, &bytes);
        assert_eq!(
            f.source().raw_commit(other),
            Err(BridgeError("COMMIT_HASH_MISMATCH"))
        );
    }
}

#[test]
fn raw_files_invalid_oids_fail_before_path_access() {
    let f = RawFilesFixture::new();
    let mut source = f.source();
    // Nach Konstruktion entfernen: ungültige OIDs müssen vor Dateizugriff scheitern.
    fs::remove_dir(&f.0).unwrap();
    for invalid in [
        "".into(),
        "HEAD".into(),
        "../file".into(),
        "--help".into(),
        "a".repeat(39),
        "a".repeat(41),
        "A".repeat(40),
        "é".repeat(20),
        format!("{}\n", "a".repeat(40)),
        format!("{}\0", "a".repeat(40)),
        format!("{}^{{commit}}", "a".repeat(40)),
    ] {
        assert_eq!(
            source.read_oid(&invalid),
            Err(BridgeError("INVALID_OBJECT_ID"))
        );
    }
    fs::create_dir(&f.0).unwrap();
}

#[test]
fn raw_files_missing_and_malformed_files_return_only_errors() {
    let f = RawFilesFixture::new();
    let id = object_oid(ObjectKind::Commit, &fixture(b"original\n"));
    assert_eq!(
        check_file(&mut f.source(), id),
        Err(BridgeError("RAW_COMMIT_PATH_MISSING"))
    );
    for bytes in [
        fixture(b"nul\0byte"),
        fixture(b"cr\rbyte"),
        fixture(b""),
        b"commit 12\0not raw".to_vec(),
        b"{\"message\":\"REST\"}".to_vec(),
    ] {
        let id = object_oid(ObjectKind::Commit, &bytes);
        f.put(id, &bytes);
        assert!(check_file(&mut f.source(), id).is_err());
    }
}

#[test]
fn raw_files_no_filename_fallback_and_existing_metadata_check() {
    let f = RawFilesFixture::new();
    let bytes = fixture(b"original\n");
    let id = object_oid(ObjectKind::Commit, &bytes);
    fs::write(f.0.join(format!("{id}.commit")), &bytes).unwrap();
    assert_eq!(
        f.source().raw_commit(id),
        Err(BridgeError("RAW_COMMIT_PATH_MISSING"))
    );
    f.put(id, &bytes);
    let meta = serde_json::to_vec(&json!({
        "sha": id.to_string(), "tree": {"sha": "b".repeat(40)}, "parents": []
    }))
    .unwrap();
    let mut reader = RestReader::new(|_: &str| Ok(meta.clone()), 1).unwrap();
    assert_eq!(
        check_commit(&mut reader, &mut f.source(), id),
        Err(BridgeError("COMMIT_METADATA_MISMATCH"))
    );
    assert_eq!(
        production_publish(),
        Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
    );
}

#[test]
fn raw_files_exact_size_limit_and_overflow() {
    let f = RawFilesFixture::new();
    let mut bytes = fixture(b"x");
    bytes.resize(MAX_RAW_COMMIT_BYTES, b'x');
    let id = object_oid(ObjectKind::Commit, &bytes);
    f.put(id, &bytes);
    assert_eq!(f.source().raw_commit(id).unwrap(), bytes);
    bytes.push(b'x');
    let oversized = object_oid(ObjectKind::Commit, &bytes);
    f.put(oversized, &bytes);
    assert_eq!(
        check_file(&mut f.source(), oversized),
        Err(BridgeError("RAW_COMMIT_TOO_LARGE"))
    );
}

#[test]
fn raw_files_require_explicit_directory_and_regular_file() {
    let f = RawFilesFixture::new();
    assert!(RawCommitFiles::from_trusted_immutable_directory(std::path::Path::new(".")).is_err());
    assert!(RawCommitFiles::from_trusted_immutable_directory(&f.0.join("..")).is_err());
    assert!(RawCommitFiles::from_trusted_immutable_directory(&f.0.join("missing")).is_err());
    let id = object_oid(ObjectKind::Commit, &fixture(b"x"));
    let path = f.0.join(id.to_string());
    fs::create_dir(&path).unwrap();
    assert_eq!(
        check_file(&mut f.source(), id),
        Err(BridgeError("RAW_COMMIT_FILE_TYPE_REJECTED"))
    );
    fs::remove_dir(&path).unwrap();
    f.put(id, &fixture(b"x"));
    assert!(RawCommitFiles::from_trusted_immutable_directory(&path).is_err());
}

#[cfg(unix)]
#[test]
fn raw_files_reject_file_and_directory_symlinks_including_parent_paths() {
    use std::os::unix::fs::symlink;
    let f = RawFilesFixture::new();
    let bytes = fixture(b"x");
    let id = object_oid(ObjectKind::Commit, &bytes);
    fs::write(f.0.join("original"), &bytes).unwrap();
    symlink("original", f.0.join(id.to_string())).unwrap();
    assert_eq!(
        check_file(&mut f.source(), id),
        Err(BridgeError("RAW_COMMIT_FILE_TYPE_REJECTED"))
    );
    let real = f.0.join("real");
    fs::create_dir(&real).unwrap();
    fs::create_dir(real.join("nested")).unwrap();
    symlink(&real, f.0.join("alias")).unwrap();
    for path in [f.0.join("alias"), f.0.join("alias/nested")] {
        assert!(RawCommitFiles::from_trusted_immutable_directory(&path).is_err());
    }
    // Auch nach Konstruktion wird eine statisch ausgetauschte Auswahl abgewiesen.
    let mut source =
        RawCommitFiles::from_trusted_immutable_directory(&real.join("nested")).unwrap();
    fs::remove_dir(real.join("nested")).unwrap();
    symlink(&f.0, real.join("nested")).unwrap();
    assert_eq!(
        source.raw_commit(id),
        Err(BridgeError("RAW_COMMIT_DIRECTORY_TYPE_REJECTED"))
    );
}

#[cfg(unix)]
#[test]
fn raw_files_reject_socket_without_opening_it() {
    let f = RawFilesFixture::new();
    let id = object_oid(ObjectKind::Commit, &fixture(b"x"));
    let _socket = std::os::unix::net::UnixListener::bind(f.0.join(id.to_string())).unwrap();
    assert_eq!(
        check_file(&mut f.source(), id),
        Err(BridgeError("RAW_COMMIT_FILE_TYPE_REJECTED"))
    );
}

fn fixture(message: &[u8]) -> Vec<u8> {
    let mut raw = format!(
        "tree {}\nauthor Test <test@localhost.invalid> 1 +0000\ncommitter Test <test@localhost.invalid> 1 +0000\n\n",
        "a".repeat(40)
    )
    .into_bytes();
    raw.extend_from_slice(message);
    raw
}

fn check(raw: Vec<u8>, id: Oid) -> BridgeResult<CommitInfo> {
    let meta = serde_json::to_vec(&json!({
        "sha": id.to_string(),
        "tree": {"sha": "a".repeat(40)},
        "parents": []
    }))
    .unwrap();
    let path = format!("{PREFIX}/git/commits/{id}");
    let mut reader = RestReader::new(
        |requested: &str| {
            assert_eq!(requested, path);
            Ok(meta.clone())
        },
        1,
    )
    .unwrap();
    let mut source = |requested: Oid| {
        assert_eq!(requested, id);
        Ok(raw.clone())
    };
    check_commit(&mut reader, &mut source, id)
}

#[test]
fn raw_contract_preserves_non_utf8_and_final_newlines() {
    for message in [b"original\xff\xfe\n\n".as_slice(), b"no final newline"] {
        let raw = fixture(message);
        let id = object_oid(ObjectKind::Commit, &raw);
        let mut source = |requested: Oid| {
            assert_eq!(requested, id);
            Ok(raw.clone())
        };
        assert_eq!(RawCommitSource::raw_commit(&mut source, id).unwrap(), raw);
        assert_eq!(check(raw, id).unwrap().message_size, message.len());
    }
}

#[test]
fn raw_contract_rejects_wrong_oid_and_changed_original_bytes() {
    let raw = fixture(b"original\n");
    let id = object_oid(ObjectKind::Commit, &raw);
    let wrong = object_oid(ObjectKind::Commit, &fixture(b"different\n"));
    assert_eq!(
        check(raw.clone(), wrong),
        Err(BridgeError("COMMIT_HASH_MISMATCH"))
    );
    let mut changed = raw;
    changed.pop();
    assert_eq!(check(changed, id), Err(BridgeError("COMMIT_HASH_MISMATCH")));
}

#[test]
fn raw_contract_rejects_other_object_hash_domains() {
    // Keine Qualifikation von Git-Typabfragen: geprüft wird die vorhandene
    // unabhängige Commit-Hashprüfung bei gültig aussehenden Commitbytes.
    let raw = fixture(b"original\n");
    for kind in [ObjectKind::Blob, ObjectKind::Tree] {
        let id = object_oid(kind, &raw);
        assert_eq!(
            check(raw.clone(), id),
            Err(BridgeError("COMMIT_HASH_MISMATCH"))
        );
    }
}

#[test]
fn raw_contract_rejects_revision_expressions_before_source_access() {
    let mut calls = 0;
    {
        let mut source = |_: Oid| {
            calls += 1;
            Err(BridgeError("UNEXPECTED_SOURCE_ACCESS"))
        };
        for text in [
            String::new(),
            "HEAD".into(),
            "refs/heads/main".into(),
            "a".repeat(39),
            "a".repeat(41),
            "A".repeat(40),
            format!("{}^{{commit}}", "a".repeat(40)),
            format!("{}\n", "a".repeat(40)),
            format!("{}\0", "a".repeat(40)),
            "é".repeat(20),
            "--help".into(),
        ] {
            let result =
                Oid::parse(&text).and_then(|id| RawCommitSource::raw_commit(&mut source, id));
            assert_eq!(result, Err(BridgeError("INVALID_OBJECT_ID")));
        }
    }
    assert_eq!(calls, 0);
}

#[test]
fn raw_contract_rejects_malformed_bytes_even_with_matching_hash() {
    for raw in [
        fixture(b"carriage\rreturn\n"),
        fixture(b"nul\0byte\n"),
        fixture(b""),
        b"not a commit\n".to_vec(),
    ] {
        let id = object_oid(ObjectKind::Commit, &raw);
        assert!(check(raw, id).is_err());
    }
}

#[test]
fn raw_contract_commit_size_boundary_is_one_mib() {
    // Nachgelagerte Parsergrenze, kein Beleg einer begrenzten Beschaffung.
    let mut raw = fixture(b"x");
    raw.resize(1024 * 1024, b'x');
    let id = object_oid(ObjectKind::Commit, &raw);
    assert!(check(raw.clone(), id).is_ok());
    raw.push(b'x');
    let id = object_oid(ObjectKind::Commit, &raw);
    assert_eq!(check(raw, id), Err(BridgeError("UNSUPPORTED_COMMIT_BYTES")));
}

#[test]
fn raw_contract_missing_object_propagates_without_success() {
    let id = object_oid(ObjectKind::Commit, &fixture(b"original\n"));
    let meta = serde_json::to_vec(&json!({
        "sha": id.to_string(), "tree": {"sha": "a".repeat(40)}, "parents": []
    }))
    .unwrap();
    let mut reader = RestReader::new(|_: &str| Ok(meta.clone()), 1).unwrap();
    let mut source = |_: Oid| Err(BridgeError("MISSING_RAW_COMMIT"));
    assert_eq!(
        check_commit(&mut reader, &mut source, id),
        Err(BridgeError("MISSING_RAW_COMMIT"))
    );
}

#[test]
fn raw_contract_success_does_not_unlock_publication() {
    let raw = fixture(b"original\n");
    let id = object_oid(ObjectKind::Commit, &raw);
    assert!(check(raw, id).is_ok());
    assert_eq!(
        production_publish(),
        Err(BridgeError("PRODUCTION_PUBLICATION_DISABLED"))
    );
}
