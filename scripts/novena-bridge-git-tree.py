#!/usr/bin/env python3
"""Independent verification of Git SHA-1 tree objects.

No HTTP, credentials, filesystem mutation or publishing.
The caller supplies a read-only nonrecursive tree provider.
"""

import copy
import hashlib
import re


class TreeError(Exception):
    pass


def oid(value):
    if type(value) is not str or not re.fullmatch(
        r"[0-9a-f]{40}", value
    ):
        raise TreeError("INVALID_OBJECT_ID")
    return value


def object_sha(kind, body):
    header = kind.encode("ascii") + b" " + str(
        len(body)
    ).encode("ascii") + b"\0"
    return hashlib.sha1(header + body).hexdigest()


def canonical_entries(entries):
    """Reconstruct a Git tree object from complete direct entries."""
    if type(entries) is not list:
        raise TreeError("INVALID_ENTRIES")

    prepared = []
    names = set()

    modes = {
        ("blob", "100644"): b"100644",
        ("blob", "100755"): b"100755",
        ("blob", "120000"): b"120000",
        ("tree", "040000"): b"40000",
        ("commit", "160000"): b"160000",
    }

    for entry in entries:
        if type(entry) is not dict or set(entry) != {
            "path", "mode", "type", "sha"
        }:
            raise TreeError("INVALID_ENTRY_FIELDS")

        name = entry["path"]
        if (type(name) is not str or not name
                or name in (".", "..")
                or "/" in name or "\0" in name
                or name in names):
            raise TreeError("INVALID_OR_DUPLICATE_NAME")

        try:
            raw_name = name.encode("utf-8")
        except UnicodeError:
            raise TreeError("INVALID_NAME_ENCODING") from None

        # Non-UTF-8 Git paths are deliberately unsupported
        # until raw-name handling is implemented.
        if raw_name.decode("utf-8") != name:
            raise TreeError("NAME_ENCODING_MISMATCH")

        key = (entry["type"], entry["mode"])
        if key not in modes:
            raise TreeError("INVALID_TYPE_OR_MODE")

        names.add(name)
        object_id = bytes.fromhex(oid(entry["sha"]))
        mode = modes[key]

        # Git sorting appends "/" to directory names.
        sort_key = raw_name + (
            b"/" if entry["type"] == "tree" else b""
        )
        record = mode + b" " + raw_name + b"\0" + object_id
        prepared.append((sort_key, record))

    prepared.sort(key=lambda item: item[0])
    return b"".join(record for _, record in prepared)


def tree_sha(entries):
    return object_sha("tree", canonical_entries(entries))


class VerifiedTrees:
    """Verify every reachable tree against its Git object ID."""

    def __init__(self, read_tree, max_trees=4096):
        if not callable(read_tree):
            raise TreeError("INVALID_READER")
        self.read_tree = read_tree
        self.max_trees = max_trees

    def flatten(self, root_oid):
        result = {}
        visited = set()
        active = set()

        def walk(current_oid, prefix):
            current_oid = oid(current_oid)

            if current_oid in active:
                raise TreeError("CYCLIC_TREE")

            if len(visited) >= self.max_trees:
                raise TreeError("TREE_LIMIT_EXCEEDED")

            visited.add(current_oid)
            active.add(current_oid)

            try:
                response = self.read_tree(current_oid)

                if type(response) is not dict:
                    raise TreeError("INVALID_RESPONSE")

                if response.get("sha") != current_oid:
                    raise TreeError("RESPONSE_OID_MISMATCH")

                if response.get("truncated") is not False:
                    raise TreeError("INCOMPLETE_TREE")

                entries = response.get("tree")

                if tree_sha(entries) != current_oid:
                    raise TreeError("TREE_HASH_MISMATCH")

                for entry in entries:
                    path = prefix + entry["path"]

                    if entry["type"] == "tree":
                        walk(entry["sha"], path + "/")
                    else:
                        if path in result:
                            raise TreeError("DUPLICATE_PATH")
                        result[path] = (
                            entry["type"],
                            entry["mode"],
                            entry["sha"],
                        )
            finally:
                active.remove(current_oid)

        walk(root_oid, "")
        return result


def verify_single_addition(reader, old_oid, new_oid, target, blob_oid):
    old = reader.flatten(old_oid)
    new = reader.flatten(new_oid)

    expected = ("blob", "100644", oid(blob_oid))

    if target in old:
        raise TreeError("TARGET_ALREADY_EXISTS")

    if new.get(target) != expected:
        raise TreeError("INVALID_TARGET")

    if set(new) - set(old) != {target}:
        raise TreeError("UNEXPECTED_ADDITION")

    if set(old) - set(new):
        raise TreeError("UNEXPECTED_REMOVAL")

    for name, entry in old.items():
        if new.get(name) != entry:
            raise TreeError("UNEXPECTED_MODIFICATION")

    return {
        "tree_verification": "PASS",
        "target": target,
        "blob_oid": blob_oid,
    }


def self_test():
    target = "novena/status/NOVENA-CURRENT-STATE.json"
    data = b'{"last_verified":null}\n'
    blob = object_sha("blob", data)

    objects = {}

    def put(entries):
        value = tree_sha(entries)
        objects[value] = {
            "sha": value,
            "truncated": False,
            "tree": copy.deepcopy(entries),
        }
        return value

    empty = put([])

    leaf = put([{
        "path": "NOVENA-CURRENT-STATE.json",
        "mode": "100644",
        "type": "blob",
        "sha": blob,
    }])

    status = put([{
        "path": "status",
        "mode": "040000",
        "type": "tree",
        "sha": leaf,
    }])

    root = put([{
        "path": "novena",
        "mode": "040000",
        "type": "tree",
        "sha": status,
    }])

    def reader(source):
        return VerifiedTrees(
            lambda key: copy.deepcopy(source[key])
        )

    result = verify_single_addition(
        reader(objects), empty, root, target, blob
    )
    assert result["tree_verification"] == "PASS"
    print("CRYPTOGRAPHIC_TREE_POSITIVE=PASS")

    cases = 0

    def reject(changed):
        nonlocal cases
        try:
            verify_single_addition(
                reader(changed), empty, root, target, blob
            )
        except TreeError:
            cases += 1
        else:
            raise AssertionError("INVALID_TREE_ACCEPTED")

    tampered = copy.deepcopy(objects)
    tampered[leaf]["tree"][0]["sha"] = "f" * 40
    reject(tampered)

    omitted = copy.deepcopy(objects)
    omitted[status]["tree"] = []
    reject(omitted)

    truncated = copy.deepcopy(objects)
    truncated[leaf]["truncated"] = True
    reject(truncated)

    duplicate = copy.deepcopy(objects)
    duplicate[leaf]["tree"].append(
        copy.deepcopy(duplicate[leaf]["tree"][0])
    )
    reject(duplicate)

    wrong_mode = copy.deepcopy(objects)
    wrong_mode[leaf]["tree"][0]["mode"] = "100755"
    reject(wrong_mode)

    wrong_oid = copy.deepcopy(objects)
    wrong_oid[root]["sha"] = "0" * 40
    reject(wrong_oid)

    unexpected = copy.deepcopy(objects)
    unexpected[root]["tree"].append({
        "path": "unexpected",
        "mode": "100644",
        "type": "blob",
        "sha": blob,
    })
    reject(unexpected)

    print("CRYPTOGRAPHIC_TREE_NEGATIVE_CASES=" + str(cases))
    assert cases == 7

    print("TREE_OBJECT_IDENTITIES=PASS")
    print("NESTED_TREE_VERIFICATION=PASS")
    print("TREE_DIFF_POLICY=PASS")
    print("COMMIT_OBJECT_VERIFICATION=NOT_IMPLEMENTED")
    print("REAL_API_REQUESTS=0")
    print("GITHUB_MUTATIONS=0")
    print("PRODUCTION_PUBLICATION=BLOCKED")
    print("TREE_MODULE_SELFTEST=PASS")


if __name__ == "__main__":
    self_test()
