# Test Log

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

Vor der Korrektur der LDB-Clock-Zuweisung wurden jedoch falsche aktive
Bildgrößen gemessen, beispielsweise:

`hactive: 3504`

`vactive: 671`

Die IT6251-Registerprogrammierung selbst entsprach weitgehend dem
historischen funktionierenden Novena-Treiber.

Die Analyse des bekannten funktionierenden Device Trees zeigte eine
wichtige Abweichung bei der Clock-Konfiguration.

Für die beiden LDB-DI-Clock-Selektoren wurde deshalb PLL2 PFD2 396 MHz
als Parent eingetragen.

Danach konnte erstmals korrekt gemessen werden:

`hactive: 1920`

`vactive: 1080`

Damit wurde die LDB-Clock-Zuweisung als wesentliche Displaykorrektur
bestätigt.

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

Die Regulator-Diagnose zeigte, dass `lcd-display-power` dadurch auch dann
aktiv blieb, wenn der IT6251-Consumer selbst nicht aktiv war.

Damit bestand die Möglichkeit, dass Linux einen bereits durch U-Boot
initialisierten IT6251-Zustand übernahm, anstatt den Chip selbst aus
einem ausgeschalteten Zustand zu initialisieren.

`regulator-always-on` wurde deshalb für `reg_display` entfernt.

Die konfigurierte Einschaltverzögerung blieb:

`startup-delay-us = <2000000>`

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

Danach begann unmittelbar der erste Product-ID-Versuch:

`power_up: product ID attempt 1/5`

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

### I2C-ISR-Diagnose

Die zusätzliche Instrumentierung zeigte während der erfolgreichen
IT6251-Schreibtransfers wiederholt ISR-Eintritte.

Beispiel:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=0 len=1`

und anschließend:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=1 len=1`

Damit wurde für diesen erfolgreichen Test bestätigt, dass die
