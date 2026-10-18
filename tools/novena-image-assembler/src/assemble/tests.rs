//! Synthetic source tests only. Not executed for NOVENA-DEBIAN-RUST-02.
use super::*;
use serde_json::{json, Value};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "novena-assembler-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn document() -> Value {
    let artifact = json!({"path":"/synthetic/unused", "origin":"synthetic fixture",
        "size":512, "sha256":"0".repeat(64)});
    json!({
        "schemaVersion":1, "profile":"novena-debian-2015", "sectorSize":512,
        "sourceRevision":manifest::REVISION,
        "inputs":{"spl":artifact.clone(), "proper":artifact.clone(),
            "firmware":artifact.clone(), "swap":artifact.clone(),
            "rootfs":artifact.clone(), "kernel":artifact.clone(), "dtb":artifact},
        "layout":{"diskId":"0x4e6f764d", "firmwareStart":4,
            "swapStart":6, "rootStart":8, "firmwareType":"0b"},
        "bootContract":{
            "pairProvenance":"synthetic evidence only", "romSplProperContract":"synthetic",
            "externalMmcEnvironment":"synthetic", "eepromAndHooks":"synthetic",
            "bootFilesAndArguments":"synthetic", "kernelDtbInitrdContract":"synthetic",
            "externalRootAndFstab":"synthetic", "partitionIdentityAndCollisions":"synthetic"}
    })
}
fn decode(v: &Value) -> Manifest {
    manifest::parse(&serde_json::to_vec(v).unwrap()).unwrap()
}
fn rejected(v: &Value) {
    assert!(manifest::parse(&serde_json::to_vec(v).unwrap()).is_err());
}

#[test]
fn mbr_bytes() {
    let plan = layout::plan(&decode(&document())).unwrap();
    let bytes = mbr::encode(&plan).unwrap();
    let mut expected = [0u8; 512];
    expected[440..444].copy_from_slice(&[0x4d, 0x76, 0x6f, 0x4e]);
    for (offset, kind, start) in [(446, 0x0b, 4), (462, 0x82, 6), (478, 0x83, 8)] {
        expected[offset..offset + 16].copy_from_slice(&[
            0, 0xfe, 0xff, 0xff, kind, 0xfe, 0xff, 0xff, start, 0, 0, 0, 1, 0, 0, 0,
        ]);
    }
    expected[510..512].copy_from_slice(&[0x55, 0xaa]);
    assert_eq!(bytes, expected);
}

#[test]
fn range_boundaries_and_overflows() {
    assert!(Range {
        offset: u64::MAX,
        size: 1
    }
    .end()
    .is_err());
    assert_eq!(
        Range {
            offset: u64::MAX - 1,
            size: 1
        }
        .end()
        .unwrap(),
        u64::MAX
    );
    assert!(layout::sector_bytes(u64::MAX).is_err());
    assert!(!Range {
        offset: 0,
        size: 512
    }
    .overlaps(Range {
        offset: 512,
        size: 1
    })
    .unwrap());
    assert!(Range {
        offset: 0,
        size: 513
    }
    .overlaps(Range {
        offset: 512,
        size: 1
    })
    .unwrap());
    let mut v = document();
    v["layout"]["firmwareStart"] = json!(3); // SPL ends exactly at FAT start.
    decode(&v);
    v["inputs"]["spl"]["size"] = json!(513);
    rejected(&v);
    let mut v = document();
    v["layout"]["rootStart"] = json!(u32::MAX as u64 - 1);
    decode(&v); // exclusive end is the configured DOS total-sector limit.
    v["layout"]["rootStart"] = json!(u32::MAX);
    rejected(&v);
    v["layout"]["rootStart"] = json!(u64::MAX);
    rejected(&v);
}

#[test]
fn overlaps_and_alignment() {
    for start in [0, 1, 2] {
        let mut v = document();
        v["layout"]["firmwareStart"] = json!(start);
        rejected(&v);
    }
    let mut v = document();
    v["layout"]["swapStart"] = json!(4);
    rejected(&v);
    let mut v = document();
    v["layout"]["rootStart"] = json!(5);
    rejected(&v);
    for size in [0, 511, 513, u64::MAX] {
        let mut v = document();
        v["inputs"]["rootfs"]["size"] = json!(size);
        rejected(&v);
    }
}

#[test]
fn invalid_manifests() {
    for (key, value) in [
        ("schemaVersion", json!(2)),
        ("schemaVersion", json!("1")),
        ("schemaVersion", json!(true)),
        ("schemaVersion", json!(-1)),
        ("sectorSize", json!(4096)),
        ("profile", json!("other")),
        ("sourceRevision", json!("rc12")),
        ("bootContract", Value::Null),
    ] {
        let mut v = document();
        v[key] = value;
        rejected(&v);
    }
    for pointer in ["", "/inputs", "/inputs/spl", "/layout", "/bootContract"] {
        let mut v = document();
        v.pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unknown".into(), json!(1));
        rejected(&v);
    }
    for key in [
        "pairProvenance",
        "romSplProperContract",
        "externalMmcEnvironment",
        "eepromAndHooks",
        "bootFilesAndArguments",
        "kernelDtbInitrdContract",
        "externalRootAndFstab",
        "partitionIdentityAndCollisions",
    ] {
        let mut v = document();
        v["bootContract"].as_object_mut().unwrap().remove(key);
        rejected(&v);
        let mut v = document();
        v["bootContract"][key] = json!(" \n");
        rejected(&v);
    }
    for hash in ["A".repeat(64), "g".repeat(64), "0".repeat(63)] {
        let mut v = document();
        v["inputs"]["spl"]["sha256"] = json!(hash);
        rejected(&v);
    }
    let mut v = document();
    v["inputs"]["spl"]["path"] = json!("relative");
    rejected(&v);
    let mut v = document();
    v["layout"]["diskId"] = json!("0x00000000");
    rejected(&v);
    let mut v = document();
    v["layout"]["firmwareType"] = json!("83");
    rejected(&v);
    let bytes = serde_json::to_string(&document()).unwrap();
    for (old, new) in [
        (
            "\"schemaVersion\":1",
            "\"schemaVersion\":1,\"schemaVersion\":1",
        ),
        (
            "\"firmwareStart\":4",
            "\"firmwareStart\":4,\"firmwareStart\":4",
        ),
        (
            "\"schemaVersion\":1",
            "\"schemaVersion\":1,\"schema\\u0056ersion\":1",
        ),
        ("\"schemaVersion\":1", "\"schemaVersion\":1.0"),
        ("\"sectorSize\":512", "\"sectorSize\":18446744073709551616"),
    ] {
        assert!(bytes.contains(old));
        assert!(manifest::parse(bytes.replace(old, new).as_bytes()).is_err());
    }
    assert!(manifest::parse(format!("{bytes} {{}}").as_bytes()).is_err());
    assert!(manifest::parse(&vec![b' '; manifest::MAX_MANIFEST as usize + 1]).is_err());
}

#[test]
fn input_hash_size_and_regular_file_checks() {
    let scratch = Scratch::new();
    let path = scratch.0.join("source");
    fs::write(&path, b"abc").unwrap();
    let mut artifact = decode(&document()).inputs.spl;
    artifact.size = 3;
    artifact.sha256 = format!("{:x}", Sha256::digest(b"abc"));
    let mut file = open_regular(&path).unwrap();
    verify_input(&mut file, &artifact).unwrap();
    artifact.sha256 = "0".repeat(64);
    assert!(verify_input(&mut file, &artifact).is_err());
    artifact.size = 4;
    assert!(verify_input(&mut file, &artifact).is_err());
    assert!(open_regular(&scratch.0).is_err());
    let link = scratch.0.join("link");
    std::os::unix::fs::symlink(&path, &link).unwrap();
    assert!(open_regular(&link).is_err());
}

#[test]
fn byte_preservation_readback_and_determinism() {
    // Deliberately raw byte fixtures: exercises the assembly core, not helper
    // filesystem recognition. These are NOT qualified FAT/swap/ext4 images.
    let scratch = Scratch::new();
    let mut m = decode(&document());
    for (i, a) in [
        &mut m.inputs.spl,
        &mut m.inputs.proper,
        &mut m.inputs.firmware,
        &mut m.inputs.swap,
        &mut m.inputs.rootfs,
        &mut m.inputs.kernel,
        &mut m.inputs.dtb,
    ]
    .into_iter()
    .enumerate()
    {
        let data = vec![u8::try_from(i + 1).unwrap(); 512];
        let path = scratch.0.join(format!("input-{i}"));
        fs::write(&path, &data).unwrap();
        a.path = path.to_str().unwrap().into();
        a.sha256 = format!("{:x}", Sha256::digest(&data));
    }
    let plan = layout::plan(&m).unwrap();
    let mut sources: Vec<_> = m
        .inputs
        .entries()
        .into_iter()
        .map(|(name, artifact)| Source {
            name,
            artifact,
            file: open_regular(Path::new(&artifact.path)).unwrap(),
        })
        .collect();
    let mut reports = Vec::new();
    let mut images = Vec::new();
    for i in 0..2 {
        let path = scratch.0.join(format!("output-{i}"));
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let result = write_image(&mut file, &plan, &mut sources).unwrap();
        reports.push(report_bytes(&m, &result).unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(&bytes[1024..1536], fs::read(&m.inputs.spl.path).unwrap());
        for (range, a) in
            plan.partitions
                .iter()
                .zip([&m.inputs.firmware, &m.inputs.swap, &m.inputs.rootfs])
        {
            assert_eq!(
                &bytes[range.offset as usize..range.end().unwrap() as usize],
                fs::read(&a.path).unwrap()
            );
        }
        assert!(bytes[512..1024].iter().all(|b| *b == 0));
        assert_eq!(result.sha256, format!("{:x}", Sha256::digest(&bytes)));
        for offset in [0, 512, 1024, plan.partitions[2].offset] {
            file.seek(SeekFrom::Start(offset)).unwrap();
            file.write_all(&[0xff]).unwrap();
            assert!(verify_output(&mut file, &plan, &sources, &result.sha256).is_err());
            file.seek(SeekFrom::Start(offset)).unwrap();
            file.write_all(&[bytes[offset as usize]]).unwrap();
        }
        file.set_len(plan.total_size - 1).unwrap();
        assert!(verify_output(&mut file, &plan, &sources, &result.sha256).is_err());
        images.push(bytes);
    }
    assert_eq!(images[0], images[1]);
    assert_eq!(reports[0], reports[1]);
    let reordered = document();
    // Whitespace/key order of input does not influence the canonical report.
    let parsed = decode(&reordered);
    let identity = OutputIdentity {
        size: 512,
        sha256: "0".repeat(64),
    };
    let first = report_bytes(&parsed, &identity).unwrap();
    let reversed_fields: Vec<_> = reordered
        .as_object()
        .unwrap()
        .iter()
        .rev()
        .map(|(key, value)| {
            format!(
                "{}: {}",
                serde_json::to_string(key).unwrap(),
                serde_json::to_string_pretty(value).unwrap()
            )
        })
        .collect();
    let reversed = format!("{{\n{}\n}}", reversed_fields.join(",\n"));
    assert_eq!(
        first,
        report_bytes(&manifest::parse(reversed.as_bytes()).unwrap(), &identity).unwrap()
    );
}
