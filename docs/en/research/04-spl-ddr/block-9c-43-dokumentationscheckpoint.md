# BLOCK 9C-43 – Documentation checkpoint from BLOCK 9C-44

Status: 2026-10-02. Created under the documentation assignment BLOCK 9C-44.
The filename was specified by the assignment; this file is not an original BLOCK 9C-43 report discovered retrospectively.

## Evidence classes and missing history

- **Original evidence:** the preserved reference ELF, its associated build tree containing sources, configuration, map and SPL, and the unchanged raw serial log for case 8B. These establish their respective bytes and static properties, or the recorded output.
- **Current reconstruction:** a local comparison of these artifacts with source code, symbol table and disassembly performed on 2026-10-02. It does not establish the historical analysis procedure or constitute a new hardware run.
- **Chat report / assignment statement:** the assignment of the topics to be documented to BLOCK 9C-42 and 9C-43 comes from the current assignment. Their complete chat reports are not available to this checkpoint as original sources.
- **Hypothesis:** any causal connection between calibration errors, timeouts, ERR050070, BSS clearing or another memory mechanism and case 8B remains unproven.

**The original repository reports for BLOCK 9C-33 through BLOCK 9C-42, inclusive, are missing from the examined repository contents.** A separate original 9C-43 report was also not found there before this checkpoint. No tests, approvals, command sequences or earlier results are reconstructed from block numbers. This file does not fill that evidentiary gap. The previously documented missing original output of the 4.10O-1L-3C/3C-R1 gate sequence also remains a gap.

## Qualified reference artifact

Local reference tree (hereafter `REF`):

```text
/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree
```

The existing project documentation associates this locked-debug version with the patch-0008 SPL. The present raw SHA256 comparison confirms:

| File relative to REF | SHA256 |
|---|---|
| spl/u-boot-spl | 69bb3a4dd2efe059cefc197cad244f13459b0ef08ccb17c886059337ca25e469 |
| SPL | 4611111bf2412257bf537a0c3b95a79b796d7359a8ff4de54046cbe2fe56a25c |
| spl/u-boot-spl.bin | 42b47045d5aa9cde5e7ec04c6579ececed596cd36643961f7e4ae66f96c307f3 |
| spl/u-boot-spl.map | 0fcf601af9d23c6997bd1cbd944fa56f580ed86dc34099fda89ce0ce26d1bf54 |
| .config | 48f2371872392021f1caaba6590fc7c4344b115ee117ca1386925f427da6eddb |

Raw SPL size: 49,480 bytes; wrapper size: 56,320 bytes. The current byte-level check confirms exactly one embedded raw SPL at offset `0xc00`. ELF, raw SPL and wrapper are distinct artifacts; the ELF is not the file to be written to a storage medium. Today's static identity check is neither a new build nor renewed media or boot qualification.

The source base is the preserved patched build tree for the U-Boot `v2026.07` revision specified in the project documentation (`ece349ade2973e220f524ce59e59711cc919263f`), with project patches 0001–0008. The version string alone does not establish this patch series.

## Original raw log and case 8B

Original file:

`test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`

830 bytes; SHA256:

`9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`

The complete raw log was read; it contains C0=C1=C2=Q1=Q2=P0=P1=0. The documented validity condition C0=C2=0 is therefore satisfied, and the unchanged preregistered classification is `8B = CONTROL_FAIL_TARGET_FAIL`.
D21A contains `start=18300000 end=18400000 brk=00000000`; D32 and D33 contain `ret=-12`. The preserved output ends after D33. `priv=?` is not a determinable pointer value. The literal `\n` in the Q1/Q2 output remains part of the original.

The classification as a single POR/P_EXT run and the boot count come from the existing test documentation and the current assignment specification; the 830 raw log bytes alone establish neither power disconnection nor the number of all starts. The preceding C0 load may precondition the state. Two nearby control/target addresses do not prove a general DDR defect.

## Calibration findings reconstructed now

Locations relative to REF:

- `board/kosagi/novena/novena_spl.c:579–584`: DDR configuration, `udelay(100)`, then WL and DQS. Neither integer return value is checked.
- `arch/arm/mach-imx/mx6/ddr.c`: WL starts at line 106, DQS at line 296; `include/wait_bit.h`: polling and return-value contract.

WL returns an error bitmask: bit values 1/2 for PHY0/PHY1 and 4 for the soft-fail/restore case. DQS accumulates 1/2 for gating, 4/8 for read delay, and 16/32 for write delay. These error checks exist, but their return values are ignored by the board path. Consequently, continuation of the SPL does not imply that these masks were zero. The original case-8B raw log does not record these values.

The relevant `wait_for_bit_le32()` calls in ddr.c (lines 25, 28, 43, 48, 170, 343, 377, 424, 488, 542 and 584) are likewise executed without checking their return values. They concern FIFO reset, precharge/CON_ACK, WL completion, dummy write, gating, read/write delay and the final CON_ACK. The two FIFO wait calls and the precharge helper may be invoked multiple times; the number of source locations is not a runtime invocation count.

The return-value contract is 0 on success, `-ETIMEDOUT` on timeout and `-EINTR` on an enabled Ctrl-C abort. Here, 100 ms and `breakable=0` are specified; the timeout condition is `get_timer(start) > 100`. Timeouts do not contribute separate bits to the error bitmasks above. Thus even a hypothetical return mask of 0 would not prove that the wait conditions succeeded.

In the reference machine code, for example, WL at `0x009099d6` and DQS gating at `0x00909cd2` route both timeout and satisfied-condition cases into the same continuation path. This confirms that the built artifact does not evaluate the timeout. It does not prove that a timeout actually occurred.

### ERR050070 and MPDGHWST: remaining source limitations

ERR050070 is documented, as specified by the assignment, as an erratum candidate requiring examination. No locally accessible original manufacturer source specifying the revision, affected silicon revisions, conditions and workaround was found in the examined project/Novena sources. Accordingly, neither its precise content nor its applicability or causal role in case 8B is claimed to be verified here. An earlier chat finding on the matter cannot be independently reconstructed without the original report and primary source. No download or online verification was performed.

`modify_dg_result()` reads MPDGHWST0–3 for both PHYs in the x64 path (`ddr.c:62–86,447–455`) and processes the upper boundary fields. The implemented calculation subtracts `0xc0`; the comment says `HW_DG_UPx - 0x80`. The comment and implemented calculation are not treated as equivalent; no fix is inferred without primary register/erratum evidence.

MPDGHWST read semantics remain **UNKNOWN**: the source code establishes that reads exist, but does not establish that they are generally free of side effects, that read-to-clear behavior is guaranteed, or that additional snapshot reads can safely be repeated. In particular, a new read is not automatically a passive observation. Any connection to ERR050070 remains open.

## SRAM stack, BSS clearing and DQS tail branch

The reference configuration contains `CONFIG_SPL_STACK=0x91ffb8`, but neither `CONFIG_SPL_STACK_R` nor `CONFIG_SPL_EARLY_BSS`. `_main` sets SP from `0x0091ffb8`, reserves GD/early malloc space and calls `board_init_f()` before clearing the DDR BSS. The early stack is therefore in SRAM/OCRAM, not in the DDR BSS or full-malloc interval. This does not establish an arbitrary amount of available stack space for future instrumentation.

The machine-code sequence is:

```text
0x0090b0b8: call board_init_f
0x0090b0cc: BSS memset [0x18200000,0x1820015c)
0x0090b0d0: call spl_relocate_stack_gd (returns 0 here)
then: transition to board_init_r
```

The clear includes `max_total_mem=0x18200030` and `mem_malloc_brk=0x1820003c`. It issues ordinary CPU zero stores; this alone does not establish their correct physical persistence. The later heap clear `[0x18300000,0x18400000)` is a different operation. BSS clearing as a cause of case 8B remains a hypothesis.

`board_init_f` calls WL at `0x0090b6a6`, then overwrites r0 with the DQS argument, restores its stack frame and branches at `0x0090b6b2` via `b.w` to `mmdc_do_dqs_calibration` (`0x00909b30`). This is a DQS tail branch, not a missing DQS call. The DQS return value reaches the original caller; `_main` then replaces r0 with the BSS address. A future post-DQS measurement point would alter this machine-code sequence and would have to be qualified again.

## Hard-coded patch-0008 address and diagnostic concepts A/B

`boot/u-boot/0008-novena-diag-control-c0-c2.patch` explicitly uses `(volatile ulong *)0x18200030UL`, not an automatic binding to the `max_total_mem` symbol. In the reference ELF, this address matches the symbol. New global/BSS diagnostic objects or a changed link layout could alter that association. Any subsequent variant would have to requalify the control/target addresses, instruction sequence and BSS boundaries; the earlier qualification is not transferable.

The following architecture definitions are **statements from the earlier chat report concerning BLOCK 9C-42, as specified by assignment BLOCK 9C-49**. They are not a locally verified original repository source: the original 9C-42 report has not been established in the local repository. No historical preregistration is inferred from them.

| Variant | Architecture according to the earlier chat report |
|---|---|
| A | Locally initialized diagnostic structure on the SRAM stack, with parameters passed to WL/DQS helpers. |
| B | Initialized global diagnostic state in SRAM `.data`, with unchanged function signatures. |

The optional measurement scope is separate from these architectural variants: existing WL/DQS return values and wait results can distinguish reported calibration errors from timeouts; return masks alone are insufficient. Output after the respective measurement window and additional evaluation affect code, stack and timing. Phase-related MMDC/calibration values are an optional measurement scope, not a definition of B as an extension of A. Reusing existing register-read values wherever possible belongs to this measurement scope; additional MMIO reads require established read semantics. MPDGHWST snapshots are not qualified until this gap is resolved.

Neither concept has been implemented, built, statically qualified or authorized for hardware use. Automatic variables before BSS clearing must not be written to BSS that will subsequently be cleared if they are intended to persist. Even on error values, diagnostics must not introduce an unpreregistered fallback, retry, calibration change or functional fix. A positive diagnostic result would still not constitute proof of the root cause.

## Source identity and resumption

Source hashes of the files used today, relative to REF:

| File | SHA256 |
|---|---|
| board/kosagi/novena/novena_spl.c | ef3252c640f2b4c2dbc52b1c3b0c4fe0f0056a4cb95033e4839109e63b09474f |
| arch/arm/mach-imx/mx6/ddr.c | 2c2c41d7c0c3ce07f73f4e20f405b17b0cd8bbda37505be124e597d696182c95 |
| include/wait_bit.h | 66b2f5fb47c01fcabda3d1327f4cdb352fe18809b418417e51b8da0febc419c9 |
| arch/arm/lib/crt0.S | f43a2ba514cf81810c4e05e2372cc25808f8d24431b8bfa17f51d6f30b2542d6 |

Symbol/machine-code examination used local `arm-linux-gnueabihf-nm` and `arm-linux-gnueabihf-objdump`; hash/byte verification produced no new artifacts. Also authoritative: README.md, AGENTS.md, docs/PROJECT-STATE.md, docs/TEST-LOG.md, docs/DECISIONS.md, docs/QUELLENVERZEICHNIS.md and the unchanged project patches. Historical Novena sources remain comparison material, not qualification of the current artifact.

Still outstanding are the original reports and local verification of the A/B mapping stated in the earlier chat report, the primary source for ERR050070 and its specific applicability, MPDGHWST read semantics, actual WL/DQS/timeout values and the causative memory mechanism. The only permissible next step within this assignment is documentation review. Further implementation, building, media access or hardware testing require a new specific assignment.

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_HARDWARE_BOOT_COUNT=1
PATCH_0008_SECOND_BOOT_ALLOWED=NO
```
