# Block 3.12 – Root-Cause-Kandidatenmatrix nach H3-1R

Stand: 2026-09-15

## 1. Zweck

Dieses Dokument führt die Ergebnisse der bisherigen Novena-I2C3-/Display-
Root-Cause-Untersuchung nach Abschluss von H3-1R und der dauerhaften
Übernahme von `es8328-power = regulator-always-on` zusammen.

Ziel ist nicht, die bereits getroffene Board-Entscheidung erneut zu testen,
sondern den noch ungeklärten elektrischen Mechanismus der beobachteten
ES8328-/I2C3-Wechselwirkung systematisch einzugrenzen und daraus den nächsten
falsifizierbaren Untersuchungsschritt abzuleiten.

## 2. Gesicherter Ausgangspunkt

Ausgangscheckpoint ist Commit:

`be1dabc653ea9937ba05a546c0de1a8c2143e888`

Die dauerhafte Board-Konfiguration lautet:

`es8328-power = regulator-always-on`

Sie wird nicht mehr als experimentelle H3-1R-Intervention behandelt.

Der gesicherte Baseline-Cold-FAIL zeigt:

`A=81/80 -> M0=93/80`

mit `arbitration lost` und `ret=-11`.

Der erfolgreiche Same-Boot-LDB-Rebind zeigt dagegen:

`A=81/80 -> M0=81/a0`

Patch 0006 lokalisiert die Divergenz damit spätestens auf den Übergang in den
Master-/START-Zustand. Vor dem MSTA-Versuch ist im sichtbaren Registersnapshot
noch kein IAL vorhanden; beim fehlerhaften M0 ist IAL bereits gesetzt und
MSTA nicht mehr gesetzt.

Im gesicherten Baseline-Cold-FAIL wird zuvor protokolliert:

`es8328-power: disabling`

Zwischen dieser Abschaltung und dem ersten beobachteten I2C3-Fehler liegen
ungefähr 10,716 Sekunden.

Für H3-1R wurde als einzige semantische Device-Tree-Änderung
`regulator-always-on` am bestehenden Audio-Regulator ergänzt.

Alle fünf vorregistrierten echten POR-Cold-Boots bestanden.
In allen fünf vollständigen Kernel-Logs galt:

- keine protokollierte Abschaltung von `es8328-power`,
- erster IT6251-START mit `M0=81/a0`,
- keine bekannte `93/80`-Signatur,
- kein untersuchtes I2C3-`arbitration lost`,
- erfolgreiche IT6251-Initialisierung,
- funktionierendes internes Display.

Unabhängig davon dokumentiert der historische Novena-Commit
`e48619edadbde342d79655e73654f0b21fc5e20b`, dass das Abschalten der
ES8328-Versorgung auf realer Novena-Hardware I2C3 beeinträchtigte. Auch diese
historische Änderung verwendet `regulator-always-on`.

Damit ist ein kausaler Beitrag des Abschaltens der ES8328-Versorgung zur
untersuchten I2C3-Cold-Boot-Fehlerklasse stark gestützt.

Nicht bewiesen ist der genaue elektrische Mechanismus.

## 3. Hardwarekontext der offenen Mechanismusfrage

Die PVT2-A-Unterlagen zeigen für den Audiozweig:

- I2C3_SCL -> R26A 330 Ohm -> AUD_I2C3_SCL -> ES8328E,
- I2C3_SDA -> R27A 330 Ohm -> AUD_I2C3_SDA -> ES8328E,
- eine zusätzliche dokumentierte Pull-up-/Versorgungsabhängigkeit auf der
  Audioseite,
- Versorgung des Codecs aus dem geschalteten Audio-Power-Domain.

Der ES8328-Zweig bleibt damit auch bei abgeschalteter Versorgung physisch mit
dem gemeinsam genutzten I2C3 verbunden.

I2C3 wird außerdem von weiteren Teilnehmern beziehungsweise Boardpfaden
genutzt, darunter EEPROM, FPGA-/Boardpfade und der über den Displaypfad
erreichbare IT6251.

Die Haupt-Pull-ups von I2C3 liegen auf einer anderen Versorgung. Bei den
historischen Unterlagen besteht weiterhin eine dokumentierte Diskrepanz bei
den angegebenen beziehungsweise gezeichneten Pull-up-Werten. Die tatsächliche
Bestückung der konkreten Platine wurde für diese Mechanismusfrage noch nicht
elektrisch verifiziert.

## 4. Root-Cause-Kandidatenmatrix

### M1 – Clamp- oder Bus-Loading-Effekt am unversorgten ES8328

Bewertung: **sehr hohe Priorität**

Hypothese:

Bei abgeschaltetem Audio-Power-Domain beeinflussen interne I/O- oder
Schutzstrukturen des weiterhin über die 330-Ohm-Serienwiderstände mit I2C3
verbundenen ES8328 mindestens eine Busleitung elektrisch.

Dafür spricht:

- direkte physische Kopplung des ES8328-Zweigs an I2C3,
- Abhängigkeit der Fehlerklasse vom Audio-Power-Zustand in H3-1R,
- unabhängiger historischer Befund zur ES8328-/I2C3-Wechselwirkung.

Nicht bewiesen:

- tatsächlicher SDA-/SCL-Pegel im unversorgten Zustand,
- tatsächlicher Clamp-Strom,
- konkrete interne ES8328-Struktur als Ursache.

Falsifizierbare Vorhersage:

Der abgeschaltete Audiozweig erzeugt einen gegenüber dem dauerhaft versorgten
Zustand messbar veränderten elektrischen Zustand auf SDA und/oder SCL.

### M2 – Rückspeisung des Audio-Power-Domains

Bewertung: **sehr hohe Priorität**

Hypothese:

Bei nominell abgeschaltetem `es8328-power` wird der Audio-Power-Domain über
I2C-Signale oder einen anderen weiterhin aktiven Signalpfad teilweise
rückgespeist.

Dafür spricht:

- physisch angeschlossener, aber unversorgter Codec-Zweig,
- starke experimentelle Abhängigkeit vom Power-Zustand.

Nicht bewiesen:

- Restspannung auf AUD_P3.3V beziehungsweise dem relevanten Audio-Rail,
- Strompfad und Stromrichtung,
- Beteiligung von SDA/SCL gegenüber anderen Codec-Signalen.

Falsifizierbare Vorhersage:

Bei nominell ausgeschaltetem Audio-Regulator bleibt eine relevante messbare
Spannung auf dem Audio-Power-Domain bestehen oder entsteht abhängig von den
angeschlossenen Signalen.

### M3 – Pull-up-/Power-Domain-Wechselwirkung

Bewertung: **hohe Priorität**

Hypothese:

Das Abschalten des Audio-Power-Domains verändert über die audioseitige
Beschaltung die effektive Pull-up-, Last- oder Pegelsituation des gemeinsam
genutzten I2C3.

Dafür spricht:

- dokumentierte audioseitige Pull-up-/Versorgungsabhängigkeit,
- gemeinsamer I2C3 mit eigener Haupt-Pull-up-Struktur,
- Power-State-Abhängigkeit des Fehlers.

Nicht bewiesen:

- tatsächliche Widerstandsbestückung der konkreten Novena,
- resultierende effektive Pull-up-Stärke,
- messbare Veränderung der High-Pegel oder Flanken.

Falsifizierbare Vorhersage:

Audio-Power OFF und Audio-Power ON unterscheiden sich messbar bei Buspegel,
Flankenform oder effektiver elektrischer Last.

### M4 – Transient beim Abschalten der Audio-Versorgung

Bewertung: **mittlere bis hohe Priorität**

Hypothese:

Nicht der spätere statische OFF-Zustand allein, sondern ein elektrischer
Transient beim Abschalten von `es8328-power` erzeugt oder hinterlässt den für
den späteren ersten IT6251-START problematischen Zustand.

Dafür spricht:

- die Abschaltung ist das kontrolliert veränderte Ereignis in H3-1R,
- das Verhindern der Abschaltung verhindert in 5/5 POR-Läufen die bekannte
  Fehlerklasse.

Dagegen beziehungsweise offen:

- der erste beobachtete I2C3-Fehler folgt erst ungefähr 10,716 Sekunden später,
- bisher wurde kein Abschalttransient elektrisch aufgezeichnet.

Falsifizierbare Vorhersage:

Beim Abschalten tritt auf Audio-Rail, SDA oder SCL ein reproduzierbarer
kurzzeitiger elektrischer Effekt auf, der bei dauerhaft eingeschalteter
Versorgung fehlt.

### M5 – IT6251-Power-/Reset-/POR-Zustand als zusätzlicher Faktor

Bewertung: **mittlere, nachgeordnete Priorität**

Die historischen Quellen und der APX803-Resetmonitor zeigen, dass echter POR
und Same-Boot-Rebind für den IT6251 unterschiedliche Hardwarezustände besitzen
können. Historische Software verwendet außerdem explizite Power-/Readiness-
Sequenzen.

H3-1R verändert diesen Pfad jedoch nicht und erreicht trotzdem 5/5 erfolgreiche
POR-Cold-Boots. Damit fehlt derzeit eine mit der Audio-Power-Evidence
vergleichbare kausale Intervention.

### M6 – U-Boot-2020.07-I2C3-Übergabezustand

Bewertung: **mittlere, nachgeordnete Priorität**

Historisches Novena-U-Boot konfiguriert Audio-Power und I2C3 früh und besitzt
eine aktive Bus-Recovery-Funktion. Der exakte entsprechende Zustand des heute
verwendeten U-Boot 2020.07 ist noch nicht bewiesen.

Dieser Kandidat bleibt offen, besitzt nach H3-1R aber geringeren unmittelbaren
Informationswert als die elektrische Untersuchung des Audiozweigs.

### M7 – interner i.MX6Q-Controller-, Clock-, Pinmux- oder Pad-Zustand

Bewertung: **niedrigere, weiterhin offene Priorität**

Der sichtbare Snapshot `A=81/80` beweist nicht die Gleichheit aller internen
Controller-, Clock-, Pinmux-, Pad- oder physischen Buszustände zwischen
Cold-Boot und Same-Boot-Rebind.

Gegen eine primäre reine Linux-Controllerursache spricht jedoch:

- der grundlegende START-Ablauf ist zwischen Linux 5.7 und 6.18 weitgehend
  gleich,
- der historische Novena-5.7-I2C-Treiber ist bytegleich mit dem entsprechenden
  Upstream-Stand,
- H3-1R verändert keinen Controllercode und beseitigt trotzdem in 5/5 POR-
  Läufen die bekannte Fehlerklasse.

### M8 – primäre Linux-6.18-START-/IAL-Regression

Bewertung: **sehr niedrige Priorität**

Die bisherige Quellanalyse liefert keinen starken Hinweis darauf, dass die
entscheidende START-Sequenz eine Novena-spezifische Regression von Linux 6.18
ist. Dieser Kandidat bleibt logisch möglich, ist gegenüber den elektrischen
Audio-Power-Kandidaten aber deutlich geschwächt.

### M9 – STMPE811 als Ursache der untersuchten IT6251-Fehlerklasse

Bewertung: **für diese Fehlerklasse experimentell ausgeschlossen**

Die kontrollierte STMPE811-Isolation beseitigte die STMPE-bezogenen Fehler,
nicht aber das intermittierende IT6251-Cold-Boot-Problem.

### M10 – `single-master` als Ursache des IAL-Ereignisses

Bewertung: **als Ursache experimentell ausgeschlossen**

Das isolierte Entfernen von `single-master` beseitigte das vom Controller
gemeldete IAL-Ereignis nicht. Die Eigenschaft beeinflusst die Behandlung des
Ereignisses, nicht dessen beobachtete Entstehung.

### M11 – bereits stehendes altes IAL-Bit

Bewertung: **stark ausgeschlossen**

Patch 0006 zeigt unmittelbar vor dem MSTA-Versuch noch kein IAL. Beim ersten
M0-Snapshot des fehlgeschlagenen Cold-Boots ist IAL dagegen bereits gesetzt.
Die Fehlerklasse lässt sich deshalb nicht hinreichend als lediglich stehen
gebliebenes IAL eines früheren Transfers erklären.

### M12 – I2C0-Arbitration-Lost als gemeinsame Root Cause

Bewertung: **separater offener Befund**

Die zusätzlich beobachteten Arbitration-Lost-Ereignisse auf I2C0 werden nicht
als Teil der I2C3-/IT6251-Root-Cause angenommen, solange keine Evidence beide
Fehlerpfade kausal verbindet.

## 5. Rangfolge nach H3-1R

Die aktuelle Priorisierung lautet:

1. elektrischer Zustand des unversorgten ES8328-/Audiozweigs: M1 bis M3,
2. Abschalttransient des Audio-Power-Domains: M4,
3. IT6251-POR-/Power-/Reset-Zustand: M5,
4. U-Boot-Übergabezustand: M6,
5. i.MX6Q-interner Controller-/Pad-/Clock-Zustand: M7,
6. primäre Linux-6.18-Regression: M8.

M9 und M10 sind für die untersuchte Fehlerklasse bereits experimentell
ausgeschlossen. M11 ist durch die START-Instrumentierung stark ausgeschlossen.
M12 bleibt getrennt.

## 6. Entscheidung über den nächsten Untersuchungsschritt

Der nächste Untersuchungsschritt soll keinen weiteren Kernel-Diagnosepatch
einführen und die dauerhafte Board-Entscheidung `regulator-always-on` nicht
routinemäßig zurücknehmen.

Der höchste Informationsgewinn liegt nun in der direkten Untersuchung des
elektrischen Audio-/I2C3-Zustands.

Der nächste Block wird deshalb definiert als:

**Block 3.13 – Elektrische ES8328-/I2C3-Messplanung**

Zunächst werden ausschließlich aus den bereits gesicherten Hardwareunterlagen
sichere und eindeutige Messpunkte für mindestens folgende Größen bestimmt:

- AUD_P3.3V beziehungsweise der tatsächlich geschaltete ES8328-Power-Rail,
- I2C3_SDA,
- I2C3_SCL,
- gemeinsame Masse als Messreferenz.

Für jeden Messpunkt werden vor einer Hardwaremessung dokumentiert:

- Schaltplanbezug,
- physisch geeigneter Messpunkt,
- erwarteter Zustand bei Audio-Power ON,
- für M1 bis M4 unterscheidbare Beobachtungen,
- Messmittel und erforderliche zeitliche Auflösung,
- Risiko eines Kurzschlusses oder einer Beeinflussung des I2C-Busses.

Erst danach wird entschieden, ob eine statische Multimetermessung ausreicht
oder ob Oszilloskop beziehungsweise Logic Analyzer erforderlich sind.

Eine eventuelle temporäre Wiederherstellung eines Audio-Power-OFF-Zustands
wäre ausschließlich als separat geplanter diagnostischer Vergleich zulässig.
Sie ist keine Rücknahme der dauerhaften Board-Konfiguration und darf erst nach
einem eigenen Sicherungs-, Risiko- und Wiederherstellungsplan erfolgen.

## 7. Nicht als nächster Schritt vorgesehen

Für den unmittelbar folgenden Untersuchungsschritt werden ausdrücklich nicht
vorgesehen:

- Kernel-Patch `0007`,
- pauschaler IT6251-Delay,
- zusätzlicher IT6251-Retry als vermeintlicher Fix,
- routinemäßige Rücknahme von `regulator-always-on`,
- Änderung von `single-master`,
- erneute STMPE811-Isolation,
- ungerichtete weitere Internetrecherche,
- Vermischung des separaten I2C0-Arbitration-Lost-Befunds mit I2C3.

## 8. Aussagegrenze

Block 3.12 erklärt den exakten elektrischen Mechanismus noch nicht.

Er hält fest, dass die derzeit stärkste offene Root-Cause-Frage nicht mehr die
grundsätzliche funktionale Eignung von `regulator-always-on` ist, sondern die
elektrische Ursache dafür, dass das Abschalten des weiterhin mit I2C3
verbundenen ES8328-/Audiozweigs die untersuchte Busfehlerklasse begünstigt.

Clamp-, Rückspeisungs-, Pull-up- und Transient-Effekte bleiben Hypothesen, bis
sie durch geeignete elektrische Beobachtungen voneinander unterschieden oder
falsifiziert werden.
