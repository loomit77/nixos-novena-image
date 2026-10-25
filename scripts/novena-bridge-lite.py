#!/usr/bin/env python3
"""Novena Bridge Lite: conservative local status snapshots."""

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOGDIR = Path.home() / "novena-backups/block-logs"

OUT_JSON = ROOT / "docs/bridge-lite/current-state.json"
OUT_MD = ROOT / "docs/PROJECT-STATUS.md"

SCHEMA_VERSION = 1
LIMIT = 12
MAX_LOG_BYTES = 32 * 1024 * 1024


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(
            lambda: stream.read(1024 * 1024), b""
        ):
            h.update(chunk)
    return h.hexdigest()


def git(*arguments):
    return subprocess.check_output(
        ["git", "--no-optional-locks", *arguments],
        cwd=ROOT,
        text=True,
    ).strip()


def values(content, key):
    prefix = key + "="
    return [
        line[len(prefix):].strip()
        for line in content.splitlines()
        if line.startswith(prefix)
    ]


def last(content, key):
    found = values(content, key)
    return found[-1] if found else None


def verify_sidecar(path):
    sidecar = Path(str(path) + ".sha256")

    if not sidecar.is_file():
        return "MISSING"

    try:
        lines = sidecar.read_text(
            encoding="utf-8"
        ).splitlines()

        if len(lines) != 1:
            return "INVALID"

        match = re.fullmatch(
            r"([0-9a-fA-F]{64})[ \t]+[*]?(.+)",
            lines[0],
        )
        if not match:
            return "INVALID"

        expected, filename = match.groups()

        if Path(filename).name != path.name:
            return "INVALID"

        return (
            "PASS"
            if digest(path) == expected.lower()
            else "FAIL"
        )
    except (OSError, UnicodeError):
        return "INVALID"


def inspect(path):
    source = path.name

    if path.stat().st_size > MAX_LOG_BYTES:
        return {
            "source": source,
            "state": "REVIEW_REQUIRED",
            "integrity": "UNKNOWN",
            "reasons": ["LOG_TOO_LARGE"],
        }

    raw = path.read_text(
        encoding="utf-8",
        errors="replace",
    )

    integrity = verify_sidecar(path)
    complete = last(raw, "LOG_COMPLETE") == "YES"
    block = last(raw, "BLOCK_ID")
    result = last(raw, "BLOCK_RESULT")
    exit_code = last(raw, "COMMAND_EXIT_STATUS")

    reasons = []

    for key in (
        "BLOCK_ID",
        "BLOCK_RESULT",
        "COMMAND_EXIT_STATUS",
    ):
        items = values(raw, key)
        if len(set(items)) > 1:
            reasons.append("CONFLICTING_" + key)

    if integrity != "PASS":
        reasons.append("LOG_INTEGRITY_" + integrity)

    if not complete:
        reasons.append("LOG_INCOMPLETE")

    if not block or result is None or exit_code is None:
        reasons.append("MISSING_RESULT_FIELDS")

    if not complete and integrity == "MISSING":
        state = "IN_PROGRESS_OR_INCOMPLETE"
    elif reasons:
        state = "REVIEW_REQUIRED"
    else:
        state = "COMPLETE"

    if reasons and any(
        r.startswith("CONFLICTING_") for r in reasons
    ):
        state = "REVIEW_REQUIRED"

    return {
        "source": source,
        "block_id": block,
        "state": state,
        "integrity": integrity,
        "complete": complete,
        "process_result": (
            "PASS" if exit_code == "0"
            else "FAIL" if exit_code is not None
            else "UNKNOWN"
        ),
        "technical_result": result or "UNKNOWN",
        "professional_acceptance": "UNKNOWN",
        "upload_result": "UNKNOWN",
        "reasons": sorted(set(reasons)),
    }


def collect(directory=LOGDIR, limit=LIMIT):
    if not directory.is_dir():
        return [], [], ["LOG_DIRECTORY_MISSING"]

    candidates = sorted(
        directory.glob("*.txt"),
        key=lambda p: (
            p.stat().st_mtime_ns,
            p.name,
        ),
        reverse=True,
    )

    finished = []
    pending = []
    warnings = []

    for path in candidates:
        if len(finished) >= limit:
            break

        try:
            item = inspect(path)
        except (OSError, UnicodeError) as error:
            pending.append({
                "source": path.name,
                "state": "REVIEW_REQUIRED",
                "reasons": [type(error).__name__],
            })
            continue

        if item["state"] == "IN_PROGRESS_OR_INCOMPLETE":
            pending.append({
                "source": item["source"],
                "state": item["state"],
                "reasons": item["reasons"],
            })
            continue

        finished.append(item)
        if item["reasons"]:
            warnings.append(item["source"])

    return finished, pending, warnings


def image_inventory():
    path = ROOT / "docs/image-inventory.json"
    raw = json.loads(path.read_text(encoding="utf-8"))

    return {
        "source": "docs/image-inventory.json",
        "record_count": raw.get("record_count"),
        "unique_file_hashes": raw.get("unique_file_hashes"),
        "host_counts": raw.get("host_counts"),
        "scan_warnings": len(
            raw.get("scan_warnings", [])
        ),
        "path_conflicts": len(
            raw.get("path_conflicts", [])
        ),
        "hardware_image_identity": "UNKNOWN",
    }


def snapshot(directory=LOGDIR):
    finished, pending, warnings = collect(directory)

    result = {
        "schema_version": SCHEMA_VERSION,
        "generated_at_utc": dt.datetime.now(
            dt.timezone.utc
        ).isoformat(),
        "git": {
            "head": git("rev-parse", "HEAD"),
            "branch": git("branch", "--show-current"),
            "worktree": (
                "CLEAN"
                if not git("status", "--porcelain=v1")
                else "MODIFIED"
            ),
            "remote_sync": "UNKNOWN",
        },
        "image_inventory": image_inventory(),
        "recent_blocks": finished,
        "pending_sources": pending,
        "review_required_sources": warnings,
        "limitations": [
            "Local logs only",
            "No independently verified upload status",
            "No automatic professional acceptance",
            "No verified hardware/image link",
            "No automatic release approval",
        ],
    }

    canonical = json.dumps(
        result,
        sort_keys=True,
        ensure_ascii=False,
    ).encode("utf-8")

    result["snapshot_id"] = hashlib.sha256(
        canonical
    ).hexdigest()

    return result


def safe_cell(value):
    return str(value).replace(
        "|", "/"
    ).replace("\n", " ")


def render_md(data):
    inv = data["image_inventory"]
    g = data["git"]

    lines = [
        "# Novena – aktueller Projektstatus",
        "",
        "Automatisch erzeugter Bridge-Lite-Snapshot.",
        "Kein eigenständiger Abnahmenachweis.",
        "",
        f"Snapshot-ID: `{data['snapshot_id']}`",
        f"Erstellt: {data['generated_at_utc']}",
        f"Git-HEAD: `{g['head']}`",
        f"Branch: `{g['branch']}`",
        f"Worktree: `{g['worktree']}`",
        "Remote-Synchronisierung: UNKNOWN",
        "",
        "## Image-Inventur",
        "",
        f"Dateivorkommen: {inv['record_count']}",
        f"Unterschiedliche Hashes: {inv['unique_file_hashes']}",
        f"Scanwarnungen: {inv['scan_warnings']}",
        f"Pfadkonflikte: {inv['path_conflicts']}",
        "",
        "Hardware-/Image-Zuordnung: UNKNOWN",
        "",
        "## Abgeschlossene oder prüfbedürftige Logs",
        "",
        "| Block | Status | SHA256 | Technik | Fachabnahme |",
        "|---|---|---|---|---|",
    ]

    for item in data["recent_blocks"]:
        cells = [
            item.get("block_id") or "UNKNOWN",
            item["state"],
            item["integrity"],
            item.get("technical_result", "UNKNOWN"),
            "UNKNOWN",
        ]
        lines.append(
            "| " + " | ".join(
                safe_cell(v) for v in cells
            ) + " |"
        )

    lines += [
        "",
        "## Vorläufige oder unvollständige Quellen",
        "",
    ]

    for item in data["pending_sources"][:20]:
        lines.append(
            "- `" + safe_cell(item["source"]) +
            "`: " + safe_cell(item["state"])
        )

    lines += [
        "",
        "## Grenzen",
        "",
        "- Keine automatische fachliche Abnahme.",
        "- Kein unabhängiger GitHub-Uploadnachweis.",
        "- Unvollständige Logs sind vorläufig.",
        "- Maßgeblich bleiben die Originalquellen.",
        "",
    ]

    return "\n".join(lines)


def publish(data):
    json_text = json.dumps(
        data,
        ensure_ascii=False,
        indent=2,
    ) + "\n"

    md_text = render_md(data)

    assert json.loads(json_text) == data
    assert data["snapshot_id"] in md_text

    temporary = []

    try:
        for destination, content in (
            (OUT_JSON, json_text),
            (OUT_MD, md_text),
        ):
            fd, name = tempfile.mkstemp(
                prefix=".bridge-lite-",
                dir=destination.parent,
            )
            path = Path(name)
            temporary.append(path)

            with os.fdopen(
                fd, "w", encoding="utf-8"
            ) as stream:
                stream.write(content)
                stream.flush()
                os.fsync(stream.fileno())

        os.replace(temporary[0], OUT_JSON)
        os.replace(temporary[1], OUT_MD)

        temporary.clear()
    finally:
        for path in temporary:
            path.unlink(missing_ok=True)


def selftest():
    with tempfile.TemporaryDirectory() as folder:
        directory = Path(folder)
        log = directory / "SYNTHETIC.txt"
        sidecar = Path(str(log) + ".sha256")

        text = (
            "BLOCK_ID=SYNTHETIC\n"
            "COMMAND_EXIT_STATUS=0\n"
            "BLOCK_RESULT=PASS\n"
            "LOG_COMPLETE=YES\n"
        )

        log.write_text(text)
        sidecar.write_text(
            digest(log) + "  " + str(log) + "\n"
        )

        item = inspect(log)
        assert item["state"] == "COMPLETE"
        assert item["integrity"] == "PASS"
        assert item["professional_acceptance"] == "UNKNOWN"
        print("COMPLETE_LOG=PASS")

        log.write_text(
            text + "BLOCK_RESULT=FAIL\n"
        )
        sidecar.write_text(
            digest(log) + "  " + str(log) + "\n"
        )
        item = inspect(log)
        assert "CONFLICTING_BLOCK_RESULT" in item["reasons"]
        assert item["state"] == "REVIEW_REQUIRED"
        print("CONFLICTING_RESULT=DETECTED")

        sidecar.unlink()
        item = inspect(log)
        assert item["integrity"] == "MISSING"
        print("MISSING_SIDECAR=DETECTED")

        log.write_text(
            "BLOCK_ID=RUNNING\n"
        )
        item = inspect(log)
        assert item["state"] == "IN_PROGRESS_OR_INCOMPLETE"
        assert not collect(directory)[0]
        print("RUNNING_LOG_EXCLUDED=PASS")

        log.write_text(text)
        sidecar.write_text(
            "0" * 64 + "  " + str(log) + "\n"
        )
        assert inspect(log)["integrity"] == "FAIL"
        print("TAMPER_DETECTION=PASS")

    print("BRIDGE_SELFTEST=PASS")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--generate", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        selftest()
        return

    if not args.generate:
        parser.error("Use --self-test or --generate")

    result = snapshot()
    publish(result)

    print("SNAPSHOT_GENERATED=PASS")
    print("SNAPSHOT_ID=" + result["snapshot_id"])
    print("FINISHED_COUNT=" +
          str(len(result["recent_blocks"])))
    print("PENDING_COUNT=" +
          str(len(result["pending_sources"])))
    print("REVIEW_COUNT=" +
          str(len(result["review_required_sources"])))


if __name__ == "__main__":
    main()
