#!/usr/bin/env python3
"""Isolated, read-only GitHub REST response adapter.

No credentials, HTTP client, mutation or publication entry point.
The caller injects a GET function for explicit API paths.
"""

import base64
import binascii
import hashlib
import re

ARCHIVE = "loomit77/project-block-logs"
REF = "refs/heads/main"


class ReadbackError(Exception):
    pass


def oid(value):
    if not isinstance(value, str) or not re.fullmatch(
        r"[0-9a-f]{40}", value
    ):
        raise ReadbackError("INVALID_OID")
    return value


def decode_blob(record):
    if not isinstance(record, dict):
        raise ReadbackError("INVALID_BLOB")
    if record.get("encoding") != "base64":
        raise ReadbackError("INVALID_ENCODING")
    if record.get("size") is None or type(record["size"]) is not int:
        raise ReadbackError("INVALID_SIZE")
    if not isinstance(record.get("content"), str):
        raise ReadbackError("INVALID_CONTENT")

    try:
        raw = base64.b64decode(
            "".join(record["content"].split()),
            validate=True,
        )
    except (ValueError, binascii.Error):
        raise ReadbackError("INVALID_BASE64") from None

    if len(raw) != record["size"]:
        raise ReadbackError("SIZE_MISMATCH")
    if len(raw) > 1024 * 1024:
        raise ReadbackError("BLOB_TOO_LARGE")
    return raw


def git_blob_oid(data):
    """Calculate a Git SHA-1 blob object identity."""
    if type(data) is not bytes:
        raise ReadbackError("INVALID_BLOB_BYTES")
    header = b"blob " + str(len(data)).encode("ascii") + b"\0"
    return hashlib.sha1(header + data).hexdigest()


class RestReader:
    def __init__(self, get_json):
        if not callable(get_json):
            raise ReadbackError("INVALID_TRANSPORT")
        self.get_json = get_json

    def get(self, path):
        response = self.get_json(path)
        if not isinstance(response, dict):
            raise ReadbackError("INVALID_RESPONSE")
        return response

    def repository(self):
        result = self.get("repos/" + ARCHIVE)
        if result.get("full_name") != ARCHIVE:
            raise ReadbackError("WRONG_REPOSITORY")
        node_id = result.get("node_id")
        if not isinstance(node_id, str) or not node_id:
            raise ReadbackError("MISSING_REPOSITORY_ID")
        return node_id

    def head(self):
        result = self.get(
            "repos/" + ARCHIVE + "/git/ref/heads/main"
        )
        if result.get("ref") != REF:
            raise ReadbackError("WRONG_REF")
        obj = result.get("object")
        if not isinstance(obj, dict) or obj.get("type") != "commit":
            raise ReadbackError("INVALID_REF_OBJECT")
        return oid(obj.get("sha"))

    def commit(self, commit_oid):
        commit_oid = oid(commit_oid)
        result = self.get(
            "repos/" + ARCHIVE + "/git/commits/" + commit_oid
        )
        if result.get("sha") != commit_oid:
            raise ReadbackError("COMMIT_MISMATCH")
        parents = result.get("parents")
        if not isinstance(parents, list):
            raise ReadbackError("INVALID_PARENTS")
        parent_oids = [
            oid(parent.get("sha"))
            for parent in parents
            if isinstance(parent, dict)
        ]
        if len(parent_oids) != len(parents):
            raise ReadbackError("INVALID_PARENTS")
        tree = result.get("tree")
        if not isinstance(tree, dict):
            raise ReadbackError("INVALID_TREE")
        return {
            "oid": commit_oid,
            "parents": parent_oids,
            "tree": oid(tree.get("sha")),
        }

    def tree(self, tree_oid):
        tree_oid = oid(tree_oid)
        result = self.get(
            "repos/" + ARCHIVE + "/git/trees/"
            + tree_oid + "?recursive=1"
        )
        if result.get("sha") != tree_oid:
            raise ReadbackError("TREE_MISMATCH")
        if result.get("truncated") is not False:
            raise ReadbackError("INCOMPLETE_TREE")
        entries = result.get("tree")
        if not isinstance(entries, list):
            raise ReadbackError("INVALID_TREE_ENTRIES")

        files = {}
        directories = set()
        all_paths = set()

        for entry in entries:
            if not isinstance(entry, dict):
                raise ReadbackError("INVALID_TREE_ENTRY")
            path = entry.get("path")
            kind = entry.get("type")
            mode = entry.get("mode")
            if not isinstance(path, str) or not path:
                raise ReadbackError("INVALID_TREE_PATH")
            if path.startswith("/") or any(
                part in ("", ".", "..") for part in path.split("/")
            ):
                raise ReadbackError("INVALID_TREE_PATH")
            if path in all_paths:
                raise ReadbackError("DUPLICATE_TREE_PATH")
            all_paths.add(path)

            entry_oid = oid(entry.get("sha"))

            if kind == "tree":
                if mode != "040000":
                    raise ReadbackError("INVALID_DIRECTORY_MODE")
                directories.add(path)
                continue

            if kind != "blob":
                raise ReadbackError("INVALID_TREE_TYPE")

            if mode not in ("100644", "100755", "120000"):
                raise ReadbackError("INVALID_BLOB_MODE")

            files[path] = (kind, mode, entry_oid)

        for path in all_paths:
            parts = path.split("/")
            for index in range(1, len(parts)):
                parent = "/".join(parts[:index])
                if parent not in directories:
                    raise ReadbackError("MISSING_PARENT_TREE")

        for directory in directories:
            if directory in files:
                raise ReadbackError("TREE_FILE_COLLISION")

        return files

    def blob(self, blob_oid):
        blob_oid = oid(blob_oid)
        result = self.get(
            "repos/" + ARCHIVE + "/git/blobs/" + blob_oid
        )
        if result.get("sha") != blob_oid:
            raise ReadbackError("BLOB_ID_MISMATCH")
        raw = decode_blob(result)
        if git_blob_oid(raw) != blob_oid:
            raise ReadbackError("BLOB_HASH_MISMATCH")
        return raw


def verify_initial_index(reader, before, after, target, expected):
    """Verify a complete initial commit using independent GET responses."""
    before, after = oid(before), oid(after)
    if before == after:
        raise ReadbackError("UNCHANGED_COMMIT")
    if reader.head() != after:
        raise ReadbackError("HEAD_MISMATCH")

    old = reader.commit(before)
    new = reader.commit(after)
    if new["parents"] != [before]:
        raise ReadbackError("PARENT_MISMATCH")

    old_tree = reader.tree(old["tree"])
    new_tree = reader.tree(new["tree"])

    if target in old_tree:
        raise ReadbackError("TARGET_ALREADY_EXISTS")

    added = set(new_tree) - set(old_tree)
    removed = set(old_tree) - set(new_tree)
    changed = {
        path for path in set(old_tree) & set(new_tree)
        if old_tree[path] != new_tree[path]
    }
    if added != {target} or removed or changed:
        raise ReadbackError("UNEXPECTED_TREE_CHANGES")

    kind, mode, blob_oid = new_tree[target]
    if (kind, mode) != ("blob", "100644"):
        raise ReadbackError("INVALID_TARGET_MODE")

    raw = reader.blob(blob_oid)
    if raw != expected:
        raise ReadbackError("CONTENT_MISMATCH")
    if reader.head() != after:
        raise ReadbackError("LATE_HEAD_CHANGE")

    return {
        "verification": "INDEPENDENT_REST_READBACK",
        "commit": after,
        "sha256": hashlib.sha256(raw).hexdigest(),
    }


def self_test():
    import copy

    before = "a" * 40
    after = "b" * 40
    old_tree = "c" * 40
    new_tree = "d" * 40
    target = "novena/status/NOVENA-CURRENT-STATE.json"
    data = b"{\"last_verified\":null}\n"
    blob_oid = git_blob_oid(data)

    prefix = "repos/" + ARCHIVE
    responses = {
        prefix: {
            "full_name": ARCHIVE,
            "node_id": "SYNTHETIC_NODE_ID",
        },
        prefix + "/git/ref/heads/main": {
            "ref": REF,
            "object": {"type": "commit", "sha": after},
        },
        prefix + "/git/commits/" + before: {
            "sha": before,
            "parents": [],
            "tree": {"sha": old_tree},
        },
        prefix + "/git/commits/" + after: {
            "sha": after,
            "parents": [{"sha": before}],
            "tree": {"sha": new_tree},
        },
        prefix + "/git/trees/" + old_tree + "?recursive=1": {
            "sha": old_tree,
            "truncated": False,
            "tree": [],
        },
        prefix + "/git/trees/" + new_tree + "?recursive=1": {
            "sha": new_tree,
            "truncated": False,
            "tree": [
                {
                    "path": "novena",
                    "type": "tree",
                    "mode": "040000",
                    "sha": "1" * 40,
                },
                {
                    "path": "novena/status",
                    "type": "tree",
                    "mode": "040000",
                    "sha": "2" * 40,
                },
                {
                    "path": target,
                "type": "blob",
                "mode": "100644",
                "sha": blob_oid,
            }],
        },
        prefix + "/git/blobs/" + blob_oid: {
            "sha": blob_oid,
            "encoding": "base64",
            "size": len(data),
            "content": base64.b64encode(data).decode("ascii"),
        },
    }

    def verify(records, expected=data):
        reader = RestReader(lambda path: copy.deepcopy(records[path]))
        assert reader.repository() == "SYNTHETIC_NODE_ID"
        return verify_initial_index(
            reader, before, after, target, expected
        )

    assert verify(responses)["sha256"] == hashlib.sha256(
        data
    ).hexdigest()

    def rejects(records):
        try:
            verify(records)
        except ReadbackError:
            return
        raise AssertionError("NEGATIVE_TEST_NOT_REJECTED")

    wrong_parent = copy.deepcopy(responses)
    wrong_parent[prefix + "/git/commits/" + after]["parents"] = []
    rejects(wrong_parent)

    truncated = copy.deepcopy(responses)
    truncated[prefix + "/git/trees/" + new_tree + "?recursive=1"][
        "truncated"
    ] = True
    rejects(truncated)

    extra = copy.deepcopy(responses)
    extra[prefix + "/git/trees/" + new_tree + "?recursive=1"][
        "tree"
    ].append({
        "path": "unexpected",
        "type": "blob",
        "mode": "100644",
        "sha": blob_oid,
    })
    rejects(extra)

    bad_blob = copy.deepcopy(responses)
    bad_blob[prefix + "/git/blobs/" + blob_oid]["content"] = "!!!"
    rejects(bad_blob)

    wrong_ref = copy.deepcopy(responses)
    wrong_ref[prefix + "/git/ref/heads/main"]["object"]["sha"] = before
    rejects(wrong_ref)

    try:
        verify(responses, b"wrong")
    except ReadbackError:
        pass
    else:
        raise AssertionError("BYTE_MISMATCH_NOT_REJECTED")

    tampered = copy.deepcopy(responses)
    changed_bytes = b"X" + data[1:]
    tampered[prefix + "/git/blobs/" + blob_oid]["content"] = (
        base64.b64encode(changed_bytes).decode("ascii")
    )
    rejects(tampered)

    orphan = copy.deepcopy(responses)
    key = prefix + "/git/trees/" + new_tree + "?recursive=1"
    orphan[key]["tree"] = [
        item for item in orphan[key]["tree"]
        if item["path"] != "novena/status"
    ]
    rejects(orphan)

    invalid_mode = copy.deepcopy(responses)
    invalid_mode[key]["tree"][-1]["mode"] = "999999"
    rejects(invalid_mode)

    duplicate = copy.deepcopy(responses)
    duplicate[key]["tree"].append(
        copy.deepcopy(duplicate[key]["tree"][-1])
    )
    rejects(duplicate)

    assert git_blob_oid(data) == blob_oid

    print("GIT_BLOB_SHA1=PASS")
    print("TREE_STRUCTURE_VALIDATION=PASS")
    print("ADDITIONAL_NEGATIVE_CASES=4")
    print("SYNTHETIC_POSITIVE=PASS")
    print("SYNTHETIC_NEGATIVE_CASES=6")
    print("READBACK_ADAPTER_SELFTEST=PASS")
    print("HTTP_REQUESTS=0")
    print("GITHUB_MUTATIONS=0")
    print("PRODUCTION_PUBLICATION=BLOCKED")


if __name__ == "__main__":
    self_test()
