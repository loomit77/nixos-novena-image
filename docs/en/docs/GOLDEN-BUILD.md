This is a translation. The German original is authoritative.

# Golden Build – Working Novena Display

Historical document state: 2026-09-13.

Context as of 2026-10-02: This document preserves the historical Golden
reference and the I2C/display diagnostic intermediate states of that time.
Statements about an unresolved root cause, diagnostic kernels and next
steps must be read in that historical context.

The functional ES8328/I2C3 investigation was subsequently completed;
`regulator-always-on` for `es8328-power` remains a permanent Novena board
requirement. The functional conclusion of 2026-09-20 is documented in
`docs/PROJECT-STATE.md` under “2026-09-20 – Block 3.15W –
Historische Root-Cause-Synthese ES8328/I2C3”. It does not determine the
exact internal leakage-current path or the analog voltage and current
waveforms of the investigated failure.

The historical Golden backup remains preserved unchanged.
The current P_EXT/SPL investigation is separate from it.

## 2026-10-11 – DOC-UPDATE-02B: Historical Identities and Evidence Limits

This clarification is based on the local source review DOC-UPDATE-02A of
2026-10-11, preserved at
`/home/loomit/novena-backups/block-logs/DOC-UPDATE-02A-20261011-085639-6dYeUv41.txt`.
The historical statements below remain as documentation of the state at the
time; this addendum clarifies the limits of what they establish, without any
new artifact verification.

| Date | Subject | Evidence and limitation |
|---|---|---|
| 2026-09-11 | Golden backup, image `5e65bfa6…31ab94b`, separate DTB `e36cd0c8…38f4cd6f` | Backup metadata and manifest; metadata commit `f26464f` does not establish complete pre-Git build provenance. The full hashes are given in the artifact sections below. |
| 2026-09-12 | Successful display test, subsequently documented baseline DTB `31f2b35e…18377e0f` | [Historical hardware test](../../TEST-LOG.md#2026-09-12--erfolgreicher-hardwaretest): stable 1920x1080 link. The byte-exact association of the complete image and media state tested with the unchanged Golden backup from September 11 remains unresolved. |
| 2026-09-22 | Separate G2X reference build from `d06de5e30f294144d0be66cc7903fedb1ed6fe0a`, U-Boot v2026.07 | `2581291008` bytes, SHA-256 `8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`; historically documented same-host/separate-store byte comparison. |

G2X is a historical build reference; details are in
[BUILD.md](BUILD.md#historical-qualified-g2x-sd-image--2026-09-22)
and the [G2X test addendum](../../TEST-LOG.md#2026-09-22--vollständiger-g2x-clean-store-reproduzierbarkeitstest).
DOC-UPDATE-02A did not find a separate original G2X execution/comparison raw
log or the designated clean-store path.
The documented comparison is not cross-host evidence, not a completely
cache-free rebuild of all dependencies, and not a qualification of later
diagnostic images. P_EXT success through SPL v2026.07 does not establish a
complete external boot. A subsequent historical SPL/U-Boot 2015 boot path
through login in the preserved serial capture does not close this evidence
gap. See the [P_EXT test section](../../TEST-LOG.md#2026-09-23--p_ext-hardwaretest-und-spl-diagnose-d0-bis-d36)
and the [serial boot capture](../../../test-logs/p-ext-2026-09-22/p-ext-boot-01.log).
Clarification of 2026-10-11 – DOC-FIX-02: The test section explicitly notes
the additional serial content after the first P_EXT SPL attempt and its
unresolved provenance; this does not qualify a complete external boot path.
The DDR/SPL root cause remains OPEN; patch0012 is not a generally proven fix
(see the [October 7 checkpoint](../../PROJECT-STATE.md#maßgeblicher-ddr-testcheckpoint--block-9c-52q-r7b-3aaj-2026-10-07)).

The repeated kernel, DTB, and STMPE details below refer to different things:
artifact identity, overlay comparison, and diagnostic progression. They are
preserved; the diagnostic sections describe the state of September 12–13,
not the subsequent functional conclusion of September 20.

## Purpose

This document describes the backed-up and verified Golden Build
of the reproducible NixOS image for the Kosagi Novena.

The Golden Build serves as a known working reference that can be used
for later kernel, device-tree, DRM, display, I2C or
bootloader experiments.

It is important to distinguish several levels:

1. the Golden Build backup created on 2026-09-11
2. the historical display DTB contained in it
3. the later Git revision of the project
4. the successful hardware validation of the current image
   on real Novena hardware on 2026-09-12

The backup from 2026-09-11 is not modified or overwritten.

## Associated Git Revision of the Original Golden Build Backup

Repository:

```text
~/nixos-novena-image
```

The following Git commit is documented in the metadata of the original
Golden Build backup:

```text
f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7
```

Short form:

```text
f26464f
```

Commit:

```text
Add reproducible Novena NixOS image project
```

This commit is the root commit of the existing Git history.

Important:

The Git commit documented in the backup must not be understood as proof
that all binary artifacts backed up in it were built from
exactly this committed source state.

In particular, the backed-up Novena DTB with SHA-256

```text
e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f
```

was subsequently identified as an artifact of an earlier pre-Git
overlay state.

The exact repository contents of commit `f26464f` were also backed up as
an independent tar.gz archive.

## Additional Git Backup Checkpoint from 2026-09-12

After the successful hardware validation, an additional
annotated Git tag was created:

```text
novena-display-working-2026-09-12
```

The tag points to commit:

```text
1d43425
```

Commit description:

```text
Document Nix closure backup for golden build
```

This tag marks a confirmed working intermediate display state
before further changes to the
I2C diagnostic instrumentation.

## Kernel

Kernel version:

```text
Linux 6.18.49
```

Backed-up kernel:

```text
kernel/zImage
```

SHA-256:

```text
30a63036889af44301f4821f76ca66a7b8f428c18e725ca5ec29775b6d5fbf5c
```

The complete kernel output tree was included in the Golden Backup.

The corresponding Golden Kernel is located in the Nix store at:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

The derivation examined later shows the display patches
`0001` and `0002` for this Golden Kernel.

The diagnostic patches `0003` and `0004` developed later are not part
of the historical Golden Kernel.

Later `0003` debug kernel:

```text
/nix/store/2d1h5iihfq560zm0sg4kh4n4ydvk2a2m-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Still later `0004` diagnostic kernel:

```text
/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

These debug builds are diagnostic developments after the Golden Build and
must not be interpreted as part of the original
backup.

## Novena Device Tree

Separately backed-up Novena DTB:

```text
dtb/imx6q-novena.dtb
```

SHA-256:

```text
e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f
```

This DTB uses Linux 6.18.49 as its kernel base, but comes from
an earlier device-tree overlay state than the project state
later versioned in `hardware/novena.nix`.

The corresponding unchanged kernel base DTB has the SHA-256 value:

```text
b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522
```

The historical Golden DTB was traced through preserved Nix derivations to
the following overlay output:

```text
/nix/store/pgyag1yxfcrwf1lz4zqpjhgag2ah564b-device-tree-overlays/imx6q-novena.dtb
```

The display DTB currently booted at the later stage, by contrast, has the SHA-256 value:

```text
31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f
```

Both DTB states are based on the same kernel base DTB.

## Kernel Configuration

Backed-up kernel configuration:

```text
config/linux-6.18.49.config
```

SHA-256:

```text
cef1365fe594704ec72394e8abbe2b96cc8ccad37090fcff42aa8d221aaa5ef3
```

Thus, in addition to the finished kernel, the configuration of the known
build state is also preserved separately.

## Finished SD Image

Backed-up image:

```text
image/nixos-novena-sd-image.img
```

SHA-256:

```text
5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b
```

This image corresponds to the NixOS Novena build successfully completed
on 2026-09-07.

Nix store origin:

```text
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

This image was successfully tested on real Novena hardware on 2026-09-12
with working Linux-side display initialization.

Clarification dated 2026-10-11: This historical attribution is not fully
substantiated at the byte level. The documented display success stands, but
there is no demonstrated complete chain from the unchanged backed-up full
image through the storage-medium state at the time to the specific boot.
In particular, the Golden DTB and the later display-baseline DTB differ.

## Hardware Validation on 2026-09-12

The current build was cold-started on real Novena hardware
on 2026-09-12.

The test reached:

* SPL/U-Boot
* Linux 6.18.49
* NixOS Stage 1
* root filesystem
* regular login
* successful Linux-side IT6251 power supply
* successful IT6251 product ID detection
* successful IT6251 initialization
* successful DisplayPort link training
* stable internal display output at 1920x1080

The product ID was read correctly on the very first attempt.

Registers read:

```text
0x00 = 0x15
0x01 = 0xca
0x02 = 0x51
0x03 = 0x62
```

Kernel message:

```text
IT6251 detected: vendor ca15 device 6251
```

## Display Link

IT6251 initialization succeeded on the very first attempt.

Link training ended after ten iterations.

System status:

```text
0x3e
```

Measured active resolution:

```text
hactive: 1920
vactive: 1080
```

Success indicators:

```text
is_stable: stable 1920x1080
display link stable
bridge_enable: exit success
```

This confirms that the current Linux-6.18.49 state is fundamentally capable
of fully initializing the internal Novena display path.

## Regulator Status After Successful Initialization

After the successful display startup, the GPIO states were checked.

Result:

```text
gpio-15 (regulator-lvds-lcd) out hi
gpio-28 (regulator-display) out hi
```

Regulator summary:

```text
lcd-lvds-power
```

* active
* 3300 mV
* backlight consumer active

and:

```text
lcd-display-power
```

* active
* 3300 mV
* consumer `2-005c-power` active

Both the display and backlight power supplies were therefore active.

## Relevant Device-Tree Corrections

The currently working display state includes, among other things:

* explicit LDB clock assignments
* `single-master;` on the display I2C bus
* Novena-specific IT6251 pinmux
* Novena-specific backlight pinmux
* `startup-delay-us = <2000000>` for `reg_display`
* no `regulator-always-on` on `reg_display`
* no `regulator-always-on` on `reg_lvds_lcd`

With this overall state, stable internal display output at
1920x1080 was confirmed.

The exact isolated effect of the clock assignments was not separately
verified in the preserved pre-Git records.

Historically, the previously visible `-EAGAIN` errors on the display I2C bus
disappeared after the introduction of `single-master;`.

However, a later source code analysis showed that `single-master;`
internally sets `multi_master = false`, thereby skipping the normal IAL
handling in the driver.

The property must therefore not be understood as proof
that hardware arbitration lost was completely prevented.

## Historical Display Overlay State

The historical Golden DTB `e36cd0c8...` differs from the DTB `31f2b35e...`
currently booted at the later stage.

The earlier state included, among other things:

* `regulator-always-on;` on `reg_display`
* `regulator-always-on;` on `reg_lvds_lcd`
* `startup-delay-us = <200000>` on `reg_display`,
  already present in the kernel base DTB
* no `single-master;` on the display I2C bus
* none of the later explicit clock assignments

The later working state, by contrast, contains:

* no `regulator-always-on` on `reg_display`
* no `regulator-always-on` on `reg_lvds_lcd`
* `startup-delay-us = <2000000>`
* `single-master;`
* explicit clock assignments

The two states must therefore not be treated as the same device-tree
generation.

## Expected IT6251 Reset NACKs

During the IT6251 reset sequences, the following messages were
temporarily observed:

```text
error -6 writing to eDP addr 0x5
error -6 writing to LVDS addr 0x5
```

These messages correspond to the known behavior during the
IT6251 reset sequence and did not prevent successful display
startup.

Initialization subsequently continued in full and ended with
a stable 1920x1080 link.

## Current Limitation

Historical diagnostic status from 2026-09-12/13; contextualization dated
2026-10-11: The functional I2C3/ES8328 root cause described as open below
was closed on 2026-09-20; see the
[closure decision](../../DECISIONS.md#2026-09-20--funktionale-es8328i2c3-root-cause-untersuchung-abgeschlossen).
The observations made at that time and their test conditions remain preserved.

The successful test on 2026-09-12 confirmed that the Linux-6.18.49 display path is fundamentally capable of working in full.

However, later tests showed that this success is not reproducible on every genuine cold start. Five genuine cold starts were performed with a test DTB in which only the STMPE811 had been disabled: 2/5 direct display successes, 3/5 direct display failures and 3/3 successful DRM rebind recoveries.

The subsequent `0004`/`0005` diagnosis further localized the decisive cold-boot divergence to the transition into the master/START state:

`FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

versus:

`Same-Boot Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Removing `single-master;` also does not prevent the IAL event reported by the i.MX6 I2C controller.

This later diagnosis does not change the historical Golden Build. The Golden Build remains a known working reference, but is not proof of deterministic cold-boot stability. The deeper root cause of the START/IAL behavior remains unresolved.

## Separate i2c-0 Diagnostic Problem

The current diagnostic patch also logs arbitration-lost events
on other i.MX I2C controllers.

On `i2c-0`, large quantities of the following message were generated after boot:

```text
NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
```

In addition, the following was observed:

```text
<i2c_imx_write> write timedout
```

This problem must be considered separately from successful IT6251 operation on
`i2c-2`.

Before further reproducibility tests, the diagnostic patch must be
made less intrusive or restricted to the relevant bus.

## STMPE811 / Touchscreen

The STMPE811 at I2C address `0x44` is active in both the historical
Golden DTB `e36cd0c8...` and the later working baseline DTB
`31f2b35e...`.

Later, the STMPE811 was disabled in a controlled test using a separate
overlay.

Baseline DTB:

```text
31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f
```

STMPE-disabled test DTB:

```text
d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8
```

The semantic DTB comparison showed `status = "disabled"` at the STMPE811 node
as the only content change.

In the hardware test, the STMPE/`0-0044` errors thereby disappeared
completely, while the intermittent IT6251 cold-boot problem
persisted.

This experimentally confirms that the observed
STMPE811/touchscreen errors are not the cause of the intermittent
IT6251 cold-boot problem.

## Project Archive

Archive of the Git commit documented in the original Golden Build
backup:

```text
metadata/nixos-novena-image-f26464f.tar.gz
```

SHA-256:

```text
75779166a971aba60003c0599294e3f69389bce789375e1e47d3baf8d295f6b3
```

The archive was generated directly from commit `f26464f` using `git archive`.

Thus, in addition to the Git remotes, an independent copy
of the exact project sources of this commit exists.

Important:

This Git archive documents the source state of `f26464f`.

It is not proof of provenance that the historical
Golden DTB `e36cd0c8...` was built from exactly this committed
source state.

## Golden Backup Path on foobox

Local backup path:

```text
~/novena-backups/nixos-display-working-2026-09-11/
```

Total size after completion of the Nix closure backup:

```text
6,2G
```

Layout:

```text
nixos-display-working-2026-09-11/
├── SHA256SUMS
├── config/
│   └── linux-6.18.49.config
├── dtb/
│   └── imx6q-novena.dtb
├── image/
│   └── nixos-novena-sd-image.img
├── kernel/
│   ├── System.map
│   ├── zImage
│   └── dtbs/
├── metadata/
│   ├── build-info.txt
│   ├── image-references.txt
│   ├── kernel-references.txt
│   ├── nix-closure-info.txt
│   └── nixos-novena-image-f26464f.tar.gz
└── nix/
    ├── image-closure.nar
    └── kernel-closure.nar
```

## Integrity Check

Source clarification dated 2026-10-11: The local backup at
`/home/loomit/novena-backups/nixos-display-working-2026-09-11/` contains the
[manifest](/home/loomit/novena-backups/nixos-display-working-2026-09-11/SHA256SUMS),
[build metadata](/home/loomit/novena-backups/nixos-display-working-2026-09-11/metadata/build-info.txt),
and [closure metadata](/home/loomit/novena-backups/nixos-display-working-2026-09-11/metadata/nix-closure-info.txt).
The manifest lists the image, DTB, kernel, configuration, source archive, and
both closure exports with the hashes documented below. The closure metadata
lists 605 image store paths and a separately exported kernel store path;
the respective sizes are `3833159224` and `96724392` bytes.
`metadata/image-references.txt` and `metadata/kernel-references.txt`
document direct store references, not complete source provenance.
The following check of 1398 files is a historical result; DOC-UPDATE-02A/02B
do not constitute a current complete integrity check of the images, archives,
or closures, nor an import or recovery test.

A common SHA-256 checksum list was generated for all regular files
of the Golden Backup:

```text
SHA256SUMS
```

After completion of all backup steps, the entire backup was checked again
with the following command:

```text
sha256sum -c SHA256SUMS
```

Final result:

```text
Fehlerhafte Einträge: 0
Geprüfte Dateien: 1398
```

At the time of the check, all 1398 backed-up files were therefore
bit-for-bit consistent with the checksum list.

## Nix Store Origin

At the time of the backup, the project symlinks pointed to the following
Nix store objects.

Image:

```text
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

Kernel:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Novena DTB:

```text
/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb
```

Kernel configuration:

```text
/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49
```

The immediate Nix store references of the image and kernel were
also saved:

```text
metadata/image-references.txt
metadata/kernel-references.txt
```

The saved metadata must be read in the context of the pre-Git provenance
of the historical device tree described
above.

## Nix Closure Backup

In addition to the finished binary artifacts, the existing
Nix store closures were exported.

### Image Closure

File:

```text
nix/image-closure.nar
```

Size:

```text
3833159224 bytes
```

Store paths included at the time of export:

```text
605
```

SHA-256:

```text
8228f00e3f9f85046a55621727e82f70395125e75f00ee0fc497402879464b02
```

### Kernel Closure

The kernel store path was not part of the previously determined
image closure and was therefore exported separately.

File:

```text
nix/kernel-closure.nar
```

Size:

```text
96724392 bytes
```

SHA-256:

```text
5f44cfedcdea2d98cb3b8ea4b56fae35e5c0894a8ca5beffb8f78e080e8b069f
```

The details of the closure backup are also documented in:

```text
metadata/nix-closure-info.txt
```

The archives can later be imported into a compatible Nix store
with:

```text
nix-store --import < archive.nar
```

The closure archives supplement the binary Golden Build artifacts and the
project sources.

They replace neither the finished SD image nor the Git repository.


## Later Diagnostic Development After the Golden Build

After the Golden backup, I2C diagnosis was continued.

Golden Kernel:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Patch base: `0001`, `0002`.

Later `0003` debug kernel:

```text
/nix/store/2d1h5iihfq560zm0sg4kh4n4ydvk2a2m-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Deriver:

```text
/nix/store/422lrj9vi0ydb6cczl0ib8wjx8ll7sqc-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv
```

Patch base: `0001`, `0002`, `0003`.

New `0004` diagnostic kernel:

```text
/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Deriver:

```text
/nix/store/qk21vdydrywnkqlh3rrvg7n9zn4pwapk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv
```

Source:

```text
/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz
```

Patch base: `0001`, `0002`, `0003`, `0004`.

This build was successfully created and its provenance verified. The
hardware tests performed subsequently are documented in the following
section.

## Diagnostic Milestone After the Golden Build: 0004/0005

Historical test status dated 2026-09-13; see the
[0005 test](../../TEST-LOG.md#2026-09-13--0005-cold-boot-fail-und-same-boot-rebind-pass).
The statement below that the root cause was open at that time is placed in
context by the functional closure on 2026-09-20; it is not a currently open task.

The historical Golden backup remains unchanged. The following
results belong exclusively to later diagnostic development and
must not be read as properties of the historical Golden DTB or
of the original Golden Build state.

With `0004`, a cold-boot FAIL with active `single-master;` showed:

`A=81/80 B=93/80 C=93/80 D=93/d8 E=93/d8 IRQ=93/d8`

After isolated removal of `single-master;`, the cold-boot failure persisted. The STMPE-disabled DTB used in this test has SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Without `single-master;`, the driver detected `IAL=0x93` and aborted with `-EAGAIN`. This confirms that `single-master;` neither causes nor prevents the IAL event reported by the i.MX6 I2C controller.

With `0005`, the early failure path was directly instrumented. Patch SHA-256:

`81c2de96d7200a6e7e2684518c711bc5686b8d5295a52d5e7aaa54efed94e556`

Kernel:

`/nix/store/97pmqlp5xvsh1l0i877lfc4r4sq6i7np-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

SHA-256 of the `zImage`:

`197ce0ef325307a877eb5a889519387be91fdf536da3bee03883f0c40264fc7c`

The first genuine cold boot with this state failed. Boot ID:

`48462ba4-4972-4d23-9261-e69c822ed584`

Direct START diagnosis:

`A=81/80 B=93/80 C=83/80 ret=-11`

In the same boot, a DRM rebind succeeded:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

with `IRQ=a2/f8` or `IRQ=a6/f8`, respectively. The rebind reached a stable 1920x1080 link.

This localizes the cold-boot divergence directly to entry into the master/START state. The deeper root cause remains unresolved.

The complete `0005` test archive has SHA-256:

`0c38c954a7fac033b5fd619653efd8eb0d5fffa3c4024329cb6d85d247e1c3db`

and was verified to be identical on Novena and foobox.

## Display-Relevant Project Files

The current project state includes, in particular:

```text
kernel/0001-drm-bridge-it6251.patch
kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch
kernel/0003-i2c-imx-debug-arbitration-lost.patch
kernel/0004-i2c-imx-debug-start-state.patch
kernel/it6251.c
kernel/it6251.c.before-drm-lifecycle
```

These files must be considered together with

```text
flake.nix
flake.lock
hardware/novena.nix
image/novena-image.nix
```

and the kernel configuration used when reproducing or further developing
the working display state.

For device-tree provenance, it must also be taken into account that the
historical Golden DTB `e36cd0c8...` comes from an earlier pre-Git
overlay state and must not be reconstructed solely from Git commit `f26464f`.

## Recovery Strategy

For later changes to the display, DRM, I2C or boot path, the
Golden Build should be preserved as a fallback reference.

Before risky changes, at least the following should be backed up or
checked, as applicable:

1. current Git revision
2. current DTB hashes
3. current image hash
4. working fallback DTB
5. external SD test medium
6. serial console access

The original Golden Backup from 2026-09-11 remains unchanged.

New experimental states should be stored in separate builds, Git commits or
additional backups.

## Currently Confirmed Reference State

Historical reference/diagnostic state from September 12–13, 2026; “current”
refers to the state at that time, not to October 11, 2026.

For further work, the historical Golden state and the
later diagnostic states must be considered separately.

The historical Golden Build from 2026-09-11 remains unchanged and
continues to be the backed-up working reference.

For the display baseline state reproduced later, the following applies:

* Kernel: Linux 6.18.49
* NixOS: 26.05.20260903.a5cc6f2
* reproduced display baseline DTB:
  `31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`
* kernel base DTB:
  `b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`
* historical Golden DTB:
  `e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`
* IT6251 product ID can be read successfully
* display link can reach a stable 1920x1080
* Linux can switch on the display power supply itself
* `single-master;` is active on the display I2C bus
* `startup-delay-us = <2000000>` is active on `reg_display`

The STMPE811-disabled DTB subsequently tested in isolation has:

`d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8`

The semantic difference from the reproduced
display baseline DTB consists exclusively of:

```dts
status = "disabled";
```

at the STMPE811 node.

The hardware test confirmed:

* the STMPE811/`0-0044` errors disappear completely
* the intermittent IT6251 cold-boot problem persists

This experimentally confirms that the observed
STMPE811/touchscreen errors are not the cause of the intermittent
IT6251 cold-boot problem.

The subsequent five-cold-boot series yielded:

* 2/5 direct display successes
* 3/5 direct display failures
* 3/3 successful DRM rebind recoveries after a failure

The current diagnostic state is therefore not yet a reproducible
production state.

In particular, the extensive I2C diagnostic path is not yet suitable for a
final production kernel.

The diagnostic kernel with
`kernel/0004-i2c-imx-debug-start-state.patch` was successfully built,
its provenance verified, and it was tested on real Novena hardware on
2026-09-13. Native cold start: FAIL. The subsequent same-boot rebind: PASS.
A rebind PASS is not a successful native cold start.
The hardware tests are documented in `docs/TEST-LOG.md` under
“2026-09-13 – Hardwaretest Diagnose-Patch 0004 mit single-master”
and “2026-09-13 – isolierter Test ohne single-master”.

This kernel patch must be distinguished from
`boot/u-boot/0004-novena-uboot-diag-d0-d36.patch`, which belongs to the
separate P_EXT/SPL diagnostic investigation. The identical patch number
refers to different investigations.

## Next Steps

Historical plan dated 2026-09-13; contextualization dated 2026-10-11:
The following I2C work items are not current outstanding requirements or
authorizations to run tests. I2C3/ES8328 has been functionally CLOSED since
2026-09-20; `regulator-always-on` for `es8328-power` remains a permanent
board requirement.

The historical Golden Build remains preserved unchanged as a known working reference.

The later diagnostic development has localized the cold-boot failure much more precisely. The next step is a separate root-cause analysis of the i.MX6 I2C master/START transition.

Starting point:

`Cold Boot FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

versus:

`Same-Boot Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Before a later development state can be classified as a production reference, the following are still required:

1. sufficiently establish the root cause of the differing START/IAL behavior
2. derive a targeted functional correction from it
3. check rollback artifacts and backups before every test
4. pass several identical genuine cold starts without manual recovery
5. remove diagnostic instrumentation or reduce it to the necessary minimum
6. only then establish a new final Golden or production reference state

Until then, the backup from 2026-09-11 remains the unchanged historical Golden backup.
