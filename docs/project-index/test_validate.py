#!/usr/bin/env python3
"""Isolated real validator paths; no production writes or network."""
import copy
import os
import sys
import csv
import hashlib
from pathlib import Path
import io
import tarfile
import json
import regress_v02 as r
import unittest

import validate as v


# INDEX-1K-25: new finite acceptance scope, not an R-/H-original matrix.
# Historical rule references below refer to the preserved validator source
# printed in INDEX-1K-19 (lines 27810ff), also reviewed by INDEX-1K-24.
# These literals must never be regenerated from r.*_cases(), the contract,
# v.REGISTER_TYPES, or the outcomes being checked.
INDEX25_TABLES = ('sources.tsv', 'source-relations.tsv', 'findings.tsv',
                  'evidence-links.tsv', 'artifacts.tsv', 'source-artifacts.tsv')
INDEX25_TYPES = ('artifact_reference', 'artifact_collection', 'missing_original',
                 'primary_source_gap', 'historical_chat')
INDEX25_REGISTERS = (
    ('SRC-000009', 'SPL-001', 'artifact_reference', '60', 'HISTORICAL_CATALOG_CHECK'),
    ('SRC-000010', 'SPL-002', 'artifact_collection', '61', 'HISTORICAL_CATALOG_CHECK'),
    ('SRC-000012', 'SPL-005', 'missing_original', '64', 'CONTENT_UNAVAILABLE'),
    ('SRC-000013', 'SPL-006', 'primary_source_gap', '65', 'CONTENT_UNAVAILABLE'),
    ('SRC-000014', 'SPL-007', 'historical_chat', '66', 'CONTENT_UNAVAILABLE'))
# label, table, field, operation/value, dependent tables removed, old, new, rule
INDEX25_ISOLATED = (
    ('empty-source_id', 'sources.tsv', 'source_id', '', ('source-relations.tsv', 'evidence-links.tsv'), 'source_id: empty ID', 'source_id: empty ID', 'unique/source_id'),
    ('empty-finding_id', 'findings.tsv', 'finding_id', '', ('evidence-links.tsv',), 'finding_id: empty ID', 'finding_id: empty ID', 'unique/finding_id'),
    ('empty-relation_id', 'source-relations.tsv', 'relation_id', '', (), 'relation_id: empty ID', 'relation_id: empty ID', 'unique/relation_id'),
    ('empty-link_id', 'evidence-links.tsv', 'link_id', '', (), 'link_id: empty ID', 'link_id: empty ID', 'unique/link_id'),
    ('duplicate-relation_id', 'source-relations.tsv', '', 'duplicate', (), 'relation_id: duplicate ID', 'relation_id: duplicate ID', 'unique/relation_id'),
    ('duplicate-link_id', 'evidence-links.tsv', '', 'duplicate', (), 'link_id: duplicate ID', 'link_id: duplicate ID', 'unique/link_id'),
    ('namespaced-duplicate', 'sources.tsv', '', 'namespace-pair', (), 'duplicate namespaced original ID', 'duplicate namespaced original ID', 'main/structure namespace pair'),
    ('source-original-id', 'sources.tsv', 'original_id', '', (), 'incomplete source', 'incomplete source', 'main/structure source completeness'),
    ('source-record-locator', 'sources.tsv', 'record_locator', '', (), 'incomplete source', 'incomplete source', 'main/structure source completeness; later current locator syntax also fails'),
    ('source-record-hash', 'sources.tsv', 'record_sha256', '', (), 'incomplete source', 'incomplete source', 'main/structure source completeness'),
    ('basis-hash-syntax', 'source-relations.tsv', 'basis_sha256', 'bad', (), 'relation.basis_sha256: invalid SHA256', 'relation.basis_sha256: invalid SHA256', 'hash_field/relation basis'),
    ('finding-hash-syntax', 'findings.tsv', 'record_sha256', 'bad', (), 'finding.record_sha256: invalid SHA256', 'finding.record_sha256: invalid SHA256', 'hash_field/finding record'),
    ('relation-source-a', 'source-relations.tsv', 'source_id_a', 'absent', (), 'invalid relation references', 'invalid relation references', 'main/structure relation source A membership'),
    ('relation-self', 'source-relations.tsv', 'source_id_b', 'SRC-000004', (), 'invalid relation references', 'invalid relation references', 'main/structure relation inequality'),
    ('basis-empty-hash', 'source-relations.tsv', 'basis_sha256', '', (), 'relation missing basis', 'relation missing basis', 'main/structure relation completeness'),
    ('finding-empty-hash', 'findings.tsv', 'record_sha256', '', (), 'finding missing provenance', 'finding missing provenance', 'main/structure finding completeness'),
    ('evidence-source', 'evidence-links.tsv', 'source_id', 'absent', (), 'invalid evidence references', 'invalid evidence references', 'main/structure evidence source membership'))


class ValidatorTests(unittest.TestCase):
    def setUp(self):
        self.base = r.MemoryPath('/fixture')
        self.repo = r.MemoryPath('/repo')
        self.ref = r.MemoryPath('/ref')
        exception = copy.deepcopy(v.load_contract()['register_exceptions'][0])
        self.register = exception['record_locator'].split('#')[0]
        r.MemoryPath.files = {'/repo/' + self.register:b'register\n'*60,
                              '/ref/keep':b'', '/repo/original.log':b'original\n', '/ref/elf':b'ELF fixture\n'}
        r.MemoryPath.links = {}
        h = v.sha256(self.repo / self.register)
        self.data = {name: [] for name in v.SCHEMAS}
        exception.update(title='Fixture', revision='', sha256='', record_sha256=h)
        self.data['sources.tsv'] = [exception]
        self.data['artifacts.tsv'] = [dict(zip(v.SCHEMAS['artifacts.tsv'], [
            'ART-000001','elf','REF_LOCKED_DEBUG/elf',v.sha256(self.ref / 'elf'),
            str((self.ref / 'elf').stat().st_size),'HISTORICAL_HASH_VERIFIED','',
            exception['record_locator'],h]))]
        self.data['source-artifacts.tsv'] = [dict(zip(v.SCHEMAS['source-artifacts.tsv'], [
            'SAL-000001','SRC-000009','ART-000001','PRIMARY','fixture only']))]
        self.index = r.MemoryPath('/fixture/index')
        r.MemoryPath.files.update(r.serialize(self.data, v.SCHEMAS))
        self.data = {name:v.table(name, columns, self.index) for name,columns in v.SCHEMAS.items()}
        v.structure(self.data)
        with r.contextlib.redirect_stdout(io.StringIO()):
            v.originals(self.data, self.repo, self.ref)

    @property
    def source(self): return self.data['sources.tsv'][0]
    @property
    def artifact(self): return self.data['artifacts.tsv'][0]
    @property
    def link(self): return self.data['source-artifacts.tsv'][0]

    def reject(self, message, originals=False):
        with self.assertRaisesRegex(v.Invalid, message):
            v.structure(self.data)
            if originals:
                v.originals(self.data, self.repo, self.ref)

    def test_N01_duplicate_source_id(self):
        self.data['sources.tsv'].append(copy.deepcopy(self.source)); self.reject('source_id: duplicate')
    def test_N02_duplicate_artifact_id(self):
        self.data['artifacts.tsv'].append(copy.deepcopy(self.artifact)); self.reject('artifact_id: duplicate')
    def test_N03_duplicate_link_id(self):
        self.data['source-artifacts.tsv'].append(copy.deepcopy(self.link)); self.reject('link_id: duplicate')
    def test_N04_dangling_source(self):
        self.link['source_id']='SRC-999999'; self.reject('dangling source')
    def test_N05_dangling_artifact(self):
        self.link['artifact_id']='ART-999999'; self.reject('dangling artifact')
    def test_N06_invalid_relation(self):
        self.link['relation']='SAME_AS'; self.reject('invalid source-artifact relation')
    def test_N07_invalid_status(self):
        self.artifact['verification_status']='PASS'; self.reject('invalid artifact status')
    def test_N08_missing_required_hash(self):
        self.artifact['sha256']=''; self.reject('missing required hash')
    def test_N09_hash_mismatch(self):
        self.artifact['sha256']='0'*64; self.reject('artifact hash mismatch', True)
    def test_N10_size_mismatch(self):
        self.artifact['size_bytes']='999'; self.reject('artifact size mismatch', True)
    def test_N11_missing_artifact(self):
        r.MemoryPath.files.pop('/ref/elf'); self.reject('file missing', True)
    def test_N12_relative_path_traversal(self):
        self.artifact['locator']='REF_LOCKED_DEBUG/../outside'; self.reject('unsafe path')
    def test_N13_absolute_path_outside_root(self):
        self.artifact['locator']='REF_LOCKED_DEBUG/'+str(self.base / 'outside'); self.reject('unsafe path')
    def test_N14_symlink_escape(self):
        r.MemoryPath.files['/fixture/outside']=b'ELF fixture\n'
        r.MemoryPath.files.pop('/ref/elf')
        r.MemoryPath.links['/ref/elf']='/fixture/outside'
        self.reject('path escapes repo', True)
    def test_N15_invalid_provenance_locator(self):
        self.artifact['provenance_locator']='register.md#L0'; self.reject('invalid locator')
    def test_N16_provenance_hash_mismatch(self):
        self.artifact['provenance_sha256']='0'*64; self.reject('provenance hash mismatch', True)
    def test_N17_source_artifact_role_mismatch(self):
        self.link['relation']='COMPONENT'; self.reject('role mismatch')
    def test_N18_missing_original_misclassified(self):
        self.source.update(source_id='SRC-000012',original_id='SPL-005',source_type='missing_original')
        self.source['verification_status']='LOCAL_HASH_VERIFIED'
        self.reject('verified source missing required hash|missing original misclassified')
    def test_N19_local_source_hash_mismatch(self):
        self.source.update(source_id='SRC-999999',namespace='fixture',original_id='one',source_type='serial_log',locator='original.log',sha256='0'*64)
        self.data['source-artifacts.tsv']=[]
        self.reject('artifact hash mismatch: SRC', True)
    def test_N20_register_not_original(self):
        self.source['sha256']='0'*64; self.reject('register source misclassified')
    def test_N21_absolute_without_alias(self):
        self.artifact['locator']=str(self.ref / 'elf'); self.reject('requires REF_LOCKED_DEBUG')
    def test_N22_invalid_hash(self):
        self.artifact['sha256']='xyz'; self.reject('invalid SHA256')
    def test_N23_explicit_root_required(self):
        with self.assertRaisesRegex(v.Invalid, 'requires --ref-locked-debug-root'):
            v.originals(self.data,self.repo)
    def test_N24_missing_report_cannot_be_local_original(self):
        self.source.update(namespace='sources-md',original_id='SPL-005',source_type='serial_log',
                           locator='original.log',sha256=v.sha256(self.repo / 'original.log'),
                           verification_status='LOCAL_HASH_VERIFIED')
        self.data['source-artifacts.tsv']=[]
        self.reject('source original role misclassified')

    def regression_relation(self):
        other = copy.deepcopy(self.source)
        other.update(source_id='SRC-000002', original_id='two')
        self.data['sources.tsv'].append(other)
        self.data['source-relations.tsv']=[dict(zip(v.SCHEMAS['source-relations.tsv'],[
            'REL-000001','SRC-000009','SRC-000002','DIFFERENT_FROM','CONFIRMED',
            self.source['record_locator'],v.sha256(self.repo / self.register)]))]

    def test_v02_relation_reference(self):
        self.regression_relation()
        self.data['source-relations.tsv'][0]['source_id_b']='absent'
        self.reject('invalid relation references')
    def test_v02_relation_type(self):
        self.regression_relation()
        self.data['source-relations.tsv'][0]['relation']='PRIMARY'
        self.reject('invalid relation type')
    def test_v02_relation_status(self):
        self.regression_relation()
        self.data['source-relations.tsv'][0]['status']='PASS'
        self.reject('invalid relation status')
    def test_v02_namespaced_identity(self):
        other=copy.deepcopy(self.source)
        other['source_id']='SRC-000002'
        self.data['sources.tsv'].append(other)
        self.reject('duplicate namespaced original ID')
    def test_v02_source_status(self):
        self.source['verification_status']='PASS'
        self.reject('invalid verification status')
    def test_v02_finding_status(self):
        self.data['findings.tsv']=[dict(zip(v.SCHEMAS['findings.tsv'],[
            'FND-000001','fixture','fixture','PASS','fixture','register.md#L1',
            v.sha256(self.repo / self.register), '']))]
        self.reject('invalid finding status')
    def test_v02_evidence_reference(self):
        self.data['evidence-links.tsv']=[dict(zip(v.SCHEMAS['evidence-links.tsv'],[
            'EVL-000001','absent','SRC-000001','SUPPORTS','register.md#L1','fixture']))]
        self.reject('invalid evidence references')

    def test_positive_fixture(self):
        v.structure(self.data)
        v.originals(self.data,self.repo,self.ref)


class ContractRegressionTests(unittest.TestCase):
    def setUp(self):
        self.contract = v.load_contract()
        self.data = {name:v.table(name, columns) for name,columns in v.SCHEMAS.items()}
        self.raw = Path(self.contract['historical']['path']).read_bytes()
        self.members = r.authenticated_archive(self.raw, self.contract)
        self.historical = r.historical_data(self.members)
        v.structure(self.data); v.project_contract(self.data)
        r.preserve(self.data, self.historical)

    def failure(self, message):
        with self.assertRaisesRegex(v.Invalid, message): v.project_contract(self.data)

    def test_K1_01_header_only(self):
        self.data['artifacts.tsv']=[]; self.data['source-artifacts.tsv']=[]
        v.structure(self.data); self.failure('missing mandatory ART')
    def test_K1_02_each_artifact_missing(self):
        base=copy.deepcopy(self.data)
        for row in base['artifacts.tsv']:
            self.data=copy.deepcopy(base)
            self.data['artifacts.tsv'].remove(row)
            self.data['source-artifacts.tsv']=[x for x in self.data['source-artifacts.tsv'] if x['artifact_id']!=row['artifact_id']]
            v.structure(self.data); self.failure('missing mandatory '+row['artifact_id'])
    def test_K1_03_each_assignment_missing(self):
        base=copy.deepcopy(self.data)
        for row in base['source-artifacts.tsv']:
            self.data=copy.deepcopy(base); self.data['source-artifacts.tsv'].remove(row)
            v.structure(self.data); self.failure('missing mandatory '+row['link_id'])
    def test_K1_04_artifact_id(self):
        self.data['artifacts.tsv'][0]['artifact_id']='ART-999999'
        self.data['source-artifacts.tsv'][0]['artifact_id']='ART-999999'
        v.structure(self.data); self.failure('missing mandatory ART-000001')
    def test_K1_05_assignment_id(self):
        self.data['source-artifacts.tsv'][0]['link_id']='SAL-999999'
        v.structure(self.data); self.failure('missing mandatory SAL-000001')
    def test_K1_06_source_binding(self):
        self.data['source-artifacts.tsv'][1]['source_id']='SRC-000009'
        self.failure('binding mismatch SAL-000002')
    def test_K1_07_artifact_binding(self):
        self.data['source-artifacts.tsv'][1]['artifact_id']='ART-000003'
        v.structure(self.data); self.failure('binding mismatch SAL-000002')
    def test_K1_08_relation_role(self):
        self.data['source-artifacts.tsv'][1]['relation']='PRIMARY'
        self.failure('binding mismatch SAL-000002')
    def test_K1_09_artifact_type(self):
        self.data['artifacts.tsv'][1]['artifact_type']='binary'
        v.structure(self.data); self.failure('binding mismatch ART-000002')
    def test_K1_10_artifact_locator(self):
        self.data['artifacts.tsv'][1]['locator']='REF_LOCKED_DEBUG/other'
        v.structure(self.data); self.failure('binding mismatch ART-000002')

    def test_K2_01_archive_hash(self):
        with self.assertRaisesRegex(v.Invalid,'archive identity'):
            r.authenticated_archive(self.raw+b'x',self.contract)
    def archive_mutation(self, kind, expected):
        # Authenticate deliberately modified outer bytes only to reach the intended
        # inner protection. Production always uses the pinned original contract.
        output=io.BytesIO()
        with tarfile.open(fileobj=output,mode='w') as archive:
            with tarfile.open(fileobj=io.BytesIO(self.raw),mode='r:') as original:
                items=list(original)
                for n,member in enumerate(items):
                    payload=original.extractfile(member).read() if member.isfile() else None
                    if n==1:
                        if kind=='missing': continue
                        if kind=='path': member.name='unexpected'
                        if kind=='link': member.type=tarfile.SYMTYPE; member.linkname='outside'; member.size=0; payload=None
                        if kind=='special': member.type=tarfile.FIFOTYPE; member.size=0; payload=None
                        if kind=='size': payload+=b'x'; member.size+=1
                        if kind=='hash': payload=bytes([payload[0]^1])+payload[1:]
                    archive.addfile(member,io.BytesIO(payload) if payload is not None else None)
                    if n==1 and kind=='duplicate': archive.addfile(member,io.BytesIO(payload))
        contract=copy.deepcopy(self.contract)
        contract['historical']['sha256']=hashlib.sha256(output.getvalue()).hexdigest()
        with self.assertRaisesRegex(v.Invalid,expected): r.authenticated_archive(output.getvalue(),contract)
    def test_K2_02_duplicate(self): self.archive_mutation('duplicate','duplicate member')
    def test_K2_03_links(self): self.archive_mutation('link','type mismatch')
    def test_K2_04_special(self): self.archive_mutation('special','type mismatch')
    def test_K2_05_unexpected_path(self): self.archive_mutation('path','unexpected path')
    def test_K2_06_missing_member(self): self.archive_mutation('missing','missing member')
    def test_K2_07_size(self): self.archive_mutation('size','size mismatch')
    def test_K2_08_member_hash(self): self.archive_mutation('hash','hash mismatch')
    def test_K2_09_validator_hash(self):
        members=self.members.copy(); key=self.contract['historical']['validator']; members[key]+=b'\n'
        with self.assertRaisesRegex(v.Invalid,'validator identity'): r.oracle_module(members,self.contract)
    def test_K2_10_missing_SRC1(self):
        self.data['sources.tsv']=[x for x in self.data['sources.tsv'] if x['source_id']!='SRC-000001']
        v.structure(self.data)
        with self.assertRaisesRegex(v.Invalid,'missing record: SRC-000001'): r.preserve(self.data,self.historical)
    def test_K2_11_missing_SRC2(self):
        self.data['sources.tsv']=[x for x in self.data['sources.tsv'] if x['source_id']!='SRC-000002']
        v.structure(self.data)
        with self.assertRaisesRegex(v.Invalid,'missing record: SRC-000002'): r.preserve(self.data,self.historical)
    def test_K2_12_all_historical_fields(self):
        for name,rows in self.historical.items():
            for row in rows:
                for field in row:
                    current=copy.deepcopy(self.data)
                    key=next(iter(row))
                    target=next(x for x in current[name] if x[key]==row[key])
                    target[field]+='changed'
                    with self.assertRaisesRegex(v.Invalid,'historical (field mismatch|missing record)'):
                        r.preserve(current,self.historical)

    def test_contract_missing_corrupt_unknown(self):
        path=r.MemoryPath('/contract')
        valid=Path(v.HERE/'baseline-contract.json').read_bytes()
        r.MemoryPath.files={'/contract':valid}
        self.assertEqual(v.load_contract(path),self.contract)
        for raw in (b'{', b'{}', valid.replace(b'novena-project-index-contract-1',b'unknown'), valid+b' '):
            r.MemoryPath.files={'/contract':raw}
            with self.assertRaisesRegex(v.Invalid,'contract:'): v.load_contract(path)
        r.MemoryPath.files={}
        with self.assertRaisesRegex(v.Invalid,'contract:'): v.load_contract(path)

    def test_register_identity_guard(self):
        for original in self.contract['register_exceptions']:
            for field in original:
                data=copy.deepcopy(self.data)
                row=next(x for x in data['sources.tsv'] if x['source_id']==original['source_id'])
                row[field]+='changed'
                # Type mutation removes exception eligibility; source role rule
                # detects that case instead of the skip guard.
                if field=='source_type':
                    with self.assertRaisesRegex(v.Invalid,'source original role misclassified'): v.structure(data)
                else:
                    with self.assertRaisesRegex(v.Invalid,'unauthorized'): v.register_guard(data)
        for number in range(1,6):
            data=copy.deepcopy(self.data)
            row=data['sources.tsv'][number-1]
            row.update(self.contract['register_exceptions'][0],source_id=f'SRC-{number:06d}')
            with self.assertRaisesRegex(v.Invalid,'unauthorized'): v.register_guard(data)

    def test_positive_additional_records(self):
        row=copy.deepcopy(self.data['sources.tsv'][0]); row.update(source_id='SRC-999999',original_id='additional')
        self.data['sources.tsv'].append(row)
        artifact=copy.deepcopy(self.data['artifacts.tsv'][1]); artifact['artifact_id']='ART-999999'
        self.data['artifacts.tsv'].append(artifact)
        link=copy.deepcopy(self.data['source-artifacts.tsv'][1]); link.update(link_id='SAL-999999',artifact_id='ART-999999')
        self.data['source-artifacts.tsv'].append(link)
        v.structure(self.data); v.project_contract(self.data); r.preserve(self.data,self.historical)

    def test_regression_CLI_missing_source_cannot_PASS(self):
        self.data['sources.tsv']=self.data['sources.tsv'][1:]
        output=io.StringIO()
        with r.patch.object(v,'table',side_effect=lambda name, columns, root: self.data[name]), \
             r.patch.object(sys,'argv',['validate.py','--v02-regression']), \
             r.contextlib.redirect_stdout(output):
            with self.assertRaisesRegex(v.Invalid,'historical missing record: SRC-000001'): v.main()
        self.assertNotIn('PASS',output.getvalue())

    def test_actual_rule_result_mismatch_is_detected(self):
        oracle=r.oracle_module(self.members,self.contract)
        with r.patch.object(r,'oracle_result',return_value='PASS'):
            with self.assertRaisesRegex(v.Invalid,'rule corpus mismatch: duplicate-source'):
                r.compare_rules(oracle,self.historical)

    def test_positive_all_register_exceptions(self):
        data={name:[] for name in v.SCHEMAS}
        path='/repo/docs/QUELLENVERZEICHNIS.md'
        files={path:b'register\n'*66}
        for exception in self.contract['register_exceptions']:
            row=copy.deepcopy(exception)
            row.update(title='fixture',revision='',sha256='',record_sha256=hashlib.sha256(files[path]).hexdigest())
            data['sources.tsv'].append(row)
        v.structure(data); v.register_guard(data)
        r.MemoryPath.files=files; r.MemoryPath.links={}
        with r.contextlib.redirect_stdout(io.StringIO()): v.originals(data,r.MemoryPath('/repo'))

    def test_positive_historical_and_rule_corpus(self):
        r.preserve(self.data,self.historical)
        oracle=r.oracle_module(self.members,self.contract)
        self.assertEqual(r.oracle_result(oracle,self.historical,{}),'PASS')
        self.assertEqual(r.compare_rules(oracle,self.historical),24)
        # Production integration, including actual result comparisons.
        with r.contextlib.redirect_stdout(io.StringIO()): r.run(self.data,self.raw,self.contract)

    def current_mutant(self, old, new, function):
        # Only current source is changed, exclusively in memory. Oracle bytes
        # and historical namespace are never mutation targets.
        source=(v.HERE/'validate.py').read_text(encoding='utf-8')
        self.assertEqual(source.count(old),1)
        namespace={'__name__':'current_mutant','__file__':str(v.HERE/'validate.py')}
        exec(compile(source.replace(old,new),'<current-mutant>','exec'),namespace)
        namespace['Invalid']=v.Invalid
        return namespace[function]

    def test_original_23_cases_retained(self):
        # Explicit expectations, independent of the corpus generator.
        expected=[
            ('unchanged-historical-structure','PASS'), ('controlled-originals','PASS'),
            ('duplicate-source','source_id: duplicate ID'),
            ('source-status','invalid verification status'), ('namespace-key','incomplete source'),
            ('source-hash','source.sha256: invalid SHA256'),
            ('register-hash-syntax','source.record_sha256: invalid SHA256'),
            ('relation-reference','invalid relation references'), ('relation-type','invalid relation type'),
            ('relation-status','invalid relation status'), ('relation-basis','relation missing basis'),
            ('finding-status','invalid finding status'), ('finding-provenance','finding missing provenance'),
            ('evidence-reference','invalid evidence references'), ('evidence-role','invalid evidence relation'),
            ('evidence-locator','evidence missing locator'), ('path-traversal','unsafe path'),
            ('absolute-path','unsafe path'), ('line-range','line range invalid'),
            ('register-hash','register hash mismatch'), ('local-original','PASS'),
            ('original-hash','artifact hash mismatch'), ('original-missing-hash','local artifact missing hash')]
        cases=r.corpus(self.historical)
        self.assertEqual([(c[0],c[4]) for c in cases[:23]],expected)
        self.assertEqual([(c[0],c[4]) for c in cases[23:]],
                         [('duplicate-finding','finding_id: duplicate ID')])

    def test_duplicate_finding_isolated(self):
        data=copy.deepcopy(self.historical)
        data['findings.tsv'].append(copy.deepcopy(data['findings.tsv'][0]))
        oracle=r.oracle_module(self.members,self.contract)
        self.assertEqual(r.oracle_result(oracle,data,{}),'finding_id: duplicate ID')
        self.assertEqual(r.current_result(data,{}),'finding_id: duplicate ID')

    def test_finding_unique_removal_compare_and_run_fail(self):
        mutant=self.current_mutant("findings = unique(data['findings.tsv'], 'finding_id')",
            "findings = set(row['finding_id'] for row in data['findings.tsv'])",'structure')
        oracle=r.oracle_module(self.members,self.contract)
        with r.patch.object(v,'structure',mutant):
            with self.assertRaisesRegex(v.Invalid,'rule corpus mismatch: duplicate-finding'):
                r.compare_rules(oracle,self.historical)
            output=io.StringIO()
            with r.contextlib.redirect_stdout(output):
                with self.assertRaisesRegex(v.Invalid,'rule corpus mismatch: duplicate-finding'):
                    r.run(self.data,self.raw,self.contract)
            self.assertNotIn('V02_REGRESSION=PASS',output.getvalue())

    def test_wrong_rule_class_rejected(self):
        oracle=r.oracle_module(self.members,self.contract)
        actual=r.current_result
        def wrong(data,files,originals=False):
            answer=actual(data,files,originals)
            return 'other rule: finding_id: duplicate ID' if answer=='finding_id: duplicate ID' else answer
        with r.patch.object(r,'current_result',side_effect=wrong):
            with self.assertRaisesRegex(v.Invalid,'rule corpus mismatch: duplicate-finding'):
                r.compare_rules(oracle,self.historical)

    def test_oserror_not_rule_result(self):
        oracle=r.oracle_module(self.members,self.contract)
        def fail(): raise OSError('finding_id: duplicate ID')
        with r.patch.dict(oracle,main=fail):
            with self.assertRaises(OSError): r.oracle_result(oracle,self.historical,{})
        with r.patch.object(v,'structure',side_effect=OSError('finding_id: duplicate ID')):
            with self.assertRaises(OSError): r.current_result(self.historical,{})

    def test_unexpected_exception_not_rule_result(self):
        with r.patch.object(v,'structure',side_effect=RuntimeError('invalid finding status')):
            with self.assertRaises(RuntimeError): r.current_result(self.historical,{})

    def test_direct_parser_path_original_controls(self):
        oracle=r.oracle_module(self.members,self.contract)
        self.assertEqual(r.direct_controls(oracle,self.historical),33)

    def test_raw_parser_removal_detected(self):
        oracle=r.oracle_module(self.members,self.contract)
        def unchecked(name,columns,index_root=v.HERE):
            with (index_root/name).open(encoding='utf-8',newline='') as handle:
                return list(csv.DictReader(handle,delimiter='\t'))
        with r.patch.object(v,'table',side_effect=unchecked):
            with self.assertRaisesRegex(v.Invalid,'direct rule mismatch: raw-sources.tsv-header'):
                r.direct_controls(oracle,self.historical)

    def test_direct_path_rule_removal_detected(self):
        mutant=self.current_mutant(
            "    require(candidate.is_relative_to(repo), f'path escapes repo: {locator}')\n",
            '', 'local_path')
        oracle=r.oracle_module(self.members,self.contract)
        with r.patch.object(v,'local_path',mutant):
            with self.assertRaisesRegex(v.Invalid,'direct rule mismatch: local_path-escape'):
                r.direct_controls(oracle,self.historical)

    def test_direct_original_hash_rule_removal_detected(self):
        mutant=self.current_mutant(
            "            require(bool(row['sha256']), f'local artifact missing hash: {row[\"source_id\"]}')\n",
            '', 'originals')
        oracle=r.oracle_module(self.members,self.contract)
        with r.patch.object(v,'originals',mutant):
            with self.assertRaisesRegex(v.Invalid,'direct rule mismatch: originals-original-empty-hash'):
                r.direct_controls(oracle,self.historical)

    def test_INDEX18_isolated_historical_comparisons(self):
        oracle=r.oracle_module(self.members,self.contract)
        cases=r.isolated_cases(self.historical)
        self.assertEqual([case[0] for case in cases], [
            'empty-source_id', 'empty-finding_id', 'empty-relation_id', 'empty-link_id',
            'duplicate-relation_id', 'duplicate-link_id', 'namespaced-duplicate',
            'source-original-id', 'source-record-locator', 'source-record-hash',
            'basis-hash-syntax', 'finding-hash-syntax', 'relation-source-a',
            'relation-self', 'basis-empty-hash', 'finding-empty-hash', 'evidence-source'])
        for label,data,files,originals,expected in cases:
            with self.subTest(case=label):
                self.assertTrue(r.error_class(r.oracle_result(oracle,data,files,originals),expected))
                self.assertTrue(r.error_class(r.current_result(data,files,originals),expected))
        self.assertEqual(r.compare_isolated(oracle,self.historical),18)

    def test_INDEX18_each_isolated_rule_bypass_detected(self):
        oracle=r.oracle_module(self.members,self.contract)
        actual_require=v.require
        for label,data,files,originals,expected in r.isolated_cases(self.historical):
            with self.subTest(case=label):
                hits=[]
                def bypass(condition,message):
                    if message == expected:
                        hits.append(message)
                    else:
                        actual_require(condition,message)
                # Only the selected current predicate is bypassed in memory.
                # Later tightening may reject too, but must not impersonate the
                # removed historical rule's exact error class.
                with r.patch.object(v,'require',side_effect=bypass):
                    answer=r.current_result(data,files,originals)
                    self.assertTrue(hits)
                    self.assertFalse(r.error_class(answer,expected))
                    with self.assertRaisesRegex(v.Invalid,'^isolated rule mismatch:'):
                        r.compare_isolated(oracle,self.historical)

    def test_INDEX18_repository_guard_removal_detected(self):
        mutant=self.current_mutant("    require(repo.is_dir(), f'repository absent: {repo}')\n", '', 'originals')
        oracle=r.oracle_module(self.members,self.contract)
        with r.patch.object(v,'originals',mutant):
            with self.assertRaisesRegex(v.Invalid,'^isolated rule mismatch: repository-absent:'):
                r.compare_isolated(oracle,self.historical)

    def test_INDEX18_all_register_historical_paths(self):
        oracle=r.oracle_module(self.members,self.contract)
        cases=r.register_cases(self.contract)
        self.assertEqual([c[0] for c in cases if c[0].endswith('-authorized')],
                         [identity+'-authorized' for identity in
                          ['SRC-000009','SRC-000010','SRC-000012','SRC-000013','SRC-000014']])
        self.assertEqual(len(cases),130)
        self.assertEqual(r.compare_registers(oracle,self.contract),130)

    def test_INDEX18_reserved_id_branch_removal_detected(self):
        mutant=self.current_mutant(
            "if row['source_type'] in REGISTER_TYPES or row['source_id'] in allowed:",
            "if row['source_type'] in REGISTER_TYPES:", 'register_guard')
        oracle=r.oracle_module(self.members,self.contract)
        for label,data,files,old,new in r.register_cases(self.contract):
            if not label.endswith('-reserved-http-bypass'): continue
            r.MemoryPath.files=files.copy(); r.MemoryPath.links={}
            self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),new)
            with r.patch.object(v,'register_guard',mutant):
                self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),'PASS')
        with r.patch.object(v,'register_guard',mutant):
            with self.assertRaisesRegex(v.Invalid,'^register comparison mismatch:'):
                r.compare_registers(oracle,self.contract)

    def test_INDEX18_each_register_field_binding_removal_detected(self):
        oracle=r.oracle_module(self.members,self.contract)
        for field in self.contract['register_exceptions'][0]:
            with self.subTest(field=field):
                if field == 'source_id':
                    # Omitting the lookup key alone still leaves the explicit
                    # source_id comparison; omit that binding as well in memory.
                    source=(v.HERE/'validate.py').read_text(encoding='utf-8')
                    self.assertEqual(source.count("expected = allowed.get(row['source_id'])"),1)
                    self.assertEqual(source.count('all(row[k] == value for k, value in expected.items())'),1)
                    source=source.replace("expected = allowed.get(row['source_id'])",
                        "expected = next((e for e in allowed.values() if all(row[k] == value for k, value in e.items() if k != 'source_id')), None)")
                    source=source.replace('all(row[k] == value for k, value in expected.items())',
                        "all(row[k] == value for k, value in expected.items() if k != 'source_id')")
                    namespace={'__name__':'current_mutant','__file__':str(v.HERE/'validate.py')}
                    exec(compile(source,'<current-mutant>','exec'),namespace)
                    namespace['Invalid']=v.Invalid
                    mutant=namespace['register_guard']
                else:
                    mutant=self.current_mutant('all(row[k] == value for k, value in expected.items())',
                        f'all(row[k] == value for k, value in expected.items() if k != {field!r})', 'register_guard')
                for label,data,files,old,new in r.register_cases(self.contract):
                    if field == 'source_type':
                        if '-type-' not in label: continue
                    elif not label.endswith('-'+field):
                        continue
                    r.MemoryPath.files=files.copy(); r.MemoryPath.links={}
                    self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),new)
                    with r.patch.object(v,'register_guard',mutant):
                        self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),'PASS')
                with r.patch.object(v,'register_guard',mutant):
                    with self.assertRaisesRegex(v.Invalid,'^register comparison mismatch:'):
                        r.compare_registers(oracle,self.contract)

    def test_INDEX18_exact_empty_hash_weakening_detected(self):
        mutant=self.current_mutant("require(row['sha256'] == '', 'register source misclassified as original')",
            "require(not row['sha256'], 'register source misclassified as original')", 'register_guard')
        oracle=r.oracle_module(self.members,self.contract)
        for label,data,files,old,new in r.register_cases(self.contract):
            if not label.endswith(('-hash-none','-hash-false','-hash-zero')): continue
            r.MemoryPath.files=files.copy(); r.MemoryPath.links={}
            self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),new)
            with r.patch.object(v,'register_guard',mutant):
                self.assertEqual(r.rule_result(lambda: v.originals(data,r.MemoryPath('/repo')),v.Invalid),'PASS')
        with r.patch.object(v,'register_guard',mutant):
            with self.assertRaisesRegex(v.Invalid,'^register comparison mismatch:'):
                r.compare_registers(oracle,self.contract)

    def test_INDEX18_direct_comparison_exceptions_propagate(self):
        oracle=r.oracle_module(self.members,self.contract)
        for error in (OSError('register exception identity unauthorized'),
                      RuntimeError('register source misclassified as original')):
            with self.subTest(error=type(error).__name__):
                # Replace only the callable; never rewrite historical bytes.
                with r.patch.dict(oracle, originals=lambda *args: (_ for _ in ()).throw(error)):
                    with self.assertRaises(type(error)): r.compare_registers(oracle,self.contract)
                with r.patch.object(v,'originals',side_effect=error):
                    with self.assertRaises(type(error)): r.compare_registers(oracle,self.contract)

    def assert_INDEX22_isolated_predicate(self, label, table, field, value,
                                          predicate, replacement, expected, changed):
        """Bind one historical violation and one current subpredicate removal."""
        oracle = r.oracle_module(self.members, self.contract)
        base = copy.deepcopy(r.corpus(self.historical)[1][1])
        files = r.corpus(self.historical)[1][2]
        self.assertEqual(r.oracle_result(oracle, base, files), 'PASS')
        self.assertEqual(r.current_result(base, files), 'PASS')
        cases = r.isolated_cases(self.historical)
        selected = [case for case in cases if case[0] == label]
        self.assertEqual(len(selected), 1)
        _, data, case_files, originals, case_expected = selected[0]
        # Independently specify the entire input delta, not just its message.
        isolated = copy.deepcopy(base)
        isolated[table][0][field] = value
        self.assertEqual(data, isolated)
        self.assertEqual(case_files, files)
        self.assertIs(originals, False)
        self.assertEqual(case_expected, expected)
        self.assertEqual(r.oracle_result(oracle, data, files), expected)
        self.assertEqual(r.current_result(data, files), expected)
        self.assertEqual(r.compare_isolated(oracle, self.historical), 18)
        mutant = self.current_mutant(predicate, replacement, 'structure')
        actual = r.current_result
        visited = []

        def observe(case_data, case_files, check_originals=False):
            visited.append(copy.deepcopy(case_data))
            return actual(case_data, case_files, check_originals)

        with r.patch.object(v, 'structure', mutant):
            self.assertEqual(actual(base, files), 'PASS')
            self.assertEqual(actual(data, files), changed)
            # The other four shared-message cases (and all remaining peers)
            # keep their exact rule outcomes under this single mutant.
            for peer_label, peer, peer_files, peer_originals, peer_expected in cases:
                if peer_label != label:
                    self.assertEqual(actual(peer, peer_files, peer_originals), peer_expected)
            with r.patch.object(r, 'current_result', side_effect=observe):
                with self.assertRaises(v.Invalid) as caught:
                    r.compare_isolated(oracle, self.historical)
            self.assertEqual(str(caught.exception),
                             f'isolated rule mismatch: {label}: {expected!r} / {changed!r}')
        position = next(n for n, case in enumerate(cases) if case[0] == label)
        self.assertEqual(visited, [case[1] for case in cases[:position + 1]])

    def test_INDEX22_M01_source_original_id(self):
        self.assert_INDEX22_isolated_predicate(
            'source-original-id', 'sources.tsv', 'original_id', '',
            "row['namespace'] and row['original_id'] and row['record_locator'] and row['record_sha256']",
            "row['namespace'] and row['record_locator'] and row['record_sha256']",
            'incomplete source', 'PASS')

    def test_INDEX22_M01_source_record_locator(self):
        self.assert_INDEX22_isolated_predicate(
            'source-record-locator', 'sources.tsv', 'record_locator', '',
            "row['namespace'] and row['original_id'] and row['record_locator'] and row['record_sha256']",
            "row['namespace'] and row['original_id'] and row['record_sha256']",
            'incomplete source', 'invalid locator: ')

    def test_INDEX22_M01_source_record_hash(self):
        self.assert_INDEX22_isolated_predicate(
            'source-record-hash', 'sources.tsv', 'record_sha256', '',
            "row['namespace'] and row['original_id'] and row['record_locator'] and row['record_sha256']",
            "row['namespace'] and row['original_id'] and row['record_locator']",
            'incomplete source', 'PASS')

    def test_INDEX22_M01_relation_source_a(self):
        self.assert_INDEX22_isolated_predicate(
            'relation-source-a', 'source-relations.tsv', 'source_id_a', 'absent',
            "row['source_id_a'] in sources and row['source_id_b'] in sources and row['source_id_a'] != row['source_id_b']",
            "row['source_id_b'] in sources and row['source_id_a'] != row['source_id_b']",
            'invalid relation references', 'PASS')

    def test_INDEX22_M01_relation_self(self):
        source_a = self.historical['source-relations.tsv'][0]['source_id_a']
        self.assert_INDEX22_isolated_predicate(
            'relation-self', 'source-relations.tsv', 'source_id_b', source_a,
            "row['source_id_a'] in sources and row['source_id_b'] in sources and row['source_id_a'] != row['source_id_b']",
            "row['source_id_a'] in sources and row['source_id_b'] in sources",
            'invalid relation references', 'PASS')

    def test_INDEX22_M02_run_requires_compare_isolated(self):
        mutant = self.current_mutant(
            "row['namespace'] and row['original_id'] and row['record_locator'] and row['record_sha256']",
            "row['namespace'] and row['record_locator'] and row['record_sha256']", 'structure')
        output = io.StringIO()

        def require_detection():
            with r.contextlib.redirect_stdout(output):
                with self.assertRaises(v.Invalid) as caught:
                    r.run(self.data, self.raw, self.contract)
            self.assertEqual(str(caught.exception),
                             "isolated rule mismatch: source-original-id: 'incomplete source' / 'PASS'")
            self.assertNotIn('V02_REGRESSION=PASS', output.getvalue())

        with r.patch.object(v, 'structure', mutant):
            with r.patch.object(r, 'compare_isolated', wraps=r.compare_isolated) as comparison:
                require_detection()
                comparison.assert_called_once()
                self.assertEqual(comparison.call_args.args[1], self.historical)
            # Simulate skipping only this integration stage in memory. The
            # same detection assertion must fail, and run must reach PASS.
            output = io.StringIO()
            with r.patch.object(r, 'compare_isolated', return_value=18):
                with self.assertRaises(self.failureException):
                    require_detection()
            self.assertIn('V02_REGRESSION=PASS', output.getvalue())

    def test_INDEX22_M03_run_requires_compare_registers(self):
        mutant = self.current_mutant(
            "require(row['sha256'] == '', 'register source misclassified as original')",
            "require(not row['sha256'], 'register source misclassified as original')", 'register_guard')
        output = io.StringIO()

        def require_detection():
            with r.contextlib.redirect_stdout(output):
                with self.assertRaises(v.Invalid) as caught:
                    r.run(self.data, self.raw, self.contract)
            self.assertEqual(str(caught.exception),
                             "register comparison mismatch: SRC-000009-hash-none: "
                             "'local artifact missing hash: SRC-000009' / 'PASS'")
            self.assertNotIn('V02_REGRESSION=PASS', output.getvalue())

        with r.patch.object(v, 'register_guard', mutant):
            with r.patch.object(r, 'compare_registers', wraps=r.compare_registers) as comparison:
                require_detection()
                comparison.assert_called_once()
                self.assertEqual(comparison.call_args.args[1], self.contract)
            output = io.StringIO()
            with r.patch.object(r, 'compare_registers', return_value=130):
                with self.assertRaises(self.failureException):
                    require_detection()
            self.assertIn('V02_REGRESSION=PASS', output.getvalue())

    def INDEX25_specs(self, group):
        """Independent inputs and outcomes; shared historical bytes only.

        This is a newly specified finite scope. It does not reconstruct the
        unavailable R-/H-matrix. MemoryPath and result adapters remain shared
        infrastructure; CLI, real filesystem and complete semantics are outside
        this acceptance. No production or regression-driver edits are needed
        for the 18 isolated / 130 register cases instrumented here.

        Remaining inventory gap: this auditor does not instrument the separate
        24-case corpus (plus its extra register-extension comparison), the 33
        direct controls, or every per-field preservation mutation in K2_12.
        Existing tests for them remain unchanged. The missing historical R-/H-
        originals prevent an authoritative mapping to their case identities;
        neither this new manifest nor driver instrumentation can supply it.
        M01-M03 retain only their previously evidenced acceptance scope.
        """
        specs = {}
        if group == 'isolated':
            base = copy.deepcopy(self.historical)
            files = {'/repo/register.md': b'register\n',
                     '/repo/original.log': b'original\n'}
            digest = hashlib.sha256(b'register\n').hexdigest()
            for row in base['sources.tsv']:
                row.update(record_locator='register.md#L1', record_sha256=digest,
                           locator='https://example.invalid/' + row['source_id'],
                           sha256='', verification_status='HISTORICAL_CATALOG_CHECK')
            for row in base['findings.tsv']:
                row.update(record_locator='register.md#L1', record_sha256=digest)
            for row in base['source-relations.tsv']:
                row.update(basis_locator='register.md#L1', basis_sha256=digest)
            for row in base['evidence-links.tsv']:
                row['locator_detail'] = 'register.md#L1'
            # The self-relation target is literal, not copied from its generator.
            self.assertEqual(base['source-relations.tsv'][0]['source_id_a'], 'SRC-000004')
            for label, table, field, value, omitted, old, new, rule in INDEX25_ISOLATED:
                data = copy.deepcopy(base)
                for name in omitted:
                    data[name] = []
                if value == 'duplicate':
                    data[table].append(copy.deepcopy(data[table][0]))
                elif value == 'namespace-pair':
                    data[table][1].update(namespace=data[table][0]['namespace'],
                                          original_id=data[table][0]['original_id'])
                else:
                    data[table][0][field] = value
                specs[label] = (data, files, old, new, False, rule)
            specs['repository-absent'] = (
                {name: [] for name in INDEX25_TABLES}, {},
                'repository absent: /absent', 'repository absent: /absent',
                None, 'originals/repository existence (direct, no structure)')
            self.assertEqual(len(specs), 18)
            return specs

        self.assertEqual(group, 'register')
        payload = b'register\n' * 66
        digest = hashlib.sha256(payload).hexdigest()
        files = {'/repo/docs/QUELLENVERZEICHNIS.md': payload}
        for identity, original, kind, line, status in INDEX25_REGISTERS:
            locator = 'docs/QUELLENVERZEICHNIS.md#L' + line
            row = dict(source_id=identity, namespace='sources-md', original_id=original,
                       source_type=kind, title='fixture', locator=locator, revision='',
                       sha256='', verification_status=status, record_locator=locator,
                       record_sha256=digest)
            # Explicit delta / old class / new class / rule. No generator enum
            # or computed truthiness is used to choose the seven hash outcomes.
            unauthorized = 'register exception identity unauthorized'
            missing = 'local artifact missing hash'
            changes = [
                ('authorized', {}, missing, 'PASS', 'full parser/structure/originals + direct originals'),
                ('source_id', {'source_id': identity + 'changed'}, missing, unauthorized, 'register_guard/source_id binding'),
                ('namespace', {'namespace': 'sources-mdchanged'}, missing, unauthorized, 'register_guard/namespace binding'),
                ('original_id', {'original_id': original + 'changed'}, missing, unauthorized, 'register_guard/original_id binding'),
                ('source_type', {'source_type': kind + 'changed'}, missing, unauthorized, 'register_guard/source_type binding'),
                ('locator', {'locator': 'docs/QUELLENVERZEICHNIS.md#L1'}, missing, unauthorized, 'register_guard/locator binding'),
                ('record_locator', {'record_locator': 'docs/QUELLENVERZEICHNIS.md#L1'}, missing, unauthorized, 'register_guard/record_locator binding'),
                ('verification_status', {'verification_status': 'DOCUMENTED_READ_SCOPE'}, missing, unauthorized, 'register_guard/status binding'),
                ('reserved-http-bypass', {'source_type': 'serial_log', 'locator': 'https://example.invalid/original'}, 'PASS', unauthorized, 'register_guard/reserved identity regardless of type'),
                ('SRC-999999', {'source_id': 'SRC-999999'}, missing, unauthorized, 'register_guard/unlisted identity'),
                ('SRC-000001', {'source_id': 'SRC-000001'}, missing, unauthorized, 'register_guard/historical identity not authorized'),
                ('SRC-000002', {'source_id': 'SRC-000002'}, missing, unauthorized, 'register_guard/historical identity not authorized'),
                ('SRC-000003', {'source_id': 'SRC-000003'}, missing, unauthorized, 'register_guard/historical identity not authorized'),
                ('SRC-000004', {'source_id': 'SRC-000004'}, missing, unauthorized, 'register_guard/historical identity not authorized'),
                ('SRC-000005', {'source_id': 'SRC-000005'}, missing, unauthorized, 'register_guard/historical identity not authorized'),
                ('hash-zeros', {'sha256': '0' * 64}, 'artifact hash mismatch', 'register source misclassified as original', 'register_guard/exact empty hash'),
                ('hash-bad', {'sha256': 'bad'}, 'artifact hash mismatch', 'register source misclassified as original', 'register_guard/exact empty hash'),
                ('hash-none', {'sha256': None}, missing, 'register source misclassified as original', 'register_guard/exact empty hash; Python entry only'),
                ('hash-false', {'sha256': False}, missing, 'register source misclassified as original', 'register_guard/exact empty hash; Python entry only'),
                ('hash-zero', {'sha256': 0}, missing, 'register source misclassified as original', 'register_guard/exact empty hash; Python entry only'),
                ('hash-space', {'sha256': ' '}, 'artifact hash mismatch', 'register source misclassified as original', 'register_guard/exact empty hash'),
                ('hash-matching', {'sha256': digest}, 'PASS', 'register source misclassified as original', 'register_guard/register bytes never original bytes')]
            for other_type in INDEX25_TYPES:
                if other_type != kind:
                    changes.append(('type-' + other_type, {'source_type': other_type},
                                    missing, unauthorized, 'register_guard/type binding'))
            self.assertEqual(len(changes), 26)
            for suffix, delta, old, new, rule in changes:
                data = {name: [] for name in INDEX25_TABLES}
                changed = dict(row, **delta)
                data['sources.tsv'] = [changed]
                # Historical originals appends the actual source identity to
                # these two errors; all current guard messages are exact.
                old_result = old if old == 'PASS' else old + ': ' + changed['source_id']
                specs[identity + '-' + suffix] = (data, files, old_result, new,
                                                 suffix == 'authorized', rule)
        self.assertEqual(len(specs), 130)
        return specs

    def INDEX25_check(self, condition, message):
        if not condition:
            raise self.failureException('INDEX25 ' + message)

    def INDEX25_inventory(self, actual, expected, stage):
        self.INDEX25_check(len(actual) == len(set(actual)), stage + ' duplicate identity')
        self.INDEX25_check(set(actual) == set(expected), stage + ' missing/extra identity')

    def INDEX25_audit(self, group, driver=None, cases=None, specs=None):
        """Observe completed adapters AND successful real comparison predicates.

        Generated inventory, observed execution inventory and independent specs
        are separate. A driver return count is deliberately not evidence.
        Every original callable is preserved and invoked; unexpected exceptions
        propagate. Only final inventory assertions classify skipped comparisons.
        """
        canonical = self.INDEX25_specs(group)
        specs = canonical if specs is None else specs
        self.INDEX25_inventory(list(specs), canonical, 'expected')
        self.INDEX25_check(specs == canonical, 'expected specification changed')
        generator_name = 'isolated_cases' if group == 'isolated' else 'register_cases'
        argument = self.historical if group == 'isolated' else self.contract
        generated = getattr(r, generator_name)(argument) if cases is None else cases
        labels = [case[0] for case in generated]
        expected_generated = [label for label in specs if label != 'repository-absent']
        self.INDEX25_inventory(labels, expected_generated, 'generated')
        # JSON distinguishes None/False/0, unlike Python equality. No coercion
        # may hide a mutation in the direct Python hash-input cases.
        def signature(data):
            return json.dumps(data, sort_keys=True)

        by_input = {signature(spec[0]): label for label, spec in specs.items()}
        self.INDEX25_check(len(by_input) == len(specs), 'ambiguous independent input')
        for case in generated:
            label, data, files = case[:3]
            wanted, wanted_files, old, new, mode, rule = specs[label]
            self.INDEX25_check(signature(data) == signature(wanted) and files == wanted_files,
                               'generated input ' + label)
            if group == 'isolated':
                self.INDEX25_check(case[3] is False and case[4] == old,
                                   'generated expectation ' + label)
            else:
                old_class = old.split(': ', 1)[0]
                self.INDEX25_check(case[3:] == (old_class, new),
                                   'generated expectation ' + label)

        oracle = r.oracle_module(self.members, self.contract)
        adapters = []
        comparisons = []
        pending = []
        depth = 0
        direct_input = []
        real_require = v.require
        real_old = r.oracle_result
        real_new = r.current_result
        real_rule = r.rule_result
        old_originals = oracle['originals']
        new_originals = v.originals

        def identify(data, files):
            label = by_input.get(signature(data))
            self.INDEX25_check(label is not None, 'executed unknown input')
            self.INDEX25_check(files == specs[label][1], 'executed files ' + label)
            return label

        def record(label, phase, side, answer):
            expected = specs[label][2 if side == 'old' else 3]
            self.INDEX25_check(answer == expected, 'executed result ' + label + '/' + side)
            event = (label, phase, side)
            adapters.append(event)
            pending.append(event)

        def adapter(side, actual, *args):
            nonlocal depth
            data, files = args[1:3] if side == 'old' else args[:2]
            mode = args[3] if side == 'old' else args[2]
            label = identify(data, files)
            self.INDEX25_check(mode is (False if group == 'isolated' else True),
                               'executed mode ' + label)
            depth += 1
            try:
                answer = actual(*args)
            finally:
                depth -= 1
            record(label, 'full' if group == 'register' else 'isolated', side, answer)
            return answer

        def originals(side, actual, data, repo, *args):
            if depth == 0:
                label = identify(data, r.MemoryPath.files)
                expected_root = '/absent' if label == 'repository-absent' else '/repo'
                self.INDEX25_check(str(repo) == expected_root, 'executed root ' + label)
                direct_input.append((label, side))
            return actual(data, repo, *args)

        def rule(call, invalid):
            before = len(direct_input)
            answer = real_rule(call, invalid)
            self.INDEX25_check(len(direct_input) == before + 1, 'direct callable not observed')
            label, side = direct_input[-1]
            self.INDEX25_check(invalid is (oracle['Invalid'] if side == 'old' else v.Invalid),
                               'direct exception identity ' + label)
            record(label, 'direct', side, answer)
            return answer

        def require(condition, message):
            # Do not count unrelated structure/parser/guard predicates. Record
            # only after the real comparison predicate completed successfully.
            real_require(condition, message)
            prefixes = (('isolated rule mismatch: ', 'isolated'),
                        ('register full path mismatch: ', 'full'),
                        ('register comparison mismatch: ', 'direct'))
            for prefix, phase in prefixes:
                if message.startswith(prefix):
                    label = message[len(prefix):].split(': ', 1)[0]
                    if label == 'repository-absent':
                        phase = 'direct'
                    self.INDEX25_check(pending == [(label, phase, 'old'), (label, phase, 'new')],
                                       'comparison adapter binding ' + label)
                    comparisons.append((label, phase))
                    pending.clear()
                    break

        if driver is None:
            driver = r.compare_isolated if group == 'isolated' else r.compare_registers
        with r.patch.object(r, generator_name, return_value=copy.deepcopy(generated)), \
             r.patch.object(r, 'oracle_result', side_effect=lambda *args: adapter('old', real_old, *args)), \
             r.patch.object(r, 'current_result', side_effect=lambda *args: adapter('new', real_new, *args)), \
             r.patch.object(r, 'rule_result', side_effect=rule), \
             r.patch.dict(oracle, originals=lambda *args: originals('old', old_originals, *args)), \
             r.patch.object(v, 'originals', side_effect=lambda *args: originals('new', new_originals, *args)), \
             r.patch.object(v, 'require', side_effect=require):
            driver(oracle, argument)

        expected_comparisons = []
        for label, spec in specs.items():
            if group == 'isolated':
                expected_comparisons.append((label, 'direct' if label == 'repository-absent' else 'isolated'))
            else:
                if spec[4]:
                    expected_comparisons.append((label, 'full'))
                expected_comparisons.append((label, 'direct'))
        expected_adapters = [(label, phase, side) for label, phase in expected_comparisons
                             for side in ('old', 'new')]
        self.INDEX25_inventory(comparisons, expected_comparisons, 'comparison')
        self.INDEX25_inventory(adapters, expected_adapters, 'adapter')
        self.INDEX25_check(not pending, 'uncompared adapter result')
        return generated, comparisons, adapters

    def test_INDEX25_M04_M05_isolated_inventory_and_results(self):
        generated, comparisons, adapters = self.INDEX25_audit('isolated')
        self.assertEqual((len(generated), len(comparisons), len(adapters)), (17, 18, 36))

    def test_INDEX25_M04_M05_register_inventory_and_results(self):
        self.assertEqual(set(v.REGISTER_TYPES), set(INDEX25_TYPES))
        generated, comparisons, adapters = self.INDEX25_audit('register')
        self.assertEqual((len(generated), len(comparisons), len(adapters)), (130, 135, 270))

    def INDEX25_skip_mutant(self, group, label):
        # Compile ONLY the current driver function in memory, with its original
        # globals. Historical bytes and production validator are not rewritten.
        name = 'compare_isolated' if group == 'isolated' else 'compare_registers'
        source = (v.HERE / 'regress_v02.py').read_text(encoding='utf-8')
        start = source.index('def ' + name + '(')
        end = source.index('\n\ndef ', start + 1)
        function = source[start:end]
        anchor = ('        old = oracle_result(oracle, data, files, originals)\n'
                  if group == 'isolated' else "        if label.endswith('-authorized'):\n")
        self.assertEqual(function.count(anchor), 1)
        replacement = f'        if label == {label!r}:\n            continue\n' + anchor
        namespace = r.__dict__.copy()
        # Resolve these at call time so the auditor's temporary wrappers also
        # observe the mutant. A copied original adapter would bypass observation.
        for helper in ('oracle_result', 'current_result', 'rule_result',
                       'isolated_cases', 'register_cases'):
            namespace[helper] = lambda *args, helper=helper: getattr(r, helper)(*args)
        exec(compile(function.replace(anchor, replacement), '<INDEX25-skip>', 'exec'), namespace)
        return namespace[name]

    def test_INDEX25_isolated_skip_sensitivity(self):
        self.INDEX25_audit('isolated')  # Prove all prerequisites before mutation.
        mutant = self.INDEX25_skip_mutant('isolated', 'evidence-source')
        with self.assertRaises(self.failureException) as caught:
            self.INDEX25_audit('isolated', driver=mutant)
        self.assertEqual(str(caught.exception), 'INDEX25 comparison missing/extra identity')

    def test_INDEX25_register_skip_sensitivity(self):
        self.INDEX25_audit('register')
        mutant = self.INDEX25_skip_mutant('register', 'SRC-000014-hash-matching')
        with self.assertRaises(self.failureException) as caught:
            self.INDEX25_audit('register', driver=mutant)
        self.assertEqual(str(caught.exception), 'INDEX25 comparison missing/extra identity')

    def test_INDEX25_missing_duplicate_and_expectation_sensitivity(self):
        for group in ('isolated', 'register'):
            with self.subTest(group=group):
                cases, _, _ = self.INDEX25_audit(group)
                missing = cases[:-1]
                duplicate = cases + [copy.deepcopy(cases[-1])]
                changed = copy.deepcopy(cases)
                last = list(changed[-1])
                last[4 if group == 'isolated' else 3] = 'MANIPULATED_EXPECTATION'
                changed[-1] = tuple(last)
                for mutant, message in (
                        (missing, 'INDEX25 generated missing/extra identity'),
                        (duplicate, 'INDEX25 generated duplicate identity'),
                        (changed, 'INDEX25 generated expectation ' + cases[-1][0])):
                    with self.assertRaises(self.failureException) as caught:
                        self.INDEX25_audit(group, cases=mutant)
                    self.assertEqual(str(caught.exception), message)
                specs = self.INDEX25_specs(group)
                specs.pop(cases[-1][0])
                with self.assertRaises(self.failureException) as caught:
                    self.INDEX25_audit(group, specs=specs)
                self.assertEqual(str(caught.exception), 'INDEX25 expected missing/extra identity')

    def test_INDEX25_executed_duplicate_sensitivity(self):
        for group in ('isolated', 'register'):
            with self.subTest(group=group):
                self.INDEX25_audit(group)
                actual = r.compare_isolated if group == 'isolated' else r.compare_registers
                def repeat(oracle, argument):
                    actual(oracle, argument)
                    return actual(oracle, argument)
                with self.assertRaises(self.failureException) as caught:
                    self.INDEX25_audit(group, driver=repeat)
                self.assertEqual(str(caught.exception), 'INDEX25 comparison duplicate identity')

    def test_INDEX25_register_current_expectation_sensitivity(self):
        cases, _, _ = self.INDEX25_audit('register')
        changed = copy.deepcopy(cases)
        last = list(changed[-1])
        last[4] = 'PASS'
        changed[-1] = tuple(last)
        with self.assertRaises(self.failureException) as caught:
            self.INDEX25_audit('register', cases=changed)
        self.assertEqual(str(caught.exception), 'INDEX25 generated expectation ' + cases[-1][0])

    def register_fixture(self, exception):
        data={name:[] for name in v.SCHEMAS}
        row=copy.deepcopy(exception)
        payload=b'register\n'*66
        row.update(title='fixture',revision='',sha256='',record_sha256=hashlib.sha256(payload).hexdigest())
        data['sources.tsv']=[row]
        r.MemoryPath.files={'/repo/docs/QUELLENVERZEICHNIS.md':payload}
        r.MemoryPath.links={}
        return data,row

    def test_each_register_direct_positive_and_hash_negative(self):
        self.assertEqual([x['source_id'] for x in self.contract['register_exceptions']],
                         ['SRC-000009','SRC-000010','SRC-000012','SRC-000013','SRC-000014'])
        for exception in self.contract['register_exceptions']:
            with self.subTest(source=exception['source_id']):
                data,row=self.register_fixture(exception)
                v.register_guard(data)
                with r.contextlib.redirect_stdout(io.StringIO()): v.originals(data,r.MemoryPath('/repo'))
                for digest in ('0'*64,'bad',None):
                    row['sha256']=digest
                    with self.assertRaisesRegex(v.Invalid,'^register source misclassified as original$'):
                        v.register_guard(data)
                    with self.assertRaisesRegex(v.Invalid,'^register source misclassified as original$'):
                        v.originals(data,r.MemoryPath('/repo'))

    def test_each_register_direct_identity_negative(self):
        for exception in self.contract['register_exceptions']:
            for field in exception:
                with self.subTest(source=exception['source_id'],field=field):
                    data,row=self.register_fixture(exception)
                    row[field]+='changed'
                    for call in (lambda: v.register_guard(data),
                                 lambda: v.originals(data,r.MemoryPath('/repo'))):
                        with self.assertRaisesRegex(v.Invalid,'^register exception identity unauthorized$'): call()
        for source_type in sorted(v.REGISTER_TYPES):
            data,row=self.register_fixture(self.contract['register_exceptions'][0])
            row.update(source_id='SRC-999999',source_type=source_type)
            with self.assertRaisesRegex(v.Invalid,'^register exception identity unauthorized$'):
                v.originals(data,r.MemoryPath('/repo'))

    def test_register_hash_removal_detected(self):
        mutant=self.current_mutant(
            "            require(row['sha256'] == '', 'register source misclassified as original')\n",
            '', 'register_guard')
        for exception in self.contract['register_exceptions']:
            data,row=self.register_fixture(exception)
            row['sha256']='0'*64
            with self.assertRaisesRegex(v.Invalid,'^register source misclassified as original$'):
                v.originals(data,r.MemoryPath('/repo'))
            # Same discriminating input passes after removal of only the hash rule.
            with r.patch.object(v,'register_guard',mutant), r.contextlib.redirect_stdout(io.StringIO()):
                v.originals(data,r.MemoryPath('/repo'))


def forbid_side_effects(event, args):
    if event == 'open':
        mode, flags = args[1:3]
        if (mode and any(c in mode for c in 'wax+')) or flags & (os.O_WRONLY | os.O_RDWR | os.O_CREAT | os.O_TRUNC | os.O_APPEND):
            raise AssertionError('test filesystem write forbidden')
    if event in {'os.remove','os.rename','os.mkdir','os.rmdir','os.symlink','os.link','os.chmod',
                 'os.utime','subprocess.Popen','socket.connect','socket.bind','os.system'}:
        raise AssertionError('test side effect forbidden: ' + event)


if __name__ == '__main__':
    sys.dont_write_bytecode = True
    sys.addaudithook(forbid_side_effects)
    unittest.main(verbosity=2)
