#!/usr/bin/env bash
# Optional manual local preparation only; never publish or promote a checkpoint.
set -u
if [[ $# -ne 3 ]]; then
    printf '%s\n' 'PREPARE_STATUS=ERROR EXPECTED=RUNNER_LOG_SHA256_OUTPUT_DIRECTORY' >&2
    exit 2
fi
exec python3 -I -B - "${BASH_SOURCE[0]}" "$@" <<'PY'
import base64
import hashlib
import json
import os
import re
import secrets
import stat
import subprocess
import sys

SOURCE_LIMIT = 32 * 1024 * 1024
JSON_LIMIT = 1024 * 1024
OUTPUT_LIMIT = 64 * 1024 * 1024
WARNING = 'HISTORICAL LOCAL PREVIEW: not a live checkpoint; no approval or publication.'


def require(condition):
    if not condition:
        raise ValueError('INVALID')


def unique(pairs):
    value = {}
    for key, item in pairs:
        require(key not in value)
        value[key] = item
    return value


def invalid(value):
    raise ValueError('INVALID_JSON')


def parse(data):
    return json.loads(data.decode('utf-8'), object_pairs_hook=unique,
                      parse_float=invalid, parse_constant=invalid)


def encode(value):
    return json.dumps(value, ensure_ascii=True, separators=(',', ':')).encode('ascii')


def digest(data):
    return hashlib.sha256(data).hexdigest()


def parts(path):
    require(type(path) is str and path.startswith('/') and '\x00' not in path)
    result = path[1:].split('/')
    require(all(p not in ('', '.', '..') for p in result))
    return result


def directory(path):
    # Pin ancestors, rejecting symlinks rather than resolving them.
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        for part in ([] if path == '/' else parts(path)):
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                            dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def read(path, limit, executable=False):
    components = parts(path)
    parent = directory('/' + '/'.join(components[:-1]))
    try:
        fd = os.open(components[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                     dir_fd=parent)
        try:
            before = os.fstat(fd)
            require(stat.S_ISREG(before.st_mode) and before.st_size <= limit)
            if executable:
                require(os.access(path, os.X_OK))
                return None
            with os.fdopen(fd, 'rb', closefd=False) as stream:
                data = stream.read(limit + 1)
            after = os.fstat(fd)
            require(len(data) <= limit)
            require((before.st_size, before.st_mtime_ns, before.st_ctime_ns) ==
                    (after.st_size, after.st_mtime_ns, after.st_ctime_ns))
            return data
        finally:
            os.close(fd)
    finally:
        os.close(parent)


def git(*args):
    # No optional index refresh, external diff, lazy object fetch or network.
    env = os.environ.copy()
    for key in list(env):
        if key.startswith('GIT_'):
            del env[key]
    env.update(GIT_OPTIONAL_LOCKS='0', GIT_TERMINAL_PROMPT='0', GIT_NO_LAZY_FETCH='1')
    with subprocess.Popen(['git', '--no-optional-locks', *args], env=env,
                          stdout=subprocess.PIPE, stderr=subprocess.DEVNULL) as child:
        try:
            data = child.stdout.read(SOURCE_LIMIT + 1)
            require(len(data) <= SOURCE_LIMIT)
            require(child.wait(timeout=30) == 0)
            return data
        finally:
            if child.poll() is None:
                child.kill()
                child.wait()


def write_private(fd, name, data):
    file_fd = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                      0o600, dir_fd=fd)
    try:
        os.fchmod(file_fd, 0o600)
        with os.fdopen(file_fd, 'wb', closefd=False) as stream:
            stream.write(data)
            stream.flush()
            os.fsync(file_fd)
    finally:
        os.close(file_fd)


def decoded(value):
    require(type(value) is str)
    return base64.b64decode(value, validate=True)


def validate(bundle, previous, manifest, originals, sidecar):
    require(type(bundle) is dict and set(bundle) == {'payload', 'provenance'})
    payload, provenance = bundle['payload'], bundle['provenance']
    require(type(payload) is dict and set(payload) == set(previous))
    require(all(payload[k] == previous[k] for k in previous if k != 'latest_block'))
    require(payload['publication']['state'] == 'NOT_PUBLISHED')
    require(payload['publication']['commit'] is None)
    block = payload['latest_block']
    require(type(block) is dict and set(block) == {
        'block_id', 'runner', 'upload', 'technical_result', 'professional_acceptance'})
    require(block['block_id'] == manifest['sources'][0]['block_id'])
    require(block['runner'] in ('PASS', 'FAIL', 'UNKNOWN'))
    for key in ('upload', 'technical_result', 'professional_acceptance'):
        require(block[key] == 'UNKNOWN')
    expected = {
        'contract': 'LOCAL_SOURCE_MERGE_V1', 'scope': 'CALLER_SUPPLIED_ORIGINALS',
        'exchange_v1_projection': 'LATEST_BLOCK_ONLY', 'build': 'UNKNOWN',
        'hardware_image_identity': 'UNKNOWN', 'remote_sync': 'UNKNOWN',
        'checkpoint_approval': 'NOT_GRANTED', 'publication_performed': False,
        'selected_log': manifest['selected_log'],
    }
    require(type(provenance) is dict and set(provenance) ==
            set(expected) | {'sources', 'reasons', 'state', 'file_selection'})
    for key, value in expected.items():
        require(provenance[key] == value)
    reasons = provenance['reasons']
    require(type(reasons) is list and all(type(x) is str for x in reasons))
    require(provenance['state'] == ('REVIEW_REQUIRED' if reasons else 'OBSERVATIONS_MERGED'))
    selection = provenance['file_selection']
    require(selection == {
        'contract': 'LOCAL_SOURCE_FILES_V1',
        'manifest_base64': base64.b64encode(encode(manifest)).decode('ascii'),
        'manifest_sha256': digest(encode(manifest)),
        'association': 'CALLER_SUPPLIED_NOT_AUTHENTICATED',
        'expected_original_hashes': 'PASS',
    })
    sources = provenance['sources']
    require(type(sources) is list and len(sources) == 3)
    for entry, source, original in zip(manifest['sources'], sources, originals):
        require(source['id'] == entry['id'] and source['kind'] == entry['kind'])
        require(source['sha256'] == digest(original))
        require(decoded(source['original_base64']) == original)
        require(source['source_authenticity'] == 'UNKNOWN')
        if entry['kind'] == 'RunnerLog':
            require(decoded(source['sidecar_base64']) == sidecar)
            require(source['sha256_integrity'] == 'PASS')
            inspection = source['inspection']
            require(inspection['integrity'] == 'PASS')
            require(inspection['process_result'] == block['runner'])
            for key in ('technical_result', 'professional_acceptance', 'upload_result'):
                require(inspection[key] == 'UNKNOWN')
            if inspection['state'] == 'REVIEW_REQUIRED':
                require('LOG_REVIEW_REQUIRED' in reasons and block['runner'] == 'UNKNOWN')
        else:
            require(source['sidecar_base64'] is None)
            require(source['sha256_integrity'] == 'UNKNOWN')
    inv = parse(originals[2])
    counts = inv.get('host_counts')
    valid = (type(inv.get('record_count')) is int and inv['record_count'] >= 0
             and type(inv.get('unique_file_hashes')) is int and inv['unique_file_hashes'] >= 0
             and type(counts) is dict
             and all(type(n) is int and n >= 0 for n in counts.values())
             and type(inv.get('scan_warnings')) is list
             and type(inv.get('path_conflicts')) is list)
    if not valid:
        require('MISSING_OR_INVALID_OBSERVATION' in reasons)
    else:
        if inv['scan_warnings'] or inv['path_conflicts']:
            require('INVENTORY_REQUIRES_REVIEW' in reasons)
        if (sum(counts.values()) != inv['record_count'] or
                inv['unique_file_hashes'] > inv['record_count']):
            require('INCONSISTENT_INVENTORY_COUNTS' in reasons)
    return bool(reasons or payload['review_required'])


def main():
    require(sys.platform == 'linux' and len(sys.argv) == 5)
    os.umask(0o077)
    launcher_argument, runner, expected_hash, output = sys.argv[1:]
    require(re.fullmatch('[0-9a-f]{64}', expected_hash) is not None)
    parts(runner)
    require(runner.endswith('.txt'))
    components = parts(output)
    binary = os.environ.get('NOVENA_BRIDGE_BINARY', '')
    read(binary, sys.maxsize, executable=True)
    root = os.getcwd()
    root_fd = directory(root)
    os.close(root_fd)
    require(git('rev-parse', '--show-toplevel').decode().rstrip('\n') == root)
    # Bind invocation to this checkout and this launcher, not merely any Git root.
    launcher = root + '/tools/novena-bridge/prepare-status.sh'
    require(os.path.abspath(launcher_argument) == launcher)
    read(launcher, JSON_LIMIT)
    cargo = read(root + '/tools/novena-bridge/Cargo.toml', JSON_LIMIT)
    require(re.search(rb'(?m)^name\s*=\s*"novena-bridge"\s*$', cargo) is not None)
    require(b'Universal NixOS SD Image for Novena' in read(root + '/README.md', JSON_LIMIT))
    log = read(runner, SOURCE_LIMIT)
    require(digest(log) == expected_hash)
    side = read(runner + '.sha256', 8192)
    lines = side.decode('utf-8').splitlines()
    require(len(lines) == 1)
    match = re.fullmatch(r'([0-9a-fA-F]{64})[ \t]+\*?(.+)', lines[0])
    require(match is not None and match[1].lower() == expected_hash)
    # A basename means the adjacent file; any other name must be the exact path.
    require(match[2] in (os.path.basename(runner), runner))
    blocks = {line[len('BLOCK_ID='):].strip() for line in
              log.decode('utf-8').splitlines() if line.startswith('BLOCK_ID=')}
    require(len(blocks) == 1 and bool(next(iter(blocks))))
    inventory_path = root + '/docs/image-inventory.json'
    inventory = read(inventory_path, SOURCE_LIMIT)
    parse(inventory)
    previous = parse(read(root + '/docs/bridge-lite/EXCHANGE-PILOT.json', JSON_LIMIT))
    head = git('rev-parse', '--verify', 'HEAD').decode().strip()
    require(re.fullmatch('[0-9a-f]{40}', head) is not None)
    branch = git('rev-parse', '--abbrev-ref', 'HEAD').decode().strip()
    observation = encode(dict(head=head, branch=branch,
                              worktree='MODIFIED' if git('status', '--porcelain=v1',
                                                        '--untracked-files=normal') else 'CLEAN',
                              remote_sync='UNKNOWN'))
    require(len(log) + len(side) + len(inventory) + len(observation) <= SOURCE_LIMIT)
    parent_path = '/' + '/'.join(components[:-1])
    parent = directory(parent_path)
    temporary = '.novena-prepare-' + secrets.token_hex(16)
    temp_fd = final_fd = None
    final_created = False
    committed = False
    try:
        try:
            os.stat(components[-1], dir_fd=parent, follow_symlinks=False)
        except FileNotFoundError:
            pass
        else:
            raise ValueError('OUTPUT_EXISTS')
        os.mkdir(temporary, 0o700, dir_fd=parent)
        temp_fd = os.open(temporary, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                          dir_fd=parent)
        os.fchmod(temp_fd, 0o700)
        write_private(temp_fd, 'git-observation.json', observation)
        temp_path = parent_path.rstrip('/') + '/' + temporary
        entries = []
        for source_id, kind, path, original in (
                (os.path.basename(runner), 'RunnerLog', runner, log),
                ('git-observation.json', 'GitObservation', temp_path + '/git-observation.json', observation),
                ('image-inventory.json', 'ImageInventory', inventory_path, inventory)):
            entries.append(dict(id=source_id, kind=kind, path=path, sha256=digest(original),
                                sidecar_path=runner + '.sha256' if kind == 'RunnerLog' else None,
                                block_id=next(iter(blocks)) if kind == 'RunnerLog' else None))
        manifest = dict(contract='LOCAL_SOURCE_FILES_V1', previous=previous,
                        selected_log=entries[0]['id'], sources=entries)
        manifest_bytes = encode(manifest)
        require(len(manifest_bytes) <= JSON_LIMIT)
        write_private(temp_fd, 'manifest.json', manifest_bytes)
        # Original sources remain original paths. Rust rechecks their expected hashes.
        # Trusted local ancestors must not be concurrently replaced during execution.
        write_private(temp_fd, 'rust-output.json', b'')
        with open(temp_path + '/manifest.json', 'rb') as stdin, \
                open(temp_path + '/rust-output.json', 'r+b') as stdout:
            subprocess.run([binary, 'prepare-local-sources', '-'], stdin=stdin,
                           stdout=stdout, stderr=subprocess.DEVNULL,
                           timeout=60, check=True)
        data = read(temp_path + '/rust-output.json', OUTPUT_LIMIT)
        review = validate(parse(data), previous, manifest, [log, observation, inventory], side)
        write_private(temp_fd, 'source-merge.json', data)
        require(read(temp_path + '/source-merge.json', OUTPUT_LIMIT) == data)
        # mkdir and link are exclusive: even an empty pre-existing target is refused.
        # No rename that could silently replace an existing directory or snapshot.
        os.mkdir(components[-1], 0o700, dir_fd=parent)
        final_created = True
        final_fd = os.open(components[-1], os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                           dir_fd=parent)
        os.fchmod(final_fd, 0o700)
        os.link('source-merge.json', 'source-merge.json', src_dir_fd=temp_fd,
                dst_dir_fd=final_fd, follow_symlinks=False)
        require(stat.S_IMODE(os.fstat(final_fd).st_mode) == 0o700)
        require(stat.S_IMODE(os.stat('source-merge.json', dir_fd=final_fd,
                                    follow_symlinks=False).st_mode) == 0o600)
        os.fsync(final_fd)
        committed = True
    finally:
        if final_fd is not None:
            if not committed:
                try:
                    os.unlink('source-merge.json', dir_fd=final_fd)
                except FileNotFoundError:
                    pass
            os.close(final_fd)
        if final_created and not committed:
            os.rmdir(components[-1], dir_fd=parent)
        if temp_fd is not None:
            for name in ('git-observation.json', 'manifest.json', 'rust-output.json', 'source-merge.json'):
                try:
                    os.unlink(name, dir_fd=temp_fd)
                except FileNotFoundError:
                    pass
            os.close(temp_fd)
            os.rmdir(temporary, dir_fd=parent)
        os.close(parent)
    print(WARNING)
    print('PREPARE_STATUS=' + ('REVIEW_REQUIRED' if review else 'PREVIEW_STORED'))
    print('PUBLICATION=NOT_PUBLISHED CHECKPOINT_APPROVAL=NOT_GRANTED')
    print('SHA256=' + digest(data))
    # Review previews are retained, but never reported as a successful qualification.
    return 3 if review else 0


try:
    sys.exit(main())
except (Exception, KeyboardInterrupt):
    print(WARNING, file=sys.stderr)
    print('PREPARE_STATUS=ERROR', file=sys.stderr)
    sys.exit(2)
PY
