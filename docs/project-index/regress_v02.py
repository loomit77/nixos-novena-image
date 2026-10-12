#!/usr/bin/env python3
"""Authenticated v0.2 oracle; no extraction, network or filesystem writes."""
import copy
import csv
import hashlib
import io
import tarfile
import contextlib
import argparse
from pathlib import Path, PurePosixPath
from types import SimpleNamespace
from unittest.mock import patch
import validate as v


class MemoryPath(PurePosixPath):
    """Controlled input fixture filesystem. Real validator path/hash rules run unchanged."""
    files = {}
    links = {}

    def expanduser(self): return self
    def resolve(self):
        parts = []
        for part in self.parts:
            if part == '..':
                if len(parts) > 1: parts.pop()
            elif part != '.': parts.append(part)
        value = str(PurePosixPath(*parts))
        return MemoryPath(self.links.get(value, value))
    def is_file(self): return str(self) in self.files
    def is_dir(self): return any(k.startswith(str(self).rstrip('/') + '/') for k in self.files)
    def read_bytes(self): return self.files[str(self)]
    def stat(self): return SimpleNamespace(st_size=len(self.read_bytes()))
    def open(self, mode='r', encoding=None, newline=None):
        if mode not in ('r', 'rb'): raise AssertionError('fixture write forbidden')
        if not self.is_file(): raise FileNotFoundError(str(self))
        b = self.read_bytes()
        return io.BytesIO(b) if mode == 'rb' else io.StringIO(b.decode(encoding or 'utf-8'), newline=newline)


def authenticated_archive(raw, contract):
    h = contract['historical']
    v.require(hashlib.sha256(raw).hexdigest() == h['sha256'], 'archive identity mismatch')
    expected = {m['name']: m for m in h['members']}
    result = {}
    seen = set()
    with tarfile.open(fileobj=io.BytesIO(raw), mode='r:') as archive:
        for member in archive:
            v.require(member.name not in seen, 'archive duplicate member')
            seen.add(member.name)
            v.require(member.name in expected, 'archive unexpected path')
            e = expected[member.name]
            v.require(member.type.decode('ascii') == e['type'] and
                      (member.isfile() or member.isdir()), 'archive member type mismatch')
            v.require(member.size == e['size'], 'archive member size mismatch')
            payload = archive.extractfile(member).read() if member.isfile() else b''
            v.require(hashlib.sha256(payload).hexdigest() == e['sha256'], 'archive member hash mismatch')
            result[member.name] = payload
    v.require(seen == set(expected), 'archive missing member')
    v.require(hashlib.sha256(result[h['validator']]).hexdigest() == h['validator_sha256'],
              'historical validator identity mismatch')
    return result


def historical_data(members):
    data = {}
    for name, columns in v.SCHEMAS.items():
        key = 'docs/project-index/' + name
        if key not in members: continue
        reader = csv.DictReader(io.StringIO(members[key].decode('utf-8')), delimiter='\t')
        v.require(tuple(reader.fieldnames or ()) == columns, 'historical header mismatch')
        data[name] = list(reader)
        v.require(all(None not in r and all(x is not None for x in r.values()) for r in data[name]),
                  'historical malformed columns')
    return data


def preserve(current, historical):
    keys = {'sources.tsv':'source_id', 'source-relations.tsv':'relation_id',
            'findings.tsv':'finding_id', 'evidence-links.tsv':'link_id'}
    for name, rows in historical.items():
        key = keys[name]
        actual = {r[key]: r for r in current[name]}
        for row in rows:
            v.require(row[key] in actual, f'historical missing record: {row[key]}')
            v.require(actual[row[key]] == row, f'historical field mismatch: {row[key]}')


def oracle_module(members, contract):
    name = contract['historical']['validator']
    code = members[name]
    v.require(hashlib.sha256(code).hexdigest() == contract['historical']['validator_sha256'],
              'historical validator identity mismatch')
    namespace = {'__name__':'historical_oracle', '__file__':'/fixture/index/validate.py'}
    exec(compile(code, name, 'exec'), namespace)
    # Only input environment changes: Path, HERE and CLI parsing. Rules remain original.
    namespace['Path'] = MemoryPath
    namespace['HERE'] = MemoryPath('/fixture/index')
    return namespace


def serialize(data, schemas):
    files = {}
    for name, columns in schemas.items():
        output = io.StringIO(newline='')
        writer = csv.DictWriter(output, fieldnames=columns, delimiter='\t')
        writer.writeheader(); writer.writerows(data[name])
        files['/fixture/index/' + name] = output.getvalue().encode()
    return files


def oracle_result(oracle, data, files, check_originals=False):
    MemoryPath.files = dict(files, **serialize(data, oracle['SCHEMAS']))
    MemoryPath.links = {}
    args = SimpleNamespace(check_originals=check_originals, repo_root=MemoryPath('/repo') if check_originals else None)
    with patch.object(argparse.ArgumentParser, 'parse_args', return_value=args), contextlib.redirect_stdout(io.StringIO()):
        try:
            oracle['main']()
            return 'PASS'
        except oracle['Invalid'] as error:
            return str(error)


def current_result(data, files, check_originals=False):
    MemoryPath.files = files.copy(); MemoryPath.links = {}
    try:
        v.structure(data)
        if check_originals:
            with contextlib.redirect_stdout(io.StringIO()):
                v.originals(data, MemoryPath('/repo'))
        return 'PASS'
    except v.Invalid as error:
        return str(error)


def corpus(historical):
    """Explicit finite corpus. Mutation and intended error are individually specified."""
    base = copy.deepcopy(historical)
    files = {'/repo/register.md':b'register\n', '/repo/original.log':b'original\n'}
    digest = hashlib.sha256(files['/repo/register.md']).hexdigest()
    for r in base['sources.tsv']:
        r.update(record_locator='register.md#L1', record_sha256=digest,
                 locator='https://example.invalid/' + r['source_id'], sha256='',
                 verification_status='HISTORICAL_CATALOG_CHECK')
    for r in base['findings.tsv']: r.update(record_locator='register.md#L1', record_sha256=digest)
    for r in base['source-relations.tsv']: r.update(basis_locator='register.md#L1', basis_sha256=digest)
    for r in base['evidence-links.tsv']: r['locator_detail']='register.md#L1'
    cases = [('unchanged-historical-structure', copy.deepcopy(historical), {}, False, 'PASS'),
             ('controlled-originals', copy.deepcopy(base), files, True, 'PASS')]
    mutations = [
        ('duplicate-source', 'sources.tsv', None, None, 'source_id: duplicate ID'),
        ('source-status','sources.tsv','verification_status','PASS','invalid verification status'),
        ('namespace-key','sources.tsv','namespace','','incomplete source'),
        ('source-hash','sources.tsv','sha256','bad','source.sha256: invalid SHA256'),
        ('register-hash-syntax','sources.tsv','record_sha256','bad','source.record_sha256: invalid SHA256'),
        ('relation-reference','source-relations.tsv','source_id_b','absent','invalid relation references'),
        ('relation-type','source-relations.tsv','relation','PRIMARY','invalid relation type'),
        ('relation-status','source-relations.tsv','status','PASS','invalid relation status'),
        ('relation-basis','source-relations.tsv','basis_locator','','relation missing basis'),
        ('finding-status','findings.tsv','status','PASS','invalid finding status'),
        ('finding-provenance','findings.tsv','record_locator','','finding missing provenance'),
        ('evidence-reference','evidence-links.tsv','finding_id','absent','invalid evidence references'),
        ('evidence-role','evidence-links.tsv','relation','PRIMARY','invalid evidence relation'),
        ('evidence-locator','evidence-links.tsv','locator_detail','','evidence missing locator'),
        ('path-traversal','sources.tsv','record_locator','../outside','unsafe path'),
        ('absolute-path','sources.tsv','record_locator','/outside','unsafe path'),
        ('line-range','sources.tsv','record_locator','register.md#L2','line range invalid'),
        ('register-hash','sources.tsv','record_sha256','0'*64,'register hash mismatch'),
    ]
    for label, name, field, value, expected in mutations:
        data = copy.deepcopy(base)
        if field is None: data[name].append(copy.deepcopy(data[name][0]))
        else: data[name][0][field] = value
        cases.append((label, data, files, True, expected))
    for label, digest, expected in [('local-original', hashlib.sha256(b'original\n').hexdigest(), 'PASS'),
                                     ('original-hash', '0'*64, 'artifact hash mismatch'),
                                     ('original-missing-hash', '', 'local artifact missing hash')]:
        data = copy.deepcopy(base)
        data['sources.tsv'][0].update(locator='original.log', sha256=digest)
        cases.append((label, data, files, True, expected))
    data = copy.deepcopy(base)
    data['findings.tsv'].append(copy.deepcopy(data['findings.tsv'][0]))
    cases.append(('duplicate-finding', data, files, True, 'finding_id: duplicate ID'))
    return cases


def error_class(result, expected):
    """Match a complete rule message or its locator/identity suffix, never substrings."""
    return result == expected or result.startswith(expected + ': ')


def compare_rules(oracle, historical):
    count = 0
    for label, data, files, originals, expected in corpus(historical):
        old = oracle_result(oracle, data, files, originals)
        new = current_result(data, files, originals)
        # v0.3 detects local missing hashes already at structure time.
        if label == 'original-missing-hash':
            v.require(error_class(old, expected) and error_class(new, 'local source missing required hash'),
                      f'rule tightening mismatch: {label}: {old!r} / {new!r}')
        else:
            v.require((old == new == 'PASS') if expected == 'PASS' else
                      (error_class(old, expected) and error_class(new, expected)),
                      f'rule corpus mismatch: {label}: {old!r} / {new!r}')
        count += 1
    # Legitimate register exception: old originals rejects absent original hash;
    # new originals checks the register, without claiming original bytes.
    data = {name:[] for name in v.SCHEMAS}
    row = copy.deepcopy(v.load_contract()['register_exceptions'][0])
    row.update(title='fixture', revision='', sha256='', record_sha256=hashlib.sha256(b'register\n').hexdigest())
    data['sources.tsv']=[row]
    path = '/repo/' + row['record_locator'].split('#')[0]
    files = {path: b'line\n'*59+b'register\n'}
    row['record_sha256']=hashlib.sha256(files[path]).hexdigest()
    old = oracle_result(oracle, data, files, True)
    new = current_result(data, files, True)
    v.require(error_class(old, 'local artifact missing hash') and new == 'PASS',
              f'register extension mismatch: {old!r} / {new!r}')
    return count


def direct_controls(oracle, historical):
    """Independent parser/path/original entry points; no preceding structure checks.

    These named controls do not claim the unavailable INDEX-1K-14 H01–H35 mapping.
    Unexpected exceptions propagate; only each implementation's Invalid is a rule result.
    """
    def result(call, invalid):
        try:
            with contextlib.redirect_stdout(io.StringIO()): call()
            return 'PASS'
        except invalid as error:
            return str(error)

    count = 0
    def compare(label, old_call, new_call, expected):
        nonlocal count
        old = result(old_call, oracle['Invalid'])
        new = result(new_call, v.Invalid)
        v.require((old == new == 'PASS') if expected == 'PASS' else
                  error_class(old, expected) and error_class(new, expected),
                  f'direct rule mismatch: {label}: {old!r} / {new!r}')
        count += 1

    for name, columns in oracle['SCHEMAS'].items():
        raw = serialize(historical, oracle['SCHEMAS'])['/fixture/index/' + name]
        header, body = raw.split(b'\n', 1)
        first = body.split(b'\n', 1)[0].removesuffix(b'\r')
        for label, payload, expected in (
                ('valid', raw, 'PASS'),
                ('header', b'wrong\n' + body, f'{name}: unexpected header'),
                ('extra-column', header + b'\n' + first + b'\textra\n', f'{name}:2: malformed columns'),
                ('missing-column', header + b'\n' + first.rsplit(b'\t', 1)[0] + b'\n', f'{name}:2: malformed columns')):
            MemoryPath.files = {'/fixture/index/' + name: payload}
            compare(f'raw-{name}-{label}', lambda: oracle['table'](name, columns),
                    lambda: v.table(name, columns, MemoryPath('/fixture/index')), expected)
            if label == 'valid':
                v.require(oracle['table'](name, columns) ==
                          v.table(name, columns, MemoryPath('/fixture/index')) == historical[name],
                          f'direct parser data mismatch: {name}')

    files = {'/repo/register.md': b'register\n', '/repo/original.log': b'original\n',
             '/outside': b'outside\n'}
    MemoryPath.files = files; MemoryPath.links = {'/repo/escape': '/outside'}
    for locator, expected in (
            ('register.md#L1', 'PASS'), ('', 'not a local locator'),
            ('https://example.invalid/x', 'not a local locator'),
            ('register.md#L0', 'invalid locator'), ('../outside', 'unsafe path'),
            ('/outside', 'unsafe path'), ('escape', 'path escapes repo'),
            ('missing', 'file missing'), ('register.md#L2', 'line range invalid'),
            ('register.md#L2-L1', 'line range invalid')):
        compare('local_path-' + locator,
                lambda: oracle['local_path'](MemoryPath('/repo'), locator),
                lambda: v.local_path(MemoryPath('/repo'), locator), expected)

    base = copy.deepcopy(corpus(historical)[1][1])
    digest = hashlib.sha256(b'original\n').hexdigest()
    for label, table, field, value, expected in (
            ('valid', 'sources.tsv', 'locator', 'https://example.invalid/x', 'PASS'),
            ('register-hash', 'sources.tsv', 'record_sha256', '0'*64, 'register hash mismatch'),
            ('finding-hash', 'findings.tsv', 'record_sha256', '0'*64, 'register hash mismatch'),
            ('basis-hash', 'source-relations.tsv', 'basis_sha256', '0'*64, 'basis hash mismatch'),
            ('original-hash', 'sources.tsv', 'sha256', '0'*64, 'artifact hash mismatch'),
            ('original-empty-hash', 'sources.tsv', 'sha256', '', 'local artifact missing hash'),
            ('evidence-path', 'evidence-links.tsv', 'locator_detail', 'missing', 'file missing')):
        data = copy.deepcopy(base)
        if label.startswith('original-'):
            data['sources.tsv'][0].update(locator='original.log', sha256=digest)
        data[table][0][field] = value
        MemoryPath.files = files; MemoryPath.links = {}
        compare('originals-' + label, lambda: oracle['originals'](data, MemoryPath('/repo')),
                lambda: v.originals(data, MemoryPath('/repo')), expected)
    return count


def isolated_cases(historical):
    """INDEX-1K-17 gaps, separate from the preserved 24-case corpus.

    Dependent rows are omitted only for empty source/finding IDs, so dangling
    references cannot turn the single intended violation into a combined case.
    """
    base = copy.deepcopy(corpus(historical)[1][1])
    files = corpus(historical)[1][2]
    cases = []
    for name, field in [('sources.tsv', 'source_id'), ('findings.tsv', 'finding_id'),
                        ('source-relations.tsv', 'relation_id'), ('evidence-links.tsv', 'link_id')]:
        data = copy.deepcopy(base)
        if name == 'sources.tsv':
            data['source-relations.tsv'] = []; data['evidence-links.tsv'] = []
        if name == 'findings.tsv': data['evidence-links.tsv'] = []
        data[name][0][field] = ''
        cases.append(('empty-' + field, data, files, False, field + ': empty ID'))
    for name, field in [('source-relations.tsv', 'relation_id'), ('evidence-links.tsv', 'link_id')]:
        data = copy.deepcopy(base)
        data[name].append(copy.deepcopy(data[name][0]))
        cases.append(('duplicate-' + field, data, files, False, field + ': duplicate ID'))
    data = copy.deepcopy(base)
    for field in ('namespace', 'original_id'):
        data['sources.tsv'][1][field] = data['sources.tsv'][0][field]
    cases.append(('namespaced-duplicate', data, files, False, 'duplicate namespaced original ID'))
    for label, name, field, value, expected in (
            ('source-original-id', 'sources.tsv', 'original_id', '', 'incomplete source'),
            ('source-record-locator', 'sources.tsv', 'record_locator', '', 'incomplete source'),
            ('source-record-hash', 'sources.tsv', 'record_sha256', '', 'incomplete source'),
            ('basis-hash-syntax', 'source-relations.tsv', 'basis_sha256', 'bad', 'relation.basis_sha256: invalid SHA256'),
            ('finding-hash-syntax', 'findings.tsv', 'record_sha256', 'bad', 'finding.record_sha256: invalid SHA256'),
            ('relation-source-a', 'source-relations.tsv', 'source_id_a', 'absent', 'invalid relation references'),
            ('relation-self', 'source-relations.tsv', 'source_id_b', base['source-relations.tsv'][0]['source_id_a'], 'invalid relation references'),
            ('basis-empty-hash', 'source-relations.tsv', 'basis_sha256', '', 'relation missing basis'),
            ('finding-empty-hash', 'findings.tsv', 'record_sha256', '', 'finding missing provenance'),
            ('evidence-source', 'evidence-links.tsv', 'source_id', 'absent', 'invalid evidence references')):
        data = copy.deepcopy(base)
        data[name][0][field] = value
        cases.append((label, data, files, False, expected))
    return cases


def compare_isolated(oracle, historical):
    for label, data, files, originals, expected in isolated_cases(historical):
        old = oracle_result(oracle, data, files, originals)
        new = current_result(data, files, originals)
        v.require(error_class(old, expected) and error_class(new, expected),
                  f'isolated rule mismatch: {label}: {old!r} / {new!r}')
    # No rows/files: a removed root guard would otherwise return PASS.
    empty = {name: [] for name in v.SCHEMAS}
    MemoryPath.files = {}; MemoryPath.links = {}
    old = rule_result(lambda: oracle['originals'](empty, MemoryPath('/absent')), oracle['Invalid'])
    new = rule_result(lambda: v.originals(empty, MemoryPath('/absent')), v.Invalid)
    v.require(error_class(old, 'repository absent') and error_class(new, 'repository absent'),
              f'isolated rule mismatch: repository-absent: {old!r} / {new!r}')
    return len(isolated_cases(historical)) + 1


def rule_result(call, invalid):
    """Only Invalid is a rule outcome; IO and unexpected exceptions propagate."""
    try:
        with contextlib.redirect_stdout(io.StringIO()): call()
        return 'PASS'
    except invalid as error:
        return str(error)


def register_cases(contract):
    """Direct originals comparisons avoid structure masking and TSV coercion.

    Non-string hash values deliberately exercise the Python entry point, not
    representable TSV values. No contract exception is added or rewritten.
    """
    payload = b'register\n' * 66
    files = {'/repo/docs/QUELLENVERZEICHNIS.md': payload}
    cases = []
    for exception in contract['register_exceptions']:
        base = {name: [] for name in v.SCHEMAS}
        row = copy.deepcopy(exception)
        row.update(title='fixture', revision='', sha256='',
                   record_sha256=hashlib.sha256(payload).hexdigest())
        base['sources.tsv'] = [row]
        prefix = exception['source_id'] + '-'
        cases.append((prefix + 'authorized', copy.deepcopy(base), files, 'local artifact missing hash', 'PASS'))
        changes = {field: value + 'changed' for field, value in exception.items()}
        changes.update(locator='docs/QUELLENVERZEICHNIS.md#L1',
                       record_locator='docs/QUELLENVERZEICHNIS.md#L1',
                       verification_status='DOCUMENTED_READ_SCOPE')
        for field, value in changes.items():
            data = copy.deepcopy(base); data['sources.tsv'][0][field] = value
            cases.append((prefix + field, data, files, 'local artifact missing hash',
                          'register exception identity unauthorized'))
        for source_type in sorted(v.REGISTER_TYPES - {exception['source_type']}):
            data = copy.deepcopy(base); data['sources.tsv'][0]['source_type'] = source_type
            cases.append((prefix + 'type-' + source_type, data, files, 'local artifact missing hash',
                          'register exception identity unauthorized'))
        data = copy.deepcopy(base)
        data['sources.tsv'][0].update(source_type='serial_log', locator='https://example.invalid/original')
        cases.append((prefix + 'reserved-http-bypass', data, files, 'PASS', 'register exception identity unauthorized'))
        for identity in ['SRC-999999'] + [f'SRC-{n:06d}' for n in range(1, 6)]:
            data = copy.deepcopy(base); data['sources.tsv'][0]['source_id'] = identity
            cases.append((prefix + identity, data, files, 'local artifact missing hash',
                          'register exception identity unauthorized'))
        for label, digest in [('zeros', '0'*64), ('bad', 'bad'), ('none', None),
                              ('false', False), ('zero', 0), ('space', ' '),
                              ('matching', hashlib.sha256(payload).hexdigest())]:
            data = copy.deepcopy(base); data['sources.tsv'][0]['sha256'] = digest
            old = 'local artifact missing hash' if not digest else ('PASS' if label == 'matching' else 'artifact hash mismatch')
            cases.append((prefix + 'hash-' + label, data, files, old, 'register source misclassified as original'))
    return cases


def compare_registers(oracle, contract):
    cases = register_cases(contract)
    for label, data, files, expected_old, expected_new in cases:
        if label.endswith('-authorized'):
            # Full parser/structure/original path on both sides for each exception.
            old = oracle_result(oracle, data, files, True)
            MemoryPath.files = dict(files, **serialize(data, v.SCHEMAS))
            MemoryPath.links = {}
            parsed = {name: v.table(name, columns, MemoryPath('/fixture/index'))
                      for name, columns in v.SCHEMAS.items()}
            v.require(parsed == data, f'register parser data mismatch: {label}')
            new = current_result(parsed, files, True)
            v.require(error_class(old, expected_old) and new == expected_new,
                      f'register full path mismatch: {label}: {old!r} / {new!r}')
        MemoryPath.files = files.copy(); MemoryPath.links = {}
        old = rule_result(lambda: oracle['originals'](data, MemoryPath('/repo')), oracle['Invalid'])
        new = rule_result(lambda: v.originals(data, MemoryPath('/repo')), v.Invalid)
        v.require(error_class(old, expected_old) and error_class(new, expected_new),
                  f'register comparison mismatch: {label}: {old!r} / {new!r}')
    return len(cases)


def run(current=None, raw=None, contract=None):
    contract = v.load_contract() if contract is None else contract
    raw = Path(contract['historical']['path']).read_bytes() if raw is None else raw
    members = authenticated_archive(raw, contract)
    historical = historical_data(members)
    if current is None:
        current = {name:v.table(name, columns) for name, columns in v.SCHEMAS.items()}
    preserve(current, historical)
    oracle = oracle_module(members, contract)
    count = compare_rules(oracle, historical)
    direct_count = direct_controls(oracle, historical)
    compare_isolated(oracle, historical)
    compare_registers(oracle, contract)
    v.structure(current); v.project_contract(current)
    print('ARCHIVE_IDENTITY=PASS HISTORICAL_DATA_IDENTITY=PASS HISTORICAL_VALIDATOR_IDENTITY=PASS')
    print(f'HISTORICAL_RULE_CORPUS=PASS cases={count} REGISTER_EXTENSION=PASS')
    print(f'HISTORICAL_DIRECT_CONTROLS=PASS cases={direct_count}')
    print('V02_REGRESSION=PASS (finite corpus; no complete semantic equivalence claim)')


if __name__ == '__main__':
    try:
        run()
    except (v.Invalid, OSError, ValueError, tarfile.TarError, UnicodeError, csv.Error) as error:
        print(f'V02_REGRESSION=FAIL: {error}')
        raise SystemExit(1)
