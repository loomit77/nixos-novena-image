# Linux i2c-imx: Historische Novena-5.7-Analyse im Vergleich zu Linux 6.18.49

## Zweck und Umfang

Diese Notiz dokumentiert die historische Analyse des i.MX-I²C-Controllerpfads
des Novena-spezifischen NixOS-Kernels Linux 5.7.0-rc2 und vergleicht ihn mit
dem aktuellen Linux-6.18.49-Diagnosestand dieses Projekts.

Die Untersuchung dient ausschließlich der Ursachenanalyse.

Hier wird kein Workaround ausgewählt und kein Patch 0007 definiert.

Die Novena blieb während dieser Analyse ausgeschaltet.

## Herkunft des historischen NixOS-Kernels

Das historische Repository `novena-next/nixos-novena` verwendet:

- Repository: `novena-next/linux`
- Branch: `nvn_v5.7-rc2`
- exakt aufgelöster Commit:
  `1fda06deecb61538ca3d07d256eb7c43d4e3432a`
- Commit-Betreff:
  `FIXME: ITE workaround`

Der Kernel identifiziert sich als Linux 5.7.0-rc2.

Der zugehörige Upstream-Stand Linux 5.7-rc2 ist:

`ae83d0b416db002fe95601e7f97f64b59514d936`

## Herkunft von i2c-imx

Am Novena-Commit `1fda06d` besitzt die vollständige Datei

`drivers/i2c/busses/i2c-imx.c`

den SHA-256:

`0f5f281102fc70cbf54e1dee07634d4b731a153771e0fdd2e51e0042fafcd467`

Die Datei ist bytegenau identisch mit Upstream Linux 5.7-rc2.

Damit enthält die von diesem historischen Novena-Kernel verwendete
i.MX-I²C-Controllerimplementierung gegenüber Upstream Linux 5.7-rc2 keine
Novena-spezifische Änderung.

Auch die historische Datei

`drivers/i2c/i2c-core-base.c`

ist zwischen dem Novena-Kernel und Upstream Linux 5.7-rc2 bytegenau identisch.

Ihr SHA-256 lautet:

`17c41c9b4a734c4d3ede9674305603f8c0a20abf6a648b153c3014de1c3a3145`

Es wurde keine Novena-spezifische Änderung am Retry-Verhalten des I²C-Cores
gefunden.

## i.MX-START-Pfad unter Linux 5.7

Die relevante START-Sequenz unter Linux 5.7 lautet:

1. IFDR programmieren
2. I2SR löschen
3. I²C-Controller aktivieren
4. 50 bis 150 µs warten
5. I2CR lesen
6. `I2CR_MSTA` setzen
7. I2CR schreiben
8. `i2c_imx_bus_busy(..., 1, ...)` aufrufen

Unter Linux 5.7 prüft `i2c_imx_bus_busy()` das Bit `I2SR_IAL`
bedingungslos.

Ist IAL gesetzt:

1. wird IAL gelöscht
2. die Funktion liefert `-EAGAIN` zurück

Dieser Ablauf liegt konzeptionell sehr nahe an genau der START-Stelle, an der
der aktuelle Linux-6.18.49-Diagnosepatch 0006 den Cold-Boot-Fehler beobachtet.

## Retry-Verhalten des I²C-Cores

Der I²C-Core von Linux 5.7 wiederholt einen Adaptertransfer, der `-EAGAIN`
liefert, nur entsprechend dem Wert von `adapter.retries`.

Der historische `i2c-imx`-Treiber weist diesem Feld keinen von null
verschiedenen Retry-Wert zu.

Die Struktur, die den Adapter enthält, wird zuvor mit null initialisiert.

Damit stellt die generische Retry-Schleife des I²C-Cores für diesen
historischen i.MX-Adapter keinen zusätzlichen automatischen Retry bereit.

Ein `-EAGAIN` während START wird somit nicht stillschweigend durch mehrere
generische Adapterversuche verdeckt.

## GPIO-Bus-Recovery unter Linux 5.7

Der `i2c-imx`-Treiber von Linux 5.7 enthält Unterstützung für eine
GPIO-basierte Bus-Recovery.

Wenn vollständige Recovery-Informationen vorhanden sind, kann nach einem
fehlgeschlagenen `i2c_imx_start()` folgender Ablauf stattfinden:

1. `i2c_recover_bus()`
2. ein zweiter Aufruf von `i2c_imx_start()`

Die Implementierung benötigt einen alternativen Pinctrl-Zustand mit dem Namen
`gpio` sowie nutzbare GPIO-Deskriptoren für SDA und SCL.

Der historische Novena-Device-Tree für I2C3 enthält:

`pinctrl-names = "default";`

und:

`pinctrl-0 = <&pinctrl_i2c3_novena>;`

Es wurde weder ein I2C3-Pinctrl-Zustand `gpio` noch eine
I2C3-Recovery-Beschreibung über `sda-gpios` und `scl-gpios` gefunden.

Damit ist die im Linux-5.7-Treiber vorhandene generische
GPIO-Bus-Recovery durch diesen historischen Novena-Device-Tree für I2C3
nicht konfiguriert.

Dies unterscheidet sich wesentlich vom historischen Novena-U-Boot.
Dessen `force_idle_bus()` schaltet SDA und SCL ausdrücklich auf GPIO um,
prüft den Leitungszustand, taktet SCL bei Bedarf, wartet darauf, dass beide
Leitungen High erreichen, und stellt anschließend das I²C-Pin-Mux wieder her.

Das beweist nicht, dass die U-Boot-Bus-Recovery den aktuellen Linux-Fehler
behebt.

Es belegt jedoch, dass der historische Bootloader einen physischen
Vorbereitungsschritt für den I²C-Bus ausführt, für den der untersuchte
Linux-5.7-Novena-Device-Tree keine entsprechende aktive
Recovery-Konfiguration bereitstellt.

## Novena 5.7 gegenüber Upstream Linux 5.7-rc2

Die Novena-Versionen von `i2c-imx.c` und dem I²C-Core sind bytegenau identisch
mit Upstream Linux 5.7-rc2.

Der Novena-Device-Tree ist dagegen nicht bytegenau mit Upstream identisch.

Die hier relevante Novena-spezifische Erweiterung ist die Einbindung der
IT6251-LVDS-zu-eDP-Bridge.

Der I2C3-Controller behält die normale Default-Pinctrl-Konfiguration und
erhält keine GPIO-Recovery-Konfiguration.

Das hier untersuchte historische Verhalten des I²C-Controllers ist somit
Upstream-Linux-Verhalten und kein Novena-spezifischer `i2c-imx`-Fix.

## Historisches IT6251-Verhalten

Der historische Novena-spezifische IT6251-Treiber enthält bereits seit seinem
ursprünglichen Treiber-Commit ein defensives Retry-Verhalten beim Lesen der
Product-ID.

Nach dem Aktivieren des Regulators wird versucht, die Product-ID bis zu
fünfmal zu lesen.

Nach einem fehlgeschlagenen Versuch folgt:

`usleep_range(100000, 200000)`

Der Kommentar im Treiber lautet:

`Sometimes it seems like multiple tries are needed`

Diese Retry-Logik existierte bereits vor dem späteren Commit
`FIXME: ITE workaround`.

Der spätere Workaround fügt weder einen weiteren Retry noch eine
START-Recovery hinzu.

Stattdessen erlaubt er `it6251_power_up()`, nach dem Fehlschlagen aller
Product-ID-Versuche Erfolg zurückzuliefern, anstatt die Bridge abzuschalten
und `-EINVAL` zurückzugeben.

Der historische Treiber tolerierte somit instabilen Zugriff auf die Bridge.

Die vorhandenen Quellen beweisen jedoch nicht, dass diese historische
Instabilität durch denselben i.MX-IAL-Zustand verursacht wurde, den wir unter
Linux 6.18.49 beobachten.

## Regulator-Timing

Der historische Novena-Display-Regulator besitzt:

`startup-delay-us = <200000>;`

Diese Eigenschaft stammt bereits aus dem ursprünglichen
Upstream-Novena-Device-Tree.

Im IT6251-Treiber gibt es zwischen der erfolgreichen Rückkehr aus
`regulator_enable()` und dem ersten Product-ID-Lesezugriff keinen eigenen
expliziten Sleep.

Wenn der Fixed-Regulator tatsächlich vom deaktivierten in den aktivierten
Zustand wechselt, kann dessen Startup-Delay jedoch innerhalb des
Regulator-Frameworks berücksichtigt werden.

Diese Unterscheidung ist wichtig.

Im aktuellen Cold-Boot-Trace mit Linux 6.18.49 beträgt der beobachtete Abstand
zwischen der protokollierten Aktivierung des IT6251-Regulators und dem ersten
Product-ID-Zugriff nur ungefähr 3 ms.

Damit wartet der aktuell getestete Pfad an dieser beobachteten Stelle
nachweislich keine 200 ms.

## Direkter Vergleich Linux 5.7 und Linux 6.18.49

Es wurde ein direkter Quellvergleich zwischen dem historischen
Linux-5.7-rc2-`i2c-imx.c` und dem rekonstruierten Linux-6.18.49-Quellstand
durchgeführt, der für Patch 0006 verwendet wird.

Der vollständige Quelldiff wird separat archiviert.

Eine auffällige semantische Änderung ist die inzwischen vorhandene
Unterscheidung zwischen Single-Master- und Multi-Master-Betrieb.

Linux 5.7 prüft IAL in `i2c_imx_bus_busy()` bedingungslos.

Linux 6.18.49 enthält:

`if (multi_master && (temp & I2SR_IAL))`

und initialisiert:

`i2c_imx->multi_master = !of_property_read_bool(..., "single-master");`

Damit aktiviert das Fehlen der Device-Tree-Eigenschaft `single-master` das
Multi-Master-Verhalten.

Patch 0005 entfernte experimentell `single-master` aus dem aktuellen
Novena-Device-Tree.

Der Cold-Boot-IAL-Fehler blieb bestehen.

Damit reicht die unterschiedliche Softwarebehandlung zwischen
Single-Master- und Multi-Master-Modus allein nicht aus, um den beobachteten
Fehler zu beseitigen.

## Zwischenstand des Novena-6.6-Zweigs

Der vorhandene Remote-Branch:

`origin/nvn_v6.6`

zeigt auf:

`62230169377ba41353d41806ae35cde734cb5cf0`

Der Commit-Betreff lautet:

`fixup! drm/bridge: Add ITE IT6251 bridge driver`

Dieser Stand identifiziert sich als Linux 6.6.6.

Die dortige Datei `i2c-imx.c` enthält weder `multi_master` noch die in
Linux 6.18.49 vorhandene Behandlung von `single-master`.

Die Single-/Multi-Master-Änderung wurde damit erst nach diesem
Novena-6.6-Stand eingeführt.

## Herkunft des Single-/Multi-Master-Verhaltens

Die genaue Upstream-Herkunft wurde anhand des offiziellen Linux-v6.18-Stands
bestimmt.

Das offizielle annotierte Tag `v6.18` verweist auf:

`7d0a66e4bb9081d75c82ec4957c50034cb0ea449`

mit dem Commit-Betreff:

`Linux 6.18`

Dieser Stand enthält sowohl `multi_master` als auch die
`single-master`-Behandlung.

Die relevante Entwicklung besteht aus zwei Upstream-Commits.

### Einführung des Single-Master-Sonderpfads

Commit:

`6692694aca86ddf6831e2be86e5089258c2789bf`

Autor-Datum:

`2024-10-14`

Commit-Betreff:

`i2c: imx: do not poll for bus busy in single master mode`

Der Commit begründet die Änderung mit dem i.MX8M-Mini-Referenzhandbuch.

Demnach sei das Polling auf Bus Busy und Arbitration Lost beim Erzeugen einer
START-Bedingung nur im Multi-Master-Betrieb erforderlich.

Als praktischer Vorteil wird genannt, unnötiges Rescheduling bei belegtem
I²C-Bus zu vermeiden und dadurch Timeouts von SMBus-Geräten zu verhindern.

Aus Gründen der Rückwärtskompatibilität wurde das neue Verhalten nicht zum
Standard für bestehende Device Trees gemacht.

Stattdessen muss `single-master` ausdrücklich gesetzt werden, um das bisherige
Bus-Busy-Polling beim START zu deaktivieren.

Der Commit ist Bestandteil von Upstream Linux 6.18.

### Korrektur des neuen Single-Master-Sonderpfads

Commit:

`768776dd4efc681cdca33a79e29bb508d6de9bc0`

Autor-Datum:

`2024-12-16`

Commit-Betreff:

`i2c: imx: fix missing stop condition in single-master mode`

Dieser Commit korrigiert eine durch `6692694` eingeführte Regression.

Im Single-Master-Modus wurden STOP-Bedingungen nicht mehr korrekt erzeugt.
Davon waren insbesondere Geräte betroffen, die eine gültige STOP-Bedingung
benötigen, beispielsweise EEPROMs.

Die Korrektur aktiviert das Polling des I2C-Bus-Busy-Bits IBB für die
STOP-Erzeugung im Single-Master-Modus wieder.

Zusätzlich wird sichergestellt, dass `i2c_imx->stopped` zu Beginn jedes
Transfers gelöscht wird, damit `i2c_imx_stop()` die STOP-Bedingung korrekt
erzeugen kann.

Der Commit bestätigt gleichzeitig ausdrücklich die ursprüngliche
Unterscheidung:

Beim START wird IBB im Single-Master-Modus weiterhin nicht gepollt.

Auch dieser Commit ist Bestandteil von Upstream Linux 6.18.

## Bedeutung der Single-/Multi-Master-Commits für die Novena

Die beiden Upstream-Commits wurden nicht als Lösung für einen
Cold-POR-Arbitration-Loss des i.MX-Controllers eingeführt.

Der erste Commit optimiert die Behandlung des START-Zustands im
Single-Master-Betrieb auf Grundlage der dokumentierten i.MX8M-Mini-Semantik.

Der zweite Commit korrigiert eine dadurch entstandene STOP-Regression.

Damit existiert derzeit kein Beleg dafür, dass `single-master` speziell den
auf der Novena beobachteten Cold-Boot-Fehler beheben sollte.

Patch 0005 war dennoch ein sinnvoller Kontrollversuch, weil damit der
Single-/Multi-Master-Pfad experimentell verändert wurde.

Dass der Cold-Boot-Fehler trotz Entfernung von `single-master` bestehen blieb,
zeigt jedoch, dass diese Softwareentscheidung allein die beobachtete
Hardwareursache nicht beseitigt.

Außerdem beziehen sich die Upstream-Commit-Begründungen ausdrücklich auf
i.MX8M Mini, während die Novena einen i.MX6Q verwendet.

Eine Übertragung der beschriebenen Hardwaresemantik auf den i.MX6Q darf daher
nicht ohne weitere Prüfung vorausgesetzt werden.

## Zusammenhang mit Patch 0006

Patch 0006 beobachtet den aktuellen Cold-POR-Fehler unmittelbar über den
Master-Übergang hinweg.

Cold-Boot-Fehler:

`A=81/80 -> M0=93/80`

Bei A gilt:

- I2SR = `0x81`
- I2CR = `0x80`

Beim ersten unmittelbaren Post-MSTA-Sample M0 gilt:

- I2SR = `0x93`
- I2CR = `0x80`

Damit gilt bereits bei M0:

- IAL ist gesetzt
- MSTA ist wieder gelöscht

Bei einem erfolgreichen LDB-Rebind innerhalb desselben Boots ergibt sich:

`A=81/80 -> M0=81/a0`

MSTA bleibt dabei gesetzt und es tritt unmittelbar kein IAL auf.

Die spätere Linux-Single-/Multi-Master-Logik verändert, wie die Software
diesen Zustand behandelt.

Sie erklärt für sich allein jedoch nicht, warum der i.MX6Q-Controller während
des Cold-POR-Übergangs IAL setzt und MSTA unmittelbar wieder verliert.

Diese Trennung zwischen Softwarebehandlung und zugrunde liegendem
Hardwarezustand bleibt für die weitere Ursachenanalyse wesentlich.

## Aktuelle Interpretation

Die historischen Belege rechtfertigen derzeit noch keinen Workaround.

Folgende Punkte sind inzwischen belastbar:

- Der historische Novena-Linux-5.7-Kernel besitzt keinen
  Novena-spezifischen `i2c-imx`-Fix.
- Sein `i2c-imx.c` entspricht bytegenau Upstream Linux 5.7-rc2.
- Der historische Linux-5.7-Novena-Device-Tree aktiviert keine
  GPIO-Bus-Recovery für I2C3.
- Historisches Novena-U-Boot führt dagegen eine aktive physische
  Bus-Idle-/Recovery-Sequenz aus.
- Der historische IT6251-Treiber toleriert fehlgeschlagene Product-ID-Zugriffe
  durch mehrere Versuche.
- Die moderne `single-master`-Logik wurde erst 2024 eingeführt.
- Ihre dokumentierte Motivation betrifft START-Polling im Single-Master-Modus
  und nicht einen bekannten Novena-Cold-POR-IAL-Fehler.
- Eine Regression dieser Änderung betraf die STOP-Erzeugung und wurde separat
  korrigiert.
- Patch 0005 hat gezeigt, dass die Auswahl dieses Softwarepfads den aktuellen
  Cold-Boot-Fehler nicht beseitigt.
- Patch 0006 zeigt, dass die entscheidende Abweichung bereits unmittelbar
  nach dem Setzen von MSTA auftritt.

Damit bleiben insbesondere folgende Bereiche für die weitere Untersuchung
relevant:

- physischer SDA-/SCL-Zustand vor dem ersten Linux-Master-Übergang
- FPGA-Zustand am gemeinsam genutzten I2C3-Bus
- ES8328-Zustand am gemeinsam genutzten I2C3-Bus
- Controller-Clock- und Runtime-PM-Zustand
- Pin-/Pad-Zustand
- Display-Power- und Reset-Sequenz
- Auswirkungen der historischen U-Boot-Busvorbereitung
- Änderungen des eigentlichen `i2c_imx_start()`-Pfads zwischen Linux 5.7 und
  Linux 6.18 unabhängig vom Single-/Multi-Master-Sonderpfad

Für keinen dieser Punkte ist die eigentliche Ursache bisher bewiesen.

## Erweiterte Analyse des i2c-imx-Pfads von Linux 5.7 bis 6.18

Die anschließende Untersuchung hat den zuvor vorgesehenen Vergleich des
eigentlichen START-, Arbitration-Loss-, Runtime-PM-, Clock- und
Pinctrl-Verhaltens durchgeführt.

Die Ergebnisse schränken den möglichen Ursachenbereich des aktuellen
Cold-POR-Fehlers weiter ein.

### Upstream-Historie von i2c-imx zwischen Linux 5.7-rc2 und Linux 6.18

Zwischen Upstream Linux 5.7-rc2

`ae83d0b416db002fe95601e7f97f64b59514d936`

und Upstream Linux 6.18 wurden 52 Commits gefunden, die

`drivers/i2c/busses/i2c-imx.c`

verändern.

Die vollständige Commit-Landkarte und die anschließende Klassifizierung
wurden als Rohartefakte im Forschungsverzeichnis archiviert.

Besonders relevant sind drei zusammengehörige Arbitration-Loss-Änderungen
aus dem Jahr 2020:

- `384a9565f70a876c2e78e58c5ca0bbf0547e4f6d`
  `i2c: imx: Fix reset of I2SR_IAL flag`
- `1de67a3dee7a279ebe4d892b359fe3696938ec15`
  `i2c: imx: Check for I2SR_IAL after every byte`
- `61e6fe59ede155881a622f5901551b1cc8748f6a`
  `i2c: imx: Don't generate STOP condition if arbitration has been lost`

Die Reihenfolge dieser Serie wurde als

`384a956 -> 1de67a3 -> 61e6fe5`

verifiziert.

### Bedeutung der historischen Arbitration-Loss-Serie

Die Commit-Beschreibung von `1de67a3` dokumentiert einen für Patch 0006
wichtigen Hardwareeffekt: Nach einem Arbitration Loss kann die
i.MX-I²C-Hardware selbständig vom Master-Modus in den Slave-Modus wechseln.

`61e6fe5` berücksichtigt anschließend, dass nach einem Arbitration Loss das
Löschen von MSTA keinen normalen STOP mehr erzeugt, weil die Hardware den
Master-Modus bereits verlassen haben kann.

Diese historischen Änderungen beweisen nicht die Ursache des Novena-Fehlers.
Sie liefern jedoch eine dokumentierte Controllersemantik, die mit der in
Patch 0006 beobachteten Zustandsfolge vereinbar ist:

`A=81/80 -> M0=93/80`

Beim ersten Post-MSTA-Sample ist IAL bereits gesetzt und MSTA bereits wieder
gelöscht.

Damit muss das fehlende MSTA-Bit nicht durch späteren Linux-Code erklärt
werden. Es ist mit einem unmittelbar beim Master-Übergang erkannten
Arbitration Loss und dem darauf folgenden autonomen Verlassen des
Master-Modus vereinbar.

Die Serie wurde laut Commit-Beschreibungen unter anderem auf Vybrid VF500
getestet. Sie beweist daher nicht, warum der i.MX6Q der Novena beim Cold POR
diesen Zustand erzeugt.

### System-Suspend- und Pinctrl-Änderungen von 2024

Zwei weitere zunächst auffällige Commits wurden vollständig untersucht:

- `358025ac091e5a54f9819b33ee9c7cb07c55ee5d`
  `i2c: imx: make controller available until system suspend_noirq() and from resume_noirq()`
- `576eba03c99435380d155e5f71d5d7603b9178f6`
  `i2c: imx: switch different pinctrl state in different system power status`

`358025ac` ergänzt insbesondere die System-Suspend-/Resume-Behandlung und
`IRQF_NO_SUSPEND`. Ziel ist es, den I²C-Controller während der kritischen
Phasen `suspend_noirq()` und `resume_noirq()` verfügbar zu halten.

Der Commit behandelt damit einen System-Suspend-/Resume-Fall und nicht den
normalen Cold-Boot-Pfad der Novena.

`576eba03` ergänzt die Auswahl von Pinctrl-Power-States:

- beim Suspend wird `pinctrl_pm_select_sleep_state(dev)` verwendet
- beim Resume wird `pinctrl_pm_select_default_state(dev)` verwendet

Diese Änderungen erklären für sich allein ebenfalls nicht den beobachteten
Cold-POR-Arbitration-Loss.

Für I2C3 der Novena ist im untersuchten Device Tree außerdem kein expliziter
I²C3-Sleep-Pinctrl-State dokumentiert. Deshalb darf aus dem bloßen Aufruf der
Pinctrl-PM-Funktion nicht auf eine konkrete elektrische Änderung der
SDA-/SCL-Pads geschlossen werden.

### Normaler Runtime-PM-Pfad unter Linux 5.7 und Linux 6.18

Der normale Runtime-PM-Pfad wurde anschließend direkt zwischen Linux
5.7-rc2 und Linux 6.18 verglichen.

Linux 5.7-rc2 führt beim Runtime-Suspend aus:

`clk_disable(i2c_imx->clk)`

Beim Runtime-Resume wird ausgeführt:

`clk_enable(i2c_imx->clk)`

Es findet dabei kein Reset von I2CR, I2SR, IFDR oder IADR statt.

Linux 6.18 ergänzt gegenüber diesem einfachen Clock-Pfad die Pinctrl-Auswahl.
Der relevante Ablauf ist:

Runtime-Suspend:

1. I²C-Clock deaktivieren
2. Pinctrl-Sleep-State auswählen

Runtime-Resume:

1. Pinctrl-Default-State auswählen
2. I²C-Clock aktivieren

Auch Linux 6.18 setzt dabei die eigentlichen I²C-Controllerregister nicht
neu auf einen definierten Grundzustand zurück.

Die zunächst durch eine breite Kontextsuche gefundenen Registerschreibzugriffe
gehören zu anderen Funktionen, insbesondere Remove- und Slave-Pfaden, und
nicht zu den normalen Runtime-PM-Callbacks.

### Autosuspend-Zeit

Der Autosuspend-Timeout beträgt sowohl unter Linux 5.7-rc2 als auch unter
Linux 6.18:

`I2C_PM_TIMEOUT = 10 ms`

Beim aktuellen Novena-Boot wird der Hardware-I2C3-Controller bereits ungefähr
bei 5,269 Sekunden registriert.

Der erste beobachtete IT6251-Product-ID-Zugriff erfolgt dagegen erst ungefähr
bei 44,319 Sekunden.

Zwischen Registrierung und erstem relevanten Display-I²C-Zugriff liegt damit
ein Zeitraum von ungefähr 39 Sekunden und somit ein sehr großer Abstand zum
10-ms-Autosuspend-Timeout.

Das macht einen zwischenzeitlichen Runtime-Autosuspend des Controllers
plausibel. Die bisher archivierten Bootlogs enthalten jedoch keine direkte
Runtime-PM-Trace-Meldung, die den exakten Suspend-/Resume-Zeitpunkt dieses
konkreten Boots beweist.

### Probe-Initialisierung unter Linux 5.7

Während des Probe-Pfads von Linux 5.7 wird der Controller zunächst aktiv
gehalten und der Clock-Divider berechnet.

Danach setzt der Treiber die Controllerregister auf einen definierten
Ausgangszustand:

`I2CR` wird mit dem hardwareabhängigen deaktivierten Grundwert beschrieben.

`I2SR` wird mit dem hardwareabhängigen Clear-Opcode beschrieben.

Erst nach der Adapterregistrierung wird der Controller wieder für
Autosuspend freigegeben.

Damit existiert unter Linux 5.7 ein klarer Ablauf aus initialer
Registerinitialisierung, späterem Autosuspend und erneutem Clock-Enable beim
nächsten Transfer.

Für den Linux-6.18-Probe-Pfad wurde in der bisherigen kompakten Extraktion
die entsprechende Registerinitialisierung nicht vollständig ausgegeben.
Aus dieser fehlenden Ausgabe wird ausdrücklich nicht geschlossen, dass die
Initialisierung unter Linux 6.18 fehlt.

### Direkter Vergleich des START-Pfads

Der für Patch 0006 entscheidende Befund ist, dass der untersuchte
`i2c_imx_start()`-Ablauf unter Linux 5.7-rc2 und Linux 6.18 in der relevanten
Sequenz übereinstimmt.

Beide Versionen führen aus:

1. IFDR programmieren
2. I2SR löschen
3. I2CR mit aktiviertem I²C-Controller beschreiben
4. 50 bis 150 µs auf Stabilisierung warten
5. I2CR lesen
6. MSTA setzen
7. I2CR schreiben
8. anschließend den Bus-Busy-/START-Zustand auswerten

Damit ist die grundlegende START-Sequenz, in der Patch 0006 den Cold-POR-
Fehler beobachtet, keine neue Linux-6.18-Sequenz.

### Beziehung der neuen Ergebnisse zu Patch 0006

Patch 0006 zeigt beim Cold POR unmittelbar vor MSTA:

`A=81/80`

Nach dem Schreiben von MSTA zeigt bereits das erste Sample:

`M0=93/80`

Beim erfolgreichen LDB-Rebind innerhalb desselben Boots lautet derselbe
Übergang dagegen:

`A=81/80 -> M0=81/a0`

Die sichtbare Registerlage unmittelbar vor dem MSTA-Schreibzugriff ist damit
in beiden Fällen gleich, während die Hardware unmittelbar danach
unterschiedlich reagiert.

Die historische Arbitration-Loss-Serie zeigt, dass das selbständige Löschen
von MSTA mit einem vom Controller erkannten Arbitration Loss vereinbar ist.

Die Analyse des START-Pfads zeigt gleichzeitig, dass der grundlegende Ablauf
zwischen Linux 5.7 und Linux 6.18 nicht neu eingeführt wurde.

Die Runtime-PM-Analyse zeigt außerdem keinen zusätzlichen Controllerregister-
Reset, der den Unterschied unmittelbar erklären würde.

### Aktualisierte Interpretation

Die bisherige Evidenz spricht zunehmend dagegen, den Fehler allein durch eine
neue Linux-6.18-START- oder Single-/Multi-Master-Softwareentscheidung zu
erklären.

Weiterhin offen sind Zustandsunterschiede, die durch das Snapshot A nicht
vollständig erfasst werden.

Dazu gehören insbesondere:

- physischer SDA-/SCL-Pegel und tatsächlicher Buszustand
- interner Zustand des i.MX6Q-I²C3-Controllers
- Clock-/Clock-Gating-Zustand
- Pinmux-/Pad-Zustand
- FPGA-Zustand am gemeinsam genutzten I2C3-Bus
- ES8328-Zustand am gemeinsam genutzten I2C3-Bus
- Zustand der LCD-/IT6251-Seite einschließlich Power und Reset
- Auswirkungen einer früheren Busbenutzung oder Busvorbereitung

Insbesondere beweist `A=81/80` nur die zu diesem Zeitpunkt sichtbaren Werte
von I2SR und I2CR. Daraus folgt nicht, dass sämtliche internen Controller-
oder physischen Buszustände zwischen Cold POR und erfolgreichem LDB-Rebind
identisch sind.

### Nächster Untersuchungsschritt

Vor einem Patch 0007 soll aus diesen Ergebnissen eine konkrete und
falsifizierbare Hypothese für die Zustandsdifferenz zwischen Cold POR und
erfolgreichem Same-Boot-LDB-Rebind abgeleitet werden.

Dabei soll insbesondere geprüft werden, welche Information unmittelbar vor
dem MSTA-Übergang noch fehlt und mit möglichst geringer zeitlicher
Beeinflussung beobachtet werden kann.

Eine weitere Verdichtung der bereits vorhandenen Post-MSTA-Samples ist nicht
prioritär, weil Patch 0006 die Divergenz bereits im ersten Sample M0 erfasst.

Ein neuer diagnostischer Kernelpatch soll erst definiert werden, wenn klar
ist, welcher konkrete Zustand damit unterschieden werden soll.

Bis dahin bleibt Patch 0006 der dokumentierte experimentelle Ausgangspunkt.
