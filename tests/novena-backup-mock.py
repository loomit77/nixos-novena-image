#!/usr/bin/env python3
"""Isolated launcher mocks only; never use the real HOME or backup tools."""
import fcntl
import os
from pathlib import Path
import subprocess
import shutil
import tempfile

root = Path(__file__).resolve().parents[1]
script = root / 'scripts/novena-backup.bash'
with tempfile.TemporaryDirectory(prefix='.backup-mock-', dir=root) as tmp:
    home = Path(tmp)
    env = dict(os.environ, HOME=tmp)
    # Only the mountpoint literal is relocated; production logic is unchanged.
    target = home/'media/NOVENA_BORG'
    (target/'repository').mkdir(parents=True)
    fixture_script = home/'launcher.bash'
    fixture_script.write_text(script.read_text().replace(
        'mountpoint=/run/media/loomit/NOVENA_BORG', f'mountpoint={target}'))
    script = fixture_script
    uuid = 'b9161f3a-5903-4761-936f-42360d13d84e'
    env['MOCK_TARGET'] = str(target)
    env['MOCK_UUID'] = uuid
    base = home / '.config/novena-borg'
    usb = home / 'novena-backups/workflow-dev/daily-close.bash'
    vm = base / 'execute-vm103-backup.bash'
    required = [usb, vm, base/'run-vm103-backup.bash', base/'image-preflight.bash',
                base/'vm103-borgmatic.yaml', base/'vm103-known-hosts',
                base/'vm103-passphrase', home/'.ssh/id_ed25519',
                home/'.config/borgmatic/novena.yaml',
                home/'.config/borgmatic/novena-passphrase']
    for path in required:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('# fixture\n')
    for path in [base/'run-vm103-backup.bash', base/'image-preflight.bash']:
        path.chmod(0o700)
        path.write_text('exit 99 # Must never run in these tests\n')
    bindir = home/'bin'
    bindir.mkdir()
    for name in ['borg', 'borgmatic', 'ssh', 'sudo', 'udisksctl']:
        path = bindir/name
        path.write_text('#!/bin/bash\nprintf forbidden > "$HOME/forbidden"\nexit 99\n')
        path.chmod(0o700)
    for name, body in {
        'findmnt': r'''if [[ " $* " == *" --fstab "* ]]; then
printf 'UUID=%s ext4 %s\n' "$MOCK_UUID" "${FSTAB_OPTIONS:-rw,noauto,user,nosuid,nodev,noexec}"
else
[[ -f $HOME/mounted ]] || exit 1
cat "$HOME/mounted"
fi''',
        'lsblk': r'''printf '%s ext4\n' "${DEVICE_UUID:-$MOCK_UUID}"''',
        'mount': r'''printf 'mount\n' >> "$HOME/order"
[[ ${MOUNT_FAIL:-0} == 0 ]] || exit 1
printf '42 %s ext4 %s\n' "$MOCK_UUID" "$MOCK_TARGET" > "$HOME/mounted"''',
        'umount': r'''printf 'umount\n' >> "$HOME/order"
[[ ${UNMOUNT_FAIL:-0} == 0 ]] || exit 1
[[ ${UNMOUNT_KEEP:-0} == 0 ]] || exit 0
rm "$HOME/mounted"''',
    }.items():
        path = bindir/name
        path.write_text('#!/bin/bash\nprintf tool\\n >> "$HOME/tools"\n' + body + '\n')
        path.chmod(0o700)
    env['PATH'] = str(bindir) + ':' + env['PATH']
    lockdir = home/'novena-backups/borg-private'
    lockdir.mkdir()
    def launchers(urc=0, vrc=0):
        usb.write_text(f'if flock -n "$HOME/novena-backups/borg-private/novena-backup.lock" true; then exit 98; fi\nprintf "usb\\n" >> "$HOME/order"\nexit {urc}\n')
        vm.write_text(f'if flock -n "$HOME/novena-backups/borg-private/novena-backup.lock" true; then exit 98; fi\nprintf "vm103\\n" >> "$HOME/order"\nexit {vrc}\n')
    def run(args=(), rc=0, fields=()):
        p = subprocess.run(['bash', str(script), *args], env=env,
                           capture_output=True, text=True)
        assert p.returncode == rc, (p.returncode, p.stdout, p.stderr)
        for field in fields:
            assert field+'\n' in p.stdout, p.stdout
        assert not (home/'forbidden').exists()
        return p
    launchers()
    run(['--check'], fields=['OVERALL_STATUS=CHECK_PASS', 'BACKUP_EXECUTED=NO'])
    assert not (home/'order').exists() and not (lockdir/'novena-backup.lock').exists()
    assert not (home/'tools').exists()
    run(['--unknown'], rc=2, fields=['OVERALL_STATUS=INVALID_ARGUMENT'])
    for urc, vrc in [(0, 0), (7, 0), (0, 9), (7, 9)]:
        launchers(urc, vrc)
        (home/'order').unlink(missing_ok=True)
        run(rc=int(bool(urc or vrc)), fields=[
            f'USB_EXIT_STATUS={urc}', f'VM103_EXIT_STATUS={vrc}',
            'OVERALL_STATUS=' + ('FAIL' if urc or vrc else 'PASS')])
        assert (home/'order').read_text() == 'mount\nusb\numount\nvm103\n'
    # Already mounted filesystems are never automatically unmounted.
    for record, expected in [
        (f'41 {uuid} ext4 {target}\n', 'usb\nvm103\n'),
        (f'41 wrong ext4 {target}\n', 'vm103\n'),
        (f'41 {uuid} xfs {target}\n', 'vm103\n'),
    ]:
        launchers()
        (home/'mounted').write_text(record)
        (home/'order').unlink(missing_ok=True)
        run(rc=int(expected == 'vm103\n'))
        assert (home/'order').read_text() == expected
        assert (home/'mounted').read_text() == record
    (home/'mounted').unlink()
    for setting, expected, status in [
        ({'DEVICE_UUID': 'wrong'}, 'vm103\n', 'USB_MOUNT_STATUS=DEVICE_MISMATCH'),
        ({'MOUNT_FAIL': '1'}, 'mount\nvm103\n', 'USB_MOUNT_STATUS=MOUNT_FAILED'),
        ({'UNMOUNT_FAIL': '1'}, 'mount\nusb\numount\nvm103\n', 'USB_UNMOUNT_STATUS=FAIL'),
    ]:
        env.update(setting)
        (home/'order').unlink(missing_ok=True)
        run(rc=1, fields=[status])
        assert (home/'order').read_text() == expected
        for key in setting:
            del env[key]
        (home/'mounted').unlink(missing_ok=True)
    # A replaced mount must not be unmounted, even if UUID and target match.
    launchers()
    usb.write_text(usb.read_text().replace('exit 0',
        'printf "99 %s ext4 %s\\n" "$MOCK_UUID" "$MOCK_TARGET" > "$HOME/mounted"\nexit 0'))
    (home/'order').unlink(missing_ok=True)
    run(rc=1, fields=['USB_UNMOUNT_STATUS=IDENTITY_CHANGED'])
    assert (home/'order').read_text() == 'mount\nusb\nvm103\n'
    (home/'mounted').unlink()
    launchers()
    (target/'repository').rmdir()
    (home/'order').unlink()
    run(rc=1, fields=['USB_MOUNT_STATUS=REPOSITORY_INACCESSIBLE'])
    assert (home/'order').read_text() == 'mount\numount\nvm103\n'
    (target/'repository').mkdir()
    (home/'order').unlink()
    with (lockdir/'novena-backup.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        run(rc=40, fields=['OVERALL_STATUS=LOCKED', 'USB_STATUS=NOT_RUN', 'VM103_STATUS=NOT_RUN'])
    assert not (home/'order').exists()
    # Every mode shares the lock, preserves child codes and selects only its target.
    for mode in ['--usb', '--hdd', '--all']:
        for urc, vrc in [(0, 0), (7, 0), (0, 9), (7, 9)]:
            launchers(urc, vrc)
            (home/'order').unlink(missing_ok=True)
            failed = (mode != '--hdd' and urc != 0) or (mode != '--usb' and vrc != 0)
            run([mode], rc=int(failed), fields=[
                'USB_EXIT_STATUS=' + (str(urc) if mode != '--hdd' else 'NA'),
                'VM103_EXIT_STATUS=' + (str(vrc) if mode != '--usb' else 'NA')])
            expected = ('mount\nusb\numount\n' if mode != '--hdd' else '')
            expected += 'vm103\n' if mode != '--usb' else ''
            assert (home/'order').read_text() == expected
        (home/'order').unlink()
        with (lockdir/'novena-backup.lock').open('w') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            run([mode], rc=40, fields=['OVERALL_STATUS=LOCKED'])
        assert not (home/'order').exists()
    launchers()
    # HDD does not inspect USB files or call any mount/device tool.
    usb.unlink()
    (home/'tools').unlink(missing_ok=True)
    run(['--hdd'], fields=['USB_STATUS=NOT_RUN', 'USB_MOUNT_STATUS=NOT_CHECKED'])
    assert not (home/'tools').exists()
    (home/'order').unlink()
    launchers()
    vm.unlink()
    run(['--usb'], fields=['VM103_STATUS=NOT_RUN'])
    assert (home/'order').read_text() == 'mount\nusb\numount\n'
    launchers()
    # Unsafe fstab permissions must fail closed, before any mount or USB child.
    for mode in ['--usb', '--all']:
        for options in ['rw,noauto,users', 'rw,noauto,user,exec', 'defaults']:
            env['FSTAB_OPTIONS'] = options
            (home/'order').unlink(missing_ok=True)
            run([mode], rc=1, fields=['USB_MOUNT_STATUS=FSTAB_NOT_APPROVED'])
            assert ((home/'order').read_text() if (home/'order').exists() else '') == (
                'vm103\n' if mode == '--all' else '')
        del env['FSTAB_OPTIONS']
    # USB-only ownership and cleanup errors never start VM103.
    for setting, status in [({'MOUNT_FAIL': '1'}, 'USB_MOUNT_STATUS=MOUNT_FAILED'),
                            ({'UNMOUNT_FAIL': '1'}, 'USB_UNMOUNT_STATUS=FAIL')]:
        env.update(setting)
        (home/'order').unlink(missing_ok=True)
        run(['--usb'], rc=1, fields=[status, 'VM103_STATUS=NOT_RUN'])
        assert 'vm103' not in (home/'order').read_text()
        for key in setting:
            del env[key]
        (home/'mounted').unlink(missing_ok=True)
    env['UNMOUNT_KEEP'] = '1'
    (home/'order').unlink(missing_ok=True)
    run(['--usb'], rc=1, fields=['USB_UNMOUNT_STATUS=STILL_MOUNTED'])
    assert (home/'order').read_text() == 'mount\nusb\numount\n'
    del env['UNMOUNT_KEEP']
    (home/'mounted').unlink()
    # TERM retains its exit code and cleans up, without proceeding to VM103.
    usb.write_text('kill -TERM "$PPID"\n')
    (home/'order').unlink()
    run(['--all'], rc=143, fields=['OVERALL_STATUS=INTERRUPTED',
                                 'USB_UNMOUNT_STATUS=PASS', 'VM103_STATUS=NOT_RUN'])
    assert (home/'order').read_text() == 'mount\numount\n'
    launchers()
    for args in [['--usb', '--hdd'], ['--all', '--check'], ['--usb', '--usb']]:
        run(args, rc=2, fields=['OVERALL_STATUS=INVALID_ARGUMENT'])
    (home/'order').unlink(missing_ok=True)
    vm.unlink()
    run(['--check'], rc=1, fields=['VM103_STATUS=LOCAL_MISSING_OR_INVALID'])
    run(rc=1, fields=['OVERALL_STATUS=PRECONDITION_FAILED'])
    assert not (home/'order').exists()
    # Fish wrappers execute only a fake bash and preserve its exit status.
    if shutil.which('fish'):
        fake_bash = bindir/'bash'
        fake_bash.write_text('#!/bin/sh\nprintf "%s\\n" "$@" > "$HOME/fish-args"\nexit 37\n')
        fake_bash.chmod(0o700)
        for mode in ['usb', 'hdd', 'all']:
            function = root/f'config/fish/functions/backup-{mode}.fish'
            p = subprocess.run(['fish', '--no-config', '-c',
                                'source "$argv[1]"; ' + f'backup-{mode}', str(function)],
                               env=env, capture_output=True, text=True)
            assert p.returncode == 37, (p.returncode, p.stderr)
            assert (home/'fish-args').read_text().splitlines() == [
                str(home/'nixos-novena-image/scripts/novena-backup.bash'), '--'+mode]
        print('FISH_WRAPPER_MOCK_STATUS=PASS')
print('MOCK_TEST_STATUS=PASS (three modes, selected prerequisites, fstab policy, mount identity, ownership, failures, order, lock, check, launcher exits)')
