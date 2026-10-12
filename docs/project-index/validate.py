#!/usr/bin/env python3
"""Novena project index v0.3: read-only TSV and optional local provenance validation."""
import argparse
import csv
import hashlib
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCHEMAS = {
    'artifacts.tsv': ('artifact_id','artifact_type','locator','sha256','size_bytes','verification_status','verified_at','provenance_locator','provenance_sha256'),
    'source-artifacts.tsv': ('link_id','source_id','artifact_id','relation','assessment'),
    'sources.tsv': ('source_id','namespace','original_id','source_type','title','locator','revision','sha256','verification_status','record_locator','record_sha256'),
    'source-relations.tsv': ('relation_id','source_id_a','source_id_b','relation','status','basis_locator','basis_sha256'),
    'findings.tsv': ('finding_id','topic','statement','status','scope','record_locator','record_sha256','reviewed_at'),
    'evidence-links.tsv': ('link_id','finding_id','source_id','relation','locator_detail','assessment'),
}
HEX = re.compile(r'^[0-9a-f]{64}$')
LOC = re.compile(r'^(?P<path>[^#]+)(?:#L(?P<start>[1-9][0-9]*)(?:-L?(?P<end>[1-9][0-9]*))?)?$')

class Invalid(Exception):
    pass

def require(condition, message):
    if not condition:
        raise Invalid(message)

def sha256(path):
    h = hashlib.sha256()
    with path.open('rb') as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()

def table(name, columns, index_root=HERE):
    with (index_root / name).open(encoding='utf-8', newline='') as handle:
        reader = csv.DictReader(handle, delimiter='\t')
        require(tuple(reader.fieldnames or ()) == columns, f'{name}: unexpected header')
        rows = list(reader)
    for number, row in enumerate(rows, 2):
        require(None not in row and all(v is not None for v in row.values()), f'{name}:{number}: malformed columns')
    return rows

def unique(rows, field):
    values = [r[field] for r in rows]
    require(all(values), f'{field}: empty ID')
    require(len(values) == len(set(values)), f'{field}: duplicate ID')
    return set(values)

def hash_field(value, field):
    require(not value or HEX.fullmatch(value) is not None, f'{field}: invalid SHA256')

def local_path(repo, locator):
    require(bool(locator) and not locator.startswith(('http://','https://')), f'not a local locator: {locator}')
    match = LOC.fullmatch(locator)
    require(match is not None, f'invalid locator: {locator}')
    rel = Path(match.group('path'))
    require(not rel.is_absolute() and '..' not in rel.parts, f'unsafe path: {locator}')
    candidate = (repo / rel).resolve()
    require(candidate.is_relative_to(repo), f'path escapes repo: {locator}')
    require(candidate.is_file(), f'file missing: {locator}')
    start = match.group('start')
    end = match.group('end')
    if start:
        with candidate.open('rb') as handle:
            count = sum(1 for _ in handle)
        last = int(end or start)
        require(int(start) <= last <= count, f'line range invalid: {locator}')
    return candidate

def originals(data, repo, reference_root=None):
    register_guard(data)
    repo = repo.expanduser().resolve()
    require(repo.is_dir(), f'repository absent: {repo}')
    count = 0
    for name in ('sources.tsv','findings.tsv'):
        for row in data[name]:
            path = local_path(repo, row['record_locator'])
            require(sha256(path) == row['record_sha256'], f'register hash mismatch: {row["record_locator"]}')
            count += 1
    for row in data['source-relations.tsv']:
        path = local_path(repo, row['basis_locator'])
        require(sha256(path) == row['basis_sha256'], f'basis hash mismatch: {row["basis_locator"]}')
        count += 1
    for row in data['sources.tsv']:
        if row['source_type'] not in REGISTER_TYPES and not row['locator'].startswith(('https://','http://')):
            path = local_path(repo, row['locator'])
            require(bool(row['sha256']), f'local artifact missing hash: {row["source_id"]}')
            require(sha256(path) == row['sha256'], f'artifact hash mismatch: {row["source_id"]}')
            count += 1
    for row in data['evidence-links.tsv']:
        if not row['locator_detail'].startswith(('https://','http://')):
            local_path(repo, row['locator_detail'])
            count += 1
    if data.get('artifacts.tsv'):
        require(reference_root is not None, 'REF_LOCKED_DEBUG requires --ref-locked-debug-root')
        reference_root = reference_root.expanduser().resolve()
        require(reference_root.is_dir(), 'reference root absent')
    for row in data.get('artifacts.tsv', []):
        path = local_path(reference_root, artifact_relative(row['locator']))
        require(path.stat().st_size == int(row['size_bytes']), f'artifact size mismatch: {row["artifact_id"]}')
        require(sha256(path) == row['sha256'], f'artifact hash mismatch: {row["artifact_id"]}')
        basis = local_path(repo, row['provenance_locator'])
        require(sha256(basis) == row['provenance_sha256'], 'artifact provenance hash mismatch')
        count += 2
    print(f'ORIGINALS_CHECK=PASS checked={count}')

REGISTER_TYPES = {'artifact_reference', 'artifact_collection', 'missing_original', 'primary_source_gap', 'historical_chat'}
SOURCE_STATUSES = {'HISTORICAL_CATALOG_CHECK','LOCAL_HASH_VERIFIED','CONTENT_UNAVAILABLE','DOCUMENTED_READ_SCOPE'}

def locator_syntax(locator):
    match = LOC.fullmatch(locator)
    require(match is not None, f'invalid locator: {locator}')
    rel = Path(match.group('path'))
    require(not rel.is_absolute() and '..' not in rel.parts and rel.parts, f'unsafe path: {locator}')
    if match.group('end'):
        require(int(match.group('end')) >= int(match.group('start')), 'invalid line range')
    return match

def artifact_relative(locator):
    require(locator.startswith('REF_LOCKED_DEBUG/'), 'artifact requires REF_LOCKED_DEBUG alias')
    rel = locator[len('REF_LOCKED_DEBUG/'):]
    locator_syntax(rel)
    require('#' not in rel, 'artifact must name whole file')
    return rel

def structure(data):
    sources = unique(data['sources.tsv'], 'source_id')
    findings = unique(data['findings.tsv'], 'finding_id')
    unique(data['source-relations.tsv'], 'relation_id')
    unique(data['evidence-links.tsv'], 'link_id')
    keys = [(r['namespace'], r['original_id']) for r in data['sources.tsv']]
    require(len(keys) == len(set(keys)), 'duplicate namespaced original ID')
    for row in data['sources.tsv']:
        require(row['namespace'] and row['original_id'] and row['record_locator'] and row['record_sha256'], 'incomplete source')
        hash_field(row['sha256'], 'source.sha256')
        hash_field(row['record_sha256'], 'source.record_sha256')
        require(row['verification_status'] in {'HISTORICAL_CATALOG_CHECK','LOCAL_HASH_VERIFIED','CONTENT_UNAVAILABLE','DOCUMENTED_READ_SCOPE'}, 'invalid verification status')
    for row in data['source-relations.tsv']:
        require(row['source_id_a'] in sources and row['source_id_b'] in sources and row['source_id_a'] != row['source_id_b'], 'invalid relation references')
        require(row['relation'] in {'SAME_AS','RELATED_TO','DIFFERENT_FROM'}, 'invalid relation type')
        require(row['status'] in {'CONFIRMED','CANDIDATE','UNRESOLVED'}, 'invalid relation status')
        require(row['basis_locator'] and row['basis_sha256'], 'relation missing basis')
        hash_field(row['basis_sha256'], 'relation.basis_sha256')
    for row in data['findings.tsv']:
        require(row['status'] in {'PROVEN','REFUTED','HYPOTHESIS','OPEN','UNCLEAR'}, 'invalid finding status')
        require(row['record_locator'] and row['record_sha256'], 'finding missing provenance')
        hash_field(row['record_sha256'], 'finding.record_sha256')
    for row in data['evidence-links.tsv']:
        require(row['finding_id'] in findings and row['source_id'] in sources, 'invalid evidence references')
        require(row['relation'] in {'SUPPORTS','CONTRADICTS','LIMITS','CONTEXT'}, 'invalid evidence relation')
        require(bool(row['locator_detail']), 'evidence missing locator')
    for row in data['sources.tsv']:
        if row['namespace'] == 'sources-md':
            expected_type = {'SPL-001':'artifact_reference','SPL-002':'artifact_collection','SPL-005':'missing_original','SPL-006':'primary_source_gap','SPL-007':'historical_chat'}.get(row['original_id'])
            require(expected_type is None or row['source_type'] == expected_type, 'source original role misclassified')
        locator_syntax(row['record_locator'])
        if row['source_type'] in REGISTER_TYPES:
            require(row['locator'] == row['record_locator'] and not row['sha256'], 'register source misclassified as original')
            expected = 'CONTENT_UNAVAILABLE' if row['source_type'] in {'missing_original','primary_source_gap','historical_chat'} else 'HISTORICAL_CATALOG_CHECK'
            require(row['verification_status'] == expected, 'missing original misclassified')
        elif not row['locator'].startswith(('http://','https://')):
            locator_syntax(row['locator'])
            require(bool(row['sha256']), 'local source missing required hash')
        require(row['verification_status'] != 'LOCAL_HASH_VERIFIED' or bool(row['sha256']), 'verified source missing required hash')
    for name, field in [('source-relations.tsv','basis_locator'), ('findings.tsv','record_locator')]:
        for row in data[name]:
            locator_syntax(row[field])
    for row in data['evidence-links.tsv']:
        if not row['locator_detail'].startswith(('http://','https://')):
            locator_syntax(row['locator_detail'])
    artifacts = unique(data.get('artifacts.tsv', []), 'artifact_id')
    unique(data.get('source-artifacts.tsv', []), 'link_id')
    for row in data.get('artifacts.tsv', []):
        require(row['artifact_type'] in {'elf','wrapper','binary','map','configuration','source','header','assembly'}, 'invalid artifact type')
        artifact_relative(row['locator'])
        require(row['verification_status'] in {'HISTORICAL_HASH_VERIFIED','UNVERIFIED'}, 'invalid artifact status')
        require(row['sha256'] and row['provenance_sha256'], 'artifact missing required hash')
        hash_field(row['sha256'], 'artifact.sha256')
        hash_field(row['provenance_sha256'], 'artifact.provenance_sha256')
        require(re.fullmatch(r'0|[1-9][0-9]*', row['size_bytes']) is not None, 'invalid artifact size')
        require(not row['verified_at'] or re.fullmatch(r'\d{4}-\d{2}-\d{2}', row['verified_at']), 'invalid verification date')
        locator_syntax(row['provenance_locator'])
    source_by_id = {r['source_id']: r for r in data['sources.tsv']}
    artifact_by_id = {r['artifact_id']: r for r in data.get('artifacts.tsv', [])}
    for row in data.get('source-artifacts.tsv', []):
        require(row['source_id'] in sources, 'dangling source')
        require(row['artifact_id'] in artifacts, 'dangling artifact')
        require(row['relation'] in {'PRIMARY','COMPONENT'}, 'invalid source-artifact relation')
        require(bool(row['assessment']), 'missing link assessment')
        source = source_by_id[row['source_id']]
        expected = {'artifact_reference':'PRIMARY','artifact_collection':'COMPONENT'}.get(source['source_type'])
        require(row['relation'] == expected, 'source-artifact role mismatch')
        if expected == 'PRIMARY':
            require(artifact_by_id[row['artifact_id']]['artifact_type'] == 'elf', 'primary reference must be ELF')

CONTRACT_SHA256 = 'c41797a207a16723e08eb311db64ab630cadb4268dfaf2e42cb60c02fb801ce7'

def load_contract(path=HERE / 'baseline-contract.json'):
    try:
        raw = path.read_bytes()
        contract = json.loads(raw)
        require(contract['version'] == 'novena-project-index-contract-1', 'contract: unknown version')
        require(hashlib.sha256(raw).hexdigest() == CONTRACT_SHA256, 'contract: damaged identity')
        return contract
    except (OSError, ValueError, KeyError, TypeError) as error:
        raise Invalid(f'contract: unavailable or malformed: {error}') from error

def register_guard(data, contract=None):
    contract = load_contract() if contract is None else contract
    allowed = {r['source_id']: r for r in contract['register_exceptions']}
    for row in data['sources.tsv']:
        if row['source_type'] in REGISTER_TYPES or row['source_id'] in allowed:
            expected = allowed.get(row['source_id'])
            require(expected is not None and all(row[k] == value for k, value in expected.items()),
                    'register exception identity unauthorized')
            require(row['sha256'] == '', 'register source misclassified as original')

def project_contract(data, contract=None):
    contract = load_contract() if contract is None else contract
    for name, key, expected in [('artifacts.tsv', 'artifact_id', contract['artifacts']),
                                ('source-artifacts.tsv', 'link_id', contract['assignments'])]:
        rows = {r[key]: r for r in data[name]}
        for required in expected:
            identity = required[key]
            require(identity in rows, f'contract: missing mandatory {identity}')
            require(all(rows[identity][k] == value for k, value in required.items()),
                    f'contract: mandatory binding mismatch {identity}')
    register_guard(data, contract)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check-originals', action='store_true')
    parser.add_argument('--repo-root', type=Path)
    parser.add_argument('--ref-locked-debug-root', type=Path)
    parser.add_argument('--index-root', type=Path, default=HERE)
    parser.add_argument('--v02-regression', action='store_true', help='delegate to authenticated independent historical oracle regression')
    args = parser.parse_args()
    require(not (args.repo_root or args.ref_locked_debug_root) or args.check_originals, 'root options require --check-originals')
    data = {name: table(name, columns, args.index_root) for name, columns in SCHEMAS.items()}
    if args.v02_regression:
        require(not args.check_originals, 'historical regression has no originals CLI mode; use separate originals check')
        sys.modules.setdefault('validate', sys.modules[__name__])
        import regress_v02
        try:
            regress_v02.run(data)
        except (ValueError, regress_v02.tarfile.TarError) as error:
            raise Invalid(f'historical regression: {error}') from error
        return
    structure(data)
    project_contract(data)
    print('STRUCTURE_CHECK=PASS PROJECT_CONTRACT_CHECK=PASS')
    for name, rows in data.items():
        print(f'{name}: {len(rows)} records')
    if args.check_originals:
        originals(data, args.repo_root or HERE.parent.parent, args.ref_locked_debug_root)

if __name__ == '__main__':
    try:
        main()
    except (Invalid, OSError, UnicodeError, csv.Error) as error:
        print(f'INDEX_VALIDATION=FAIL: {error}', file=sys.stderr)
        sys.exit(1)
