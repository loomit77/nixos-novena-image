### BLOCK 4.10O-1N – Hardwareergebnis Fall 8B (2026-10-01)

Der einmalige, zuvor präregistrierte Hardwaretest mit U-Boot-Patch 0008 ist abgeschlossen.

Kontrolladresse: `max_total_mem = 0x18200030`
Zieladresse: `mem_malloc_brk = 0x1820003c`
Testwert: `0x18300000`

Gemessene Werte: C0=00000000, C1=00000000, C2=00000000, Q1=00000000, Q2=00000000, P0=00000000, P1=00000000.

Die Gültigkeitsbedingung C0=C2=0 ist erfüllt.
Präregistriertes Ergebnis: `8B = CONTROL_FAIL_TARGET_FAIL`.

Originalprotokoll: `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`
Protokollgröße: 830 Bytes.
SHA-256: `9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`

Der Bootpfad erreicht die MMC-Initialisierung und meldet `ret=-12`.
Die Root Cause des beobachteten Speicherverhaltens bleibt ungeklärt.

Nächster Schritt: Read-only-Abgleich mit vorhandenen Projektergebnissen und Primärquellen vor weiteren Tests, Builds oder Patches.

PATCH_0008_HARDWARE_BOOT_COUNT=1
PATCH_0008_SECOND_BOOT_ALLOWED=NO
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO

Einschraenkung: Der vorausgehende C0-Load kann den beobachteten Speicherzustand vorkonditioniert haben. Aus Fall 8B folgt deshalb kein Nachweis eines bestimmten DDR-, MMDC-, Cache- oder sonstigen Speichermechanismus.
