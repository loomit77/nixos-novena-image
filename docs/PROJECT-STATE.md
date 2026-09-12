# Project State

Stand: 2026-09-12

## Ziel

Erstellung eines universellen und reproduzierbaren NixOS-SD-Images für
das Kosagi Novena.

Das Image soll die für das Novena notwendigen Boot-, Kernel- und
Hardwarekomponenten enthalten und auf anderen Novena-Geräten verwendbar
sein.

## Entwicklungsrechner

Primäre Build-Maschine:

* Host: foobox
* Betriebssystem: CachyOS
* Architektur: x86_64

## Zielhardware

Kosagi Novena:

* SoC: NXP/Freescale i.MX6 Quad
* Architektur: ARMv7 / armhf
* serieller Konsolenzugriff vorhanden
* internes Novena-Display: Innolux N133HSE-EA1
* Display-Bridge: ITE IT6251

## Aktueller Kernel

Version:

`6.18.49`

Nix-Build-Ergebnis:

`/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

## Aktueller Device Tree

Nix-Build-Ergebnis:

`/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb`

## Aktuelle Kernelkonfiguration

Nix-Build-Ergebnis:

`/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49`

## Patchserie

### 0001

`kernel/0001-drm-bridge-it6251.patch`

Ziel:

Unterstützung der IT6251 Display-Bridge.

Der aktuelle Treiber basiert funktional auf dem historischen
Novena-IT6251-Treiber und enthält zusätzliche Diagnoseausgaben.

### 0002

`kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

Ziel:

Unterstützung des Innolux N133HSE-EA1 Panels.

### 0003

`kernel/0003-i2c-imx-debug-arbitration-lost.patch`

Ziel:

Zusätzliche Diagnose für I2C-Arbitration-Lost- und Timeout-Probleme auf
dem Novena.

Dieser Patch ist aktuell ausdrücklich ein Diagnose-Patch und noch nicht
für einen finalen Produktionsstand vorgesehen.

Insbesondere erzeugt er auf `i2c-0` sehr große Mengen an Meldungen der
Form:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

Diese Diagnoseausgabe muss vor einem finalen Image entfernt oder auf den
relevanten Display-I2C-Bus begrenzt werden.

## Aktuelles Image

Build erfolgreich abgeschlossen:

2026-09-07

Exit-Code:

`0`

Nix-Store-Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

Größe:

2581291008 Bytes

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

Dieses Image wurde am 2026-09-12 auf echter Novena-Hardware erfolgreich
mit funktionierender Linux-seitiger Displayinitialisierung getestet.

## Aktuelles Testmedium

Auf der foobox wurde eine 29.7-GiB-SD-Karte als externes Novena-
Testmedium verwendet.

Partitionierung:

### Partition 1

* Gerät auf foobox beim letzten Flash-Test: `/dev/sda1`
* Größe: 128 MiB
* Dateisystem: FAT
* Label: `FIRMWARE`
* UUID: `2178-694E`
* PARTUUID: `2178694e-01`

### Partition 2

* Gerät auf foobox beim letzten Flash-Test: `/dev/sda2`
* Größe: 2.3 GiB
* Dateisystem: ext4
* Label: `NIXOS_SD`
* UUID: `44444444-4444-4444-8888-888888888888`
* PARTUUID: `2178694e-02`

Wichtig:

Die Gerätebezeichnung `/dev/sda` ist nicht dauerhaft garantiert und muss
vor jedem destruktiven Schreibvorgang erneut geprüft werden.

## Bestätigte Display-Konfiguration

Der aktuelle Device Tree enthält für den Displaypfad unter anderem:

* IT6251 auf I2C-Adresse `0x5c`
* Innolux N133HSE-EA1 Panel
* Novena-spezifisches IT6251-Pinmux
* Novena-spezifisches Backlight-Pinmux
* LDB-Clock-Zuweisung auf PLL2 PFD2 396 MHz
* `single-master;` auf dem Display-I2C-Bus `i2c3`
* `startup-delay-us = <2000000>` für `reg_display`
* kein `regulator-always-on` auf `reg_display`

Die Clock-Zuweisung wurde durch erfolgreiche Erkennung einer stabilen
Auflösung von 1920x1080 bestätigt.

## Erkenntnis zum I2C-Multi-Master-Modus

Der Linux-i.MX-I2C-Treiber aktiviert standardmäßig Multi-Master-Betrieb,
wenn im Device Tree keine Eigenschaft `single-master` vorhanden ist.

Vor der Änderung wurden auf dem Display-I2C-Bus wiederholt Meldungen mit

`I2SR=0x93`

und Rückgabewert

`-11`

beobachtet.

Die Diagnose zeigte, dass `-11` aus der Behandlung des
Arbitration-Lost-Bits `I2SR_IAL` im i.MX-I2C-Treiber entstand.

Nach Einfügen von

`single-master;`

auf `i2c3` verschwanden diese Arbitration-Lost-/EAGAIN-Fehler auf dem
Display-I2C-Bus.

`single-master;` ist deshalb Bestandteil des aktuell funktionierenden
Display-Stands.

## Erkenntnis zur Display-Stromversorgung

Frühere Tests verwendeten auf `reg_display`:

`regulator-always-on`

Dadurch blieb die Versorgung des IT6251 während des Linux-Starts aktiv
und ein von U-Boot vorbereiteter Zustand konnte erhalten bleiben.

Nach Entfernen von `regulator-always-on` wurde nachgewiesen, dass Linux
den IT6251 tatsächlich selbst aus einem ausgeschalteten Regulatorzustand
einschalten kann.

Der Regulator verwendet aktuell:

`startup-delay-us = <2000000>`

Die gemessene Verzögerung zwischen `regulator_enable` und dem ersten
IT6251-Zugriff lag bei ungefähr zwei Sekunden und wurde damit korrekt
eingehalten.

## Erfolgreicher Hardwaretest am 2026-09-12

Am 2026-09-12 wurde das aktuelle Image auf echter Novena-Hardware
kalt gestartet.

Der IT6251 wurde bereits im ersten Product-ID-Versuch erfolgreich
erkannt.

Gelesene Product-ID-Register:

* Register `0x00`: `0x15`
* Register `0x01`: `0xca`
* Register `0x02`: `0x51`
* Register `0x03`: `0x62`

Kernelmeldung:

`IT6251 detected: vendor ca15 device 6251`

Die anschließende Initialisierung war im ersten Versuch erfolgreich.

Das DisplayPort-Linktraining endete nach zehn Iterationen.

Ermittelter Systemstatus:

`0x3e`

Ermittelte aktive Auflösung:

* hactive: 1920
* vactive: 1080

Kernelmeldungen:

`is_stable: stable 1920x1080`

`display link stable`

`bridge_enable: exit success`

Damit ist nachgewiesen, dass der aktuelle Linux-6.18.49-Stand den
IT6251 und das interne Novena-Display grundsätzlich vollständig unter
Linux initialisieren kann.

## Regulatorstatus nach erfolgreicher Initialisierung

Nach dem erfolgreichen Displaystart wurden folgende GPIO-Zustände
festgestellt:

`gpio-15 (regulator-lvds-lcd) out hi`

`gpio-28 (regulator-display) out hi`

Die Regulator-Zusammenfassung zeigte:

`lcd-lvds-power`

* aktiv
* 3300 mV
* Backlight-Consumer aktiv

`lcd-display-power`

* aktiv
* 3300 mV
* Consumer `2-005c-power` aktiv

Damit sind sowohl die Displayversorgung als auch die
Backlight-Versorgung nach der erfolgreichen Initialisierung aktiv.

## Erwartete Reset-NACKs

Während der IT6251-Initialisierung treten beim Schreiben des
Reset-Registers `0x05` vorübergehend Fehler `-6` auf.

Beobachtet wurden unter anderem:

`error -6 writing to eDP addr 0x5`

`error -6 writing to LVDS addr 0x5`

Diese Fehler sind nach aktuellem Kenntnisstand kein Hinweis auf einen
fehlgeschlagenen Displaystart.

Der historische Novena-Treiber erwartet während dieser Reset-Sequenzen
vorübergehende I2C-Nichtantworten.

Im erfolgreichen Test vom 2026-09-12 lief die Initialisierung nach
diesen Meldungen vollständig weiter und endete mit einem stabilen
1920x1080-Link.

## Noch offenes I2C-Diagnoseproblem

Der aktuelle Diagnose-Patch protokolliert Arbitration-Lost-Ereignisse
nicht nur für den Displaybus, sondern auch für andere i.MX-I2C-
Controller.

Auf `i2c-0` wurden nach dem Boot sehr große Mengen von Meldungen der Form

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

beobachtet.

Diese Meldungen überfluten den Kernel-Log und können die serielle Konsole
unbenutzbar machen.

Zusätzlich wurde auf `i2c-0` später ein

`<i2c_imx_write> write timedout`

beobachtet.

Dieses Verhalten ist getrennt vom erfolgreichen IT6251-Betrieb auf
`i2c-2` zu betrachten.

Der Debug-Patch muss vor weiteren Reproduzierbarkeitstests entschärft
werden.

## Aktueller Arbeitsstand

Das aktuelle Image erreicht erfolgreich:

1. SPL/U-Boot
2. Laden des Bootskripts
3. Linux 6.18.49
4. Device Tree
5. initrd / NixOS Stage 1
6. Root-Dateisystem
7. regulären NixOS-Login
8. Linux-seitiges Einschalten der IT6251-Versorgung
9. erfolgreiche Product-ID-Erkennung des IT
