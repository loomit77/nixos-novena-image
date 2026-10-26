#!/usr/bin/env python3
"""Isolated Novena Bridge REST readback integration.

No HTTP client, credentials, mutation or publication entry point.
Only an explicitly injected read-only response provider is accepted.
"""

import base64
import copy
import hashlib
import importlib.util
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(
        name, ROOT / filename
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


api = load("novena_bridge_api_187", "novena-bridge-api-readback.py")
git_tree = load("novena_bridge_tree_187", "novena-bridge-git-tree.py")

ARCHIVE = "loomit77/project-block-logs"
TARGET = "novena/status/NOVENA-CURRENT-STATE.json"


class VerificationError(Exception):
    pass


def verify_initial_index(get_json, repository_id, before, after, expected):
    """Verify the remote object graph with independent read-only responses."""

    before = api.oid(before)
    after = api.oid(after)

    if before == after:
        raise VerificationError("UNCHANGED_COMMIT")

    if type(expected) is not bytes:
        raise VerificationError("INVALID_EXPECTED_BYTES")

    if len(expected) > 1024 * 1024:
        raise VerificationError("PAYLOAD_TOO_LARGE")

    reader = api.RestReader(get_json)

    if reader.repository() != repository_id:
        raise VerificationError("REPOSITORY_ID_MISMATCH")

    if reader.head() != after:
        raise VerificationError("INITIAL_HEAD_MISMATCH")

    previous = reader.commit(before)
    current = reader.commit(after)

    if current["parents"] != [before]:
        raise VerificationError("COMMIT_PARENT_MISMATCH")

    prefix = "repos/" + ARCHIVE + "/git/trees/"

    def get_tree(tree_oid):
        return reader.get(prefix + git_tree.oid(tree_oid))

    verified = git_tree.VerifiedTrees(get_tree)

    # Each tree and its referenced subtrees must match their Git SHA-1 IDs.
    old_files = verified.flatten(previous["tree"])
    new_files = verified.flatten(current["tree"])

    if TARGET in old_files:
        raise VerificationError("TARGET_ALREADY_EXISTS")

    if set(new_files) - set(old_files) != {TARGET}:
        raise VerificationError("UNEXPECTED_ADDITIONS")

    if set(old_files) - set(new_files):
        raise VerificationError("UNEXPECTED_REMOVALS")

    for name, value in old_files.items():
        if new_files.get(name) != value:
            raise VerificationError("UNEXPECTED_MODIFICATION")

    kind, mode, blob_oid = new_files[TARGET]

    if (kind, mode) != ("blob", "100644"):
        raise VerificationError("INVALID_TARGET_MODE")

    actual = reader.blob(blob_oid)

    if actual != expected:
        raise VerificationError("PAYLOAD_MISMATCH")

    if reader.head() != after:
        raise VerificationError("FINAL_HEAD_MISMATCH")

    return {
        "verification": "SYNTHETIC_INTEGRATED_READBACK",
        "repository": ARCHIVE,
        "head": after,
        "target": TARGET,
        "sha256": hashlib.sha256(actual).hexdigest(),
        "commit_object_hash_verified": False,
        "publication_performed": False,
    }


def self_test():
    data = b'{"last_verified":null}\n'
    blob_oid = git_tree.object_sha("blob", data)

    trees = {}

    def add_tree(entries):
        sha = git_tree.tree_sha(entries)
        trees[sha] = {
            "sha": sha,
            "truncated": False,
            "tree": copy.deepcopy(entries),
        }
        return sha

    old_root = add_tree([])

    leaf = add_tree([{
        "path": "NOVENA-CURRENT-STATE.json",
        "mode": "100644",
        "type": "blob",
        "sha": blob_oid,
    }])

    middle = add_tree([{
        "path": "status",
        "mode": "040000",
        "type": "tree",
        "sha": leaf,
    }])

    new_root = add_tree([{
        "path": "novena",
        "mode": "040000",
        "type": "tree",
        "sha": middle,
    }])

    before = "a" * 40
    after = "b" * 40
    repository_id = "SYNTHETIC_REPOSITORY_ID"

    prefix = "repos/" + ARCHIVE

    responses = {
        prefix: {
            "full_name": ARCHIVE,
            "node_id": repository_id,
        },
        prefix + "/git/ref/heads/main": {
            "ref": "refs/heads/main",
            "object": {"type": "commit", "sha": after},
        },
        prefix + "/git/commits/" + before: {
            "sha": before,
            "parents": [],
            "tree": {"sha": old_root},
        },
        prefix + "/git/commits/" + after: {
            "sha": after,
            "parents": [{"sha": before}],
            "tree": {"sha": new_root},
        },
        prefix + "/git/blobs/" + blob_oid: {
            "sha": blob_oid,
            "encoding": "base64",
            "size": len(data),
            "content": base64.b64encode(data).decode("ascii"),
        },
    }

    for sha, record in trees.items():
        responses[prefix + "/git/trees/" + sha] = copy.deepcopy(record)

    def run(source, expected=data, late_head=None):
        source = copy.deepcopy(source)
        head_path = prefix + "/git/ref/heads/main"
        head_reads = 0

        def get(route):
            nonlocal head_reads
            if route not in source:
                raise VerificationError("MISSING_RESPONSE")
            response = copy.deepcopy(source[route])
            if route == head_path:
                head_reads += 1
                if head_reads == 2 and late_head is not None:
                    response["object"]["sha"] = late_head
            return response

        return verify_initial_index(
            get, repository_id, before, after, expected
        )

    result = run(responses)
    assert result["sha256"] == hashlib.sha256(data).hexdigest()
    assert result["commit_object_hash_verified"] is False
    assert result["publication_performed"] is False
    print("INTEGRATED_POSITIVE=PASS")

    count = 0

    def rejects(name, source, expected=data, late_head=None):
        nonlocal count
        try:
            run(source, expected, late_head)
        except (VerificationError, api.ReadbackError, git_tree.TreeError):
            count += 1
            print("REJECTED_" + name + "=PASS")
        else:
            raise AssertionError("UNEXPECTED_ACCEPTANCE:" + name)

    wrong_parent = copy.deepcopy(responses)
    wrong_parent[prefix + "/git/commits/" + after]["parents"] = []
    rejects("WRONG_PARENT", wrong_parent)

    missing_tree = copy.deepcopy(responses)
    del missing_tree[prefix + "/git/trees/" + middle]
    rejects("MISSING_SUBTREE", missing_tree)

    tampered_tree = copy.deepcopy(responses)
    tampered_tree[prefix + "/git/trees/" + leaf]["tree"] = []
    rejects("TREE_HASH_MISMATCH", tampered_tree)

    truncated = copy.deepcopy(responses)
    truncated[prefix + "/git/trees/" + leaf]["truncated"] = True
    rejects("TRUNCATED_TREE", truncated)

    bad_blob = copy.deepcopy(responses)
    changed = b"X" + data[1:]
    bad_blob[prefix + "/git/blobs/" + blob_oid].update({
        "size": len(changed),
        "content": base64.b64encode(changed).decode("ascii"),
    })
    rejects("BLOB_HASH_MISMATCH", bad_blob)

    wrong_repo = copy.deepcopy(responses)
    wrong_repo[prefix]["node_id"] = "OTHER_REPOSITORY"
    rejects("WRONG_REPOSITORY", wrong_repo)

    rejects("WRONG_PAYLOAD", responses, b"wrong")
    rejects("LATE_HEAD_CHANGE", responses, late_head="c" * 40)

    assert count == 8
    print("INTEGRATED_NEGATIVE_CASES=8")
    print("TREE_SHA1_VERIFICATION=PASS")
    print("BLOB_SHA1_VERIFICATION=PASS")
    print("COMMIT_OBJECT_HASH_VERIFICATION=NOT_IMPLEMENTED")
    print("REAL_HTTP_REQUESTS=0")
    print("GITHUB_MUTATIONS=0")
    print("PRODUCTION_PUBLICATION=BLOCKED")
    print("INTEGRATION_SELFTEST=PASS")


if __name__ == "__main__":
    self_test()
