Translation note: The German original remains authoritative.

# Source Register – Novena / Universal NixOS SD Image

Initial version dated 2026-10-01. Subsequent source additions and status
clarifications are dated separately; the documentation proposal dated
2026-10-04 includes no new online verification.

## Rules

- `GEPRUEFT`: URL viewed online on the specified date; the content is not automatically applicable to our pinned source revision.
- `HISTORISCH`: The source documents an older state; compare it with current sources and artifacts.
- `OFFEN`: Link known, content or attribution not yet checked.
- For reproducible statements, add the commit, tag, revision, access date, and local location wherever possible.
- Before new builds, patches, or hardware tests, first cross-check `docs/TEST-LOG.md`, `docs/DECISIONS.md`, `docs/PROJECT-STATE.md`, and this register.

## U-Boot – Primary Sources and Documentation

| ID | Source | URL | Status | Relevance |
|---|---|---|---|---|
| UB-001 | Official U-Boot repository | https://github.com/u-boot/u-boot | GEPRUEFT 2026-10-01 | Version and commit comparison; the revision pinned for our builds is authoritative. |
| UB-002 | U-Boot Memory Management | https://docs.u-boot.org/en/latest/develop/memory.html | GEPRUEFT 2026-10-01 | SPL memory phases, SRAM/DRAM, BSS, and stack; general description, cross-check for the specific board. |
| UB-003 | U-Boot Kconfig (main) | https://github.com/u-boot/u-boot/blob/main/Kconfig | GEPRUEFT 2026-10-01 | SPL malloc, early allocation, CLEAR_ON_INIT; `main` is not evidence for 2026.07. |
| UB-004 | U-Boot malloc.h (mirror) | https://github.com/ARM-software/u-boot/blob/master/include/malloc.h | GEPRUEFT 2026-10-01 | Declarations of mem_malloc_start/end/brk; take the mirror and branch into account. |
| UB-005 | Official U-Boot Git | https://git.u-boot-project.org/u-boot/u-boot.git | LINK BEKANNT 2026-10-01 | Official Git endpoint; record the specific revision. |

## Novena – Historical U-Boot Sources

| ID | Source | URL | Status | Relevance |
|---|---|---|---|---|
| NV-001 | Marek Vasut: DDR calibration on Novena, patch 2/2 (2015) | https://www.mail-archive.com/u-boot@lists.denx.de/msg196451.html | GEPRUEFT 2026-10-01; HISTORISCH | Particularly important: documented enabling of DDR calibration and adjustment of parameters for the Novena memory module. No established cause of the current case 8B. |
| NV-002 | Novena SPL – Android/Google U-Boot mirror | https://android.googlesource.com/platform/external/u-boot/+/refs/heads/android-tv-s-beta3/board/kosagi/novena/novena_spl.c | GEPRUEFT 2026-10-01; HISTORISCH | DDR I/O, MMDC calibration values, and DDR geometry; compare the revision with the current pinned revision. |
| NV-003 | Novena SPL – historical U-Boot mirror | https://nest-open-source.googlesource.com/nest-learning-thermostat/5.8.2/u-boot/+/refs/heads/master/u-boot-imx/board/kosagi/novena/novena_spl.c | GEPRUEFT 2026-10-01; HISTORISCH | Additional revision for comparison; do not treat it as a current Novena reference. |
| NV-004 | Novena SPL – historical Git tree | https://fedorapeople.org/cgit/ausil/public_git/u-boot.git/tree/board/kosagi/novena/novena_spl.c?id=c1a6f371ae65f1a3709e872e08bed51321e292fc | GEPRUEFT 2026-10-01; HISTORISCH | DDR parameters tied to a specific commit ID. |

## Primary Evidence within the Project (No Internet Links)

- `docs/PROJECT-STATE.md`: Project state and known limitations.
- `docs/DECISIONS.md`: Decisions, preregistration, and case classification.
- `docs/TEST-LOG.md`: Test history, source and artifact identities.
- `test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log`: Case 7C.
- `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`: Case 8B.
- `boot/u-boot/default.nix`, `boot/u-boot/0004-*.patch` through `0008-*.patch`: Instrumentation currently in use.

## Still to Be Completed

This register is a **first, verifiable version**, not a claim that all URLs used in earlier project phases have already been recorded. Existing URLs from `docs/`, `README.md`, `AGENTS.md`, `boot/`, `kernel/`, and the documented project history are to be collected in a separate read-only inventory step, checked for duplicates, and then added together with their source and status. External links must not be marked as having had their content checked solely because they are mentioned.

## Open Research Questions – Block 4.10O-1N-8F-5

1. What specific DDR calibration changes does the historical Novena patch from 2015 contain?
2. Are these changes included, modified, or replaced in the pinned U-Boot-2026.07 source revision?
3. Which DDR calibration functions are actually called in our SPL, and how are return values handled?
4. Which already documented tests answer parts of these questions, so that no repetition is necessary?
5. What additional sources are required before a new test is even considered?

## Local Source Checkpoint – BLOCK 9C-44 (2026-10-02)

None of the following entries represents a new online check.
[Reconstruction report with the exact REF path and source hashes](../../../research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md) (German original).

| ID | Local Source | Status and Limit of the Claim |
|---|---|---|
| SPL-001 | Preserved locked-debug reference ELF `REF/spl/u-boot-spl` | HEUTE HASH-/STATISCH GEPRÜFT; SHA256 `69bb3a4dd2efe059cefc197cad244f13459b0ef08ccb17c886059337ca25e469`; symbol/machine-code evidence, no new boot. |
| SPL-002 | `REF/board/kosagi/novena/novena_spl.c`, `REF/arch/arm/mach-imx/mx6/ddr.c`, `REF/include/wait_bit.h`, `REF/arch/arm/lib/crt0.S`, `REF/.config`, map and wrapper | HEUTE LOKAL GEPRÜFT; board calls, return-value handling, SRAM stack/BSS/tail path; no evidence of actual calibration values. |
| SPL-003 | `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log` | ORIGINALBELEG; read in full, 830 bytes; SHA256 `9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`; case 8B, no root cause. |
| SPL-004 | `boot/u-boot/0008-novena-diag-control-c0-c2.patch` and existing preregistration | LOKAL GEPRÜFT; hardcoded `0x18200030` matches `max_total_mem` in the reference ELF; no guarantee for a new layout. |
| SPL-005 | Repository original reports BLOCK 9C-33 through 9C-42 | FEHLEND in the repository contents checked; no reconstructed execution history. Nor is 9C-43 present here as an original report. |
| SPL-006 | ERR050070 manufacturer evidence and MPDGHWST register-read semantics | OFFENE PRIMÄRQUELLENLÜCKE; no locally verified erratum revision/applicability and no established absence of side effects for additional reads. |
| SPL-007 | Task specifications concerning 9C-42/9C-43 and A/B; correction according to BLOCK 9C-49 | CHAT-/AUFTRAGSANGABE from the earlier chat report on BLOCK 9C-42; no locally verified repository original source, original report 9C-42 not established locally. A: locally initialized diagnostic structure on the SRAM stack with parameters passed to WL/DQS helpers. B: initialized global diagnostic state in SRAM-`.data` with unchanged function signatures. Optional measurement scope is separate from these variants. |

The open questions about return-value handling have been narrowed by the
static reconstruction performed at that checkpoint: WL/DQS are called,
but their return values and the relevant wait return values are not evaluated.
Actual values, ERR050070 applicability, MPDGHWST read semantics and the
root cause remain open.
Clarification dated 2026-10-04: Status statements SPL-001 through SPL-007 and “today” refer to the source checkpoint of 2026-10-02. They are retained as a historical review state. Subsequent original-content checks and the local intake of RM Rev. 2 update individual source gaps without replacing earlier original reports. The actual calibration values, specific applicability of ERR050070, general absence of side effects from additional reads, and the root cause remain open.

## NXP/Freescale – Local Reference Manuals

Source intake: 2026-10-03.

The following two PDF files are available locally. Their copies were compared byte-for-byte with the files in `~/Downloads` (`PDF_IDENTITAET=PASS`).

| ID | Source | Local location | Status |
|---|---|---|---|
| NXP-RM2-1 | i.MX 6Dual/6Quad Reference Manual, Rev. 2, June 2014, Part 1, 3828 PDF pages | `research/nxp-reference-manuals/sources/38e11e86a1aade90ca272ce6a4d522c6fa6344b6f6c12e6767a46ed46451b0e7.pdf` | AVAILABLE LOCALLY; PDF metadata and SHA256 checked |
| NXP-RM2-2 | i.MX 6Dual/6Quad Reference Manual, Rev. 2, June 2014, Part 2, 1989 PDF pages | `research/nxp-reference-manuals/sources/IMX6DQRMr2_part2.pdf` | AVAILABLE LOCALLY; MMDC chapter 44 partially reviewed for content |

### Primary-Source Findings from Revision 2

- Section 44.11.3.1.2: hardware DQS gating and half-cycle correction.
- Section 44.12.7: `MDMISC` and supported channel operating modes.
- Section 44.12.61: `MPWLHWERR` and write-leveling result fields.
- Sections 44.12.62 et seq.: `MPDGHWST` status registers.

These statements concern Revision 2 from 2014. Revision 6 from 2020 and AN4467 Rev. 2 were still not available as locally verified original documents at this checkpoint.

The previously documented source gaps remain as historical research findings. This addendum updates their status for the register descriptions from Revision 2 that have since become available.

These sources do not yet establish the root cause of Novena cold-start failure 8B.

Full file metadata, SHA256 values, and provenance: `research/nxp-reference-manuals/README.md`.

## Source Clarifications – Documentation State as of 2026-10-04

None of the following statements denotes a new online check.

**NXP-RM2-DQS – SUBSTANTIATED:** Local NXP-RM2-2, §44.11.3.1.2, steps 31–35, printed p. 3871: completion indication, reading the upper MPDGHWST boundary field, and setting it to the upper boundary minus half a cycle. §44.12.62 et seq., printed pp. 3996 et seq., describe the MPDGHWST result fields. The prescribed result read is substantiated; the general absence of side effects from additional or repeated diagnostic reads remains OPEN.

**NXP-RM2-DECODER – SUBSTANTIATED with limitation:** §44.4.4.1, Tables 44-4/44-5, printed pp. 3838–3840: x16/x32 examples with eight banks, 15 row bits, and ten column bits. No substantiated actual x64 address mapping for the current Novena and no proof of aliasing.

**NXP-ERR050070 – SUBSTANTIATED as a documented earlier original-content check:** IMX6DQCE Rev. 7, 02/2019, p. 143, PDF index 142; inclusion of ERR050070 according to the revision table. Provenance: `research/05-gesamtanalyse/22-quellenprovenienz/01-NXP-ORIGINALQUELLEN.md` and `05-PROVENIENZMATRIX.md`, N01. The erratum describes the limited WL_HW_ERRn indication during automatic write leveling and `MPWLHWERR` as an alternative criterion for the byte lanes in use. No local original PDF or SHA256 of its bytes is available; specific applicability and triggering in case 8B remain OPEN.

**UB-HIST-DQS – SUBSTANTIATED within the documented source scope:** The qualified REF based on v2026.07 uses `0xc0`; its comment mentions `0x80`. The historical review draft of 2015-12-22 already contains this difference. The documented commit `cec2f200b4bfcc466e4a83196ed5fdd54678c1c7` changes two subtractions to `0x80`. Provenance and reviewed release boundaries: `research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md` and `22-quellenprovenienz/04-U-BOOT-HISTORISCHE-BELEGE.md`. This is neither proof of a Novena root cause nor evidence of a successful Novena comparison test.

**UB-HIST-ENABLE – SUBSTANTIATED with a gap:** Commit metadata and the documented graph association with v2016.03-rc2/v2016.03 are available for commit `89d485940106c095ec69ecb12a78a42e096dece7`. The complete historical parameter diff is missing because the blobs are unavailable. The older description “patch from 2015” must not be equated with a fully reviewed release artifact.

**KOSAGI-DDR – historical source scope:** `research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md` documents historical module, calibration, board-revision, and cold-start ECO reports together with their limitations. These reports identify neither the current DIMM nor the current ECO population and do not establish a cause for case 8B.

**SPL PROVENANCE:** The exact locked-debug REF path, base revision, and separate identities of ELF, raw SPL, wrapper, map, and configuration are recorded in the documentation checkpoint under `research/04-spl-ddr/`. The case-8B raw-log identity is separate. Reconstructed static qualification does not prove a historical analysis process for which no original report survives.

**BLOCK-8G – assignment-provided statement:** `STATIC_EVIDENCE_EXHAUSTED=YES` and `ROOT_CAUSE=UNRESOLVED` are carried forward according to assignment 9C-47W-8H. No original report for 9C-47W-8G has been established in the specifically examined sources. No original report has been reconstructed.

The statements now supported locally by RM Rev. 2 do not constitute an original-content check of RM Rev. 6 or AN4467 Rev. 2. Earlier open research questions remain dated historical entries.

## 2026-10-05 – BLOCK 9C-52C: Consolidated Historical Source State

This addendum adopts the user-confirmed consolidation state 9C-51A/9C-52B under assignment 9C-52C. Independent local original reports for 51A/52B were not found in the examined material. Statements extending beyond the local evidence cited below therefore explicitly have assignment provenance; they are not a new original-content or online check. No network research was carried out in this block.

`BELEGT` means a finding confirmed within the scope of its source; `STARK BELEGT` means a strong association without complete byte-level provenance; `QUELLENLUECKE` means required primary information is missing; `NICHT BELEGT` means a specific association has not been established. Retrieval status, original source, local copy, and inference remain separate. The supplementary CSV covers this consolidation addendum, not retrospectively every entry in older versions of the source register.

### Historical DDR Originals

| Source | Historical revision | Evidence basis and limitation |
|---|---|---|
| Novena ddr3 notes | oldid 621 | Documented original-content check in `research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md` and `22-quellenprovenienz/03-NOVENA-KOSAGI-ORIGINALQUELLEN.md`; no raw HTML copy archived there. |
| Novena PVT2 ECO List | oldid 1372 | Local original copy `research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.html`; current ECO population unknown. |
| U-boot PVT Notes | oldid 1755 | Local original copy `research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.html`; SPL calibration and trace-length relationship. |
| Novena Issue Log | oldid 826 | Local copy `research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.html`; oldid 826 independently substantiated locally by `wgRevisionId=826` and the Retrieved-from permalink. |
| Novena DVT Issue Log | oldid 944 | Documented original-content check in `22-quellenprovenienz/03-NOVENA-KOSAGI-ORIGINALQUELLEN.md`; historical burn-in, not current DIMM identity. |
| Novena PVT Issue Log | oldid 1057 | Local copy `research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.html`; no evidence of the current DDR cause. |
| System boot time messages | oldid 1832 | Historical source rediscovered according to the confirmed consolidation state 51A/52B; neither a local raw copy nor the full URL has been established here. |

**STRONGLY SUBSTANTIATED:** Historical Novena DDR provenance is strongly established; continuity of geometry from 2014 to 2026 is strongly supported (consolidation state 51A/52B). **SUBSTANTIATED:** Dynamic calibration for replaceable Novena SO-DIMMs is historically documented; for local software derivation, see `research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md`. This identifies neither the present DIMM nor its SPD or actual number of ranks. Historically successful DIMMs do not explain the current failure.

### Factory R1, Software, and Repository Generations

**SUBSTANTIATED within the confirmed historical source state:** Official name `novena-mmc-disk-r1.img`, historical URL `http://repo.novena.io/novena/images/novena-mmc-disk-r1.img`, SHA256 `26d368cb4b3aa43e411703f8c659d3e229deacfe75af38c1f82489dd9af80dbb`, MD5 `6923a145cbdc75b420408fc2d09ba4f8`, use as the shipped Novena image, and the historical shrink/expand concept. Name, URL, and hashes are already recorded in `docs/HISTORICAL-MEDIA.md`. The factory procedure using `dd`, recreating p3 to the end of the card, `fsck`, and `resize2fs`, as well as the signature bytes, are already contained in the local primary copy `research/01-original-novena-docs/sources/kosagi-novena-main-page.html`, “Disk Imaging” section. The OpenPGP signature packet timestamp `2014-12-22 07:35:33 UTC` can be derived from these bytes. The packet timestamp is not a verified release or creation date for the binary image.

**SOURCE GAP:** No hash-verified R1 binary copy was recovered; complete cryptographic verification of the signature, the exact original partitioning, and the exact release shrink step remain open. `FACTORY_R1_BINARY_RECOVERED=NO`. A negative search result does not justify `FACTORY_R1_IMAGE_LOST=YES`.

**STRONGLY SUBSTANTIATED according to the confirmed consolidation state:** Factory U-Boot as Debian version `2014.10-novena-rc12`; the strongest commit candidate is tag `v2014.10-novena-rc12`, `c5efeadb913c8246be372054adcd6106ad4fe067`. The tag association is documented locally in `research/02-historical-software/extracts/xobs-u-boot-novena-tag-history.txt` and `research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md`. `FACTORY_R1_EXACT_UBOOT_COMMIT=NICHT_BELEGT`: the candidate and the exact Factory-R1 commit remain distinct.

**STRONGLY SUBSTANTIATED according to the confirmed consolidation state:** Factory/shipped Novena kernel `3.17.0-rc5-00217-gfd79638`. Byte-exact attribution to the unrecovered R1 binary image remains open. Later kernel and bootloader updates are not retroactively attributed to R1.

Historically distinct: `repo.novena.io/repo/` is the earlier APT generation, `repo.novena.io/debian/` the later APT/package generation, and `repo.novena.io/novena/images/` the image-artifact path. The exact switchover date `/repo/` → `/debian/` and the complete signed historical Release/InRelease/Packages/Sources/pool chain remain open: `SIGNED_APT_INDEX_CHAIN=QUELLENLUECKE`.

**SUBSTANTIATED within the confirmed early source state:** `/repo/`, key identifier `03C7B7EC`, `kosagi-repo_1.0-r1`, `novena-eeprom_2.1-1`, and `novena-firstrun_1.4-r1`. Temporal proximity does not prove that precisely these package versions were included in the unrecovered R1 binary image.

### Historical Communication Sources

The following findings come from the confirmed state 51A/52B; corresponding local original reports or raw copies have not been established here, except for the channel/OFTC reference specified below. Full URLs are not added without supporting evidence.

- **SUBSTANTIATED:** Channel/OFTC reference in the local primary copy `research/01-original-novena-docs/sources/kosagi-novena-main-page.html`, “Potential contributors”: `irc.oftc.net #kosagi`. Conversation contents remain a **SOURCE GAP**. No public historical IRC log archive has yet been found; that is not proof of nonexistence.
- The Kosagi forum is a real technical source; known thread `viewtopic.php?id=421`, topic “Novena Firmware”.
- C3D2 documents a real Novena and refers to `factory-image`. Current possession of an R1 binary copy by C3D2 is **NOT SUBSTANTIATED**.
- 31C3 is a reliable time/event anchor for bunnie/xobs/fail0verflow, not proof of a factory image archived there.
- xobs “Novena First Run”, 2014-10-12, documents work on the final Novena disk image; “Factory test”, 2014-11-16, documents the factory phase.

These source findings do not resolve the current root cause and do not reopen the completed H3-1R/I2C3/ES8328 investigation.

## 2026-10-06 – BLOCK 9C-52E: Completed Historical Source Checkpoint

Consolidation of the completed read-only blocks 9C-52D, 9C-52D-1, and 9C-52D-2R according to their assignment-confirmed results, supplemented by read-only cross-checking of local primary-source locations. No new online check, root-cause investigation, or hardware observation. The classifications `BELEGT`, `STARK BELEGT`, `NEGATIVBEFUND`, `NICHT BELEGT`, `WIDERSPRUCH`, and `QUELLENLUECKE` remain distinct; better provenance does not raise an evidence classification.

### 9C-52D: Source Register Audit

`SOURCE_CSV_FULLY_REVIEWED=YES`, `SOURCE_CSV_ENRICHMENT_NEEDED=YES`, and `DOCUMENTATION_CORRECTION_NEEDED=YES` described the completed audit. The 13 identified CSV entries are given concrete local source locations and existing canonical URLs. oldid 826 is independently substantiated locally. The factory procedure and signature bytes have local primary evidence; the missing binary copy, complete signature verification, and release shrink remain separate source gaps. The OFTC association for `#kosagi` is substantiated locally; conversation contents and a public historical log archive remain open. The confirmed assignment provenance remains visible for the later `/debian/` APT generation; the local builder establishes `/repo/`, and the local main page establishes the image path.

### 9C-52D-1: xobs/novena-image

**SUBSTANTIATED:** Local historical primary source `/home/loomit/novena-historical-research/novena-image`, branch `master`, origin `https://github.com/xobs/novena-image.git`, qualified HEAD `f62c4f3b452199ab882e2c3dba16d4e31dfb1d56`. Complete local history: 36 commits; oldest commit `a9d5066d55f2cdbdef1589f2bd12ca1e176ac2bb` dated 2014-10-10. Commit `24a84d8c92e3583dffcbbe7dcc424dc94884e29f` dated 2014-10-15 introduces loopback creation with `loopback_size=3965190144` and `truncate "-s${imgsize}" "${imgname}"`. The logical total size of the builder image file is **SUBSTANTIATED**; the same size for the published `novena-mmc-disk-r1.img` is **NOT SUBSTANTIATED**.

**SUBSTANTIATED in the builder:** MBR, `fdisk -C32 -H32`, MMC signature `0x4e6f764d`, SATA signature `0x4e6f7653`, p1 FAT `/boot`, p2 swap, p3 ext4 root to the remaining end. The boot partition size evolves `32M -> 64M -> 32M`. **SOURCE GAP:** Exact Factory-R1 sector geometry; fdisk defaults, historical tooling, and the specific release run have not been fully qualified. **NEGATIVE FINDING:** No implementation of a release shrink step using `resize2fs`, `sfdisk`, or similar in the complete local history. This does not exclude an external release step. Repository `dd` is used for SPL or later sample checks; it does **NOT SUBSTANTIATE** a release shrink procedure.

**SUBSTANTIATED:** `8b64953` dated 2014-11-04 references the following, among others, in `mmc-install.sh`:

```text
u-boot-novena_2014.10-novena-rc12_armhf.deb
linux-headers-novena_1.14-r1_armhf.deb
linux-image-novena_1.14-r1_armhf.deb
novena-disable-ssp_1.1-1_armhf.deb
novena-eeprom_2.1-1_armhf.deb
novena-eeprom-gui_1.2-r1_armhf.deb
kosagi-repo_1.0-r1_all.deb
novena-firstrun_1.4-r1_all.deb
xorg-novena_1.2-r1_all.deb
kosagi.key
```

This improves package chronology but does not prove the contents of the published Factory-R1 image or establish a complete R1 package manifest. The side-branch commits `92ba999`, `444744c`, `4baf00a`, and `53e2521` carry December 2014 commit dates but were merged into master only in December 2015 via `29d5ed5` / `f62c4f3`. Today's master is not evidence of the master state in December 2014.

```text
FACTORY_R1_RELEASE_BUILDER_IDENTIFIED=PARTIAL
FACTORY_R1_RELEASE_SHRINK_PROCEDURE_IDENTIFIED=NO
FACTORY_R1_EXACT_IMAGE_SIZE_EXPLAINED=NO
FACTORY_R1_EXACT_PARTITION_GEOMETRY_EXPLAINED=NO
```

### 9C-52D-2R: OpenBSD

**SUBSTANTIATED:** Independent historical source `/home/loomit/novena-historical-research/openbsd-src`. Initial support: `31642defb5b3233700d202713f8a65d872d823c3`, `2015-05-08T03:38:26Z`, “Add initial board specific parts of novena support, tested by djm@”. The commit and subsequent commits document contemporary tests on real Novena hardware and continued operation; for example, `05d12a02f11` (“committed from Novena”). `a5b7bc8b7df` reports 4 GB physical RAM and U-Boot values `memstart: 0x10000000`, `memsize: 0xf0000000`. The examined path (`sys/arch/armv7/armv7/armv7.c`, `sys/arch/armv7/imx/imx_machdep.c`) adopts bootloader-initialized RAM, initially uses machid `0x10ad` / 4269, and relies on the bootloader RAM description. Dependency on U-Boot is explicit. **NEGATIVE FINDING:** No Novena-specific DDR initialization in this examined OpenBSD path.

**SUBSTANTIATED:** Tested SDHC support, including `2852902afc7` (“Tested by djm”). **NEGATIVE FINDING:** No complete local boot log found. **SOURCE GAP:** Complete exact SD boot sequence. The independent evidence of real operation/RAM proves neither identical DIMM/SPD/DDR parameters nor an identical U-Boot revision or boot path to Factory R1 or the current NixOS/Patch0010 test.

### 9C-52D-2R: gentoo-on-novena and novena-overlay

**SUBSTANTIATED:** `/home/loomit/novena-historical-research/gentoo-on-novena`, HEAD `f49fcab518c5bb4b2fb0e90baecf8b504209bfff`, `https://github.com/sakaki-/gentoo-on-novena`. README and release history document bootable complete Novena microSD images (libre and standard), v1.0.0/v1.0.1, v1.0.1 approximately 770/797 MiB compressed and 6 GiB unpacked, p1 vfat `/boot`, p2 swap, p3 ext4 `/`, Linux 4.7.2, and `imx6q-novena.dtb` as `/boot/novena.dtb`. A real board/desktop test report exists; **NEGATIVE FINDING:** No complete local serial boot log. U-Boot package: `dev-embedded/u-boot-novena-2014.10.8`; Factory-R1 attribution **NOT SUBSTANTIATED**. The relationship to xobs is explicit through the overlay. This is further later independent operational evidence, not independent DDR initialization.

**SUBSTANTIATED:** `/home/loomit/novena-historical-research/novena-overlay`, HEAD `8f88536ebcd4fc107b75a97006a8447628d08638`, complete history of 30 commits, `https://github.com/sakaki-/novena-overlay`. The README describes a port of the then-current xobs Novena Debian packages. The ebuild `dev-embedded/u-boot-novena/u-boot-novena-2014.10.8.ebuild` maps `2014.10.8 -> v2014.10.r8-novena` from `xobs/u-boot-novena`; the declared source identity can be traced locally in the separate U-Boot repository: `b98333336941440e2792715853cbe0211577e0d7`. This does not retroactively cryptographically verify an originally downloaded release archive. Factory-R1 attribution **NOT SUBSTANTIATED**. The README documents sakaki's port of xobs's Linux 4.4 patches to Linux 4.7.2. The explicit xobs package relationship does not establish independent Factory-R1 provenance.

### Cross-Comparison and Open Limitations

```text
NEW_INDEPENDENT_NOVENA_BOOT_EVIDENCE=YES
NEW_INDEPENDENT_RAM_EVIDENCE=YES
INDEPENDENT_DDR_INITIALIZATION_EVIDENCE=NO
```

Historical counterevidence: independent later systems document successful operation of real Novena hardware with multiple GiB of RAM. This weighs against the blanket assertion that Novena fundamentally cannot work above a small DDR region. It does not explain the current C1/Q1 failure and does not establish identity of DIMM, SPD, DDR configuration, U-Boot, or the physical memory path. Neither a software nor a hardware cause of the current failure follows from this. The Factory-R1 binary copy, complete cryptographic verification, exact U-Boot commit, package manifest, partition geometry, release shrink, signed APT index chain, and Factory DIMM/SPD identity remain open; current status in `PROJECT-STATE.md`. I2C3/ES8328 remains closed; `52Q=WAIT`.
