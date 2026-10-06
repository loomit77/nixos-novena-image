# Quellenverzeichnis – Novena / universelles NixOS-SD-Image

Stand der Erstfassung: 2026-10-01. Spätere Quellenaufnahmen und Statuspräzisierungen sind jeweils gesondert datiert; der Dokumentationsvorschlag vom 2026-10-04 enthält keine neue Onlineprüfung.

## Regeln

- `GEPRUEFT`: URL am angegebenen Datum online eingesehen; der Inhalt ist nicht automatisch auf unseren gepinnten Quellstand übertragbar.
- `HISTORISCH`: Quelle dokumentiert einen älteren Stand; mit aktuellen Quellen und Artefakten vergleichen.
- `OFFEN`: Link bekannt, Inhalt oder Zuordnung noch nicht überprüft.
- Für reproduzierbare Aussagen möglichst Commit, Tag, Revision, Abrufdatum und lokale Fundstelle ergänzen.
- Vor neuen Builds, Patches oder Hardwaretests erst `docs/TEST-LOG.md`, `docs/DECISIONS.md`, `docs/PROJECT-STATE.md` und dieses Verzeichnis abgleichen.

## U-Boot – Primärquellen und Dokumentation

| ID | Quelle | URL | Status | Relevanz |
|---|---|---|---|---|
| UB-001 | Offizielles U-Boot-Repository | https://github.com/u-boot/u-boot | GEPRUEFT 2026-10-01 | Versions- und Commitvergleich; für unsere Builds gepinnte Revision maßgeblich. |
| UB-002 | U-Boot Memory Management | https://docs.u-boot.org/en/latest/develop/memory.html | GEPRUEFT 2026-10-01 | SPL-Speicherphasen, SRAM/DRAM, BSS und Stack; allgemeine Darstellung, boardspezifisch abgleichen. |
| UB-003 | U-Boot Kconfig (main) | https://github.com/u-boot/u-boot/blob/main/Kconfig | GEPRUEFT 2026-10-01 | SPL-Malloc, frühe Allokation, CLEAR_ON_INIT; `main` ist kein Nachweis für 2026.07. |
| UB-004 | U-Boot malloc.h (Spiegel) | https://github.com/ARM-software/u-boot/blob/master/include/malloc.h | GEPRUEFT 2026-10-01 | Deklarationen von mem_malloc_start/end/brk; Spiegel und Branch beachten. |
| UB-005 | Offizielles U-Boot-Git | https://git.u-boot-project.org/u-boot/u-boot.git | LINK BEKANNT 2026-10-01 | Offizieller Git-Endpunkt; konkreten Stand festhalten. |

## Novena – historische U-Boot-Quellen

| ID | Quelle | URL | Status | Relevanz |
|---|---|---|---|---|
| NV-001 | Marek Vasut: DDR-Kalibrierung auf Novena, Patch 2/2 (2015) | https://www.mail-archive.com/u-boot@lists.denx.de/msg196451.html | GEPRUEFT 2026-10-01; HISTORISCH | Besonders wichtig: dokumentierte Aktivierung der DDR-Kalibrierung und Anpassung von Parametern für das Novena-Speichermodul. Keine belegte Ursache des aktuellen Falls 8B. |
| NV-002 | Novena SPL – Android/Google U-Boot-Spiegel | https://android.googlesource.com/platform/external/u-boot/+/refs/heads/android-tv-s-beta3/board/kosagi/novena/novena_spl.c | GEPRUEFT 2026-10-01; HISTORISCH | DDR-I/O, MMDC-Kalibrierungswerte und DDR-Geometrie; Revision mit aktuellem gepinnten Stand vergleichen. |
| NV-003 | Novena SPL – historischer U-Boot-Spiegel | https://nest-open-source.googlesource.com/nest-learning-thermostat/5.8.2/u-boot/+/refs/heads/master/u-boot-imx/board/kosagi/novena/novena_spl.c | GEPRUEFT 2026-10-01; HISTORISCH | Zusätzlicher Vergleichsstand, nicht als aktuelle Novena-Referenz behandeln. |
| NV-004 | Novena SPL – historischer Git-Baum | https://fedorapeople.org/cgit/ausil/public_git/u-boot.git/tree/board/kosagi/novena/novena_spl.c?id=c1a6f371ae65f1a3709e872e08bed51321e292fc | GEPRUEFT 2026-10-01; HISTORISCH | An konkrete Commit-ID gebundene DDR-Parameter. |

## Projektinterne Primärbelege (keine Internetlinks)

- `docs/PROJECT-STATE.md`: Projektzustand und bekannte Grenzen.
- `docs/DECISIONS.md`: Entscheidungen, Präregistrierung und Fallklassifikation.
- `docs/TEST-LOG.md`: Testhistorie, Quellen- und Artefaktidentitäten.
- `test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log`: Fall 7C.
- `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log`: Fall 8B.
- `boot/u-boot/default.nix`, `boot/u-boot/0004-*.patch` bis `0008-*.patch`: aktuell eingesetzte Instrumentierung.

## Noch zu vervollständigen

Dieses Verzeichnis ist eine **erste, nachprüfbare Fassung**, keine Behauptung, sämtliche in früheren Projektphasen benutzten URLs bereits erfasst zu haben. Vorhandene URLs aus `docs/`, `README.md`, `AGENTS.md`, `boot/`, `kernel/` und der dokumentierten Projektgeschichte sind in einem gesonderten rein lesenden Inventurschritt zu sammeln, auf Duplikate zu prüfen und anschließend mit Quelle und Status nachzutragen. Externe Links dürfen nicht allein wegen einer Nennung als inhaltlich geprüft markiert werden.

## Offene Recherchefragen – Block 4.10O-1N-8F-5

1. Welche konkreten DDR-Kalibrierungsänderungen enthält der historische Novena-Patch von 2015?
2. Sind diese Änderungen im gepinnten U-Boot-2026.07-Quellstand enthalten, geändert oder ersetzt?
3. Welche DDR-Kalibrierungsfunktionen werden in unserem SPL tatsächlich aufgerufen, und wie werden Rückgabewerte behandelt?
4. Welche bereits dokumentierten Tests beantworten Teile dieser Fragen, sodass keine Wiederholung nötig ist?
5. Welche zusätzlichen Quellen sind erforderlich, bevor überhaupt ein neuer Test erwogen wird?

## Lokaler Quellencheckpoint – BLOCK 9C-44 (2026-10-02)

Keine der nachfolgenden Angaben ist eine neue Onlineprüfung.
[Rekonstruktionsbericht mit exaktem REF-Pfad und Quellhashes](../research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md).

| ID | Lokale Quelle | Status und Aussagegrenze |
|---|---|---|
| SPL-001 | Erhaltenes Locked-Debug-Referenz-ELF `REF/spl/u-boot-spl` | HEUTE HASH-/STATISCH GEPRÜFT; SHA256 `69bb3a4dd2efe059cefc197cad244f13459b0ef08ccb17c886059337ca25e469`; Symbol-/Maschinencodebeleg, kein neuer Boot. |
| SPL-002 | `REF/board/kosagi/novena/novena_spl.c`, `REF/arch/arm/mach-imx/mx6/ddr.c`, `REF/include/wait_bit.h`, `REF/arch/arm/lib/crt0.S`, `REF/.config`, Map und Wrapper | HEUTE LOKAL GEPRÜFT; Boardaufrufe, Rückgabebehandlung, SRAM-Stack/BSS/Tail-Pfad; kein Nachweis realer Kalibrierungswerte. |
| SPL-003 | `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log` | ORIGINALBELEG; vollständig gelesen, 830 Byte; SHA256 `9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`; Fall 8B, keine Root Cause. |
| SPL-004 | `boot/u-boot/0008-novena-diag-control-c0-c2.patch` und bestehende Präregistrierung | LOKAL GEPRÜFT; hartcodierte `0x18200030` stimmt im Referenz-ELF mit `max_total_mem` überein; keine Garantie für ein neues Layout. |
| SPL-005 | Repository-Originalberichte BLOCK 9C-33 bis 9C-42 | FEHLEND im geprüften Repositorybestand; keine rekonstruierte Durchführungshistorie. Auch 9C-43 liegt hier nicht als Originalbericht vor. |
| SPL-006 | ERR050070-Herstellerbeleg und MPDGHWST-Register-Lesesemantik | OFFENE PRIMÄRQUELLENLÜCKE; keine lokale verifizierte Erratum-Revision/Anwendbarkeit und keine belegte Nebenwirkungsfreiheit zusätzlicher Lesungen. |
| SPL-007 | Auftragsangaben zu 9C-42/9C-43 und A/B; Korrektur gemäß BLOCK 9C-49 | CHAT-/AUFTRAGSANGABE aus dem früheren Chatbericht zu BLOCK 9C-42; keine lokal verifizierte Repository-Originalquelle, Originalbericht 9C-42 lokal nicht nachgewiesen. A: lokal initialisierte Diagnose-Struktur auf dem SRAM-Stack mit Parameterweitergabe an WL-/DQS-Helfer. B: initialisierter globaler Diagnosezustand in SRAM-`.data` bei unveränderten Funktionssignaturen. Optionaler Messumfang ist davon getrennt. |

Die offenen Fragen zur Rückgabebehandlung sind durch die heutige statische
Rekonstruktion eingegrenzt: WL/DQS werden aufgerufen, ihre Rückgaben und die
einschlägigen Warte-Rückgaben werden nicht ausgewertet. Offen bleiben die
realen Werte, ERR050070-Anwendbarkeit, MPDGHWST-Lesesemantik und Root Cause.

Einordnung vom 2026-10-04: Die Statusangaben SPL-001 bis SPL-007 und „heute“ beziehen sich auf den Quellencheckpoint vom 2026-10-02. Sie bleiben als historischer Prüfstand erhalten. Spätere Originalinhaltsprüfungen und die lokale Aufnahme des RM Rev. 2 aktualisieren einzelne Quellenlücken, ohne frühere Originalberichte zu ersetzen. Weiterhin offen sind die tatsächlichen Kalibrierungswerte, die konkrete ERR050070-Anwendbarkeit, die allgemeine Nebenwirkungsfreiheit zusätzlicher Lesungen und die Root Cause.

## NXP/Freescale – lokale Referenzhandbücher

Quellenaufnahme: 2026-10-03.

Die folgenden beiden PDF-Dateien sind lokal vorhanden. Ihre
Kopien wurden byteweise gegen die Dateien unter `~/Downloads`
geprüft (`PDF_IDENTITAET=PASS`).

| ID | Quelle | Lokale Fundstelle | Status |
|---|---|---|---|
| NXP-RM2-1 | i.MX 6Dual/6Quad Reference Manual, Rev. 2, Juni 2014, Teil 1, 3828 PDF-Seiten | `research/nxp-reference-manuals/sources/38e11e86a1aade90ca272ce6a4d522c6fa6344b6f6c12e6767a46ed46451b0e7.pdf` | LOKAL VORHANDEN; PDF-Metadaten und SHA256 geprüft |
| NXP-RM2-2 | i.MX 6Dual/6Quad Reference Manual, Rev. 2, Juni 2014, Teil 2, 1989 PDF-Seiten | `research/nxp-reference-manuals/sources/IMX6DQRMr2_part2.pdf` | LOKAL VORHANDEN; MMDC-Kapitel 44 inhaltlich teilweise geprüft |

### Primärquellenbefunde aus Revision 2

- Abschnitt 44.11.3.1.2: Hardware-DQS-Gating und Halbtakt-Korrektur.
- Abschnitt 44.12.7: `MDMISC` und unterstützte Kanalbetriebsarten.
- Abschnitt 44.12.61: `MPWLHWERR` und Write-Leveling-Ergebnisfelder.
- Abschnitte 44.12.62 ff.: `MPDGHWST`-Statusregister.

Die Angaben betreffen Revision 2 von 2014. Revision 6 von
2020 und AN4467 Rev. 2 liegen weiterhin nicht als lokal
verifizierte Originaldokumente vor.

Die früher dokumentierten Quellenlücken bleiben als
historischer Recherchebefund erhalten. Für die inzwischen
vorliegenden Registerbeschreibungen aus Revision 2 ist
ihr Status durch diesen Nachtrag aktualisiert.

Aus diesen Quellen ergibt sich noch keine nachgewiesene
Root Cause des Novena-Kaltstartfehlers 8B.

Vollständige Dateimetadaten, SHA256-Werte und Herkunft:
`research/nxp-reference-manuals/README.md`.

## Quellenpräzisierungen – Dokumentationsstand 2026-10-04

Keine der folgenden Angaben bezeichnet eine neue Onlineprüfung.

**NXP-RM2-DQS – BELEGT:** Lokales NXP-RM2-2, §44.11.3.1.2, Schritte 31–35, gedruckte S. 3871: Abschlussanzeige, Lesen des oberen MPDGHWST-Grenzfelds und Einstellung auf oberen Grenzwert minus einen halben Takt. §44.12.62 ff., gedruckte S. 3996 ff., beschreibt die MPDGHWST-Ergebnisfelder. Die vorgesehene Ergebnislesung ist belegt; eine allgemeine Nebenwirkungsfreiheit zusätzlicher oder wiederholter Diagnosezugriffe bleibt OFFEN.

**NXP-RM2-DECODER – BELEGT mit Grenze:** §44.4.4.1, Tabellen 44-4/44-5, gedruckte S. 3838–3840: x16/x32-Beispiele mit acht Banken, 15 Row-Bits und zehn Column-Bits. Keine belegte reale x64-Adresszuordnung der aktuellen Novena und kein Aliasierungsnachweis.

**NXP-ERR050070 – BELEGT als dokumentierte frühere Originalinhaltsprüfung:** IMX6DQCE Rev. 7, 02/2019, S. 143, PDF-Index 142; Aufnahme von ERR050070 laut Revisionstabelle. Provenienz: `research/05-gesamtanalyse/22-quellenprovenienz/01-NXP-ORIGINALQUELLEN.md` und `05-PROVENIENZMATRIX.md`, N01. Beschrieben wird die eingeschränkte WL_HW_ERRn-Anzeige bei automatischem Write-Leveling und MPWLHWERR als alternatives Kriterium für genutzte Byte-Lanes. Kein lokales Original-PDF und kein SHA256 seiner Bytes vorhanden; konkrete Anwendbarkeit und Auslösung in Fall 8B bleiben OFFEN.

**UB-HIST-DQS – BELEGT im dokumentierten Quellenumfang:** Der qualifizierte REF auf Basis v2026.07 verwendet `0xc0`, sein Kommentar nennt `0x80`. Der historische Reviewentwurf vom 22.12.2015 enthält diese Differenz bereits. Der dokumentierte Commit `cec2f200b4bfcc466e4a83196ed5fdd54678c1c7` ändert zwei Subtraktionen auf `0x80`. Provenienz und geprüfte Releasegrenzen: `research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md` und `22-quellenprovenienz/04-U-BOOT-HISTORISCHE-BELEGE.md`. Kein Nachweis einer Novena-Root-Cause oder eines erfolgreichen Novena-Vergleichstests.

**UB-HIST-ENABLE – BELEGT mit Lücke:** Für Commit `89d485940106c095ec69ecb12a78a42e096dece7` sind Commitmetadaten und die dokumentierte Graphzuordnung zu v2016.03-rc2/v2016.03 vorhanden. Der vollständige historische Parameterdiff fehlt wegen nicht vorhandener Blobs. Die ältere Bezeichnung „Patch von 2015“ darf nicht mit einem vollständig geprüften Releaseartefakt gleichgesetzt werden.

**KOSAGI-DDR – historischer Quellenumfang:** `research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md` dokumentiert historische Modul-, Kalibrierungs-, Boardrevisions- und Kaltstart-ECO-Berichte mit ihren Grenzen. Diese Berichte identifizieren weder das aktuelle DIMM noch die aktuelle ECO-Bestückung und beweisen keine Ursache von Fall 8B.

**SPL-PROVENIENZ:** Der exakte Locked-Debug-REF-Pfad, die Basisrevision und getrennten Identitäten von ELF, Raw-SPL, Wrapper, Map und Konfiguration stehen im Dokumentationscheckpoint unter `research/04-spl-ddr/`. Die Fall-8B-Rohlogidentität ist davon getrennt. Rekonstruierte statische Qualifikation beweist keinen nicht erhaltenen historischen Analyseablauf.

**BLOCK-8G – Auftragsangabe:** `STATIC_EVIDENCE_EXHAUSTED=YES` und `ROOT_CAUSE=UNRESOLVED` werden gemäß Auftrag 9C-47W-8H geführt. Ein Originalbericht 9C-47W-8G ist in den gezielt geprüften Quellen nicht nachgewiesen. Kein rekonstruierter Originalbericht.

Die inzwischen lokal belegten Aussagen aus RM Rev. 2 sind keine Originalprüfung von RM Rev. 6 oder AN4467 Rev. 2. Die früheren offenen Recherchefragen bleiben datierte historische Einträge.

## 2026-10-05 – BLOCK 9C-52C: konsolidierter historischer Quellenstand

Dieser Nachtrag übernimmt den vom Benutzer bestätigten Konsolidierungsstand
9C-51A/9C-52B gemäß Auftrag 9C-52C. Eigenständige lokale Originalberichte
51A/52B wurden im geprüften Bestand nicht gefunden. Angaben, die über die
unten genannten lokalen Belege hinausgehen, haben daher ausdrücklich
Auftragsprovenienz; sie sind keine neue Originalinhalts- oder Onlineprüfung.
Keine Netzwerkrecherche wurde in diesem Block durchgeführt.

`BELEGT` bezeichnet einen innerhalb seiner Quelle bestätigten Befund;
`STARK BELEGT` eine starke Zuordnung ohne vollständige Byteprovenienz;
`QUELLENLUECKE` fehlende erforderliche Primärinformation;
`NICHT BELEGT` eine nicht nachgewiesene konkrete Zuordnung. Abrufstatus,
Originalquelle, lokale Kopie und Ableitung bleiben davon getrennt.
Die ergänzende CSV erfasst diesen Konsolidierungsnachtrag, nicht rückwirkend
sämtliche Einträge der älteren Quellenverzeichnisfassungen.

### Historische DDR-Originale

| Quelle | Historische Revision | Belegbasis und Grenze |
|---|---|---|
| Novena ddr3 notes | oldid 621 | Dokumentierte Originalinhaltsprüfung in `research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md` und `22-quellenprovenienz/03-NOVENA-KOSAGI-ORIGINALQUELLEN.md`; keine dort archivierte HTML-Rohkopie. |
| Novena PVT2 ECO List | oldid 1372 | Lokale Originalkopie `research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.html`; heutige ECO-Bestückung offen. |
| U-boot PVT Notes | oldid 1755 | Lokale Originalkopie `research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.html`; SPL-Kalibrierung und Leitungslängenbezug. |
| Novena Issue Log | oldid 826 | Lokale Kopie `research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.html`; oldid 826 unabhängig lokal durch `wgRevisionId=826` und Retrieved-from-Permalink belegt. |
| Novena DVT Issue Log | oldid 944 | Dokumentierte Originalinhaltsprüfung in `22-quellenprovenienz/03-NOVENA-KOSAGI-ORIGINALQUELLEN.md`; historischer Burn-in, keine heutige DIMM-Identität. |
| Novena PVT Issue Log | oldid 1057 | Lokale Kopie `research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.html`; kein aktueller DDR-Ursachenbeleg. |
| System boot time messages | oldid 1832 | Wiedergefundene historische Quelle gemäß bestätigtem Konsolidierungsstand 51A/52B; lokale Rohkopie und vollständige URL hier nicht nachgewiesen. |

STARK BELEGT: Die historische Novena-DDR-Provenienz ist stark etabliert;
die Geometriekontinuität 2014–2026 ist stark gestützt (Konsolidierungsstand
51A/52B). BELEGT: Dynamische Kalibrierung für austauschbare Novena-SO-DIMMs
ist historisch dokumentiert; lokale Softwareableitung siehe
`research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md`.
Dies identifiziert weder das heutige DIMM noch dessen SPD oder reale
Rank-Anzahl. Historisch erfolgreiche DIMMs erklären den aktuellen Fehler nicht.

### Factory-R1, Software und Repositorygenerationen

BELEGT im bestätigten historischen Quellenstand: offizieller Name
`novena-mmc-disk-r1.img`, historische URL
`http://repo.novena.io/novena/images/novena-mmc-disk-r1.img`, SHA256
`26d368cb4b3aa43e411703f8c659d3e229deacfe75af38c1f82489dd9af80dbb`,
MD5 `6923a145cbdc75b420408fc2d09ba4f8`, Einsatz als ausgeliefertes
Novena-Image und historisches Shrink/Expand-Konzept. Name, URL und Hashes
stehen bereits in `docs/HISTORICAL-MEDIA.md`. Die Factory-Prozedur mit `dd`,
Neuerzeugung von p3 bis zum Kartenende, `fsck` und `resize2fs` sowie die
Signaturbytes sind bereits in der lokalen Primärkopie
`research/01-original-novena-docs/sources/kosagi-novena-main-page.html`,
Abschnitt „Disk Imaging“, enthalten. Die OpenPGP-Signatur-Paketzeit
`2014-12-22 07:35:33 UTC` ist aus diesen Bytes ableitbar. Die Paketzeit ist kein verifiziertes
Release- oder Erstellungsdatum des Binärimages.

QUELLENLUECKE: Keine hashverifizierte R1-Binärkopie wiedergefunden;
kryptographische Gesamtprüfung der Signatur, exakte Originalpartitionierung
und exakter Release-Shrink-Schritt bleiben offen.
`FACTORY_R1_BINARY_RECOVERED=NO`. Ein negatives Suchergebnis begründet
keine Aussage `FACTORY_R1_IMAGE_LOST=YES`.

STARK BELEGT gemäß bestätigtem Konsolidierungsstand: Factory-U-Boot als
Debian-Version `2014.10-novena-rc12`; stärkster Commit-Kandidat ist Tag
`v2014.10-novena-rc12`, `c5efeadb913c8246be372054adcd6106ad4fe067`.
Die Tagzuordnung ist lokal in
`research/02-historical-software/extracts/xobs-u-boot-novena-tag-history.txt`
und `research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md` dokumentiert.
`FACTORY_R1_EXACT_UBOOT_COMMIT=NICHT_BELEGT`: Kandidat und exakter
Factory-R1-Commit bleiben getrennt.

STARK BELEGT gemäß bestätigtem Konsolidierungsstand: Factory-/shipped-Novena-
Kernel `3.17.0-rc5-00217-gfd79638`. Die bytegenaue Zuordnung zum nicht
wiedergefundenen R1-Binärimage bleibt offen. Spätere Kernel- und
Bootloaderupdates werden nicht rückwirkend R1 zugeschrieben.

Historisch getrennt: `repo.novena.io/repo/` ist die frühere APT-Generation,
`repo.novena.io/debian/` die spätere APT-/Paketgeneration und
`repo.novena.io/novena/images/` der Image-Artefaktpfad. Exaktes Umschaltdatum
`/repo/` → `/debian/` und vollständige signierte historische
Release/InRelease-/Packages-/Sources-/Pool-Kette bleiben offen:
`SIGNED_APT_INDEX_CHAIN=QUELLENLUECKE`.

BELEGT im bestätigten frühen Quellenstand: `/repo/`, Schlüsselkennung
`03C7B7EC`, `kosagi-repo_1.0-r1`, `novena-eeprom_2.1-1` und
`novena-firstrun_1.4-r1`. Zeitliche Nähe beweist nicht, dass exakt diese
Paketversionen im nicht wiedergefundenen R1-Binärimage enthalten waren.

### Historische Kommunikationsquellen

Die folgenden Befunde stammen aus dem bestätigten Stand 51A/52B;
zugehörige lokale Originalberichte oder Rohkopien sind, mit Ausnahme des
nachfolgend präzisierten Kanal-/OFTC-Bezugs, hier nicht nachgewiesen.
Vollständige URLs werden ohne vorhandenen Beleg nicht ergänzt.

- BELEGT: Kanal-/OFTC-Bezug in der lokalen Primärkopie
  `research/01-original-novena-docs/sources/kosagi-novena-main-page.html`,
  „Potential contributors“: `irc.oftc.net #kosagi`. Gesprächsinhalte bleiben
  QUELLENLUECKE. Ein öffentliches historisches IRC-Logarchiv wurde bislang nicht gefunden;
  das ist kein Nichtexistenzbeweis.
- Das Kosagi-Forum ist eine reale technische Quelle; bekannter Thread
  `viewtopic.php?id=421`, Thema „Novena Firmware“.
- C3D2 dokumentiert eine reale Novena und verweist auf `factory-image`.
  Ein heutiger Besitz einer R1-Binärkopie bei C3D2 ist NICHT BELEGT.
- 31C3 ist ein belastbarer Zeit-/Ereignisanker für bunnie/xobs/fail0verflow,
  kein Nachweis eines dort archivierten Factory-Images.
- xobs „Novena First Run“, 2014-10-12, dokumentiert Arbeit am finalen
  Novena-Disk-Image; „Factory test“, 2014-11-16, die Factory-Phase.

Diese Quellenbefunde lösen keine aktuelle Root Cause und öffnen die
abgeschlossene H3-1R-/I2C3-/ES8328-Untersuchung nicht wieder.

## 2026-10-06 – BLOCK 9C-52E: abgeschlossener historischer Quellencheckpoint

Konsolidierung der abgeschlossenen read-only Blöcke 9C-52D, 9C-52D-1 und
9C-52D-2R gemäß deren im Auftrag bestätigten Ergebnissen, ergänzt durch
lesenden Abgleich lokaler Primärfundstellen. Keine neue Onlineprüfung,
Ursachenforschung oder Hardwarebeobachtung. Die Klassifikationen BELEGT,
STARK BELEGT, NEGATIVBEFUND, NICHT BELEGT, WIDERSPRUCH und QUELLENLUECKE
bleiben getrennt; bessere Provenienz erhöht keine Aussageklasse.

### 9C-52D: Quellenverzeichnis-Audit

`SOURCE_CSV_FULLY_REVIEWED=YES`, `SOURCE_CSV_ENRICHMENT_NEEDED=YES` und
`DOCUMENTATION_CORRECTION_NEEDED=YES` bezeichneten den abgeschlossenen Audit.
Die 13 identifizierten CSV-Einträge erhalten konkrete lokale Fundstellen und
vorhandene kanonische URLs. oldid 826 ist unabhängig lokal belegt. Factory-
Prozedur und Signaturbytes besitzen lokale Primärbelege; fehlende Binärkopie,
Signaturgesamtprüfung und Release-Shrink sind davon getrennte Quellenlücken.
Für `#kosagi` ist der OFTC-Bezug lokal belegt, Gesprächsinhalte und ein
öffentliches historisches Logarchiv bleiben offen. Bei der späteren
`/debian/`-APT-Generation bleibt die bestätigte Auftragsprovenienz kenntlich;
der lokale Builder belegt `/repo/`, die lokale Main Page den Imagepfad.

### 9C-52D-1: xobs/novena-image

BELEGT: lokale historische Primärquelle
`/home/loomit/novena-historical-research/novena-image`, Branch `master`,
Origin `https://github.com/xobs/novena-image.git`, qualifizierter HEAD
`f62c4f3b452199ab882e2c3dba16d4e31dfb1d56`. Vollständige lokale Historie:
36 Commits; ältester Commit `a9d5066d55f2cdbdef1589f2bd12ca1e176ac2bb`
vom 2014-10-10. Commit `24a84d8c92e3583dffcbbe7dcc424dc94884e29f`
vom 2014-10-15 führt Loopback-Erzeugung mit `loopback_size=3965190144`
und `truncate "-s${imgsize}" "${imgname}"` ein. BELEGT ist die logische
Gesamtgröße der Builder-Image-Datei; dieselbe Größe für das veröffentlichte
`novena-mmc-disk-r1.img` ist NICHT BELEGT.

BELEGT im Builder: MBR, `fdisk -C32 -H32`, MMC-Signatur `0x4e6f764d`,
SATA-Signatur `0x4e6f7653`, p1 FAT `/boot`, p2 swap, p3 ext4 root bis zum
verbleibenden Ende. Die Boot-Partitionsgröße entwickelt sich `32M -> 64M -> 32M`.
QUELLENLUECKE: exakte Factory-R1-Sektorgeometrie; fdisk-Defaults, historisches
Tooling und konkreter Release-Lauf sind nicht vollständig qualifiziert.
NEGATIVBEFUND: keine Implementierung eines Release-Shrink-Schritts mittels
`resize2fs`, `sfdisk` o. ä. in der vollständigen lokalen Historie. Das schließt
einen externen Release-Schritt nicht aus. Repository-`dd` dient SPL bzw.
späterer Stichprobenprüfung; eine Release-Shrink-Prozedur ist damit NICHT BELEGT.

BELEGT: `8b64953` vom 2014-11-04 referenziert in `mmc-install.sh` u. a.:

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

Dies verbessert die Paketchronologie, beweist aber keinen Inhalt des
veröffentlichten Factory-R1-Images und kein vollständiges R1-Paketmanifest.
Die Side-Branch-Commits `92ba999`, `444744c`, `4baf00a`, `53e2521` tragen
Dezember-2014-Commitdaten, wurden erst im Dezember 2015 über `29d5ed5` /
`f62c4f3` in master integriert. Heutiges master ist kein Nachweis des
Masterzustands vom Dezember 2014.

```text
FACTORY_R1_RELEASE_BUILDER_IDENTIFIED=PARTIAL
FACTORY_R1_RELEASE_SHRINK_PROCEDURE_IDENTIFIED=NO
FACTORY_R1_EXACT_IMAGE_SIZE_EXPLAINED=NO
FACTORY_R1_EXACT_PARTITION_GEOMETRY_EXPLAINED=NO
```

### 9C-52D-2R: OpenBSD

BELEGT: unabhängige historische Quelle
`/home/loomit/novena-historical-research/openbsd-src`. Initialer Support:
`31642defb5b3233700d202713f8a65d872d823c3`, `2015-05-08T03:38:26Z`,
„Add initial board specific parts of novena support, tested by djm@“.
Commit und Folgecommits belegen zeitgenössische Tests auf realer Novena und
laufenden Betrieb; etwa `05d12a02f11` („committed from Novena“).
`a5b7bc8b7df` berichtet 4 GB physischen RAM und U-Boot-Angaben
`memstart: 0x10000000`, `memsize: 0xf0000000`. Der untersuchte Pfad
(`sys/arch/armv7/armv7/armv7.c`, `sys/arch/armv7/imx/imx_machdep.c`)
übernimmt bootloaderinitialisierten RAM, verwendet initial machid
`0x10ad` / 4269 und die Bootloader-RAM-Beschreibung. U-Boot-Abhängigkeit
ist explizit. NEGATIVBEFUND: keine Novena-spezifische DDR-Initialisierung
in diesem untersuchten OpenBSD-Pfad.

BELEGT: getestete SDHC-Unterstützung, u. a. `2852902afc7` („Tested by djm“).
NEGATIVBEFUND: kein vollständiger lokaler Bootlog gefunden. QUELLENLUECKE:
vollständiger exakter SD-Bootablauf. Der unabhängige reale Betriebs-/RAM-Beleg
beweist weder identisches DIMM/SPD/DDR-Parameter noch identischen U-Boot-Stand
oder Bootpfad zu Factory R1 oder zum aktuellen NixOS/Patch0010-Test.

### 9C-52D-2R: gentoo-on-novena und novena-overlay

BELEGT: `/home/loomit/novena-historical-research/gentoo-on-novena`, HEAD
`f49fcab518c5bb4b2fb0e90baecf8b504209bfff`,
`https://github.com/sakaki-/gentoo-on-novena`. README und Releasehistorie
dokumentieren bootfähige Novena-microSD-Gesamtimages (libre und standard),
v1.0.0/v1.0.1, v1.0.1 etwa 770/797 MiB komprimiert, 6 GiB entpackt,
p1 vfat `/boot`, p2 swap, p3 ext4 `/`, Linux 4.7.2 und
`imx6q-novena.dtb` als `/boot/novena.dtb`. Ein realer Board-/Desktop-
Testbericht ist vorhanden; NEGATIVBEFUND: kein vollständiger lokaler
serieller Bootlog. U-Boot-Paket: `dev-embedded/u-boot-novena-2014.10.8`;
Factory-R1-Zuordnung NICHT BELEGT. Beziehung zu xobs explizit über das Overlay.
Weiterer späterer unabhängiger Betriebsbeleg, keine unabhängige DDR-Initialisierung.

BELEGT: `/home/loomit/novena-historical-research/novena-overlay`, HEAD
`8f88536ebcd4fc107b75a97006a8447628d08638`, vollständige Historie mit
30 Commits, `https://github.com/sakaki-/novena-overlay`. README beschreibt
Port der damaligen xobs-Novena-Debian-Pakete. Das Ebuild
`dev-embedded/u-boot-novena/u-boot-novena-2014.10.8.ebuild` bildet
`2014.10.8 -> v2014.10.r8-novena` von `xobs/u-boot-novena` ab;
deklarierte Quellidentität lokal im separaten U-Boot-Repository nachvollziehbar:
`b98333336941440e2792715853cbe0211577e0d7`. Das verifiziert kein ursprünglich
heruntergeladenes Release-Archiv nachträglich kryptographisch.
Factory-R1-Zuordnung NICHT BELEGT. README dokumentiert sakakis Portierung der
xobs-Linux-4.4-Patches auf Linux 4.7.2. Der explizite xobs-Paketbezug erzeugt
keine unabhängige Factory-R1-Provenienz.

### Quervergleich und offene Grenzen

```text
NEW_INDEPENDENT_NOVENA_BOOT_EVIDENCE=YES
NEW_INDEPENDENT_RAM_EVIDENCE=YES
INDEPENDENT_DDR_INITIALIZATION_EVIDENCE=NO
```

Historischer Gegenbeleg: unabhängige spätere Systeme dokumentieren erfolgreichen
Betrieb realer Novena-Hardware mit mehreren GiB RAM. Dies spricht gegen die
pauschale Behauptung, Novena könne oberhalb eines kleinen DDR-Bereichs grundsätzlich
nicht funktionieren. Es erklärt den aktuellen C1/Q1-Fehler nicht und belegt keine
Identität von DIMM, SPD, DDR-Konfiguration, U-Boot oder physischem Speicherpfad.
Weder Software- noch Hardwareursache des aktuellen Fehlers folgt daraus.
Factory-R1-Binärkopie, vollständige Kryptoverifikation, exakter U-Boot-Commit,
Paketmanifest, Partitionsgeometrie, Release-Shrink, signierte APT-Indexkette und
Factory-DIMM/SPD-Identität bleiben offen; aktueller Status in PROJECT-STATE.md.
I2C3/ES8328 bleibt abgeschlossen; `52Q=WAIT`.
