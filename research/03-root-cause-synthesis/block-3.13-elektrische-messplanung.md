# Block 3.13 – Elektrische ES8328-/I2C3-Messplanung

## 1. Zweck und Ausgangspunkt

Block 3.13 setzt die in Block 3.12 festgelegte elektrische
Root-Cause-Untersuchung fort.

Die dauerhafte Novena-Board-Konfiguration

`es8328-power = regulator-always-on`

wird dabei nicht erneut als offene Konfigurationsfrage behandelt.

Gesichert ist aus den vorherigen Untersuchungsblöcken:

- Im Baseline-Cold-FAIL wird `es8328-power` ungefähr 10,716 Sekunden vor
  dem ersten beobachteten I2C3-Fehler abgeschaltet.
- Der gesicherte Baseline-Cold-FAIL zeigt beim ersten untersuchten
  Master-START:
  `A=81/80 -> M0=93/80`.
- Der erfolgreiche Same-Boot-LDB-Rebind zeigt dagegen:
  `A=81/80 -> M0=81/a0`.
- Die Änderung H3-1R fügte ausschließlich `regulator-always-on` für
  `es8328-power` hinzu.
- Mit dieser Änderung waren 5/5 vorab festgelegte echte POR-Cold-Boots
  erfolgreich.
- In diesen fünf vollständigen Kernel-Logs fehlen die bekannte
  `93/80`-Signatur und das untersuchte I2C3-`arbitration lost`.
- Ein historischer Novena-Kernel-Commit dokumentiert unabhängig, dass
  das Abschalten der ES8328-Versorgung auf realer Novena-Hardware I2C3
  stören kann.

Damit ist ein kausaler Beitrag des Abschaltens der ES8328-Versorgung zur
untersuchten I2C3-Cold-Boot-Fehlerklasse stark gestützt.

Der genaue elektrische Mechanismus ist weiterhin nicht bewiesen.

## 2. Korrektur der PVT2-A-Schaltplantopologie

Die visuelle Gegenprüfung der archivierten Originalseite 14 des
Novena-PVT2-A-Schaltplans hat eine frühere, aus der PDF-Textextraktion
abgeleitete Interpretation korrigiert.

Die korrigierte Schaltplan-Evidence ist dauerhaft in

`research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md`

festgehalten.

### 2.1 Serienwiderstände des ES8328-Zweigs

Der ES8328-Zweig ist über zwei 330-Ohm-Serienwiderstände mit dem
globalen I2C3 verbunden:

`I2C3_SCL -> R26A 330R -> AUD_I2C3_SCL -> ES8328E`

`I2C3_SDA -> R27A 330R -> AUD_I2C3_SDA -> ES8328E`

R26A und R27A bilden damit elektrisch besonders wertvolle
Beobachtungsstellen, weil jeweils eine Seite zum globalen I2C3 und die
andere Seite zum ES8328-/Audiozweig gehört.

### 2.2 Lage der Pull-ups R10B und R11B

Die visuelle Prüfung des Originalschaltplans zeigt:

- R10B = 1 kohm von `P3.3V_DELAYED` nach `AUD_I2C3_SCL`
- R11B = 1 kohm von `P3.3V_DELAYED` nach `AUD_I2C3_SDA`

Die Pull-ups liegen damit auf der ES8328-Seite von R26A/R27A.

Die frühere Zuordnung von R10B/R11B direkt zur globalen
`I2C3_SCL`-/`I2C3_SDA`-Seite war falsch.

### 2.3 Korrektur zu R11A

R11A ist kein I2C-Pull-up.

Die visuelle Prüfung der Originalseite 14 ordnet R11A dem
Mikrofonbereich zu. Der Widerstand ist dort mit 10 kohm beschriftet und
steht im Zusammenhang mit `MIC_DIFF_P`.

Die frühere, aus der Text-Extraktion abgeleitete Interpretation

`R11A = 1 kohm von AUD_P3.3V nach AUD_I2C3_SDA`

ist damit verworfen.

## 3. Relevante Power-Domain-Struktur

Der ES8328E wird aus dem geschalteten Audio-Power-Rail `AUD_P3.3V`
versorgt.

Die Audio-Power-Schaltung auf derselben Schaltplanseite enthält unter
anderem:

- Q11A, FDN304P, im Pfad zwischen `P3.3V_DELAYED` und `AUD_P3.3V`,
- Q10A, 2N7002W, im `AUD_PWRON`-Steuerpfad,
- Q12A, 2N7002W, in einem mit
  `active pulldown to ensure audio codec reset`
  bezeichneten Schaltungsteil.

Die genaue Transistor-Stromführung dieses Pulldown-Pfads wird für die
Messplanung nicht weiter behauptet, als sie aus dem Schaltplan sicher
abgeleitet werden kann.

Entscheidend ist zunächst die getrennte Versorgungstopologie:

    globaler I2C3
         |
      R26A/R27A
         |
    AUD_I2C3_SCL/SDA
         |
         +---- ES8328E-I/O
         |
      R10B/R11B
         |
    P3.3V_DELAYED

während die Versorgung des Codecs separat erfolgt:

    P3.3V_DELAYED
         |
    Audio-Power-Schaltung
         |
      AUD_P3.3V
         |
       ES8328E

Damit kann schaltungstechnisch ein Zustand existieren, in dem
`P3.3V_DELAYED` vorhanden ist und die ES8328-seitigen I2C-Netze über
R10B/R11B hochgezogen werden, während `AUD_P3.3V` abgeschaltet ist.

Dieser Schaltungszustand ist gesichert.

Nicht gesichert ist bislang, welcher Strom tatsächlich in diesem Zustand
durch die ES8328-I/O-Struktur fließt.

## 4. Konsequenz für die Root-Cause-Kandidaten

Der korrigierte Schaltplanbefund erhöht den Informationswert einer
direkten Untersuchung der bereits in Block 3.12 priorisierten Kandidaten.

### M1 – Clamp- oder Bus-Loading-Effekt

Ein unversorgter ES8328 besitzt weiterhin elektrische Verbindungen zu
`AUD_I2C3_SCL` und `AUD_I2C3_SDA`.

Da diese Netze über R10B/R11B aus `P3.3V_DELAYED` hochgezogen werden,
ist ein Clamp- oder Loading-Effekt eine konkrete elektrische Hypothese.

Der Schaltplan beweist einen solchen Effekt nicht.

### M2 – Rückspeisung des Audio-Power-Domains

Die Kombination aus versorgten I2C-Pull-ups und abgeschalteter
Codec-Versorgung macht einen möglichen Rückspeisepfad über die
ES8328-I/O-Struktur zu einer konkreten Hypothese.

Nicht bewiesen sind:

- das Vorhandensein eines solchen internen Pfades,
- seine Stromstärke,
- die betroffene I2C-Leitung,
- eine dadurch verursachte Busstörung.

### M3 – Pull-up-/Power-Domain-Wechselwirkung

M3 wird durch den korrigierten Schaltplan besonders konkret:

Die ES8328-seitigen Pull-ups gehören zu `P3.3V_DELAYED`, während die
eigentliche Codec-Versorgung zu `AUD_P3.3V` gehört.

Damit handelt es sich tatsächlich um zwei unterschiedliche
Versorgungszustände, deren Wechselwirkung untersucht werden kann.

### M4 – Abschalttransient

Der Schaltplan enthält eine aktive Audio-Power-Steuerung und einen
ausdrücklich bezeichneten Pulldown.

Ein zeitabhängiger Effekt während des Abschaltens von `AUD_P3.3V`
bleibt daher als Kandidat offen.

Der statische Schaltplan beweist keinen solchen Transienten.

## 5. Vorläufige elektrische Messgrößen

Aus der korrigierten Topologie ergeben sich für die weitere Planung
mindestens folgende Messgrößen:

1. `AUD_P3.3V`
2. globale Seite von `I2C3_SCL` an R26A
3. ES8328-seitiges `AUD_I2C3_SCL` an R26A
4. globale Seite von `I2C3_SDA` an R27A
5. ES8328-seitiges `AUD_I2C3_SDA` an R27A
6. `P3.3V_DELAYED`
7. sichere gemeinsame Masse

Besonders wichtig ist der Vergleich beider Seiten von R26A und R27A.

Eine messbare Spannungsdifferenz über einem der 330-Ohm-Widerstände
würde einen Strom durch den entsprechenden ES8328-I2C-Zweig anzeigen.

Aus einer solchen Beobachtung allein dürfte jedoch noch nicht ohne
weitere Evidence auf den internen Strompfad im ES8328 geschlossen
werden.

## 6. Noch nicht festgelegte physische Messpunkte

Die Schaltplan-Netze sind jetzt ausreichend bestimmt.

Noch nicht bestimmt ist, welche konkreten Pads, Bauteilanschlüsse oder
Testpunkte auf der tatsächlich getesteten Novena-Platine:

- eindeutig identifiziert,
- mechanisch erreichbar,
- elektrisch sicher,
- und für Multimeter- beziehungsweise Oszilloskop-Tastköpfe geeignet

sind.

Insbesondere wird noch nicht vorausgesetzt, dass die Pads von R26A und
R27A auf der realen Platine ohne erhöhtes Kurzschlussrisiko direkt
messbar sind.

Auch ein geeigneter Massepunkt ist vor einer Hardwaremessung eindeutig
festzulegen.

## 7. Nächster Teilschritt – physische Messpunktkarte

Der nächste Teilschritt von Block 3.13 ist die physische Zuordnung der
Schaltplan-Netze zur realen PVT2-A-Platine.

Priorität besitzen:

- R26A und beide Seiten des Widerstands,
- R27A und beide Seiten des Widerstands,
- ein sicherer Punkt für `AUD_P3.3V`,
- ein sicherer Punkt für `P3.3V_DELAYED`,
- ein mechanisch sicherer Massepunkt.

Dazu werden zunächst vorhandene Layout-Unterlagen und geeignete
Platinenabbildungen ausgewertet.

Erst wenn ein Messpunkt eindeutig zugeordnet ist, werden für ihn
festgelegt:

- erwartete Spannung beziehungsweise Signalform,
- Messgerät,
- erforderliche zeitliche Auflösung,
- Tastkopf-/Masseanschluss,
- Kurzschluss- und Belastungsrisiko.

## 8. Noch keine Hardwaremessung

Aus diesem Dokument folgt noch keine Freigabe für eine Messung an der
Novena.

Insbesondere wird derzeit nicht:

- `regulator-always-on` zurückgenommen,
- ein Audio-Power-OFF-Test erzeugt,
- ein Tastkopf an R26A oder R27A angeschlossen,
- ein Logic Analyzer an I2C3 angeschlossen,
- Kernel-Patch `0007` begonnen.

Eine temporäre Wiederherstellung eines Audio-Power-OFF-Zustands wäre
nur als separat vorbereiteter diagnostischer Vergleich zulässig.

Dafür wären vorher ein eigener Sicherungs-, Risiko- und
Wiederherstellungsplan erforderlich.

## 9. Aussagegrenze

Die neue Schaltplan-Evidence erklärt, warum der ES8328-/Audiozweig eine
besonders plausible elektrische Untersuchungsstelle ist.

Zusammen mit der bereits gesicherten H3-1R-Evidence ergibt sich eine
starke Begründung dafür, den Mechanismus am ES8328-/I2C3-Übergang
direkt zu untersuchen.

Noch nicht bewiesen sind:

- Clamp-Verhalten des unversorgten ES8328,
- Rückspeisung über die I2C-Pins,
- ein konkreter interner ES8328-Strompfad,
- eine bestimmte betroffene Leitung,
- ein Abschalttransient als unmittelbare Ursache des späteren IAL,
- die exakte elektrische Kette vom Abschalten von `AUD_P3.3V` bis zum
  beobachteten `M0=93/80`.

Block 3.13 bleibt deshalb eine Root-Cause-Untersuchung und keine
nachträgliche Begründung einer bereits angenommenen Mechanismusursache.
