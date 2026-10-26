#!/usr/bin/env bash
# Manual, read-only local preview. Never updates or approves a status.
set -u
if [[ $# -ne 2 || ! $2 =~ ^[0-9a-f]{64}$ ]]; then
    printf '%s\n' 'HISTORICAL LOCAL PREVIEW: expected FILE EXPECTED_SHA256 (64 lowercase hex digits).' >&2
    exit 2
fi
command -v python3 >/dev/null 2>&1 || {
    printf '%s\n' 'HISTORICAL LOCAL PREVIEW: Python 3 required.' >&2
    exit 2
}
exec python3 -I -B - "$1" "$2" <<'PY'
import hashlib
import json
import os
import re
import stat
import sys

PREFIX = "HISTORICAL LOCAL PREVIEW (not live; no approval): "
LIMIT = 32 * 1024 * 1024
RESULT = ("PASS", "FAIL", "UNKNOWN")


def require(condition):
    if not condition:
        raise ValueError()


def obj(value, keys):
    require(type(value) is dict and set(value) == set(keys.split()))
    return value


def token(value, allowed):
    require(type(value) is str and value in allowed)
    return value


def text(value, nullable=False):
    require((nullable and value is None) or type(value) is str)


def strings(value):
    require(type(value) is list and all(type(x) is str for x in value))


def hexhash(value, length=64):
    require(type(value) is str and re.fullmatch('[0-9a-f]{%d}' % length, value))


def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result)
        result[key] = value
    return result


def invalid(_):
    raise ValueError()


def read_private(path):
    # Pin every directory while traversing, including relative paths and '..'.
    # Fail closed where descriptor-relative no-follow traversal is unavailable.
    require(hasattr(os, 'O_NOFOLLOW') and hasattr(os, 'O_DIRECTORY'))
    require(os.open in os.supports_dir_fd)
    parts = path.split('/')
    require(parts[-1] not in ('', '.', '..'))
    directory = os.open('/' if path.startswith('/') else '.',
                        os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        for part in parts[:-1]:
            if part in ('', '.'):
                continue
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                            dir_fd=directory)
            os.close(directory)
            directory = child
        fd = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW
                     | getattr(os, 'O_NONBLOCK', 0), dir_fd=directory)
        try:
            before = os.fstat(fd)
            require(stat.S_ISREG(before.st_mode))
            require(stat.S_IMODE(before.st_mode) == 0o600)
            require(before.st_size <= LIMIT)
            with os.fdopen(fd, 'rb', closefd=False) as stream:
                data = stream.read(LIMIT + 1)
            after = os.fstat(fd)
            require(len(data) <= LIMIT)
            require((before.st_size, before.st_mtime_ns, before.st_ctime_ns,
                     before.st_mode) ==
                    (after.st_size, after.st_mtime_ns, after.st_ctime_ns,
                     after.st_mode))
            return data
        finally:
            os.close(fd)
    finally:
        os.close(directory)


def validate(payload, provenance):
    # Minimal Exchange-v1 shape; opaque references are checked but never printed.
    p = obj(payload, 'schema_version project generated_at_utc last_verified '
            'current latest_block build evidence publication review_required')
    require(type(p['schema_version']) is int and p['schema_version'] == 1)
    token(p['project'], ('novena',))
    text(p['generated_at_utc'])
    current = obj(p['current'], 'state observed_at_utc')
    token(current['state'], ('LAST_VERIFIED', 'CURRENT_UNKNOWN', 'IN_PROGRESS'))
    text(current['observed_at_utc'], True)
    checkpoint = p['last_verified']
    if checkpoint is not None:
        obj(checkpoint, 'block_id git_head qualified_at_utc qualification evidence_files')
        text(checkpoint['block_id'])
        require(bool(checkpoint['block_id']))
        hexhash(checkpoint['git_head'], 40)
        text(checkpoint['qualified_at_utc'])
        token(checkpoint['qualification'], ('PASS',))
        strings(checkpoint['evidence_files'])
        require(bool(checkpoint['evidence_files']))
        require(all(checkpoint['evidence_files']))
        require(len(set(checkpoint['evidence_files'])) == len(checkpoint['evidence_files']))
    if current['state'] == 'LAST_VERIFIED':
        require(checkpoint is not None and current['observed_at_utc'] is not None)
    block = p['latest_block']
    if block is not None:
        obj(block, 'block_id runner upload technical_result professional_acceptance')
        text(block['block_id'])
        require(bool(block['block_id']))
        for key in ('runner', 'upload', 'technical_result', 'professional_acceptance'):
            token(block[key], RESULT)
    build = obj(p['build'], 'state source')
    token(build['state'], ('UNKNOWN', 'NOT_STARTED', 'RUNNING', 'PASS', 'FAIL'))
    text(build['source'], True)
    require(type(p['evidence']) is list)
    for item in p['evidence']:
        obj(item, 'log_file sha256_file local_integrity archive_upload')
        for key in ('log_file', 'sha256_file'):
            text(item[key])
            require(bool(item[key]))
        for key in ('local_integrity', 'archive_upload'):
            token(item[key], RESULT)
    publication = obj(p['publication'], 'state archive commit')
    token(publication['state'], ('NOT_PUBLISHED',))
    token(publication['archive'], ('loomit77/project-block-logs',))
    require(publication['commit'] is None)
    strings(p['review_required'])
    require(len(set(p['review_required'])) == len(p['review_required']))

    v = provenance
    require(type(v) is dict)
    keys = ('contract scope exchange_v1_projection selected_log sources reasons state '
            'build hardware_image_identity remote_sync checkpoint_approval publication_performed')
    require(set(v) in (set(keys.split()), set(keys.split()) | {'file_selection'}))
    for key, expected in {
        'contract': 'LOCAL_SOURCE_MERGE_V1', 'scope': 'CALLER_SUPPLIED_ORIGINALS',
        'exchange_v1_projection': 'LATEST_BLOCK_ONLY', 'build': 'UNKNOWN',
        'hardware_image_identity': 'UNKNOWN', 'remote_sync': 'UNKNOWN',
        'checkpoint_approval': 'NOT_GRANTED',
    }.items():
        token(v[key], (expected,))
    require(v['publication_performed'] is False)
    text(v['selected_log'], True)
    strings(v['reasons'])
    token(v['state'], ('OBSERVATIONS_MERGED', 'REVIEW_REQUIRED'))
    require((v['state'] == 'REVIEW_REQUIRED') == bool(v['reasons']))
    require(type(v['sources']) is list and len(v['sources']) <= 256)
    ids = set()
    for source in v['sources']:
        obj(source, 'id kind sha256 original_base64 sidecar_base64 '
            'sha256_integrity inspection source_authenticity')
        text(source['id'])
        require(bool(source['id']) and source['id'] not in ids)
        ids.add(source['id'])
        token(source['kind'], ('RunnerLog', 'GitObservation', 'ImageInventory'))
        hexhash(source['sha256'])
        text(source['original_base64'])
        text(source['sidecar_base64'], True)
        token(source['sha256_integrity'], ('PASS', 'FAIL', 'UNKNOWN', 'MISSING', 'INVALID'))
        token(source['source_authenticity'], ('UNKNOWN',))
        require(type(source['inspection']) is dict)
    if 'file_selection' in v:
        selection = obj(v['file_selection'], 'contract manifest_base64 manifest_sha256 '
                        'association expected_original_hashes')
        token(selection['contract'], ('LOCAL_SOURCE_FILES_V1',))
        text(selection['manifest_base64'])
        hexhash(selection['manifest_sha256'])
        token(selection['association'], ('CALLER_SUPPLIED_NOT_AUTHENTICATED',))
        token(selection['expected_original_hashes'], ('PASS',))


try:
    require(len(sys.argv) == 3)
    hexhash(sys.argv[2])
    data = read_private(sys.argv[1])
    require(hashlib.sha256(data).hexdigest() == sys.argv[2])
    bundle = json.loads(data.decode('utf-8'), object_pairs_hook=unique,
                        parse_constant=invalid, parse_float=invalid)
    obj(bundle, 'payload provenance')
    p, v = bundle['payload'], bundle['provenance']
    validate(p, v)
    block = p['latest_block']
    statuses = {'project': 'novena', 'latest_block': 'PRESENT' if block else 'UNKNOWN'}
    for key in ('runner', 'upload', 'technical_result', 'professional_acceptance'):
        statuses[key] = block[key] if block else 'UNKNOWN'
    statuses.update(current_state=p['current']['state'], build_state=p['build']['state'],
                    publication_state=p['publication']['state'], provenance_state=v['state'])
    safe_reasons = {
        'LOG_REVIEW_REQUIRED', 'MISSING_OR_INVALID_OBSERVATION',
        'CONFLICTING_OBSERVATIONS', 'INCONSISTENT_INVENTORY_COUNTS',
        'INVENTORY_REQUIRES_REVIEW', 'MISSING_OBSERVATION',
        'CONFLICTING_BLOCK_SOURCES', 'SELECTED_LOG_MISSING_BLOCK_ID', 'NO_SELECTED_LOG',
        'LAST_VERIFIED_NOT_QUALIFIED', 'ARCHIVE_UPLOAD_NOT_INDEPENDENTLY_VERIFIED',
        'CURRENT_LIVE_STATE_UNKNOWN', 'HISTORICAL_SIX_BLOCK_SAMPLE_ONLY',
    }
    reasons = p['review_required'] + v['reasons']
    known = sorted(set(reasons) & safe_reasons)
    hidden = sum(reason not in safe_reasons for reason in reasons)
    output = [PREFIX + ' '.join(key + '=' + value for key, value in statuses.items()),
              PREFIX + 'review_reasons=' + (','.join(known) if known else 'NONE_ALLOWLISTED')
              + ' other_reason_count=' + str(hidden)]
except (Exception, KeyboardInterrupt):
    print(PREFIX + 'invalid or mismatched input.', file=sys.stderr)
    sys.exit(2)

print('\n'.join(output))
PY
