# Novena PVT2 – Quellenmetadaten

## Zweck

Diese Datei dokumentiert Herkunft und lokale Identität der im
Novena-Root-Cause-Projekt verwendeten originalen PVT2-Hardwarequellen.

Die Originaldateien werden bytegenau im Verzeichnis `sources/` archiviert und
dürfen nicht verändert werden. Die SHA-256-Prüfsummen dienen als kanonische
lokale Identität der jeweiligen Forschungsartefakte.

Aus HTTP-, PDF-, ZIP- oder Dateisystem-Metadaten allein werden keine
technischen Schlussfolgerungen über die Hardware abgeleitet.

## PVT2-Schaltplan-PDF

### Quelle

- Artefakt: `novena_pvt2.PDF`
- Quell-URL: `http://bunniefoo.com/novena/pvt2_release/novena_pvt2.PDF`
- Abgerufen: 2026-09-15
- HTTP-Ergebnis: `200 OK`
- Content-Type: `application/pdf`
- Content-Length: `6451371` Bytes
- Server Last-Modified: `Fri, 04 Jul 2014 17:19:28 GMT`
- ETag: `"53b6e220-6270ab"`

### Lokale Verifikation

- Dateityp: PDF 1.3
- Seiten: 16
- Dateigröße: 6451371 Bytes
- SHA-256: `1d4c1ecdf3399867b68c0b13959b30c7cc7c23d051de2aa2c87cd66652502951`

### PDF-Metadaten

- Creator: Altium Designer
- Producer: llPDFLib 3.x
- von `pdfinfo` gemeldetes Erstellungsdatum:
  `Sat Jul 5 02:00:00 2014 CEST`
- JavaScript: ja
- verschlüsselt: nein

### Provenienzbewertung

Die Datei wurde direkt vom historischen
`bunniefoo.com/novena/pvt2_release/`-Pfad abgerufen.

Sie wird bytegenau im heruntergeladenen Zustand aufbewahrt.

## PVT2-Altium-Quellarchiv

### Quelle

- Artefakt: `novena_pvt2.zip`
- Bezeichnung auf der archivierten Kosagi-Seite: `Altium source`
- Quell-URL: `http://bunniefoo.com/novena/pvt2_release/novena_pvt2.zip`
- Tatsächlicher Download für die lokale Archivierung: HTTPS
- Abgerufen: 2026-09-20
- HTTP-Ergebnis: `200 OK`
- Content-Type: `application/zip`
- Content-Length: `7172816` Bytes
- Server Last-Modified: `Fri, 04 Jul 2014 17:20:28 GMT`
- ETag: `"53b6e25c-6d72d0"`

Die archivierte Kosagi-Seite
`sources/kosagi-novena-pvt-design-source.html` führt dieses Archiv
ausdrücklich als PVT2-`Altium source` auf.

### Lokale Verifikation

- Dateigröße: 7172816 Bytes
- ZIP-Integritätsprüfung mit `unzip -t`: erfolgreich
- SHA-256:
  `a8322aaa3147044ae3dbaf229de44980b92d9ec920f20d50e140769281ff5b4c`

### Relevanter Archivinhalt

Das Archiv enthält 19 Dateien mit insgesamt 25439938 Bytes
unkomprimiert.

Enthalten sind unter anderem:

- `01docmap.SchDoc`
- `02cpu_power.SchDoc`
- `03cpu_sodimm.SchDoc`
- `04pwr_pmic.SchDoc`
- `05pwr_input.SchDoc`
- `06cpu_soc.SchDoc`
- `07cpu_sdcard.SchDoc`
- `07sdcard.SchDoc`
- `08usb.SchDoc`
- `09sata.SchDoc`
- `10ethernet100.SchDoc`
- `11ethernetGbit.SchDoc`
- `12mPCIe.SchDoc`
- `13hdmi_lcd.SchDoc`
- `14audio.SchDoc`
- `15fpga.SchDoc`
- `16gpio_misc.SchDoc`
- `novena_pvt2.PrjPcb`
- `novena_pvt2_a.PcbDoc`

Für die Untersuchung des I2C3-/Audio-Power-Domain-Problems sind insbesondere
`14audio.SchDoc` und `novena_pvt2_a.PcbDoc` relevant.

Das im ZIP gespeicherte Datum von `novena_pvt2_a.PcbDoc` ist
`2014-07-05 00:59`. Dieses ZIP-Metadatum wird nur als Quellenmetadatum
festgehalten und nicht als unabhängiger Nachweis eines Revisions- oder
Fertigungszeitpunkts interpretiert.

### Provenienzbewertung

Das Archiv stammt direkt aus dem historischen PVT2-Release-Pfad und wird von
der archivierten Kosagi-PVT-Design-Source-Seite ausdrücklich als
Altium-Quelle der PVT2-Hauptplatine bezeichnet.

Das Archiv wird bytegenau im heruntergeladenen Zustand aufbewahrt.

## PVT2-Gerber-Archiv

### Quelle

- Artefakt: `gerbers-novena_pvt2_a.zip`
- Bezeichnung auf der archivierten Kosagi-Seite: `Gerbers`
- Quell-URL:
  `http://bunniefoo.com/novena/pvt2_release/gerbers-novena_pvt2_a.zip`
- Tatsächlicher Download für die lokale Archivierung: HTTPS
- Abgerufen: 2026-09-20
- HTTP-Ergebnis: `200 OK`
- Content-Type: `application/zip`
- Content-Length: `954306` Bytes
- Server Last-Modified: `Fri, 24 Feb 2017 07:45:48 GMT`
- ETag: `"58afe4ac-e8fc2"`

Die archivierte Kosagi-Seite
`sources/kosagi-novena-pvt-design-source.html` führt dieses Archiv
ausdrücklich als PVT2-`Gerbers` auf.

### Lokale Verifikation

- Dateigröße: 954306 Bytes
- ZIP-Integritätsprüfung mit `unzip -t`: erfolgreich
- SHA-256:
  `66664c73456ddf5458b1576709374402ca83238d820d747ccc40a3010207a319`

### Relevanter Archivinhalt

Das Archiv enthält 30 Einträge mit insgesamt 4322408 Bytes
unkomprimiert.

Dazu gehören unter anderem:

- Kupfer-/Gerberdaten `G1` bis `G4`
- `GBL`
- `GTL`
- `GBO`
- `GTO`
- `GBP`
- `GTP`
- `GBS`
- `GTS`
- `GKO`
- `GP1` bis `GP4`
- Bohrdaten `RoundHoles.TXT` und `SlotHoles.TXT`
- weitere Altium-Fertigungsdateien und Reports

Die Einträge des Gerber-Archivs tragen überwiegend ZIP-Zeitstempel vom
`2014-07-08`.

Der HTTP-Header des heute abrufbaren ZIP-Archivs meldet dagegen
`Last-Modified: Fri, 24 Feb 2017 07:45:48 GMT`.

Diese beiden Angaben werden bewusst getrennt dokumentiert. Das
HTTP-`Last-Modified`-Datum von 2017 wird nicht als Erstellungsdatum des
PCB-Layouts oder der enthaltenen Gerberdaten interpretiert.

### Provenienzbewertung

Das Archiv stammt direkt aus dem historischen PVT2-Release-Pfad und wird von
der archivierten Kosagi-PVT-Design-Source-Seite ausdrücklich als
Gerber-Paket der PVT2-Hauptplatine bezeichnet.

Das Archiv wird bytegenau im heruntergeladenen Zustand aufbewahrt.

## Beziehung der Quellen zueinander

Die archivierte Kosagi-PVT-Design-Source-Seite führt für die PVT2-Hauptplatine
unter anderem folgende getrennte Artefakte auf:

- Schematics
- Altium source
- Altium-referenced bitmaps
- STEP 3D file
- Gerbers

Damit sind `novena_pvt2.zip` und `gerbers-novena_pvt2_a.zip` zwei offiziell
aufgeführte, voneinander unabhängige Artefakte desselben historischen
PVT2-Release-Bereichs.

Die zeitliche Nähe der im Altium-Archiv und Gerber-Archiv gespeicherten
Dateizeitstempel ist mit einer engen Beziehung der Artefakte vereinbar.
Eine exakte Revisionsidentität zwischen `novena_pvt2_a.PcbDoc` und den
Gerberdaten wird daraus jedoch nicht ohne zusätzliche Prüfung abgeleitet.

## Verwendung in Block 3.15M

Das Altium-Archiv wurde in Block 3.15M als zusätzliche Primärquelle
herangezogen, nachdem die PDF-Schaltpläne allein keine sichere physische
Zuordnung von Pads, Vias und Messpunkten erlaubten.

Die Datei `novena_pvt2_a.PcbDoc` enthält nach der bisherigen
Read-only-Untersuchung unter anderem die Altium-Datenbereiche:

- `Components6`
- `Nets6`
- `Pads6`
- `Vias6`
- `Tracks6`
- `Rules6`
- `Polygons6`

`Components6` liefert bereits Bauteil-, Layer- und Positionsinformationen.
`Nets6` enthält erkennbare Netznamen.

`Pads6` enthält relevante Binärdaten, lässt sich aber mit einer einfachen
`strings`-Auswertung nicht zuverlässig als Pad-zu-Netz-Zuordnung
rekonstruieren.

Deshalb werden unbekannte Binärfelder nicht manuell interpretiert. Für die
weitere Analyse soll ein dokumentierter Altium-PcbDoc-Importer bzw. Parser
verwendet werden.

Die Originalarchive selbst bleiben dabei unverändert.
