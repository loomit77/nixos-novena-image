> This is a translation. The German original is authoritative.

# Image Layout

Context as of 2026-10-02: The artifact identities and reproducibility
verification below describe the historical G2X reference state of
2026-09-22. They must not be read as identities of the later Patch-0008
diagnostic image. Its separate image and medium qualification is documented
in `docs/PROJECT-STATE.md`.

Addendum dated 2026-10-11: The later BI0/Patch0012 diagnostic images
are separate artifacts as well. The historical G2X identities and their
reproducibility evidence do not transfer to these variants.

The earlier localization of the stall in SPL-side MMC initialization
describes the investigation corridor observed at the time. Later Q1 and
control findings already concern the early memory path before MMC
initialization. MMC has not been demonstrated to be the origin; `ret=-12`
does not establish a DDR root cause.

The complete self-contained P_EXT boot remains unproven. The documentation
reference dated 2026-10-11 points to the
[DDR A-B-B-A checkpoint of October 7](../../PROJECT-STATE.md#maßgeblicher-ddr-testcheckpoint--block-9c-52q-r7b-3aaj-2026-10-07).
Earlier assessments of case 8B remain historical checkpoints.

## Historical Reference

The project originated from the analysis of a known
Novena NixOS image.

The historical boot files under `boot/reference/` are retained in the
repository for provenance and comparison. However, they are
no longer the active source of the production bootloader.

## Partition Table of the Historical G2X Image – 2026-09-22

The historical G2X image uses a DOS/MBR partition table.

Disk ID:

`0x2178694e`

Logical sector size:

`512` bytes

Total size:

`2581291008` bytes

Total number of sectors:

`5041584`

### Partition 1

Type:

`W95 FAT32 (LBA)`, MBR type `0x0c`

Label:

`FIRMWARE`

Start:

Sector `16384`

Size:

`262144` sectors = `128 MiB`

The FAT partition contains the boot files for U-Boot.

### Partition 2

Type:

Linux, MBR type `0x83`

Label:

`NIXOS_SD`

Start:

Sector `278528`

Size:

`4761008` sectors

The partition is marked as bootable in the MBR.

It contains the ext4 root filesystem of the NixOS system.

## Area Before the First Partition

The area before sector `16384` contains the SPL.

The SPL built locally within the project is written starting at byte offset:

`1024`

into the image.

SPL of the historical G2X reference image:

* Size: `52224` bytes
* SHA-256:
  `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0`

The SPL is based on U-Boot `v2026.07` with the project's local
Novena modifications and is intended for the external V1 path
on USDHC2.

## Contents of the FIRMWARE Partition

The static inspection of the historical qualified G2X image found:

* `u-boot-dtb.img`
* `zImage`
* `initrd.uimg`
* `novena.dtb`
* `boot.cmd`
* `boot.scr`

The embedded `u-boot-dtb.img` is the U-Boot-v2026.07 artifact
built locally within the project:

* Size: `602120` bytes
* SHA-256:
  `c5a2d0e7f2b9ebeae6387eee98630a43944f2cd2b52fc8a5a98a2e82ba55f9f7`

Other artifacts extracted during the static test:

### Kernel

File:

`zImage`

Size:

`17457664` bytes

SHA-256:

`20a10cebceb35e44440f626a80d1b69245b21d917023dc31998841569af21bc6`

### Initrd

File:

`initrd.uimg`

Size:

`27925712` bytes

SHA-256:

`55609b167c97ee15afb06075b4b3e8127a60e8f1c0298960fc1849843b32a3bc`

### Device Tree

File:

`novena.dtb`

Size:

`59628` bytes

SHA-256:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

## Boot Script

The `boot.scr` contained in the historical G2X image sets the NixOS system
path and explicitly loads all boot files from the external SD via `mmc 1:1`.

Load addresses used:

* Kernel: `0x12000000`
* Initrd: `0x20000000`
* Device Tree: `0x1f000000`

The final boot is performed with:

`bootz 0x12000000 0x20000000 0x1f000000`

The system closure contained in the historically statically qualified G2X image is:

`/nix/store/axbsx57xnrbqdya5bk877g21g87kish7-nixos-system-novena-26.05.20260903.a5cc6f2`

## Historical Qualified G2X Image – 2026-09-22

Nix store path:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Size:

`2581291008` bytes

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

A complete rebuild from the same Git commit in a separate
isolated Nix store produced exactly the same size and the same
SHA-256 value.

A direct `cmp` comparison of the two images showed byte-for-byte identity.

## Historical Reference Boot Files

The following files are still located under `boot/reference/`:

* `novena.dtb`
* `novena-imx6-spl.bin`
* `u-boot-dtb.img`
* `zImage`

These files document a known historical
Novena boot state.

They must not be confused with the active boot artifacts built
from the current project.

## Limits of Hardware Evidence

The static image inspection confirms the layout and the embedded
artifacts.

On real Novena hardware, the following part of the
self-contained boot chain has now been confirmed:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07`

The subsequent path has not yet been confirmed:

`SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

The A-B-B-A finding of October 7 shows DQS `ret=0`, Q1=0 and C1=0 with
`valid=7` in all four runs, followed by MMC1 and D32 `ret=-12`.
The MMC endpoint must be distinguished from the preceding Q1/C1 zero
signature; it does not explain that signature. Neither a DDR cause nor a
complete self-contained boot has been established. This observation neither
changes the historical G2X identities nor qualifies newer image layouts.
