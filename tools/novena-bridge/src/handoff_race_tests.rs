//! Reale Operationen, feste Reihenfolge statt Annahmen über den Scheduler.
use super::*;
use crate::offline_readback::ExportClaim;
use sha2::{Digest, Sha256};
use std::os::unix::fs::symlink;
use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture {
    root: PathBuf,
    dir: PathBuf,
    claims: Vec<ExportClaim>,
    bytes: Vec<Vec<u8>>,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "novena-race-300-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let dir = root.join("parent/commits");
        fs::create_dir_all(&dir).unwrap();
        let mut pairs: Vec<_> = ["first", "other"].iter().map(|message| {
            let bytes = format!("tree {}\nauthor Test <test@localhost.invalid> 1 +0000\ncommitter Test <test@localhost.invalid> 1 +0000\n\n{message}\n", "a".repeat(40)).into_bytes();
            (object_oid(ObjectKind::Commit, &bytes), bytes)
        }).collect();
        pairs.sort_by_key(|p| p.0);
        let claims = pairs
            .iter()
            .map(|(oid, bytes)| {
                fs::write(dir.join(oid.to_string()), bytes).unwrap();
                ExportClaim {
                    oid: *oid,
                    length: bytes.len(),
                    payload_sha256: Sha256::digest(bytes).into(),
                }
            })
            .collect();
        Self {
            root,
            dir,
            claims,
            bytes: pairs.into_iter().map(|p| p.1).collect(),
        }
    }
    fn file(&self) -> PathBuf {
        self.dir.join(self.claims[0].oid.to_string())
    }
    fn changed(&self) -> Vec<u8> {
        let mut b = self.bytes[0].clone();
        *b.last_mut().unwrap() = b'x';
        b
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn exchanges_before_open_reject_changed_bytes_links_and_missing_inventory() {
    for case in 0..8 {
        let f = Fixture::new();
        let mut once = false;
        let result = read_handoff_observed(&f.dir, &f.claims, |phase, path| {
            if phase == "before_open" && !once {
                once = true;
                match case {
                    0 => {
                        // Anderer Inode, gleiche Länge.
                        let replacement = f.root.join("replacement");
                        fs::write(&replacement, f.changed()).unwrap();
                        fs::rename(replacement, path).unwrap();
                    }
                    1 => fs::write(path, f.changed()).unwrap(), // Gleicher Inode.
                    2 => {
                        fs::remove_file(path).unwrap();
                        symlink(f.root.join("missing"), path).unwrap();
                    }
                    3 => fs::remove_file(path).unwrap(),
                    4 => {
                        fs::hard_link(path, f.root.join("alias")).unwrap();
                    }
                    5 => {
                        fs::write(f.dir.join("unexpected"), b"x").unwrap();
                    }
                    6 => {
                        fs::remove_file(f.dir.join(f.claims[1].oid.to_string())).unwrap();
                    }
                    7 => {
                        fs::remove_file(path).unwrap();
                        fs::create_dir(path).unwrap();
                    }
                    _ => unreachable!(),
                }
            }
            Ok(())
        });
        assert!(once);
        assert!(result.is_err(), "case {case}");
    }
}

#[test]
fn anchored_parent_exchange_cannot_redirect_subsequent_opens() {
    for symlink_replacement in [false, true] {
        let f = Fixture::new();
        let old_parent = f.root.join("original-parent");
        let result = read_handoff_observed(&f.dir, &f.claims, |phase, _| {
            if phase == "anchored" {
                fs::rename(f.root.join("parent"), &old_parent).unwrap();
                if symlink_replacement {
                    symlink(&old_parent, f.root.join("parent")).unwrap();
                } else {
                    fs::create_dir_all(&f.dir).unwrap();
                    fs::write(f.file(), f.changed()).unwrap();
                }
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(result[&f.claims[0].oid], f.bytes[0]);
        assert_eq!(result[&f.claims[1].oid], f.bytes[1]);
        if symlink_replacement {
            assert!(read_handoff(&f.dir, &f.claims).is_err());
        }
    }
}

#[test]
fn opened_descriptor_uses_original_inode_but_checks_the_actual_bytes() {
    for case in 0..5 {
        let f = Fixture::new();
        let mut once = false;
        let result = read_handoff_observed(&f.dir, &f.claims, |phase, descriptor| {
            if phase == "opened" && !once {
                once = true;
                match case {
                    0 => {
                        fs::rename(f.file(), f.root.join("original")).unwrap();
                        fs::write(f.file(), f.changed()).unwrap();
                    }
                    1 => fs::write(descriptor, f.changed()).unwrap(),
                    2 => fs::remove_file(f.file()).unwrap(),
                    3 => {
                        fs::hard_link(f.file(), f.root.join("alias")).unwrap();
                    }
                    4 => {
                        fs::rename(f.file(), f.root.join("original")).unwrap();
                        symlink(f.root.join("missing"), f.file()).unwrap();
                    }
                    _ => unreachable!(),
                }
            }
            Ok(())
        });
        assert!(once);
        if case == 0 || case == 4 {
            // Garantiert sind gelesene Originalbytes, keine dauerhafte Bindung
            // des Namens an den Inode und kein atomarer Verzeichnissnapshot.
            assert_eq!(result.unwrap()[&f.claims[0].oid], f.bytes[0]);
        } else {
            assert!(result.is_err(), "case {case}");
        }
    }
}

#[test]
fn digest_and_oid_remain_independent_and_replays_reopen_files() {
    let mut f = Fixture::new();
    let first = read_handoff(&f.dir, &f.claims).unwrap();
    assert_eq!(first, read_handoff(&f.dir, &f.claims).unwrap());
    f.claims[0].payload_sha256[0] ^= 1;
    assert_eq!(
        read_handoff(&f.dir, &f.claims).unwrap_err(),
        BridgeError("RAW_HANDOFF_DIGEST")
    );
    let changed = f.changed();
    fs::write(f.file(), &changed).unwrap();
    f.claims[0].payload_sha256 = Sha256::digest(&changed).into();
    assert_eq!(
        read_handoff(&f.dir, &f.claims).unwrap_err(),
        BridgeError("COMMIT_HASH_MISMATCH")
    );
    assert_eq!(first[&f.claims[0].oid], f.bytes[0]);
}
