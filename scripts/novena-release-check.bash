#!/usr/bin/env bash
set -Eeuo pipefail

# Source-level preflight; not a substitute for inspecting
# the fully evaluated NixOS configuration and final image.
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

printf '%s\n' "=== NOVENA RELEASE SECURITY PREFLIGHT ==="

FAIL=0

if grep -Eq 'initialPassword[[:space:]]*=[[:space:]]*"nixos"' \
    configuration.nix; then
  printf '%s\n' "BLOCKER=KNOWN_ROOT_INITIAL_PASSWORD"
  FAIL=1
fi

if grep -Eq 'PasswordAuthentication[[:space:]]*=[[:space:]]*true' \
    configuration.nix; then
  printf '%s\n' "BLOCKER=PASSWORD_AUTHENTICATION_ENABLED"
  FAIL=1
fi

if grep -Eq 'PermitRootLogin[[:space:]]*=[[:space:]]*"yes"' \
    configuration.nix; then
  printf '%s\n' "BLOCKER=ROOT_SSH_LOGIN_PERMITTED"
  FAIL=1
fi

if grep -Eq 'novena-development-access.nix' \
    configuration.nix flake.nix; then
  printf '%s\n' "BLOCKER=DEVELOPMENT_ACCESS_MODULE_REFERENCED"
  FAIL=1
fi

if test "$FAIL" -ne 0; then
  printf '%s\n' "RELEASE_PREFLIGHT=BLOCKED"
  exit 1
fi

printf '%s\n' "SOURCE_PREFLIGHT=PASS"
printf '%s\n' "BLOCKER=EVALUATED_CONFIGURATION_NOT_AUDITED"
printf '%s\n' "BLOCKER=FINAL_IMAGE_NOT_AUDITED"
printf '%s\n' "RELEASE_APPROVAL=BLOCKED"
exit 1
