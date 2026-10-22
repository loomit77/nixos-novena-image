#!/usr/bin/env bash
# Run as a separate Bash process; never source this file.
set -Eeuo pipefail
set +x
umask 077

usb=NOT_RUN
vm103=NOT_RUN
result=FAIL
usb_rc=NA
vm103_rc=NA
report() {
    printf 'USB_STATUS=%s\nUSB_EXIT_STATUS=%s\nVM103_STATUS=%s\nVM103_EXIT_STATUS=%s\nOVERALL_STATUS=%s\n' \
        "$usb" "$usb_rc" "$vm103" "$vm103_rc" "$result"
}
mount_status=NOT_CHECKED
unmount_status=NOT_NEEDED
owned_id=
mountpoint=/run/media/loomit/NOVENA_BORG
uuid=b9161f3a-5903-4761-936f-42360d13d84e
# Read kernel mount metadata only; exact mountpoint, never containing filesystem.
mount_record() { findmnt -rn -M "$mountpoint" -o ID,UUID,FSTYPE,TARGET; }
valid_record() {
    [[ $1 =~ ^[0-9]+\ $uuid\ ext4\ $mountpoint$ ]]
}
cleanup() {
    local current
    [[ -n $owned_id ]] || return 0
    current=$(mount_record) || current=
    if valid_record "$current" && [[ ${current%% *} == "$owned_id" ]]; then
        owned_id=
        if umount -- "$mountpoint" >&2; then
            if mount_record >/dev/null; then
                unmount_status=STILL_MOUNTED
                return 1
            fi
            unmount_status=PASS
        else
            unmount_status=FAIL
            return 1
        fi
    else
        owned_id=
        unmount_status=IDENTITY_CHANGED
        return 1
    fi
}
finish() {
    local rc=$?
    trap - EXIT
    if ! cleanup; then
        result=FAIL
        if (( rc == 0 )); then rc=1; fi
    fi
    report
    printf 'USB_MOUNT_STATUS=%s\nUSB_UNMOUNT_STATUS=%s\n' "$mount_status" "$unmount_status"
    exit "$rc"
}
trap finish EXIT
trap 'result=INTERRUPTED; exit 130' INT
trap 'result=INTERRUPTED; exit 143' TERM

mode=all
if (( $# == 1 )); then
    case $1 in
        --usb) mode=usb ;;
        --hdd) mode=hdd ;;
        --all) mode=all ;;
        --check) mode=check ;;
        *) mode=invalid ;;
    esac
elif (( $# != 0 )); then
    mode=invalid
fi
if [[ $mode == invalid ]]; then
    printf 'Usage: bash scripts/novena-backup.bash [--usb|--hdd|--all|--check]\n' >&2
    result=INVALID_ARGUMENT
    exit 2
fi

usb_launcher="$HOME/novena-backups/workflow-dev/daily-close.bash"
vm_launcher="$HOME/.config/novena-borg/execute-vm103-backup.bash"
base="$HOME/.config/novena-borg"
# Local inspection only: no config validation through borgmatic or preflight.
check_files() {
    local file
    for file in "$@"; do
        [[ -f $file && -r $file && ! -L $file ]] || return 1
    done
}
if [[ $mode != hdd ]]; then
if check_files "$usb_launcher" "$HOME/.config/borgmatic/novena.yaml" \
    "$HOME/.config/borgmatic/novena-passphrase" && bash -n "$usb_launcher"; then
    usb=LOCAL_READY
else
    usb=LOCAL_MISSING_OR_INVALID
fi
fi
if [[ $mode != usb ]]; then
if check_files "$vm_launcher" "$base/run-vm103-backup.bash" \
    "$base/image-preflight.bash" "$base/vm103-borgmatic.yaml" \
    "$base/vm103-known-hosts" "$base/vm103-passphrase" "$HOME/.ssh/id_ed25519" \
    && [[ -x $base/run-vm103-backup.bash && -x $base/image-preflight.bash ]] \
    && bash -n "$vm_launcher" && bash -n "$base/run-vm103-backup.bash" \
    && bash -n "$base/image-preflight.bash"; then
    vm103=LOCAL_READY
else
    vm103=LOCAL_MISSING_OR_INVALID
fi
fi
if [[ $mode == check ]]; then
    printf 'CHECK_SCOPE=LOCAL_FILES_AND_SYNTAX_ONLY\nBACKUP_EXECUTED=NO\n'
    result=CHECK_INCOMPLETE
    [[ $usb == LOCAL_READY && $vm103 == LOCAL_READY ]] || exit 1
    result=CHECK_PASS
    exit 0
fi
if [[ ( $mode != hdd && $usb != LOCAL_READY ) || ( $mode != usb && $vm103 != LOCAL_READY ) ]]; then
    result=PRECONDITION_FAILED
    exit 1
fi
if (( EUID == 0 )); then result=ROOT_FORBIDDEN; exit 1; fi
usb=NOT_RUN
vm103=NOT_RUN
command -v flock >/dev/null || { result=LOCK_UNAVAILABLE; exit 1; }
# Dedicated lock, held across BOTH child processes. Never remove this file.
# Existing launchers retain their own guards and lock descriptors unchanged.
lock_dir="$HOME/novena-backups/borg-private"
[[ -d $lock_dir && ! -L $lock_dir ]] || { result=LOCK_UNAVAILABLE; exit 1; }
lock="$lock_dir/novena-backup.lock"
[[ ! -L $lock ]] || { result=LOCK_UNAVAILABLE; exit 1; }
exec 8>"$lock"
if ! flock -n 8; then
    result=LOCKED
    exit 40
fi
printf 'COMMON_LOCK=ACQUIRED\n' >&2
# Preserve proven argument-free start paths, including all mount/repo guards.
# Child output goes to stderr; stdout contains the final machine-readable result.
prepare_usb() {
    local current device entry
    current=$(mount_record) || current=
    if [[ -n $current ]]; then
        valid_record "$current" || { mount_status=FOREIGN_MOUNT; return 1; }
        mount_status=ALREADY_MOUNTED
    else
        # UUID is the only device selector. Never cache /dev/sdX names.
        device=$(lsblk -dnro UUID,FSTYPE -- "/dev/disk/by-uuid/$uuid") || {
            mount_status=DEVICE_UNAVAILABLE; return 1;
        }
        [[ $device == "$uuid ext4" ]] || { mount_status=DEVICE_MISMATCH; return 1; }
        # Only a fixed, administrator-managed fstab user mount is supported.
        entry=$(findmnt --fstab -rn -M "$mountpoint" -o SOURCE,FSTYPE,OPTIONS) || entry=
        [[ $entry == "UUID=$uuid ext4 "* ]] || { mount_status=FSTAB_NOT_APPROVED; return 1; }
        [[ ${entry##* } == rw,noauto,user,nosuid,nodev,noexec ]] || {
            mount_status=FSTAB_NOT_APPROVED; return 1;
        }
        # Recheck immediately before mount; never mount over an existing mount.
        if mount_record >/dev/null; then mount_status=MOUNT_CHANGED; return 1; fi
        if ! mount -- "$mountpoint" >&2; then mount_status=MOUNT_FAILED; return 1; fi
        current=$(mount_record) || current=
        if ! valid_record "$current"; then
            mount_status=POSTCHECK_FAILED
            unmount_status=OWNERSHIP_UNKNOWN
            return 1 # Ownership cannot be established: never unmount an unknown mount.
        fi
        owned_id=${current%% *}
        mount_status=MOUNTED
    fi
    [[ -d $mountpoint/repository && -r $mountpoint/repository && -x $mountpoint/repository && -w $mountpoint/repository ]] || {
        mount_status=REPOSITORY_INACCESSIBLE; return 1;
    }
}
if [[ $mode != hdd ]]; then
if prepare_usb; then
    usb=RUNNING
    if bash "$usb_launcher" >&2; then usb_rc=0; usb=PASS
    else usb_rc=$?; usb=FAIL; fi
else
    usb=MOUNT_FAILED
fi
# Cleanup is attempted once, before VM103, also after a USB launcher failure.
cleanup || :
fi
if [[ $mode != usb ]]; then
vm103=RUNNING
if bash "$vm_launcher" >&2; then vm103_rc=0; vm103=PASS
else vm103_rc=$?; vm103=FAIL; fi
fi
if [[ ( $mode == hdd || $usb == PASS ) && ( $mode == usb || $vm103 == PASS ) && $unmount_status != FAIL && $unmount_status != STILL_MOUNTED && $unmount_status != IDENTITY_CHANGED ]]; then
    result=PASS
else
    result=FAIL
    exit 1
fi
