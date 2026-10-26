//! Pure SHA-1 Git object verification. No Git subprocesses, writes or network.
use crate::{BridgeError, BridgeResult};
use sha1::{Digest, Sha1};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oid([u8; 20]);
impl Oid {
    pub fn parse(text: &str) -> BridgeResult<Self> {
        if text.len() != 40
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(BridgeError("INVALID_OBJECT_ID"));
        }
        let mut bytes = [0; 20];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex checked");
        }
        Ok(Self(bytes))
    }
    pub fn bytes(&self) -> &[u8; 20] {
        &self.0
    }
}
impl std::fmt::Display for Oid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
}
pub fn object_oid(kind: ObjectKind, body: &[u8]) -> Oid {
    let name = match kind {
        ObjectKind::Blob => "blob",
        ObjectKind::Tree => "tree",
        ObjectKind::Commit => "commit",
    };
    let mut h = Sha1::new();
    h.update(format!("{name} {}\0", body.len()).as_bytes());
    h.update(body);
    Oid(h.finalize().into())
}
pub fn verify_blob(body: &[u8], expected: Oid) -> BridgeResult<()> {
    if object_oid(ObjectKind::Blob, body) != expected {
        return Err(BridgeError("BLOB_HASH_MISMATCH"));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    File,
    Executable,
    Symlink,
    Directory,
    Gitlink,
}
impl Mode {
    pub fn from_api(kind: &str, mode: &str) -> BridgeResult<Self> {
        match (kind, mode) {
            ("blob", "100644") => Ok(Self::File),
            ("blob", "100755") => Ok(Self::Executable),
            ("blob", "120000") => Ok(Self::Symlink),
            ("tree", "040000") => Ok(Self::Directory),
            ("commit", "160000") => Ok(Self::Gitlink),
            _ => Err(BridgeError("INVALID_TYPE_OR_MODE")),
        }
    }
    fn raw(&self) -> &'static [u8] {
        match self {
            Self::File => b"100644",
            Self::Executable => b"100755",
            Self::Symlink => b"120000",
            Self::Directory => b"40000",
            Self::Gitlink => b"160000",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub mode: Mode,
    pub oid: Oid,
}
pub fn tree_bytes(entries: &[TreeEntry]) -> BridgeResult<Vec<u8>> {
    let mut names = BTreeSet::new();
    let mut sorted = Vec::new();
    for entry in entries {
        let name = &entry.name;
        if name.is_empty()
            || name == "."
            || name == ".."
            || name.contains(['/', '\0'])
            || !names.insert(name)
        {
            return Err(BridgeError("INVALID_OR_DUPLICATE_NAME"));
        }
        let mut key = name.as_bytes().to_vec();
        if entry.mode == Mode::Directory {
            key.push(b'/');
        }
        sorted.push((key, entry));
    }
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut body = Vec::new();
    for (_, entry) in sorted {
        body.extend(entry.mode.raw());
        body.push(b' ');
        body.extend(entry.name.as_bytes());
        body.push(0);
        body.extend(entry.oid.bytes());
    }
    Ok(body)
}
pub fn tree_oid(entries: &[TreeEntry]) -> BridgeResult<Oid> {
    Ok(object_oid(ObjectKind::Tree, &tree_bytes(entries)?))
}

#[derive(Debug)]
pub struct TreeRecord {
    pub oid: Oid,
    pub truncated: bool,
    pub entries: Vec<TreeEntry>,
}
pub trait TreeReader {
    fn tree(&mut self, oid: Oid) -> BridgeResult<TreeRecord>;
}
#[derive(Debug, PartialEq, Eq)]
pub struct FlatTree {
    pub files: BTreeMap<String, (Mode, Oid)>,
    pub directories: BTreeSet<String>,
}

pub fn flatten_verified(reader: &mut impl TreeReader, root: Oid) -> BridgeResult<FlatTree> {
    struct Walk {
        active: BTreeSet<Oid>,
        count: usize,
        bytes: usize,
        flat: FlatTree,
    }
    fn walk(
        reader: &mut impl TreeReader,
        oid: Oid,
        prefix: &str,
        depth: usize,
        w: &mut Walk,
    ) -> BridgeResult<()> {
        if depth > 128 || w.count >= 4096 || w.flat.files.len() + w.flat.directories.len() > 100_000
        {
            return Err(BridgeError("TREE_LIMIT_EXCEEDED"));
        }
        if !w.active.insert(oid) {
            return Err(BridgeError("CYCLIC_TREE"));
        }
        w.count += 1;
        let record = reader.tree(oid)?;
        if record.oid != oid || record.truncated {
            return Err(BridgeError("INCOMPLETE_OR_WRONG_TREE"));
        }
        let bytes = tree_bytes(&record.entries)?;
        w.bytes += bytes.len();
        if w.bytes > 32 * 1024 * 1024 {
            return Err(BridgeError("TREE_BYTE_LIMIT_EXCEEDED"));
        }
        if object_oid(ObjectKind::Tree, &bytes) != oid {
            return Err(BridgeError("TREE_HASH_MISMATCH"));
        }
        for entry in record.entries {
            let path = format!("{prefix}{}", entry.name);
            if path.len() > 4096 {
                return Err(BridgeError("TREE_PATH_TOO_LONG"));
            }
            if entry.mode == Mode::Directory {
                if !w.flat.directories.insert(path.clone()) {
                    return Err(BridgeError("DUPLICATE_PATH"));
                }
                walk(reader, entry.oid, &format!("{path}/"), depth + 1, w)?;
            } else if w.flat.files.insert(path, (entry.mode, entry.oid)).is_some() {
                return Err(BridgeError("DUPLICATE_PATH"));
            }
        }
        w.active.remove(&oid);
        Ok(())
    }
    let mut w = Walk {
        active: BTreeSet::new(),
        count: 0,
        bytes: 0,
        flat: FlatTree {
            files: BTreeMap::new(),
            directories: BTreeSet::new(),
        },
    };
    walk(reader, root, "", 0, &mut w)?;
    Ok(w.flat)
}

/// Includes empty directories in the change policy, beyond the Python baseline.
pub fn verify_single_addition(
    old: &FlatTree,
    new: &FlatTree,
    target: &str,
    blob: Oid,
) -> BridgeResult<()> {
    if target.is_empty()
        || target
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == ".." || p.contains('\0'))
    {
        return Err(BridgeError("INVALID_TARGET"));
    }
    if old.files.contains_key(target) || old.directories.contains(target) {
        return Err(BridgeError("TARGET_ALREADY_EXISTS"));
    }
    if new.files.get(target) != Some(&(Mode::File, blob)) || new.files.len() != old.files.len() + 1
    {
        return Err(BridgeError("UNEXPECTED_ADDITIONS_OR_REMOVALS"));
    }
    for (path, entry) in &old.files {
        if new.files.get(path) != Some(entry) {
            return Err(BridgeError("UNEXPECTED_MODIFICATION"));
        }
    }
    let mut directories = old.directories.clone();
    let mut parent = target;
    while let Some((p, _)) = parent.rsplit_once('/') {
        directories.insert(p.to_owned());
        parent = p;
    }
    if new.directories != directories {
        return Err(BridgeError("UNEXPECTED_DIRECTORY_CHANGES"));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub tree: Oid,
    pub parents: Vec<Oid>,
    pub message_size: usize,
}
fn identity(line: &[u8]) -> bool {
    let Some((who, offset)) = line.rsplit_once_bytes(b' ') else {
        return false;
    };
    if offset.len() != 5
        || !matches!(offset[0], b'+' | b'-')
        || !offset[1..].iter().all(u8::is_ascii_digit)
    {
        return false;
    }
    let Some((who, time)) = who.rsplit_once_bytes(b' ') else {
        return false;
    };
    if time.is_empty() || !time.iter().all(u8::is_ascii_digit) || !who.ends_with(b">") {
        return false;
    }
    let Some(at) = who.windows(2).position(|s| s == b" <") else {
        return false;
    };
    at > 0
        && who.len() > at + 3
        && !who[..at].iter().any(|b| matches!(b, b'<' | b'>'))
        && !who[at + 2..who.len() - 1]
            .iter()
            .any(|b| matches!(b, b'<' | b'>'))
}
// Byte slices have no stable split_once predicate API on older Rust toolchains.
trait SplitBytes {
    fn rsplit_once_bytes(&self, needle: u8) -> Option<(&[u8], &[u8])>;
}
impl SplitBytes for [u8] {
    fn rsplit_once_bytes(&self, needle: u8) -> Option<(&[u8], &[u8])> {
        self.iter()
            .rposition(|b| *b == needle)
            .map(|i| (&self[..i], &self[i + 1..]))
    }
}

/// Narrow unsigned profile, root or one parent; other valid Git variants fail closed.
pub fn inspect_commit(raw: &[u8]) -> BridgeResult<CommitInfo> {
    if raw.is_empty() || raw.len() > 1024 * 1024 || raw.contains(&0) || raw.contains(&b'\r') {
        return Err(BridgeError("UNSUPPORTED_COMMIT_BYTES"));
    }
    let separator = raw
        .windows(2)
        .position(|b| b == b"\n\n")
        .ok_or(BridgeError("MISSING_HEADER_SEPARATOR"))?;
    let lines: Vec<_> = raw[..separator].split(|b| *b == b'\n').collect();
    if lines.len() != 3 && lines.len() != 4 {
        return Err(BridgeError("UNEXPECTED_COMMIT_HEADERS"));
    }
    fn link(line: &[u8], prefix: &[u8]) -> BridgeResult<Oid> {
        let value = line
            .strip_prefix(prefix)
            .ok_or(BridgeError("UNEXPECTED_HEADER_ORDER"))?;
        Oid::parse(std::str::from_utf8(value).map_err(|_| BridgeError("INVALID_HEADER_OID"))?)
    }
    let tree = link(lines[0], b"tree ")?;
    let mut parents = Vec::new();
    let author = if lines.len() == 4 {
        parents.push(link(lines[1], b"parent ")?);
        2
    } else {
        1
    };
    for (line, prefix) in [
        (lines[author], b"author ".as_slice()),
        (lines[author + 1], b"committer ".as_slice()),
    ] {
        if !line.strip_prefix(prefix).is_some_and(identity) {
            return Err(BridgeError("INVALID_IDENTITY_HEADER"));
        }
    }
    let message_size = raw.len() - separator - 2;
    if message_size == 0 {
        return Err(BridgeError("EMPTY_COMMIT_MESSAGE"));
    }
    Ok(CommitInfo {
        tree,
        parents,
        message_size,
    })
}
pub fn verify_commit(
    raw: &[u8],
    expected: Oid,
    parent: Option<Oid>,
    tree: Oid,
) -> BridgeResult<CommitInfo> {
    let info = inspect_commit(raw)?;
    if object_oid(ObjectKind::Commit, raw) != expected {
        return Err(BridgeError("COMMIT_HASH_MISMATCH"));
    }
    if info.tree != tree || info.parents != parent.into_iter().collect::<Vec<_>>() {
        return Err(BridgeError("COMMIT_LINK_MISMATCH"));
    }
    Ok(info)
}

/// Fixed allowlisted path; all other files and directories must remain identical.
pub fn verify_single_replacement(
    old: &FlatTree,
    new: &FlatTree,
    baseline: Oid,
    blob: Oid,
) -> BridgeResult<()> {
    let target = crate::status::TARGET;
    if old.files.get(target) != Some(&(Mode::File, baseline)) {
        return Err(BridgeError("STALE_BASE_CONFLICT"));
    }
    let mut expected = old.files.clone();
    expected.insert(target.to_owned(), (Mode::File, blob));
    if new.files != expected || new.directories != old.directories {
        return Err(BridgeError("UNEXPECTED_MODIFICATION"));
    }
    Ok(())
}
