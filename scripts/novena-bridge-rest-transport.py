#!/usr/bin/env python3
"""Read-only REST transport for the Novena Bridge.

No mutation methods, credential storage, publication or CLI.
Transport callable may be injected for synthetic tests.
"""

import json
import os
import re
import subprocess


ARCHIVE = "loomit77/project-block-logs"
PREFIX = "repos/" + ARCHIVE


class TransportError(Exception):
    pass


def allowed_endpoint(endpoint):
    if type(endpoint) is not str:
        raise TransportError("INVALID_ENDPOINT")

    if endpoint == PREFIX:
        return endpoint

    if endpoint == PREFIX + "/git/ref/heads/main":
        return endpoint

    for kind in ("commits", "trees", "blobs"):
        prefix = PREFIX + "/git/" + kind + "/"
        if endpoint.startswith(prefix):
            suffix = endpoint[len(prefix):]
            if re.fullmatch(r"[0-9a-f]{40}", suffix):
                return endpoint

    raise TransportError("ENDPOINT_NOT_ALLOWLISTED")


def normalize_tree(response):
    """Normalize one nonrecursive GitHub REST tree response."""
    if type(response) is not dict:
        raise TransportError("INVALID_TREE_RESPONSE")

    if response.get("truncated") is not False:
        raise TransportError("INCOMPLETE_TREE")

    entries = response.get("tree")
    if type(entries) is not list:
        raise TransportError("INVALID_ENTRIES")

    normalized = []
    permitted = {"path", "mode", "type", "sha", "size", "url"}

    for entry in entries:
        if type(entry) is not dict:
            raise TransportError("INVALID_ENTRY")

        if not {"path", "mode", "type", "sha"} <= set(entry):
            raise TransportError("MISSING_FIELDS")

        if not set(entry) <= permitted:
            raise TransportError("UNKNOWN_FIELDS")

        if "size" in entry and (
            type(entry["size"]) is not int or entry["size"] < 0
        ):
            raise TransportError("INVALID_SIZE")

        if "url" in entry and type(entry["url"]) is not str:
            raise TransportError("INVALID_URL")

        normalized.append({
            "path": entry["path"],
            "mode": entry["mode"],
            "type": entry["type"],
            "sha": entry["sha"],
        })

    return {
        "sha": response.get("sha"),
        "truncated": False,
        "tree": normalized,
    }


class ReadOnlyTransport:
    def __init__(self, executor=None, max_requests=256):
        if executor is not None and not callable(executor):
            raise TransportError("INVALID_EXECUTOR")
        if type(max_requests) is not int or not 1 <= max_requests <= 256:
            raise TransportError("INVALID_REQUEST_LIMIT")

        self.executor = executor or self._gh_get
        self.max_requests = max_requests
        self.requests = 0

    @staticmethod
    def _gh_get(endpoint):
        environment = os.environ.copy()
        environment["GH_PROMPT_DISABLED"] = "1"
        environment["GIT_TERMINAL_PROMPT"] = "0"

        try:
            result = subprocess.run(
                ["gh", "api", "--method", "GET", endpoint],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
                timeout=40,
                env=environment,
            )
        except (OSError, subprocess.TimeoutExpired):
            raise TransportError("READ_ONLY_REQUEST_FAILED") from None

        if result.returncode != 0:
            raise TransportError("READ_ONLY_REQUEST_FAILED")

        return result.stdout

    def get(self, endpoint):
        allowed_endpoint(endpoint)

        if self.requests >= self.max_requests:
            raise TransportError("REQUEST_LIMIT_EXCEEDED")

        self.requests += 1
        raw = self.executor(endpoint)

        if type(raw) is not bytes or len(raw) > 8 * 1024 * 1024:
            raise TransportError("INVALID_RESPONSE_BYTES")

        try:
            document = json.loads(raw)
        except (ValueError, UnicodeDecodeError):
            raise TransportError("INVALID_RESPONSE_JSON") from None

        if type(document) is not dict:
            raise TransportError("INVALID_RESPONSE_SHAPE")

        if endpoint.startswith(PREFIX + "/git/trees/"):
            return normalize_tree(document)

        return document


def self_test():
    import hashlib
    import importlib.util
    from pathlib import Path

    spec = importlib.util.spec_from_file_location(
        "novena_tree_194",
        Path(__file__).resolve().parent / "novena-bridge-git-tree.py",
    )
    tree = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(tree)

    blob = hashlib.sha1(b"blob 4\0test").hexdigest()

    entries = [{
        "path": "test.txt",
        "mode": "100644",
        "type": "blob",
        "sha": blob,
    }]

    tree_oid = tree.tree_sha(entries)
    endpoint = PREFIX + "/git/trees/" + tree_oid

    response = {
        "sha": tree_oid,
        "truncated": False,
        "tree": [{
            **entries[0],
            "size": 4,
            "url": "https://api.github.com/example",
        }],
    }

    calls = []

    def fake_executor(path):
        calls.append(path)
        return json.dumps(response).encode("utf-8")

    transport = ReadOnlyTransport(
        executor=fake_executor,
        max_requests=1,
    )
    result = transport.get(endpoint)

    assert result["tree"] == entries
    assert tree.tree_sha(result["tree"]) == tree_oid
    assert calls == [endpoint]

    print("NORMALIZED_TREE_SHA1=PASS")

    negative_count = 0

    def rejects(callback):
        nonlocal negative_count
        try:
            callback()
        except TransportError:
            negative_count += 1
        else:
            raise AssertionError("INVALID_INPUT_ACCEPTED")

    rejects(lambda: transport.get(endpoint))
    rejects(lambda: allowed_endpoint(PREFIX + "/issues"))
    rejects(lambda: allowed_endpoint(PREFIX + "/git/trees/" + "x" * 40))
    rejects(lambda: allowed_endpoint(PREFIX + "/git/trees/" + tree_oid + "?recursive=1"))

    malformed = ReadOnlyTransport(executor=lambda _: b"invalid")
    rejects(lambda: malformed.get(endpoint))

    truncated = dict(response, truncated=True)
    rejects(lambda: normalize_tree(truncated))

    unknown = {
        **response,
        "tree": [{**response["tree"][0], "unexpected": 123}],
    }
    rejects(lambda: normalize_tree(unknown))

    assert negative_count == 7

    print("NEGATIVE_CASES=7")
    print("HTTP_METHODS=GET_ONLY")
    print("REAL_HTTP_REQUESTS=0")
    print("GITHUB_MUTATIONS=0")
    print("PRODUCTION_PUBLICATION=BLOCKED")
    print("TRANSPORT_SELFTEST=PASS")


if __name__ == "__main__":
    self_test()
