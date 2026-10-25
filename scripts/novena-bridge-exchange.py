#!/usr/bin/env python3
"""Novena Bridge Exchange: isolated historical evidence pilot."""

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import tempfile
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parent.parent
LOGS = Path.home() / "novena-backups/block-logs"

SCHEMA_PATH = ROOT / "docs/bridge-lite/exchange-schema-v1.json"
JSON_PATH = ROOT / "docs/bridge-lite/EXCHANGE-PILOT.json"
MD_PATH = ROOT / "docs/bridge-lite/EXCHANGE-PILOT.md"

CASES = (
    ("NOVENA-SSH-GIT-INTEGRATION-120", "FAIL"),
    ("NOVENA-SSH-GIT-INTEGRATION-120R", "PASS"),
    ("NOVENA-BRIDGE-LITE-HARDENING-123", "FAIL"),
    ("NOVENA-BRIDGE-LITE-HARDENING-123R", "PASS"),
    ("NOVENA-BRIDGE-CONCEPT-127", "FAIL"),
    ("NOVENA-BRIDGE-CONCEPT-127R", "PASS"),
)

PALETTE = {
    "PASS": ("GREEN", "✅"),
    "FAIL": ("RED", "❌"),
    "REVIEW_REQUIRED": ("ORANGE", "⚠️"),
    "IN_PROGRESS": ("BLUE", "🔄"),
    "UNKNOWN": ("GREY", "❔"),
}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(
            lambda: handle.read(1024 * 1024), b""
        ):
            digest.update(chunk)
    return digest.hexdigest()


def marker(content, key):
    prefix = key + "="
    found = [
        line[len(prefix):].strip()
        for line in content.splitlines()
        if line.startswith(prefix)
    ]
    if len(set(found)) > 1:
        raise ValueError("CONFLICTING_" + key)
    return found[-1] if found else None


def inspect(block_id, expected):
    matches = list(LOGS.glob(block_id + "-*.txt"))
    if len(matches) != 1:
        raise ValueError(
            f"EXPECTED_ONE_LOG: {block_id}: {len(matches)}"
        )

    path = matches[0]
    sidecar = Path(str(path) + ".sha256")

    if not sidecar.is_file():
        raise ValueError("MISSING_SIDECAR: " + block_id)

    sidecar_lines = sidecar.read_text(
        encoding="utf-8"
    ).splitlines()

    if len(sidecar_lines) != 1:
        raise ValueError("INVALID_SIDECAR: " + block_id)

    match = re.fullmatch(
        r"([0-9a-fA-F]{64})[ \t]+[*]?(.+)",
        sidecar_lines[0],
    )

    if not match:
        raise ValueError("INVALID_SIDECAR: " + block_id)

    recorded_hash, recorded_path = match.groups()

    if Path(recorded_path).name != path.name:
        raise ValueError("SIDECAR_NAME_MISMATCH: " + block_id)

    if sha256(path) != recorded_hash.lower():
        raise ValueError("HASH_MISMATCH: " + block_id)

    if path.stat().st_size > 32 * 1024 * 1024:
        raise ValueError("LOG_TOO_LARGE: " + block_id)

    content = path.read_text(
        encoding="utf-8", errors="replace"
    )

    if marker(content, "BLOCK_ID") != block_id:
        raise ValueError("BLOCK_ID_MISMATCH")

    result = marker(content, "BLOCK_RESULT")
    code = marker(content, "COMMAND_EXIT_STATUS")

    if result != expected:
        raise ValueError("UNEXPECTED_BLOCK_RESULT")

    if code != ("0" if expected == "PASS" else "1"):
        raise ValueError("UNEXPECTED_PROCESS_RESULT")

    if marker(content, "LOG_COMPLETE") != "YES":
        raise ValueError("INCOMPLETE_LOG")

    block = {
        "block_id": block_id,
        "runner": expected,
        "upload": "UNKNOWN",
        "technical_result": result,
        "professional_acceptance": "UNKNOWN",
    }

    evidence = {
        "log_file": path.name,
        "sha256_file": sidecar.name,
        "local_integrity": "PASS",
        "archive_upload": "UNKNOWN",
    }

    return block, evidence


def create_index(records):
    latest, _ = records[-1]

    return {
        "schema_version": 1,
        "project": "novena",
        "generated_at_utc": dt.datetime.now(
            dt.timezone.utc
        ).isoformat(),
        "last_verified": None,
        "current": {
            "state": "CURRENT_UNKNOWN",
            "observed_at_utc": None,
        },
        "latest_block": dict(latest),
        "build": {
            "state": "UNKNOWN",
            "source": None,
        },
        "evidence": [
            dict(evidence)
            for _, evidence in records
        ],
        "publication": {
            "state": "NOT_PUBLISHED",
            "archive": "loomit77/project-block-logs",
            "commit": None,
        },
        "review_required": [
            "LAST_VERIFIED_NOT_QUALIFIED",
            "ARCHIVE_UPLOAD_NOT_INDEPENDENTLY_VERIFIED",
            "CURRENT_LIVE_STATE_UNKNOWN",
            "HISTORICAL_SIX_BLOCK_SAMPLE_ONLY",
        ],
    }


def render_markdown(index, records, json_digest):
    lines = [
        "# NOVENA-BRIDGE-LITE – Exchange-Pilot",
        "",
        "Status: isolierter Test, keine Veröffentlichung.",
        "",
        f"Erstellt: {index['generated_at_utc']}",
        "",
        "LAST_VERIFIED: UNKNOWN (null im JSON)",
        "CURRENT: ❔ CURRENT_UNKNOWN",
        "BUILD: ❔ UNKNOWN",
        "PUBLICATION: NOT_PUBLISHED",
        "",
        f"JSON-SHA256: `{json_digest}`",
        "",
        "## Historische Runner-Ergebnisse",
        "",
        "| Block | Runner | Upload | Fachabnahme |",
        "|---|---|---|---|",
    ]

    for block, _ in records:
        status = block["runner"]
        icon = PALETTE[status][1]
        lines.append(
            f"| `{block['block_id']}` "
            f"| {icon} {status} "
            "| ❔ UNKNOWN | ❔ UNKNOWN |"
        )

    lines.extend([
        "",
        "## Farbkonvention",
        "",
    ])

    for status, (color, symbol) in PALETTE.items():
        lines.append(f"- {symbol} {status}: {color}")

    lines.extend([
        "",
        "## Einschränkungen",
        "",
        "- Sechs ausgewählte historische Originalprotokolle.",
        "- SHA256 nur gegen lokale Sidecars geprüft.",
        "- Archiv-Upload nicht unabhängig verifiziert.",
        "- Fachabnahme bleibt UNKNOWN.",
        "- LAST_VERIFIED wurde nicht automatisch gesetzt.",
        "- Aktueller Build- und Hardwarezustand unbekannt.",
        "- Keine automatische GitHub-Veröffentlichung.",
        "",
    ])

    return "\n".join(lines)


def validate(index):
    schema = json.loads(
        SCHEMA_PATH.read_text(encoding="utf-8")
    )

    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.Draft202012Validator(
        schema,
        format_checker=jsonschema.FormatChecker(),
    ).validate(index)

    assert index["last_verified"] is None
    assert index["current"]["state"] == "CURRENT_UNKNOWN"
    assert index["publication"]["state"] == "NOT_PUBLISHED"

    for evidence in index["evidence"]:
        assert evidence["local_integrity"] == "PASS"
        assert evidence["archive_upload"] == "UNKNOWN"


def self_test():
    sample = [
        (
            {
                "block_id": "SYNTHETIC-FAIL",
                "runner": "FAIL",
                "upload": "PASS",
                "technical_result": "FAIL",
                "professional_acceptance": "UNKNOWN",
            },
            {
                "log_file": "synthetic.txt",
                "sha256_file": "synthetic.txt.sha256",
                "local_integrity": "PASS",
                "archive_upload": "UNKNOWN",
            },
        )
    ]

    index = create_index(sample)
    validate(index)

    assert index["last_verified"] is None
    assert index["latest_block"]["runner"] == "FAIL"
    assert index["latest_block"]["upload"] == "PASS"

    invalid = json.loads(json.dumps(index))
    invalid["latest_block"]["runner"] = "INVALID"

    schema = json.loads(SCHEMA_PATH.read_text())
    validator = jsonschema.Draft202012Validator(schema)
    assert list(validator.iter_errors(invalid))

    assert len(PALETTE) == 5
    assert len(set(PALETTE.values())) == 5

    print("SYNTHETIC_FAIL_CASE=PASS")
    print("INVALID_STATUS_REJECTED=PASS")
    print("UNQUALIFIED_CHECKPOINT_REMAINS_NULL=PASS")
    print("STATUS_PALETTE=PASS")
    print("EXCHANGE_SELFTEST=PASS")


def atomic_file(destination, contents):
    fd, name = tempfile.mkstemp(
        prefix=".novena-exchange-",
        dir=destination.parent,
    )
    temporary = Path(name)

    try:
        with os.fdopen(
            fd, "w", encoding="utf-8"
        ) as stream:
            stream.write(contents)
            stream.flush()
            os.fsync(stream.fileno())

        os.replace(temporary, destination)
    finally:
        temporary.unlink(missing_ok=True)


def generate():
    records = [inspect(*case) for case in CASES]

    assert len(records) == 6
    assert sum(
        block["runner"] == "PASS"
        for block, _ in records
    ) == 3

    assert sum(
        block["runner"] == "FAIL"
        for block, _ in records
    ) == 3

    index = create_index(records)
    validate(index)

    json_text = json.dumps(
        index, indent=2, ensure_ascii=False
    ) + "\n"

    json_digest = hashlib.sha256(
        json_text.encode("utf-8")
    ).hexdigest()

    markdown = render_markdown(
        index, records, json_digest
    )

    assert json_digest in markdown
    assert markdown.endswith("\n")

    atomic_file(JSON_PATH, json_text)
    atomic_file(MD_PATH, markdown)

    print("REAL_EVIDENCE_COUNT=6")
    print("HISTORICAL_PASS=3")
    print("HISTORICAL_FAIL=3")
    print("EXCHANGE_SCHEMA_VALIDATION=PASS")
    print("LAST_VERIFIED=NULL")
    print("CURRENT_STATE=CURRENT_UNKNOWN")
    print("BUILD_STATE=UNKNOWN")
    print("ARCHIVE_UPLOAD=UNKNOWN")
    print("GITHUB_PUBLICATION=NO")
    print("JSON_SHA256=" + json_digest)
    print("EXCHANGE_PILOT_GENERATED=PASS")


def main():
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-test", action="store_true")
    group.add_argument("--generate", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        self_test()
    else:
        generate()


if __name__ == "__main__":
    main()
