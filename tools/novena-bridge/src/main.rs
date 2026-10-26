//! Lesende Kommandozeile der Novena Bridge.

use novena_bridge::{canonical_json, git_blob_oid, parse_exchange_v1, validate_exchange_v1};
use novena_bridge::{
    objects::Oid,
    protocol,
    status::{self, PreviousKind, Status},
};
use std::{
    env, fs,
    io::{self, Read, Write},
    path::Path,
};

const USAGE: &str = "usage: novena-bridge {blob-id|canonical-json|exchange-v1-json|validate-exchange-v1|prepare-initial|prepare-objects|prepare-update-objects|verify-offline|verify-update-offline|verify-attestation|inspect-log|prepare-local-sources} FILE\n       novena-bridge {prepare-status|prepare-from-archive} PREVIOUS CANDIDATE\n       novena-bridge plan-cas REPOSITORY_ID BEFORE_OID AFTER_OID\nFILE may be - for stdin. Offline only; production publication is blocked.";

fn read_input(path: &std::ffi::OsStr, limit: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    if path == "-" {
        io::stdin()
            .lock()
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)?;
    } else {
        let path = Path::new(path);
        let meta = fs::symlink_metadata(path)?;
        if !meta.is_file() || meta.len() > limit as u64 {
            return Err("INPUT_NOT_REGULAR_OR_TOO_LARGE".into());
        }
        let file = fs::File::open(path)?;
        if !file.metadata()?.is_file() {
            return Err("INPUT_NOT_REGULAR".into());
        }
        file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    }
    if bytes.len() > limit {
        return Err("INPUT_TOO_LARGE".into());
    }
    Ok(bytes)
}
fn json_line(v: &serde_json::Value) -> Result<Vec<u8>, serde_json::Error> {
    let mut b = serde_json::to_vec(v)?;
    b.push(b'\n');
    Ok(b)
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let command = args.first().and_then(|a| a.to_str()).ok_or(USAGE)?;
    // Unconditional rejection occurs before opening any path or interpreting flags.
    if matches!(command, "publish" | "production-publish" | "--publish") {
        protocol::production_publish()?;
    }
    let output = match (command, args.len()) {
        ("prepare-local-sources", 2) => json_line(&novena_bridge::local_sources::prepare_bundle(
            &read_input(&args[1], status::MAX_PAYLOAD)?,
        )?)?,
        ("inspect-log", 2) => json_line(&novena_bridge::logs::inspect_file(Path::new(&args[1]))?)?,
        (
            "blob-id"
            | "canonical-json"
            | "exchange-v1-json"
            | "validate-exchange-v1"
            | "prepare-initial"
            | "prepare-objects"
            | "prepare-update-objects"
            | "verify-update-offline"
            | "verify-offline"
            | "verify-attestation",
            2,
        ) => {
            let limit = if matches!(
                command,
                "blob-id"
                    | "canonical-json"
                    | "prepare-objects"
                    | "prepare-update-objects"
                    | "verify-update-offline"
                    | "verify-offline"
                    | "verify-attestation"
            ) {
                32 * 1024 * 1024
            } else {
                status::MAX_PAYLOAD
            };
            let bytes = read_input(&args[1], limit)?;
            match command {
                "blob-id" => format!("{}\n", git_blob_oid(&bytes)).into_bytes(),
                "canonical-json" => canonical_json(&bytes)?,
                "exchange-v1-json" => exchange_output(&bytes, false)?,
                "validate-exchange-v1" => exchange_output(&bytes, true)?,
                "prepare-initial" => {
                    status::prepare_initial(&Status::parse(&bytes, PreviousKind::Local)?)?
                        .bytes()
                        .to_vec()
                }
                "prepare-objects" => {
                    json_line(&novena_bridge::planning::plan_offline_bundle(&bytes)?)?
                }
                "prepare-update-objects" => {
                    json_line(&novena_bridge::planning::plan_update_bundle(&bytes)?)?
                }
                "verify-attestation" => {
                    json_line(&novena_bridge::attestation::verify_bundle(&bytes)?.as_json())?
                }
                "verify-update-offline" => {
                    json_line(&protocol::verify_update_bundle(&bytes)?.as_json())?
                }
                "verify-offline" => json_line(&protocol::verify_offline_bundle(&bytes)?.as_json())?,
                _ => unreachable!(),
            }
        }
        ("prepare-status" | "prepare-from-archive", 3) => {
            if args[1] == "-" && args[2] == "-" {
                return Err("STDIN_CANNOT_BE_USED_TWICE".into());
            }
            let kind = if command == "prepare-status" {
                PreviousKind::Local
            } else {
                PreviousKind::ConfirmedArchive
            };
            let previous = Status::parse(&read_input(&args[1], status::MAX_PAYLOAD)?, kind)?;
            let candidate = Status::parse(
                &read_input(&args[2], status::MAX_PAYLOAD)?,
                PreviousKind::Local,
            )?;
            status::prepare(&previous, &candidate)?.bytes().to_vec()
        }
        ("plan-cas", 4) => {
            let id = args[1].to_str().ok_or("INVALID_REPOSITORY_ID")?;
            let before = Oid::parse(args[2].to_str().ok_or("INVALID_OID")?)?;
            let after = Oid::parse(args[3].to_str().ok_or("INVALID_OID")?)?;
            let request = protocol::update_refs_request(id, status::REF, before, after)?;
            json_line(
                &serde_json::json!({"request":request,"write_performed":false,"bridge_publication":"BLOCKED"}),
            )?
        }
        _ => return Err(USAGE.into()),
    };
    // Every validation completes before stdout receives any bytes.
    io::stdout().lock().write_all(&output)?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("novena-bridge: {error}");
        std::process::exit(1);
    }
}

// Validate fully before returning any output bytes.
fn exchange_output(content: &[u8], schema: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let document = parse_exchange_v1(content)?;
    if schema {
        validate_exchange_v1(&document)?;
    }
    let mut output = serde_json::to_vec(&document)?;
    output.push(b'\n');
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_preserves_pilot_output() {
        let pilot = include_bytes!("../../../docs/bridge-lite/EXCHANGE-PILOT.json");
        assert_eq!(
            exchange_output(pilot, true).unwrap(),
            exchange_output(pilot, false).unwrap()
        );
        assert_eq!(
            exchange_output(pilot, true).unwrap(),
            canonical_json(pilot).unwrap()
        );
    }

    #[test]
    fn syntax_schema_and_policy_remain_separate() {
        for content in [br#"{"a":1,"a":2}"#.as_slice(), br#"{"n":1.0}"#, b"\xff"] {
            assert!(exchange_output(content, false).is_err());
            assert!(exchange_output(content, true).is_err());
        }
        assert!(exchange_output(b"{}", false).is_ok());
        assert!(exchange_output(b"{}", true).is_err());
        let mut v = parse_exchange_v1(include_bytes!(
            "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
        ))
        .unwrap();
        v["publication"]["state"] = serde_json::json!("CONFIRMED");
        assert!(exchange_output(&serde_json::to_vec(&v).unwrap(), true).is_ok());
    }
}
