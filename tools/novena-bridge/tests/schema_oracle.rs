//! Independent installed Python jsonschema oracle; no downloads or bytecode writes.
use novena_bridge::{parse_exchange_v1, validate_exchange_v1};
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn actual_schema_differential() {
    let mut base = parse_exchange_v1(include_bytes!(
        "../../../docs/bridge-lite/EXCHANGE-PILOT.json"
    ))
    .unwrap();
    base["last_verified"] = json!({"block_id":"test","git_head":"a".repeat(40),
        "qualified_at_utc":"2026-10-26T12:00:00Z","qualification":"PASS","evidence_files":["test"]});
    let mut cases = vec![base.clone()];
    fn walk(v: &Value, path: String, paths: &mut Vec<String>) {
        paths.push(path.clone());
        match v {
            Value::Object(o) => {
                for (k, child) in o {
                    walk(child, format!("{path}/{k}"), paths);
                }
            }
            Value::Array(a) => {
                for (i, child) in a.iter().enumerate() {
                    walk(child, format!("{path}/{i}"), paths);
                }
            }
            _ => (),
        }
    }
    let mut paths = Vec::new();
    walk(&base, String::new(), &mut paths);
    for path in paths {
        for replacement in [
            Value::Null,
            json!(true),
            json!(0),
            json!(1),
            json!(""),
            json!("PASS"),
            json!([]),
            json!({}),
            json!("a".repeat(40)),
            json!(format!("{}\n", "a".repeat(40))),
        ] {
            let mut case = base.clone();
            *case.pointer_mut(&path).unwrap() = replacement;
            cases.push(case);
        }
        if let Some(o) = base.pointer(&path).unwrap().as_object() {
            for k in o.keys() {
                let mut case = base.clone();
                case.pointer_mut(&path)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove(k);
                cases.push(case);
            }
            let mut case = base.clone();
            case.pointer_mut(&path).unwrap()["extra"] = json!(1);
            cases.push(case);
        }
    }
    for date in [
        "2024-02-29T12:00:00Z",
        "2026-02-29T00:00:00Z",
        "2026-01-01t00:00:00z",
        "2026-01-01T00:00:00.123+23:59",
        "2026-01-01T24:00:00Z",
    ] {
        let mut case = base.clone();
        case["generated_at_utc"] = json!(date);
        cases.push(case);
    }
    let mut child = Command::new("python3").args(["-B", "-c", r#"
import json,sys,jsonschema,datetime,re
schema, cases=json.load(sys.stdin)
jsonschema.Draft202012Validator.check_schema(schema)
checker=jsonschema.FormatChecker()
# Installed jsonschema has no optional RFC3339 package. Supply an independent
# stdlib checker for this corpus (ordinary seconds; leap-second tests in Rust).
@checker.checks('date-time', raises=(ValueError,))
def date_time(s):
    if not isinstance(s,str): return True
    if not re.fullmatch(r'[0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?([Zz]|[+-][0-9]{2}:[0-9]{2})',s): return False
    if int(s[11:13])>23 or int(s[14:16])>59 or int(s[17:19])>59: return False
    if s[-1] not in 'Zz' and (int(s[-5:-3])>23 or int(s[-2:])>59): return False
    datetime.datetime.fromisoformat(s.upper().replace('Z','+00:00'))
    return True
v=jsonschema.Draft202012Validator(schema,format_checker=checker)
json.dump([v.is_valid(c) for c in cases],sys.stdout)
"#]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("installed Python jsonschema required for independent development oracle");
    let schema: Value = serde_json::from_slice(include_bytes!(
        "../../../docs/bridge-lite/exchange-schema-v1.json"
    ))
    .unwrap();
    // Feed on another thread so large stdin and stdout cannot deadlock.
    let mut stdin = child.stdin.take().unwrap();
    let input = serde_json::to_vec(&json!([schema, cases])).unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(&input).unwrap());
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let expected: Vec<bool> = serde_json::from_slice(&out.stdout).unwrap();
    for (i, (case, oracle)) in cases.iter().zip(expected).enumerate() {
        assert_eq!(
            validate_exchange_v1(case).is_ok(),
            oracle,
            "oracle case {i}: {case}"
        );
    }
    eprintln!("independent actual-schema cases: {}", cases.len());
}
