This is a translation. The German original is authoritative.

# Build

Documentation addendum dated 2026-10-11: The qualified G2X build of
2026-09-22 is a historical reference. On 2026-10-11, the local, uncommitted
working tree contained patch stack 0001–0012 in `boot/u-boot/default.nix`,
including diagnostic instrumentation and DDR diagnostic variants. This
diagnostic state is not part of the committed source revision documented
here and must not be equated with G2X. The artifact hashes and
reproducibility evidence below do not transfer to it. No new build was
performed or qualified here. For the later hardware state, the
[DDR checkpoint of October 7](../../PROJECT-STATE.md#maßgeblicher-ddr-testcheckpoint--block-9c-52q-r7b-3aaj-2026-10-07)
is authoritative; the root cause remains unresolved.

Context as of 2026-10-02: The U-Boot/image identities and the
same-host/separate-store reproducibility verification below refer to
the historical G2X reference state of 2026-09-22.

The later Patch-0008 diagnostic image used for the test on 2026-10-01
was fully built and readback-verified as a separate state. Its qualification
is documented in `docs/PROJECT-STATE.md` under “BLOCK 4.10O-1N-7B bis 7E –
vollständiges Patch-0008-Image als Pre-POR-Zustand qualifiziert”.
The G2X reproducibility verification does not apply to this diagnostic image.

The single hardware test on 2026-10-01 yielded
`8B = CONTROL_FAIL_TARGET_FAIL`; its execution and limitations are documented
in `docs/TEST-LOG.md`. A complete self-contained P_EXT boot and a specific
root cause remain unproven.

## Build System

Primary development and build machine:

`foobox`

Host architecture:

`x86_64-linux`

Target architecture:

`armv7l-linux`

The complete Novena SD image is produced with Nix as a cross-build.

The dated historical inventory of foobox is documented in
`BUILD-HOST.md`.

## Flake and Source Revision

Project root:

`~/nixos-novena-image`

The Nix dependencies are fixed by `flake.nix` and `flake.lock`.

The project checkpoint used for the historical G2X reproducibility verification
is:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

Tree:

`4fb8a30d07273d2eb288f6aed49423ea30eb12a1`

The pinned Nixpkgs revision is:

`a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4`

NAR hash:

`sha256-r2f1oUwixlgq9zOdYLqJLfS/lWBT60/IITjhTKI59JU=`

## Production Kernel

Current kernel:

`6.18.49`

The production kernel patch series contains only the Novena adaptations
required for operation:

1. `kernel/0001-drm-bridge-it6251.patch`
2. `kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

The earlier I2C diagnostic patches are no longer part of the production
kernel revision.

`es8328-power` with `regulator-always-on` is a permanent
Novena board requirement.

## U-Boot of the Historical G2X Reference Build

The bootloader is built locally within the project from U-Boot `v2026.07`.

Upstream commit:

`ece349ade2973e220f524ce59e59711cc919263f`

Version identity:

`2026.07-00003-gf8baca04e22d`

`SOURCE_DATE_EPOCH`:

`1789984363`

The build uses the standard pinned
`pkgs.buildUBoot` component.

Artifacts qualified for the historical G2X reference build:

* `SPL`
  * Size: `52224` bytes
  * SHA-256:
    `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0`
* `u-boot-dtb.img`
  * Size: `602120` bytes
  * SHA-256:
    `c5a2d0e7f2b9ebeae6387eee98630a43944f2cd2b52fc8a5a98a2e82ba55f9f7`

The image recipe incorporates these artifacts built locally within the project directly.
The historical files under `boot/reference/` now serve only as
reference artifacts.

## Deterministic Bash Cross-Build

A reproducibility issue in the ARM Bash binary was found during a
complete rebuild in a separate Nix store.

During the cross-build, Bash generated `builtins/pipesize.h` using a
helper program executed on the build host. This allowed the
build host's dynamic pipe state to enter the
ARM target binary as `PIPESIZE`.

In particular, the following values were observed:

* `65536`
* `8192`

The cause was traced to the host-side pipe capacity and its change
when the Linux pipe soft limit was reached.

The project fix is located at:

`patches/bash/0001-bash-cross-build-use-deterministic-pipe-size.patch`

and is enabled through:

`overlays/reproducible-bash.nix`

only for cross-builds.

For the cross-build, the following is set to a fixed value:

`NIX_CROSS_PIPESIZE=4096`

`4096` is used here as a deterministic conservative Linux target value
and is not understood as a claim about the dynamic actual
pipe capacity.

The fix applies to both the interactive and the
non-interactive Bash build.

Native Bash builds remain unchanged by the cross-only overlay.

The resulting ARM binaries were checked separately. In both variants,
the value `4096` was verified in the relevant code; the earlier
host-side generation through `psize.aux` and `psize.sh` no longer takes place in the
qualified cross-build.

The corresponding project commit is:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

## Historical Qualified G2X SD Image – 2026-09-22

Normal-store output:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Size:

`2581291008` bytes

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

The static image check confirmed, among other things:

* DOS/MBR disk ID `0x2178694e`;
* partition 1 starting at sector `16384`;
* partition 2 starting at sector `278528`;
* project-local SPL at offset `1024`;
* project-local `u-boot-dtb.img` in the FAT partition;
* `boot.scr`, kernel, initrd and DTB in the FAT partition;
* explicit `mmc 1:1` load operations in the boot script.

## Reproducibility Verification

The complete image was subsequently rebuilt from exactly the commit

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

in a newly created isolated Nix store.

The clean store did not use any already realized
Novena-specific outputs from the normal `/nix/store`.
Normal cache dependencies were permitted.

Clean-store result:

* logical Nix path:
  `/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`
* size:
  `2581291008` bytes
* SHA-256:
  `8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

The direct byte comparison between the normal-store and
clean-store images yielded:

`cmp = 0`

This proves byte-for-byte reproducibility on the same build host across two
separate Nix stores.

This verification is not yet a cross-host reproducibility verification.
A later independent build on the L14 is separate from this.

## Historical result Symlink

The existing symlink `result` is deliberately not redirected to the current
image.

It continues to point to the historical Golden Build:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

This preserves the historical reference unchanged.

## Hardware Qualification

The historical G2X software build and its same-host/separate-store
reproducibility are qualified within the scope established at that time.

The actual P_EXT test on the Novena now verifies the following part of the
boot chain:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07`

The subsequent complete path has not yet been verified:

`SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

The later A-B-B-A finding of October 7 shows Q1/C1 ZERO already before
MMC1, followed by D32 `ret=-12`. This MMC endpoint does not explain the
earlier signature. Further hardware qualification requires separate
authorization. The complete self-contained P_EXT boot remains unverified.

## Kernel Working Directories

During development, the following may exist locally, among others:

* `kernel/linux-6.18.49`
* `kernel/linux-6.18.49-orig`
* `kernel/panel-patch`

These directories are working copies and are not versioned.

Kernel changes of lasting relevance must be available as patch files under
`kernel/`.
