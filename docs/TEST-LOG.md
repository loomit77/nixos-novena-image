# Test Log

Stand: 2026-09-12

## Frühere NixOS-Tests

Ein älterer NixOS-Ansatz verwendete den Novena-spezifischen Kernel:

`5.7.0-rc2`

Ein früherer Bootversuch erreichte den Kernel, endete jedoch mit:

`VFS: Unable to mount root fs on unknown-block(0,0)`

Damit war nachgewiesen, dass Kernel und Device Tree grundsätzlich
gestartet werden konnten, die Root-/Initrd-Kette aber noch nicht
korrekt war.

## Analyse des bekannten funktionierenden Images

Das bekannte Novena-NixOS-Image wurde untersucht.

Festgestellt wurden:

* Bootloaderbereich vor Partition 1
* FAT-Partition `FIRMWARE`
* ext4-Partition `NIXOS_SD`
* SPL/U-Boot
* Kernel
* Device Tree
* initrd
* U-Boot-Bootskript

Diese Erkenntnisse bilden die Grundlage für das neue universelle Image.

## Linux 6.18.49

Für das aktuelle Image wird Linux 6.18.49 verwendet.

Patchserie:

1. IT6251 DRM Bridge
2. Innolux N133HSE-EA1 Panel
3. I2C-IMX-Diagnose für Arbitration Lost und Timeouts

## 2026-09-07 – aktuelles Diagnose-Image gebaut

Das aktuelle NixOS-SD-Image wurde erfolgreich gebaut.

Exit-Code:

`0`

Nix-Store-Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

Größe:

`2581291008` Bytes

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

Das Image enthält zusätzliche Instrumentierung im IT6251-Treiber und im
i.MX-I2C-Treiber.

## Frühere Displaydiagnose

Bei früheren Tests wurde festgestellt, dass der IT6251 grundsätzlich mit
Linux 6.18.49 angesprochen werden kann.

In einem frühen Diagnosezustand wurden jedoch falsche aktive Bildgrößen
gemessen, beispielsweise:

`hactive: 3504`

`vactive: 671`

Die IT6251-Registerprogrammierung selbst entsprach weitgehend dem
historischen funktionierenden Novena-Treiber.

Im später funktionierenden Device-Tree-Zustand sind für die beiden
LDB-DI-Clock-Selektoren explizite Clock-Zuweisungen vorhanden.

Mit dem späteren Gesamtzustand konnte korrekt gemessen werden:

`hactive: 1920`

`vactive: 1080`

Damit ist belegt, dass der funktionierende spätere Displayzustand die
expliziten Clock-Zuweisungen enthält und eine stabile 1920x1080-Ausgabe
erreicht.

Der genaue einzelne Vor-Git-Test, in dem diese Clock-Zuweisungen
erstmals eingeführt wurden, und ihre isolierte Wirkung gegenüber den
anderen gleichzeitig entwickelten Device-Tree-Korrekturen sind aus den
erhaltenen Aufzeichnungen nicht eindeutig rekonstruierbar.

## I2C-Arbitration-Lost-Diagnose

Der Linux-i.MX-I2C-Treiber aktiviert standardmäßig Multi-Master-Betrieb,
wenn im Device Tree keine Eigenschaft `single-master` gesetzt ist.

Auf dem Display-I2C-Bus wurden wiederholt folgende Diagnosemeldungen
beobachtet:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

Die zugehörigen IT6251-Zugriffe endeten mit:

`-11`

Die Quellcodeanalyse zeigte, dass dieser Rückgabewert durch die
Arbitration-Lost-Behandlung des i.MX-I2C-Treibers erzeugt wurde.

Daraufhin wurde für den Displaybus `i2c3` im Device Tree gesetzt:

`single-master;`

Nach dieser Änderung verschwanden die `-11`-/Arbitration-Lost-Fehler auf
dem Display-I2C-Bus.

Diese Änderung bleibt Bestandteil des aktuell funktionierenden
Teststands.

## Untersuchung von regulator-always-on

Ein vorheriger Teststand enthielt für die IT6251-Versorgung:

`regulator-always-on`

Auch `reg_lvds_lcd` war in einem früheren Overlay-Zustand mit
`regulator-always-on` versehen.

Die Regulator-Diagnose zeigte, dass `lcd-display-power` dadurch auch dann
aktiv blieb, wenn der IT6251-Consumer selbst nicht aktiv war.

Damit bestand die Möglichkeit, dass Linux einen bereits durch U-Boot
initialisierten IT6251-Zustand übernahm, anstatt den Chip selbst aus
einem ausgeschalteten Zustand zu initialisieren.

`regulator-always-on` wurde deshalb für `reg_display` entfernt.

Der Kernel-Basis-DTB enthielt bereits eine Einschaltverzögerung von:

`startup-delay-us = <200000>`

Im späteren Teststand wurde diese auf:

`startup-delay-us = <2000000>`

erhöht.

Ein anschließender Test bestätigte, dass Linux den Regulator tatsächlich
aus- und wieder einschalten konnte und die konfigurierte Verzögerung von
ungefähr zwei Sekunden eingehalten wurde.

## Früherer Test nach echtem Power-Cycle

Bei einem früheren Test nach Entfernen von `regulator-always-on` trat
nach dem Einschalten der IT6251-Versorgung folgendes Verhalten auf:

1. erster I2C-Schreibtransfer: `-110` / `ETIMEDOUT`
2. nachfolgende Zugriffe: `-6` / `ENXIO`
3. IT6251 Product-ID konnte nicht gelesen werden
4. Linux deaktivierte den Regulator nach dem Fehlschlag
5. internes Display blieb aus

Die relevante Sequenz begann mit:

`power_up: regulator_enable`

und ungefähr zwei Sekunden später mit dem ersten Product-ID-Zugriff.

Der erste Zugriff endete mit:

`<i2c_imx_write> write timedout`

Dieser Fehler war der Ausgangspunkt für die erweiterte
I2C-ISR-Instrumentierung des aktuellen Diagnose-Images.

## 2026-09-12 – erfolgreicher Hardwaretest

Am 2026-09-12 wurde das aktuelle Diagnose-Image erneut auf echter
Novena-Hardware kalt gestartet.

Das System erreichte den regulären NixOS-Login.

Der Test zeigte diesmal einen vollständig erfolgreichen Linux-seitigen
Displaystart.

### IT6251 Probe

Der IT6251-Treiber wurde auf:

`i2c-2`

mit Adresse:

`0x5c`

registriert.

Die Probe wurde erfolgreich abgeschlossen:

`IT6251 DRM bridge registered`

`probe: exit success`

### Linux-seitiges Power-Up

Die DRM-Bridge rief anschließend die Power-Up-Sequenz auf:

`bridge_pre_enable: enter`

`power_up: enter powered=0`

`power_up: regulator_enable`

`power_up: regulator enabled`

Danach begann der erste Product-ID-Versuch:

`power_up: product ID attempt 1/5`

Die Kernel-Zeitstempel des erfolgreichen Tests zeigten, dass die
konfigurierte Einschaltverzögerung von ungefähr zwei Sekunden vor dem
ersten IT6251-Zugriff eingehalten wurde.

Anders als beim vorherigen fehlgeschlagenen Test trat diesmal kein
Timeout beim ersten IT6251-Zugriff auf.

### Product-ID erfolgreich

Bereits der erste Product-ID-Versuch war vollständig erfolgreich.

Register `0x00`:

`0x15`

Register `0x01`:

`0xca`

Register `0x02`:

`0x51`

Register `0x03`:

`0x62`

Ergebnis:

`IT6251 detected: vendor ca15 device 6251`

Danach:

`power_up: exit success`

`bridge_pre_enable: exit ret=0 powered=1`

Damit ist nachgewiesen, dass Linux den IT6251 in diesem Test nach dem
Einschalten der Versorgung selbst erfolgreich erreichen konnte.

### Display-Link

Die anschließende IT6251-Initialisierung war erfolgreich.

Das DisplayPort-Linktraining endete nach zehn Iterationen.

Systemstatus:

`0x3e`

Gemessene aktive Auflösung:

`hactive: 1920`

`vactive: 1080`

Erfolgsindikatoren:

`is_stable: stable 1920x1080`

`display link stable`

`bridge_enable: exit success`

Damit wurde auf echter Novena-Hardware ein stabiler interner
1920x1080-Display-Link erreicht.

### Regulatorstatus

Nach erfolgreicher Displayinitialisierung wurden folgende GPIO-Zustände
beobachtet:

`gpio-15 (regulator-lvds-lcd) out hi`

`gpio-28 (regulator-display) out hi`

Die Regulator-Zusammenfassung zeigte sowohl `lcd-lvds-power` als auch
`lcd-display-power` aktiv bei 3300 mV.

Der IT6251 war als Consumer `2-005c-power` eingetragen.

### Erwartete IT6251-Reset-NACKs

Während der Reset-Sequenzen wurden temporär folgende Meldungen
beobachtet:

`error -6 writing to eDP addr 0x5`

`error -6 writing to LVDS addr 0x5`

Die Initialisierung lief anschließend erfolgreich weiter.

Diese Meldungen verhinderten den erfolgreichen Displaystart nicht und
werden für diesen Test als erwartete temporäre Nichtantworten während
der IT6251-Resetsequenz eingeordnet.

### I2C-ISR-Diagnose

Die zusätzliche Instrumentierung zeigte während der erfolgreichen
IT6251-Schreibtransfers wiederholt ISR-Eintritte.

Beispiel:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=0 len=1`

und anschließend:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=1 len=1`

Damit wurde für diesen erfolgreichen Test bestätigt, dass die
instrumentierten IT6251-I2C-Transfers bis in den Interruptpfad des
i.MX-I2C-Treibers verfolgt werden konnten.

Die Diagnoseinstrumentierung selbst ist jedoch sehr umfangreich und
kann das Timing beeinflussen.

Sie ist deshalb noch nicht für einen finalen Produktionskernel
geeignet.

## Separates i2c-0-Diagnoseproblem

Nach dem erfolgreichen Displaystart wurden auf `i2c-0` große Mengen von
Meldungen der Form:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

beobachtet.

Später trat dort zusätzlich auf:

`<i2c_imx_write> write timedout`

Dieses Verhalten ist getrennt vom erfolgreichen IT6251-Betrieb auf
`i2c-2` zu betrachten.

Der Diagnose-Patch muss vor abschließenden Reproduzierbarkeitstests
entfernt, entschärft oder auf den tatsächlich benötigten Bus begrenzt
werden.

## STMPE811 / Touchscreen

Die spätere Untersuchung des laufenden Device Trees zeigte, dass der
STMPE811 auf I2C-Adresse `0x44` weiterhin aktiv ist.

Beim Boot wurde er zunächst erfolgreich erkannt:

`stmpe811 detected, chip id: 0x811`

Auch das Touchscreen-Eingabegerät wurde registriert.

Im weiteren Betrieb wurden wiederholt STMPE-I2C-Fehler beobachtet,
darunter:

`stmpe-i2c 0-0044: failed to read regs 0xb: -110`

sowie entsprechende Fehler mit:

* `-6`
* `-11`

Ein Vergleich des historischen Golden-DTB `e36cd0c8...` mit dem später
aktuell gebooteten DTB `31f2b35e...` zeigte, dass der STMPE811-/
Touchscreen-Teilbaum in beiden Zuständen aktiv und inhaltlich
gleich ist.

Damit ist die frühere Annahme widerlegt, dass der funktionierende
Displayzustand durch eine Deaktivierung des STMPE811 entstanden sei.

Displaypfad und STMPE-/Touchscreen-Problem sind getrennt zu behandeln.

Als nächster kontrollierter Test soll nur der STMPE811-/
Touchscreen-Pfad reproduzierbar deaktiviert werden, während der
funktionierende Displaypfad unverändert bleibt.

## Rekonstruktion der Vor-Git-Displayentwicklung

Die Git-Historie des Projekts beginnt mit:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Commit:

`Add reproducible Novena NixOS image project`

Der historische Golden-DTB besitzt den SHA-256-Wert:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Der später aktuell gebootete DTB besitzt:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

Beide basieren auf demselben Kernel-Basis-DTB:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Die DTB-Unterschiede stammen deshalb aus unterschiedlichen
Overlay-Zuständen.

Der historische `e36cd0c8...`-Zustand enthielt unter anderem:

* `regulator-always-on` auf `reg_display`
* `regulator-always-on` auf `reg_lvds_lcd`
* kein `single-master;`
* keine späteren expliziten Clock-Zuweisungen
* den Basiswert `startup-delay-us = <200000>`

Der später funktionierende `31f2b35e...`-Zustand enthält:

* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`
* `single-master;`
* explizite Clock-Zuweisungen
* `startup-delay-us = <2000000>`

Der Commit `f26464f...` enthält bereits den späteren
Device-Tree-Quellzustand mit `single-master`, 2-Sekunden-Delay und den
Clock-Zuweisungen.

Deshalb darf der in der Golden-Build-Sicherung dokumentierte
Git-Commit nicht als Provenienznachweis dafür verwendet werden, dass
der historische DTB `e36cd0c8...` aus exakt diesem committed
Quellzustand gebaut wurde.

Die vorhergehenden Display-Experimente fanden vor der vorhandenen
Git-Historie statt und müssen aus Nix-Artefakten, Device Trees, Backups
und Testaufzeichnungen rekonstruiert werden.

## Nächste Testschritte

Vor einer endgültigen Einstufung des Displaystands als reproduzierbar
sind vorgesehen:

1. STMPE811-/Touchscreen-Pfad kontrolliert deaktivieren
2. überprüfen, dass der Displaypfad unverändert funktioniert
3. I2C-Diagnoseinstrumentierung entfernen oder gezielt begrenzen
4. mindestens fünf identische echte Kaltstarts durchführen
5. pro Start Boot-ID und monotone Kernel-Zeitstempel sichern
6. Display-, IT6251- und relevante I2C-Meldungen vergleichen

Bis diese Tests abgeschlossen sind, bleibt der aktuelle Stand ein
bestätigter funktionierender Entwicklungsstand und noch kein finaler
Produktionsstand.
