# BLOCK 9C-43 – Dokumentationscheckpoint aus BLOCK 9C-44

Stand: 2026-10-02. Erstellt im Dokumentationsauftrag BLOCK 9C-44.
Der Dateiname ist vom Auftrag vorgegeben; diese Datei ist kein nachträglich
aufgefundenes Originalprotokoll von BLOCK 9C-43.

## Evidenzklassen und fehlende Historie

- **Originalbelege:** erhaltenes Referenz-ELF, zugehöriger Buildbaum mit
  Quellen, Konfiguration, Map und SPL sowie das unveränderte serielle
  Fall-8B-Rohlog. Sie belegen ihre jeweiligen Bytes und statischen Eigenschaften
  beziehungsweise die aufgezeichneten Ausgaben.
- **Aktuelle Rekonstruktion:** am 2026-10-02 erfolgter lokaler Abgleich dieser
  Artefakte mit Quellcode, Symboltabelle und Disassemblierung. Er belegt keinen
  damaligen Analyseablauf und keinen neuen Hardwarelauf.
- **Chatbericht/Auftragsangabe:** die Zuordnung der zu dokumentierenden Themen
  zu BLOCK 9C-42 und 9C-43 stammt aus dem aktuellen Auftrag. Deren vollständige
  Chatberichte liegen diesem Checkpoint nicht als Originalquelle vor.
- **Hypothese:** eine ursächliche Verbindung von Kalibrierungsfehlern, Timeouts,
  ERR050070, BSS-Clear oder einem anderen Speichermechanismus zu Fall 8B ist
  weiterhin unbewiesen.

**Die Repository-Originalberichte BLOCK 9C-33 bis einschließlich 9C-42 fehlen
im geprüften Repositorybestand.** Auch ein eigener Originalbericht 9C-43
wurde dort vor diesem Checkpoint nicht gefunden. Aus den Blocknummern werden
keine Tests, Freigaben, Befehlsfolgen oder früheren Ergebnisse rekonstruiert.
Diese Datei ersetzt diese Lücke nicht. Die bereits dokumentierte fehlende
Originalausgabe der 4.10O-1L-3C/3C-R1-Gate-Abfolge bleibt ebenfalls eine Lücke.

## Qualifiziertes Referenzartefakt

Lokaler Referenzbaum (im Folgenden `REF`):

```text
/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree
```

Die vorhandene Projektdokumentation ordnet diesen Locked-Debug-Stand dem
Patch-0008-SPL zu. Der heutige rohe SHA256-Abgleich bestätigt:

| Datei relativ zu REF | SHA256 |
|---|---|
| spl/u-boot-spl | 69bb3a4dd2efe059cefc197cad244f13459b0ef08ccb17c886059337ca25e469 |
| SPL | 4611111bf2412257bf537a0c3b95a79b796d7359a8ff4de54046cbe2fe56a25c |
| spl/u-boot-spl.bin | 42b47045d5aa9cde5e7ec04c6579ececed596cd36643961f7e4ae66f96c307f3 |
| spl/u-boot-spl.map | 0fcf601af9d23c6997bd1cbd944fa56f580ed86dc34099fda89ce0ce26d1bf54 |
| .config | 48f2371872392021f1caaba6590fc7c4344b115ee117ca1386925f427da6eddb |

Raw-SPL-Größe: 49.480 Byte; Wrappergröße: 56.320 Byte. Die aktuelle
Byteprüfung bestätigt genau ein eingebettetes Raw-SPL bei Offset `0xc00`.
ELF/Raw-SPL/Wrapper sind unterschiedliche Artefakte; das ELF ist nicht die
auf ein Medium zu schreibende Datei. Die heutige statische Identitätsprüfung
ist weder ein neuer Build noch eine erneute Medien- oder Bootqualifikation.

Die Quellbasis ist der erhaltene gepatchte Buildbaum des in den
Projektdokumenten festgelegten U-Boot-`v2026.07`-Stands
(`ece349ade2973e220f524ce59e59711cc919263f`) mit Projektpatches 0001–0008.
Die Versionszeile allein beweist diese Patchkette nicht.

## Originalrohlog und Fall 8B

Originaldatei:

`test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`

830 Byte; SHA256:

`9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`

Das vollständige Rohlog wurde gelesen; es enthält C0=C1=C2=Q1=Q2=P0=P1=0.
Damit ist die dokumentierte Gültigkeitsbedingung C0=C2=0 erfüllt und die
unveränderte Präregistrierung ergibt
`8B = CONTROL_FAIL_TARGET_FAIL`.
D21A enthält `start=18300000 end=18400000 brk=00000000`;
D32 und D33 enthalten `ret=-12`. Nach D33 endet die erhaltene Ausgabe.
`priv=?` ist kein bestimmbarer Zeigerwert. Das literale `\n` in der
Q1/Q2-Ausgabe bleibt Bestandteil des Originals.

Die Einordnung als einmaliger POR/P_EXT-Lauf und der Bootzähler stammen aus
der vorhandenen Testdokumentation und der aktuellen Auftragsvorgabe; die
830 Rohlogbytes allein beweisen weder Stromtrennung noch Anzahl aller Starts.
Der vorausgehende C0-Load kann den Zustand vorkonditionieren.
Zwei nahe Kontroll-/Targetadressen beweisen keinen allgemeinen DDR-Defekt.

## Aktuell rekonstruierte Kalibrierungsbefunde

Fundstellen relativ zu REF:

- `board/kosagi/novena/novena_spl.c:579–584`: DDR-Konfiguration,
  `udelay(100)`, dann WL und DQS. Beide int-Rückgaben werden nicht ausgewertet.
- `arch/arm/mach-imx/mx6/ddr.c`: WL ab Zeile 106, DQS ab Zeile 296;
  `include/wait_bit.h`: Polling und Rückgabevertrag.

WL liefert eine Fehlerbitmaske: Bitwerte 1/2 für PHY0/PHY1 und 4 für den
Soft-Fail-/Restore-Fall. DQS akkumuliert 1/2 für Gating, 4/8 für Read Delay
und 16/32 für Write Delay. Diese Fehlerprüfungen sind vorhanden; ihre
Rückgaben werden vom Boardpfad ignoriert. Aus weiterlaufendem SPL folgt
deshalb nicht, dass diese Masken Null waren. Im Fall-8B-Originalrohlog
werden diese Werte nicht aufgezeichnet.

Die einschlägigen `wait_for_bit_le32()`-Aufrufe in ddr.c (Zeilen
25, 28, 43, 48, 170, 343, 377, 424, 488, 542 und 584) werden ebenfalls
ohne Rückgabeprüfung ausgeführt. Sie betreffen FIFO-Reset, Precharge/CON_ACK,
WL-Abschluss, Dummy-Write, Gating sowie Read-/Write-Delay und abschließendes
CON_ACK. Die beiden FIFO-Warteaufrufe und die Precharge-Hilfe können
mehrfach aufgerufen werden; Quellstellenzahl ist keine Laufzeitanzahl.

Der Vertrag ist 0 bei Erfolg, `-ETIMEDOUT` bei Timeout und
`-EINTR` bei freigegebenem CTRL-C-Abbruch. Hier sind 100 ms und
`breakable=0` angegeben; die Timeoutbedingung ist `get_timer(start) > 100`.
Timeouts gehen nicht als eigene Bits in die obigen Fehlerbitmasken ein.
Auch eine hypothetische Rückgabemaske 0 wäre deshalb kein Beweis für
erfolgreiche Wartebedingungen.

Im Referenzmaschinencode führen etwa WL bei `0x009099d6` und DQS-Gating
bei `0x00909cd2` sowohl Timeout als auch erfüllte Bedingung in denselben
Fortsetzungspfad. Dies bestätigt die fehlende Timeoutauswertung im gebauten
Artefakt. Es beweist keinen tatsächlich eingetretenen Timeout.

### ERR050070 und MPDGHWST: verbleibende Quellengrenzen

ERR050070 wird gemäß Auftrag als zu prüfender Erratum-Kandidat dokumentiert.
Ein lokal zugänglicher Hersteller-Originalbeleg mit Revision, betroffenen
Siliconständen, Bedingungen und Workaround wurde in den geprüften
Projekt-/Novena-Quellen nicht gefunden. Deshalb werden hier weder sein
genauer Inhalt noch seine Anwendbarkeit oder eine Ursache für Fall 8B
als verifiziert behauptet. Ein früherer Chatbefund dazu bleibt ohne
Originalbericht und Primärquelle nicht unabhängig nachvollziehbar.
Es erfolgte kein Download und keine Onlineprüfung.

`modify_dg_result()` liest MPDGHWST0–3 beider PHYs im x64-Pfad
(`ddr.c:62–86,447–455`) und verarbeitet die oberen Grenzfelder.
Die implementierte Rechnung subtrahiert `0xc0`; der Kommentar nennt
`HW_DG_UPx - 0x80`. Kommentar und implementierte Rechnung werden nicht
gleichgesetzt; daraus wird ohne Register-/Erratum-Primärbeleg kein Fix abgeleitet.

MPDGHWST-Lesesemantik bleibt **UNKNOWN**: Der Quellcode belegt vorhandene
Lesungen, aber keine allgemeine Nebenwirkungsfreiheit, kein garantiertes
Read-to-clear-Verhalten und keine sichere Wiederholbarkeit zusätzlicher
Snapshot-Lesungen. Insbesondere ist eine neue Lesung keine automatisch
passive Beobachtung. Ein Zusammenhang mit ERR050070 bleibt offen.

## SRAM-Stack, BSS-Clear und DQS-Tail-Branch

Die Referenzkonfiguration enthält `CONFIG_SPL_STACK=0x91ffb8`,
kein `CONFIG_SPL_STACK_R` und kein `CONFIG_SPL_EARLY_BSS`.
`_main` setzt SP aus `0x0091ffb8`, reserviert GD/frühen Malloc-Bereich
und ruft `board_init_f()` vor dem DDR-BSS-Clear auf.
Der frühe Stack liegt damit im SRAM/OCRAM, nicht im DDR-BSS- oder
Full-Malloc-Intervall. Das ist kein Nachweis beliebiger Stackreserve für
eine künftige Instrumentierung.

Die Maschinenfolge lautet:

```text
0x0090b0b8: Aufruf board_init_f
0x0090b0cc: BSS-memset [0x18200000,0x1820015c)
0x0090b0d0: Aufruf spl_relocate_stack_gd (liefert hier 0)
danach: Übergang board_init_r
```

Der Clear umfasst `max_total_mem=0x18200030` und
`mem_malloc_brk=0x1820003c`. Er gibt gewöhnliche CPU-Null-Stores aus;
deren korrekte physische Persistenz wird dadurch nicht bewiesen.
Der spätere Heap-Clear `[0x18300000,0x18400000)` ist ein anderer Vorgang.
Ein BSS-Clear als Ursache von Fall 8B bleibt Hypothese.

`board_init_f` ruft WL bei `0x0090b6a6` auf, überschreibt danach r0 mit
dem DQS-Argument, restauriert seinen Frame und springt bei `0x0090b6b2`
per `b.w` nach `mmdc_do_dqs_calibration` (`0x00909b30`).
Das ist ein DQS-Tail-Branch, kein fehlender DQS-Aufruf. Die DQS-Rückgabe
gelangt zum ursprünglichen Aufrufer; `_main` ersetzt r0 anschließend durch
die BSS-Adresse. Ein künftig eingefügter Nach-DQS-Messpunkt würde diese
Maschinenfolge ändern und müsste erneut qualifiziert werden.

## Hartcodierte Patch-0008-Adresse und Diagnosekonzepte A/B

`boot/u-boot/0008-novena-diag-control-c0-c2.patch` verwendet ausdrücklich
`(volatile ulong *)0x18200030UL`, keine automatische Bindung an das
Symbol `max_total_mem`. Im Referenz-ELF stimmt diese Adresse mit dem
Symbol überein. Neue globale/BSS-Diagnoseobjekte oder ein geändertes
Linklayout könnten diese Zuordnung verändern. Jede spätere Variante
müsste die Control-/Targetadressen, Instruktionsfolge und BSS-Grenzen
neu qualifizieren; die alte Qualifikation ist nicht übertragbar.

Die folgenden Architekturdefinitionen sind **Angaben aus dem früheren
Chatbericht zu BLOCK 9C-42 gemäß Auftrag BLOCK 9C-49**. Sie sind keine lokal
verifizierte Repository-Originalquelle: Der Originalbericht 9C-42 ist im
lokalen Repository nicht nachgewiesen. Daraus wird keine historische
Präregistrierung abgeleitet.

| Variante | Architektur laut früherem Chatbericht |
|---|---|
| A | Lokal initialisierte Diagnose-Struktur auf dem SRAM-Stack mit Parameterweitergabe an WL-/DQS-Helfer. |
| B | Initialisierter globaler Diagnosezustand in SRAM-`.data` bei unveränderten Funktionssignaturen. |

Der optionale Messumfang ist von diesen Architekturvarianten getrennt:
Vorhandene WL-/DQS-Rückgaben und Warteergebnisse können gemeldete
Kalibrierungsfehler und Timeouts unterscheiden; Rückgabemasken allein
reichen nicht. Ausgabe nach dem jeweiligen Messfenster und zusätzliche
Auswertung beeinflussen Code, Stack und Timing. Phasenbezogene
MMDC-/Kalibrierungswerte sind ein optionaler Messumfang, keine Definition
von B als Erweiterung von A. Vorhandene Registerlesewerte möglichst
wiederzuverwenden gehört zu diesem Messumfang; zusätzliche MMIO-Lesungen
verlangen belegte Lesesemantik. MPDGHWST-Snapshots sind vor Klärung dieser
Lücke nicht qualifiziert.

Beide Konzepte sind weder implementiert noch gebaut, statisch qualifiziert
oder hardwarefreigegeben. Automatische Werte vor dem BSS-Clear dürften
nicht zur dauerhaften Erhaltung in später gelöschte BSS geschrieben werden.
Eine Diagnose darf auch bei Fehlerwerten keinen nicht präregistrierten
Fallback, Retry, Kalibrierungswechsel oder funktionalen Fix einschleusen.
Ein positives Diagnoseergebnis wäre noch kein Root-Cause-Beweis.

## Quellenidentität und Wiederaufnahme

Quellhashes der heute verwendeten Dateien relativ zu REF:

| Datei | SHA256 |
|---|---|
| board/kosagi/novena/novena_spl.c | ef3252c640f2b4c2dbc52b1c3b0c4fe0f0056a4cb95033e4839109e63b09474f |
| arch/arm/mach-imx/mx6/ddr.c | 2c2c41d7c0c3ce07f73f4e20f405b17b0cd8bbda37505be124e597d696182c95 |
| include/wait_bit.h | 66b2f5fb47c01fcabda3d1327f4cdb352fe18809b418417e51b8da0febc419c9 |
| arch/arm/lib/crt0.S | f43a2ba514cf81810c4e05e2372cc25808f8d24431b8bfa17f51d6f30b2542d6 |

Symbol-/Maschinenprüfung mit lokalem `arm-linux-gnueabihf-nm` und
`arm-linux-gnueabihf-objdump`; Hash-/Byteprüfung ohne neue Artefakte.
Zusätzlich maßgeblich: README.md, AGENTS.md, docs/PROJECT-STATE.md,
docs/TEST-LOG.md, docs/DECISIONS.md, docs/QUELLENVERZEICHNIS.md und
die unveränderten Projektpatches. Historische Novena-Quellen bleiben
Vergleichsmaterial, keine aktuelle Artefaktqualifikation.

Offen bleiben die Originalberichte und die lokale Verifikation der aus dem
früheren Chatbericht angegebenen A/B-Zuordnung,
ERR050070-Primärquelle und konkrete Anwendbarkeit, MPDGHWST-Lesesemantik,
reale WL-/DQS-/Timeoutwerte und der verursachende Speichermechanismus.
Der nächste zulässige Schritt innerhalb dieses Auftrags ist ausschließlich
Dokumentationsprüfung. Weitere Implementierung, Build, Mediumzugriff oder
Hardwaretest erfordern einen neuen konkreten Auftrag.

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_HARDWARE_BOOT_COUNT=1
PATCH_0008_SECOND_BOOT_ALLOWED=NO
```
