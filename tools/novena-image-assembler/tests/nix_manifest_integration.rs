use novena_image_assembler::manifest;
use serde_json::Value;
use std::{fs, path::PathBuf};

fn load(name: &str) -> Value {
    let root = PathBuf::from(std::env::var("NOVENA_JESSIE_24_TESTDIR").expect("test directory"));
    let bytes = fs::read(root.join(format!("{name}.manifest.json"))).expect("manifest");
    serde_json::from_slice(&bytes).expect("valid JSON")
}

#[test]
#[ignore = "requires NOVENA_JESSIE_24_TESTDIR with Nix-generated synthetic manifests"]
fn nix_reference_and_experimental_manifests() {
    let reference = load("reference");
    let experiment = load("experimental");

    let reference_bytes = serde_json::to_vec(&reference).unwrap();
    let experimental_bytes = serde_json::to_vec(&experiment).unwrap();

    let r = manifest::parse(&reference_bytes).expect("Nix reference manifest must parse");
    let e = manifest::parse(&experimental_bytes).expect("Nix experimental manifest must parse");

    assert!(!r.is_experimental());
    assert!(e.is_experimental());

    let mut invalid = experiment.clone();
    invalid["bootContract"] = reference["bootContract"].clone();

    assert!(
        manifest::parse(&serde_json::to_vec(&invalid).unwrap()).is_err(),
        "experiment must reject qualified bootContract"
    );

    let mut invalid = reference.clone();
    invalid["bootContract"] = Value::Null;

    assert!(
        manifest::parse(&serde_json::to_vec(&invalid).unwrap()).is_err(),
        "reference must reject missing boot evidence"
    );
}
