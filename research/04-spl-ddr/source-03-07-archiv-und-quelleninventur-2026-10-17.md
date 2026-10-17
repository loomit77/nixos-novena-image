# Novena DDR/SPL – SOURCE-03 bis SOURCE-07

Stand: 17.10.2026. Konsolidierte lokale Forschungsdokumentation für das Projekt „Novena – universelles NixOS SD-Image“.

## 1. Zweck und Geltungsbereich

Diese Dokumentation verbindet die Ergebnisse von SOURCE-03 bis SOURCE-07: historische DDR-/SPL-Provenienz, aktuelle Diagnoseartefakte, Register- und Kalibrierungsunterschiede sowie den tatsächlich erreichten Umfang der Kommunikations- und Archivrecherche. Sie bewahrt belastbare Ergebnisse und deren Grenzen, ohne die umfangreichen Originalprotokolle zu duplizieren.

**Gesicherter Kernbefund:** Die vier A-B-B-A-Läufe vom 07.10.2026 und der Patch0013-Lauf vom 14.10.2026 zeigen Q1/C1 ZERO vor MMC1 und anschließend D32 `ret=-12`. Die DDR-/SPL-Root-Cause ist nicht bewiesen. Finale MPWLDECTRL-Werte und ein tatsächlicher MDMISC-Readback desselben Boots fehlen im geprüften Bestand.

Dieser Auftrag erstellt ausschließlich diese Datei. Er umfasst keine neuen Builds, Patches, Hardwaretests, Medienzugriffe, Netzwerkabfragen oder Datenbankänderungen. Beschreibungen früherer Aktionen sind historische Befunde und keine erneute Durchführung. Die abgeschlossene I2C3-/ES8328-Untersuchung bleibt geschlossen; `regulator-always-on` bleibt die dokumentierte dauerhafte Boardanforderung.

Die Evidenzklassen E1–E8 in Abschnitt 12 gelten im gesamten Dokument. „Gesichert“ bedeutet jeweils innerhalb der angegebenen Klasse: Ein gesicherter Quellcodebefund ist noch kein gemessener Hardwarezustand. Eine Hypothese wird auch durch mehrere historische Vergleichsberichte nicht zur bewiesenen Ursache.

## 2. Quellen und Originalprotokolle

### Pfadkonventionen

| Kürzel | Absoluter lokaler Pfad |
|---|---|
| P | `/home/loomit/nixos-novena-image` |
| B | `/home/loomit/novena-backups` |
| R | `/home/loomit/novena-research` |
| H | `/home/loomit/novena-historical-research` |
| K | `/home/loomit/novena-research/kosagi-archive` |
| I | `/home/loomit/novena-dateiinventur` |

`B/block-logs/<Dateiname>` bezeichnet stets die erhaltene lokale Originaldatei. Die Kennungen SOURCE-03 bis SOURCE-07 bezeichnen hier ausschließlich die folgenden Protokolle vom 17.10.2026, nicht das ältere `DDR-SPL-SOURCE-03-20261012-…`.

### Vollständige Protokollidentitäten

Die fünf Dateien wurden vollständig lokal eingelesen und auf ihre thematischen Belege durchsucht; für die Konsolidierung wurden Analyseergebnisse, enthaltene Primärausgaben und spätere Korrekturen zusammengeführt. Die unten angegebenen SHA256 wurden am 17.10.2026 aus den vollständigen Originalbytes berechnet und stimmen jeweils mit dem vorhandenen `.sha256`-Sidecar überein. Darstellungsnormalisierung von Zeilenenden wurde nicht für die Hashbildung verwendet.

| Kennung | Originaldatei unter B/block-logs/ | Byte | SHA256 |
|---|---|---:|---|
| SOURCE-03 | `NOVENA-CODEX-SOURCE-03-20261017-111150-mWA5kSLo.txt` | 441665 | `3141aea132d52af03866ff99cb1571cce8e860dfd1d2a69ef1e1c889e1750b96` |
| SOURCE-04 | `NOVENA-CODEX-SOURCE-04-20261017-112441-mHAtOzjB.txt` | 1847101 | `7db638bc6e92d788ec0183f5e40792c6a30451dbffa53a1319288d4f071e5547` |
| SOURCE-05 | `NOVENA-CODEX-SOURCE-05-20261017-113405-TbgHApMU.txt` | 2797747 | `ccfae2e6c901d517ed6ce14aca2ab9709531c32e6507635c18ed6287688c118f` |
| SOURCE-06 | `NOVENA-CODEX-SOURCE-06-20261017-114116-ouFObU8B.txt` | 802400 | `dc4114097cb1a8b10d98bf885d239d1206f649439f8b39ed52c3794675311a8b` |
| SOURCE-07 | `NOVENA-CODEX-SOURCE-07-20261017-114804-36qA3xp5.txt` | 2156732 | `6c40e49a2b79d00317cc21b8c93afa077118d349c4e6845db60134a00da0091e` |

Die in SOURCE-07 für SOURCE-03–06 genannten Git-Blob-IDs stimmen mit den lokal berechneten Git-Objektidentitäten überein. Für SOURCE-07 wurde die Objektidentität hier zusätzlich aus den vollständigen lokalen Bytes berechnet, ohne ein Git-Objekt zu schreiben:

| Protokoll | Git-Blob-ID (SHA-1 über Git-Blobheader und Originalbytes) |
|---|---|
| SOURCE-03 | `fd928580fa081ff85f85c0dafcdd01d72c86cec1` |
| SOURCE-04 | `c274cfabcf38d4714c9fbfa3b516feece976cd36` |
| SOURCE-05 | `ba9f0855d0cc3c9a0c397a7cf3f48daaa23185ba` |
| SOURCE-06 | `65f18267a0ddd680cce978add0f1732474e534b1` |
| SOURCE-07 | `858d92b65c3daf792175f7f6acd5a5c10a211c86` |

Dies identifiziert die Protokollbytes, nicht einen remote geprüften Commit oder erfolgreichen Upload.

Die Protokolle enthalten auch eingebettete ältere Ausgaben und wiederholte Zusammenfassungen. Solche Wiederholungen sind keine unabhängigen Messungen. `BLOCK_RESULT=PASS` und `LOG_COMPLETE=YES` betreffen die jeweilige Analyse-/Protokolldurchführung; sie bedeuten keinen DDR-PASS.

Die vollständigen Originalprotokolle sind laut Auftragsangabe im Repository `loomit77/project-block-logs`, Verzeichnis `novena/2026/10/`, archiviert:

- [SOURCE-03 auf GitHub](https://github.com/loomit77/project-block-logs/blob/main/novena/2026/10/NOVENA-CODEX-SOURCE-03-20261017-111150-mWA5kSLo.txt)
- [SOURCE-04 auf GitHub](https://github.com/loomit77/project-block-logs/blob/main/novena/2026/10/NOVENA-CODEX-SOURCE-04-20261017-112441-mHAtOzjB.txt)
- [SOURCE-05 auf GitHub](https://github.com/loomit77/project-block-logs/blob/main/novena/2026/10/NOVENA-CODEX-SOURCE-05-20261017-113405-TbgHApMU.txt)
- [SOURCE-06 auf GitHub](https://github.com/loomit77/project-block-logs/blob/main/novena/2026/10/NOVENA-CODEX-SOURCE-06-20261017-114116-ouFObU8B.txt)
- [SOURCE-07 auf GitHub](https://github.com/loomit77/project-block-logs/blob/main/novena/2026/10/NOVENA-CODEX-SOURCE-07-20261017-114804-36qA3xp5.txt)

Diese Navigationslinks verwenden `main` und wurden wegen des Netzwerkverbots nicht aufgerufen. Remoteinhalt, Zugänglichkeit, Branch und Byteidentität wurden in diesem Auftrag nicht geprüft. Maßgebliche lokale Inhaltsidentität ist der jeweilige SHA256 oben. `B/source-audit-20261017/SOURCE-03-07-LOG-INDEX.txt` ist ein ergänzender Wegweiser; SOURCE-07 wird darin noch als laufende Inventur bezeichnet. Dieser Index ersetzt keines der fünf Originalprotokolle.

### Arbeitsteilung der Untersuchungen und vorhandene Dokumentation

| Quelle | Schwerpunkt | Aktualisierung und Grenze |
|---|---|---|
| SOURCE-03 | Kommunikationsdatenbanken, originale Novena-/DDR-Mails, Forum, Abgleich mit A-B-B-A | Erschließt zuvor nicht direkt erreichbare Originalmails; Patch0013-Teststatus noch offen |
| SOURCE-04 | Registervertrag, historische und aktuelle Initialisierung, SPD, Terminierungsphasen | Trennt Schreibwerte und Readbacks; damalige Aussage „0013 nicht als getestet qualifiziert“ wird durch SOURCE-05–07 überholt |
| SOURCE-05 | SPL-/Imageidentität, direkte Binäranalyse, Build- und Testprovenienz | Belegt 0013-Artefakt und dokumentierten Fehlversuch; ersetzt keinen Registerdump |
| SOURCE-06 | Suche nach finalen WL-/MDMISC-Werten in erhaltenen Logs und Kommunikation | Kein zusammengehöriger Endwertsatz gefunden; damalige Archivprüflücke wird durch SOURCE-07 teilweise verkleinert |
| SOURCE-07 | Archivinventur, UART-/IRC-Überlieferungen, Korrektur der Builddatierung | Build am 13.10., Test am 14.10.; kein neuer WL-Endwertsatz und keine Root-Cause |

Der vorhandene Projektstand wurde insbesondere über [PROJECT-STATE](../../docs/PROJECT-STATE.md), [TEST-LOG](../../docs/TEST-LOG.md), [DECISIONS](../../docs/DECISIONS.md) und [QUELLENVERZEICHNIS](../../docs/QUELLENVERZEICHNIS.md) abgeglichen. Deren maßgeblicher DDR-Checkpoint beschreibt bereits A-B-B-A und die Grenzen der Medienzuordnung. Neu zusammengeführt werden hier vor allem Patch0013, die phasenbezogene Registerklassifikation und SOURCE-07.

Die älteren Dokumente bleiben als datierte Stände erhalten:

| Bestehende Dokumentation | Funktion und Abgrenzung |
|---|---|
| [Checkpoint 9C-43/44](block-9c-43-dokumentationscheckpoint.md) | Statische Rekonstruktion vom 02.10.; kein wiedergefundenes Original von 9C-43 und kein A/B/0013-ELF |
| [11 – Archiv-Dateiaudit](../05-gesamtanalyse/11-ARCHIV-DATEIAUDIT.md) | Identitätsaudit vom 03.10.; kein Ersatz für SOURCE-07-Archivinhalte |
| [13 – Rohlog- und Evidenzregister](../05-gesamtanalyse/13-ROHLOG-UND-EVIDENZREGISTER.md) | Ältere Logidentitäten und Bootsegmentgrenzen; spätere Versuche hier ergänzt |
| [18 – U-Boot-DDR-Historie](../05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md) | Historische Codepfade und Releasegrenzen; damals offene Originalmailzugriffe sind teilweise geschlossen |
| [19 – Kosagi-DDR-Hardware](../05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md) | Board-/Modulkontext; keine heutige DIMM-Identifikation |
| [20 – Externe Quellenmatrix](../05-gesamtanalyse/20-EXTERNE-QUELLENMATRIX.md), [21 – Recherchelücken](../05-gesamtanalyse/21-EXTERNE-RECHERCHE-LUECKEN.md) | Historische Lesetiefen und Lücken; inzwischen gelesene RM-/Mailstellen werden hier datiert eingeordnet |

Diese Dateien wurden nicht verändert. Ältere fehlende Quellenzugriffe werden nicht rückwirkend als damalige erfolgreiche Prüfung dargestellt.

## 3. Technischer Ausgangszustand

### Historische DDR-/SPL-Linie

Die zentrale historische Vergleichsquelle ist `H/u-boot-novena`, ausdrücklich Commit `bd5c7d3290bb94bf8af17760c321df67d2641343` vom 04.09.2015, mit `board/kosagi/novena/ddr3.c`, `novena_spl.c`, `spd.h` und `arch/arm/cpu/armv7/mx6/ddr.c`. Der damalige Commitbetreff aktualisiert die Debian-Paketversion und ist selbst kein eigenständiger DDR-Fix.

Die Paketlinie `2014.10.r7-novena.1` und das Banner `2015.04-rc1-02853-gbd5c7d3` sind unterschiedliche Versionsangaben desselben historischen Zusammenhangs: Das Makefile basiert auf 2015.04-rc1. Daraus folgt kein automatischer Widerspruch, aber auch keine vollständige Binärprovenienz. Der spätere Repository-HEAD darf nicht als bd5-Stand verwendet werden.

Bereits dokumentierte Versionsbeziehungen: rc5/rc7 besitzen dieselbe DDR3-Datei; rc12/rc14 ebenfalls, jedoch nicht dieselbe gesamte SPL-Datei. Die rc12-Quellzuordnung lautet `c5efeadb913c8246be372054adcd6106ad4fe067`; bd5 hat eine weiterentwickelte boardspezifische DDR-Datei. Quellen- und Releasegleichheit sind deshalb immer datei- beziehungsweise artefaktbezogen zu beurteilen. Die Factory-R1-Zuordnung bleibt von der r7-Paketlinie getrennt.

Der historische Boardpfad lautet: SPD oder Ersatz-SPD → IO-Konfiguration → DDR-Konfiguration → WL → DQS → kurzer Speichertest → BSS-Clear → weiterer SPL-Pfad. Die Kalibrierungsrückgaben werden ignoriert; auch ein gedruckter Speichertestfehler beendet den gelesenen Boardcode nicht zwingend. Der historische Speichertest umfasst laut Code 256 KiB in `[0x10000000,0x10040000)` mit 16-Bit-Zugriff und Summenvergleich. Er qualifiziert weder den heutigen Kontrollbereich um `0x18200030` noch das gesamte DIMM.

### Aktueller Projektpfad

Die gepinnte U-Boot-Basis der untersuchten Projektartefakte ist `ece349ade2973e220f524ce59e59711cc919263f` („Prepare v2026.07“, 06.07.2026), ergänzt durch den jeweiligen Projektpatchstack. Der Hardwarebeleg reicht für den Teilpfad `P_EXT → i.MX6-ROM → externe SD/USDHC2 → SPL v2026.07`. Der weitere eigenständige Pfad bis U-Boot proper, Bootskript, Kernel/Initrd/DTB und externem Root-Dateisystem ist nicht vollständig hardwareseitig nachgewiesen.

Der aktuelle Boardpfad verwendet feste DDR-Strukturen statt einer SPD-basierten Modulkonfiguration. Ein Quellvergleich, ein erfolgreicher Build, die Einbettung ins Image, ein Medienreadback und ein beobachteter Boot sind getrennte Qualifikationsstufen.

## 4. Gesicherte Hardwarebeobachtungen

### A-B-B-A vom 07.10.2026

Originalverzeichnis: `P/test-logs/2026-10-07/`. A bedeutet BI0 mit Projektpatches bis 0011, B zusätzlich Patch0012. Diese Bezeichnungen sind nicht die älteren Diagnosearchitekturentwürfe A/B aus DECISIONS.

| Lauf / Originaldatei | Variante | Aufzeichnungsintervall, +02:00 | SHA256 des Rohlogs | Ergebnis |
|---|---|---|---|---|
| `9C-52Q-R7B-3AAJ-RUN1-BI0-CONTROL.serial.log` | A | 15:11:04–15:13:07 | `8aaaeeca6824ed6d4fa06ae104fe7093af5ea2299f2f19c33cfa493ac2467b30` | ZERO |
| `9C-52Q-R7B-3AAJ-RUN2-PATCH0012.serial.log` | B | 16:30:08–16:31:19 | `6ead07c0a617a57360b5dc5b9824cf0938deeb3da26e212c3c44daa8a773b41f` | ZERO |
| `9C-52Q-R7B-3AAJ-RUN3-PATCH0012.serial.log` | B | 16:34:12–16:34:53 | `5e934af26723fe5c52b7e7c082e45421151cc53e2b2e9750d947a3242cd39f43` | ZERO |
| `9C-52Q-R7B-3AAJ-RUN4-BI0-CONTROL-REVERSION.serial.log` | A | 16:47:16–16:47:46 | `1d363050f219df360ad408ecdd5de1252d32b1ec35931560d707f8be2d67f327` | ZERO |

Die Hashes wurden für diese Konsolidierung erneut lokal bestätigt. Laut TEST-LOG erfolgte zwischen RUN2 und RUN3 kein Rewrite; vor RUN4 wurde A erneut vollständig geschrieben und durch Image- und separaten SPL-Readback bestätigt. Diese Medienbindung ist dort ausdrücklich teilweise übernommene bestätigte Auftragsprovenienz. Die seriellen Bytes allein beweisen weder das geschriebene Image noch POR, Stromtrennung oder vollständige Versorgungsvorgeschichte.

Alle vier Originale zeigen dieselbe relevante Ausgabe:

```text
NOVENA-DIAG WL wait=0 gcr0=00000000 hwerr0=f0f87878 gcr1=000000d0 hwerr1=e1e1f0e1
NOVENA-DIAG WL ret=0
NOVENA-DIAG DQS PATH v=1 ds=2 ci=1 ca=1 t=0
NOVENA-DIAG DQS WAIT e=000f7bfb f=00000000
NOVENA-DIAG DQS STATUS s=00000000
NOVENA-DIAG DQS RET ret=0
NOVENA-DIAG HEAP q1=00000000
NOVENA-DIAG CTRL c1=00000000 valid=7
Trying to boot from MMC1
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
```

Das Muster ist `A_ZERO–B_ZERO–B_ZERO–A_ZERO`. Die Anzeigen belegen den erfassten Status- und Rückgabepfad, keinen vollständigen DDR-Funktions-PASS. Der durch Patch0012 ergänzte Wait-Rückgabewert wird nicht separat ausgegeben; die bestehende DQS-Bitmap wurde dafür nicht erweitert.

### Patch0013 am 14.10.2026

Original: `B/block-logs/NOVENA-DDR-08H-20261014-124804.serial.log`, 1764 Byte, SHA256 `410fdfe43f2102f2ab77c23c3cc3b2995c13b6ef4a3da0521420cf282abfc5a5`. Die Sitzung dauert laut Kopf/Ende von 12:48:04 bis 12:48:55 +02:00, mit picocom 3.1, 115200 Baud, 8N1 und ohne Flow Control. Auch dieser Rohlog wurde erneut gelesen und gehasht.

Er zeigt dieselbe oben wiedergegebene WL-/DQS-/ZERO-/D32-Folge. Der erwartete SPL-Hash steht zusätzlich in `NOVENA-DDR-08C-20261014-123834-9h4OW9gx.txt`; die Artefakt- und Medienkette ist in Abschnitt 5 aufgeführt. Dies belegt einen dokumentierten erfolglosen 0013-Versuch. Es belegt keinen DDR-PASS und keine gemeinsame kontrollierte A-B-B-A-Serie mit dem 07.10.

### Bedeutung von Q1, C1 und D32

Q1 ist der unmittelbare volatile Readback der Heapzeigervariablen `mem_malloc_brk` nach deren Zuweisung. Er ist kein Readback des Inhalts an der Heapstartadresse `0x18300000`. C1 liest nach dem Store von `0x18300000` an die Kontrolladresse `0x18200030`. Für das untersuchte 0013-Binärlayout liegt das Q1-Ziel bei `0x1820003c`; die Zugriffe sind 32 Bit breit. Die Symboladresse ist nicht ohne Binärprüfung auf andere Linklayouts übertragbar.

Q1 wird vor dem Heap-`memset` aufgenommen. Die C0/Store/C1/Restore/C2-Zugriffe geschehen vor `mem_malloc_init()`, ihre Ausgabe erst danach. Die Druckreihenfolge HEAP → CTRL entspricht daher nicht der Reihenfolge dieser Speicherzugriffe.

`valid=7` kodiert ausschließlich `C0==0`, `C2==0` und `C2==C0`. C1 geht in diese Maske nicht ein. Die Maske bestätigt somit die Kontrollbedingungen, keinen gelungenen Nichtnull-Store/Load-Test. Die Rücklesediskrepanz ist belegt; ob physisches Schreibversagen, ein Lesepfadproblem oder ein anderer Speicherpfadmechanismus vorliegt, ist damit nicht entschieden.

D32 `ret=-12` folgt zeitlich auf ZERO. SOURCE-05 findet im Binärcode einen passenden ENOMEM-Pfad nach Allokationsaufrufen für 120 beziehungsweise 480 Byte. Der Rohlog entscheidet nicht, welcher Aufruf scheitert; `-ENOMEM` beweist keinen realen Kapazitätsmangel. Ein falsch rückgelesener Allocatorzustand ist eine plausible Folgeerklärung, keine bewiesene Ursache.

MMU aus, DCache aus, ICache an und R0=`0x18300000` sind laut PROJECT-STATE übernommene frühere Qualifikationsbefunde. Ein eigenständiger ursprünglicher Qualifikationsbericht ist dort nicht nachgewiesen. Die seriellen Logs messen diese Prozessorzustände nicht.

## 5. Binär- und Imageprovenienz

### A/B/0013-Identitäten

Quelle: SOURCE-05, „Provenienzmatrix“ und direkte Binäranalyse. Alle drei SPL-Dateien sind 52224 Byte groß, alle drei Images 2581291008 Byte. SOURCE-05 bestätigt den jeweiligen SPL-Hash auch für den Imagebereich ab Byteoffset 1024 mit Länge 52224. Das ist eine Datei-/Bereichsprüfung, kein erneuter Medienreadback in SOURCE-05 oder diesem Auftrag.

| Variante | Patchumfang | SPL-SHA256 | Image-SHA256 | Testzuordnung |
|---|---|---|---|---|
| A | bis 0011, BI0 | `263168797669b2aad37ab6e46a3555e9b5fdaca8e906d01e9b87f5ad42fc25ba` | `ff9ea88aa0d26db148e51751f32a3d4b5afc48dcc8c974957f0005ae6d569b8a` | RUN1/RUN4, 07.10. |
| B | A + 0012 | `41e6fc05ec133d5b1777094c2dddda91bb19dda3d022360010c9b2288f81c448` | `32b17519dee56addf29bd6b9c105be9ea0934c25817f02800787145d9ac88cbd` | RUN2/RUN3, 07.10. |
| 0013 | B + RTT_NOM-Änderung | `23e01ebb1be34a47887f0145acf19853c5632c686dc4147fd22dc4f01efb25b8` | `937776dbb17f0a870fca22519c20a04845dd530de7db8c0496b3e5d286cc2934` | DDR-08H, 14.10. |

Die absoluten Originalpfade sind:

| Variante | SPL | Image |
|---|---|---|
| A | `/nix/store/p4rwpc38iajn0gk3vf7f6cwbxafbb0c2-uboot-novena_defconfig-armv7l-unknown-linux-gnueabihf-2026.07/SPL` | `/nix/store/xgp0l80pcbaimirw5sqswnrg1qrci396-nixos-novena-sd-image.img` |
| B | `/nix/store/wk33h1v8v243zrafly3y8rsg1fjr9cvk-uboot-novena_defconfig-armv7l-unknown-linux-gnueabihf-2026.07/SPL` | `/nix/store/23f1flbp5dsyr9qxdh2mvkbfqaf3kjmi-nixos-novena-sd-image.img` |
| 0013 | `/nix/store/cy3a0lrp4n5vhjzpxk0yhy86akbaak1i-uboot-novena_defconfig-armv7l-unknown-linux-gnueabihf-2026.07/SPL` | `/nix/store/p4fwzdzm8j57xji213vcva4wg5a79hiq-nixos-novena-sd-image.img` |

Nix-Store-Pfade sind lokale Fundorte. Die vollständigen Hashes bleiben für spätere Identifikation entscheidend, auch falls diese Pfade nicht mehr existieren.

### Patch0013-Build und Testkette

`boot/u-boot/0013-novena-spl-rtt-nom-rzq2.patch` hat den in SOURCE-05 bestätigten SHA256 `ef32016792391521bf5d27ab1921750037ac250481320569d456ab5c89aca397`. B und 0013 unterscheiden sich über alle 52224 SPL-Bytes an genau einem Byte: Dateioffset `0xc649`, B=`01`, 0013=`02`. Das passt zu `rtt_nom=1 → 2`. Die Instruktionsbytes bleiben gleich; das Datenbyte verändert die datenabhängige Ausführung.

| Original unter B/block-logs/ | Datum | Belegter Schritt und Grenze |
|---|---|---|
| `NOVENA-DDR-07D-20261013-193958-8xmZIJxB.txt` | 13.10., 19:39:58–19:41:30 +02:00 | Tatsächlicher U-Boot-Build; Anwendung der Patches und Ausgabe cy3a…; kein Hardwaretest |
| `NOVENA-DDR-07E-20261013-194517-le7pbZAk.txt` | 13.10. | Bestätigung von Buildartefakt und Derivation |
| `NOVENA-DDR-07I-20261013-195503-b83bvlPP.txt` | 13.10. | Image-/SPL-Byteidentität |
| `NOVENA-DDR-07K-20261013-200119-LJXUcDmY.txt` | 13.10. | In SOURCE-05 aufgeführter vollständiger Image-Readback |
| `NOVENA-DDR-08C-20261014-123834-9h4OW9gx.txt` | 14.10. | Erwarteter SPL-Hash vor dem Lauf |
| `NOVENA-DDR-08H-20261014-124804.serial.log` | 14.10. | Aufgezeichnete Fehlerfolge; Hash in Abschnitt 4 |

Für DDR-07D wurde der Originaldateihash hier erneut berechnet: `b32dbe497b69f9838566c05cd872a87df99d596e49b748203e3321b06e7382d4`. Für die übrigen Kettenglieder werden ohne separate Hashprüfung keine neuen Dateihashes behauptet. Die Kette dokumentiert frühere Schritte; in diesem Auftrag wurde kein Medium gelesen.

**Datierung:** Build am 13.10.2026, Test am 14.10.2026. Ein zusätzlicher Neubau am 14.10. ist nicht nachgewiesen. Der fest gesetzte Epoch-Wert `1789984363`, das Bannerdatum 21.09. und die konfigurierte Localversion `gf8baca04e22d` sind keine tatsächlichen Buildzeit- oder vollständigen Quellidentitätsnachweise.

Die in Kommunikationsprotokollen zusätzlich erwähnte Bezeichnung `0013-novena-diag-wl-walat-snapshots.patch` ist eine Namenskollision. Ihre Erwähnung beweist weder Anwendung noch Build oder Test. Sie gehört nicht zum hier identifizierten RTT_NOM-Test.

### Historischer Paket-SPL und Begleitartefakte

SOURCE-05 bestätigt im verifizierten lokalen Raw-Master unter `B/historical-novena-usb-2026-09-28/` den historischen bd5-Paket-SPL als Bereich ab `0x3512e000`, Länge 44032 Byte, SHA256 `e2f2b80692c3ca55d29bce39fc98efd44c09046959acda46c50675ef3728e276`. Dessen DDR-Disassembly stammt aus `DDR-SPL-GOOD-PROVENANCE-01-20261012-092644-xRyTVf3G.txt` und wurde in SOURCE-05 als vorhandener Befund ausgewertet. Diese Bereichsidentität ist kein Nachweis jedes historisch gestarteten Mediums.

„GOOD“ bezeichnet deshalb einen historischen Paket-/Quellzusammenhang mit Betriebsbelegen. Die Kette SPL-Hash → tatsächlich gestartete MMC-Bytes → Bootdatum → DIMM → Registerzustand ist nicht für alle historischen Boots geschlossen. `DDR-SPL-BOOT-PROVENANCE-01G-R1-20261012-101043-bFwCRgMS.txt` verbindet das MMC-Backup vom 12.09. mit GOOD_R7; die unveränderte Fortgeltung bis zum 22.09. bleibt offen.

Ein passendes ursprüngliches ELF-/Map-Paar für A, B oder 0013 wurde im gezielt geprüften Bestand nicht gefunden. Der Locked-Debug-Baum `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/` gehört zum älteren Patch0008-Stand. Er ist kein Ersatzbegleiter der späteren Binärdateien.

## 6. DDR-Register und Kalibrierung

### Tatsächlicher WL-/DQS-Messumfang

Die in Abschnitt 4 aufgezeichneten `gcr0/1` und `hwerr0/1` sind MPWLGCR-/MPWLHWERR-Statuswörter. Sie sind keine MPWLDECTRL-Lane-Verzögerungen. DQS-Wait-/Statusanzeigen belegen ebenfalls nicht die finalen Gate-, Read- oder Write-Delaywerte. Insbesondere wird `ret=0` nicht als erfolgreicher CPU-Speicherzugriff interpretiert.

SOURCE-06 fand in den untersuchten erhaltenen Aufzeichnungen keinen zusammengehörigen Satz finaler MPWLDECTRL0/1 beider PHYs. SOURCE-07 liefert keinen solchen neuen Fund. Im historischen Git-Objekt `bd5c7d3:board/kosagi/novena/ddr3.c`, um Zeile 389, stehen vier DEBUG-Ausgaben nach WL in der Reihenfolge PHY0 CTRL0, PHY0 CTRL1, PHY1 CTRL0, PHY1 CTRL1. Ein Formatstring und die im Code vorhandenen Readbacks sind kein aufgezeichneter Endwertsatz.

### Initialwerte und Registervergleich

PHY0-Basis ist `0x021b0000`, PHY1-Basis `0x021b4000`. Sämtliche Werte der folgenden Tabelle sind Quell-/Binärbefunde oder daraus abgeleitete programmierbare Anteile. Sie sind keine finalen Hardwaremesswerte.

| Register | Offset | Historischer bd5-Startwert PHY0 / PHY1 | Aktueller Startwert PHY0 / PHY1 | Befund und Grenze |
|---|---|---|---|---|
| MPWLDECTRL0 | `+0x80c` | `00420048` / `005a0104` | identisch | Startwerte; WL kann überschreiben, korrigieren oder bei Soft-Fail restaurieren |
| MPWLDECTRL1 | `+0x810` | `006f0059` / `01070113` | identisch | Kein finaler Vierer-Readback belegt |
| MPDGCTRL0 | `+0x83c` | `437c040b` / `444f0446` | identisch | HW-Gating und Nachkorrektur verändern die Werte |
| MPDGCTRL1 | `+0x840` | `0413040e` / `044d0422` | identisch | Finale Werte fehlen |
| MPRDDLCTL | `+0x848` | `4c424249` / `4e48414f` | identisch | Vor Gating `40404040`, historisch nochmals vor Read-Cal; Endwerte fehlen |
| MPWRDLCTL | `+0x850` | `42414641` / `46374b43` | identisch | Vor Write-Cal `40404040`; Endwerte fehlen |
| MPZQHWCTRL | `+0x800` | `a1390001`, später `a1390003` | gleiche nominale Werte | Keine belegte nominale Startwertdifferenz; HW-Ergebnisfelder bleiben getrennt |

Fundstellen: SOURCE-04 „Registervergleich“; historisch `novena_spl.c:577–624`, `ddr3.c:254–804`; gepinnter v2026.07-Boardcode `novena_spl.c:465–515`, gemeinsamer `ddr.c:62–605`. Ein vorhandener projektlokaler Textauszug ist `research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-novena-spl-after.txt`; er ersetzt die explizite Commitbindung nicht.

Nach IMX6DQRM Rev. 6 §44.12.34–35 enthält jedes MPWLDECTRL-Halbwort den Offset in Bits 6:0, einen Halbzyklus in Bit 8 und Ganzzyklen in Bits 10:9; Ganzzykluswert 3 ist reserviert. Die zweite Lane liegt um 16 Bit verschoben. Rechnerisch gilt `Offset/256 + Halbzyklus/2 + Ganzzyklen` in Taktperioden. Diese Kodierung erlaubt die Einordnung der Starttabellen, aber keine Rekonstruktion fehlender finaler Lane-Messwerte.

### WALAT/RALAT und MDMISC

Herstellerquelle: `P/research/nxp-reference-manuals/sources/IMX6DQRM_Rev6_202005.pdf`, NXP i.MX6Dual/Quad Reference Manual, Rev. 6, Mai 2020, SHA256 laut SOURCE-05 `8ee5d62a592f27c83bda37a9fb7fff57810e8b6d19fa6badbb9b5f1c8699b505`. Einschlägig sind §44.12.7, gedruckte Seiten 3708–3709, §44.11.6 und §44.12.34–35. Die Dokumentidentität und Aufnahmeprovenienz sind zusätzlich im lokalen [RM-Wegweiser](../nxp-reference-manuals/README.md) dokumentiert.

MDMISC liegt bei `+0x018`: WALAT in Bits 17:16, RALAT in Bits 8:6, BI_ON in Bit 12, MIF3_MODE in Bits 10:9 und CALIB_PER_CS in Bit 20. Die Herstellerbeschreibung verlangt WALAT unter Berücksichtigung der WL-Verzögerungen: Ein Offset `WL_DL_ABS_OFFSETn > 0x1f` verlangt zusätzliche Latenz; konfigurierte Ganzzyklen müssen zusätzlich berücksichtigt werden. RALAT betrifft den Zeitpunkt der Datenübernahme aus dem internen FIFO und dessen Board-/Chipverzögerungen.

| Phase / abgeleiteter Zustand | Historischer bd5-Pfad | Aktueller A/B/0013-Pfad |
|---|---|---|
| Initiale WALAT/RALAT | 3/3 | 0/5 |
| WL-interne Maximierung | 3/7 | 3/7 |
| WL-Ende | Keine ursprüngliche PHY0-Latenzrücksicherung | Rücksicherung des gesicherten Wertes nach PHY0 |
| DQS-Ende | Gesicherter, bereits maximierter Wert nach PHY1 | Gesicherter Wert nach PHY0 |
| Abgeleiteter PHY0-Endzustand | 3/7 | 0/5 |
| Programmierbarer MDMISC-Anteil initial → Ende | `000306c0 → 000307c0` | Upstream `00001740`; mit BI0/0011 `00000740`, jeweils restauriert |

Diese Endanteile sind Code-/Binärableitungen, keine vollständigen Registerreadbacks. MDMISC enthält auch Statusanteile. Der aktuelle PHY0-Restore passt zur dokumentierten DDR3-Core-Zuständigkeit; der verbliebene Kommentar „MMDCP1“ beweist keinen falschen Registerzugriff. Relevant ist die Wertdifferenz, deren tatsächlicher Effekt offen bleibt.

SOURCE-05 bestätigt die aktuellen Restore-Stores im 0013-Binärlayout bei `0x00909b74` für WL und `0x00909d8c` für DQS, jeweils nach PHY0 `+0x18`. Die WL-Lane-Korrektur mit Schwelle `0x148` ist ebenfalls bereits im aktuellen Binärcode vorhanden. Eine fehlende Übernahme dieser bekannten Korrektur ist somit keine gestützte alleinige Erklärung.

**Bewertung E6/E8:** WALAT0 bleibt eine konkrete herstellerseitig begründete Prüfspur. Ohne finale Lane-/Zykluswerte und gemessenes MDMISC desselben Boots ist weder ein konkreter WALAT-Verstoß noch eine erforderliche Einstellung berechenbar. Ebenso ist „RALAT5 ist zu klein“ für diese Novena nicht bewiesen.

### MPODTCTRL und MR1/MR2 nach Phasen

| Gegenstand / Phase | Historisch bd5 | A/B | 0013 |
|---|---|---|---|
| Parameter `rtt_nom`, `rtt_wr` | 2, 2 | 1, 0 | 2, 0 |
| MPODTCTRL beider PHYs, `+0x818` | `00011117` | `00022227` | `00011117` |
| Initiales DRAM MR1 | `0040` | `0004` | `0040` |
| WL-Eintritt, MR1 | `0080` | `0080` | `0080` |
| WL-Austritt, CS0 MR1 | fest `0004` | fest `0004` | fest `0004` |
| Initiales MR2 ohne SRT | `0408` | `0008` | `0008` |
| MR2 bei historischem SRT=1 | `0488` möglich | fester aktueller Pfad ohne diesen SPD-Wert | unverändert zu B |

Quelle: SOURCE-04/05; historische Initialisierung in `arch/arm/cpu/armv7/mx6/ddr.c:456–551`, aktuelle in `arch/arm/mach-imx/mx6/ddr.c:1471–1586`. Die Tabelle beschreibt programmierte Kommandos; tatsächliche DRAM-Mode-Register wurden nicht ausgelesen.

Nach RM6 §44.12.37 kodiert `00011117` viermal 120 Ω, `00022227` viermal 60 Ω. Im unteren Nibble 7 bleibt aktive Read-CS-ODT aus, passive Read-CS-ODT sowie aktive/passive Write-CS-ODT sind eingeschaltet. Diese MMDC-interne Terminierung ist von DRAM-interner RTT_NOM/RTT_WR über MR1/MR2 zu trennen.

Der Zahlenwert `rtt_nom=1` ergibt hier MR1 `0004` beziehungsweise RZQ/4, Wert 2 MR1 `0040` beziehungsweise RZQ/2. `rtt_wr=0` deaktiviert dynamische Schreibterminierung. Die abweichenden Boardkommentare sind keine zuverlässige Kodierungsquelle; das Datenblatt des aktuellen DRAMs fehlt.

Patch0013 stellt den historischen Gesamtzustand nicht wieder her: MR2 bleibt aktuell, WALAT/RALAT bleiben aktuell, und der WL-Austritt schreibt auf CS0 erneut MR1 `0004`. Eine Wiederherstellung von MR1 `0040` nach WL ist nicht belegt. Bei einem historischen Zwei-Rank-Fall darf die für CS0 gelesene Folge nicht pauschal auf alle Ranks übertragen werden. Zwei PHYs bei 64 Bit beweisen keine zwei Ranks.

### DQS-Korrektur 0xc0/0x80 und Read-Kalibrierung

Historischer bd5-Code und untersuchter aktueller Pin berechnen aus den oberen MPDGHWST-Grenzfeldern denselben Abzug `0xc0`. `0xc0` entspricht ¾ T, `0x80` ½ T. Bei gleichem oberen Grenzwert verschiebt der kleinere Abzug das resultierende Gate rechnerisch um ¼ T nach hinten. Das ist eine Berechnung, keine gemessene Novena-Takt- oder Fensterbreite.

Der lokale RM6-Text §44.11.3.1.2, Schritte 34/35, stützt laut SOURCE-04 unmittelbar die Halbzykluskorrektur `−0x80`; die Felddefinitionen stehen in §44.12.46/47. Auch RM Rev. 2 von 2014 nennt nach SOURCE-03 bereits die halbe Periode. Die Differenz darf deshalb nicht allein mit einem erst später geänderten RM erklärt werden. Die in Commit-/Mailtexten zitierte AN4467-Gegenposition wurde in SOURCE-03–07 nicht durch ein direkt gelesenes AN4467-Original qualifiziert.

Der spätere Mainline-Commit `cec2f200b4bfcc466e4a83196ed5fdd54678c1c7` (Autor 25.06.2026, Integration 27.07.2026) ändert zwei Subtraktionen auf `0x80`. Er liegt nach dem Projektpin. Originalpatch und externer Testbericht sind in Abschnitt 8 identifiziert. Der DH-electronics-Test betrifft andere Hardware. Da auch bd5 `0xc0` verwendet, erklärt diese Differenz allein keinen historischen GOOD-/aktuellen FAIL-Unterschied. Finale Novena-MPDGHWST-/MPDGCTRL-Werte fehlen.

Der historische Read-Kalibrierungspfad enthält zusätzliche Vergleichswert-/Delayinitialisierung und einen Read-spezifischen Dummy Write. Patch0012 ergänzt diesen Dummy Write im aktuellen Pfad. Dessen negativer A-B-B-A-Test begrenzt die isolierte Fixhypothese, widerlegt aber nicht den Herstellerablauf oder mögliche Wechselwirkungen.

## 7. SPD und DIMM

Historisch wird SPD an Adresse `0x50` gelesen. Der bd5-Pfad fällt bei Lese- oder SPD-Prüffehler auf `default_spd` zurück. Ein historisch erfolgreicher Boot beweist daher keine erfolgreiche Auswertung des realen EEPROMs. Adresse `0x50` allein identifiziert keinen I2C-Bus.

Das Ersatzarray in `H/u-boot-novena/board/kosagi/novena/spd.h` umfasst 256 Byte. SHA256 der dekodierten Arraybytes laut SOURCE-05: `fe5eb873cc6f2ec1d1d72875f1050cdc11279a186dc82d1ca3e3f58a6c5984bd`. Dieser Hash bezeichnet die Arraybytes, nicht die C-Datei. Die enthaltene Zeichenfolge lautet `S9905469-145.A00LF`.

| Merkmal | Historischer SPD-/Ersatz-SPD-Pfad | Aktueller fester Boardpfad |
|---|---|---|
| Ranks / CS | Aus SPD; Ersatzarray rechnerisch ein Rank | `ncs=1` |
| Gerätebreite / Bus | Ersatzarray x8, 64-Bit-Modul | `cfg.width=64`, `dsize=2`; keine unabhängige Chipbreitenerkennung |
| Dichte / Kapazität | Ersatzarray 4-Gbit-Chips, rechnerisch 4 GiB | `cs_density=32` als 4-GiB-CS-Adressbereich; keine Modulerkennung |
| Row / Column / Banken | Ersatzarray 16/10/8 | fest 16/10/8 |
| Page Size | Ersatzarray rechnerisch 1 KiB | fest 2 KiB |
| Datenrate / Timingzweig | SPD-1600-Zweig möglich | Eingang 1600, vor Timingberechnung auf 1066-Klasse begrenzt |
| Rechentakt i.MX6Q | 528 MHz | 528 MHz |
| tRCD / tRC / tRAS | Aus Ersatzarray und historischem Algorithmus 12,72/46,32/33,72 ns | fest 13,75/48,75/35 ns |
| SRT / RTT | SPD beziehungsweise feste Werte innerhalb der SPD-Funktion | feste Boardwerte |

Die historischen Zeiten sind Ergebnisse der ganzzahligen MTB-Skalierung einschließlich `+1`, keine neu qualifizierten Modulspezifikationen. Derselbe Rechentakt beweist keine gleichen Timingzweige, etwa für tFAW/tRRD. Der Strukturbezeichner `elpida_4gib_1600` identifiziert kein tatsächlich eingebautes Elpida-Modul.

SOURCE-04 benennt zusätzlich eine historische Schnittstelleninkonsistenz: Der SPD-Pfad setzt `cfg.width` auf die Gerätebreite, übergibt dieses Feld aber an die IO-Konfiguration mit Breitenentscheidungen `>=32/64`; aktuell wird explizit 64 übergeben. Das verhindert eine pauschale Gleichsetzung des historischen Pfads mit einer vollständig korrekten Referenzinitialisierung. Es beweist keine aktuelle Ursache.

**Fehlender Nachweis:** Ein dem aktuellen Testaufbau eindeutig zugeordneter SPD-Dump, eine Modul-/Chipkennung, reale Rankzahl sowie ein passendes Modulfoto oder Datenblatt wurden nicht identifiziert. Historische Beispielmodule und das Ersatzarray schließen diese Lücke nicht. Der begrenzte Suchbefund beweist nicht, dass unter beliebig benannten Bildern grundsätzlich kein Modulfoto existiert.

## 8. Historische Mailinglisten

### Umfang und Suchmethode

Der Katalog `R/communication-archive/indexes/u-boot-messages-v1.sqlite` enthält 633333 Datensätze: Segment 0 mit 526627, Segment 1 mit 106706. Die Tabellen `messages` und `references_map` enthalten Metadaten und Referenzbeziehungen; Nachrichtentexte liegen als `<Commit>:m` in `R/communication-archive/mailinglists/u-boot/segment-{0,1}.git`.

SOURCE-03 belegt Indizes auf Message-ID, Betreff und Referenz-ID, aber keine FTS-Tabelle. Message-ID allein besitzt dort keinen deklarierten Eindeutigkeitsschutz. Für reproduzierbare lokale Fundstellen werden deshalb Segment, Git-Objekt und Pfad `m` zusammen angegeben. Ein Mailarchiv-Commit ist kein U-Boot-Quellcommit.

Die historischen SQLite-Abfragen verwendeten `mode=ro&immutable=1`, teilweise zusätzlich `query_only` und `temp_store=MEMORY`. Ein früher Aggregatversuch in SOURCE-03 endete mit `disk I/O error`; folgende Speicherabfragen gelangen. Das ist kein Nachweis einer defekten Datenbank und keine durchgeführte Datenbankintegritätsprüfung.

| Untersuchung | Tatsächlich dokumentierter Inhaltsumfang | Grenze |
|---|---|---|
| SOURCE-03 | Alle 231 Novena-Betrefftreffer im Originaltext; weitere ausgewählte DDR-Mails | Allgemeine Suchzählungen betreffen Betreffzeilen, nicht alle Nachrichtentexte |
| SOURCE-04 | Gezielte Register-/Quell- und Mailvergleiche | Keine vollständige Mailarchiv-Volltextprüfung |
| SOURCE-05 | Schwerpunkt Binär-/Image-/Testprovenienz | Kein Anspruch auf Gesamtprüfung der Kommunikation |
| SOURCE-06 | 244 ausgewählte Originalmails zu Novena, WALAT beziehungsweise i.MX6-WL; keine fehlgeschlagenen Blobzugriffe in dieser Auswahl | Überschneidung mit den 231 ist möglich; Zahlen nicht addieren |
| SOURCE-07 | Konsolidierung und erneute gezielte Schlüsselquellen | Kein neuer Nachweis flächendeckender Mailprüfung |

Eine Volltextauswertung aller 633333 Nachrichten ist nicht belegt. Betreffauswahl verfehlt Mails, die Novena oder den relevanten Mechanismus nur im Nachrichtentext nennen. Beispielsweise ergab RZQ null Betrefftreffer, aber 15 Treffer innerhalb der 231 Novena-Nachrichtentexte. Null Betrefftreffer beweisen keine Abwesenheit des Themas.

### Tatsächlich geprüfte Schlüsselnachrichten

Die folgenden Originalobjekte sind in SOURCE-03–07 ausgewertet und wurden für diese Dokumentation erneut lokal gelesen. Datumsangaben stammen aus den Mailheadern. Die vollständigen Original-Message-IDs sind in Codeformat gesetzt, damit sie in Markdown nicht als HTML verschwinden.

| Datum | Segment / Git-Objekt, jeweils Pfad `m` | Original-Message-ID | Inhalt und Evidenzgrenze |
|---|---|---|---|
| 16.12.2015 | 0 / `5b62a7cee35745e388624d5bfc35c5d5351908b3` | `<1450276807-8960-2-git-send-email-marex@denx.de>` | Enable DDR calibration on Novena; zusammen mit Terminierungs-, Latenz- und Timingänderungen. Quellpatch, keine Messung |
| 22.12.2015 | 0 / `5aafb61b08a186922fd1f2b1297468166e5e0c7e` | `<201512220226.41529.marex@denx.de>` | Diskussion zusammengehöriger Parameter und Fehlerbehandlung; kein WL-Endwertsatz |
| 21.06.2016 | 0 / `202c541c9e7939e61350f41059b4e1cca27c02cd` | `<1466534502-17233-3-git-send-email-eric@nelint.com>` | RFC: Novena übergibt Kalibrierungsparameter; Entwurf ist kein Integrations-/Bootnachweis |
| 21.06.2016 | 0 / `4673d1e91a1a65802d0cea06ba1c8f3b0ba15be4` | `<1466534502-17233-6-git-send-email-eric@nelint.com>` | RFC: Übergabe von `mx6_ddr_sysinfo`; keine Hardwarewerte |
| 30.03.2018 | 0 / `ec8fcb989d8f7df3da1b4ef72123c304d0e18b1e` | `<20180330010443.18418-1-marex@denx.de>` | WL-Ergebnisse nahe `0x17f`, Korrektur oberhalb `0x148`; Fremdbericht, Korrektur aktuell bereits binär vorhanden |
| 06.04.2022 | 0 / `245acf0b01d969a8f981e290a98d6f2c2f377a61` | `<20220406115325.98086-2-francesco.dolcini@toradex.com>` | RALAT/WALAT speichern/restaurieren; erforderliche Zyklus-/WALAT-Wahl geschieht nicht automatisch |
| 14.06.2023 | 0 / `fd3a65506a16935659952b7e6dbed106abdce2fb` | `<20230614090137.17626-1-francesco@dolcini.it>` | Colibri-iMX6-Bitfehler oberhalb 60 °C; `MDMISC 00081740 → 000b17c0`. Andere Hardware |
| 25.06.2026 | 1 / `0cee938a29084922210600126fa5dd822dcb43df` | `<20260625141755.16568-1-marex@nabladev.com>` | DQS-Abzug `0xc0 → 0x80`; Herstellerverweise im Patch ersetzen keine direkt gelesene AN4467 |
| 03.07.2026 | 1 / `0fe92f2415c0b26c180967abb255dd1ad6ac5d65` | `<cb2b7b7f20a1456c85cc99791b302305@dh-electronics.com>` | Klimakammertest mit DH electronics i.MX6Solo SoM; kein Novena-Test |

Der Bericht vom 03.07. nennt Winbond W634GU6RB sowie Intelligent Memory IM4G16D3FABG-125I, Samsung K4B4G1646E-BMMA und Etron EM6HE16EWAKG-10IH. Das sind berichtete externe Testbedingungen, keine Identität unseres DIMMs.

Der Novena-Enable-Quellcommit lautet `89d485940106c095ec69ecb12a78a42e096dece7`, Autorendatum 16.12.2015, Commitdatum 02.02.2016. Die vorhandene historische Graphprüfung bestätigt ihn ab v2016.03-rc2/v2016.03, nicht in den geprüften v2016.01/v2016.03-rc1. Die nun direkt gelesene Patchmail schließt die frühere Mailzugriffslücke; sie behauptet nicht rückwirkend einen erfolgreichen Abruf damals fehlender Partial-Clone-Blobs.

## 9. Kosagi-Forum, UART und IRC

### Forumumfang und Datumsgrenzen

`K/metadata/sources.sqlite` katalogisiert 343 Captures zu 315 Threads. Die Queue enthält 343 `stored`, 121 `metadata_only` und zwei `failed`. Die Capture-Zeitspanne 12.01.2015–06.02.2023 bezeichnet Archivaufnahmen; sie ist keine lückenlose Beitrags- oder Gesprächsabdeckung. `retrieved_utc` bezeichnet wiederum den lokalen Abruf, nicht das historische Beitragsdatum.

SOURCE-03 durchsucht die 343 lokalen Captures; SOURCE-06 bezieht alle katalogisierten Snapshots in die gezielte Inhaltsprüfung ein, SOURCE-07 konkretisiert weitere UART-/IRC-Stellen. Threads 488 und 561 bleiben gemäß dem in SOURCE-07 herangezogenen Übergabestand vom 14.10. offen. Metadaten ohne gespeicherten Text sind keine geprüften Gesprächsinhalte.

### Historische UART-Überlieferungen

Alle folgenden Pfade sind relativ zu K. Es handelt sich um veröffentlichte Originalberichte mit eingebetteten UART-Ausgaben, nicht um unabhängig erhaltene vollständige Terminaldateien. Beitrag und Capture werden getrennt datiert.

| Kennung | Autor / Beitrag / historisches Datum | Lokale Originaldatei | Erhaltener Befund und Grenze |
|---|---|---|---|
| F191 | ballanux, Beitrag 28, 07.05.2015 | `raw/forum/thread-0191-20210417112726-82842d8cce02.html` | Laut Autor BusPirate-UART: SPL `gc5efead`, Speichertest-Pass, interne SD, U-Boot `g42ae9d5`, POR; danach PMIC-Fehler |
| F229 | ballanux, Beitrag 1, 21.05.2015, und Beitrag 6, 04.06.2015 | `raw/forum/thread-0229-20220527124039-e1b332756f1c.html` | Zwei getrennte Bootausgaben mit unterschiedlichen SPL-Versionen; Speichertest-Pass, danach PMIC-Fehler; kein einheitlicher Lauf |
| F303 | dileepvr, Beitrag 12, 01.09.2015 | `raw/forum/thread-0303-20190919130634-0c05813f347d.html` | Früher Debug-UART-Ausschnitt einer Screen-Sitzung, SPL `g0e39e8c`, Speichertest-Pass |
| F421 | frostsnow, Beitrag 10, 21.04.2017 | `raw/forum/thread-0421-20220125140414-7a8e166178e0.html` | bd5, SPD-Prüffehler und Ersatz-SPD, Speichertest-Pass, interne SD und U-Boot proper; Resetursache WDOG, kein POR-Beleg |

Keiner dieser Ausschnitte liefert einen zusammengehörigen Vierer-Satz finaler WL-Werte oder eine geschlossene Bindung an SPL-SHA256, DIMM und MDMISC. Speichertest-Pass und nachfolgender PMIC-Fehler sind getrennte Befunde; ein vollständiger erfolgreicher Systemboot folgt daraus nicht automatisch.

Weitere historische SPD-Berichte: Thread 223, Beitrag 1 von `job`, 16.05.2015, `raw/forum/thread-0223-20220703214010-49285b7efad6.html`, dokumentiert `Fatal DDR error 0x03` bei SPD-Korruption und berichtete Erholung nach Wiederherstellung. Thread 210, Beiträge vom 05.05.2015, `raw/forum/thread-0210-20220703200546-5236606aa084.html`, enthält SPD-Lesefehler/`0x55` und einen Bericht über blockierte gemeinsam genutzte Leitungen. Diese historischen Ursachen haben eine andere Signatur und erklären den heutigen festen Konfigurationspfad nicht direkt.

### Drei getrennte IRC-Belegarten

| Kennung | Datum / Ort | Lokale Originaldatei relativ zu K | Aussage und Grenze |
|---|---|---|---|
| IRC-ANN | bunnie, 25.05.2014, Thread 4 | `raw/forum/thread-0004-20150112231752-c4382d665fa0.html` | Ankündigung, bunnie und xobs seien häufig auf OFTC `#kosagi`; kein Chatlog |
| IRC-HELP | Robb W., 21.05.2015, Beitrag 11, Thread 216 | `raw/forum/thread-0216-20221126130907-4e13a2c5018c.html` | Bericht über IRC-Hilfe bei SPD-/SPL-Recovery über `irc://irc.oftc.net:7000/kosagi`; Gespräch selbst fehlt |
| IRC-QUOTE | Jookia, 12.09.2019, Beitrag 6, Thread 529 | `raw/forum/thread-0529-20221126221429-1c66100d2741.html` | Fünf zitierte Zeilen von MadHacker, 19:07–19:12, über Batterie-Entladestrom; tatsächlicher überlieferter Gesprächsausschnitt |

IRC-QUOTE präzisiert ältere pauschale Aussagen über fehlende Gesprächsinhalte: Ein Ausschnitt ist überliefert. Sein tatsächliches Gesprächsdatum ist nicht unabhängig vom Veröffentlichungsdatum belegt; Zeitzone und Vollständigkeit fehlen. Er enthält keine DDR-, WL- oder WALAT-Messwerte.

Die lokale Wiki-Kopie `P/research/01-original-novena-docs/sources/kosagi-novena-main-page.html` nennt ebenfalls OFTC und `#kosagi`. Ein eigenständiges historisches IRC-Vollarchiv, ein Export mit belastbarem Datumsbereich oder ein konkreter öffentlicher Logarchivverweis wurde im geprüften Bestand nicht identifiziert. Channel-Ankündigung, Hilfsbericht und zitierter Ausschnitt dürfen nicht als Vollarchiv zusammengefasst werden.

## 10. Archivinventur

### Methode und erreichte Lesetiefe in SOURCE-07

Die Inventur baut auf den Dateilisten und `I/katalog/novena-katalog-v2.sqlite` auf. Deren Grundlage ist der 09.10.2026; deshalb ergänzte SOURCE-07 eine aktuelle Dateinamensinventur der damals sieben freigegebenen Wurzeln: P, B, R, H, I sowie `/home/loomit/novena-system` und `/home/loomit/novena-linux-old`. Die beiden zuletzt genannten Bestände sind hier ausschließlich als berichteter historischer Prüfumfang aufgeführt, nicht als neu durchsuchte Quellen dieses Auftrags.

SOURCE-07 findet 53 Dateien mit den verlangten Archiv-/Kompressionsendungen: 18 Projekt-/Sicherungsarchive A01–A18 und 35 Software-Testdateien beziehungsweise komprimierte Ressourcen. Keine `.7z`- oder `.zst`-Dateien wurden gefunden; Verzeichnis-Symlinks wurden nicht verfolgt. Endungsauswahl und ausgeschlossene Symlinkziele begrenzen die Vollständigkeit.

Alle 18 Archivverzeichnisse waren lesbar. Passende reguläre Textmitglieder mit ausgewählten Endungen unter 2 MB wurden direkt in den Arbeitsspeicher gelesen, ohne Dateisystemextraktion. Das große SD-Image in A03 wurde übersprungen. Archivlesbarkeit ist keine vollständige Integritäts-, Herkunfts- oder Inhaltsqualifikation aller Mitglieder. Für die folgende Übersicht werden Größen aus SOURCE-07 übernommen; nicht dort belegte vollständige Archivhashes werden nicht ergänzt.

### A01–A18

| ID | Archivpfad | Format / Byte | Relevanter Inhalt und Prüfgrenze |
|---|---|---|---|
| A01 | `P/research/01-original-novena-docs/sources/novena_pvt2.zip` | ZIP / 7172816 | Hardwarequellen, 19 Mitglieder; kein serieller Namenskandidat |
| A02 | `P/research/01-original-novena-docs/sources/gerbers-novena_pvt2_a.zip` | ZIP / 954306 | Gerber, 30 Mitglieder; kein serieller Namenskandidat |
| A03 | `B/nixos-display-working-2026-09-11.tar` | TAR / 6618071040 | Golden-Sicherung, 1409 Mitglieder; Image, Kernel, Metadaten; Imageinhalt nicht untersucht |
| A04 | `B/novena-i2c-0006-evidence-e904173c-2026-09-15.tar.gz` | TAR.GZ / 48459 | Historische Evidenz, Boot-ID/Kernel-/Systemtexte; kein WL-Fund |
| A05 | `B/historical-software-before-linux-5.7-2026-09-15.tar.gz` | TAR.GZ / 421570 | Historische `ddr3.c`, `novena_spl.c` und Quelltextauszüge; Code, keine Endwertaufzeichnung |
| A06 | `B/novena-forschungsdokumente-20261003-131843.tar.gz` | TAR.GZ / 183749 | Forschungsstand 03.10., Dokumente 18–22; Analyse-/Codezitate |
| A07 | `B/nixos-display-working-2026-09-11/metadata/nixos-novena-image-f26464f.tar.gz` | TAR.GZ / 8633939 | Repository-/Referenzsicherung, `boot/reference/novena-imx6-spl.bin`, `u-boot-dtb.img` |
| A08 | `B/nixos-display-working-2026-09-11/test-logs/novena-coldboot-tests-2026-09-12.tar.gz` | TAR.GZ / 108391 | Sieben Kernelprotokolle; keine SPL-WL-Ausgaben |
| A09 | `B/0004-no-single-master-2026-09-13/novena-0004-no-single-master-test-2026-09-13.tar.gz` | TAR.GZ / 156584 | Coldboot-/Rebind-Kernel- und Ergebnisprotokolle |
| A10 | `B/0005-no-single-master-2026-09-13/novena-0005-no-single-master-test-2026-09-13.tar.gz` | TAR.GZ / 31454 | Entsprechende Kernel-/Ergebnisprotokolle |
| A11 | `B/nixos-novena-image-403cc94-2026-09-18/nixos-novena-image-403cc94.tar.gz` | TAR.GZ / 17419235 | Repositorystand, Referenzdateien, Evidenz, Quelltextauszüge |
| A12 | `B/nixos-novena-image-3790db6-2026-09-18/nixos-novena-image-3790db6.tar.gz` | TAR.GZ / 21727894 | Weiterer Repositorystand; bekannte Quellen-/IRC-Verweise |
| A13 | `B/codex-preinstall-2026-09-30/test-logs.tar.gz` | TAR.GZ / 13784 | P_EXT-Logs 22./23.09. und Buildlogs; bekannte serielle Dateien byteidentisch |
| A14 | `B/project-index-v02-before-v03-60785fc2/project-index-v02.tar` | TAR / 20480 | Projektindex, keine seriellen Mitglieder |
| A15 | `B/git-cleanup-checkpoints/20261012T140244Z-wZj0HDvC/repository.tar` | TAR / 175052800 | Repository-/Git-Sicherung mit U-Boot und Forschung; nicht sämtliche historischen Git-Blobs inhaltlich geprüft |
| A16 | `B/git-cleanup-checkpoints/20261012T140244Z-wZj0HDvC/restore-test/nixos-novena-image/research/01-original-novena-docs/sources/novena_pvt2.zip` | ZIP / 7172816 | Restore-Kopie von A01 |
| A17 | `B/git-cleanup-checkpoints/20261012T140244Z-wZj0HDvC/restore-test/nixos-novena-image/research/01-original-novena-docs/sources/gerbers-novena_pvt2_a.zip` | ZIP / 954306 | Restore-Kopie von A02 |
| A18 | `B/borg-private/workflow-review-20261017-092317.tar.gz` | TAR.GZ / 2314 | Vier Workflow-Mitglieder, kein serieller Namenskandidat |

Datumsbestandteile der Archivnamen belegen zunächst die Benennung, nicht automatisch einen unabhängig qualifizierten Erstellungszeitpunkt.

### Weitere 35 Dateien und verbleibende Inhaltslücken

20 Dateien sind U-Boot-Test-/Bildressourcen unter `R/u-boot-{v2020.07,current,v2026.07-novena-external,v2026.07-novena-external-nogit-test}/`, darunter `fitimage.bin.gz`, `ifwi.bin.gz`, `armbian.bmp.xz` und komprimierte Logos. Bei großen dekomprimierten Testdateien war die Lektüre auf 1048577 Byte begrenzt.

15 Dateien sind Perl-Testarchive unter `H/openbsd-src/gnu/usr.bin/perl/cpan/`, insbesondere Archive-Tar, IO-Compress und IPC-Cmd. `bad-efs.zip` scheiterte an der UTF-8-Namensdekodierung; verschlüsselte ZIP-Inhalte wurden nicht geöffnet. Diese Bestände lieferten keinen identifizierten Novena-Kalibrierungsmitschnitt.

Die seriellen P_EXT-Mitglieder von A13 sind laut SOURCE-07 byteidentisch mit den Repositorydateien. Besonders wichtig ist `P/test-logs/p-ext-2026-09-22/p-ext-boot-01.log`, SHA256 `506f8eaf409b8d9a6e40fced1da822df43b865df507d08d094d9106046e7debb`: Der Mitschnitt enthält getrennte Bootabschnitte für SPL 2026.07 und anschließend bd5. Der historische Speichertest-Pass gehört zum späteren historischen Abschnitt und darf dem vorausgehenden aktuellen SPL-Lauf nicht zugerechnet werden.

SOURCE-07 schließt damit einen begrenzten Archivnamens-/Textprüfumfang, aber nicht alle Dateisysteminhalte großer Images, alle Git-Blobs, alle endungslosen/binären Mitschnitte oder nicht eingehängte beziehungsweise nicht katalogisierte Bestände. Ein neuer bd5/r7-DEBUG-Vierer-Wertesatz wurde nicht gefunden.

## 11. Negative Tests

Hier sind tatsächlich negative Hardwareversuche von negativen Suchergebnissen und statischen Ausschlüssen getrennt.

| Kategorie / Gegenstand | Ergebnis | Tragfähige Aussage und Grenze |
|---|---|---|
| Hardware: isoliertes BI_ON-Experiment | Laut bestätigtem Projektcheckpoint weiterhin ZERO | BI_ON-Umschaltung allein nicht als ausreichender Fix gestützt; ursprüngliche Durchführung hier nicht neu qualifiziert |
| Hardware: Patch0012 | A-B-B-A vom 07.10. vollständig ZERO | Read-spezifischer Dummy Write allein unter diesen Bedingungen nicht ausreichend; Kombinationen bleiben offen |
| Hardware: Patch0013 | DDR-08H vom 14.10. weiterhin ZERO | Isolierte RTT_NOM-Änderung beseitigt die Signatur nicht; kein Test der gesamten historischen Terminierung |
| Statische/zeitliche Eingrenzung: erst MMC verursacht ZERO | Q1/C1 liegen vor MMC | Nachgelagerter MMC-Fehler erklärt die frühere Signatur nicht |
| Statische/zeitliche Eingrenzung: Heap-Clear verursacht Q1 | Q1 vor Heap-`memset`, C1 vor `mem_malloc_init()` | Der spätere Heap-Clear ist keine Erklärung dieser früheren Aufnahmen |
| Binärprüfung: fehlende bekannte WL-Lane-Korrektur | Korrektur oberhalb `0x148` aktuell vorhanden | Pauschale Nichtübernahme nicht gestützt; konkrete Laufzeitwirkung bleibt offen |
| Suche nach finalen WL-/MDMISC-Werten | Kein geeigneter zusammengehöriger Satz gefunden | Begrenzter Negativbefund im geprüften Quellenraum, kein Nichtexistenzbeweis |
| Suche nach IRC-Vollarchiv | Kein eigenständiges Vollarchiv identifiziert | IRC-QUOTE bleibt trotzdem ein tatsächlicher Ausschnitt |

Weitere Grenzen aus SOURCE-06: Der erste Patch0009-Mitschnitt vom 05.10. ist leer und trägt keine Beobachtung. Mehrere kurze Patch0010-/BI-Dateien sind byteidentisch (SHA256 `b220f0b9af12fa57c1828792fab5806fffaff3910fc6e4410c9382616ffb12b4`); Dateiidentität beweist weder mehrere unabhängige Ereignisse noch deren historische Gleichheit. Restore-Kopien sind keine zusätzlichen Messläufe.

## 12. Evidenzmatrix

### Verbindliche Klassen

| Klasse | Bedeutung | Beispiel | Unzulässige Aufwertung |
|---|---|---|---|
| E1 | Gemessene Hardwarewerte beziehungsweise aufgezeichnete laufbezogene Ausgaben | Q1/C1 ZERO, WL-/DQS-Status, D32 im Originalrohlog | Aus Status-/Rückgabewerten keinen allgemeinen DDR-PASS ableiten |
| E2 | Binärbefunde | SPL-Hash, Einbettung, Instruktionen, Byteunterschied B/0013 | Keine automatische Bindung an jeden historischen Boot |
| E3 | Quellcodebefunde | Initialtabellen, MDMISC-Restore, SPD-Fallback, MR-Kommandos | Schreibabsicht und abgeleiteten Endzustand nicht als Messung ausgeben |
| E4 | Herstellerdokumentation | RM6-Felddefinitionen, WALAT-Regel, DQS-Halbzyklus | Keine automatische Auslösung oder Anwendbarkeit im konkreten Lauf |
| E5 | Historische Fremdberichte | F191/F229/F303/F421, Colibri-/DH-Berichte, IRC-QUOTE | Keine Gleichsetzung von Board, DIMM, Bootpfad oder Binäridentität |
| E6 | Hypothesen | Unpassender WALAT-Endzustand, Terminierung, Geometrie, physischer Pfad | Plausibilität ist kein Root-Cause-Nachweis |
| E7 | Negative Ergebnisse | Isolierte Fixversuche ohne Wirkung; begrenzte erfolglose Suche | Kein universeller Ausschluss von Wechselwirkungen oder historischen Quellen |
| E8 | Fehlende Nachweise | Finale Lane-Werte, MDMISC desselben Boots, aktuelles SPD | Fehlende Werte nicht aus Initialtabellen oder anderen Läufen ergänzen |

### Aussagebezogene Provenienz und Grenzen

| Aussage | Klasse | Original / Datum / Zuordnung | Quellenbeleg | Grenze |
|---|---|---|---|---|
| ZERO vor MMC und D32 | E1 | Vier RUN-Rohlogs, 07.10., A/B; DDR-08H, 14.10., 0013 | Abschnitt 4 mit SHA256; SOURCE-03/05/06/07 | Rücklesediskrepanz, keine physische Einzelursache |
| A/B/0013-Byteidentitäten | E2 | Nix-Store-SPLs und Images | Abschnitt 5, SOURCE-05 | Dateiidentität und Medien-/Bootidentität getrennt |
| 0013 am 13.10. gebaut | Historische Buildausgabe | DDR-07D, 13.10., cy3a… | Originalhash und Zeitraum in Abschnitt 5; SOURCE-07 | Kein zusätzlicher Build am 14.10. belegt |
| B/0013 unterscheiden sich in einem Datenbyte | E2/E3 | Offset `0xc649`, `01→02` | SOURCE-05 „Binäranalyse“ | Gleiche Instruktionsbytes bedeuten nicht gleiche datenabhängige Ausführung |
| Historischer bd5-Paket-SPL | E2 | Raw-Masterbereich `0x3512e000`, 44032 Byte | Bereichshash in Abschnitt 5; GOOD-PROVENANCE-01 | Keine geschlossene Zuordnung jedes GOOD-Boots |
| WL-Startwerte vorhanden | E3 | Historischer und aktueller Boardcode | Abschnitt 6; SOURCE-04/06 | Keine finalen WL-Messwerte |
| Effektiv 3/7 gegenüber 0/5 | E2/E3 | Historischer bd5- und aktueller Restorepfad | SOURCE-04/05; Abschnitt 6 | Ableitung, MDMISC-Readback fehlt |
| WALAT hängt von WL-/Ganzzykluswerten ab | E4 | IMX6DQRM Rev. 6, Mai 2020 | PDF-SHA256 und §§ in Abschnitt 6 | Konkrete Verletzung nicht bestimmbar |
| ODT und MR-Zustände unterscheiden sich phasenweise | E2/E3/E4 | bd5, A/B, 0013 | Abschnitt 6; SOURCE-04/05 | Kein ausgelesenes finales DRAM-MR |
| DQS `0x80` herstellerseitig gestützt | E3/E4/E5 | RM6, Patch 25.06., Testbericht 03.07.2026 | Abschnitte 6/8 mit Message-IDs und Objekten | Kein getesteter Novena-Fix; bd5 ebenfalls `0xc0` |
| Ersatz-SPD eindeutig identifizierbar | E3 | Historisches `spd.h`, dekodierte 256 Byte | Arrayhash in Abschnitt 7; SOURCE-05 | Kein aktueller EEPROM-Dump |
| Historische UART-/IRC-Überlieferungen vorhanden | E5 | Vier Forumthreads und drei IRC-Belegarten | Abschnitt 9; SOURCE-03/07 | Fremdberichte, kein WL-Endwertsatz |
| Archivprüfumfang 18 + 35 | Inventur/E7 | SOURCE-07, 17.10. | Abschnitt 10; Originalprotokollhash in Abschnitt 2 | Begrenzte Text-/Endungsprüfung, keine Vollqualifikation |
| DDR-/SPL-Ursache | E6/E8 | Gesamte Evidenz | Abschnitte 11/13 | Weiterhin unbewiesen |

## 13. Offene Fragen

1. **Finale MPWLDECTRL-Werte:** Für einen konkreten Boot fehlen MPWLDECTRL0/1 beider PHYs samt acht Lane-Ergebnissen, Halb- und Ganzzyklusanteilen. Statuswörter und Startwerte schließen diese Lücke nicht.
2. **MDMISC desselben Boots:** Ohne tatsächlichen Endreadback ist der Code-Endzustand nicht laufbezogen verifiziert. Lane-Werte und MDMISC aus verschiedenen Boots wären keine belastbare gemeinsame WALAT-Bewertung.
3. **Weitere Kalibrierungsendwerte:** Finale MPDGCTRL-, MPRDDLCTL-, MPWRDLCTL- und passende Grenzwert-/Fensterbelege fehlen. Ein `ret=0` ersetzt sie nicht.
4. **Aktuelles DIMM/SPD:** Modul-/Chipidentität, SPD, Rankzahl, Datenblatt und belastbare Bindung an die Testhardware sind ungeklärt.
5. **Historische Bootprovenienz:** Nicht für jeden GOOD-Bericht ist SPL-Hash → Medium → Bootdatum → DIMM → Registerdump → erfolgreicher Speicherzugriff geschlossen. Paket-SPL und gemischte Logs genügen nicht.
6. **Terminierungsendzustand:** Der phasen- und CS-bezogene Gesamtzustand ist nicht als Hardwaredump erhalten. Der einzelne 0013-Test ist kein Test einer vollständigen historischen Rückkonfiguration.
7. **Herstellerquellen:** Direkt gelesene AN4467-Originalstellen und modulbezogene Betriebsbedingungen bleiben von Entwicklerzitaten getrennt; konkrete Errata-/Siliconzuordnung ist keine bewiesene Fehlerauslösung.
8. **IRC-Vollarchiv:** Ein eigenständiges Archiv ist nicht identifiziert; IRC-ANN, IRC-HELP und IRC-QUOTE liefern unterschiedliche, begrenzte Evidenz.
9. **Mailinglisten-Volltext:** Nachrichten außerhalb der bisherigen Auswahl wurden nicht vollständig geprüft. Die 231/244 sind kein Gesamtarchivabschluss.
10. **Forum und Archive:** Metadaten ohne Volltext, Threads 488/561, große Imageinhalte, historische Git-Blobs, endungslose/binäre Mitschnitte und nicht katalogisierte Bestände bleiben teilweise ungeprüft.
11. **DDR-/SPL-Root-Cause:** Zusatzlatenzen, Terminierung, Geometrie/Timing, Kalibrierungsvoraussetzungen und physischer Speicherpfad sind plausible Untersuchungsrichtungen. Keine davon ist als Ursache bewiesen; ein vollständiger eigenständiger externer Boot bleibt offen.

## 14. Priorisierte weitere Quellenarbeit

Die folgenden Empfehlungen betreffen ausschließlich bereits vorhandene lokale Quellen und lesende Analyse. Sie setzen weder einen neuen Build noch einen Patch, neue Registerzugriffe oder einen Hardwaretest voraus.

1. **Endwertsuche präzisieren:** In erhaltenen Protokollen ursprüngliche DEBUG-Dateinamen, Anhänge, Paste-Verweise und Bootsegmentgrenzen verfolgen. Insbesondere nach einem tatsächlichen bd5/r7-Vierer-Readback suchen. Die bereits erfolgte SOURCE-07-Inventur nicht pauschal wiederholen; nur die in Abschnitt 10 benannten Inhaltslücken gezielt untersuchen.
2. **Provenienzkette schließen:** Vorhandene Build-, Readback- und Bootprotokolle über Hashes und Zeitpunkte verbinden. Historischen Paket-SPL, internes MMC-Backup und einzelne GOOD-Bootabschnitte getrennt halten. Fehlende Kettenglieder ausdrücklich stehen lassen.
3. **Terminierungsphasen statisch vervollständigen:** Vorhandenen historischen und aktuellen Binärcode für MR1 nach WL, MR2, CS-Auswahl und MPODTCTRL vergleichen. Bereits vorhandene Binärbefunde verwenden; nicht auf einen neuen Build ausweichen, weil A/B/0013-ELF-/Map-Begleiter fehlen.
4. **Mailreferenzen gezielt erweitern:** Lokale `References`-/`In-Reply-To`-Ketten zur Novena-Kalibrierungsserie, Latenzrücksicherung und DQS-Korrektur verfolgen. Je Nachricht Segment, Objekt, Original-Message-ID und tatsächlich gelesenen Textumfang festhalten; Betreffsuche und Volltextprüfung getrennt zählen.
5. **Historische UART-/IRC-Belege weiter zuordnen:** F191/F229/F303/F421 als einzelne Überlieferungen behandeln; nach bereits lokal vorhandenen Anhängen oder Originaldateiverweisen suchen. IRC-QUOTE liefert einen Ausschnitt, aber keinen DDR-Beleg und keine Vollarchivdatierung.
6. **Vorhandene Modulbelege prüfen:** Lokal vorhandene SPD-Dateien, Inventurverweise und Bilder nur bei belastbarer Zuordnung zur aktuellen Hardware verwenden. Ersatz-SPD und historische Beispielmodule separat belassen.
7. **Nur bei vorhandenem zusammengehörigem Wertesatz rechnen:** Finale Lane-/Zykluswerte gegen die lokale NXP-WALAT-Regel und MDMISC desselben Boots auswerten. Fehlt dieser Satz weiterhin, bleibt die konkrete WALAT-Bewertung mit dem vorhandenen Material unentscheidbar.

Aus dem aktuellen Quellenstand wird keine neue Hardware-, Patch- oder Buildmaßnahme empfohlen. Die entscheidenden dafür noch offenen Belege sind in Abschnitt 13 benannt.

## 15. Integrität und Dokumentationsstand

Diese Datei ist eine Konsolidierung vom 17.10.2026 und keine neu durchgeführte DDR-Qualifikation. Die fünf SOURCE-Originaldateien wurden vollständig eingelesen und gehasht; die vier A-B-B-A-Rohlogs, DDR-08H und der erhaltene DDR-07D-Buildlog wurden zusätzlich lokal geprüft. Die neun in Abschnitt 8 aufgeführten Mailoriginale wurden erneut aus vorhandenen Git-Objekten gelesen. Archive, Herstellerunterlagen und Binärdisassemblierungen werden darüber hinaus mit der in SOURCE-03–07 dokumentierten Lesetiefe übernommen, ohne eine erneute Vollprüfung aller Bestände zu behaupten.

Vor dem Schreiben wurden Git-Zustand und vorhandene Nutzeränderungen lesend erfasst. Die Zieldatei war nicht vorhanden und wird exklusiv neu angelegt. Ein Inhalts-/Symlinkmanifest der vorhandenen Repositorydateien außerhalb `.git` wurde ausschließlich im Arbeitsspeicher erfasst, um nach der Anlage die Beschränkung auf diese eine neue Datei zu prüfen. Die abschließende Prüfung ist eine lesende Dokumentationskontrolle, kein Software- oder Hardwaretest.

Es wurden keine bestehenden Projektdateien überschrieben, keine Git-Mutationen, Netzwerkzugriffe, Builds, Patches, Hardwaretests, Blockgerätezugriffe oder Datenbankänderungen ausgeführt und keine Hilfsdateien außerhalb des Repositories angelegt. Git-Abfragen erfolgen ohne optionale Schreiblocks; lokale Git-Objektlesungen mit deaktiviertem Lazy Fetch. Diese Aussage betrifft die eigenen Aktionen dieses Dokumentationsauftrags, nicht sämtliche in den historischen Protokollen beschriebenen Abläufe und nicht eine forensische Garantie aller Dateisystemmetadaten des Hosts.

Der Integritätsnachweis der lokalen Protokolle ist vom laut Auftrag vorhandenen GitHub-Archiv getrennt. Die Remoteablage wurde nicht neu geprüft. Fehlende historische Originalausgaben, Endwerte und Hardwareidentitäten wurden nicht rekonstruiert oder durch plausible Zahlen ersetzt.

**Dokumentierter Abschluss:** SOURCE-03–07 sind für diesen Forschungsgegenstand konsolidiert; die DDR-/SPL-Root-Cause, finale Kalibrierungswerte, aktuelle DIMM-Identität und Teile der historischen Bootprovenienz bleiben offen.
