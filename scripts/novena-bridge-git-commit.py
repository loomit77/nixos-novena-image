#!/usr/bin/env python3
"""Strict verification of raw Git SHA-1 commit objects.

Only synthetic/read-only use. No HTTP, publication or object writes.

This verifier deliberately supports a narrow commit-header profile.
Unsupported valid Git commit variants are rejected, not normalized.
"""

import hashlib
import re
import subprocess


class CommitError(Exception):
    pass


def validate_oid(value):
    if type(value) is not str or not re.fullmatch(
        r"[0-9a-f]{40}", value
    ):
        raise CommitError("INVALID_OID")
    return value


def object_hash(kind, body):
    if type(body) is not bytes:
        raise CommitError("INVALID_OBJECT_BYTES")
    header = kind.encode("ascii") + b" " + str(
        len(body)
    ).encode("ascii") + b"\0"
    return hashlib.sha1(header + body).hexdigest()


def inspect_commit(raw):
    """Read one strictly constrained raw commit, preserving bytes."""
    if type(raw) is not bytes:
        raise CommitError("INVALID_RAW_COMMIT")
    if not raw or len(raw) > 1024 * 1024:
        raise CommitError("INVALID_COMMIT_SIZE")
    if b"\0" in raw or b"\r" in raw:
        raise CommitError("UNSUPPORTED_COMMIT_BYTES")

    if b"\n\n" not in raw:
        raise CommitError("MISSING_HEADER_SEPARATOR")

    headers, message = raw.split(b"\n\n", 1)
    lines = headers.split(b"\n")

    # Strict profile: tree, exactly one parent, author, committer.
    # No multiline signatures, encoding, extra parents or headers.
    if len(lines) != 4:
        raise CommitError("UNEXPECTED_COMMIT_HEADERS")

    names = (b"tree ", b"parent ", b"author ", b"committer ")
    for line, prefix in zip(lines, names):
        if not line.startswith(prefix):
            raise CommitError("UNEXPECTED_HEADER_ORDER")

    try:
        tree = lines[0][5:].decode("ascii")
        parent = lines[1][7:].decode("ascii")
    except UnicodeDecodeError:
        raise CommitError("INVALID_HEADER_OID") from None

    validate_oid(tree)
    validate_oid(parent)

    # Reject malformed identity lines without rebuilding them.
    identity_pattern = (
        rb"[^\n<>]+ <[^<>\n]+> [0-9]+ [+-][0-9]{4}"
    )

    for line, prefix in (
        (lines[2], b"author "),
        (lines[3], b"committer "),
    ):
        if not re.fullmatch(identity_pattern, line[len(prefix):]):
            raise CommitError("INVALID_IDENTITY_HEADER")

    if not message:
        raise CommitError("EMPTY_COMMIT_MESSAGE")

    return {
        "tree": tree,
        "parents": [parent],
        "message_size": len(message),
    }


def verify_commit(raw, expected_oid, expected_parent, expected_tree):
    """Verify raw-object identity and the expected tree/parent links."""
    expected_oid = validate_oid(expected_oid)
    expected_parent = validate_oid(expected_parent)
    expected_tree = validate_oid(expected_tree)

    record = inspect_commit(raw)

    actual_oid = object_hash("commit", raw)
    if actual_oid != expected_oid:
        raise CommitError("COMMIT_HASH_MISMATCH")

    if record["parents"] != [expected_parent]:
        raise CommitError("UNEXPECTED_COMMIT_PARENT")

    if record["tree"] != expected_tree:
        raise CommitError("UNEXPECTED_COMMIT_TREE")

    return {
        "oid": actual_oid,
        "tree": expected_tree,
        "parents": [expected_parent],
        "verification": "RAW_GIT_SHA1_COMMIT",
    }


def self_test():
    tree = "a" * 40
    parent = "b" * 40

    raw = (
        b"tree " + tree.encode("ascii") + b"\n"
        + b"parent " + parent.encode("ascii") + b"\n"
        + b"author Novena Test <test@localhost.invalid> 1760000000 +0000\n"
        + b"committer Novena Test <test@localhost.invalid> 1760000000 +0000\n"
        + b"\n"
        + b"Synthetic Bridge commit\n"
    )

    expected_oid = object_hash("commit", raw)

    result = verify_commit(
        raw, expected_oid, parent, tree
    )

    assert result["oid"] == expected_oid

    print("RAW_COMMIT_POSITIVE=PASS")

    # Independent Git implementation computes the same identity.
    try:
        process = subprocess.run(
            ["git", "hash-object", "-t", "commit", "--stdin"],
            input=raw,
            capture_output=True,
            check=False,
            timeout=10,
        )
    except (OSError, subprocess.TimeoutExpired):
        raise CommitError("GIT_REFERENCE_UNAVAILABLE") from None

    if process.returncode != 0:
        raise CommitError("GIT_REFERENCE_FAILED")

    reference_oid = process.stdout.decode("ascii").strip()
    if reference_oid != expected_oid:
        raise CommitError("GIT_REFERENCE_HASH_MISMATCH")

    print("GIT_REFERENCE_HASH=PASS")

    negative_count = 0

    def rejects(name, callback):
        nonlocal negative_count
        try:
            callback()
        except CommitError:
            negative_count += 1
            print("REJECTED_" + name + "=PASS")
        else:
            raise AssertionError("UNEXPECTED_ACCEPTANCE:" + name)

    rejects(
        "TAMPERED_MESSAGE",
        lambda: verify_commit(
            raw + b"tampered", expected_oid, parent, tree
        ),
    )

    rejects(
        "WRONG_EXPECTED_PARENT",
        lambda: verify_commit(
            raw, expected_oid, "c" * 40, tree
        ),
    )

    rejects(
        "WRONG_EXPECTED_TREE",
        lambda: verify_commit(
            raw, expected_oid, parent, "d" * 40
        ),
    )

    rejects(
        "EXTRA_PARENT",
        lambda: inspect_commit(
            raw.replace(
                b"author ",
                b"parent " + ("c" * 40).encode("ascii")
                + b"\nauthor ",
                1,
            )
        ),
    )

    rejects(
        "EXTRA_HEADER",
        lambda: inspect_commit(
            raw.replace(
                b"\n\nSynthetic",
                b"\nencoding UTF-8\n\nSynthetic",
                1,
            )
        ),
    )

    rejects(
        "WRONG_HEADER_ORDER",
        lambda: inspect_commit(
            raw.replace(
                b"tree " + tree.encode("ascii"),
                b"parent " + tree.encode("ascii"),
                1,
            )
        ),
    )

    rejects(
        "MISSING_SEPARATOR",
        lambda: inspect_commit(
            raw.replace(b"\n\nSynthetic", b"\nSynthetic", 1)
        ),
    )

    rejects(
        "NUL_IN_COMMIT",
        lambda: inspect_commit(raw + b"\0"),
    )

    rejects(
        "INVALID_OBJECT_ID",
        lambda: verify_commit(raw, "invalid", parent, tree),
    )

    assert negative_count == 9

    print("COMMIT_NEGATIVE_CASES=9")
    print("COMMIT_SHA1_IDENTITY=PASS")
    print("COMMIT_PARENT_LINK=PASS")
    print("COMMIT_TREE_LINK=PASS")
    print("UNSUPPORTED_COMMIT_PROFILES=FAIL_CLOSED")
    print("RAW_GITHUB_COMMIT_BYTES=NOT_AVAILABLE")
    print("REAL_HTTP_REQUESTS=0")
    print("GITHUB_MUTATIONS=0")
    print("PRODUCTION_PUBLICATION=BLOCKED")
    print("COMMIT_MODULE_SELFTEST=PASS")


if __name__ == "__main__":
    self_test()
