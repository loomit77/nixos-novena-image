## 2026-10-01 – BLOCK 4.10O-1N: Patch-0008-Hardwaretest, Fall 8B

### Ausgangslage

Nach dem einmaligen Hardwarebefund Fall 7C wurde in Block 4.10O
ein adressvergleichender Kontrolltest präregistriert.

Der Test vergleicht:

- Kontrollvariable: `max_total_mem`
- Kontrolladresse: `0x18200030`
- Zielvariable: `mem_malloc_brk`
- Zieladresse: `0x1820003c`
- Testwert: `0x18300000`

Patch `0008-novena-diag-control-c0-c2.patch` wurde vor dem
Hardwaretest statisch und auf Maschinencodeebene qualifiziert.

Das vollständige NixOS-SD-Image wurde vor dem Test gebaut,
auf das Testmedium geschrieben und zurückverifiziert.

### Hardwaredurchführung

Datum: 2026-10-01

Testgerät: Novena i.MX6 Quad

Testart: Einmaliger Kaltstart/POR mit Patch 0008

Serielle Verbindung: 115200 Baud, 8N1, ohne Flusskontrolle

Hardwarestarts mit diesem präregistrierten Test: 1

Ein zweiter Hardwarestart wurde nicht durchgeführt.

### Originalprotokoll

Pfad:

`test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`

Dateigröße: 830 Bytes

SHA-256:

`9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`

### Beobachtete Diagnosewerte

C0 = 0x00000000
C1 = 0x00000000
C2 = 0x00000000

Q1 = 0x00000000
Q2 = 0x00000000

P0 = 0x00000000
P1 = 0x00000000

Weitere relevante Beobachtungen:

- SPL meldet `Trying to boot from MMC1`.
- D21A zeigt `start=18300000`, `end=18400000` und `brk=00000000`.
- D21B zeigt `priv=?` und `brk=00000000`.
- D32 meldet `ret=-12`.
- D33 meldet `ret=-12`.
- Danach enthält das gesicherte Protokoll keine weitere Bootausgabe.

Der Text `priv=?` wird nicht als konkreter Zeigerwert interpretiert.

Die Q1/Q2-Diagnose enthält weiterhin ein literales `\n`.
Dies ist eine bekannte Eigenschaft des Diagnosepatches.

### Präregistrierte Klassifikation

Das Gültigkeitskriterium ist erfüllt:

C0 = 0x00000000
C2 = 0x00000000

Der Kontroll-Load C1 beobachtet nicht den geschriebenen
Testwert, sondern Null.

Auch der Ziel-Load Q1 beobachtet Null.

Damit gilt:

RESULT=8B
RESULT_NAME=CONTROL_FAIL_TARGET_FAIL
TEST_VALID=YES

### Aussagegrenzen

Der Hardwarebefund zeigt, dass das beobachtete Verhalten
nicht ausschließlich auf den untersuchten Target-Load von
`mem_malloc_brk` begrenzt ist.

Der Befund identifiziert jedoch keinen verursachenden
Speichermechanismus.

Insbesondere sind nicht bewiesen:

- ein allgemeiner DDR-Defekt;
- eine bestimmte physische Bank-/Row-/Column-Zuordnung;
- ein MMDC-Fehler;
- ein Cache- oder Barrier-Mechanismus;
- eine durch den BSS-Clear verursachte Störung.

Die mögliche Vorkonditionierung durch den C0-Load bleibt
eine dokumentierte Einschränkung.

### Abschlussstatus

PATCH_0008_HARDWARE_BOOT_COUNT=1
PATCH_0008_SECOND_BOOT_ALLOWED=NO
HARDWARE_TEST_RESULT=8B
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO

Vor weiteren Hardwaretests, Patches oder Builds erfolgt
ein erneuter Abgleich mit der vorhandenen Projekt- und
Primärquellendokumentation.
