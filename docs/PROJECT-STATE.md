# Project State

Stand: 2026-09-13

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

Für die verschiedenen Referenz- und Diagnosezustände müssen drei
Kernel-Builds unterschieden werden.

### Historischer Golden-Kernel

Version:

`6.18.49`

Nix-Store-Ausgabe:

`/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Dieser Build enthält die Display-Patches `0001` und `0002`, aber noch
nicht die späteren I2C-Diagnose-Patches.

### Bisheriger Debug-Kernel

Nix-Store-Ausgabe:

`/nix/store/2d1h5iihfq560zm0sg4kh4n4ydvk2a2m-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Deriver:

`/nix/store/422lrj9vi0ydb6cczl0ib8wjx8ll7sqc-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Dieser Build enthält `0001`, `0002` und `0003`.

Mit diesem Kernel wurde die Fünf-Cold-Boot-Serie mit dem
STMPE811-deaktivierten Device Tree durchgeführt.

### Neuer 0004-Diagnosekernel

Nix-Store-Ausgabe:

`/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Deriver:

`/nix/store/qk21vdydrywnkqlh3rrvg7n9zn4pwapk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Kernel-Quelle:

`/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz`

Separate Nix-Ausgaben:

* `out`: `/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49`
* `dev`: `/nix/store/4c7nvncjad0sbgahcngalj8nn0la7njh-linux-armv7l-unknown-linux-gnueabihf-6.18.49-dev`
* `modules`: `/nix/store/z1dl1i2pkqzz3p3cg1y9s7knhmbwzczy-linux-armv7l-unknown-linux-gnueabihf-6.18.49-modules`

Das Modulverzeichnis ist `lib/modules/6.18.49`.

Die Derivation wurde geprüft und enthält exakt die Patchfolge
`0001`, `0002`, `0003`, `0004`.

Der 0004-Kernel ist erfolgreich gebaut und seine Provenienz ist geprüft,
aber zum Stand dieses Dokuments noch nicht auf der Novena getestet.

## Device-Tree-Referenz- und Teststände

Für die aktuelle Diagnose müssen Baseline- und Test-DTB klar
unterschieden werden.

Die reproduzierte Display-Baseline stammt aus:

`/nix/store/l8xp4si099wjbkl2qn7ckndi3fqf5c5b-device-tree-overlays/imx6q-novena.dtb`

SHA-256:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

Der unveränderte Kernel-Basis-DTB besitzt dagegen den SHA-256-Wert:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Die Display-Baseline entsteht reproduzierbar aus diesem Kernel-Basis-DTB
durch Anwendung der im Projekt definierten Display- und SD-Overlays.

Für den kontrollierten STMPE811-Test wurde daraus zusätzlich ein
separater Test-DTB gebaut:

`d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8`

Der dekompilierte semantische Vergleich zwischen Baseline- und Test-DTB
zeigte als einzigen inhaltlichen Unterschied:

```dts
status = "disabled";
```

am STMPE811-Knoten.

Mit diesem STMPE811-deaktivierten Test-DTB wurde die dokumentierte
Fünf-Cold-Boot-Serie durchgeführt.

Der physisch derzeit auf der externen SD-Karte vorhandene aktive DTB wird
an dieser Stelle bewusst nicht als aktueller Zustand behauptet, solange
er nach den Test- und Umbauarbeiten nicht erneut direkt vom Medium
verifiziert wurde.

## Historischer Golden-DTB

Zusätzlich existiert ein früherer gesicherter Display-DTB:

`/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb`

SHA-256:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Dieser DTB ist Bestandteil des Golden-Backups:

`nixos-display-working-2026-09-11`

Er wurde aus einem früheren Device-Tree-Overlay-Zustand gebaut und ist
nicht mit der später reproduzierten Display-Baseline `31f2b35e...`
identisch.

Die Unterschiede und ihre historische Einordnung sind weiter unten im
Abschnitt zur Vor-Git-Entwicklung dokumentiert.

## Aktuelle Kernelkonfiguration

Nix-Build-Ergebnis:

`/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49`

## Patchserie

### 0001

`kernel/0001-drm-bridge-it6251.patch`

Unterstützung der IT6251 Display-Bridge.

### 0002

`kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

Unterstützung des Innolux N133HSE-EA1 Panels.

### 0003

`kernel/0003-i2c-imx-debug-arbitration-lost.patch`

Zusätzliche Diagnose für I2C-Arbitration-Lost- und Timeout-Probleme.
Der Patch ist ausdrücklich ein Diagnose-Patch und noch nicht für einen
finalen Produktionsstand vorgesehen.

### 0004

`kernel/0004-i2c-imx-debug-start-state.patch`

Dieser Patch erfasst für IT6251-Adresse `0x5c` Register-Snapshots an
mehreren Stellen des START-Pfads:

* A — nach Controller-Aktivierung und Stabilisierung, vor MSTA
* B — unmittelbar nach dem Schreiben von MSTA
* C — nach `i2c_imx_bus_busy()`
* D — nach der Konfiguration von IIEN, MTX und TXAK
* E — unmittelbar vor dem ersten Adressbyte

An A bis E werden keine zusätzlichen Kernelmeldungen ausgegeben. Die
Snapshots werden erst aus dem bereits instrumentierten ISR-Pfad
zusammengefasst ausgegeben.

Der Patch wurde gegen den exakten Linux-6.18.49-Quellstand mit bereits
angewendetem `0003` geprüft, lässt sich sauber anwenden und wurde
anschließend erfolgreich gebaut. Ein Hardwaretest steht noch aus.

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

Die reproduzierte Display-Baseline `31f2b35e...` enthält für den
Displaypfad unter anderem:

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

Die exakte Linux-6.18.49-Quellcodeanalyse des i.MX-I2C-Treibers zeigt:

```c
i2c_imx->multi_master =
        !of_property_read_bool(pdev->dev.of_node, "single-master");
```

Damit setzt `single-master;` den internen Treiberzustand eindeutig auf
`multi_master = false`.

Historisch verschwanden nach Einführung von `single-master;` die zuvor
sichtbaren `-11`-/`EAGAIN`-Fehler auf dem Display-I2C-Bus.

Die spätere Quellcodeanalyse präzisiert jedoch die Bedeutung dieser
Beobachtung: Bei `multi_master = false` werden die normalen IAL-Prüfungen
in `i2c_imx_bus_busy()` und `i2c_imx_trx_complete()` übersprungen.

`single-master;` verhindert daher nicht, dass die Hardware das
`I2SR_IAL`-Bit setzt.

Die Cold-Boot-Fehler zeigen, dass auch mit aktivem `single-master;` ein
IAL-Zustand auftreten kann. Bei den fehlgeschlagenen IT6251-Transfers
wurde anschließend zusätzlich `RXAK` beobachtet; der ISR-Pfad liefert
in diesem Fall `-ENXIO`, also `-6`.

Die frühere Beobachtung bleibt historisch korrekt, darf aber nicht mehr
als Beweis dafür interpretiert werden, dass Hardware-Arbitration-Lost
vollständig verhindert wurde.

## Erkenntnis zur Display-Stromversorgung

Frühere Tests verwendeten auf `reg_display` und zeitweise auch auf
`reg_lvds_lcd` die Eigenschaft `regulator-always-on`.

Nach Entfernen dieser Eigenschaft wurde geprüft, ob Linux den IT6251
selbst aus einem ausgeschalteten Regulatorzustand einschalten kann.

Der Kernel-Basis-DTB enthielt ursprünglich `startup-delay-us = <200000>`.
Im später funktionierenden Zustand wurde dieser Wert auf
`startup-delay-us = <2000000>` erhöht.

Die konfigurierte Verzögerung von ungefähr zwei Sekunden wurde in den
Bootlogs tatsächlich eingehalten.

Die spätere Fünf-Cold-Boot-Serie zeigt jedoch, dass diese zwei Sekunden
allein keine reproduzierbare Initialisierung garantieren:

* 2 von 5 Kaltstarts initialisierten das Display direkt erfolgreich.
* 3 von 5 Kaltstarts scheiterten beim ersten IT6251-Zugriff.

In Cold Boot 5 gelang ein DRM-Rebind bereits nach ungefähr 20,8 Sekunden
ausgeschaltetem Display-Regulator. Beim erfolgreichen Rebind wurde die
IT6251-Product-ID nur wenige Millisekunden nach `regulator enabled`
erfolgreich gelesen.

Eine einfache Erklärung im Sinne von „der IT6251 benötigt lediglich
mehr zusätzliche Wartezeit nach dem Regulator-Enable“ ist damit stark
entkräftet.

Das 2-Sekunden-Startup-Delay bleibt Teil des bekannten funktionierenden
Gesamtzustands, ist aber nicht als alleinige Root-Cause-Lösung des
Cold-Boot-Problems anzusehen.

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

Die Fünf-Cold-Boot-Serie hat das relevante Display-I2C-Fehlermuster
deutlich eingegrenzt.

Bei erfolgreichen Starts beziehungsweise erfolgreichen Rebinds wurden
für die ersten IT6251-Transfers wiederholt `I2SR=0xa2` beziehungsweise
`0xa6` mit `I2CR=0xf8` beobachtet.

Bei den drei fehlgeschlagenen Kaltstarts begann dagegen bereits der erste
IT6251-Transfer mit `I2SR=0x93` und `I2CR=0xd8`. Teilweise wurde bei
Timeouts außerdem `I2SR=0x91` mit `I2CR=0xd8` beobachtet.

`I2SR=0x93` enthält unter anderem `ICF`, `IAL`, `IIF` und `RXAK`.
`I2CR=0xd8` enthält kein `MSTA`.

Die vorhandene Debugausgabe liest I2SR und I2CR im ISR-Wrapper, bevor der
Master-ISR die Statusbits verarbeitet. Daraus folgt nicht, dass der
Controller den Transfer ohne MSTA begonnen hat.

Die Quellcodeanalyse von `i2c_imx_start()` zeigt ausdrücklich, dass MSTA
vor dem Transfer angefordert wird. Der beobachtete Zustand bedeutet nur,
dass MSTA zum Zeitpunkt des ISR-Snapshots bereits nicht mehr gesetzt war.

Für i.MX6 wird `I2SR_CLR_OPCODE_W0C` verwendet. `i2c_imx_start()` schreibt
vor jedem START `0x00` nach I2SR und löscht damit alte Statusbits.

Ein lediglich aus einem früheren Transfer stehen gebliebenes IAL-Bit ist
daher stark entkräftet. Noch nicht geklärt ist, warum die Hardware bei
den fehlgeschlagenen Cold Boots überhaupt ein frisches IAL erzeugt.
Genau dafür wurde Diagnose-Patch `0004` erstellt.

Unabhängig von diesem IT6251-Fehlermuster wurden mit Diagnose-Patch
`0003` auch auf `i2c-0` umfangreiche Arbitration-Lost-Meldungen
beobachtet, insbesondere:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

Diese Meldungen konnten den Kernel-Log stark überfluten und die serielle
Konsole praktisch unbenutzbar machen.

Später wurde auf `i2c-0` außerdem beobachtet:

`<i2c_imx_write> write timedout`

Diese `i2c-0`-Beobachtungen sind vom hier untersuchten IT6251-Pfad auf
`i2c-2` getrennt zu behandeln. Ihre genaue Ursache ist ebenfalls nicht
geklärt.

Die umfangreiche Diagnoseinstrumentierung muss vor einem finalen
Produktionsstand entfernt oder gezielt auf das tatsächlich notwendige
Minimum begrenzt werden.

## STMPE811 / Touchscreen

Der STMPE811 auf I2C-Adresse `0x44` war sowohl im historischen Golden-DTB
`e36cd0c8...` als auch im späteren Baseline-DTB `31f2b35e...` aktiv.

Der verwendete Device-Tree-Quellstand besitzt für diesen Knoten das Label
`touch`; im Basis-DTB ist das Symbol ebenfalls vorhanden. Damit ist
`&touch` als Overlay-Ziel eindeutig belegt.

Für den kontrollierten Test wurde ein separates Overlay erstellt:

```dts
&touch {
        status = "disabled";
};
```

Baseline-DTB:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

STMPE-deaktivierter Test-DTB:

`d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8`

Der dekompilierte semantische Vergleich zeigte als einzigen inhaltlichen
Unterschied `status = "disabled"` am STMPE811-Knoten.

Im laufenden System wurde anschließend bestätigt:

* STMPE811 im Live-Device-Tree deaktiviert
* keine STMPE-Probe
* keine `stmpe-i2c 0-0044`-Fehler
* keine zugehörige STMPE-Fehlerflut

Das Display-Cold-Boot-Problem blieb jedoch bestehen.

Damit ist experimentell bestätigt, dass die STMPE811-/Touchscreen-Fehler
nicht die Ursache des intermittierenden IT6251-Cold-Boot-Problems waren.


## STMPE-isolierte Fünf-Cold-Boot-Serie

Mit dem STMPE811-deaktivierten Test-DTB wurden fünf echte Kaltstarts
durchgeführt.

Ergebnis:

* Cold Boot 1: PASS
* Cold Boot 2: PASS
* Cold Boot 3: FAIL, danach DRM-Rebind PASS
* Cold Boot 4: FAIL, danach DRM-Rebind PASS
* Cold Boot 5: FAIL, danach unmittelbarer DRM-Rebind PASS

Gesamt:

* native Cold-Boot-Erfolge: 2/5
* native Cold-Boot-Fehler: 3/5
* erfolgreiche Runtime-Recoveries nach Fehler: 3/3

Das ursprüngliche Fünf-Boot-Stabilitätskriterium wurde damit nicht
erfüllt.

Erfolgreiche Starts beziehungsweise Recoveries zeigen typischerweise
`I2SR=0xa2` oder `0xa6` mit `I2CR=0xf8`.

Die drei fehlgeschlagenen Cold Boots zeigen beim ersten IT6251-Transfer
`I2SR=0x93` mit `I2CR=0xd8`, teilweise ergänzt durch Timeout-Zustände
`0x91/0xd8`.

Alle drei Fehler konnten im selben laufenden System durch Unbind/Bind des
DRM-Pfads behoben werden.

Die Recovery beweist noch nicht, welcher einzelne Effekt dafür
verantwortlich ist. Möglich bleiben insbesondere I2C-Controller-
Reinitialisierung, Buszustand, Regulator-Power-Cycle, geänderte Reihenfolge
oder Timing sowie Kombinationen daraus.

Eine Root Cause ist damit noch nicht bewiesen.

## Diagnose-Meilenstein 0004/0005 am 2026-09-13

Die Hardwaretests der Diagnose-Patches `0004` und `0005` sind abgeschlossen.

`0004` wurde zunächst mit aktivem `single-master;` getestet. Der native Cold Boot schlug fehl und zeigte:

`A=81/80 B=93/80 C=93/80 D=93/d8 E=93/d8 IRQ=93/d8`

Im gleichen Boot war ein DRM-Rebind erfolgreich.

Anschließend wurde `single-master;` isoliert entfernt, während STMPE811 deaktiviert blieb. Der resultierende DTB besitzt SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Auch ohne `single-master;` blieb der Cold-Boot-Fehler bestehen.
`i2c_imx_bus_busy()` erkannte `IAL=0x93` und brach mit `-EAGAIN` (`-11`) ab.
Damit ist bestätigt, dass `single-master;` das vom i.MX6-I2C-Controller
gemeldete IAL-Ereignis nicht verursacht und nicht verhindert; die
Eigenschaft verändert die Treiberbehandlung.

Für die Ausgabe der START-Snapshots auch auf diesem frühen Fehlerpfad wurde `kernel/0005-i2c-imx-debug-start-error.patch` ergänzt.

SHA-256 des `0005`-Patches:

`81c2de96d7200a6e7e2684518c711bc5686b8d5295a52d5e7aaa54efed94e556`

Kernel mit `0001` bis `0005`:

`/nix/store/97pmqlp5xvsh1l0i877lfc4r4sq6i7np-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Deriver:

`/nix/store/lakip7c5jxqj4ys7vgwzh1fcspk465vx-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Quelle:

`/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz`

SHA-256 des `0005`-`zImage`:

`197ce0ef325307a877eb5a889519387be91fdf536da3bee03883f0c40264fc7c`

Der erste echte Cold Boot mit diesem Stand schlug fehl.

Boot-ID:

`48462ba4-4972-4d23-9261-e69c822ed584`

Der neue Fehlerpfad zeigte reproduzierbar:

`A=81/80 B=93/80 C=83/80 ret=-11`

A liegt vor dem `MSTA`-Versuch und enthält noch kein `IAL`. Unmittelbar nach dem `MSTA`-Versuch zeigt B ein frisches `IAL`; `MSTA` ist bereits nicht mehr gesetzt. C wurde nach Rückkehr aus `i2c_imx_bus_busy()` erfasst, nachdem dort `IAL` gelöscht wurde. Der Rückgabewert ist `-EAGAIN`.

Im exakt gleichen Boot war ein DRM-Rebind erfolgreich. Die erfolgreichen START-Snapshots lauteten wiederholt:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

mit `IRQ=a2/f8` beziehungsweise `IRQ=a6/f8`.

Der IT6251 erreichte anschließend `System status: 0x3e`, `hactive: 1920`, `vactive: 1080`, `display link stable` und `bridge_enable: exit success`.

Der direkte Same-Boot-Vergleich lautet damit:

`Cold Boot FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

`Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Die entscheidende Divergenz entsteht beim Eintritt in den Master-/START-Zustand. Die tieferliegende Root Cause ist weiterhin nicht bewiesen.

Das vollständige Testarchiv `novena-0005-no-single-master-test-2026-09-13.tar.gz` wurde auf Novena und foobox identisch verifiziert.

SHA-256:

`0c38c954a7fac033b5fd619653efd8eb0d5fffa3c4024329cb6d85d247e1c3db`

foobox-Pfad:

`/home/loomit/novena-backups/0005-no-single-master-2026-09-13/novena-0005-no-single-master-test-2026-09-13.tar.gz`

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

Das aktuelle Image erreicht grundsätzlich den vollständigen Linux-Displaypfad bis zu einem stabilen 1920x1080-Link, ist aber weiterhin nicht cold-boot-stabil.

Die STMPE811-Isolation ist abgeschlossen. Die STMPE-I2C-Fehler verschwinden bei deaktiviertem STMPE811 vollständig; das intermittierende IT6251-Cold-Boot-Problem bleibt bestehen.

Die Diagnose-Patches `0004` und `0005` wurden inzwischen gebaut, provenienzgeprüft und auf echter Novena-Hardware getestet.

Der entscheidende Befund ist jetzt direkt gemessen:

`Cold Boot FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

gegen:

`Same-Boot Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Der Zustand A ist identisch. Beim fehlgeschlagenen Cold Boot erscheint unmittelbar nach dem Versuch, `MSTA` zu setzen, ein frisches `IAL`; `MSTA` bleibt nicht gesetzt. Beim erfolgreichen Rebind bleibt `MSTA` gesetzt und anschließend erscheint `IBB`.

Das Entfernen von `single-master;` beseitigt das vom i.MX6-I2C-Controller gemeldete IAL-Ereignis nicht. Ohne `single-master;` erkennt `i2c_imx_bus_busy()` das IAL und liefert `-EAGAIN`.

Die Root Cause des unterschiedlichen Hardwareverhaltens zwischen Cold Boot und Same-Boot-Rebind ist weiterhin offen.

`loglevel=7` und die Diagnoseinstrumentierung bleiben für die Root-Cause-Untersuchung bewusst aktiv.

## Nächster geplanter Test

Der Diagnose-Meilenstein `0005` ist abgeschlossen und extern gesichert.

Der nächste Schritt ist eine gezielte Root-Cause-Analyse des Master-/START-Übergangs auf dem i.MX6-I2C-Controller.

Ausgangspunkt:

`FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

`PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Zu untersuchen sind insbesondere der tatsächliche Buszustand unmittelbar vor und beim START, mögliche SDA-/SCL-Zustände, Controller-/Pinmux-/Clock-Zustand zwischen Cold Boot und Rebind, die Initialisierungsreihenfolge anderer I2C-Geräte und Controller sowie das zusätzlich beobachtete Arbitration-Lost-Verhalten auf `i2c-0`.

Eine funktionale Korrektur soll erst vorgenommen werden, wenn eine konkrete Ursache ausreichend belegt ist.

Vor jedem neuen Kernel-, DTB- oder Bootmedium-Test müssen Rollback-Artefakte und Backups erneut geprüft werden.

## Reproduzierbarkeitsziel nach STMPE-Test

Das Fünf-Cold-Boot-Kriterium nach dem STMPE-Test wurde durchgeführt und nicht bestanden:

* 5 echte Kaltstarts
* 2 direkte Display-PASS
* 3 direkte Display-FAIL
* 3 erfolgreiche Runtime-Recoveries nach den drei Fehlern

Die anschließende `0004`-/`0005`-Diagnose hat den Fehlerpfad wesentlich genauer lokalisiert, aber noch keine Root Cause beseitigt.

Der aktuelle Displaystand darf deshalb weiterhin nicht als cold-boot-stabiler Produktionsstand eingestuft werden.

Ein neues Stabilitätskriterium wird erst nach einer gezielten funktionalen Korrektur festgelegt. Danach muss erneut eine Serie identischer echter Kaltstarts ohne manuelle Recovery bestanden werden.

## Versionskontrolle

Git wurde für dieses Projekt erst am 2026-09-11 eingerichtet.

Initial-Commit:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Ausgangs-HEAD vor den Dokumentationsänderungen vom 2026-09-13:

`9719cd3b9dc1199536b23fc5b8dca93280611dec`

Commit-Betreff:

`Document Novena display provenance and test state`

Der Ausgangsstand vor den Dokumentations- und Diagnoseänderungen vom
2026-09-13 ist damit durch Commit
`9719cd3b9dc1199536b23fc5b8dca93280611dec` in Git gesichert.

Der Dokumentationsstand bis einschließlich der STMPE- und frühen
Cold-Boot-Diagnose wurde anschließend mit Commit `b4fa143`
(`Document STMPE and display cold-boot diagnosis`) gesichert und zu
Codeberg sowie GitHub gepusht.

Die danach vorgenommenen Änderungen an `hardware/novena.nix`, die
Diagnose-Patches `0004` und `0005` sowie die hier dokumentierten
Hardwaretestergebnisse sind zum Stand dieses Dokuments noch nicht als
gemeinsamer Projektstand committed. Vor einem Commit muss der vollständige
Diff gemeinsam geprüft werden.

Die Vor-Git-Displayentwicklung muss dagegen anhand der erhaltenen
Nix-Artefakte, Device Trees, Backups und Chat-/Testaufzeichnungen
rekonstruiert werden.
