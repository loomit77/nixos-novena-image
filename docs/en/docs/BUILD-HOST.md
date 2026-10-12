This is a translation. The German original is authoritative.

# Build Host

Historical inventory: 2026-09-14

The host details below remain associated with the verification state at that time.
The separate addendum dated 2026-10-11 does not constitute a complete host reverification.

## Purpose

This document describes the verified development and
build machine for the universal NixOS SD image project for the
Kosagi Novena.

The configuration documented here serves as a reference for
reproducible x86_64-to-ARMv7 cross-builds and for serial
access to the Novena hardware.

Project-specific kernel, device-tree, display and I2C test states are
not documented here, but in particular in:

- `docs/PROJECT-STATE.md`
- `docs/GOLDEN-BUILD.md`
- `docs/TEST-LOG.md`

## Computer

Hostname:

```text
foobox
```

Motherboard:

```text
ASRock X300-ITX
```

CPU:

```text
AMD Ryzen 7 5700G with Radeon Graphics
```

CPU configuration:

```text
8 Kerne
16 Threads
AMD-V vorhanden
```

Firmware:

```text
BIOS P1.70
Firmware-Datum: 2025-11-21
```

## Operating System

Distribution:

```text
CachyOS
```

Architecture:

```text
x86_64
```

Kernel in use as of this document:

```text
7.2.4-3-cachyos
```

Shell:

```text
fish 4.9.3
```

Git:

```text
2.55.0
```

## RAM

Physically installed:

```text
64 GiB DDR4
```

Module population:

```text
Channel A DIMM1: 32 GiB Kingston KF3200C16D4/32GX
Channel B DIMM1: 32 GiB Kingston KF3200C16D4/32GX
```

Both memory channels therefore have a symmetrical module population.

Currently configured memory speed:

```text
2400 MT/s
```

The modules nominally belong to the DDR4-3200 class. Changing the
memory clock rate or enabling a faster memory profile was
deliberately not carried out as part of the Novena project.

Linux reports:

```text
MemTotal: 61582688 kB
```

This corresponds to approximately:

```text
58.7 GiB
```

of usable main memory.

The difference from the physically installed 64 GiB is plausibly explained.

The integrated Radeon graphics reserves:

```text
4096 MiB VRAM
```

In addition, the kernel reports approximately:

```text
1373404 KiB reserved
```

for other firmware, kernel and device regions.

There is therefore no indication of undetected or incorrectly
addressed RAM.

## ZRAM

Active swap device:

```text
/dev/zram0
```

Compression:

```text
zstd
```

Size:

```text
ca. 58.7 GiB
```

Priority:

```text
100
```

This provides an additional large compressed swap area in RAM.

## Stability Assessment

As of this document, a separate synthetic RAM or CPU stress test using tools such as
`stress-ng` or `memtester` had not been
performed.

Neither program was installed on foobox.

An additional installation solely for this test was deliberately
avoided, since the machine had already repeatedly performed real and sometimes
extensive Nix, kernel and ARMv7 cross-builds without observed
memory, compiler or stability errors.

These real builds are considered a sufficient practical functional test
for the current project state.

If non-reproducible compiler errors, crashes,
memory errors or other stability problems occur later, a
targeted synthetic CPU and RAM test should be performed then.

## Storage

System drive:

```text
Samsung SSD 970 EVO Plus 1TB
```

Device:

```text
/dev/nvme0n1
```

Partitioning:

```text
nvme0n1p1   4 GiB       vfat
nvme0n1p2   ca. 927 GiB LUKS
```

The encrypted main system uses Btrfs.

At the time of verification, approximately:

```text
302 GiB
```

of unallocated Btrfs storage space was available.

High utilization of individual already allocated data or metadata chunks
should therefore not be interpreted as an acute shortage of space.

A Btrfs balance run is not required solely on account of this chunk utilization.

## NVMe Health

The state was checked with `smartctl`.

Drive:

```text
Samsung SSD 970 EVO Plus 1TB
```

Firmware:

```text
2B2QEXM7
```

Overall SMART status:

```text
PASSED
```

Key values:

```text
Critical Warning:                 0x00
Temperature:                      46 C
Temperature Sensor 2:             49 C
Available Spare:                  100 %
Percentage Used:                  0 %
Data Units Read:                  ca. 6.39 TB
Data Units Written:               ca. 8.76 TB
Power Cycles:                     1284
Power On Hours:                   484
Unsafe Shutdowns:                 71
Media and Data Integrity Errors:  0
Error Information Log Entries:    1219
```

The existing error log entries were reported as:

```text
Invalid Field in Command
```

.

These are therefore not documented NAND, media or
data integrity errors.

The NVMe state is considered healthy for the current project state.

## Network

Ethernet controller:

```text
Realtek RTL8111/8168
```

Interface:

```text
enp2s0
```

Address used during verification:

```text
192.168.1.10/24
```

The specific IPv4 address is not part of the reproducible
build configuration and may change independently of the project.

## Serial Novena Console

An FTDI adapter is available for serial access to the Novena.

USB device:

```text
FTDI FT232R
```

USB ID:

```text
0403:6001
```

Kernel module:

```text
ftdi_sio
```

The current TTY device is:

```text
/dev/ttyUSB0
```

Where possible, project commands should use the persistent path
rather than the volatile `/dev/ttyUSB0` name:

```text
/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_BH00304A-if00-port0
```

This currently points to:

```text
/dev/ttyUSB0
```

The persistent `by-id` path is the preferred reference for future
serial test and logging commands.

## Nix

Installed version:

```text
nix 2.35.2
```

The system-wide Nix configuration is located at:

```text
/etc/nix/nix.conf
```

The complete verified contents are:

```text
build-users-group = nixbld
experimental-features = fetch-tree flakes nix-command
extra-platforms = armv7l-linux
extra-system-features = gccarch-armv7-a
```

In particular, the following Nix features are therefore permanently enabled:

```text
fetch-tree
flakes
nix-command
```

Earlier project commands regularly had to be executed, for example, with:

```text
--extra-experimental-features 'nix-command flakes'
```

.

This addition is no longer required with the current system-wide configuration.

## Nix Platforms

The native platform was verified as:

```text
x86_64-linux
```

The cross-platform used for the Novena was verified as:

```text
armv7l-linux
```

The ARMv7 hard-float cross environment:

```text
pkgsCross.armv7l-hf-multiplatform
```

is functional on foobox.

An ARMv7 cross-GCC was successfully built by Nix.

Verified GCC version:

```text
15.3.0
```

This confirms the basic x86_64-to-ARMv7 cross-build path.

## Nix Build Settings

As of this document, the effective settings are:

```text
cores = 0
max-jobs = 1
```

In addition, Nix reports the following system features, among others:

```text
benchmark
big-parallel
gccarch-armv7-a
kvm
nixos-test
uid-range
```

`cores = 0` allows a derivation to automatically use the
available CPU cores.

`max-jobs = 1` limits the number of concurrently running
derivations.

These values were deliberately not changed.

During earlier tests, different combinations of `--max-jobs` and `--cores`
were tried on the command line. So far, this has not provided a
sufficiently justified optimal global setting.

In particular, for a single large kernel build,
`max-jobs = 1` together with `cores = 0` can be appropriate, since the single
derivation can use the available CPU resources without simultaneously
competing with other large derivations for RAM and CPU time.

These parameters should only be changed after reproducible
build time measurements.

## Flake Verification

The repository:

```text
~/nixos-novena-image
```

was successfully evaluated with permanently enabled
flake support.

The verified Git revision was:

```text
35357fd405259a24f18c86a6fd0dc673c98a12f5
```

`nix flake show` recognized:

```text
nixosConfigurations
└── novena
```

as a valid NixOS configuration.

## Verified Basic Commands

Native host platform:

```fish
nix eval --impure --raw --expr 'builtins.currentSystem'
```

Expected result:

```text
x86_64-linux
```

ARMv7 cross-platform:

```fish
nix eval --impure --raw --expr '
let
  pkgs = import <nixpkgs> {};
in
  pkgs.pkgsCross.armv7l-hf-multiplatform.stdenv.hostPlatform.system
'
```

Expected result:

```text
armv7l-linux
```

Flake structure:

```fish
cd ~/nixos-novena-image
nix flake show
```

With the current Nix configuration, these commands do not require an
additional parameter for `nix-command` or `flakes`.

`--impure` is independent of this and remains necessary for evaluations
that explicitly use impure evaluation.

## Repository and Backup State

Temporary working and backup files from the previous
diagnostic steps were removed from the unversioned repository working tree
after being backed up and verified separately.

Backup directory:

```text
~/novena-backups/foobox-repo-backups-2026-09-13
```

The files backed up there were:

1. copied,
2. recorded with SHA-256,
3. successfully verified with `sha256sum -c`,
4. compared byte by byte with the original files.

All checked copies were identical.

The file already tracked by Git:

```text
kernel/it6251.c.before-drm-lifecycle
```

was not removed from the version history and was correctly restored
from Git after the cleanup.

The subsequently checked repository state was:

```text
## main...origin/main
```

The working tree was therefore clean.

## Maintaining This Document

This document should be updated when the infrastructure of foobox
relevant to the Novena build changes substantially.

This includes, in particular:

- CPU or RAM
- build operating system
- Nix version
- `/etc/nix/nix.conf`
- cross-compilation platform
- relevant build parallelism
- serial adapter or its persistent device path
- storage relevant to builds
- additional build tools used permanently

Short-lived kernel, device-tree or I2C diagnostic states,
on the other hand, do not belong in this document.


## 2026-10-11 – Separate Evidence and Log Transport

`GITHUB-UPLOAD-22-20261011-081030-9t8jAP0Y.txt` records
`git version 2.56.0` on foobox. This supplements the historical Git
version 2.55.0 from September 14; it does not constitute a complete new
verification of the operating system, kernel, RAM, Nix configuration, or
other host properties.

The operational rules in the local working-tree file
`NOVENA-ARBEITSREGELN.txt` (repository root, especially section 11;
availability in the published repository is not guaranteed), and the
separate direct-upload operating rules at the local foobox path
`/home/loomit/novena-backups/github-transport/DIREKTUPLOAD-ARBEITSREGELN.md`
describe the runner, wrapper, and uploader. Original local logs are stored
under `~/novena-backups/block-logs/`; the separate transport repository is
under `~/novena-backups/github-transport/project-block-logs/`.

GITHUB-UPLOAD21 documents the transport documentation checkpoint;
GITHUB-UPLOAD22 is a read-only inventory without executed test cases.
GITHUB-UPLOAD24 adds the wrapper's completion display; its syntax check
is not a complete functional qualification of the uploader. The documented
success and failure subcases remain limited. Network failures, resumption,
idempotency, conflicts, and consistency with help output remain open.
Technical block results and transport results must be assessed separately.
Archiving logs does not extend any project authorization.
