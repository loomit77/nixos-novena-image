# Block 3.13F – Primärquellenbasierte Eingrenzung von FPGA und ES8328

## 1. Zweck

Block 3.13F setzt die in Block 3.13E begonnene quellenbasierte
Root-Cause-Untersuchung fort.

Da keine elektrischen Messgeräte zur Verfügung stehen und keine
Hardwaremessungen durchgeführt werden, soll die verbleibende
Root-Cause-Frage ausschließlich mit vorhandenen Schaltungsunterlagen,
Herstellerdokumentation, historischen Novena-Quellen und den bereits
gesicherten experimentellen Ergebnissen weiter eingegrenzt werden.

Der Block verfolgt zwei eng begrenzte Ziele:

1. Prüfung, ob der direkt an I2C3 angeschlossene Spartan-6-FPGA während
   Power-on, Initialisierung oder Konfiguration eine plausible primäre
   Busbelastung darstellt.

2. Prüfung, welche belastbaren Aussagen zu den ES8328-Versorgungen und
   insbesondere zur Digital-I/O-Versorgung gemacht werden können und ob
   damit die Mechanismuskandidaten M1 bis M4 weiter voneinander getrennt
   werden können.

Der Block soll ausdrücklich keinen nicht belegten elektrischen
Mechanismus als Root Cause festlegen.

## 2. Ausgangslage

Block 3.13E hatte bereits zwei voneinander unabhängige Evidenzlinien
zusammengeführt.

Historisch dokumentiert der Novena-Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

mit dem Betreff

`ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

dass das Abschalten der ES8328-Versorgung nach damaliger Beobachtung
I2C3 störte und unter anderem Bildschirm, EEPROM und Senoko
beeinträchtigte.

Unabhängig davon änderte der aktuelle Versuch H3-1R ausschließlich die
ES8328-Versorgung auf `regulator-always-on`.

Danach waren fünf von fünf vorregistrierten echten POR-Kaltstarts
erfolgreich.

Dabei verschwanden im untersuchten I2C3-Pfad:

- die bekannte Fehlersignatur `A=81/80 -> M0=93/80`,
- der untersuchte `arbitration lost`,
- der IT6251-Initialisierungsfehler.

Der Bildschirm funktionierte in allen fünf Läufen.

Damit war die kausale Beteiligung der ES8328-/Audio-Power-Domain bereits
stark gestützt.

Offen blieb der genaue elektrische Mechanismus.

## 3. Relevante Mechanismuskandidaten

Für Block 3.13F bleiben insbesondere folgende Kandidaten relevant:

### M1 – Clamp oder statische Busbelastung

Ein unversorgter ES8328 könnte über seine digitalen Anschlüsse den Bus
elektrisch belasten.

Ein konkreter interner Clamp-Pfad ist jedoch nicht belegt.

### M2 – Backfeeding

Über weiterhin versorgte I2C-Leitungen könnte Strom in einen
abgeschalteten ES8328-Power-Domain gelangen.

Ein konkreter interner Strompfad ist jedoch nicht belegt.

### M3 – Pull-up-/Power-Domain-Interaktion

Die I2C-Leitungen auf der Codec-Seite bleiben aus einer Versorgung
hochgezogen, während die digitale I/O-Versorgung des Codecs abgeschaltet
werden kann.

Diese Versorgungskonstellation ist direkt aus den vorhandenen Quellen
ableitbar.

Der daraus entstehende konkrete elektrische Fehlermechanismus ist
dadurch noch nicht bestimmt.

### M4 – Abschalttransient

Der aktive Audio-Power-Schaltkreis könnte beim Abschalten einen
zeitabhängigen elektrischen Zustand erzeugen, der I2C3 beeinflusst.

Ohne elektrische Zeit-, Spannungs- oder Strommessungen ist dieser
Mechanismus nicht nachgewiesen.

## 4. Bereits bekannte Novena-I2C3-Topologie

Die PVT2-A-Schaltung zeigt für den ES8328-Zweig:

`I2C3_SCL -> R26A 330 Ohm -> AUD_I2C3_SCL -> ES8328`

und

`I2C3_SDA -> R27A 330 Ohm -> AUD_I2C3_SDA -> ES8328`.

Auf der Codec-Seite befinden sich:

- R10B = 1 kOhm von `P3.3V_DELAYED` nach `AUD_I2C3_SCL`,
- R11B = 1 kOhm von `P3.3V_DELAYED` nach `AUD_I2C3_SDA`.

Die Pull-ups befinden sich damit auf der ES8328-Seite der
330-Ohm-Serienwiderstände.

Die detaillierte Schaltung zeigt außerdem:

- ES8328 DVDD liegt an `AUD_P3.3V`,
- ES8328 PVDD liegt an `AUD_P3.3V`.

Damit können DVDD und PVDD zusammen mit der Audio-Versorgung abgeschaltet
werden, während die Codec-seitigen I2C-Pull-ups weiterhin aus
`P3.3V_DELAYED` versorgt werden.

Diese Topologie ist für M1 bis M4 unmittelbar relevant.

## 5. FPGA als weiterer direkter I2C3-Teilnehmer

Die PVT2-A-Schaltung zeigt eine direkte Verbindung des Spartan-6-FPGA
mit I2C3.

Dabei gilt:

- FPGA-Pin P4 = `IO_L2P_3` = `I2C3_SCL`,
- FPGA-Pin P3 = `IO_L2N_3` = `I2C3_SDA`.

Zwischen diesen FPGA-Pins und dem globalen I2C3-Bus befindet sich in der
untersuchten Schaltung kein Serienwiderstand und kein Pegelwandler.

Damit war zu prüfen, ob der FPGA während seines Power-on- oder
Konfigurationszustands selbst eine relevante Busbelastung erzeugen
könnte.

## 6. Archivierte Spartan-6-Primärquellen

Für diese Prüfung wurden zwei offizielle AMD/Xilinx-Dokumente dauerhaft
archiviert:

### UG380

`research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.pdf`

Spartan-6 FPGA Configuration User Guide, UG380, Version 2.11,
2019-03-22.

SHA-256:

`4afb6472018a9b3fa3bc1be906a0d15f86a17567d1dd7e930debfce3de362c9b`

### UG381

`research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.pdf`

Spartan-6 FPGA SelectIO Resources User Guide, UG381, Version 1.7,
2015-10-21.

SHA-256:

`4a0fc9078af54edc1104452fe5fa2bfaa82118b5501ba9fd000c1e1e2310821a`

Die HTTP-Header beider Downloads wurden ebenfalls archiviert und alle
vier Dateien in

`research/01-original-novena-docs/SHA256SUMS`

aufgenommen.

Die ausführliche FPGA-Auswertung ist zusätzlich dokumentiert in:

`research/01-original-novena-docs/notes/spartan6-i2c3-configuration-state.md`

## 7. Spartan-6-I/O-Zustand während Power-on und Konfiguration

UG380 und UG381 dokumentieren, dass die normalen User-I/O-Ausgangstreiber
während Power-on, Initialisierung und Konfiguration in einem
High-Z-Zustand gehalten werden.

P3 und P4 sind normale User-I/Os.

Damit gilt für die untersuchte Phase:

- P3 und P4 sind keine normalen aktiv treibenden FPGA-Ausgänge,
- ihre Ausgangstreiber befinden sich während Initialisierung und
  Konfiguration in High-Z.

Dieser Befund schwächt eine einfache Hypothese deutlich, nach der der
FPGA bereits durch seinen normalen POR-/Konfigurationszustand I2C3 aktiv
treibt.

## 8. HSWAPEN auf Novena

Der PVT2-A-Schaltplan zeigt:

- R13F = 4,7 kOhm von `P3.3V_DELAYED` nach `FPGA_HSWAPEN`,
- R12F = 4,7 kOhm von `FPGA_HSWAPEN` nach GND,
- R12F ist `DNP`.

Damit wird `FPGA_HSWAPEN` in der dokumentierten Bestückung nach High
gezogen.

UG380 und UG381 dokumentieren:

- HSWAPEN Low aktiviert die internen Pull-ups der User-I/Os,
- HSWAPEN High deaktiviert diese Pull-ups.

Für die dokumentierte Novena-Bestückung sind die
HSWAPEN-gesteuerten internen User-I/O-Pull-ups während der relevanten
Power-on-/Konfigurationsphase somit deaktiviert.

Damit ergibt sich für P3/P4 während dieser Phase:

- Ausgangstreiber High-Z,
- HSWAPEN-gesteuerte interne Pull-ups deaktiviert.

## 9. Grenze der FPGA-Aussage

Der High-Z-Befund darf nicht auf den gesamten Bootvorgang ausgedehnt
werden.

UG380 und UG381 dokumentieren die Freigabe von GTS während der
Startup-Sequenz.

Nach der GTS-Freigabe gehen User-I/Os in den vom geladenen User-Design
bestimmten Zustand über.

Daher bleibt offen:

- ob und wann im konkreten Novena-Bootpfad ein FPGA-Design aktiv ist,
- welchen Zustand dieses Design P3/P4 zuweist,
- ob ein Post-Configuration-Zustand des FPGA I2C3 beeinflussen könnte.

Ein solcher Post-Configuration-Einfluss wurde durch Block 3.13F nicht
ausgeschlossen.

Für einen konkreten FPGA-Post-Configuration-Fehler auf I2C3 wurde in den
untersuchten Quellen jedoch kein positiver Novena-spezifischer Beleg
gefunden.

## 10. Historischer Novena-U-Boot-Befund zum FPGA

In den untersuchten historischen Novena-U-Boot-Quellen ist

`NOVENA_FPGA_RESET_N_GPIO`

als GPIO5_IO07 definiert.

Der historische SPL setzt dieses Signal auf Low.

In den untersuchten Novena-spezifischen U-Boot-Stellen wurde dagegen
keine FPGA-Bitstream-Ladeoperation gefunden.

Wichtig ist die Aussagegrenze:

`FPGA_RESET_N`

ist anhand der untersuchten Quellen nicht als identisch mit dem
dedizierten Spartan-6-Konfigurationssignal `PROGRAM_B` belegt.

Daher darf aus dem Low-Zustand von `FPGA_RESET_N` nicht abgeleitet werden,
dass sich der FPGA dadurch zwingend im Konfigurationszustand befindet
oder dass P3/P4 deshalb dauerhaft High-Z bleiben.

## 11. Ergebnis des FPGA-Untersuchungszweigs

Belegt ist:

- P3/P4 sind direkt an I2C3 angeschlossene Spartan-6-User-I/Os.
- Während Power-on, Initialisierung und Konfiguration sind ihre
  Ausgangstreiber High-Z.
- Novena zieht HSWAPEN High.
- Damit sind die HSWAPEN-gesteuerten internen User-I/O-Pull-ups in
  dieser Phase deaktiviert.

Das schwächt den normalen FPGA-POR-/Konfigurationszustand als einfache
Erklärung für die untersuchte I2C3-Busbelastung deutlich.

Nicht belegt ist:

- der Zustand von P3/P4 nach GTS-Freigabe,
- der konkrete Zustand eines eventuell aktiven User-Designs,
- eine vollständige Ausschließung des FPGA während des gesamten
  Bootvorgangs.

## 12. ES8328-Dokumentationssuche

Die lokale Quelleninventur ergab kein archiviertes
ES8328-Herstellerdatenblatt.

Ein zusätzlicher direkter Downloadversuch über einen öffentlich
indexierten Datenblattspiegel endete mit HTTP 404.

Das dabei entstandene HTTP-Artefakt blieb ausschließlich unter `/tmp`
und wurde nicht in das Projektarchiv aufgenommen.

Eine anschließende lokale Suche auf der foobox fand ebenfalls keine
ES8328-Datenblatt-PDF.

Es wurde bewusst darauf verzichtet, zahlreiche weitere Datenblattspiegel
auszuprobieren.

Damit soll verhindert werden, dass schwach nachvollziehbare
Sekundärkopien eine höhere Evidenzstufe erhalten als die tatsächlich
verfügbaren Primär- und Projektquellen.

## 13. Lokale ES8328-Linux-Quellen

Als belastbare lokale Softwarequelle wurde der vorhandene
`novena-next/linux`-Quellbestand untersucht.

Repository:

`https://github.com/novena-next/linux.git`

Untersuchter lokaler HEAD:

`1fda06deecb61538ca3d07d256eb7c43d4e3432a`

Betreff:

`FIXME: ITE workaround`

Datum:

`2020-01-27T05:13:57+01:00`

Die Aussagen dieses Abschnitts beziehen sich ausdrücklich auf diesen
untersuchten historischen Quellstand und werden nicht pauschal auf jede
Linux-Version übertragen.

## 14. ES8328-Versorgungen laut lokalem Device-Tree-Binding

Die Datei

`Documentation/devicetree/bindings/sound/es8328.txt`

beschreibt:

- `DVDD-supply` als Versorgung des digitalen Kerns mit 1,8 bis 3,6 V,
- `AVDD-supply` als analoge Versorgung,
- `PVDD-supply` als Versorgung der digitalen I/Os mit 1,8 bis 3,6 V,
- eine weitere analoge Ausgangsversorgung.

Für die Root-Cause-Frage ist insbesondere die Beschreibung von PVDD als

`digital IO supply voltage 1.8 - 3.6V`

relevant.

Damit ist aus dem untersuchten lokalen Linux-Quellbestand direkt belegt,
dass PVDD als Digital-I/O-Versorgung des ES8328 modelliert wird.

## 15. ES8328-Regulator-Handling im lokalen Codec-Treiber

Die Datei

`sound/soc/codecs/es8328.c`

definiert vier Regulator-Supplies:

- DVDD,
- AVDD,
- PVDD,
- HPVDD.

Der Treiber holt diese Supplies gemeinsam über
`devm_regulator_bulk_get()`.

Der untersuchte Treiber enthält außerdem einen Suspend-Pfad, der nach
dem Abschalten des Codec-Clocks

`regulator_bulk_disable()`

für die ES8328-Supplies aufruft.

Beim Resume werden die Supplies über

`regulator_bulk_enable()`

wieder eingeschaltet und der Regcache anschließend synchronisiert.

Auch beim Component-Probe werden die Supplies gemeinsam eingeschaltet.

Damit ist belegt, dass der untersuchte historische Linux-Treiber einen
Betriebszustand vorsieht, in dem die ES8328-Supplies vollständig
abgeschaltet werden können.

## 16. Keine besondere Powered-off-I2C-Sequenz gefunden

Der untersuchte I2C-Treiber

`sound/soc/codecs/es8328-i2c.c`

bindet den ES8328 über Regmap an I2C an.

In den untersuchten relevanten Stellen wurde keine besondere
Abschaltsequenz für CCLK/CDATA und keine dokumentierte Behandlung eines
Zustands gefunden, in dem:

- DVDD beziehungsweise PVDD abgeschaltet sind,
- die externen I2C-Leitungen aber weiterhin auf High gezogen werden.

Aus dem Fehlen einer solchen Sequenz folgt nicht, dass dieser Zustand
elektrisch sicher oder unsicher ist.

Es liefert insbesondere keinen Beweis für oder gegen einen internen
Clamp-, Backfeed- oder sonstigen Strompfad.

## 17. Verbindung von Linux-Supply-Modell und Novena-Schaltung

Die Kombination aus lokaler Linux-Quelle und Novena-Schaltung ist für
die Root-Cause-Eingrenzung besonders relevant.

Die Linux-Quelle beschreibt:

- DVDD als Digital-Core-Supply,
- PVDD als Digital-I/O-Supply.

Die Novena-Schaltung verbindet sowohl DVDD als auch PVDD mit
`AUD_P3.3V`.

Diese Audio-Versorgung kann abgeschaltet werden.

Gleichzeitig werden auf der Codec-Seite:

- `AUD_I2C3_SCL`,
- `AUD_I2C3_SDA`

weiterhin über jeweils 1 kOhm aus `P3.3V_DELAYED` hochgezogen.

Damit ist folgende Versorgungskonstellation direkt durch die
untersuchten Quellen gestützt:

- ES8328 Digital-Core-Supply abgeschaltet,
- ES8328 Digital-I/O-Supply abgeschaltet,
- externe Codec-seitige I2C-Pull-ups weiterhin versorgt.

Das ist ein konkreter Power-Domain-Grenzzustand.

Der interne elektrische Effekt dieses Zustands ist damit noch nicht
bestimmt.

## 18. Bewertung von M1 – Clamp oder Busbelastung

M1 bleibt mit der dokumentierten Topologie vereinbar.

Ein unversorgter Digital-I/O-Power-Domain bei weiterhin extern
hochgezogenen digitalen Leitungen ist als Untersuchungsrichtung
relevant.

Nicht belegt ist jedoch:

- die interne Eingangsschaltung von CCLK/CDATA,
- eine konkrete Schutzdiode,
- ein Clamp-Pfad,
- eine resultierende Pinspannung,
- ein resultierender Strom,
- eine daraus folgende statische Busbelastung.

M1 bleibt daher offen und darf nicht als bewiesener Mechanismus
bezeichnet werden.

## 19. Bewertung von M2 – Backfeeding

M2 bleibt ebenfalls mit der dokumentierten Versorgungskonstellation
vereinbar.

Nicht belegt sind jedoch:

- ein konkreter interner Backfeed-Pfad,
- die Richtung eines möglichen Stromflusses,
- dessen Größenordnung,
- eine dadurch entstehende Teilversorgung des Codecs,
- ein Zusammenhang eines solchen Stroms mit der beobachteten
  I2C3-Fehlersignatur.

Ohne Herstellerangabe oder elektrische Messung bleibt M2 offen.

## 20. Bewertung von M3 – Pull-up-/Power-Domain-Interaktion

M3 erhält durch Block 3.13F die stärkste direkte Quellenstützung der vier
betrachteten Mechanismuskandidaten.

Belegt beziehungsweise direkt aus den Quellen ableitbar ist:

- PVDD wird im untersuchten Linux-Binding als Digital-I/O-Supply
  beschrieben.
- Novena versorgt PVDD aus `AUD_P3.3V`.
- `AUD_P3.3V` kann abgeschaltet werden.
- Die Codec-seitigen I2C-Pull-ups werden dagegen aus
  `P3.3V_DELAYED` versorgt.
- Damit können die externen digitalen Leitungen weiterhin hochgezogen
  sein, obwohl die Digital-I/O-Versorgung des Codecs abgeschaltet ist.

Dieser Power-Domain-Grenzzustand ist belegt.

Nicht bewiesen ist, welcher interne elektrische Mechanismus daraus
gegebenenfalls den I2C3-Fehler erzeugt.

M3 darf deshalb nicht als Nachweis eines bestimmten Clamp- oder
Backfeed-Mechanismus gelesen werden.

## 21. Bewertung von M4 – Abschalttransient

Die Novena-Schaltung enthält einen aktiven Audio-Power-Schaltkreis mit
Q11A, Q10A und Q12A.

Die Schaltungsdokumentation beschreibt dabei unter anderem eine aktive
Pulldown-Funktion im Audio-Codec-Power-/Reset-Umfeld.

Damit bleibt ein zeitabhängiger Abschalttransient als Mechanismus mit der
Schaltung vereinbar.

Nicht vorhanden sind jedoch:

- gemessene Spannungsverläufe,
- gemessene Stromverläufe,
- eine zeitliche Korrelation zwischen Versorgungstransient und
  I2C3-Fehler,
- ein Herstellerbeleg, der genau diesen Mechanismus beschreibt.

M4 bleibt deshalb offen und nicht nachgewiesen.

## 22. Vergleich M1 bis M4

Nach Block 3.13F ergibt sich folgende Evidenzlage:

### M1

Clamp oder statische Busbelastung:

mit der Topologie vereinbar, aber interner Mechanismus nicht belegt.

### M2

Backfeeding:

mit der Topologie vereinbar, aber Strompfad und Wirkung nicht belegt.

### M3

Pull-up-/Power-Domain-Interaktion:

von M1 bis M4 am unmittelbarsten durch Schaltung und lokales
Linux-Supply-Modell gestützt.

Der konkrete daraus entstehende elektrische Fehlermechanismus bleibt
nicht bewiesen.

### M4

Abschalttransient:

mit dem aktiven Audio-Power-Schaltkreis vereinbar, aber ohne Messung
nicht nachgewiesen.

Damit können M1, M2 und M4 nicht seriös ausgeschlossen werden.

M3 beschreibt am besten den direkt belegten Power-Domain-Grenzzustand,
ist aber noch keine vollständige mikroskopische Root-Cause-Erklärung.

## 23. Quellenübergreifende Gesamtsicht

Die stärkste Aussage entsteht nicht aus einem einzelnen Dokument,
sondern aus mehreren voneinander verschiedenen Evidenzlinien.

### Schaltung

Die ES8328-Digitalversorgungen können abgeschaltet sein, während die
Codec-seitigen I2C-Pull-ups weiter versorgt werden.

### Historischer Linux-/Novena-Befund

Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

dokumentiert historisch, dass das Abschalten von `es8328-power` nach
damaliger Beobachtung I2C3 störte und mehrere I2C3-Funktionen
beeinträchtigte.

### Aktueller unabhängiger Versuch

H3-1R änderte nur die ES8328-Versorgung auf dauerhaft eingeschaltet.

Danach waren fünf von fünf vorregistrierten echten POR-Kaltstarts
erfolgreich.

### FPGA-Primärquellen

Die Herstellerdokumentation schwächt einen normalen
Spartan-6-POR-/Konfigurationszustand als einfache konkurrierende
Busbelastung, weil die relevanten User-I/O-Ausgangstreiber während
dieser Phase High-Z sind und Novenas HSWAPEN-Beschaltung die internen
Pull-ups deaktiviert.

Diese Evidenzlinien sind miteinander vereinbar und zeigen auf denselben
kausalen Bereich: die ES8328-/Audio-Power-Domain und ihre Verbindung mit
I2C3.

## 24. Root-Cause-Ebene nach Block 3.13F

Nach Block 3.13F kann die Root Cause auf der Ebene des kausalen Bereichs
deutlich enger beschrieben werden.

Stark gestützt ist:

**Das Abschalten der ES8328-/Audio-Power-Domain ist kausal an der
untersuchten I2C3-Kaltstartstörung beteiligt.**

Ebenfalls direkt gestützt ist der dabei entstehende
Power-Domain-Grenzzustand:

**Die Digital-I/O-Versorgung des ES8328 kann abgeschaltet sein, während
seine externen I2C-Leitungen weiterhin aus `P3.3V_DELAYED` hochgezogen
werden.**

Nicht bestimmt ist:

**welcher interne oder transiente elektrische Mechanismus innerhalb
dieses Grenzzustands die konkrete I2C3-Störung erzeugt.**

## 25. Aussagegrenze

Block 3.13F beweist nicht:

- einen internen ES8328-Clamp-Pfad,
- Backfeeding in eine bestimmte Versorgung,
- einen bestimmten Stromwert,
- eine bestimmte Spannung an CCLK oder CDATA,
- einen bestimmten Abschalttransienten,
- die vollständige Ausschließung des FPGA nach Eintritt in den
  User-Mode,
- die elektrische Identität jedes historischen I2C3-Fehlers mit der
  aktuellen Signatur `A=81/80 -> M0=93/80`.

Insbesondere darf aus einer absoluten Eingangsspannungsgrenze eines
Bauteils nicht ohne weitere Dokumentation auf eine bestimmte interne
Schutzdiode oder einen Backfeed-Pfad geschlossen werden.

Für eine weitere Trennung von M1, M2, M3 und M4 wären zusätzliche
belastbare Herstellerinformationen über den unversorgten
CCLK/CDATA-Zustand oder elektrische Messungen erforderlich.

Solche Messungen stehen für dieses Projekt nicht zur Verfügung.

## 26. Konsequenz für weitere Kernel-Experimente

Block 3.13F liefert keinen Anlass für Patch 0007.

Ebenso besteht kein Anlass:

- STMPE811 erneut zu isolieren,
- den bereits geprüften Single-Master-Pfad zu wiederholen,
- zusätzliche IT6251-Delays als Root-Cause-Test einzubauen,
- `regulator-always-on` routinemäßig wieder zurückzunehmen.

Die dauerhafte Versorgung des ES8328 bleibt die dokumentierte
Board-Konfiguration.

Die verbleibende Unsicherheit betrifft den exakten elektrischen
Mechanismus und nicht die praktische Board-Konfiguration.

## 27. Status des FPGA-Kandidaten

Für den FPGA gilt nach Block 3.13F:

**deutlich geschwächt für den normalen POR-/Initialisierungs-/
Konfigurationszustand.**

Grund:

- P3/P4 sind User-I/Os,
- Ausgangstreiber High-Z,
- interne HSWAPEN-Pull-ups auf Novena deaktiviert.

Formal offen bleibt:

**ein möglicher Post-Configuration-Einfluss eines geladenen
User-Designs.**

Dafür wurde bislang kein konkreter positiver Novena-spezifischer Beleg
gefunden.

## 28. Status von M1 bis M4 nach Block 3.13F

### M1 – Clamp/Busbelastung

Status:

**offen; mit der Topologie vereinbar; nicht belegt.**

### M2 – Backfeeding

Status:

**offen; mit der Topologie vereinbar; nicht belegt.**

### M3 – Pull-up-/Power-Domain-Interaktion

Status:

**stark gestützter Power-Domain-Grenzzustand; exakter elektrischer
Mechanismus nicht bewiesen.**

### M4 – Abschalttransient

Status:

**offen; mit der Schaltung vereinbar; ohne Messung nicht nachgewiesen.**

## 29. Abschluss von Block 3.13F

Block 3.13F beendet die gezielte quellenbasierte Suche nach einer
weiteren Unterscheidung von M1 bis M4.

Der normale Spartan-6-POR-/Konfigurationszustand wurde als einfache
konkurrierende Erklärung deutlich geschwächt.

Für den ES8328 wurde aus lokalen historischen Linux-Quellen zusätzlich
belegt, dass PVDD als Digital-I/O-Versorgung modelliert wird und der
Codec-Treiber die Versorgungen gemeinsam abschalten kann.

Zusammen mit der Novena-Schaltung ergibt sich damit ein direkt
dokumentierter Power-Domain-Grenzzustand:

- Digital-Core- und Digital-I/O-Versorgung des ES8328 können aus sein,
- die Codec-seitigen I2C-Pull-ups können gleichzeitig weiter versorgt
  sein.

Die vorhandenen Quellen reichen nicht aus, um daraus seriös einen
bestimmten internen Clamp-, Backfeed- oder transienten Mechanismus
abzuleiten.

Die Root-Cause-Untersuchung wird daher auf folgender Evidenzstufe
geschlossen:

**Die ES8328-/Audio-Power-Domain-Abschaltung ist als kausaler Bereich der
untersuchten I2C3-Kaltstartstörung stark eingegrenzt. Der genaue
elektrische Mechanismus bleibt ohne zusätzliche Herstellerinformation
oder Hardwaremessung unbestimmt.**

Die praktische Konsequenz bleibt unverändert:

**`es8328-power` bleibt auf Novena dauerhaft eingeschaltet.**
