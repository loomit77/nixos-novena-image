# NOVENA-DDR-IMPLEMENT-168 – 2026-10-21

Ergebnis: `IMPLEMENTATION_RESULT=PASS_SOURCE_ONLY`.
Dies ist eine Quellimplementierung mit statischem Textvergleich, kein Build-,
Link-, Image-, Medien- oder Hardwareerfolg und kein Root-Cause-Nachweis.

## A. Bestand und tatsächlich veränderte Dateien

Vor Beginn: Branch `main`, HEAD
`c940d67c4df8a08f040106be3715318063874682`, sauberer Worktree.
Index-SHA256 vor und nach der Implementierung:
`b1010f26b06e00cdc8bd4d59a3cc53cd7bb0dad0b1e69d8853e95ab2e61b36d5`.
Alle sieben verlangten Einstiegsdateien existieren; keine Ersatzstelle nötig.
README, aktuelle Nachträge, DDR-Testauswertung und vollständige serielle
A-B-B-A-Logs vom 07.10. sowie DDR-08H vom 14.10. wurden berücksichtigt.
Letztere zeigen Q1/C1 ZERO vor D32 `ret=-12`; das ist kein Kalibrierungs-PASS.

Schreibziele wurden vor der Änderung auf Hauptprojekt-Patch, Nix-Einbindung
und zugehörige Dokumentation begrenzt. Tatsächlich geändert/neu angelegt:

- `boot/u-boot/0014-novena-historical-ddr-init.patch` (neu).
- `boot/u-boot/default.nix` (aktive Patchliste).
- `docs/NOVENA-DDR-IMPLEMENT-168.md` (dieser Bericht, neu).
- `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`, `docs/DECISIONS.md`,
  `docs/BUILD.md` (datierte Nachträge).

Keine ausgepackten U-Boot-Quellen angelegt. Die folgenden Quellpfade sind
**Zielpfade im Patch**, keine zusätzlichen Projektdateien:
`board/kosagi/novena/{novena_spl.c,ddr3.c,spd.h,Makefile}` und
`configs/novena_defconfig`. Die Patcharchitektur bleibt damit erhalten.

## B. Implementierte Sequenz

Ein Aufruf `novena_ddr_init(spd_i2c_ret)` in `board_init_f()`:

1. SPD auf I2C1/Bus 0 lesen; historischen CRC-/Fallbackpfad durchlaufen.
2. Historische Geometrie, Geschwindigkeitsklasse und Integer-Timings bestimmen.
3. Historischen IOMUX mit `cfg.width` programmieren.
4. Historische MMDC-Konfiguration einschließlich Startkalibrierwerten,
   Byte-Delays, ODT, ZQ, MRS, Powerdown und Refresh ausführen.
5. Vollständiges historisches Write Leveling ausführen.
6. Vollständiges historisches DQS-Gating, Read- und Write-Training ausführen.
7. WL-/DQS-Rückgaben anzeigen und historischen begrenzten Post-Test ausführen.
8. Zur modernen ARM-CRT zurückkehren; sie übernimmt BSS und `board_init_r()`.

Patchfundstellen: Einstieg Zeile 1528, I2C-Anbindung 1553,
einziger Boardaufruf 1722. Die neue Datei `ddr3.c` beginnt bei Patchzeile 27;
ihre Quellzeilennummer ist daher jeweils Patchzeile minus 26.

## C. Herkunft und statischer Vergleich

Historische Quelle ausschließlich aus Git-Objekten von
`/home/loomit/novena-historical-research/u-boot-novena`:
`bd5c7d3290bb94bf8af17760c321df67d2641343`.
Der dortige aktuelle HEAD ist ein späterer Stand und wurde nicht als
Referenzersatz verwendet. Forschungsrepositories wurden ausschließlich gelesen.

| Übernahme | Historische Fundstelle in bd5c7d3 | Neue Patchfundstelle |
|---|---|---|
| WL und MMIO-/Trainingshelfer | `board/kosagi/novena/ddr3.c:101–414` | 111–424 |
| DQS, Gating, Read-/Write-Training | dieselbe Datei:416–805 | 426–815 |
| SPD-Decoder | dieselbe Datei:807–976 | 817–994 |
| Begrenzter Post-Test | dieselbe Datei:142–211, 979–985 | 153–221, 996–1003 |
| IOMUX | `arch/arm/cpu/armv7/mx6/ddr.c:77–149` | 1011–1084 |
| MMDC-Konfiguration und Timingrechnung | dieselbe Datei:244–558 | 1086–1401 |
| DDR-/Gruppen-/Kalibrier-/Parametertabellen | `board/kosagi/novena/novena_spl.c:518–624` | 1403–1517 |
| Ersatz-SPD | `board/kosagi/novena/spd.h:1–29` | 1729–1757 |
| CRC16 und DDR3-CRC-Prüfung | `common/ddr_spd.c:67–118` | bestehende gemeinsame Implementierung |

Ziel ist der unveränderte Projektpin
`ece349ade2973e220f524ce59e59711cc919263f`, Version `2026.07`, aus
`boot/u-boot/default.nix:4–11`. Die lokalen 2020.07- und 2026.07-Quellen
sind Vergleichsstände, keine zusätzlichen GOOD-Binaries.

Die CRC-Funktionskörper sind im historischen und gepinnten modernen
`common/ddr_spd.c` byteidentisch. `CONFIG_DDR_SPD=y` bindet dieses Objekt
über `common/Makefile:71` auch in SPL ein. Keine zweite CRC-Implementierung.

Die Funktionskörper von WL, DQS, ihren Hilfsfunktionen, Post-Test, IOMUX
und MMDC wurden vollständig auf Gleichheit verglichen, mit Ausnahme von
Funktionsnamen/Linkage und Whitespace. Der SPD-Decoder ist ab dem
Lesefehlerzweig unverändert. Alle 256 Ersatzbytes sind bytegleich.
Benutzte MMDC-Strukturmember sind im gepinnten modernen Header vorhanden.

Vorarbeiten: PORT-162 begründet die Richtung; DDR-PORT-164 verlangt den
zusammenhängenden Pfad; EXTRACT-166 konsolidiert die Berichte.
HISTORY-167R präzisiert HISTORY-165: keine Erzählung einer 2016 entfernten
SPD-Implementierung. Die Mainline-Aufnahme war bewusst begrenzt;
spätere Fremderfolge qualifizieren weder dieses Binary noch dessen P_EXT-Pfad.
`bd5c7d3` bleibt Referenz für die Gesamtsequenz, kein isolierter Fix.

Die fünf Originalprotokolle unter `/home/loomit/novena-backups/block-logs/`
wurden lokal gefunden, ihre Berichte/relevanten Ausgaben herangezogen und
ihre vollständigen Dateihashes gegen die vorhandenen Sidecars verglichen:

| Kennung | TXT-SHA256 |
|---|---|
| NOVENA-UBOOT-PORT-162 | `7622b2c28aefecc567da529e4462fb40e6eb743fc7a91b0a9cc231481e7d0b18` |
| NOVENA-DDR-PORT-164 | `a1d2ca22a334c1999805741cef0ef63166109f7d7f8214857569bc16688f9272` |
| NOVENA-UBOOT-HISTORY-165 | `46fa96815f68203d64a8435ec9409bf6d4ed7b19b915003af36e5dc9c04dc022` |
| NOVENA-CODEX-EXTRACT-166 | `5ca2fffc71aacf0fd6eb8ba25c19aa315ddbd15be40e522440602c51bfbe5847` |
| NOVENA-UBOOT-HISTORY-167R | `dc2f7715a6cd04e1a63a2beeeb15aab5ae0d00312cb3bf85b72e628b8d64291e` |

Dies behauptet keine neue vollständige Inhaltsabnahme aller eingebetteten
86.972 Protokollzeilen oder ihrer früheren externen Quellen.

## D. Ausdrückliche Abweichungen gegenüber bd5c7d3

- Moderne Einzelheader ersetzen entfernte `common.h`-/`imx-common`-Includes;
  moderne Register-/SPD-Typen bleiben maßgeblich. Kopierte Hilfsfunktionen
  sind `static`; nur `novena_ddr_init()` besitzt externe Linkage.
- Historische IOMUX-/MMDC-Funktionen sind boardlokal umbenannt und zusammen
  mit Tabellen nach `ddr3.c` verschoben. Herkunft/Copyright bleibt erhalten.
- I2C1-Setupfehler wird auf dem SRAM-Stack weitergegeben; Bus 0 wird
  ausdrücklich gewählt. Setup-/Buswahlfehler nutzen denselben Fallback
  wie ein Lesefehler, ohne Zugriff über einen unvorbereiteten/falschen Bus.
  Das schließt die moderne Schnittstellenlücke; historisch fehlte die Prüfung.
- Erfolgreiches Lesen wird angezeigt; CRC-Fehler meldet die bestehende
  Funktion. Die historischen Fallbackmeldungen bleiben erhalten.
- WL-/DQS-Rückgaben werden erst nach beiden Phasen gemeinsam ausgegeben.
  Auch bei Fehlern wird historisch weitergegangen; kein neuer Abbruchpfad.
  Zusätzliche Ausgabe verändert die verstrichene Zeit, nicht die MMIO-Sequenz.
- Moderne Strukturzusätze `pd_fast_exit=0`, `ddr_type=0`, `refsel=1`,
  `refr=7`, `mpzqlp2ctl=0` sowie `SRT=0` sind explizit initialisiert.
  Der historische MMDC-Code verwendet die neuen Felder nicht; Refresh
  bleibt darin fest 1/7. SPD setzt SRT wie historisch anschließend selbst.
- Historischer Board-BSS-Clear und direkter `board_init_r(NULL, 0)` entfallen
  zwingend zugunsten des modernen CRT-Rückkehrvertrags.
- Historische interne/externe MMC-Auswahl wird nicht übernommen; der
  vorhandene moderne externe USDHC2-Vertrag bleibt erhalten.
- `CONFIG_MX6_DDRCAL` ist deaktiviert: dessen moderne Kalibrierung wird durch
  das boardlokale Objekt ersetzt. `CONFIG_DDR_SPD` wird aktiviert.
- Whitespace bereinigt; keine experimentelle Register-/Timingkorrektur.

## E. SPD, Fallback und Integerrechnung

Patchzeilen 817–994: SPD-Struktur auf dem Stack, Adresse `0x50`, Offset 0,
Adresslänge 1, 256 Byte. Lesefehler → Ersatzarray → CRC-Prüfung.
CRC-Fehler → Ersatzarray, danach historisch **keine erneute CRC-Prüfung**.
CRC-gültige semantisch unbrauchbare Daten fallen nicht generell zurück.
Unbekannte Seitengrößen behalten gegebenenfalls die Vorbelegung 2.
Unzureichende Geschwindigkeit, falsche Modulbreite oder nicht unterstützte
Bankstruktur erreichen den historischen Fatalpfad ohne Timeout.

Historische Ganzzahldivision und Reihenfolge bleiben erhalten:
`mtb_scale = (dividend * 100) / divisor`, anschließend `(Timingwert + 1)
mal mtb_scale`; keine Fine-Timebase-Auswertung. Ersatz-SPD ergibt
12 statt 12,5 sowie tRCD/tRC/tRAS 1272/4632/3372 in ns×100.
Keine neue allgemeine SPD-Validierung, auch kein Schutz vor Divisor 0.
Die Geschwindigkeitsklasse bleibt vor der MMDC-Timingzweigauswahl erhalten;
kein modernes Herabsetzen auf 1066 und keine zusätzliche MRS-Wartezeit.

## F. MMDC, WL, DQS und historische Besonderheiten

| Punkt | Bewusst erhaltene Operation | Patchzeile |
|---|---|---|
| MDMISC PHY0/PHY1 | WL maximiert beide PHYs, kein WL-Restore; DQS sichert PHY0 und schreibt nach PHY1 | 325–335, 460, 750 |
| MAPSR | WL setzt Bit 0; DQS setzt Bit 1; beide löschen am Ende Bit 3 | 311, 418, 447, 747 |
| RALAT/WALAT | SPD setzt 3/3; WL maximiert 7/3; DQS sichert erst danach; kein PHY0-Restore | 982–983, 325–335, 460–466, 750 |
| WL-Ergebnis | Fehlerbits beider PHYs, Softfail bei viermal `001f001f`, dann Startwerte zurück; keine moderne Lane-Korrektur | 358–382 |
| DQS-Gating | Getrennte RMW für Bit 30/28; FIFO zweimal; Gate-Nachrechnung subtrahiert `0xc0` trotz `0x80` im historischen Kommentar | 223–289, 565–610 |
| Read-Vorbereitung | Vergleichswort, zweiter Dummy Write, Read-Delay neu auf `40404040` | 624–643 |
| Write-Vorbereitung | Historischer weiterer Dummy Write bleibt unter `#if 0`; Delayinitialisierung und Messung bleiben aktiv | 685–715 |
| IOMUX-Breite | `cfg.width` ist Chipbreite, beim Ersatz-SPD 8; ausdrücklich keine Korrektur auf Modulbreite 64 | 1011–1084, 1533 |
| Polling | Historische unbegrenzte Wait-Schleifen einschließlich CON_ACK, FIFO, WL, Gating, Read/Write | 223–815 |
| Vor BSS | SPD/Status auf SRAM-Stack, Tabellen nichtnullinitialisiert in `.data`, Ersatzarray konstant; kein neuer Heap-/BSS-Zustand | 817, 1404–1547 |

MAPSR-Bits 3:1 sind laut in DDR-PORT-164 ausgewertetem RM reserviert;
PHY1-MDMISC-Restore ist dort keine bewiesene DDR3-Anforderung. Übernahme
dient der Quellreproduktion und ist keine technische Bestätigung dieser
historischen Operationen. PHY0 bleibt auf maximierten Latenzen.
Gatingfehler verhindern die historische Nachbearbeitung nicht.
Nach DQS bleiben Refresh-, CS-, Pullup- und CON_REQ-Abschlussfolge erhalten.

Der historische Post-Test bleibt aktiv: 16-Bit-Schreib-/Lesefolge über
256 KiB ab `0x10000000`, Summenvergleich. Er prüft weder den ganzen RAM
noch speziell BSS/Heap und besitzt weiterhin die historischen normalen
C-Zeigerzugriffe. Seine spätere Ausgabe ist kein vollständiger DDR-PASS.
Im vorliegenden Auftrag wurde dieser Code ausschließlich geschrieben/gelesen.

## G. SPL und USDHC2

Patchzeilen 1553–1723 ersetzen den modernen DDR-Block vollständig.
`board_init_f()` ruft weder gemeinsame DDR-Helfer noch zusätzlich BSS-Clear
oder `board_init_r()` auf. `arch/arm/lib/crt0.S` bleibt unverändert.
Die neuen Tabellen benötigen keinen vorab nutzbaren DDR-BSS; tatsächliche
ELF-Platzierung und Stackreserve müssen später geprüft werden.

Patch0001 erhält `USDHC2_BASE_ADDR` und `MXC_ESDHC2_CLK` in
`board_mmc_init()`. Der SPL-Loader benutzt die einzige registrierte
Legacy-MMC-Instanz als MMC1/Device 0; das ist nicht die Proper-Nummerierung.
Patch0002 behält Proper-Bootziel `mmc1`, Patch0003 das nichtpersistente
Environment. Keine neue interne microSD-Abhängigkeit. Der DDR-Port ändert
weder ROM-/P_EXT-Auswahl noch SPL-Ladeoffset, Nutzlastnamen oder Imageformat.

## H. Patchintegration und statische Prüfung

Aktiv: 0001–0008 → 0014 (`boot/u-boot/default.nix:19–30`).
0009–0013 bleiben bytegleich erhalten, sind jedoch inaktiv. Damit entfallen
deren moderne WL-/DQS-Instrumentierung und isolierte BI-/Dummy-Write-/RTT-
Änderungen. Die allgemeine Datei `arch/arm/mach-imx/mx6/ddr.c` bleibt im
effektiven Stack identisch zum Pin. Es gibt keinen zweiten aktiven DDR-Pfad.

0004–0008 bleiben als Diagnoseinstrumentierung in MMC/Allocator/SPL aktiv.
Ihre ausführlichen Q1/Q2-, C0/C1/C2- und D0–D36-Meldungen unterscheiden sich
von den durch 0010 verkürzten bisherigen Logs. Neue WL-/DQS-Ausgabe ist
`NOVENA-DDR historical WL=… DQS=…`; keine alte WAIT-/STATUS-Bitmap behaupten.
Der vorhandene LOCALVERSION-Text bleibt erhalten und identifiziert den
neuen Port allein nicht; künftig Patch-/ELF-/Imagehash mitführen.

Sämtliche alten Patches 0001–0013 gegen HEAD bytegleich geprüft.
Alter und neuer Stack wurden nur als Text im Prozessspeicher rekonstruiert:
vollständige eindeutige Hunk-Kontexte, Hunk-Zeilenzahlen und Rückvergleich
des erzeugten Patches erfolgreich. Keine Git-Anwendung, kein Checkout,
kein Compiler, kein Nix-Aufruf. Das ersetzt die spätere reale Patchphase nicht.
Zwei erste Vorbereitungsaufrufe scheiterten vor Dateianlage an Zeitstempeln
in Patchköpfen beziehungsweise verschobenen alten Hunk-Zeilenangaben;
danach vollständiger Kontextvergleich ohne Auslassen von Kontextzeilen.

Patch-SHA256:
`8abe90f3c1cbdc510582d3f1255c3b4f1bead73005712e63f401a58791d54a7d`.
Rekonstruierte neue `ddr3.c` (1521 Zeilen):
`66d327fa045460c959d4f533033b8cef667c24e507cf272e71fa7299fcde2818`.
`git diff --check` für die erfassten Änderungen ohne Befund;
neue Patchdatei zusätzlich textuell auf Whitespace/Hunk-Struktur geprüft.

## I. Risiken und offene Punkte

Kompilierbarkeit, Linkbarkeit, tatsächliche SPL-Größe, Stacktiefe und
I2C-/CRC-Laufzeit bleiben ungeprüft. Die historischen Auffälligkeiten,
semantisch problematische CRC-gültige SPD und Endlosschleifen bleiben
bewusste Risiken. Keine neue Messung entscheidet ihre Hardwarewirkung.
Historische echte SPD-/Fallbackwahl und ursprüngliche Binary-Buildprovenienz
sind weiterhin nicht geschlossen. Root Cause des modernen DDR-Fehlers offen.
Ein erfolgreicher historischer Boot qualifiziert den Port nicht.

Protokollgrenze: Der externe Runner wurde wegen verbotener Schreibzugriffe
außerhalb des Hauptprojekts nicht gestartet. Werkzeugausgaben dieser Sitzung
und dieser Bericht sind kein nachträglich erfundenes vollständiges Runner-TXT.
Kein Upload und keine neue Archivierungsqualifikation.

## J. Anforderungen an einen später separat freizugebenden Build

Vollständiges NixOS-SD-Image aus dokumentiertem Pin und Patchhash bauen;
SPL und Proper müssen aus demselben Paket stammen. Reale Patchphase,
Kconfig-Endwerte (DDR_SPD, Legacy-SPL-I2C, MX6_DDRCAL aus), Symbole,
ELF/Map/Disassembly prüfen. Einziger DDR-Aufruf, geladene `.data`/`.rodata`,
SRAM-Stack/GD/Early-Heap-Abstände, DDR-BSS/Full-Heap und SPL-Limit 64 KiB
nachweisen. Grenzen nicht als vermeintliche Reparatur verschieben.
IVT/Boot-Data/ROM-Ladebereich, SPL-Raw-Offset, FAT-`u-boot-dtb.img`, Header,
Kernel/Initrd/DTB/Rootvertrag und vollständigen Imagehash qualifizieren.
Ein isolierter Komponentenbuild ersetzt die vollständige Imageprüfung nicht.

## K. Anforderungen an spätere Hardwaretests

Eigene Freigaben für Medium, Schreibbereich und genaue Versuchszahl.
Geräteidentität/Mounts/Sicherung/Artefakthash prüfen; gesamten geschriebenen
Bereich zurücklesen und vergleichen. Rohaufnahme vor Start aktivieren.
Definierten POR mit P_EXT/externer SD/USDHC2 von Warmstart/Recovery trennen.
SPD-Lese-/CRC-/Fallbackwahl, DIMM, Kalibrierungsanzeigen und Phasengrenzen
erfassen; tatsächliche MDMISC/MAPSR/WL-Endwerte benötigen gegebenenfalls
gesondert freigegebene Instrumentierung, die hier nicht hinzugefügt wurde.
Normale DDR-Zugriffe an benötigten BSS-/Heap-/Proper-Bereichen, MMC-Laden,
Proper und vollständigen externen Linux-/Rootboot getrennt bewerten.
Interne Bootloader-/Environment-/Rootabhängigkeit gesondert ausschließen.
Keine automatischen Wiederholungen bei Hängern/Fehlern.

`IMPLEMENTATION_RESULT=PASS_SOURCE_ONLY`
