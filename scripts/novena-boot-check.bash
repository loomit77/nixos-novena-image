#!/usr/bin/env bash
set -Eeuo pipefail

# Read-only Novena inspection. Execute through the established
# run-and-upload.bash wrapper; never log credentials or private keys.
#
# Usage: bash scripts/novena-boot-check.bash root@192.168.11.13
# Optional:
#   NOVENA_SSH_IDENTITY=/path/to/private/key
#   NOVENA_KNOWN_HOSTS=/path/to/known_hosts

TARGET="${1:?Usage: $0 user@host}"

case "$TARGET" in
  *[!a-zA-Z0-9_.@:-]*|'') echo "Invalid SSH target" >&2; exit 2 ;;
esac

SSH_OPTS=(
  -T
  -o StrictHostKeyChecking=yes
  -o ConnectTimeout=10
  -o ServerAliveInterval=10
  -o ServerAliveCountMax=2
)

if [[ -n "${NOVENA_KNOWN_HOSTS:-}" ]]; then
  SSH_OPTS+=(-o "UserKnownHostsFile=$NOVENA_KNOWN_HOSTS")
fi

if [[ -n "${NOVENA_SSH_IDENTITY:-}" ]]; then
  SSH_OPTS+=(
    -o BatchMode=yes
    -o IdentitiesOnly=yes
    -i "$NOVENA_SSH_IDENTITY"
  )
else
  SSH_OPTS+=(-o BatchMode=no)
fi

printf '%s\n' "=== NOVENA STANDARD BOOT CHECK ==="
printf 'TARGET=%s\n' "$TARGET"
printf 'CHECK_UTC=%s\n' "$(date -u +%FT%TZ)"
printf '%s\n' "MODE=READ_ONLY"
printf '%s\n' "SERIAL_AND_VISUAL_BOOT_EVIDENCE=SEPARATE"

CHECK_TMP="$(mktemp)"
trap 'rm -f -- "$CHECK_TMP"' EXIT

set +e
ssh "${SSH_OPTS[@]}" "$TARGET" 'sh -s' <<'REMOTE' | tee "$CHECK_TMP"
set -eu

section() {
  printf '\n=== %s ===\n' "$1"
}

section IDENTITY
hostname
uname -a
cat /etc/os-release
cat /proc/cmdline
printf 'BOOT_ID='
cat /proc/sys/kernel/random/boot_id
printf 'UPTIME='
cat /proc/uptime

section NETWORK
ip -br address
ip route

section STORAGE
lsblk -b -o NAME,SIZE,FSTYPE,LABEL,MOUNTPOINTS
findmnt -no SOURCE,FSTYPE,SIZE,OPTIONS /
df -B1 /

section MEMORY
cat /proc/meminfo | head -n 8

section DRM_CONNECTORS
for p in /sys/class/drm/card*-*/status; do
  test -r "$p" || continue
  printf '%s=' "$p"
  cat "$p"
done

section DRM_MODES
for p in /sys/class/drm/card*-*/modes; do
  test -r "$p" || continue
  printf 'PATH=%s\n' "$p"
  cat "$p"
done

section BACKLIGHT
for p in /sys/class/backlight/*; do
  test -d "$p" || continue
  printf 'BACKLIGHT=%s\n' "$p"
  for name in brightness actual_brightness max_brightness; do
    if test -r "$p/$name"; then
      printf '%s=' "$name"
      cat "$p/$name"
    fi
  done
done

section IT6251
ls -ld \
  /sys/bus/i2c/devices/2-005c \
  /sys/bus/i2c/devices/2-005c/driver 2>&1 || true

section FAILED_SYSTEMD_UNITS
systemctl --no-pager --plain --failed 2>&1 || true

section DISPLAY_KERNEL_EVIDENCE
journalctl -k -b --no-pager -o short-monotonic 2>&1 |
  grep -Ei \
    'it6251|display link stable|1920x1080|drm|panel|backlight|error -6' |
  tail -n 110 || true

section GROW_EVIDENCE
journalctl -b --no-pager -o short-monotonic 2>&1 |
  grep -Ei 'grow|resize|repart|mmcblk' |
  tail -n 60 || true

section RESULT
printf '%s\n' "NOVENA_REMOTE_COLLECTION=COMPLETE"
REMOTE
SSH_STATUS=${PIPESTATUS[0]}
set -e

if (( SSH_STATUS != 0 )); then
  printf 'SSH_EXIT_STATUS=%s\n' "$SSH_STATUS"
  printf '%s\n' "NOVENA_BOOT_CHECK=FAIL"
  exit 1
fi

python3 -B - "$CHECK_TMP" "$TARGET" <<'PYJSON'
import datetime
import json
import re
import sys

path, target = sys.argv[1:]
with open(path, encoding="utf-8", errors="replace") as handle:
    raw = handle.read()

def extract(name):
    match = re.search(
        "^" + re.escape(name) + "=(.*)$",
        raw,
        re.MULTILINE,
    )
    return match.group(1).strip() if match else None

boot_id = extract("BOOT_ID")
complete = extract("NOVENA_REMOTE_COLLECTION") == "COMPLETE"

checks = {
    "ssh": "PASS",
    "collection": "PASS" if complete else "FAIL",
    "boot_id": "PASS" if boot_id else "UNKNOWN",
    "serial_capture": "NOT_APPLICABLE",
    "visual_boot_observation": "NOT_APPLICABLE",
}

overall = "PASS" if complete and boot_id else "FAIL"

record = {
    "schema_version": 1,
    "timestamp_utc": datetime.datetime.now(
        datetime.timezone.utc
    ).isoformat(),
    "target": target,
    "boot_id": boot_id,
    "image_identity": None,
    "checks": checks,
    "overall": overall,
}

print("BOOT_CHECK_JSON=" + json.dumps(record, sort_keys=True))

if overall != "PASS":
    raise SystemExit(1)
PYJSON

printf '%s\n' "NOVENA_BOOT_CHECK=PASS"
