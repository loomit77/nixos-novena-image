use novena_bridge::objects::*;
use novena_bridge::{BridgeError, BridgeResult};
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};

fn oid(c: char) -> Oid {
    Oid::parse(&c.to_string().repeat(40)).unwrap()
}
fn entry(name: &str, mode: Mode, id: Oid) -> TreeEntry {
    TreeEntry {
        name: name.into(),
        mode,
        oid: id,
    }
}
fn raw_commit(tree: Oid, parent: Oid) -> Vec<u8> {
    format!("tree {tree}\nparent {parent}\nauthor Novena Test <test@localhost.invalid> 1760000000 +0000\ncommitter Novena Test <test@localhost.invalid> 1760000000 +0000\n\nSynthetic Bridge commit\n").into_bytes()
}

#[test]
fn python_reference_vectors_and_independent_git_hashes() {
    let python = r#"
import importlib.util,json,sys
from pathlib import Path
root=Path(sys.argv[1])
def load(name):
    spec=importlib.util.spec_from_file_location('ref_'+name,root/('novena-bridge-'+name+'.py'))
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
t=load('git-tree'); c=load('git-commit')
entries=[{'path':'foo','mode':'040000','type':'tree','sha':'a'*40},
         {'path':'foo.bar','mode':'100644','type':'blob','sha':'b'*40},
         {'path':'Grüße','mode':'100755','type':'blob','sha':'c'*40},
         {'path':'link','mode':'120000','type':'blob','sha':'d'*40},
         {'path':'sub','mode':'160000','type':'commit','sha':'e'*40}]
raw=(b'tree '+b'a'*40+b'\nparent '+b'b'*40+b'\nauthor Novena Test <test@localhost.invalid> 1760000000 +0000\ncommitter Novena Test <test@localhost.invalid> 1760000000 +0000\n\nSynthetic Bridge commit\n')
json.dump({'tree':t.tree_sha(entries),'tree_hex':t.canonical_entries(entries).hex(),'commit':c.object_hash('commit',raw),'blob':t.object_sha('blob',b'{"last_verified":null}\n')},sys.stdout)
"#;
    let output = Command::new("python3")
        .args(["-B", "-c", python])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let entries = vec![
        entry("foo", Mode::Directory, oid('a')),
        entry("foo.bar", Mode::File, oid('b')),
        entry("Grüße", Mode::Executable, oid('c')),
        entry("link", Mode::Symlink, oid('d')),
        entry("sub", Mode::Gitlink, oid('e')),
    ];
    assert_eq!(
        tree_oid(&entries).unwrap().to_string(),
        expected["tree"].as_str().unwrap()
    );
    let bytes = tree_bytes(&entries).unwrap();
    assert_eq!(
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        expected["tree_hex"]
    );
    let raw = raw_commit(oid('a'), oid('b'));
    let id = object_oid(ObjectKind::Commit, &raw);
    assert_eq!(id.to_string(), expected["commit"]);
    assert!(verify_commit(&raw, id, Some(oid('b')), oid('a')).is_ok());
    assert_eq!(
        object_oid(ObjectKind::Blob, b"{\"last_verified\":null}\n").to_string(),
        expected["blob"]
    );
    for (kind, body, id) in [
        ("commit", raw, id),
        ("tree", bytes, tree_oid(&entries).unwrap()),
    ] {
        let mut child = Command::new("git")
            .args(["--no-optional-locks", "hash-object", "-t", kind, "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(&body).unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        assert_eq!(
            String::from_utf8(out.stdout).unwrap().trim(),
            id.to_string()
        );
    }
}

#[test]
fn raw_commit_reference_negatives_and_profiles() {
    let raw = raw_commit(oid('a'), oid('b'));
    let id = object_oid(ObjectKind::Commit, &raw);
    assert!(verify_commit(
        &[raw.clone(), b"tampered".to_vec()].concat(),
        id,
        Some(oid('b')),
        oid('a')
    )
    .is_err());
    assert!(verify_commit(&raw, id, Some(oid('c')), oid('a')).is_err());
    assert!(verify_commit(&raw, id, Some(oid('b')), oid('d')).is_err());
    let text = String::from_utf8(raw.clone()).unwrap();
    for malformed in [
        text.replacen("author ", &format!("parent {}\nauthor ", oid('c')), 1),
        text.replace("\n\nSynthetic", "\nencoding UTF-8\n\nSynthetic"),
        text.replacen("tree ", "parent ", 1),
        text.replace("\n\nSynthetic", "\nSynthetic"),
        format!("{text}\0"),
        text.replace("+0000", "00000"),
        text.replace("1760000000", "-1"),
        text.replace("\n\nSynthetic Bridge commit\n", "\n\n"),
    ] {
        assert!(inspect_commit(malformed.as_bytes()).is_err(), "{malformed}");
    }
    for bad in [
        "invalid".into(),
        "A".repeat(40),
        format!("{}\n", "a".repeat(40)),
        "é".repeat(20),
    ] {
        assert!(Oid::parse(&bad).is_err());
    }
    assert!(inspect_commit(&vec![b'x'; 1024 * 1024 + 1]).is_err());
    let root = text.replace(&format!("parent {}\n", oid('b')), " ");
    assert!(inspect_commit(root.as_bytes()).is_err());
    let root = text.replace(&format!("parent {}\n", oid('b')), "");
    assert!(inspect_commit(root.as_bytes()).unwrap().parents.is_empty());
}

struct Trees(BTreeMap<Oid, Vec<TreeEntry>>);
impl Trees {
    fn put(&mut self, entries: Vec<TreeEntry>) -> Oid {
        let id = tree_oid(&entries).unwrap();
        self.0.insert(id, entries);
        id
    }
}
impl TreeReader for Trees {
    fn tree(&mut self, id: Oid) -> BridgeResult<TreeRecord> {
        Ok(TreeRecord {
            oid: id,
            truncated: false,
            entries: self.0.get(&id).ok_or(BridgeError("MISSING_TREE"))?.clone(),
        })
    }
}
#[test]
fn nested_readback_tampering_omission_and_change_policy() {
    let mut trees = Trees(BTreeMap::new());
    let empty = trees.put(vec![]);
    let blob = object_oid(ObjectKind::Blob, b"{\"last_verified\":null}\n");
    let leaf = trees.put(vec![entry("NOVENA-CURRENT-STATE.json", Mode::File, blob)]);
    let middle = trees.put(vec![entry("status", Mode::Directory, leaf)]);
    let root = trees.put(vec![entry("novena", Mode::Directory, middle)]);
    let old = flatten_verified(&mut trees, empty).unwrap();
    let new = flatten_verified(&mut trees, root).unwrap();
    let target = "novena/status/NOVENA-CURRENT-STATE.json";
    verify_single_addition(&old, &new, target, blob).unwrap();
    let extra = trees.put(vec![
        entry("novena", Mode::Directory, middle),
        entry("extra-empty", Mode::Directory, empty),
    ]);
    let extra = flatten_verified(&mut trees, extra).unwrap();
    assert!(verify_single_addition(&old, &extra, target, blob).is_err());
    for changes in [
        vec![],
        vec![entry("NOVENA-CURRENT-STATE.json", Mode::File, oid('f'))],
        vec![entry("NOVENA-CURRENT-STATE.json", Mode::Executable, blob)],
    ] {
        trees.0.insert(leaf, changes);
        assert!(flatten_verified(&mut trees, root).is_err());
    }
    trees.0.remove(&middle);
    assert!(flatten_verified(&mut trees, root).is_err());
    assert!(verify_single_addition(&new, &new, target, blob).is_err());
    assert!(verify_blob(b"altered", blob).is_err());
    for bad in ["", ".", "..", "a/b", "nul\0name"] {
        assert!(tree_bytes(&[entry(bad, Mode::File, blob)]).is_err());
    }
    assert!(tree_bytes(&[
        entry("same", Mode::File, blob),
        entry("same", Mode::Directory, empty)
    ])
    .is_err());
}
