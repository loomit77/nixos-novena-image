# Block 3.14B – Controllerseitige START-Zustandsrekonstruktion

## 1. Ziel

Block 3.14B rekonstruiert den unmittelbar am i.MX6-I2C-Controller
beobachtbaren Ablauf eines fehlgeschlagenen und eines erfolgreichen
I2C3-START-Versuchs.

Ausgangspunkt sind die bereits mit Diagnose-Patch 0006 gesicherten
Registerbeobachtungen A, M0 bis M7, B, C, D und E.

Die Untersuchung ist eine reine Quellen- und Logrekonstruktion.

Es wurden keine neuen Kernel-Patches, Builds, Bootversuche oder
Hardwaremessungen durchgeführt.

Block 3.14B untersucht ausdrücklich nicht erneut, ob die
ES8328-/Audio-Power-Domain an der Kaltstartstörung beteiligt ist. Diese
Frage wurde bereits in den vorhergehenden Root-Cause-Blöcken behandelt.

Ziel ist stattdessen die engere Frage:

**Was lässt sich aus dem exakten Linux-6.18.49-i2c-imx-Code und den
bereits gesicherten M0-bis-M7-Messungen über den controllerseitigen
START-Fehler sagen?**

## 2. Ausgangspunkt

Diagnose-Patch 0006 hatte bereits folgenden direkten A/B-Vergleich
geliefert:

Cold-Boot-FAIL:

`A=81/80 -> M0=93/80`

Same-Boot-LDB-Rebind-PASS:

`A=81/80 -> M0=81/a0`

Der Zustand A unmittelbar vor dem Setzen von `MSTA` ist damit in beiden
Fällen gleich.

Die Divergenz ist bereits bei der ersten beobachtbaren
Post-MSTA-Probe M0 vorhanden.

Block 3.14B löst die Bedeutung dieser Registerwerte und den danach
ausgeführten Treiberpfad vollständig auf.

## 3. Untersuchte Kernelquelle

Für die Analyse wurde der bereits vorhandene Linux-6.18.49-Quellbaum
mit Diagnose-Patch 0006 verwendet:

`/home/loomit/novena-analysis/linux-6.18.49-i2c-0006`

Untersuchte Datei:

`drivers/i2c/busses/i2c-imx.c`

Die Diagnoseänderungen des Projekts sind in:

`kernel/0006-i2c-imx-debug-start-transition.patch`

dokumentiert.

Die Untersuchung dieses Blocks verändert weder Kernelquelle noch Patch.

## 4. Relevante I2SR-Bits

Der untersuchte `i2c-imx`-Treiber definiert:

`I2SR_RXAK = 0x01`

`I2SR_IIF = 0x02`

`I2SR_SRW = 0x04`

`I2SR_IAL = 0x10`

`I2SR_IBB = 0x20`

`I2SR_IAAS = 0x40`

`I2SR_ICF = 0x80`

Damit ergeben sich für die in den Messungen auftretenden Werte:

### 0x81

`0x81 = 0x80 + 0x01`

Gesetzt sind:

- `ICF`
- `RXAK`

Nicht gesetzt sind insbesondere:

- `IBB`
- `IAL`
- `IIF`

### 0x93

`0x93 = 0x80 + 0x10 + 0x02 + 0x01`

Gesetzt sind:

- `ICF`
- `IAL`
- `IIF`
- `RXAK`

Nicht gesetzt ist insbesondere:

- `IBB`

### 0x83

`0x83 = 0x80 + 0x02 + 0x01`

Gesetzt sind:

- `ICF`
- `IIF`
- `RXAK`

Nicht gesetzt sind insbesondere:

- `IAL`
- `IBB`

### 0xa1

`0xa1 = 0x80 + 0x20 + 0x01`

Gesetzt sind:

- `ICF`
- `IBB`
- `RXAK`

Nicht gesetzt ist insbesondere:

- `IAL`

## 5. Relevante I2CR-Bits

Der untersuchte Treiber definiert:

`I2CR_DMAEN = 0x02`

`I2CR_RSTA = 0x04`

`I2CR_TXAK = 0x08`

`I2CR_MTX = 0x10`

`I2CR_MSTA = 0x20`

`I2CR_IIEN = 0x40`

`I2CR_IEN = 0x80`

Daraus ergeben sich:

### 0x80

`0x80 = IEN`

Der Controller ist aktiviert.

`MSTA` ist nicht gesetzt.

### 0xa0

`0xa0 = IEN + MSTA`

Der Controller ist aktiviert und befindet sich im Masterzustand.

### 0xf8

`0xf8 = IEN + IIEN + MSTA + MTX + TXAK`

Der Controller ist aktiviert, Master und für die folgende
Übertragungsphase konfiguriert.

## 6. Bedeutung der Diagnose-Snapshots

Die Diagnoseausgaben verwenden immer die Reihenfolge:

`I2SR/I2CR`

Ein Wert wie:

`93/80`

bedeutet deshalb:

`I2SR=0x93`

gefolgt von:

`I2CR=0x80`

Wichtig ist, dass diese beiden Registerwerte nicht atomar gleichzeitig
erfasst werden.

Patch 0006 führt für jedes M-Sample zuerst einen MMIO-Lesezugriff auf
`I2SR` und danach einen MMIO-Lesezugriff auf `I2CR` aus.

`93/80` darf deshalb nicht als exakt gleichzeitiger Hardware-Snapshot
beider Register interpretiert werden.

Die Reihenfolge der Beobachtung ist:

1. `I2SR` lesen,
2. danach `I2CR` lesen.

## 7. Exakter START-Pfad

Im untersuchten `i2c_imx_start()` wird unmittelbar vor dem
START-Versuch der Snapshot A aufgenommen.

Danach wird `I2CR` gelesen, `I2CR_MSTA` gesetzt und der neue Wert in
`I2CR` geschrieben.

Der relevante Ablauf lautet logisch:

`A erfassen`

`-> I2CR lesen`

`-> MSTA setzen`

`-> I2CR schreiben`

`-> M0 bis M7 erfassen`

`-> i2c_imx_bus_busy()`

`-> C erfassen`

Patch 0006 führt M0 bis M7 als enge Folge geordneter Registerlesungen
aus.

Innerhalb dieses Fensters befinden sich bewusst:

- keine Delays,
- keine Logausgaben,
- keine Timestamp-Abfragen.

Für jedes Sample wird zuerst `I2SR` und anschließend `I2CR` gelesen.

B entspricht M0.

## 8. Fehlgeschlagener Cold-Boot-START

Der gesicherte Cold-Boot-Fehler zeigt vor dem MSTA-Versuch:

`A=81/80`

Damit gilt unmittelbar vor dem START-Versuch:

I2SR:

- `ICF=1`
- `IAL=0`
- `IBB=0`

I2CR:

- `IEN=1`
- `MSTA=0`

Nach dem Schreiben von `MSTA` zeigt Patch 0006:

`M0=93/80`

`M1=93/80`

`M2=93/80`

`M3=93/80`

`M4=93/80`

`M5=93/80`

`M6=93/80`

`M7=93/80`

Damit ist bereits bei der ersten beobachtbaren I2SR-Lesung nach dem
MSTA-Schreibzugriff:

- `IAL=1`
- `IIF=1`
- `IBB=0`

Bei der jeweils danach ausgeführten I2CR-Lesung ist:

- `IEN=1`
- `MSTA=0`

Der Zustand bleibt über alle acht unmittelbar aufeinanderfolgenden
M-Samples gleich.

Es wurde innerhalb M0 bis M7 kein Zwischenzustand wie:

`81/a0`

oder:

`a1/a0`

beobachtet.

## 9. Erfolgreicher Same-Boot-START

Beim erfolgreichen Same-Boot-LDB-Rebind ist der Pre-MSTA-Zustand
ebenfalls:

`A=81/80`

Nach dem MSTA-Schreibzugriff zeigt ein direkt beobachteter erfolgreicher
START:

`M0=81/a0`

`M1=81/a0`

`M2=81/a0`

`M3=81/a0`

`M4=81/a0`

`M5=81/a0`

`M6=a1/a0`

`M7=a1/a0`

Damit ist bereits bei M0:

- `MSTA=1`
- `IAL=0`
- `IBB=0`

Bei M6 und M7 ist zusätzlich:

- `IBB=1`

Der erfolgreiche Ablauf zeigt damit einen direkt beobachteten
Zwischenzustand:

`81/a0`

in dem der Controller `MSTA` bereits als gesetzt zurückliefert, während
`IBB` noch nicht gesetzt ist.

Erst etwas später wird:

`a1/a0`

beobachtet.

Damit ist belegt, dass `MSTA` und `IBB` in dieser Diagnose nicht
notwendigerweise in derselben Registerbeobachtung sichtbar werden.

Andere erfolgreiche Transfers desselben Evidence-Satzes zeigen
teilweise über das gesamte M0-bis-M7-Fenster:

`81/a0`

Das bedeutet lediglich, dass das sehr kurze achtfache Messfenster enden
kann, bevor `IBB` beobachtbar wird.

Es widerspricht dem erfolgreichen START nicht.

## 10. Direkter FAIL-/PASS-Vergleich

Der entscheidende Vergleich lautet:

### FAIL

`A=81/80`

`-> MSTA schreiben`

`-> M0=93/80`

`-> M1...M7=93/80`

### PASS

`A=81/80`

`-> MSTA schreiben`

`-> M0=81/a0`

`-> später a1/a0`

Der Zustand unmittelbar vor dem MSTA-Schreibzugriff ist gleich.

Im erfolgreichen Fall wird `MSTA` bereits bei der ersten
Post-MSTA-I2CR-Lesung als gesetzt zurückgelesen.

Im Fehlerfall ist dagegen bereits bei der ersten
Post-MSTA-I2SR-Lesung `IAL` gesetzt und bei der unmittelbar folgenden
I2CR-Lesung `MSTA` nicht mehr gesetzt.

Damit liegt die beobachtbare Divergenz zwischen:

- dem MSTA-Schreibzugriff,
- und der allerersten danach ausgeführten I2SR-Lesung.

Die Instrumentierung kann den internen Hardwareablauf innerhalb dieses
noch kleineren Fensters nicht weiter auflösen.

## 11. Verhalten von i2c_imx_bus_busy()

Nach M0 bis M7 ruft `i2c_imx_start()` auf:

`i2c_imx_bus_busy(i2c_imx, 1, atomic)`

Die untersuchte Linux-6.18.49-Funktion liest zunächst `I2SR`.

Im Multi-Master-Modus prüft sie:

`temp & I2SR_IAL`

Ist `IAL` gesetzt, führt sie aus:

`i2c_imx_clear_irq(i2c_imx, I2SR_IAL);`

und gibt anschließend zurück:

`-EAGAIN`

Im untersuchten Fehlerfall liest dieser Pfad erneut den bereits aus
M0 bis M7 bekannten Zustand mit `I2SR=0x93`.

Damit entsteht die protokollierte Meldung:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

Danach wird `IAL` gelöscht und `-EAGAIN` zurückgegeben.

## 12. Erklärung von B=93/80, C=83/80 und ret=-11

B entspricht M0 und ist im Fehlerfall:

`B=93/80`

Nach M0 bis M7 ruft der Treiber `i2c_imx_bus_busy()` auf.

Dort wird das gesetzte `IAL` erkannt und über
`i2c_imx_clear_irq()` gelöscht.

Danach kehrt `i2c_imx_bus_busy()` mit:

`-EAGAIN`

zurück.

Erst anschließend nimmt `i2c_imx_start()` den Snapshot C auf.

Dieser lautet:

`C=83/80`

Die Differenz zwischen:

`0x93`

und:

`0x83`

ist genau:

`I2SR_IAL = 0x10`

Damit ist der Übergang:

`B=93/80`

`-> C=83/80`

direkt durch den ausgeführten Treiberpfad erklärt.

Der Rückgabewert:

`ret=-11`

entspricht:

`-EAGAIN`

Damit sind `C=83/80` und `ret=-11` keine unabhängigen zusätzlichen
Fehlerphänomene.

Sie sind direkte Folgen des bereits zuvor beobachteten gesetzten
`IAL`-Bits und seiner Behandlung in `i2c_imx_bus_busy()`.

## 13. Erfolgreicher weiterer START-Pfad

Beim erfolgreichen START liefert `i2c_imx_bus_busy()` keinen
IAL-Fehler zurück.

Der bekannte erfolgreiche Snapshot C lautet:

`C=a1/a0`

Damit sind:

- `IBB=1`
- `MSTA=1`
- `IAL=0`

Anschließend konfiguriert `i2c_imx_start()` den Controller für die
Übertragungsphase.

Die erfolgreichen Snapshots D und E lauten:

`D=a1/f8`

`E=a1/f8`

Damit bleibt der Bus als busy beobachtet und der Controller befindet
sich im Master-/Transmit-Zustand.

Der bekannte erfolgreiche Gesamtpfad lautet somit:

`A=81/80`

`-> MSTA schreiben`

`-> M0=81/a0`

`-> später a1/a0`

`-> C=a1/a0`

`-> D=a1/f8`

`-> E=a1/f8`

## 14. Herkunft des Multi-Master-Modus

Der Linux-6.18.49-i2c-imx-Treiber setzt:

`i2c_imx->multi_master = !of_property_read_bool(..., "single-master");`

Der Quellcode kommentiert ausdrücklich, dass Multi-Master-Modus
standardmäßig aktiviert ist und die Property `single-master` aus
Kompatibilitätsgründen verwendet wird.

Die untersuchten Novena-I2C3-DTS-Stände enthalten im hier relevanten
Zustand keine `single-master`-Property.

Damit gilt für den untersuchten Zustand:

`multi_master = true`

Deshalb wertet `i2c_imx_bus_busy()` das gesetzte `I2SR_IAL` aus und
kehrt mit `-EAGAIN` zurück.

## 15. Bedeutung von „arbitration lost“

Die Bezeichnung `arbitration lost` ist im untersuchten Log direkt an
das Hardware-Statusbit:

`I2SR_IAL`

und dessen Behandlung durch den `i2c-imx`-Treiber gebunden.

Direkt bewiesen ist deshalb:

**Der i.MX6-I2C-Controller meldet beim fehlgeschlagenen START sein
IAL-Hardware-Statusbit.**

Nicht daraus bewiesen ist:

**dass ein zweiter physischer I2C-Master tatsächlich gleichzeitig eine
Übertragung durchgeführt hat.**

Der Multi-Master-Modus des Treibers ist ohne `single-master`
standardmäßig aktiv.

Das Vorhandensein des Softwaremodus ist deshalb kein Nachweis eines
zweiten physischen Masters auf I2C3.

Ebenso beweist das IAL-Bit allein nicht den konkreten elektrischen
Mechanismus, durch den der Controller den Arbitration-Lost-Zustand
erreicht.

## 16. Controllerseitig direkt bewiesen

Durch die Kombination aus Patch 0006, gesicherter Evidence und
Linux-6.18.49-Quellcode ist direkt belegt:

1. FAIL und PASS beginnen mit demselben Snapshot `A=81/80`.

2. Die Divergenz entsteht erst nach dem MSTA-Schreibzugriff.

3. Beim erfolgreichen START wird bei M0 bereits `MSTA=1`
   zurückgelesen.

4. `IBB` kann im erfolgreichen Ablauf erst einige Registerlesungen
   später sichtbar werden.

5. Beim fehlgeschlagenen START ist bereits bei der ersten
   Post-MSTA-I2SR-Lesung M0 `IAL=1`.

6. Bei der unmittelbar danach ausgeführten I2CR-Lesung ist
   `MSTA=0`.

7. M0 bis M7 bleiben im Fehlerfall vollständig `93/80`.

8. Innerhalb des M0-bis-M7-Fensters wird kein erfolgreicher
   MSTA-Zwischenzustand beobachtet.

9. `i2c_imx_bus_busy()` liest anschließend den gesetzten
   IAL-Zustand.

10. Der Treiber löscht `IAL` und gibt `-EAGAIN` zurück.

11. Dadurch ist der spätere Snapshot `C=83/80` erklärt.

12. `ret=-11` ist die direkte Rückgabe dieses `-EAGAIN`-Pfads.

## 17. Aussagegrenzen

Nicht bewiesen ist:

- der exakte interne Hardwarezustand zwischen dem MSTA-Schreibzugriff
  und der ersten I2SR-Lesung M0,
- die exakte Dauer dieses Fensters,
- ob `MSTA` intern zunächst kurz angenommen und anschließend wieder
  gelöscht wurde,
- ob `MSTA` niemals als stabiler Masterzustand erreicht wurde,
- ein tatsächlich konkurrierender zweiter physischer I2C-Master,
- der mikroskopische elektrische Mechanismus hinter dem IAL-Ereignis.

Insbesondere dürfen die seriell ausgeführten I2SR-/I2CR-Lesungen nicht
als atomarer gemeinsamer Snapshot interpretiert werden.

Die Aussage:

`M0=93/80`

bedeutet exakt:

1. zuerst wurde `I2SR=0x93` gelesen,
2. danach wurde `I2CR=0x80` gelesen.

## 18. Einordnung gegenüber Block 3.14A

Block 3.14A hat die Beteiligung der ES8328-/Audio-Power-Domain und den
zeitlichen Zusammenhang mit dem späteren I2C3-Fehler rekonstruiert.

Block 3.14B ergänzt diese Analyse auf Controller-Ebene.

Zusammen ergibt sich:

`es8328-power Late-Cleanup`

`-> Power-Domain-Grenzzustand`

`-> späterer IT6251-I2C3-START`

`-> identischer Pre-MSTA-Zustand A=81/80`

`-> MSTA-Schreibzugriff`

`-> FAIL: erste Post-MSTA-I2SR-Lesung bereits mit IAL`

`-> MSTA bei nachfolgender I2CR-Lesung nicht gesetzt`

`-> bus_busy erkennt IAL`

`-> IAL wird gelöscht`

`-> -EAGAIN / -11`

Die Controlleranalyse bestimmt weiterhin nicht den mikroskopischen
elektrischen Mechanismus des Power-Domain-Grenzzustands.

## 19. Konsequenz

Block 3.14B liefert keinen Grund für eine Änderung der bestehenden
Board-Konfiguration.

Es wird insbesondere kein:

- Patch 0007,
- Retry-Workaround,
- zusätzlicher START-Delay,
- Bus-Recovery-Workaround,
- neuer Single-Master-Test,
- neuer STMPE811-Isolationstest

aus diesem Befund abgeleitet.

Die bestehende Entscheidung bleibt:

**`es8328-power` bleibt dauerhaft mit `regulator-always-on`
eingeschaltet.**

Die Diagnose-Patches 0003 bis 0006 bleiben
Entwicklungsinstrumentierung und sind nicht als Produktionszustand
einzustufen.

## 20. Abschluss

Block 3.14B schließt die controllerseitige Rekonstruktion des
beobachteten START-Fehlers ab.

Der Fehler ist auf Controller-Ebene bis auf das sehr kleine Fenster
zwischen dem MSTA-Schreibzugriff und der ersten danach ausgeführten
I2SR-Lesung eingegrenzt.

Innerhalb dieses Fensters ist mit der vorhandenen Instrumentierung kein
weiterer Zwischenzustand beobachtbar.

Direkt danach liegt bereits das vom i.MX6-I2C-Controller gesetzte
IAL-Hardwarebit vor.

Der anschließend beobachtete `-EAGAIN`-/`-11`-Pfad sowie der Übergang
von `I2SR=0x93` zu `I2SR=0x83` sind vollständig durch den
Linux-6.18.49-i2c-imx-Code erklärt.

Ein zweiter physischer Master und der genaue elektrische Mechanismus
sind dadurch nicht bewiesen.

Damit ist Block 3.14B inhaltlich abgeschlossen.
