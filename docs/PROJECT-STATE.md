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

## Aktuell gebooteter Device Tree

Der aktuell auf der externen SD-Karte im FAT-Dateisystem `FIRMWARE`
verwendete Device Tree stammt aus:

`/nix/store/l8xp4si099wjbkl2qn7ckndi3fqf5c5b-device-tree-overlays/imx6q-novena.dtb`

SHA-256:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

Der DTB auf der aktuell gebooteten externen SD-Karte ist bytegleich mit
diesem Nix-Store-Ergebnis.

Der unveränderte Kernel-Basis-DTB besitzt dagegen den SHA-256-Wert:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Der aktuelle DTB entsteht reproduzierbar aus diesem Kernel-Basis-DTB
durch Anwendung der im Projekt definierten Device-Tree-Overlays.

## Historischer Golden-DTB

Zusätzlich existiert ein früherer gesicherter Display-DTB:

`/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb`

SHA-256:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Dieser DTB ist Bestandteil des Golden-Backups:

`nixos-display-working-2026-09-11`

Er wurde aus einem früheren Device-Tree-Overlay-Zustand gebaut und ist
nicht mit dem aktuell gebooteten DTB `31f2b35e...` identisch.

Die Unterschiede und ihre historische Einordnung sind weiter unten im
Abschnitt zur Vor-Git-Entwicklung dokumentiert.

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

Auf der laufenden Novena erscheint dieses externe Testmedium aktuell als:

* `/dev/mmcblk1p1` — `FIRMWARE`
* `/dev/mmcblk1p2` — `NIXOS_SD`

Das interne etwa 3.7-GiB-MMC erscheint aktuell als `/dev/mmcblk2`.

## Bestätigte Display-Konfiguration

Der aktuell gebootete Device Tree enthält für den Displaypfad unter
anderem:

* IT6251 auf I2C-Adresse `0x5c`
* Innolux N133HSE-EA1 Panel
* Novena-spezifisches IT6251-Pinmux
* Novena-spezifisches Backlight-Pinmux
* explizite LDB-Clock-Zuweisungen
* `single-master;` auf dem Display-I2C-Bus `i2c3`
* `startup-delay-us = <2000000>` für `reg_display`
* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`

Mit diesem Device Tree wurde auf echter Novena-Hardware erfolgreich ein
stabiler Display-Link mit einer aktiven Auflösung von 1920x1080
erreicht.

Die Clock-Zuweisungen sind Bestandteil des funktionierenden aktuellen
Stands. Der genaue einzelne Vor-Git-Test, in dem diese Zuweisungen
erstmals eingeführt wurden, konnte aus den erhaltenen historischen
Aufzeichnungen bislang nicht eindeutig rekonstruiert werden.

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

Auch `reg_lvds_lcd` wurde in einem früheren Overlay-Zustand mit

`regulator-always-on`

versehen.

Dadurch blieben die entsprechenden Versorgungen während des
Linux-Starts aktiv und ein von U-Boot vorbereiteter Zustand konnte
erhalten bleiben.

Nach Entfernen von `regulator-always-on` wurde getestet, ob Linux den
IT6251 vollständig selbst aus einem ausgeschalteten
Display-Regulatorzustand einschalten kann.

Bei einem frühen Test ohne `regulator-always-on` konnte die
IT6251-Product-ID zunächst nicht erfolgreich gelesen werden.

Der Kernel-Basis-DTB enthielt zu diesem Zeitpunkt bereits:

`startup-delay-us = <200000>`

also eine Verzögerung von 200 ms.

Im später funktionierenden Zustand wurde dieser Wert auf

`startup-delay-us = <2000000>`

erhöht.

Die gemessene Verzögerung zwischen `regulator_enable` und dem ersten
IT6251-Zugriff lag danach bei ungefähr zwei Sekunden und wurde damit
korrekt eingehalten.

Mit diesem Zustand konnte Linux den IT6251 selbst einschalten und die
Product-ID bereits beim ersten Versuch erfolgreich lesen.

## Vor-Git-Entwicklung des Display-Device-Trees

Die Entwicklung des funktionierenden Display-Device-Trees begann vor
der Einrichtung der Git-Historie dieses Projekts.

Der erste Commit des Repositories ist:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Zeitpunkt:

`2026-09-11 15:20:23 +0200`

Commit-Betreff:

`Add reproducible Novena NixOS image project`

Dieser Commit ist der Root-Commit des derzeit bekannten
Repository-Verlaufs.

Für die vorhergehenden Display-Experimente existiert in der vorhandenen
Git-Historie kein älterer Commit.

Die Vor-Git-Zustände müssen deshalb aus den erhaltenen Nix-Artefakten,
Device Trees, Backups und Testaufzeichnungen rekonstruiert werden.

## Rekonstruierter früher Display-Overlay-Zustand

Der historische Golden-DTB mit SHA-256

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

lässt sich über erhaltene Nix-Derivationen auf folgenden damaligen
Overlay-Output zurückführen:

`/nix/store/pgyag1yxfcrwf1lz4zqpjhgag2ah564b-device-tree-overlays/imx6q-novena.dtb`

Der zugrunde liegende Kernel-Basis-DTB ist bytegleich mit dem Basis-DTB
des späteren aktuellen Builds:

SHA-256:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Die Unterschiede zwischen `e36cd0c8...` und dem heutigen
`31f2b35e...` entstehen deshalb aus unterschiedlichen
Overlay-Zuständen und nicht aus unterschiedlichen Kernel-Basis-DTBs.

Der rekonstruierte ältere Display-Overlay-Zustand enthielt unter
anderem:

`regulator-always-on;`

für `reg_display` sowie `reg_lvds_lcd`.

Er enthielt noch nicht:

* die späteren `assigned-clocks`
* die späteren `assigned-clock-parents`
* `single-master;` auf `i2c3`
* die Erhöhung von `startup-delay-us` auf 2000000 µs

Der Kernel-Basis-DTB enthielt bereits einen
`startup-delay-us`-Wert von 200000 µs.

## Übergang zum späteren Display-Zustand

Aus den erhaltenen Chat-, Nix- und Device-Tree-Artefakten lässt sich
folgende Entwicklung rekonstruieren:

1. früher Display-Overlay-Zustand mit `regulator-always-on`
2. erfolgreicher Build des später als `e36cd0c8...` gesicherten DTB
3. Entfernung von `regulator-always-on`
4. Test eines echten Linux-seitigen Einschaltens des IT6251
5. zunächst fehlgeschlagene Product-ID-Erkennung
6. Erhöhung von `startup-delay-us` auf 2000000 µs
7. erfolgreiche Product-ID-Erkennung nach Linux-seitigem Einschalten
8. Einführung von `single-master;` zur Vermeidung der beobachteten
   Arbitration-Lost-/EAGAIN-Probleme auf dem Display-I2C-Bus
9. spätestens im Initial-Commit `f26464f...` waren zusätzlich die
   expliziten Clock-Zuweisungen vorhanden

Die genaue zeitliche Position der Clock-Zuweisungen innerhalb der
Vor-Git-Entwicklung ist derzeit nicht eindeutig rekonstruierbar.

Ebenso ist nicht für jeden einzelnen Vor-Git-Test ein eigenständiger
gespeicherter Quellstand erhalten.

Die oben dokumentierten Device-Tree-Unterschiede selbst sind dagegen
durch die erhaltenen DTBs und Nix-Derivationen direkt belegt.

## Einordnung des Golden-Backups vom 2026-09-11

Das Golden-Backup

`nixos-display-working-2026-09-11`

enthält in seinen Metadaten den Git-Commit:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Gleichzeitig enthält beziehungsweise referenziert es den historischen
Golden-DTB mit SHA-256:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Eine direkte Untersuchung von

`hardware/novena.nix`

im Commit `f26464f...` zeigte jedoch bereits:

* `assigned-clocks`
* `assigned-clock-parents`
* `startup-delay-us = <2000000>`
* `single-master;`

und kein `regulator-always-on`.

Der Golden-DTB `e36cd0c8...` besitzt dagegen den früheren
Overlay-Zustand mit `regulator-always-on` und ohne diese späteren
Eigenschaften.

Daraus folgt:

Der im Golden-Backup gespeicherte Commit `f26464f...` dokumentiert
den damals zugeordneten Git-Stand.

Er darf jedoch nicht als Provenienznachweis dafür interpretiert werden,
dass der Golden-DTB `e36cd0c8...` aus exakt dem committed Quellzustand
von `f26464f...` gebaut wurde.

Der frühere Quellzustand war vor Einrichtung der Git-Historie vorhanden
und ist heute über die erhaltenen Nix-Artefakte und Device Trees
rekonstruiert, aber nicht als eigener Git-Commit verfügbar.

## Vergleich der beiden relevanten Display-DTB-Generationen

### Historischer Golden-DTB

SHA-256:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Wesentliche Display-Eigenschaften:

* `reg_display` mit `regulator-always-on`
* `reg_lvds_lcd` mit `regulator-always-on`
* ursprüngliches Basis-Delay von 200000 µs auf `reg_display`
* kein `single-master;`
* keine späteren expliziten Clock-Zuweisungen

### Aktuell gebooteter DTB

SHA-256:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

Wesentliche Display-Eigenschaften:

* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`
* `startup-delay-us = <2000000>` auf `reg_display`
* `single-master;` auf `i2c3`
* explizite `assigned-clocks`
* explizite `assigned-clock-parents`

Beide Zustände basieren auf demselben Kernel-Basis-DTB mit SHA-256:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

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

Der Debug-Patch muss vor abschließenden Reproduzierbarkeits- und
Langzeittests entschärft oder auf den relevanten Bus begrenzt werden.

## STMPE811 / Touchscreen

Der aktuelle Device Tree enthält weiterhin den STMPE811 auf
I2C-Adresse `0x44`.

Der Knoten ist nicht deaktiviert.

Der STMPE811 wird beim Boot zunächst erfolgreich erkannt:

`stmpe811 detected, chip id: 0x811`

Auch das Touchscreen-Eingabegerät wird zunächst registriert.

Im weiteren Betrieb wurden jedoch wiederholt fehlerhafte
STMPE-I2C-Zugriffe beobachtet, darunter Rückgabewerte:

* `-110` — Timeout
* `-6` — keine Antwort des Geräts
* `-11` — erneuter Versuch beziehungsweise temporär nicht verfügbar

Typische Kernelmeldungen lauten:

`stmpe-i2c 0-0044: failed to read regs 0xb: -110`

beziehungsweise entsprechend mit `-6` oder `-11`.

Der historische Golden-DTB `e36cd0c8...` und der aktuell gebootete DTB
`31f2b35e...` enthalten bezüglich des STMPE811 denselben aktiven
STMPE-/Touchscreen-Teilbaum.

Damit ist nachgewiesen:

Die erfolgreiche Displayinitialisierung wurde nicht durch eine
Deaktivierung des STMPE811 erreicht.

Die Display-Entwicklung und das aktuelle STMPE-/Touchscreen-Problem sind
getrennte Themen.

Als kontrollierter nächster Test soll deshalb nur der STMPE811-/
Touchscreen-Pfad reproduzierbar deaktiviert werden, während der
funktionierende Displaypfad unverändert bleibt.

Vor diesem Test muss der genaue Overlay-Zielknoten aus dem verwendeten
Linux-6.18.49-Device-Tree-Quellstand eindeutig bestimmt werden.

## Aktueller Bootzustand

Die aktuell getestete Novena verhält sich beim Booten wie folgt:

* externe SD-Karte eingesetzt:
  Boot von der externen SD-Karte
* keine externe SD-Karte:
  Boot von der SATA-SSD
* Einschalten mit gedrückter User-Taste:
  Boot über das interne MMC beziehungsweise den Recovery-Pfad

Die aktuelle NixOS-Arbeit erfolgt auf der externen SD-Karte.

## Aktueller NixOS-Laufzeitstand

Aktuell bestätigt:

* NixOS `26.05.20260903.a5cc6f2 (Yarara)`
* Linux `6.18.49`
* Architektur `armv7l`
* Root-Dateisystem auf `/dev/mmcblk1p2`
* `/nix/store` auf `/dev/mmcblk1p2`
* Kernelparameter mit `loglevel=7`
* serieller Konsolenzugriff vorhanden

`loglevel=7` bleibt vorläufig bewusst aktiviert, solange die
I2C-/STMPE-Diagnose noch nicht abgeschlossen ist.

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
9. erfolgreiche Product-ID-Erkennung des IT6251
10. Initialisierung der IT6251-Bridge
11. DisplayPort-Linktraining
12. stabilen Display-Link
13. aktive Auflösung 1920x1080
14. funktionierendes internes Novena-Display

Der Linux-seitige Displaypfad ist damit grundsätzlich funktionsfähig.

Noch offen sind insbesondere:

* reproduzierbare Untersuchung beziehungsweise kontrollierte
  Deaktivierung des fehlerhaften STMPE811-/Touchscreen-Pfads
* anschließende Prüfung, ob der Displaypfad davon unbeeinflusst bleibt
* Entschärfung beziehungsweise Entfernung des sehr ausführlichen
  I2C-Diagnose-Patches
* mehrere identische echte Kaltstarts zur Bestätigung der
  Reproduzierbarkeit
* endgültige Bereinigung des Images für einen universellen
  Produktionsstand

## Nächster geplanter Test

Der nächste kontrollierte Hardwaretest soll den STMPE811 beziehungsweise
den zugehörigen Touchscreen-Pfad deaktivieren, ohne den funktionierenden
Displaypfad zu verändern.

Dazu wird zunächst der tatsächlich verwendete Linux-6.18.49-
Device-Tree-Quellstand untersucht, um den exakten STMPE811-Knoten
beziehungsweise sein DTS-Label zu bestimmen.

Danach soll ein eigenes, klar getrenntes Device-Tree-Overlay erstellt
werden.

Vor dem Schreiben auf das Testmedium werden mindestens geprüft:

* Quell-DTB-Hash
* erzeugter DTB-Hash
* STMPE811-Status im erzeugten DTB
* unveränderte Display-Eigenschaften
* vorhandene Rückfallkopie des aktuell funktionierenden DTB

Erst danach erfolgt ein Hardwaretest auf der Novena.

## Reproduzierbarkeitsziel nach STMPE-Test

Nach Kontrolle des STMPE-/Touchscreen-Pfads sollen mindestens fünf
identische echte Kaltstarts durchgeführt werden.

Ein echter Kaltstart bedeutet für diesen Test:

1. Novena sauber herunterfahren
2. Versorgung vollständig entfernen
3. sicherstellen, dass das Board tatsächlich stromlos ist
4. kurze stromlose Wartezeit
5. mit unverändert eingelegter externer SD-Karte neu einschalten
6. keine User-/Recovery-Taste betätigen
7. vollständigen Bootvorgang und Displayinitialisierung beobachten
8. Kernel- und relevante I2C-/Displaymeldungen sichern

Da die Echtzeituhr beziehungsweise frühe Systemzeit auf der Novena
nicht als zuverlässig bestätigt ist, sollen die Testläufe zusätzlich
über Boot-ID und monotone Kernel-Zeitstempel unterschieden werden.

## Versionskontrolle

Git wurde für dieses Projekt erst am 2026-09-11 eingerichtet.

Initial-Commit:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Aktueller dokumentierter HEAD:

`92daa0ce11c7713c7ab54c18668299d364f524d1`

Commit-Betreff:

`Document successful Novena display cold boot`

Der funktionierende aktuelle Projektstand ist damit in Git gesichert.

Die Vor-Git-Displayentwicklung muss dagegen anhand der erhaltenen
Nix-Artefakte, Device Trees, Backups und Chat-/Testaufzeichnungen
rekonstruiert werden.
