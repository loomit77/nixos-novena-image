# Block 3.14C – i.MX6-START- und Arbitration-Lost-Semantik

## 1. Ziel

Block 3.14C schließt unmittelbar an Block 3.14B an.

Block 3.14B hatte den fehlgeschlagenen I2C3-START beim POR-Cold-Boot
auf das kleinste mit der vorhandenen Softwareinstrumentierung
beobachtbare Fenster eingegrenzt:

`A=81/80 -> M0=93/80`

A liegt unmittelbar vor dem Schreiben von `I2CR_MSTA`.

M0 ist die erste danach ausgeführte I2SR-Lesung.

Zwischen dem MMIO-Schreibzugriff auf I2CR und M0 liegt im instrumentierten
Linux-6.18.49-Code keine weitere Softwareoperation.

Ziel von Block 3.14C ist deshalb nicht eine weitere zeitliche
Instrumentierung, sondern die quellenbasierte Klärung der
Controllersemantik dieses Übergangs.

Insbesondere werden folgende Fragen untersucht:

- wie historische Freescale-/Novena-Software einen START erzeugt,
- wie `I2SR_IAL` dabei behandelt wird,
- wie `I2SR_IBB` und `I2CR_MSTA` einzuordnen sind,
- ob die beobachtete Signatur mit einem dokumentierten
  Arbitration-Lost-/Masterübernahme-Verhalten vereinbar ist,
- ob ein bekanntes i.MX6DQ-Silicon-Erratum die Signatur erklärt.

Es wird kein neuer Kernel gebaut und kein neuer Bootversuch durchgeführt.

Es wird kein Patch 0007 erstellt.

## 2. Ausgangspunkt aus Block 3.14B

Der fehlgeschlagene POR-Cold-Boot zeigt:

`A=81/80`

unmittelbar gefolgt von:

`M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80`

Die relevanten Bits sind:

- `I2SR_ICF = 0x80`
- `I2SR_IBB = 0x20`
- `I2SR_IAL = 0x10`
- `I2SR_IIF = 0x02`
- `I2CR_MSTA = 0x20`

Damit bedeutet `I2SR=0x93` unter anderem:

- `IAL=1`,
- `IIF=1`,
- `IBB=0`.

`I2CR=0x80` bedeutet:

- I2C-Modul aktiviert,
- `MSTA=0`.

Beim erfolgreichen Same-Boot-Rebind beginnt derselbe sichtbare
Pre-START-Zustand mit:

`A=81/80`

Die erste Post-MSTA-Probe ist dort jedoch:

`M0=81/a0`

Damit ist beim erfolgreichen Pfad `MSTA=1`, während beim
fehlgeschlagenen Pfad bei der ersten beobachtbaren Post-MSTA-Probe
bereits `IAL=1` und `MSTA=0` vorliegen.

## 3. Softwareseitige Auflösungsgrenze

Die Untersuchung der Diagnose-Patches 0004 und 0006 bestätigt die
Reihenfolge:

1. Pre-MSTA-Snapshot A,
2. I2CR lesen,
3. `I2CR_MSTA` setzen,
4. I2CR schreiben,
5. unmittelbar danach I2SR lesen: M0,
6. anschließend I2CR lesen: M0-Kontrollwert.

Zwischen dem Schreiben von `MSTA` und der ersten I2SR-Lesung M0 liegt
keine weitere Softwareoperation.

Eine zusätzliche Softwareprobe zwischen MSTA-Schreibzugriff und M0 ist
damit mit diesem Instrumentierungsansatz nicht möglich.

Eine dichtere Variante von Patch 0006 würde die zeitliche
Beweisgrenze nicht weiter verschieben.

Daraus wird weiterhin kein Patch 0007 abgeleitet.

## 4. Historischer Novena-U-Boot-Treiber

Als lokal archivierte historische Quelle wurde untersucht:

`research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/drivers/i2c/mxc_i2c.c`

Der Treiber definiert unter anderem:

`I2CR_MSTA = (1 << 5)`

`I2SR_IBB = (1 << 5)`

`I2SR_IAL = (1 << 4)`

Der historische START-Pfad führt folgende Schritte aus:

1. I2C-Modul aktivieren bzw. stabilisieren,
2. Status behandeln,
3. auf `ST_BUS_IDLE` warten,
4. `I2CR_MSTA` setzen,
5. auf `ST_BUS_BUSY` warten.

Der relevante Ablauf lautet damit sinngemäß:

`Bus Idle -> MSTA setzen -> IBB erwarten`

Die Funktion `wait_for_sr_state()` liest dabei I2SR und prüft vor der
eigentlichen Zustandsbedingung auf `I2SR_IAL`.

Ist `IAL` gesetzt, behandelt der Treiber dies als:

`Arbitration lost`

und bricht den Warteschritt mit einem Fehler ab.

Die historische Novena-/Freescale-Software behandelt ein auftretendes
IAL damit ausdrücklich als höher priorisierten Fehler gegenüber dem
erwarteten Buszustand.

## 5. Bedeutung für die Cold-Fail-Signatur

Der beobachtete Cold-Fail kann damit controllerseitig folgendermaßen
eingeordnet werden:

`Bus vor START sichtbar idle`

`-> Software setzt MSTA`

`-> erste beobachtbare Post-MSTA-Probe enthält bereits IAL`

`-> IBB ist nicht gesetzt`

`-> MSTA ist bei der unmittelbar folgenden I2CR-Lesung nicht mehr gesetzt`

Die Signatur ist mit einem fehlgeschlagenen Übergang in den
Masterzustand und der Arbitration-Lost-Semantik des i.MX-I2C-Controllers
vereinbar.

Damit wird eine in Block 3.14B noch offene Frage enger eingeordnet:
Das Verschwinden von `MSTA` muss nicht durch einen späteren
Linux-Treiberpfad verursacht worden sein.

Insbesondere liegt zwischen dem MSTA-Schreibzugriff und M0 kein
Linux-Code, der `MSTA` wieder löscht.

Der anschließend beobachtete Linux-Pfad:

`93/80 -> 83/80 -> -EAGAIN`

ist weiterhin eine nachgelagerte Reaktion auf das bereits vorhandene
IAL-Hardwarebit.

## 6. Historischer Linux-/Freescale-Beleg zur Arbitration-Lost-Semantik

Für die Einordnung des Übergangs von `MSTA` zu `IAL` ist ein
historischer `i2c-imx`-Commit besonders relevant.

Commit:

`639a26cf0771cb5a4d61a0f7777882cbda989753`

Commit-Betreff:

`i2c: imx: Add arbitration lost check`

Der Commit stammt aus der historischen Freescale-Entwicklung des
i.MX-I2C-Treibers.

Die Commit-Beschreibung begründet die zusätzliche
Arbitration-Lost-Prüfung mit der i.MX-Controllerspezifikation.

Für den hier untersuchten START-Fall ist insbesondere die dort
beschriebene Controllerreaktion relevant:

Versucht der Controller die Masterübernahme bzw. START-Erzeugung in
einem Zustand, in dem diese nicht erfolgreich abgeschlossen werden
kann, kann die Hardware den Masterzustand wieder verlassen,
`I2SR_IAL` setzen und dabei keinen STOP erzeugen.

Damit existiert ein historischer Freescale/Linux-Beleg dafür, dass
ein nach dem Setzen von `MSTA` beobachtetes:

`IAL=1`

zusammen mit einem anschließend nicht mehr gesetzten:

`MSTA=0`

nicht voraussetzt, dass Linux `MSTA` softwareseitig wieder gelöscht
hat.

Dies passt zur Novena-Beobachtung:

`A=81/80`

`-> MSTA-Schreibzugriff`

`-> M0=93/80`

Zwischen MSTA-Schreibzugriff und M0 liegt in der vorhandenen
Instrumentierung kein weiterer Linux-Code.

Der historische Beleg erklärt damit einen in Block 3.14B noch offenen
Punkt: Das schnelle Verschwinden von `MSTA` ist grundsätzlich mit
Controller-Hardwareverhalten bei einer fehlgeschlagenen
Masterübernahme vereinbar.

Der Beleg bestimmt jedoch nicht, welcher elektrische Zustand auf der
Novena den Controller zu dieser Reaktion veranlasst hat.

Insbesondere beweist er weder:

- einen zweiten physischen I2C-Master,
- einen konkret gemessenen Bus-Busy-Zustand,
- einen bestimmten SDA-Pegel,
- einen bestimmten SCL-Pegel.

## 7. Moderner Linux-Upstream-Kontext

Die vorhandene historische Linux-Recherche dokumentiert außerdem die
weitere Entwicklung der `i2c-imx`-Behandlung von Bus Busy und
Arbitration Lost.

Relevante lokale Unterlagen sind unter anderem:

`research/02-historical-software/notes/linux-i2c-imx-5.7-vs-6.18-analyse.md`

sowie die zugehörigen Extracts unter:

`research/02-historical-software/extracts/`

Der aktuelle Linux-6.18-Pfad unterscheidet Single- und Multi-Master-
Behandlung.

Die Device-Tree-Property `single-master` beeinflusst dabei die
softwareseitige Prüfung auf Bus Busy und Arbitration Lost.

Dies darf nicht mit der Entstehung des Hardwarebits `I2SR_IAL`
verwechselt werden.

Die Software kann entscheiden, wie sie auf ein Hardwareereignis
reagiert.

Sie beweist dadurch keinen tatsächlich vorhandenen zweiten physischen
I2C-Master.

Für die Novena-Untersuchung bleibt daher die bereits in Block 3.14B
getroffene Aussage bestehen:

`arbitration lost` ist zunächst die Bedeutung des vom Controller
gesetzten IAL-Bits und kein Nachweis eines zweiten physischen Masters.

## 8. ERR007805

Die vorhandene lokale Linux-Historie enthält ebenfalls die Behandlung
des i.MX-Erratums `ERR007805`.

Der dokumentierte Inhalt betrifft den Betrieb mit 400 kHz:

Die SCL-Low-Zeit kann dabei die I2C-Spezifikation verletzen.

Der Linux-Treiber begrenzt für betroffene Controller deshalb die
Busfrequenz auf maximal 384 kHz.

Dieses Erratum beschreibt nicht:

- den beobachteten MSTA-zu-IAL-Übergang,
- ein automatisches Arbitration-Lost-Ereignis beim START,
- die Cold-Boot-Abhängigkeit von `es8328-power`,
- die Signatur `A=81/80 -> M0=93/80`.

`ERR007805` liefert deshalb keine Erklärung für den untersuchten
Novena-Cold-Fail.

Im untersuchten i.MX6DQ-Errata-Kontext wurde kein passendes bekanntes
Silicon-Erratum identifiziert, das die beobachtete START-Signatur als
Controllerfehlfunktion beschreibt.

## 9. Verbindung zu H3-1R

Block 3.14C ändert die bereits experimentell abgesicherte
Board-Beobachtung nicht.

H3-1R hatte ausschließlich `regulator-always-on` für
`es8328-power` hinzugefügt.

Das Ergebnis waren fünf von fünf erfolgreichen vorregistrierten
POR-Cold-Boots.

Dabei trat weder:

`M0=93/80`

noch das untersuchte I2C3-Arbitration-Lost-Ereignis auf.

Damit ist die ES8328-/Audio-Power-Domain kausal mit dem Auftreten des
Fehlers verbunden.

Block 3.14C erklärt nun besser, wie der Controller auf den daraus
entstehenden kritischen Buszustand reagieren kann.

Block 3.14C bestimmt jedoch nicht den mikroskopischen elektrischen
Mechanismus, der diesen Zustand erzeugt.

## 10. Weiterhin offene elektrische Mechanismen

Die folgenden Kandidaten bleiben voneinander ununterschieden:

### M1 – Clamp / Bus-Loading

Ein nicht versorgter ES8328 könnte über die 330-Ohm-Serienwiderstände
R26A und R27A die I2C3-Leitungen beeinflussen.

### M2 – Backfeeding

Über I2C3 könnte Energie in die abgeschaltete Audio-Domain
zurückgespeist werden.

### M3 – Pull-up-/Power-Domain-Interaktion

Die Audio-seitigen 1-kOhm-Pull-ups R10B und R11B liegen an
`P3.3V_DELAYED` und können mit dem Zustand von `AUD_P3.3V`
wechselwirken.

### M4 – Power-down-Transient

Das Abschalten der Audio-Domain könnte einen transienten Zustand auf
SDA oder SCL erzeugen, der noch beim späteren IT6251-START relevant ist.

Ohne elektrische Messung von SDA, SCL und den beteiligten
Versorgungsschienen können diese Mechanismen nicht sicher
voneinander getrennt werden.

## 11. Was direkt bewiesen ist

Direkt durch die vorhandenen Novena-Messungen und die lokale
Quellenrekonstruktion abgesichert ist:

1. Vor dem fehlgeschlagenen START ist A `81/80`.
2. Zwischen MSTA-Schreibzugriff und M0 liegt keine weitere
   Softwareoperation.
3. Bereits M0 enthält `IAL`.
4. Bei M0 ist `IBB` nicht gesetzt.
5. Bei der unmittelbar folgenden I2CR-Lesung ist `MSTA` nicht gesetzt.
6. Der Linux-Treiber erzeugt `-EAGAIN` erst nach diesem Hardwarezustand.
7. Der historische Novena-U-Boot-Treiber prüft vor START auf Bus Idle,
   setzt anschließend MSTA und erwartet danach Bus Busy.
8. Derselbe historische Treiber behandelt IAL ausdrücklich als
   Arbitration Lost.
9. `ERR007805` betrifft die SCL-Low-Zeit bei hohen Busfrequenzen und
   erklärt die untersuchte Signatur nicht.
10. Das dauerhafte Einschalten von `es8328-power` beseitigte die
    untersuchte Cold-Boot-Signatur in fünf von fünf vorregistrierten
    POR-Versuchen.

## 12. Was nicht bewiesen ist

Nicht bewiesen sind:

- ein tatsächlich konkurrierender zweiter physischer I2C-Master,
- der exakte SDA-Pegel im kritischen MSTA-Fenster,
- der exakte SCL-Pegel im kritischen MSTA-Fenster,
- die Dauer eines möglichen elektrischen Transienten,
- welcher der Mechanismen M1 bis M4 tatsächlich wirkt,
- dass der Bus im kritischen Moment elektrisch in einem bestimmten,
  nicht gemessenen Pegelzustand lag,
- dass ein i.MX6DQ-Silicon-Bug die Ursache ist.

Insbesondere darf aus `IAL` nicht nachträglich ein elektrisch
gemessener „Bus busy“-Zustand konstruiert werden.

Die Leitungszustände wurden im kritischen Zeitfenster nicht gemessen.

## 13. Konsequenz

Die controllerseitige Untersuchung muss nicht durch einen weiteren
Diagnose-Patch verdichtet werden.

Es wird insbesondere kein:

- Patch 0007,
- zusätzlicher START-Delay,
- Retry-Workaround,
- Bus-Recovery-Workaround,
- erneuter Single-Master-Test,
- erneuter STMPE811-Isolationstest

aus Block 3.14C abgeleitet.

Die bestehende Boardentscheidung bleibt unverändert:

**`es8328-power` bleibt dauerhaft mit `regulator-always-on`
eingeschaltet.**

Die Diagnose-Patches 0003 bis 0006 bleiben
Entwicklungsinstrumentierung.

Das separate I2C0-Arbitration-Lost-Thema bleibt von der hier
untersuchten I2C3-Ursachenkette getrennt.

## 14. Abschluss

Block 3.14C erweitert die controllerseitige Rekonstruktion aus
Block 3.14B um die historische und dokumentierte START-/
Arbitration-Lost-Semantik.

Die Signatur:

`A=81/80 -> M0=93/80`

ist mit einem fehlgeschlagenen Übergang des i.MX-I2C-Controllers in den
Masterzustand vereinbar.

Das bereits bei M0 gesetzte `IAL`, das fehlende `IBB` und das nicht
mehr gesetzte `MSTA` entstehen vor der späteren Linux-Fehlerbehandlung.

Damit wird der Linux-`-EAGAIN`-Pfad weiter als Folge und nicht als
Ursprung des Fehlers eingeordnet.

Ein passendes bekanntes i.MX6DQ-Silicon-Erratum wurde nicht
identifiziert.

Die verbleibende ungeklärte Ebene ist damit nicht eine weitere
softwareseitige zeitliche Unterteilung zwischen MSTA und M0, sondern
der konkrete elektrische Mechanismus an der I2C3-/Audio-Power-Domain-
Grenze.

Dieser Mechanismus bleibt ohne elektrische Messung offen.

Block 3.14C ist damit inhaltlich abgeschlossen.
