#!/usr/bin/env python3
"""Focused local-only publication tests. No production endpoints."""
import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    'gate', Path(__file__).with_name('novena-bridge-publish-gate.py'))
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class PublicationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='novena-publication-154-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.remote = self.root / 'archive.git'
        self.seed = self.root / 'seed'
        gate.git(self.root, 'init', '--bare', '--initial-branch=main', str(self.remote))
        (self.remote / 'NOVENA_SYNTHETIC_TEST_ONLY').write_text('synthetic only\n')
        gate.git(self.root, 'init', '--initial-branch=main', str(self.seed))
        gate.git(self.seed, 'config', 'user.name', 'Synthetic Test')
        gate.git(self.seed, 'config', 'user.email', 'test@localhost.invalid')
        gate.git(self.seed, 'commit', '--allow-empty', '-m', 'Synthetic ancestor')
        self.ancestor = gate.head(self.seed)
        (self.seed / 'README').write_text('baseline\n')
        gate.git(self.seed, 'add', 'README')
        gate.git(self.seed, 'commit', '-m', 'Synthetic baseline')
        gate.git(self.seed, 'push', str(self.remote), 'main')
        self.baseline = gate.head(self.remote)
        self.document = json.loads((gate.ROOT / 'docs/bridge-lite/EXCHANGE-CANDIDATE-149.json').read_bytes())

    def publish(self, document=None, expected=None):
        return gate.create_initial_index(self.remote, expected or self.baseline,
                                         document or self.document, self.root)

    def test_candidate_readback_and_external_provenance(self):
        original = copy.deepcopy(self.document)
        result = self.publish()
        gate.verify_readback(self.remote, result['head'], gate.payload(original))
        gate.verify_single_change(self.remote, self.baseline, result['head'])
        self.assertEqual(result['baseline_head'], self.baseline)
        self.assertEqual(self.document, original)
        self.assertIsNone(self.document['publication']['commit'])
        self.assertEqual(self.document['latest_block']['block_id'],
                         'NOVENA-BRIDGE-PUBLISH-GATE-GIT-146R')
        with self.assertRaisesRegex(gate.GateError, 'TARGET_ALREADY_EXISTS'):
            self.publish(expected=result['head'])
        with self.assertRaisesRegex(gate.GateError, 'STALE_REMOTE_HEAD'):
            self.publish()

    def test_policy_rejections_leave_head_unchanged(self):
        for field, value in [('state', 'CONFIRMED'), ('commit', 'a' * 40)]:
            candidate = copy.deepcopy(self.document)
            candidate['publication'][field] = value
            with self.assertRaises(gate.planner.PolicyError):
                self.publish(candidate)
        candidate = copy.deepcopy(self.document)
        candidate['extra'] = True
        with self.assertRaises(gate.jsonschema.ValidationError):
            self.publish(candidate)
        with self.assertRaisesRegex(gate.GateError, 'INVALID_EXPECTED_HEAD'):
            self.publish(expected='main')
        self.assertEqual(gate.head(self.remote), self.baseline)

    def test_concurrent_advance_and_rewind_rejected(self):
        # A rewind can be fast-forwardable from the publisher's new commit;
        # exact old-OID CAS must reject it as well as a divergent advance.
        gate.git(self.seed, 'commit', '--allow-empty', '-m', 'Concurrent change')
        concurrent = gate.head(self.seed)
        for replacement in (concurrent, self.ancestor):
            real_git = gate.git
            def race(directory, *args, **kwargs):
                if Path(directory) == self.remote and args[0] == 'update-ref':
                    real_git(self.remote, 'fetch', '--no-write-fetch-head', str(self.seed), replacement)
                    real_git(self.remote, 'update-ref', 'refs/heads/main', replacement, self.baseline)
                return real_git(directory, *args, **kwargs)
            with patch.object(gate, 'git', race):
                with self.assertRaisesRegex(gate.GateError, 'CONCURRENT_REMOTE_CHANGE'):
                    self.publish()
            if replacement:
                self.assertEqual(gate.head(self.remote), replacement)
            real_git(self.remote, 'update-ref', 'refs/heads/main', self.baseline)

    def test_extra_commit_file_rejected(self):
        real_git = gate.git
        def extra(directory, *args, **kwargs):
            if args[0] == 'commit':
                (Path(directory) / 'extra').write_text('unexpected\n')
                real_git(directory, 'add', 'extra')
            return real_git(directory, *args, **kwargs)
        with patch.object(gate, 'git', extra):
            with self.assertRaisesRegex(gate.GateError, 'UNEXPECTED_COMMIT_CHANGES'):
                self.publish()
        self.assertEqual(gate.head(self.remote), self.baseline)

    def test_readback_mismatches(self):
        result = self.publish()
        with self.assertRaisesRegex(gate.GateError, 'REMOTE_CONTENT_MISMATCH'):
            gate.verify_readback(self.remote, result['head'], b'wrong bytes\r\n')
        with self.assertRaisesRegex(gate.GateError, 'REMOTE_HEAD_MISMATCH'):
            gate.verify_readback(self.remote, self.baseline, gate.payload(self.document))

    def test_production_entry_unconditionally_disabled(self):
        with patch.object(gate, 'PRODUCTION_ENABLED', True):
            with self.assertRaisesRegex(gate.GateError, 'PRODUCTION_PUBLICATION_DISABLED'):
                gate.production_publish(self.remote, self.baseline, self.document)


class GraphQLTests(unittest.TestCase):
    """All mutations are fake in-memory calls; no Git objects are created."""
    def setUp(self):
        self.before, self.after = 'a' * 40, 'b' * 40
        self.repo, self.ref = 'SYNTHETIC_REPOSITORY_ID', 'refs/heads/main'
        self.document = json.loads((gate.ROOT / 'docs/bridge-lite/EXCHANGE-CANDIDATE-149.json').read_bytes())
        self.original = copy.deepcopy(self.document)
        self.head = self.before
        self.calls = []
        self.response = {'data': {'updateRefs': {'clientMutationId': None}}}
        self.record = {'oid': self.after, 'parents': [self.before], 'changes': [
            {'path': gate.TARGET, 'status': 'added', 'mode': '100644',
             'type': 'blob', 'bytes': gate.payload(self.document)}]}
        self.race = None
        self.result_head = self.after
        self.reads = 0
        self.corrupt_readback = False
        self.late_head = None

    def commit(self, repo, oid):
        self.assertEqual((repo, oid), (self.repo, self.after))
        self.reads += 1
        record = copy.deepcopy(self.record)
        if self.corrupt_readback and self.reads > 1:
            record['changes'][0]['bytes'] += b' '
        return record

    def client(self, request):
        self.calls.append(copy.deepcopy(request))
        update = request['variables']['input']['refUpdates'][0]
        if self.race:
            self.head = self.race
        if self.head != update['beforeOid']:
            return {'data': {'updateRefs': None}, 'errors': [{'message': 'conflict'}]}
        self.head = self.result_head
        return copy.deepcopy(self.response)

    def publish(self):
        test = self
        class Reader:
            def head(self, repo, ref):
                test.assertEqual((repo, ref), (test.repo, test.ref))
                if test.late_head and test.reads > 1:
                    return test.late_head
                return test.head
            def commit(self, repo, oid):
                return test.commit(repo, oid)
        return gate.synthetic_graphql_publication(
            self.client, Reader(), self.repo, self.ref,
            self.before, self.after, self.document)

    def test_exact_request_and_readback(self):
        result = self.publish()
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.calls[0]['variables'], {'input': {
            'repositoryId': self.repo, 'refUpdates': [{
                'name': self.ref, 'beforeOid': self.before,
                'afterOid': self.after, 'force': False}]}})
        self.assertIs(self.calls[0]['variables']['input']['refUpdates'][0]['force'], False)
        self.assertEqual(self.reads, 2)
        self.assertEqual(result['head'], self.after)
        self.assertEqual(result['sha256'], gate.planner.sha256(gate.payload(self.document)))
        self.assertEqual(self.document, self.original)
        self.assertIsNone(self.document['publication']['commit'])
        self.assertIsNone(self.document['last_verified'])
        self.assertEqual(self.document['latest_block']['block_id'],
                         'NOVENA-BRIDGE-PUBLISH-GATE-GIT-146R')

    def test_stale_head_no_mutation(self):
        self.head = 'c' * 40
        with self.assertRaisesRegex(gate.GateError, 'STALE_REMOTE_HEAD'):
            self.publish()
        self.assertEqual(self.calls, [])

    def test_concurrent_advance_and_rewind(self):
        for replacement in ('c' * 40, 'd' * 40):
            with self.subTest(replacement=replacement):
                self.setUp()
                self.race = replacement  # divergent advance / ancestor rewind
                with self.assertRaises(gate.GateError):
                    self.publish()
                self.assertEqual(self.head, replacement)
                self.assertEqual(len(self.calls), 1)

    def test_errors_missing_malformed_and_unexpected_fields(self):
        for response in (None, [], {}, {'errors': []},
                         {'data': None}, {'data': {}},
                         {'data': {'updateRefs': None}},
                         {'data': {'updateRefs': {}}},
                         {'data': {'updateRefs': {'clientMutationId': 'wrong'}}},
                         {'data': {'updateRefs': {'clientMutationId': None, 'extra': 1}}},
                         {'data': {'updateRefs': {'clientMutationId': None}, 'extra': 1}},
                         {'data': {'updateRefs': {'clientMutationId': None}}, 'extra': 1},
                         {'data': {'updateRefs': {'clientMutationId': None}},
                          'errors': [{'message': 'partial error'}]}):
            with self.subTest(response=response):
                self.setUp()
                self.response = response
                with self.assertRaises(gate.GateError):
                    self.publish()
                self.assertEqual(len(self.calls), 1)

    def test_wrong_resulting_head_and_late_change(self):
        for late in (False, True):
            self.setUp()
            if late:
                self.late_head = 'c' * 40
            else:
                self.result_head = 'c' * 40
            with self.assertRaisesRegex(gate.GateError, 'RESULTING_HEAD_MISMATCH'):
                self.publish()
            self.assertEqual(len(self.calls), 1)

    def test_commit_validation_before_mutation(self):
        changes = [
            lambda r: r.update(parents=['c' * 40]),
            lambda r: r.update(parents=[self.before, 'c' * 40]),
            lambda r: r.update(oid='c' * 40),
            lambda r: r.update(extra=True),
            lambda r: r['changes'].append(copy.deepcopy(r['changes'][0])),
            lambda r: r.update(changes=[]),
        ]
        for field, value in [('path', 'extra'), ('status', 'modified'),
                             ('mode', '120000'), ('mode', '100755'),
                             ('type', 'tree'), ('bytes', b'wrong bytes'),
                             ('extra', True)]:
            changes.append(lambda r, f=field, v=value: r['changes'][0].update({f: v}))
        for corrupt in changes:
            self.setUp()
            corrupt(self.record)
            with self.assertRaises(gate.GateError):
                self.publish()
            self.assertEqual(self.calls, [])

    def test_readback_bad_bytes_no_retry(self):
        self.corrupt_readback = True
        with self.assertRaisesRegex(gate.GateError, 'UNEXPECTED_TARGET_OR_BYTES'):
            self.publish()
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.head, self.after)

    def test_transport_exception_no_retry(self):
        calls = []
        def unavailable(request):
            calls.append(request)
            raise TimeoutError('synthetic ambiguous outcome')
        self.client = unavailable
        with self.assertRaises(TimeoutError):
            self.publish()
        self.assertEqual(len(calls), 1)

    def test_invalid_identifiers(self):
        for repo, ref, before, after in [('', self.ref, self.before, self.after),
                (self.repo, 'main', self.before, self.after),
                (self.repo, self.ref, 'main', self.after),
                (self.repo, self.ref, self.before, self.before)]:
            with self.assertRaises(gate.GateError):
                gate.update_refs_request(repo, ref, before, after)

    def test_policy_rejection_before_mutation(self):
        for field, value in [('state', 'CONFIRMED'), ('commit', self.after)]:
            self.setUp()
            self.document['publication'][field] = value
            with self.assertRaises(gate.planner.PolicyError):
                self.publish()
            self.assertEqual(self.calls, [])

    def test_production_entry_unconditionally_disabled(self):
        self.assertIs(gate.PRODUCTION_ENABLED, False)
        with patch.object(gate, 'PRODUCTION_ENABLED', True):
            with self.assertRaisesRegex(gate.GateError, '^PRODUCTION_PUBLICATION_DISABLED$'):
                gate.production_publish(client=self.client)
        self.assertEqual(self.calls, [])


if __name__ == '__main__':
    unittest.main(verbosity=2)
