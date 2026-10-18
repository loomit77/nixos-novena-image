# NXP/Freescale – i.MX 6Dual/6Quad Referenzhandbuch

Historischer Rev.-2-Aufnahmestand: 2026-10-03

Datierter Rev.-6-Nachtrag: 2026-10-11; Aufnahme am 2026-10-09.

## Zweck

Dieses Verzeichnis bewahrt unter `sources/` zwei historische Rev.-2-PDF-Teile
und das separat aufgenommene Rev.-6-PDF des
i.MX 6Dual/6Quad Applications Processor Reference Manual
von Freescale Semiconductor beziehungsweise NXP Semiconductors auf.

Die Dokumente dienen als historische Herstellerprimärquellen
für die Untersuchung des DDR-Controllers (MMDC), seiner
Register und der DDR-Kalibrierung auf Kosagi Novena.

Die Aufnahme der Dokumente ist kein Nachweis einer
bestimmten Ursache für den Novena-Bootfehler.

## Quelle 1 – Revision 2, Teil 1

Datei:
`38e11e86a1aade90ca272ce6a4d522c6fa6344b6f6c12e6767a46ed46451b0e7.pdf`

Herausgeber: Freescale Semiconductor, Inc.

Dokument: i.MX 6Dual/6Quad Applications Processor
Reference Manual

Revision: 2, Juni 2014

Umfang: 3828 PDF-Seiten

Dateigröße: 23662587 Bytes

SHA256:
`38e11e86a1aade90ca272ce6a4d522c6fa6344b6f6c12e6767a46ed46451b0e7`

Hersteller-Fundstelle:
https://community.nxp.com/t5/i-MX-Processors-Knowledge-Base/i-MX-6DQ-Reference-Manual-IMX6DQRM-R2-Part-1/ta-p/1115983

## Quelle 2 – Revision 2, Teil 2

Datei:
`IMX6DQRMr2_part2.pdf`

Herausgeber: Freescale Semiconductor, Inc.

Dokument: i.MX 6Dual/6Quad Applications Processor
Reference Manual

Revision: 2, Juni 2014

Umfang: 1989 PDF-Seiten

Dateigröße: 12140372 Bytes

SHA256:
`6e107fbb6a13b1a5f83660cb1050a3955d2f211b4df2246613e00e20e43b4884`

Hersteller-Fundstelle:
https://community.nxp.com/t5/i-MX-Processors-Knowledge-Base/i-MX-6DQ-Reference-Manual-IMX6DQRM-R2-Part-2/ta-p/1118510

## Relevante MMDC-Abschnitte in Teil 2

- Kapitel 44: Multi Mode DDR Controller (MMDC)
- Abschnitt 44.11.3: DQS-Gating-Kalibrierung
- Abschnitt 44.11.3.1.2: Hardware-DQS-Gating
- Abschnitt 44.12.7: MMDCx_MDMISC
- Abschnitt 44.12.61: MMDCx_MPWLHWERR
- Abschnitte 44.12.62 ff.: MMDCx_MPDGHWST

## Quellenstatus und Grenzen

Die beiden PDF-Dateien wurden am 2026-10-03 auf
der foobox aus ~/Downloads übernommen.

Die Quelldateien wurden mittels SHA256 identifiziert.
Der anschließende byteweise Vergleich der Kopien
mit den Quelldateien ergab PDF_IDENTITAET=PASS.

Die beiden Teile werden unverändert aufbewahrt.

Die lokale PDF-Identität beweist für sich allein
nicht die Identität mit einem aktuellen Download
vom NXP-Server.

Diese Quellen dokumentieren Revision 2 von 2014.
Sie ersetzen weder IMX6DQRM Revision 6 von 2020
noch die Originalanwendungsnotiz AN4467.

Herstellerangaben, U-Boot-Implementierung und
tatsächliche Hardwarebeobachtungen sind getrennt
auszuwerten.

## Quelle 3 – Revision 6, Aufnahme vom 2026-10-09

Lokale Datei: [sources/IMX6DQRM_Rev6_202005.pdf](sources/IMX6DQRM_Rev6_202005.pdf)

Herausgeber laut PDF-Metadaten: NXP Semiconductors. Titelblatt:
*i.MX 6Dual/6Quad Applications Processor Reference Manual*,
Dokumentnummer IMX6DQRM, Rev. 6, 05/2020.

Umfang: 5643 PDF-Seiten. Dateigröße: 35753863 Bytes.

SHA256:
`8ee5d62a592f27c83bda37a9fb7fff57810e8b6d19fa6badbb9b5f1c8699b505`

**Aufnahmeprovenienz:**
`BLOCK-9C-52Q-R7B-3AAR-R6D-SRC03-20261009-133326-5JWdlUSq.txt`
unter `/home/loomit/novena-backups/block-logs/` dokumentiert die Übernahme
von `/home/loomit/Downloads/IMX6DQRM-1.pdf` an den obigen Projektpfad.
`SOURCE_HASH_GATE=PASS`, `COPY_AND_HASH_GATE=PASS` und `SOURCE_IN_PROJECT=YES`
belegen den damaligen Aufnahmeumfang; Quelle und Ziel sind an denselben
SHA256 gebunden. Das Protokoll enthält keine neue Build-, Patch- oder
Hardwareprüfung.

**Lokaler Abgleich vom 2026-10-11:** Alle drei PDF-Dateihashes stimmen mit
den hier angegebenen Identitäten überein. Titelblatt und Metadaten der
Rev.-6-Datei wurden lokal gelesen. Damit sind Dateiidentität und
Dokumentkennzeichnung bestätigt, keine vollständige inhaltliche Qualifikation
der MMDC-/Kalibrierungsangaben oder Verifikation gegen einen NXP-Download.
Das Aufnahmeprotokoll dokumentiert eine lokale Downloads-Datei; eine darin
nicht belegte Downloadkette wird nicht ergänzt.

Die Rev.-2-Nachweise und ihre Aussagegrenzen bleiben oben erhalten.
Ältere Projektangaben zum damals fehlenden Rev.-6-Original werden durch
diesen datierten Aufnahmenachtrag eingeordnet. Rev. 6 ist nun lokal
vorhanden; dadurch werden weder Rev.-2-Nachweise ersetzt noch AN4467 oder
eine Novena-DDR-Ursache qualifiziert. Quellenidentität, inhaltliche Auswertung,
U-Boot-Implementierung und Hardwarebeobachtung bleiben getrennte Nachweise.

## Untersuchungsstatus

ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
PUSH_AUTHORIZED=NO

## NXP-RM-REV5-201806

- Dokument: i.MX 6Dual/6Quad Applications Processor Reference Manual
- Dokumentnummer: IMX6DQRM
- Revision: 5, 06/2018
- Originaldatei bei Aufnahme: ~/Downloads/IMX6DQRM1.pdf
- Projektdatei: sources/IMX6DQRM_Rev5_201806.pdf
- SHA256: d4651cfbeceadf311059503a7f71281aff26bb6cb3b1416402b9104469d71108
- Umfang: 5813 PDF-Seiten
- Provenienz: Lokaler foobox-Fund, geprüft am 2026-10-13
- Prüfgrundlage: PDF-Titelseite, Dateigröße und SHA256
- Verwendung: Primärquelle für den Vergleich der MMDC-Dokumentation
  zwischen den Revisionen 2, 5 und 6
- Einschränkung: Die Aufnahme beweist keine konkrete DDR-Root-Cause.

## NXP-RM-REV4-201709

- Dokument: i.MX 6Dual/6Quad Applications Processor Reference Manual
- Dokumentnummer: IMX6DQRM
- Revision: 4, 09/2017
- Originaldatei bei Aufnahme: ~/Downloads/606909.pdf
- Projektdatei: sources/IMX6DQRM_Rev4_201709.pdf
- SHA256: dffaf2e21ebf7a980c3d0d5e681871dfb98f958a084fd6f45349e1c1d392d1ea
- Umfang: 5833 PDF-Seiten
- Provenienz: Lokaler foobox-Fund, geprüft am 2026-10-13
- Prüfgrundlage: PDF-Titelseite und SHA256
- Verwendung: MMDC-Quellenvergleich der Revisionen 2, 4, 5 und 6
- Einschränkung: Keine Aussage zur tatsächlichen DDR-Root-Cause.

## NXP-RM-REV1-201304

- Dokument: i.MX 6Dual/6Quad Applications Processor Reference Manual
- Dokumentnummer: IMX6DQRM
- Revision: 1, 04/2013
- Herausgeber: Freescale Semiconductor, Inc.
- Originaldatei bei Aufnahme: ~/Downloads/317e917c7c5ac8d02fb54f0811fb336c310f34f8d454875820b83e8b202a74f6.pdf
- Projektdatei: sources/IMX6DQRM_Rev1_201304.pdf
- SHA256: 317e917c7c5ac8d02fb54f0811fb336c310f34f8d454875820b83e8b202a74f6
- Umfang: 5827 PDF-Seiten
- Provenienz: Online-Kopie, lokal auf foobox geprüft am 2026-10-13
- Prüfgrundlage: PDF-Titelseite, Dokumentnummer und SHA256
- Verwendung: Historischer MMDC-/DDR-Quellenvergleich
- Einschränkung: Keine Aussage zur tatsächlichen DDR-Root-Cause.

### IMX6DQRM Revision 3 (07/2015)

- Dokument: i.MX 6Dual/6Quad Applications Processor Reference Manual
- Dokumentnummer: IMX6DQRM
- Revision: 3, 07/2015
- Lokale Datei: `sources/IMX6DQRM_Rev3_201507.pdf`
- Seiten: 5845
- SHA256: `69f0642a0e3b86f8b4f732b9f1076ab25391e744a0e93f6e131bce6707f473d6`
- Historische Quelle: Freescale Semiconductor
- Wayback-Snapshot: 2015-10-22 20:43:56 UTC
- Archiv-URL: https://web.archive.org/web/20151022204356id_/http://cache.freescale.com/files/32bit/doc/ref_manual/IMX6DQRM.pdf
- Verifikation: Dokumentnummer, Revision und Datum auf der Titelseite geprüft.
