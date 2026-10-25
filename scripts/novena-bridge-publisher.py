#!/usr/bin/env python3
"""Novena Bridge Exchange: safe read-only publication planner."""

import argparse
import base64
import copy
import hashlib
import json
import subprocess
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parent.parent
SCHEMA = ROOT / "docs/bridge-lite/exchange-schema-v1.json"
PILOT = ROOT / "docs/bridge-lite/EXCHANGE-PILOT.json"

ARCHIVE = "loomit77/project-block-logs"
TARGET = "novena/status/NOVENA-CURRENT-STATE.json"


class PolicyError(Exception):
    pass


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def canonical(document):
    return (
        json.dumps(
            document,
            sort_keys=True,
            ensure_ascii=False,
            separators=(",", ":"),
        ) + "\n"
    ).encode("utf-8")


def read_json(path):
    path = Path(path)

    if not path.is_file() or path.is_symlink():
        raise PolicyError("INVALID_INPUT_FILE")

    if path.stat().st_size > 1024 * 1024:
        raise PolicyError("INPUT_TOO_LARGE")

    try:
        return json.loads(path.read_bytes())
    except (UnicodeError, ValueError):
        raise PolicyError("INVALID_JSON") from None


def schema_validator():
    schema = read_json(SCHEMA)
    jsonschema.Draft202012Validator.check_schema(schema)

    return jsonschema.Draft202012Validator(
        schema,
        format_checker=jsonschema.FormatChecker(),
    )


def validate(document, validator, remote=False):
    validator.validate(document)
    publication = document["publication"]

    if publication["archive"] != ARCHIVE:
        raise PolicyError("WRONG_ARCHIVE")

    if remote:
        if publication["state"] != "CONFIRMED":
            raise PolicyError("REMOTE_NOT_CONFIRMED")
        if publication["commit"] is None:
            raise PolicyError("REMOTE_COMMIT_MISSING")
    else:
        if publication["state"] != "NOT_PUBLISHED":
            raise PolicyError("FALSE_PUBLICATION_CLAIM")
        if publication["commit"] is not None:
            raise PolicyError("UNVERIFIED_COMMIT")

    return document


def protect_checkpoint(previous, candidate):
    if previous["last_verified"] != candidate["last_verified"]:
        raise PolicyError("CHECKPOINT_CHANGE_FORBIDDEN")

    return candidate


def prepare(previous, candidate, validator, remote=False):
    validate(previous, validator, remote=remote)
    validate(candidate, validator)

    return copy.deepcopy(
        protect_checkpoint(previous, candidate)
    )


def compare_and_plan(base_bytes, expected_sha, candidate):
    actual = (
        sha256(base_bytes)
        if base_bytes is not None
        else None
    )

    if actual != expected_sha:
        raise PolicyError("STALE_BASE_CONFLICT")

    payload = canonical(candidate)

    return {
        "target": TARGET,
        "base_sha256": actual,
        "candidate_sha256": sha256(payload),
        "candidate_bytes": len(payload),
        "write_performed": False,
    }


def remote_read():
    """Read existing archive file using GitHub CLI GET only."""
    command = [
        "gh", "api",
        "--method", "GET",
        "repos/" + ARCHIVE + "/contents/" + TARGET,
        "-f", "ref=main",
    ]

    try:
        result = subprocess.run(
            command,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        raise PolicyError("REMOTE_READ_UNAVAILABLE") from None

    if result.returncode != 0:
        raise PolicyError("REMOTE_READ_FAILED")

    try:
        record = json.loads(result.stdout)

        if record.get("type") != "file":
            raise PolicyError("REMOTE_NOT_A_FILE")

        if record.get("encoding") != "base64":
            raise PolicyError("REMOTE_ENCODING_UNSUPPORTED")

        encoded = record["content"].replace("\n", "")
        raw = base64.b64decode(encoded, validate=True)

        if len(raw) > 1024 * 1024:
            raise PolicyError("REMOTE_TOO_LARGE")

        document = json.loads(raw)
    except (KeyError, ValueError, TypeError, UnicodeError):
        raise PolicyError("REMOTE_CONTENT_INVALID") from None

    return document, raw


def expect_rejection(label, action):
    try:
        action()
    except (PolicyError, jsonschema.ValidationError):
        print(label + "=REJECTED")
    else:
        raise AssertionError(label + "_NOT_REJECTED")


def self_test():
    validator = schema_validator()
    pilot = read_json(PILOT)
    validate(pilot, validator)

    checkpoint = {
        "block_id": "SYNTHETIC-BASE",
        "git_head": (
            "ddecfe464f07baf8929765568e94049b499280f3"
        ),
        "qualified_at_utc": "2026-10-25T12:00:00Z",
        "qualification": "PASS",
        "evidence_files": ["synthetic.txt"],
    }

    previous = copy.deepcopy(pilot)
    previous["last_verified"] = checkpoint

    failed = copy.deepcopy(previous)
    failed["latest_block"] = {
        "block_id": "SYNTHETIC-FAIL",
        "runner": "FAIL",
        "upload": "PASS",
        "technical_result": "FAIL",
        "professional_acceptance": "UNKNOWN",
    }

    result = prepare(previous, failed, validator)
    assert result["last_verified"] == checkpoint
    assert result["latest_block"]["runner"] == "FAIL"

    print("FAILED_BLOCK_PRESERVES_CHECKPOINT=PASS")

    passed = copy.deepcopy(pilot)
    passed["latest_block"] = {
        "block_id": "SYNTHETIC-PASS",
        "runner": "PASS",
        "upload": "PASS",
        "technical_result": "PASS",
        "professional_acceptance": "UNKNOWN",
    }

    assert prepare(
        pilot, passed, validator
    )["last_verified"] is None

    print("UNQUALIFIED_PASS_NOT_PROMOTED=PASS")

    removal = copy.deepcopy(failed)
    removal["last_verified"] = None

    expect_rejection(
        "CHECKPOINT_REMOVAL",
        lambda: prepare(previous, removal, validator),
    )

    replacement = copy.deepcopy(failed)
    replacement["last_verified"]["block_id"] = "OTHER"

    expect_rejection(
        "CHECKPOINT_REPLACEMENT",
        lambda: prepare(previous, replacement, validator),
    )

    invalid = copy.deepcopy(failed)
    invalid["latest_block"]["runner"] = "INVALID"

    expect_rejection(
        "INVALID_STATUS",
        lambda: prepare(previous, invalid, validator),
    )

    claimed = copy.deepcopy(failed)
    claimed["publication"]["state"] = "CONFIRMED"

    expect_rejection(
        "FALSE_PUBLICATION",
        lambda: prepare(previous, claimed, validator),
    )

    claimed_commit = copy.deepcopy(failed)
    claimed_commit["publication"]["commit"] = "a" * 40

    expect_rejection(
        "UNVERIFIED_COMMIT",
        lambda: prepare(previous, claimed_commit, validator),
    )

    old_bytes = canonical(previous)
    planned = compare_and_plan(
        old_bytes, sha256(old_bytes), result
    )

    assert planned["write_performed"] is False

    expect_rejection(
        "STALE_BASE",
        lambda: compare_and_plan(
            old_bytes, "0" * 64, result
        ),
    )

    assert canonical(previous) == old_bytes

    print("SIX_NEGATIVE_CASES=PASS")
    print("NO_FILE_WRITES=PASS")
    print("SELF_TEST=PASS")


def dry_run(candidate_path, previous_path, check_remote):
    validator = schema_validator()
    candidate = read_json(candidate_path)
    validate(candidate, validator)

    if check_remote:
        previous, previous_bytes = remote_read()
        prepare(
            previous, candidate, validator, remote=True
        )
        print("REMOTE_READ=PASS")
        print("REMOTE_SHA256=" + sha256(previous_bytes))
        print("REMOTE_CONCURRENCY_LOCK=NO")
    else:
        source = previous_path or PILOT
        previous_bytes = Path(source).read_bytes()
        previous = read_json(source)

        prepare(previous, candidate, validator)

        print(
            "LOCAL_BASELINE=PASS"
            if previous_path
            else "DEFAULT_PILOT_BASELINE=PASS"
        )

    plan = compare_and_plan(
        previous_bytes,
        sha256(previous_bytes),
        candidate,
    )

    print("TARGET=" + plan["target"])
    print("CANDIDATE_SHA256=" + plan["candidate_sha256"])
    print("CANDIDATE_BYTES=" + str(plan["candidate_bytes"]))
    print("SCHEMA_VALIDATION=PASS")
    print("LAST_VERIFIED_PRESERVED=PASS")
    print("DRY_RUN=PASS")
    print("REMOTE_WRITE=NO")
    print("GIT_COMMIT=NO")
    print("GIT_PUSH=NO")


def main():
    parser = argparse.ArgumentParser(
        description="Novena Bridge Publisher: dry-run only"
    )

    mode = parser.add_mutually_exclusive_group(required=True)

    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--dry-run", action="store_true")

    parser.add_argument(
        "--candidate", type=Path, default=PILOT
    )
    parser.add_argument(
        "--previous-file", type=Path
    )
    parser.add_argument(
        "--remote-check", action="store_true"
    )

    args = parser.parse_args()

    if args.remote_check and not args.dry_run:
        parser.error("--remote-check requires --dry-run")

    if args.remote_check and args.previous_file:
        parser.error(
            "--remote-check conflicts with --previous-file"
        )

    try:
        if args.self_test:
            self_test()
        else:
            dry_run(
                args.candidate,
                args.previous_file,
                args.remote_check,
            )
    except (
        PolicyError,
        jsonschema.ValidationError,
        jsonschema.SchemaError,
    ) as error:
        print("PUBLISHER_RESULT=FAIL")
        print(
            "REASON=" + (
                str(error)
                if isinstance(error, PolicyError)
                else "JSON_SCHEMA_ERROR"
            )
        )
        raise SystemExit(1)


if __name__ == "__main__":
    main()
