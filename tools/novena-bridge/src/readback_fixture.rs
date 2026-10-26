//! Gemeinsame synthetische Readback-Fixture für Offline- und lokale TLS-Tests.
use super::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone)]
pub(crate) struct Fixture {
    pub(crate) records: BTreeMap<String, Value>,
    pub(crate) raw: BTreeMap<Oid, Vec<u8>>,
    pub(crate) before: Oid,
    pub(crate) after: Oid,
    pub(crate) head: Oid,
    pub(crate) head_sequence: VecDeque<Oid>,
    pub(crate) leaf: Oid,
    pub(crate) blob: Oid,
}
pub(crate) fn e(name: &str, mode: Mode, oid: Oid) -> TreeEntry {
    TreeEntry {
        name: name.into(),
        mode,
        oid,
    }
}
pub(crate) fn base64(bytes: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
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
pub(crate) fn payload() -> PreparedStatus {
    prepare_initial(
        &Status::parse(
            include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json"),
            PreviousKind::Local,
        )
        .unwrap(),
    )
    .unwrap()
}
impl Fixture {
    pub(crate) fn new(extra: bool) -> Self {
        let mut f = Self {
            records: BTreeMap::new(),
            raw: BTreeMap::new(),
            before: Oid::parse(&"a".repeat(40)).unwrap(),
            after: Oid::parse(&"b".repeat(40)).unwrap(),
            head: Oid::parse(&"c".repeat(40)).unwrap(),
            head_sequence: VecDeque::new(),
            leaf: Oid::parse(&"d".repeat(40)).unwrap(),
            blob: object_oid(ObjectKind::Blob, payload().bytes()),
        };
        f.records.insert(
            PREFIX.into(),
            json!({"full_name":ARCHIVE,"node_id":"SYNTHETIC_REPOSITORY_ID"}),
        );
        let existing = object_oid(ObjectKind::Blob, b"preserved");
        let old = f.put_tree(vec![e("README.txt", Mode::File, existing)]);
        f.before = f.put_commit(old, None, "Baseline");
        f.leaf = f.put_tree(vec![e("NOVENA-CURRENT-STATE.json", Mode::File, f.blob)]);
        let middle = f.put_tree(vec![e("status", Mode::Directory, f.leaf)]);
        let mut entries = vec![
            e("README.txt", Mode::File, existing),
            e("novena", Mode::Directory, middle),
        ];
        if extra {
            let empty = f.put_tree(vec![]);
            entries.push(e("unexpected-empty", Mode::Directory, empty));
        }
        let root = f.put_tree(entries);
        f.after = f.put_commit(root, Some(f.before), "Synthetic Bridge commit");
        f.head = f.after;
        f.records.insert(format!("{PREFIX}/git/blobs/{}",f.blob),json!({"sha":f.blob.to_string(),"encoding":"base64","size":payload().bytes().len(),"content":base64(payload().bytes())}));
        f
    }
    pub(crate) fn put_tree(&mut self, entries: Vec<TreeEntry>) -> Oid {
        let id = tree_oid(&entries).unwrap();
        let entries:Vec<_>=entries.into_iter().map(|e| {let (kind,mode)=match e.mode {Mode::File=>("blob","100644"),Mode::Directory=>("tree","040000"),_=>panic!()};json!({"path":e.name,"mode":mode,"type":kind,"sha":e.oid.to_string(),"size":0,"url":"https://example.invalid/ignored"})}).collect();
        self.records.insert(
            format!("{PREFIX}/git/trees/{id}"),
            json!({"sha":id.to_string(),"tree":entries,"truncated":false}),
        );
        id
    }
    pub(crate) fn put_commit(&mut self, tree: Oid, parent: Option<Oid>, message: &str) -> Oid {
        let parent_line = parent.map(|p| format!("parent {p}\n")).unwrap_or_default();
        let raw=format!("tree {tree}\n{parent_line}author Novena Test <test@localhost.invalid> 1760000000 +0000\ncommitter Novena Test <test@localhost.invalid> 1760000000 +0000\n\n{message}\n").into_bytes();
        let id = object_oid(ObjectKind::Commit, &raw);
        self.raw.insert(id, raw);
        self.records.insert(format!("{PREFIX}/git/commits/{id}"),json!({"sha":id.to_string(),"tree":{"sha":tree.to_string()},"parents":parent.into_iter().map(|p|json!({"sha":p.to_string()})).collect::<Vec<_>>()}));
        id
    }
}
