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
