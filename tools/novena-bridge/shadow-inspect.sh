#!/usr/bin/env bash
# Optional read-only inspection; no status update or checkpoint promotion.
set -u

fail() {
    printf '%s\n' "$1" >&2
    exit 2
}

[[ $# -eq 1 ]] || fail 'Usage: shadow-inspect.sh RUNNER.txt'
[[ -f $1 && ! -L $1 ]] || fail 'Input must be an existing regular, non-symlink file.'

binary=${NOVENA_BRIDGE_BINARY-/tmp/novena-bridge-224-target.Qdwp9h2s/debug/novena-bridge}
# Require a file path, never implicit command lookup through PATH.
[[ $binary == */* && -f $binary && -x $binary ]] ||
    fail 'Set NOVENA_BRIDGE_BINARY to an executable Rust CLI file path.'
command -v python3 >/dev/null 2>&1 || fail 'Python 3 is required.'

# Keep JSON in memory. Suppress CLI diagnostics, which may contain source data.
report=$("$binary" inspect-log "$1" 2>/dev/null)
cli_status=$?
if (( cli_status != 0 )); then
    printf 'inspect-log failed (exit %d).\n' "$cli_status" >&2
    exit "$cli_status"
fi

printf '%s' "$report" | python3 -I -B -c '
import json
import sys

def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError()
        result[key] = value
    return result

def invalid_constant(value):
    raise ValueError()

try:
    report = json.load(sys.stdin, object_pairs_hook=unique_object,
                       parse_constant=invalid_constant)
    keys = {"source", "block_id", "complete", "state", "integrity",
            "process_result", "claimed_block_result", "technical_result",
            "professional_acceptance", "upload_result", "reasons"}
    if not isinstance(report, dict) or set(report) != keys:
        raise ValueError()
    if not isinstance(report["source"], str):
        raise ValueError()
    for key in ("block_id", "claimed_block_result"):
        if report[key] is not None and not isinstance(report[key], str):
            raise ValueError()
    if type(report["complete"]) is not bool:
        raise ValueError()
    if not isinstance(report["reasons"], list) or not all(
            isinstance(reason, str) for reason in report["reasons"]):
        raise ValueError()
    statuses = {
        "state": {"COMPLETE", "REVIEW_REQUIRED"},
        "integrity": {"PASS", "FAIL", "MISSING", "INVALID"},
        "process_result": {"PASS", "FAIL", "UNKNOWN"},
        "upload_result": {"UNKNOWN"},
        "technical_result": {"UNKNOWN"},
        "professional_acceptance": {"UNKNOWN"},
    }
    for key, allowed in statuses.items():
        if not isinstance(report[key], str) or report[key] not in allowed:
            raise ValueError()
except (ValueError, TypeError, RecursionError):
    print("Invalid inspect-log JSON report.", file=sys.stderr)
    sys.exit(2)

# Only validated status tokens are printed; omit source, claims and free text.
print(" ".join(key + "=" + report[key] for key in statuses))
print("complete=" + str(report["complete"]).lower()
      + " reasons=" + str(len(report["reasons"])))
'
