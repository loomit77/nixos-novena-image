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


if __name__ == '__main__':
    unittest.main(verbosity=2)
