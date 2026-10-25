#!/usr/bin/env python3
"""Novena Bridge Exchange: local synthetic publication gate.

Production publication is deliberately unavailable.
"""

import argparse
import copy
import hashlib
import json
import importlib.util
import os
import re
import subprocess
import tempfile
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parent.parent
PILOT = ROOT / "docs/bridge-lite/EXCHANGE-PILOT.json"

TARGET = "novena/status/NOVENA-CURRENT-STATE.json"
PRODUCTION_ENABLED = False

# Reuse planner validation and canonical serialization; no network calls here.
_spec = importlib.util.spec_from_file_location(
    "novena_bridge_publisher", ROOT / "scripts/novena-bridge-publisher.py"
)
planner = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(planner)


class GateError(Exception):
    pass


def git_environment():
    env = {key: value for key, value in os.environ.items()
           if not key.startswith("GIT_")}
    env.update(GIT_TERMINAL_PROMPT="0", GIT_CONFIG_NOSYSTEM="1",
               GIT_CONFIG_GLOBAL=os.devnull, GIT_ALLOW_PROTOCOL="file")
    return env


def git(directory, *args, check=True):
    env = git_environment()

    result = subprocess.run(
        ["git", "-c", "core.hooksPath=" + os.devnull, "-C", str(directory), *args],
        capture_output=True,
        text=True,
        timeout=30,
        check=False,
        env=env,
    )
    if check and result.returncode:
        raise GateError("GIT_OPERATION_FAILED")
    return result


def head(repository):
    value = git(
        repository, "rev-parse", "refs/heads/main"
    ).stdout.strip()

    if not re.fullmatch(r"[0-9a-f]{40}", value):
        raise GateError("INVALID_GIT_HEAD")
    return value


def payload(document):
    planner.validate(document, planner.schema_validator())

    if document["last_verified"] is not None:
        raise GateError("INITIAL_CHECKPOINT_NOT_NULL")
    if document["publication"]["state"] != "NOT_PUBLISHED":
        raise GateError("FALSE_PUBLICATION_CLAIM")
    if document["publication"]["commit"] is not None:
        raise GateError("UNVERIFIED_COMMIT")

    return planner.canonical(document)


def verify_readback(repository, expected_head, expected_bytes):
    """Verify one immutable commit and check branch HEAD on both sides."""
    if head(repository) != expected_head:
        raise GateError("REMOTE_HEAD_MISMATCH")
    if git(repository, "cat-file", "-t", expected_head + ":" + TARGET).stdout.strip() != "blob":
        raise GateError("TARGET_NOT_BLOB")
    stored = subprocess.run(
        ["git", "-C", str(repository), "show", expected_head + ":" + TARGET],
        capture_output=True, timeout=30, check=False, env=git_environment(),
    )
    if stored.returncode or stored.stdout != expected_bytes:
        raise GateError("REMOTE_CONTENT_MISMATCH")
    if head(repository) != expected_head:
        raise GateError("REMOTE_HEAD_MISMATCH")


def verify_single_change(work, baseline, new_head):
    if git(work, "rev-list", "--parents", "-n", "1", new_head).stdout.strip() != (
        new_head + " " + baseline
    ):
        raise GateError("UNEXPECTED_COMMIT_PARENT")
    changes = git(work, "diff-tree", "--no-commit-id", "--name-status",
                  "-r", baseline, new_head).stdout.splitlines()
    if changes != ["A\t" + TARGET]:
        raise GateError("UNEXPECTED_COMMIT_CHANGES")


def create_initial_index(
    test_remote, expected_head, document, test_root
):
    """Write only inside a marked, local synthetic bare Git repo."""

    remote = Path(test_remote).resolve()
    root = Path(test_root).resolve()

    if remote == root or not remote.is_relative_to(root):
        raise GateError("REMOTE_OUTSIDE_TEST_ROOT")

    if not remote.is_dir():
        raise GateError("TEST_REMOTE_MISSING")

    marker = remote / "NOVENA_SYNTHETIC_TEST_ONLY"
    if not marker.is_file() or marker.is_symlink():
        raise GateError("TEST_MARKER_MISSING")

    if git(
        remote, "rev-parse", "--is-bare-repository"
    ).stdout.strip() != "true":
        raise GateError("REMOTE_NOT_BARE")

    if not re.fullmatch(r"[0-9a-f]{40}", expected_head):
        raise GateError("INVALID_EXPECTED_HEAD")
    data = payload(document)

    if head(remote) != expected_head:
        raise GateError("STALE_REMOTE_HEAD")

    with tempfile.TemporaryDirectory(
        prefix="novena-publish-work-",
        dir=root,
    ) as workdir:
        work = Path(workdir) / "checkout"

        cloned = subprocess.run(
            [
                "git", "clone", "--quiet", "--no-hardlinks",
                str(remote), str(work),
            ],
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
            env=git_environment(),
        )
        if cloned.returncode:
            raise GateError("TEST_CLONE_FAILED")

        if head(work) != expected_head:
            raise GateError("CLONED_HEAD_MISMATCH")

        if git(work, "status", "--porcelain").stdout.strip():
            raise GateError("UNCLEAN_TEST_WORKTREE")

        if Path(git(
            work, "remote", "get-url", "origin"
        ).stdout.strip()).resolve() != remote:
            raise GateError("UNEXPECTED_TEST_ORIGIN")

        destination = work / TARGET

        if git(work, "ls-tree", expected_head, "--", TARGET).stdout.strip():
            raise GateError("TARGET_ALREADY_EXISTS")
        if any(parent.is_symlink() for parent in destination.parents):
            raise GateError("SYMLINK_TARGET_PARENT")
        if destination.exists() or destination.is_symlink():
            raise GateError("TARGET_ALREADY_EXISTS")

        destination.parent.mkdir(
            parents=True,
            exist_ok=True,
        )
        destination.write_bytes(data)

        git(work, "add", "--", TARGET)
        staged = git(
            work, "diff", "--cached", "--name-only"
        ).stdout.splitlines()

        if staged != [TARGET]:
            raise GateError("UNEXPECTED_STAGED_FILES")

        git(
            work, "config", "user.name",
            "Novena Synthetic Test",
        )
        git(
            work, "config", "user.email",
            "synthetic@localhost.invalid",
        )
        git(
            work, "commit", "-m",
            "Synthetic Bridge Exchange initialization",
        )

        new_head = head(work)

        verify_single_change(work, expected_head, new_head)
        verify_readback(work, new_head, data)

        # Local synthetic transport only: import objects without moving main.
        # update-ref provides exact old-OID CAS, including concurrent rewinds.
        git(remote, "fetch", "--quiet", "--no-write-fetch-head",
            str(work), new_head)
        updated = git(remote, "update-ref", "refs/heads/main",
                      new_head, expected_head, check=False)
        if updated.returncode:
            raise GateError("CONCURRENT_REMOTE_CHANGE")

        verify_readback(remote, new_head, data)

        return {
            "verified": True,
            "head": new_head,
            "baseline_head": expected_head,
            "target": TARGET,
            "verification": "LOCAL_GIT_READBACK_ONLY",
            "sha256": hashlib.sha256(data).hexdigest(),
        }


def production_publish(*args, **kwargs):
    raise GateError("PRODUCTION_PUBLICATION_DISABLED")


def exact_fields(value, fields):
    if type(value) is not dict or set(value) != set(fields):
        raise GateError("MALFORMED_OR_UNEXPECTED_RESPONSE")
    return value


def update_refs_request(repository_id, ref_name, before_oid, after_oid):
    """Build only; no HTTP adapter, credentials or production entry point."""
    if (not isinstance(repository_id, str) or not repository_id.strip()
            or ref_name != "refs/heads/main"):
        raise GateError("INVALID_REPOSITORY_OR_REF")
    for oid in (before_oid, after_oid):
        if not isinstance(oid, str) or not re.fullmatch(r"[0-9a-f]{40}", oid):
            raise GateError("INVALID_COMMIT_OID")
    if before_oid == after_oid:
        raise GateError("UNCHANGED_COMMIT")
    return {
        "query": "mutation($input: UpdateRefsInput!) { updateRefs(input: $input) { clientMutationId } }",
        "variables": {"input": {
            "repositoryId": repository_id,
            "refUpdates": [{"name": ref_name, "beforeOid": before_oid,
                            "afterOid": after_oid, "force": False}],
        }},
    }


def verify_api_commit(record, before_oid, after_oid, data):
    """Strict normalized reader contract, not a raw GitHub response schema.

    Reader must enumerate the complete tree difference (including all pages),
    resolve immutable objects and return raw blob bytes, never decoded text.
    """
    exact_fields(record, ("oid", "parents", "changes"))
    if record["oid"] != after_oid or record["parents"] != [before_oid]:
        raise GateError("UNEXPECTED_COMMIT_PARENT")
    if type(record["changes"]) is not list or len(record["changes"]) != 1:
        raise GateError("UNEXPECTED_COMMIT_CHANGES")
    change = exact_fields(record["changes"][0],
                          ("path", "status", "mode", "type", "bytes"))
    if change != {"path": TARGET, "status": "added", "mode": "100644",
                  "type": "blob", "bytes": data} or type(change["bytes"]) is not bytes:
        raise GateError("UNEXPECTED_TARGET_OR_BYTES")


def synthetic_graphql_publication(client, reader, repository_id, ref_name,
                                  before_oid, after_oid, document):
    """Injected transport model ONLY. No real client/reader is supplied here.

    reader.head(repo, ref) -> OID; reader.commit(repo, oid) -> normalized record.
    client(request) -> decoded GraphQL response. Exceptions propagate; no retry.
    reader is independent of the mutation response and commit preparation.
    """
    data = payload(document)
    request = update_refs_request(repository_id, ref_name, before_oid, after_oid)
    verify_api_commit(reader.commit(repository_id, after_oid), before_oid, after_oid, data)
    if reader.head(repository_id, ref_name) != before_oid:
        raise GateError("STALE_REMOTE_HEAD")
    response = exact_fields(client(request), ("data",))
    result = exact_fields(response["data"], ("updateRefs",))
    mutation = exact_fields(result["updateRefs"], ("clientMutationId",))
    if mutation["clientMutationId"] is not None:
        raise GateError("UNEXPECTED_MUTATION_ID")
    # updateRefs does not supply authoritative resulting HEAD in this selection.
    # Failure from this point can mean the update happened: never retry/rollback.
    if reader.head(repository_id, ref_name) != after_oid:
        raise GateError("RESULTING_HEAD_MISMATCH")
    verify_api_commit(reader.commit(repository_id, after_oid), before_oid, after_oid, data)
    if reader.head(repository_id, ref_name) != after_oid:
        raise GateError("RESULTING_HEAD_MISMATCH")
    return {"verification": "SYNTHETIC_API_READBACK_ONLY", "baseline_head": before_oid,
            "head": after_oid, "repository_id": repository_id, "ref": ref_name,
            "target": TARGET, "sha256": hashlib.sha256(data).hexdigest()}


def expect_rejection(label, callback):
    try:
        callback()
    except (GateError, planner.PolicyError, jsonschema.ValidationError):
        print(label + "=REJECTED")
    else:
        raise AssertionError(label + "_NOT_REJECTED")


def self_test():
    document = json.loads(
        PILOT.read_text(encoding="utf-8")
    )
    candidate_bytes = payload(document)

    assert PRODUCTION_ENABLED is False

    expect_rejection(
        "PRODUCTION_CALL",
        lambda: production_publish(),
    )

    with tempfile.TemporaryDirectory(
        prefix="novena-publish-gate-144s-"
    ) as temporary:
        root = Path(temporary).resolve()
        remote = root / "archive.git"
        seed = root / "seed"

        init = subprocess.run(
            [
                "git", "init", "-q", "--bare",
                "--initial-branch=main", str(remote),
            ],
            capture_output=True,
            text=True,
            check=False,
        )
        if init.returncode:
            raise GateError("TEST_ARCHIVE_INIT_FAILED")

        (remote / "NOVENA_SYNTHETIC_TEST_ONLY").write_text(
            "Local synthetic test repository only\n"
        )

        cloned = subprocess.run(
            ["git", "clone", "--quiet", str(remote), str(seed)],
            capture_output=True,
            text=True,
            check=False,
            env=git_environment(),
        )
        if cloned.returncode:
            raise GateError("SEED_CLONE_FAILED")

        git(
            seed, "config", "user.name",
            "Novena Synthetic Test",
        )
        git(
            seed, "config", "user.email",
            "synthetic@localhost.invalid",
        )
        (seed / "README.txt").write_text(
            "Synthetic Git archive\n"
        )

        git(seed, "add", "README.txt")
        git(seed, "commit", "-m", "Synthetic baseline")
        git(seed, "push", "origin", "main")

        baseline = head(remote)

        expect_rejection(
            "WRONG_EXPECTED_HEAD",
            lambda: create_initial_index(
                remote, "0" * 40, document, root
            ),
        )
        expect_rejection(
            "OUTSIDE_TEST_ROOT",
            lambda: create_initial_index(
                remote, baseline, document,
                root / "unrelated",
            ),
        )

        false_claim = copy.deepcopy(document)
        false_claim["publication"]["state"] = "CONFIRMED"

        expect_rejection(
            "FALSE_PUBLICATION_CLAIM",
            lambda: create_initial_index(
                remote, baseline, false_claim, root
            ),
        )

        promoted = copy.deepcopy(document)
        promoted["last_verified"] = {
            "block_id": "SYNTHETIC",
            "git_head": "a" * 40,
            "qualified_at_utc": "2026-10-25T12:00:00Z",
            "qualification": "PASS",
            "evidence_files": ["synthetic.txt"],
        }
        expect_rejection(
            "UNAPPROVED_CHECKPOINT",
            lambda: create_initial_index(
                remote, baseline, promoted, root
            ),
        )

        assert head(remote) == baseline

        result = create_initial_index(
            remote, baseline, document, root
        )

        assert result["verified"]
        assert result["head"] != baseline
        assert result["sha256"] == hashlib.sha256(
            candidate_bytes
        ).hexdigest()

        print("LOCAL_INITIAL_PUBLICATION=PASS")
        print("REMOTE_READBACK=PASS")

        expect_rejection(
            "DUPLICATE_INITIAL_PUBLICATION",
            lambda: create_initial_index(
                remote, result["head"], document, root
            ),
        )
        expect_rejection(
            "STALE_BASE_AFTER_PUBLICATION",
            lambda: create_initial_index(
                remote, baseline, document, root
            ),
        )

        assert head(remote) == result["head"]
        print("REMOTE_HEAD_PRESERVED=PASS")
        print("SYNTHETIC_ARCHIVE_ONLY=YES")

    print("TEMPORARY_ARCHIVE_REMOVED=PASS")
    print("PRODUCTION_PUBLICATION=DISABLED")
    print("SELF_TEST=PASS")


def main():
    parser = argparse.ArgumentParser(
        description="Novena Bridge: local publication gate"
    )
    parser.add_argument(
        "--self-test", action="store_true", required=True
    )
    args = parser.parse_args()
    if args.self_test:
        self_test()


if __name__ == "__main__":
    main()
