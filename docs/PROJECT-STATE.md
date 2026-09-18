# Project State

Stand: 2026-09-15

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

## Aktueller Checkpoint nach Diagnose-Patch 0006 – 2026-09-15

Dieser Abschnitt ist der maßgebliche Wiedereinstiegspunkt für die weitere
I2C-/Display-Diagnose. Ältere Abschnitte dieses Dokuments bleiben als
historische Dokumentation erhalten; bei widersprüchlichen Aussagen gilt
dieser Checkpoint zusammen mit dem verifizierten Git-Stand und den
gesicherten Testartefakten.

### Versionskontrolle des 0006-Checkpoints

Verifizierter Ausgangs-HEAD vor Abschluss des 0006-Checkpoints:

`1d1384adc462371d741b4b95afe4c93e9c149052`

Commit-Betreff:

`Document build host and refresh project overview`

Zu diesem Ausgangszeitpunkt waren `HEAD`, `origin/main` und `github/main`
identisch.

Der 0006-Checkpoint umfasst die Änderung an `hardware/novena.nix`, den
neuen Patch `kernel/0006-i2c-imx-debug-start-transition.patch` sowie die
zugehörige Projektdokumentation. Als abgeschlossener Git-Checkpoint gilt
dieser Stand erst, wenn der aktuelle Commit zu beiden Projekt-Remotes
übertragen wurde, `HEAD`, `origin/main` und `github/main` dort erneut
identisch verifiziert wurden und der lokale Worktree sauber ist.

Der endgültige Commit-Hash wird hier bewusst nicht vorab festgeschrieben,
da diese Datei selbst Bestandteil dieses Commits ist. Für einen späteren
Wiedereinstieg ist der dann auf beiden Remotes verifizierte aktuelle
`HEAD` maßgeblich.

### Diagnose-Patch 0006

Patch:

`kernel/0006-i2c-imx-debug-start-transition.patch`

SHA-256:

`23a0e0c00860565fca1d8ccb1546b4d8e893d0f423e6c450013e76a47b70903f`

Der Patch ergänzt acht unmittelbar aufeinanderfolgende Messpunkte M0 bis M7
nach dem Schreiben von MSTA. Innerhalb dieses Messfensters werden bewusst
keine Delays, Logs oder Timestamp-Abfragen ausgeführt. Die MMIO-Lesezugriffe
selbst bleiben eine unvermeidbare diagnostische Timing-Beeinflussung.

Kernel-Deriver:

`/nix/store/2g78mma5gxjvr9g6mci7c66mkcy2yssk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Kernel-Ausgabe:

`/nix/store/f03kxwghama2l7if4p1798fvcyrhvwjj-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Separate Ausgaben:

* `dev`: `/nix/store/ay6ds8zsb4bxm284xjvw4vydp09w3dv0-linux-armv7l-unknown-linux-gnueabihf-6.18.49-dev`
* `modules`: `/nix/store/08ls92b1vpj0v2nqa3ssygdms6zsslbb-linux-armv7l-unknown-linux-gnueabihf-6.18.49-modules`

SHA-256 des `zImage`:

`f5cb8d264c14286e53a0ea5884e167c7d4815c529046a71e1813cb37512302d5`

Kernel-Basis-DTB SHA-256:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Finaler Overlay-DTB SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Der finale DTB ist bytegleich mit dem bereits getesteten Zustand ohne
`single-master;`. STMPE811 ist deaktiviert; `single-master;` ist auf `i2c3`
nicht vorhanden. Damit blieb die Device-Tree-Seite für den 0006-Vergleich
konstant.

### Externes 0006-Testmedium

Beim letzten verifizierten foobox-Zugriff war die externe Test-SD `/dev/sda`
mit `FIRMWARE` auf `/dev/sda1` und `NIXOS_SD` auf `/dev/sda2`. Diese
Gerätebezeichnung darf bei einem späteren Einstecken nicht vorausgesetzt
werden und muss vor jedem Schreibzugriff erneut ermittelt werden.

Vor dem 0006-Test wurde bestätigt:

* vorheriges `zImage`: 0005, SHA-256 `197ce0ef325307a877eb5a889519387be91fdf536da3bee03883f0c40264fc7c`
* `novena.dtb`: SHA-256 `0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`
* U-Boot lädt `zImage`, `initrd.uimg` und `novena.dtb` direkt von `FIRMWARE`

Für 0006 wurde ausschließlich `zImage` ersetzt. DTB, initrd, Bootskript,
Root-Dateisystem und U-Boot blieben unverändert. Der neue `zImage`-Hash
wurde vom Medium als `f5cb8d264c14286e53a0ea5884e167c7d4815c529046a71e1813cb37512302d5`
verifiziert.

Rollback-Sicherung vor der Änderung:

`/home/loomit/novena-backups/test-sd-before-0006-2026-09-14`

Sie enthält den ersten 16-MiB-Bereich einschließlich Prepartition/U-Boot
sowie ein vollständiges Image der FIRMWARE-Partition.

Die externe Test-SD bleibt nach dem Test unverändert im reproduzierbaren
0006-Zustand.

### 0006 echter POR-Cold-Boot

Der Test erfolgte aus vollständig ausgeschaltetem Zustand. U-Boot meldete:

`Reset cause: POR`

Boot-ID:

`e904173c-7b90-4f71-b3da-e8e450528670`

Das interne Display blieb aus. Der Fehler trat bei den IT6251-Product-ID-
Zugriffen reproduzierbar auf:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

`NOVENA-I2C: IT6251 START failure A=81/80 B=93/80 C=83/80 ret=-11`

Neue 0006-Messung:

`M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80`

Damit ist der fehlerhafte Zustand bereits bei der ersten beobachtbaren
Probe nach dem MSTA-Schreibzugriff vollständig vorhanden. A unmittelbar vor
dem MSTA-Versuch ist noch `81/80`; bei M0 ist IAL bereits gesetzt und MSTA
bereits wieder gelöscht.

Die Divergenz wurde damit auf das sehr kleine Zeitfenster zwischen dem
Pre-MSTA-Snapshot A und der ersten Post-MSTA-Beobachtung M0 eingegrenzt.
Die tieferliegende Ursache ist weiterhin nicht bewiesen.

### Same-Boot-A/B-Vergleich

Ein einfaches Unbind/Bind des IT6251-I2C-Treibers registrierte die DRM-Bridge
erneut, reaktivierte aber nicht den vollständigen Displaypfad und erzeugte
keinen neuen Product-ID-Transfer.

Ein anschließendes Same-Boot-Unbind/Bind des `imx-ldb`-Plattformtreibers
reaktivierte dagegen die vollständige Pipeline. Die IT6251-Initialisierung
und der Displaystart waren erfolgreich.

Erfolgreicher Beginn:

`M0=81/a0`

Spätere Samples zeigen je nach Transfer weiterhin `81/a0` oder den Übergang
zu `a1/a0`. Die erfolgreichen START-Snapshots entsprechen:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

mit erfolgreichen IRQ-Zuständen `a2/f8` beziehungsweise `a6/f8`.

Der Displaypfad erreichte anschließend wieder:

* `System status: 0x3e`
* `hactive: 1920`
* `vactive: 1080`
* `display link stable`
* `bridge_enable: exit success`

Der entscheidende Same-Boot-Vergleich lautet somit:

`Cold FAIL: A=81/80 -> M0=93/80`

`LDB-Rebind PASS: A=81/80 -> M0=81/a0`

Der Pre-MSTA-Zustand ist gleich; die Divergenz ist bereits beim ersten
beobachtbaren Zustand nach dem MSTA-Schreibzugriff vorhanden. Dies spricht
gegen eine normale Arbitration-Lost-Situation erst während eines bereits
laufenden Transfers und lokalisiert das Problem sehr eng am Master-/START-
Erwerb. Eine konkrete Root Cause folgt daraus noch nicht.

### Evidence und Provenienz

Evidence-Verzeichnis auf der foobox:

`/home/loomit/novena-backups/i2c-0006-evidence-e904173c-2026-09-15`

Die acht von `SHA256SUMS` erfassten Dateien wurden nach der Übertragung mit
`sha256sum -c SHA256SUMS` vollständig als `OK` verifiziert.

Archiv:

`/home/loomit/novena-backups/novena-i2c-0006-evidence-e904173c-2026-09-15.tar.gz`

SHA-256:

`2b4b635ecfb693a2d7471efd1c67e9ccc65e240d8a08b207cd3e9805750d0ee8`

Die Novena-Wallclock war während der Aufzeichnung falsch und zeigte in
Dateien teilweise den 13. September. Maßgeblich für die Zuordnung sind die
Boot-ID, monotone Kernel-Zeitstempel und die SHA-256-verifizierten
Testartefakte. Der physische Test fand am 2026-09-15 statt.

Nach der Evidence-Sicherung wurde die Novena sauber heruntergefahren.
Aktueller physischer Zustand am Checkpoint: **Novena ausgeschaltet**.

### Aktueller technischer Befund

Die STMPE811-Isolation ist abgeschlossen und STMPE811 bleibt für diese
Diagnose deaktiviert. Das Entfernen von `single-master;` beseitigt das
Hardware-IAL-Ereignis nicht.

0006 zeigt nun:

`FAIL: A=81/80 -> M0=93/80`

gegen:

`PASS: A=81/80 -> M0=81/a0`

Beim Cold-Boot-Fehler ist IAL somit vor der ersten beobachtbaren
Post-MSTA-Probe gesetzt und MSTA bereits wieder gelöscht.

Es existiert weiterhin **keine bewiesene Root Cause** und **keine
funktionale Fehlerbehebung**. Insbesondere wurde noch keine Retry-, Delay-
oder Bus-Recovery-Maßnahme als Lösung übernommen.

Die Patches `0003` bis `0006` und `loglevel=7` bleiben
Diagnoseinstrumentierung und sind nicht als Produktionszustand anzusehen.

### Nächster Schritt

Nach Abschluss und Verifikation dieses 0006-Git-Checkpoints beginnt in
einem neuen Chat die nächste Root-Cause-Phase. Ausgangspunkt ist
ausschließlich der verifizierte 0006-Befund.

Ein weiterer Diagnose-Patch (`0007`) oder eine funktionale Änderung wird
erst festgelegt, nachdem aus dem 0006-Ergebnis ein konkreter nächster
Versuch abgeleitet wurde.

Vor jedem weiteren Kernel-, DTB- oder Testmedium-Eingriff sind die vorhandenen
Rollback-Artefakte und Backups erneut zu prüfen.

### Wiedereinstieg nach einer Pause oder in einem neuen Chat

Für den Wiedereinstieg gelten in dieser Reihenfolge als maßgebliche Quellen:

1. der aktuelle und auf beiden Remotes verifizierte Git-Commit,
2. diese Datei `docs/PROJECT-STATE.md`, insbesondere dieser Checkpoint,
3. `docs/TEST-LOG.md`,
4. SHA-256-verifizierte Build-, Backup- und Evidence-Artefakte.

Gesprächserinnerungen und Chat-Zusammenfassungen sind nur ergänzende Quellen.
Bei einem Widerspruch haben Git, committed Projektdokumentation und
verifizierte Artefakte Vorrang.

Beim Wiedereinstieg soll zunächst ausschließlich ein Read-only-Statuscheck
auf der foobox erfolgen. Erst danach werden neue Änderungen geplant.

## Checkpoint nach Block 2.5 – quellenübergreifende historische Novena-Sichtung – 2026-09-15

Dieser Abschnitt ergänzt den unmittelbar vorhergehenden 0006-Checkpoint.
Der technische Ausgangsbefund von 0006 bleibt unverändert maßgeblich.
Block 2.5 diente ausschließlich der quellenübergreifenden historischen
Einordnung und hat weder eine Root Cause bewiesen noch eine funktionale
Änderung am Kernel, Device Tree oder Testmedium eingeführt.

### Git- und Sicherungsstand

Der abgeschlossene Forschungsstand von Block 2.5 ist in folgendem Commit
gesichert:

`afdc25adb7e5c94408a339ebb6a43e26654e8268`

Commit-Betreff:

`Archive cross-source Novena research`

Parent:

`8c1c0f6418726aae75ec525bc4fa0eb2ef4411b3`

Der Commit enthält ausschließlich sechs Änderungen unter
`research/02-historical-software/`:

* das aktualisierte `SHA256SUMS`-Manifest,
* vier neue quellenübergreifende Primärquellen-Extrakte,
* die zentrale deutsche Analyse
  `notes/quellenuebergreifende-novena-sichtung.md`.

Commit-Statistik:

* 6 Dateien geändert
* 4408 Einfügungen
* keine Löschungen

Das aktualisierte Block-2-Manifest enthält 134 Einträge.

SHA-256 des Manifests:

`345d9f100fb5b86ea0d3ac8ab2071454035480e11993279f40513a0358e0e422`

Die fünf neuen Forschungsdateien wurden vor und nach dem Commit über ihre
SHA-256-Werte verifiziert. Die in drei Primärquellen-Extrakten enthaltenen
historischen Whitespace-Eigenschaften wurden bewusst quellgetreu erhalten;
die neu verfasste Analysenote und das Manifest sind `git diff --check`-sauber.

Vor dem Commit wurde zusätzlich eine bytegenaue Sicherung angelegt:

`/home/loomit/novena-backups/block-2.5-vor-commit-2026-09-15`

Der Commit wurde erfolgreich zu GitHub übertragen und dort anschließend per
`ls-remote` exakt als `afdc25adb7e5c94408a339ebb6a43e26654e8268`
verifiziert.

Auch der Push zu Codeberg war erfolgreich und meldete:

`8c1c0f6..afdc25a  main -> main`

mit Rückgabecode 0. Dadurch wurde der lokale Tracking-Ref `origin/main` auf
`afdc25adb7e5c94408a339ebb6a43e26654e8268` aktualisiert. Eine zusätzliche
unabhängige `ls-remote`-Verifikation von Codeberg war unmittelbar danach wegen
des bekannten intermittierenden SSH-Problems auf Port 22 nicht möglich; neue
SSH-Verbindungen wurden von Codeberg wiederholt geschlossen. Dies ist als
noch offene Remote-Leseverifikation zu behandeln, nicht als fehlgeschlagener
Push.

Zum Abschluss standen lokal:

* `HEAD = afdc25adb7e5c94408a339ebb6a43e26654e8268`
* `origin/main = afdc25adb7e5c94408a339ebb6a43e26654e8268`
* `github/main = afdc25adb7e5c94408a339ebb6a43e26654e8268`
* sauberer Worktree relativ zu `origin/main`

### Umfang und Quellenbasis von Block 2.5

Block 2.5 war eine quellenübergreifende Sichtung der bereits gesicherten
Novena-Hardware- und Softwarequellen. Ziel war nicht die Suche nach einem
passenden historischen Workaround, sondern die Prüfung, ob sich aus den
verschiedenen Entwicklungsständen belastbare Hinweise auf bislang übersehene
Hardwarezustände, Abhängigkeiten oder Initialisierungsreihenfolgen ergeben.

Einbezogen wurden insbesondere:

* die bereits archivierten originalen Novena-Hardwareunterlagen,
* historische Novena-U-Boot-Quellen,
* historische Linux-/Device-Tree-Stände aus `xobs/novena-linux` und
  `novena-next/linux`,
* die historischen `novena-next/nixos-novena`-Quellen,
* die gesicherte NixOS-Wiki-Revision,
* das historische IT6251-Werkzeug,
* sowie gezielt das von der historischen Dokumentation referenzierte
  `novena-next/docs`-Repository.

Die breite historische Quellensuche ist mit Block 2.5 abgeschlossen. Weitere
zufällige Repository-Suchen sollen vor der nächsten experimentellen Phase
nicht erfolgen.

### Historisch belastbare Befunde

Die quellenübergreifende Sichtung hat mehrere zuvor getrennte Befunde
zusammengeführt.

Erstens existiert ein direkter historischer Novena-Beleg für eine
Wechselwirkung zwischen der Audio-Versorgung `es8328-power` und I2C3. Der
Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

änderte 2020 die Versorgung des ES8328 von `regulator-boot-on` auf
`regulator-always-on`. Die Commit-Beschreibung hält ausdrücklich fest, dass
das Abschalten dieser Versorgung offenbar den I2C3-Bus beeinträchtigte und
unter anderem Bildschirm, EEPROM und Senoko störte. Dieser Befund ist ein
starker historischer Hinweis auf eine reale Audio-Power-/I2C3-Wechselwirkung
auf Novena-Hardware.

Zweitens zeigen die historischen IT6251-Implementierungen wiederholte
Robustheitsmaßnahmen beim Power-up und bei der Readiness-Erkennung. Im
historischen Linux-Treiber wurden Product-ID-Leseversuche mehrfach
nachgebessert. Der Commit

`fc52d5f71a01541572adf259b0cc174dc3df45ce`

mit dem Betreff `it6251: Attempt to make powerup more robust` reduzierte die
Versuchsanzahl und vergrößerte die Wartezeit nach fehlgeschlagenen
Product-ID-Leseversuchen auf 100000 bis 200000 Mikrosekunden. Dabei existiert
jedoch kein fester zusätzlicher Delay vor dem ersten I2C-Zugriff. Eine früher
auffällige Zahl `150000` stammte lediglich aus einer später entfernten
Debug-Ausgabe und ist kein festes Initialdelay.

Drittens enthält das historische Novena-U-Boot seit der Einführung der
proper-LVDS-Unterstützung durch

`331ae846ad9fee532cf04268da76701093e4e4ed`

eine explizite IT6251-Power-Sequenz. GPIO5_28 wird zunächst LOW gesetzt, nach
10 ms HIGH gesetzt und anschließend weitere 20 ms gewartet. Danach wird die
IT6251-Readiness über die bekannten Product-ID-Werte `0x15`, `0xca`, `0x51`
und `0x62` abgefragt, bevor Initialisierung und Backlight-Freigabe fortgesetzt
werden. Die U-Boot-Implementierung verweist dabei ausdrücklich auf den
historischen Sean-Cross-/xobs-Linux-Code.

Viertens änderte sich die Device-Tree-Power-Policy im historischen
Novena-Kernel mehrfach. Zwischen den untersuchten Ständen 4.4, 4.19 WIP3,
5.7-rc2 und 6.6 wechselten insbesondere `regulator-boot-on` und
`regulator-always-on` für Audio-, Display- und LVDS-Versorgungen sowie die
Display-Startup-Delays. Damit existiert keine einzelne historische
Power-Policy, die ohne weitere Prüfung als universelle Novena-Lösung
übernommen werden kann.

Zusätzlich zeigen ältere Display-Device-Tree-Änderungen, dass die
Displayversorgung in der Vergangenheit bewusst verändert wurde. Der Commit
`5f3c4528c7714e23b174c232af70fcaafecf2ba1` sollte die Displayversorgungen
beim Boot vollständig neu starten; der spätere Commit
`29f549a50d76fb73f88cbcdc4a4391c266f4f557` brachte `reg_display` beim Boot
wieder hoch. Auch diese Änderungen belegen eine historische Sensitivität der
Display-Power-Sequenz, ohne die aktuelle Ursache zu beweisen.

Die Hardwareunterlagen ergänzen diese Softwarebefunde. I2C3 verbindet unter
anderem ES8328, EEPROM, FPGA-/Boardpfade und den über den LCD-/eDP-Pfad
erreichbaren IT6251. Die eDP-Adapter-Unterlagen zeigen außerdem einen
separaten Reset-/Power-Kontext des IT6251 einschließlich APX803-Resetmonitor.
Produktionsänderungen und ECO-Unterlagen zeigen zugleich, dass nicht jede
reale Boardbestückung vollständig aus einem einzelnen Schaltplan- oder
BOM-Stand abgeleitet werden darf.

### Abgrenzung gegenüber dem aktuellen 0006-Fehler

Keiner der historischen Befunde beweist, dass der aktuelle Linux-6.18.49-
Fehler dieselbe Ursache besitzt.

Der maßgebliche aktuelle Messbefund bleibt unverändert:

`Cold FAIL: A=81/80 -> M0=93/80`

gegen:

`LDB-Rebind PASS: A=81/80 -> M0=81/a0`

Damit ist beim fehlgeschlagenen Cold Boot das IAL-Bit bereits in der ersten
beobachtbaren Post-MSTA-Probe gesetzt und MSTA bereits wieder gelöscht. Die
historischen Quellen dokumentieren Power-, Readiness- und I2C3-Abhängigkeiten,
aber keinen Nachweis dafür, dass genau eine dieser Abhängigkeiten den heutigen
`A=81/80 -> M0=93/80`-Übergang verursacht.

Insbesondere folgt aus Block 2.5 nicht, dass ein historisches
`regulator-always-on`, ein zusätzlicher IT6251-Delay, ein Retry, ein
Power-Cycle oder eine Bus-Recovery-Maßnahme als Lösung übernommen werden
sollte.

### Prioritäten für die nächste Root-Cause-Phase

Aus Block 2.5 ergibt sich eine Reihenfolge für die weitere Untersuchung,
keine Fix-Reihenfolge.

Priorität 1 ist die aktuelle ES8328-/Audio-Power-Wechselwirkung mit I2C3. Zu
klären ist, welchen realen Regulator-, GPIO- und Buszustand Linux 6.18.49 auf
der getesteten Novena vor dem ersten IT6251-START erzeugt und ob sich dieser
zwischen echtem POR-Cold-Boot und erfolgreichem Same-Boot-LDB-Rebind
unterscheidet.

Priorität 2 ist die aktuelle IT6251-/Display-Power- und Reset-Sequenz. Die
historische U-Boot-Sequenz und die Linux-Readiness-Retries sind hierbei
Vergleichsmaterial, dürfen aber nicht ungeprüft als Fix übernommen werden.

Priorität 3 ist der auf dem aktuellen Testmedium vorhandene U-Boot-2020.07-
Pfad. Zu klären ist, welche Display-, Audio-, I2C3-, GPIO- und Power-Zustände
dieser U-Boot-Stand vor Übergabe an Linux tatsächlich hinterlässt.

Priorität 4 bleibt der Linux-6.18.49-Controllerzustand selbst, insbesondere
Pinctrl, Clock, Runtime-PM und die Controller-Lifecycle-Unterschiede zwischen
Cold Boot und Same-Boot-Rebind. Der identische sichtbare Pre-MSTA-Snapshot A
beweist nicht, dass alle versteckten Controller-, Pad- oder physischen
Buszustände identisch sind.

Priorität 5 ist erst danach ein kontrollierter Vergleich mit historischen
Kernelständen beziehungsweise relevanten Implementierungsunterschieden, wenn
dies zur Falsifikation einer konkreten Hypothese erforderlich wird.

### Noch nicht beschlossene Maßnahmen

Block 2.5 führt zu keiner neuen dauerhaften Projektentscheidung in
`docs/DECISIONS.md`. Die dort dokumentierte Patch- und Diagnosepolitik bleibt
unverändert.

Insbesondere ist weiterhin nicht beschlossen:

* ein Patch `0007`,
* ein Retry des fehlgeschlagenen STARTs,
* ein zusätzlicher pauschaler Delay,
* eine I2C-Bus-Recovery als Fehlerbehebung,
* ein erzwungener IT6251-Power-Cycle,
* `regulator-always-on` als neue Display- oder Audio-Policy,
* oder die Übernahme eines historischen Device-Tree-Zustands.

Eine solche Änderung darf erst aus einem konkreten, falsifizierbaren Test
abgeleitet werden.

### Aktueller physischer Zustand

Während Block 2.5 wurden keine weiteren Hardwaretests durchgeführt. Die
Novena blieb ausgeschaltet. Das externe Testmedium bleibt im dokumentierten
0006-Zustand; vor jedem erneuten Zugriff auf der foobox muss seine aktuelle
Gerätebezeichnung neu ermittelt werden.

Die vorhandenen 0006-Evidence-, Rollback- und Backup-Artefakte bleiben die
Ausgangsbasis für die nächste experimentelle Phase.

### Wiedereinstieg in Block 3

Block 2.5 ist als historische Quellenphase abgeschlossen. Die nächste Phase
beginnt nicht mit einem neuen Patch, sondern mit einem Read-only-Checkpoint
auf der foobox und der Ableitung eines einzelnen falsifizierbaren Tests aus
den oben priorisierten Hypothesen.

Für den Wiedereinstieg gelten als maßgeblich:

1. der Git-Checkpoint
   `afdc25adb7e5c94408a339ebb6a43e26654e8268` für die abgeschlossene
   Block-2.5-Forschung,
2. der unmittelbar vorhergehende 0006-Checkpoint in dieser Datei,
3. die zentrale Forschungsanalyse
   `research/02-historical-software/notes/quellenuebergreifende-novena-sichtung.md`,
4. `docs/TEST-LOG.md`,
5. die SHA-256-verifizierten 0006-Evidence-, Build-, Backup- und
   Rollback-Artefakte.

Die noch ausstehende unabhängige Codeberg-`ls-remote`-Verifikation kann bei
einer später wieder stabilen SSH-Verbindung nachgeholt werden. Sie ändert
nichts am erfolgreich protokollierten Codeberg-Push und darf nicht mit einer
technischen Unsicherheit des Novena-Tests vermischt werden.

Vor Block 3 wird kein Patch `0007` angelegt.

## Checkpoint Block 3 – H3-1 ES8328-/Audio-Power und I2C3 – 2026-09-15

### Ausgangspunkt

Block 3 begann auf dem verifizierten Git-Stand
`1120875370557ea79a0c9499d83caa167af901dd` und ohne Patch `0007`.
Ausgangspunkt war der mit Patch `0006-i2c-imx-debug-start-transition.patch`
beobachtete Unterschied unmittelbar nach dem Setzen von MSTA:

`Cold FAIL: A=81/80 -> M0=93/80`

gegenüber

`Same-Boot-LDB-Rebind PASS: A=81/80 -> M0=81/a0`.

Priorität 1 aus Block 2.5 war die historische ES8328-/Audio-Power-
Wechselwirkung mit dem gemeinsam genutzten I2C3.

### H3-1 – Audio-Power-State-Hypothese

Als einzelner erster Test wurde H3-1 formuliert: Der Zustand von
`es8328-power` beziehungsweise des zugehörigen Audio-Power-/GPIO-Pfads
unterscheidet sich im für den ersten IT6251-START relevanten Zeitraum zwischen
einem fehlerhaften POR-Cold-Boot und dem erfolgreichen Same-Boot-LDB-Rebind
und beeinflusst dadurch möglicherweise den gemeinsam genutzten I2C3.

Die Hardware- und historische Quellenlage macht diese Hypothese konkret:
ES8328 und IT6251 teilen sich I2C3. Der ES8328-Zweig ist über Serienwiderstände
an I2C3 gekoppelt und besitzt eine zusätzliche Pull-up-/Versorgungsabhängigkeit
vom Audio-Power-Bereich. Historisch wurde die Device-Tree-Policy für
`es8328-power` von `regulator-boot-on` auf `regulator-always-on` geändert,
weil das Abschalten der ES8328-Versorgung auf realer Novena-Hardware den
I2C3-Bus beeinträchtigte.

Dieser historische Befund ist ein Hinweis und kein Beweis für die aktuelle
Root Cause.

### Block 3.5 – kontrollierter POR-FAIL und Same-Boot-Rebind

Ein echter POR-Cold-Boot reproduzierte den bekannten Fehler. Der erste
IT6251-Zugriff zeigte:

`A=81/80 B=93/80 C=83/80 ret=-11`

und Patch `0006` erfasste unmittelbar nach dem MSTA-Schreibzugriff:

`M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80`.

Alle fünf Product-ID-Versuche schlugen mit derselben Signatur fehl. Der
IT6251-Treiber deaktivierte danach seinen Display-Regulator und der interne
Bildschirm blieb aus.

Ohne Reboot wurde anschließend ausschließlich der bereits etablierte
Same-Boot-LDB-Unbind/Bind-Vergleich durchgeführt. Danach wechselte der
IT6251-Zugriff unmittelbar auf den erfolgreichen Zustand:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

und

`M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0`.

Der IT6251 konnte anschließend initialisiert werden und der Display-Link wurde
stabil.

Damit enthält derselbe Boot sowohl den reproduzierten Cold-FAIL als auch den
späteren Same-Boot-PASS.

### Post-Boot-Beobachtung des Audio-Power-Pfads

Vor beziehungsweise nach dem erfolgreichen LDB-Rebind war im laufenden
System folgender sichtbarer Zustand feststellbar:

* `es8328-power`: 5000 mV,
* zugehöriger Audio-Regulator-GPIO: `out hi`,
* ES8328 als I2C-Gerät `2-0011` vorhanden.

Dieser identische spätere Softwarezustand reicht jedoch nicht zur
Falsifikation einer transienten Audio-Power-Hypothese aus.

Die nachfolgende Auswertung des vollständigen, gesicherten Kernel-Logs zeigte
nämlich einen zuvor nicht berücksichtigten Vorgang während des Cold-Boots.

### Block 3.6 – zeitliche Rekonstruktion aus der gesicherten Evidence

Im vollständigen `dmesg` des Cold-FAIL-/Same-Boot-PASS-Laufs befindet sich
genau eine explizite Meldung zu `es8328-power`:

`[   33.761742] es8328-power: disabling`

Der aktuell gebootete Live-Device-Tree enthält für
`/regulator-audio-codec` weiterhin die Property `regulator-boot-on`.

Für `es8328-power` existiert im gespeicherten Kernel-Log keine spätere
explizite Enable-Meldung.

Der relevante Ablauf des Cold-FAIL ist:

1. bei 33.761742 s: `es8328-power: disabling`,
2. bei 42.395351 s: der IT6251-Treiber beginnt das Einschalten seines
   eigenen Display-Power-Regulators,
3. bei 43.172998 s: `imx-es8328 sound: Unable to register: -517`,
4. bei 44.471549 s: der IT6251-Display-Regulator ist eingeschaltet,
5. bei 44.471609 s: erster Product-ID-Versuch,
6. bei 44.478167 s: I2C3 meldet `arbitration lost`, I2SR `0x93`,
7. bei 44.478235 s: der IT6251-START schlägt mit
   `A=81/80 B=93/80 C=83/80 ret=-11` fehl,
8. bei 44.478258 s: alle acht unmittelbaren Post-MSTA-Samples sind
   `93/80`.

Zwischen der expliziten Meldung `es8328-power: disabling` und dem ersten
beobachteten I2C3-Fehler liegen damit ungefähr 10,716 Sekunden.

Später initialisiert sich der Audio-Pfad weiter:

* 46.079553 s: `imx-es8328 sound` erreicht die ASoC-Registrierung,
* 46.194765 s: das ES8328-Headphone-Input-Gerät wird registriert.

Der erfolgreiche Same-Boot-IT6251-Versuch erfolgt wesentlich später bei
ungefähr 178,39 s und zeigt die bekannte erfolgreiche START-Transition
`M0=81/a0`.

### Bewertung von H3-1

Die ursprüngliche Zwischenbewertung, H3-1 sei aufgrund identischer
Post-Boot-Regulator-/GPIO-Zustände falsifiziert, wird durch die vollständige
Zeitreihenanalyse eingeschränkt.

Gesichert ist nun:

* Der sichtbare Post-Boot-Zustand des Audio-Power-Pfads ist beim späteren
  Vergleich aktiv.
* Während des vorausgehenden Cold-Boots schaltet Linux `es8328-power`
  nachweislich bei 33.761742 s ab.
* Der erste fehlerhafte IT6251-START folgt rund 10,7 Sekunden später.
* Das Log enthält keine explizite spätere Enable-Meldung für
  `es8328-power`.
* Diese zeitliche Korrelation beweist keine Kausalität.
* Sie verhindert aber, dass die transiente ES8328-/Audio-Power-Hypothese
  allein anhand des späteren Post-Boot-Zustands verworfen wird.

Die historische ES8328-/I2C3-Spur bleibt deshalb als konkrete
Root-Cause-Hypothese aktiv.

### Nächster einzelner falsifizierbarer Test – H3-1R

Als nächster Test ist ausschließlich H3-1R vorgesehen:

Wenn das automatische Abschalten von `es8328-power` während des Cold-Boots
eine notwendige Voraussetzung für die spätere I2C3-Fehlersignatur
`A=81/80 -> M0=93/80` schafft, dann muss ein ansonsten vergleichbarer
POR-Cold-Boot, bei dem ausschließlich dieses automatische Abschalten
verhindert wird, die Fehlersignatur reproduzierbar verändern oder beseitigen.

Die Interpretation ist vorab festgelegt:

* Bleibt bei nachweislich nicht abgeschaltetem `es8328-power` die identische
  Cold-Boot-Signatur `M0=93/80` mit `ret=-11` bestehen, spricht dies gegen
  H3-1R.
* Wechselt der Cold-Boot reproduzierbar auf die erfolgreiche
  `M0=81/a0`-Transition beziehungsweise einen erfolgreichen IT6251-Zugriff,
  stützt dies einen kausalen Zusammenhang.
* Ein einzelner erfolgreicher POR-Boot reicht nicht als Bestätigung, da auch
  mit dem bisherigen Zustand bereits spontane POR-PASS-Läufe beobachtet
  wurden.

H3-1R ist ein experimenteller Root-Cause-Test und noch keine dauerhafte
Systementscheidung.

Insbesondere ist `regulator-always-on` zu diesem Zeitpunkt nicht als Fix oder
neue Policy beschlossen.

### Evidence

Die Rohdaten dieses Laufs befinden sich im Repository unter:

`evidence/block-3.5-h3-1-2026-09-15/`

Enthalten sind unter anderem vollständiges `dmesg`, der IT6251-/I2C3-Auszug,
Regulator- und GPIO-Zustände, Live-Device-Tree-Daten, I2C3-Geräte und
LDB-Binding.

Alle Dateien wurden über `SHA256SUMS` erfolgreich verifiziert.

Zusätzlich wurde vor der Dokumentationsänderung eine unabhängige bytegleiche
Sicherung angelegt:

`~/novena-backups/block-3.5-h3-1-2026-09-15/`

`diff -qr` bestätigte:

`EVIDENCE_BACKUP=IDENTISCH`

Damit existieren vor weiteren experimentellen Änderungen zwei voneinander
getrennte, verifizierte Kopien der Block-3.5-Rohdaten.

### Änderungsgrenzen nach diesem Checkpoint

Bis zur ausdrücklichen Vorbereitung von H3-1R gilt weiterhin:

* kein Patch `0007`,
* kein Retry als Fehlerbehebung,
* kein zusätzlicher pauschaler IT6251-Delay,
* keine I2C-Bus-Recovery als Fix,
* kein manueller Audio-GPIO-Eingriff,
* kein erzwungener IT6251-Power-Cycle,
* kein dauerhaft beschlossenes `regulator-always-on`.

Die nächste funktionale Änderung darf ausschließlich der kontrollierten
Falsifikation von H3-1R dienen.


## Block 3.8–3.10 – H3-1R: kontrollierter ES8328-Power-Test

### Ziel und Hypothese

Aus Block 3.5 und Block 3.6 ergab sich als einzelne falsifizierbare
Folgehypothese H3-1R:

Wenn das automatische Abschalten von `es8328-power` während des Cold-Boots
eine notwendige Voraussetzung oder einen kausalen Beitrag zur späteren
I2C3-Fehlersignatur

`A=81/80 -> M0=93/80`

liefert, dann muss ein ansonsten unveränderter POR-Cold-Boot, bei dem
ausschließlich dieses automatische Abschalten verhindert wird, die
Fehlersignatur reproduzierbar verändern oder beseitigen.

Die Änderung wurde ausdrücklich als experimentelle Intervention und nicht
vorab als dauerhafter Fix behandelt.

### Block 3.8 – Device-Tree-Pfad und Zielknoten

Vor der funktionalen Änderung wurde die vollständige Device-Tree-Kette
read-only nachvollzogen.

Der von der aktuellen NixOS-Konfiguration verwendete Novena-DTB entsteht aus:

1. den vom Linux-6.18.49-Kernel gebauten DTBs,
2. den in `hardware/novena.nix` definierten Device-Tree-Overlays,
3. dem daraus erzeugten `device-tree-overlays`-Paket,
4. `config.hardware.deviceTree.package`,
5. `system.build.novenaDtb`,
6. der Kopie als `novena.dtb` in das FIRMWARE-Dateisystem des SD-Images.

Der Basis-DTB und der vor H3-1R verwendete finale DTB enthielten für
`reg_audio_codec`:

`regulator-name = "es8328-power"`

und:

`regulator-boot-on`

aber kein:

`regulator-always-on`.

Der Symbolbereich des DTB enthält:

`reg_audio_codec = "/regulator-audio-codec"`

Damit ist `&reg_audio_codec` ein gültiges Overlay-Ziel.

Zusätzlich wurde anhand der bereits vorhandenen Änderung von
`reg_display/startup-delay-us` nachgewiesen, dass die Overlays aus
`hardware/novena.nix` tatsächlich in den final verwendeten DTB eingehen.

### Block 3.9 – einzelne experimentelle Intervention

Für H3-1R wurde in `hardware/novena.nix` genau ein zusätzliches
Device-Tree-Overlay angelegt.

Dieses ergänzt am bestehenden Knoten `&reg_audio_codec` ausschließlich:

`regulator-always-on;`

Unverändert blieben insbesondere:

* `regulator-boot-on`,
* Regulatorname `es8328-power`,
* GPIO-Zuordnung,
* Active-High-Polarität,
* 5-V-Spannung,
* 400-ms-Startup-Delay,
* I2C3-Konfiguration,
* IT6251-Konfiguration,
* Display-Power-Timing,
* Kernel 6.18.49,
* Kernel-Patches `0001` bis `0006`.

Es wurde kein Patch `0007` angelegt.

### Semantische DTB-Verifikation

Vor dem Hardwaretest wurden Baseline-DTB und H3-1R-DTB dekompiliert und
vollständig miteinander verglichen.

Baseline-DTB:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

H3-1R-DTB:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

Der vollständige DTS-Vergleich ergab genau eine semantische Ergänzung:

`regulator-always-on;`

am Knoten:

`reg_audio_codec: regulator-audio-codec`

Damit war die unabhängige Variable vor dem ersten H3-1R-Hardwaretest
eindeutig auf diese eine Device-Tree-Property begrenzt.

Die zugehörigen Testartefakte wurden zusätzlich unter

`~/novena-backups/h3-1r-test-2026-09-15/`

gesichert und per SHA-256 verifiziert.

### Aktivierung auf dem Testmedium

Das externe Testmedium wurde vor der Änderung read-only geprüft.

Der aktive `novena.dtb` auf dem Medium hatte den Baseline-Hash:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

und war damit byteidentisch zum zuvor untersuchten Baseline-DTB.

`boot.cmd` lädt ausdrücklich `novena.dtb`.

Vor der Änderung wurde der aktive Medium-DTB zusätzlich unter

`~/novena-backups/h3-1r-test-2026-09-15/novena.dtb.from-medium-before-h3-1r`

gesichert.

Danach wurde ausschließlich der aktive `novena.dtb` durch den bereits
verifizierten H3-1R-DTB ersetzt.

Nach erneutem read-only Mount hatte der persistierte Medium-DTB den
erwarteten H3-1R-Hash:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

Die übrigen geprüften Bootdateien behielten ihre vorherigen Hashes.

### Vorregistriertes Testprotokoll

Vor Beginn der H3-1R-Serie wurde folgendes Protokoll festgelegt:

* fünf echte POR-Cold-Boots,
* vor jedem Versuch vollständiges Herunterfahren,
* physisches Trennen der Stromversorgung,
* mindestens 30 Sekunden vollständig ohne Strom,
* unveränderte externe H3-1R-Test-SD,
* kein `reboot`,
* kein LDB-Unbind/Bind,
* keine manuellen I2C-Zugriffe vor der Beweissicherung,
* keine funktionale Änderung zwischen den fünf Versuchen.

Primäre Messgröße war der erste IT6251-START nach POR.

Die bekannte Cold-FAIL-Signatur war:

`A=81/80 -> M0=93/80`

mit `ret=-11` beziehungsweise `arbitration lost`.

Die bekannte erfolgreiche START-Signatur war:

`A=81/80 -> M0=81/a0`

mit anschließend normalem I2C-Transfer.

Als sekundäre Interventionskontrolle durfte bei aktivem
`regulator-always-on` keine automatische Meldung

`es8328-power: disabling`

mehr auftreten.

Ein einzelner erfolgreicher POR-Boot war ausdrücklich nicht als
Bestätigung ausreichend, da unter der Baseline bereits spontane
POR-PASS-Läufe beobachtet worden waren.

### Ergebnis der fünf vorregistrierten POR-Cold-Boots

Alle fünf H3-1R-Versuche wurden vollständig durchgeführt.

| Versuch | always-on | Power-Abschaltung | erster M0 | `93/80` | Arbitration lost | LVDS |
| --- | --- | --- | --- | --- | --- | --- |
| H3-1R/01 | ja | nein | `81/a0` | nein | nein | connected |
| H3-1R/02 | ja | nein | `81/a0` | nein | nein | connected |
| H3-1R/03 | ja | nein | `81/a0` | nein | nein | connected |
| H3-1R/04 | ja | nein | `81/a0` | nein | nein | connected |
| H3-1R/05 | ja | nein | `81/a0` | nein | nein | connected |

Ergebnis der vorregistrierten Serie:

`5/5 POR-Cold-Boots = PASS`

Bei allen fünf Boots bestätigte der Live-Device-Tree:

`regulator-boot-on=PRESENT`

und:

`regulator-always-on=PRESENT`.

Der Audio-Regulator-GPIO war bei der späteren Beweissicherung in allen fünf
Läufen `out hi`.

`es8328-power` wurde jeweils mit 5000 mV angezeigt.

Die vollständigen Kernel-Logs aller fünf Läufe wurden nach Abschluss der
Serie gemeinsam durchsucht.

In keinem der fünf H3-1R-Läufe findet sich:

* `es8328-power: disabling`,
* die bekannte `93/80`-Fehlersignatur,
* `arbitration lost`,
* ein IT6251-Product-ID-Fehler.

Damit handelt es sich nicht lediglich um fünf optisch erfolgreiche
Display-Boots. Die bekannte instrumentierte I2C3-Fehlerklasse blieb in allen
fünf vollständigen Kernel-Logs aus.

### Vergleich mit dem gesicherten Baseline-Cold-FAIL

Die Baseline-Evidence aus Block 3.5 wurde vor dem Vergleich erneut vollständig
per SHA-256 verifiziert.

Im Baseline-Cold-FAIL befindet sich:

`[   33.761742] es8328-power: disabling`

Der erste bekannte I2C3-Fehler folgt bei:

`[   44.478167] ... arbitration lost in bus_busy, I2SR=0x93`

unmittelbar gefolgt von:

`[   44.478235] ... IT6251 START failure A=81/80 B=93/80 C=83/80 ret=-11`

Patch `0006` zeigt für denselben START:

`M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80`

Der Baseline-Live-Device-Tree enthält `regulator-boot-on`, aber kein
nachgewiesenes `regulator-always-on`.

Zwischen der expliziten Audio-Regulator-Abschaltung und dem ersten
beobachteten I2C3-Fehler liegen ungefähr 10,716 Sekunden.

Demgegenüber gilt für alle fünf H3-1R-Cold-Boots:

* `regulator-always-on` ist im Live-DT vorhanden,
* keine automatische `es8328-power`-Abschaltung wird protokolliert,
* der erste IT6251-START beginnt erfolgreich mit `M0=81/a0`,
* die `93/80`-Signatur tritt im vollständigen Kernel-Log nicht auf,
* `arbitration lost` tritt nicht auf,
* der IT6251 initialisiert erfolgreich,
* LVDS ist verbunden und der interne Bildschirm funktioniert.

### Historische Übereinstimmung

Die aktuelle experimentelle Beobachtung stimmt mit einer unabhängig
gefundenen historischen Novena-Änderung überein.

Commit:

`e48619edadbde342d79655e73654f0b21fc5e20b`

Betreff:

`ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

Die historische Änderung ersetzte für `es8328-power` die bisherige
`regulator-boot-on`-Policy durch `regulator-always-on`.

Die zugehörige historische Beschreibung dokumentiert, dass das Abschalten
der ES8328-Versorgung auf realer Novena-Hardware den I2C3-Bus beeinträchtigte
und dadurch unter anderem Display-, EEPROM- und Senoko-Kommunikation gestört
wurde.

Diese historische Beobachtung entstand unabhängig von der aktuellen
Linux-6.18.49-/IT6251-Untersuchung.

### Bewertung von H3-1R

H3-1R wird durch die vorregistrierte Versuchsserie deutlich gestützt.

Die aktuelle Evidenz besteht aus drei zusammenpassenden Ebenen:

1. Im instrumentierten Baseline-Cold-FAIL wird `es8328-power` abgeschaltet.
   Später tritt die charakteristische I2C3-START-Fehlersignatur `M0=93/80`
   mit `arbitration lost` auf.
2. Bei einer kontrollierten Intervention, die ausschließlich
   `regulator-always-on` für denselben Audio-Power-Regulator ergänzt, bleiben
   in fünf von fünf echten POR-Cold-Boots sowohl die Audio-Abschaltung als
   auch die gesamte bekannte I2C3-Fehlerklasse aus.
3. Historische Novena-Entwicklung dokumentiert unabhängig davon dieselbe
   Wechselwirkung zwischen dem Abschalten von `es8328-power` und einem
   gestörten I2C3-Bus.

Damit besteht starke experimentelle Evidenz dafür, dass das automatische
Abschalten des ES8328-Power-Domains einen kausalen Beitrag zur untersuchten
Novena-I2C3-Cold-Boot-Fehlerklasse leistet.

Die frühere Annahme, der Audio-Power-Pfad könne aufgrund identischer späterer
Post-Boot-Regulatorzustände ausgeschlossen werden, ist damit nicht haltbar.

### Noch nicht bewiesen

Die Versuchsserie bestimmt noch nicht den exakten elektrischen Mechanismus,
durch den das Abschalten der ES8328-Versorgung I2C3 beeinflusst.

Insbesondere ist noch nicht entschieden, ob die Ursache beispielsweise in:

* einem Pegel- oder Clamp-Effekt am unversorgten ES8328,
* einer Rückspeisung über die I2C-Leitungen,
* der zusätzlichen codec-seitigen SDA-Pull-up-/Versorgungsstruktur,
* einem transienten Zustand beim Abschalten oder späteren Einschalten,
* oder einem anderen elektrischen Effekt des Audio-Power-Domains

liegt.

Diese Mechanismen dürfen ohne zusätzliche elektrische Messungen oder
gezielte weitere Tests nicht als bewiesen bezeichnet werden.

Ebenso ist `5/5 PASS` keine Garantie dafür, dass unter allen denkbaren
Startbedingungen niemals wieder ein Fehler auftreten kann.

### Status von `regulator-always-on`

`regulator-always-on` hat den vorregistrierten H3-1R-Test erfolgreich
bestanden und entspricht zusätzlich der historischen Novena-Lösung für die
ES8328-/I2C3-Wechselwirkung.

Zum Zeitpunkt dieses Checkpoints wird die Änderung dennoch weiterhin als
validierte experimentelle Intervention geführt.

Die Entscheidung, sie als dauerhafte Novena-Konfiguration zu übernehmen,
erfolgt ausdrücklich erst nach Sicherung und Review dieses Checkpoints.

Es wurde weiterhin kein Kernel-Patch `0007` angelegt.

### Evidence der H3-1R-Serie

Die fünf vollständigen POR-Testläufe befinden sich im Repository unter:

`evidence/block-3.10-h3-1r-por-series-2026-09-15/`

mit:

* `boot-01/`
* `boot-02/`
* `boot-03/`
* `boot-04/`
* `boot-05/`

Jeder Lauf enthält unter anderem vollständiges `dmesg`, IT6251-START-Auszug,
Audio-Power-Auszug, kritisches Zeitfenster, Regulator- und GPIO-Zustand,
I2C3-Geräte, Live-Device-Tree-Zustand, DRM-Displaystatus, Testkennung und
`SHA256SUMS`.

Alle fünf Repository-Kopien wurden in Block 3.10C-R einzeln in ihren
tatsächlichen Zielverzeichnissen mit

`sha256sum -c SHA256SUMS`

geprüft.

Für jeden Lauf ergab die Prüfung:

`SHA256_EXIT=0`

Zusätzlich wurden Quelle und Repository-Kopie für jeden Boot byteweise
verglichen:

* `BOOT-01: IDENTISCH`
* `BOOT-02: IDENTISCH`
* `BOOT-03: IDENTISCH`
* `BOOT-04: IDENTISCH`
* `BOOT-05: IDENTISCH`

Quelle und Ziel enthalten für jeden Lauf jeweils 15 Dateien.

Die unabhängigen Ausgangskopien bleiben zusätzlich unter

`~/novena-backups/h3-1r-trials-2026-09-15/`

erhalten.

### Korrektur der ersten Block-3.10C-Verifikation

Die erste Ziel-Hash-Prüfung in Block 3.10C verwendete nach dem ersten
Verzeichniswechsel einen relativen Zielpfad.

Dadurch schlug `cd` für `boot-02` bis `boot-05` fehl und die anschließend
ausgegebenen `sha256sum`-Ergebnisse stammten weiterhin aus dem vorherigen
Verzeichnis.

Diese Ausgaben werden ausdrücklich nicht als gültige Zielverifikation
gewertet.

Block 3.10C-R wiederholte die Prüfung mit absoluten Pfaden und bestätigte für
alle fünf tatsächlichen Zielverzeichnisse:

`SHA256_EXIT=0`

Damit ist die Repository-Evidence nach der korrigierten Prüfung vollständig
verifiziert.

### Änderungsgrenzen nach Block 3.10

Bis zur ausdrücklichen Entscheidung über die dauerhafte Übernahme gilt:

* kein Patch `0007`,
* keine zusätzliche Kerneländerung,
* kein Retry als Fix,
* keine zusätzliche I2C-Bus-Recovery,
* kein zusätzlicher pauschaler Display-Delay,
* keine weitere Veränderung der H3-1R-Test-SD,
* keine weitere funktionale Änderung an `hardware/novena.nix`.

Der aktuelle H3-1R-Stand soll zunächst unverändert gesichert, dokumentiert
und als eigener Git-Checkpoint geprüft werden.

## Block 3.11 – Dauerhafte Übernahme von `es8328-power` als `regulator-always-on`

Nach Abschluss und Verifikation der H3-1R-Versuchsserie wurde über den
Status der getesteten Device-Tree-Änderung ausdrücklich entschieden.

### Dauerhafte Board-Entscheidung

Für Novena wird der bestehende Regulator `es8328-power` dauerhaft mit
`regulator-always-on` konfiguriert.

Die Änderung wird damit nicht mehr nur als experimentelle
H3-1R-Intervention geführt, sondern als boardspezifische
Novena-Hardwarekonfiguration übernommen.

Die formale Projektentscheidung ist zusätzlich in `docs/DECISIONS.md`
unter `2026-09-15 – ES8328-Versorgung bleibt dauerhaft eingeschaltet`
dokumentiert.

### Grundlage der Entscheidung

Die Entscheidung beruht auf drei voneinander unterscheidbaren
Evidenzsträngen:

1. Im gesicherten Baseline-Cold-FAIL wurde `es8328-power` während des
   Bootvorgangs abgeschaltet. Später folgte beim ersten relevanten
   IT6251-START die bekannte Fehlersignatur `A=81/80 -> M0=93/80`
   mit `arbitration lost`.

2. H3-1R änderte gegenüber dem Baseline-Device-Tree semantisch
   ausschließlich den bestehenden Audio-Power-Regulator durch Ergänzung
   von `regulator-always-on`.

   Unter dieser Intervention bestanden fünf von fünf vorregistrierten
   echten POR-Cold-Boots.

   In allen fünf vollständigen Kernel-Logs galt:

   * keine Abschaltung von `es8328-power`,
   * keine `93/80`-Fehlersignatur,
   * kein `arbitration lost`,
   * erfolgreicher erster IT6251-START mit `M0=81/a0`,
   * Display aktiv.

3. Die unabhängige historische Novena-Änderung
   `e48619edadbde342d79655e73654f0b21fc5e20b` dokumentiert ebenfalls,
   dass das Abschalten von `es8328-power` I2C3 auf realer
   Novena-Hardware beeinträchtigt, und verwendet als Abhilfe ebenfalls
   `regulator-always-on`.

Damit besteht eine Kombination aus kontrollierter aktueller
Versuchsevidenz und unabhängigem historischem Hardwarebefund.

### Umbenennung des Device-Tree-Overlays

Nach der dauerhaften Entscheidung wurde ausschließlich der bisherige
experimentelle Overlay-Name `novena-h3-1r-es8328-always-on` in den
neutralen dauerhaften Namen `novena-es8328-power-always-on` geändert.

Der `dtsText` des Overlays wurde dabei nicht verändert.

Die funktionale Device-Tree-Änderung bleibt ausschließlich
`regulator-always-on` am bestehenden Knoten `reg_audio_codec`.

### Reproduzierbarkeitsprüfung nach der Umbenennung

Nach der reinen Overlay-Umbenennung wurde der projektdefinierte
`system.build.novenaDtb` erneut gebaut.

Finaler Store-Pfad:

`/nix/store/cl4dh5ak90rrsh399fqdr2rjvmnmy6rn-imx6q-novena.dtb`

SHA-256 des neu gebauten finalen DTB:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

Der bereits in den fünf erfolgreichen H3-1R-POR-Tests verwendete und
gesicherte DTB

`~/novena-backups/h3-1r-test-2026-09-15/imx6q-novena-h3-1r.dtb`

hat exakt denselben SHA-256:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

Der direkte Vergleich ergab:

`DTB_BYTE_COMPARE=IDENTISCH`

Zusätzlich wurden beide DTBs mit `dtc 1.8.1` dekompiliert.

Beide dekompilierten DTS-Dateien haben SHA-256:

`0648dbd8a3379beca016220df777c1969b005b169521f618628c9142b6b7da9a`

Der Vergleich ergab:

`DTS_COMPARE=IDENTISCH`

Beide dekompilierten Device Trees enthalten 18 Vorkommen von
`regulator-always-on`.

Damit erzeugt die dauerhaft benannte Konfiguration bytegenau denselben
Device Tree wie die bereits in fünf POR-Cold-Boots getestete
H3-1R-Konfiguration.

Ein zusätzlicher Hardwaretest allein aufgrund der Umbenennung ist daher
nicht erforderlich.

### Aussagegrenze

Die dauerhafte Übernahme von `regulator-always-on` bedeutet nicht, dass
der exakte elektrische Mechanismus der ES8328-/I2C3-Wechselwirkung
bestimmt wurde.

Insbesondere bleiben mögliche Clamp-, Rückspeisungs-, Pull-up- oder
andere transiente elektrische Effekte ohne zusätzliche elektrische
Messungen offen.

Die Entscheidung lautet daher nicht, dass der vollständige elektrische
Root Cause bewiesen sei.

Entschieden ist, dass das automatische Abschalten der ES8328-Versorgung
unter den untersuchten Bedingungen einen kausalen Beitrag zur
beobachteten I2C3-Cold-Boot-Fehlerklasse leistet und dass das Verhindern
dieser Abschaltung eine ausreichend stark validierte boardspezifische
Konfiguration darstellt.

### Konsequenzen für den weiteren Projektstand

Ab Block 3.11 gilt:

* `regulator-always-on` für `es8328-power` ist dauerhafte
  Novena-Board-Konfiguration.
* Der dauerhafte Overlay-Name lautet `novena-es8328-power-always-on`.
* Die H3-1R-Evidence bleibt unverändert erhalten.
* Die H3-1R-Test-SD wird für diese reine Umbenennung nicht erneut
  verändert oder getestet.
* Es wird kein Kernel-Patch `0007` für diese Lösung eingeführt.
* Die Diagnose-Patches `0003` bis `0006` bleiben weiterhin getrennt von
  dieser Device-Tree-Entscheidung zu bewerten.

## Block 3.12 – Root-Cause-Kandidatenmatrix nach H3-1R – 2026-09-15

### Ausgangspunkt

Block 3.12 wurde auf dem gesicherten Ausgangscheckpoint

`be1dabc653ea9937ba05a546c0de1a8c2143e888`

durchgeführt.

Die in Block 3.11 dauerhaft getroffene Board-Entscheidung bleibt unverändert:

`es8328-power = regulator-always-on`

Diese Einstellung wird nicht mehr als experimentelle H3-1R-Intervention
behandelt. Gegenstand der weiteren Root-Cause-Untersuchung ist der noch
ungeklärte elektrische Mechanismus der ES8328-/I2C3-Wechselwirkung.

### Grundlage der Synthese

Vor der Kandidatenbildung wurden die drei bisherigen Untersuchungsbereiche
zusammengeführt:

1. originale Novena-Dokumentation und Schaltplanunterlagen,
2. historische Novena-, Kosagi-, U-Boot- und Kernelquellen,
3. die vorhandenen Kernel-, Device-Tree- und Hardwaretests einschließlich
   der instrumentierten Linux-6.18.49-Cold-Boot-/Rebind-Evidence.

Die Kernelvergleichs-Evidence liegt nicht in einem separaten historischen
`research/03-*`-Block. Die relevanten realen Testergebnisse befinden sich
vor allem in `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md` und der gesicherten
Evidence. Die Quellcodevergleiche von Linux 5.7 und 6.18 sind zusätzlich in
Block 2 dokumentiert.

Die Synthese bestätigt insbesondere:

- Der gesicherte Baseline-Cold-FAIL zeigt
  `A=81/80 -> M0=93/80` mit frischem IAL und `ret=-11`.
- Der erfolgreiche Same-Boot-LDB-Rebind zeigt
  `A=81/80 -> M0=81/a0`.
- Im Baseline-Cold-FAIL wird `es8328-power` ungefähr 10,716 Sekunden vor
  dem ersten beobachteten I2C3-Fehler abgeschaltet.
- H3-1R verhindert ausschließlich diese automatische Abschaltung durch
  `regulator-always-on` und erreicht 5/5 erfolgreiche echte POR-Cold-Boots.
- In diesen fünf vollständigen Kernel-Logs fehlen die bekannte `93/80`-
  Signatur und das untersuchte I2C3-`arbitration lost` vollständig.
- Der historische Novena-Commit
  `e48619edadbde342d79655e73654f0b21fc5e20b` dokumentiert unabhängig eine
  reale Störung von I2C3 beim Abschalten der ES8328-Versorgung.
- Die grundlegende START-Sequenz des i.MX-I2C-Treibers unterscheidet sich
  zwischen den untersuchten Linux-5.7- und Linux-6.18-Ständen nicht so,
  dass daraus derzeit eine primäre Linux-6.18-Regression folgt.
- Das Entfernen von `single-master` beseitigt das Hardware-IAL nicht.
- Die STMPE811-Isolation beseitigt die untersuchte IT6251-Cold-Boot-
  Fehlerklasse nicht.

### Root-Cause-Kandidaten nach H3-1R

Die noch offenen Kandidaten wurden nach Informationswert und bestehender
Evidence neu priorisiert.

Höchste Priorität besitzen die elektrischen Mechanismen am weiterhin mit
I2C3 verbundenen ES8328-/Audiozweig:

1. Clamp- oder Bus-Loading-Effekt am unversorgten ES8328,
2. Rückspeisung des abgeschalteten Audio-Power-Domains,
3. Pull-up-/Power-Domain-Wechselwirkung,
4. elektrischer Transient beim Abschalten der Audio-Versorgung.

Nachgeordnet offen bleiben:

- IT6251-Power-/Reset-/POR-Zustand,
- aktueller U-Boot-2020.07-I2C3-Übergabezustand,
- interner i.MX6Q-Controller-, Clock-, Pinmux- oder Pad-Zustand.

Eine primäre Linux-6.18-START-/IAL-Regression besitzt nach der bisherigen
Evidence nur noch sehr niedrige Priorität.

Für die untersuchte Fehlerklasse sind STMPE811 und `single-master` als
Ursache experimentell ausgeschlossen. Ein lediglich aus einem früheren
Transfer stehen gebliebenes IAL-Bit ist durch Patch 0006 stark ausgeschlossen.

Die zusätzlich beobachteten Arbitration-Lost-Ereignisse auf I2C0 bleiben ein
separater offener Befund und werden ohne verbindende Evidence nicht mit der
I2C3-/IT6251-Root-Cause gleichgesetzt.

### Nächster Untersuchungsschritt

Der nächste Root-Cause-Schritt wird als

**Block 3.13 – Elektrische ES8328-/I2C3-Messplanung**

definiert.

Vor einer Hardwaremessung werden aus den bereits gesicherten
Schaltplanunterlagen eindeutige und sichere Messpunkte für mindestens
folgende Größen bestimmt:

- tatsächlich geschalteter ES8328-/Audio-Power-Rail beziehungsweise AUD_P3.3V,
- I2C3_SDA,
- I2C3_SCL,
- gemeinsame Masse.

Für jeden Messpunkt müssen Schaltplanbezug, physischer Messpunkt, erwarteter
Zustand, erforderliches Messmittel, zeitliche Auflösung und Messrisiko vor
dem Anschluss eines Messgeräts dokumentiert werden.

Erst danach wird entschieden, ob statische Multimetermessungen genügen oder
Oszilloskop beziehungsweise Logic Analyzer erforderlich sind.

Eine temporäre Wiederherstellung eines Audio-Power-OFF-Zustands wäre nur als
separat geplanter diagnostischer Vergleich zulässig. Sie stellt keine
Rücknahme der dauerhaften Board-Konfiguration dar und erfordert vorab einen
eigenen Sicherungs-, Risiko- und Wiederherstellungsplan.

### Änderungsgrenzen nach Block 3.12

Für Block 3.12 wurde keine funktionale Systemänderung vorgenommen.

Insbesondere gilt weiterhin:

- kein Kernel-Patch `0007`,
- kein zusätzlicher IT6251-Delay,
- kein zusätzlicher IT6251-Retry als vermeintlicher Fix,
- keine routinemäßige Rücknahme von `regulator-always-on`,
- keine erneute Änderung von `single-master`,
- keine erneute STMPE811-Isolation.

Die vollständige Kandidatenmatrix befindet sich unter:

`research/03-root-cause-synthesis/block-3.12-root-cause-kandidatenmatrix.md`

### Aussagegrenze

Block 3.12 beweist noch keinen einzelnen elektrischen Mechanismus.

Gesichert ist die dauerhafte Board-Konfiguration und der stark gestützte
kausale Beitrag des Abschaltens der ES8328-Versorgung zur untersuchten
I2C3-Cold-Boot-Fehlerklasse.

Clamp-, Rückspeisungs-, Pull-up- und Transient-Effekte bleiben voneinander
zu unterscheidende Hypothesen.

## Block 3.13D – Korrigierte elektrische Topologie als Messgrundlage

### Ausgangspunkt

Block 3.13 setzt die in Block 3.12 festgelegte elektrische
ES8328-/I2C3-Root-Cause-Untersuchung fort.

Die dauerhafte Novena-Board-Konfiguration

`es8328-power = regulator-always-on`

bleibt unverändert und wird nicht erneut als offene Konfigurationsfrage
behandelt.

Vor einer Hardwaremessung wurde die bereits archivierte
PVT2-A-Schaltplan-Evidence erneut visuell gegen die Originalseite 14
geprüft.

Dabei wurde eine relevante frühere Interpretation aus der
PDF-Textextraktion korrigiert.

### Korrigierte ES8328-/I2C3-Topologie

Gesichert ist jetzt folgende Struktur:

`I2C3_SCL -> R26A 330R -> AUD_I2C3_SCL -> ES8328E`

`I2C3_SDA -> R27A 330R -> AUD_I2C3_SDA -> ES8328E`

Die visuelle Prüfung der Original-Schaltplanseite zeigt außerdem:

- R10B = 1 kohm von `P3.3V_DELAYED` nach `AUD_I2C3_SCL`,
- R11B = 1 kohm von `P3.3V_DELAYED` nach `AUD_I2C3_SDA`.

Damit liegen R10B und R11B auf der ES8328-Seite von R26A/R27A und nicht,
wie zuvor aus der Text-Extraktion interpretiert, direkt auf der globalen
I2C3-Seite.

Die ursprüngliche Block-1-Dokumentation wurde entsprechend korrigiert.

### Korrektur zu R11A

Die frühere Interpretation

`R11A = 1 kohm von AUD_P3.3V nach AUD_I2C3_SDA`

war falsch.

Die visuelle Originalprüfung zeigt R11A im Mikrofonbereich. R11A ist dort
mit 10 kohm beschriftet und steht im Zusammenhang mit `MIC_DIFF_P`.

R11A wird deshalb nicht mehr als Bestandteil der I2C3-Pull-up-Struktur
geführt.

### Relevante Power-Domain-Struktur

Der ES8328E wird aus dem geschalteten Rail `AUD_P3.3V` versorgt.

Auf derselben Schaltplanseite ist eine eigene Audio-Power-Schaltung
dokumentiert. Sie enthält unter anderem Q11A, Q10A und Q12A sowie die
Beschriftung:

`active pulldown to ensure audio codec reset`

Für die Root-Cause-Untersuchung ist damit folgende Trennung wesentlich:

- `AUD_I2C3_SCL` und `AUD_I2C3_SDA` werden über R10B/R11B aus
  `P3.3V_DELAYED` hochgezogen.
- Die eigentliche ES8328-Versorgung erfolgt separat über das geschaltete
  `AUD_P3.3V`.

Damit kann schaltungstechnisch ein Zustand existieren, in dem
`P3.3V_DELAYED` vorhanden ist und die ES8328-seitigen I2C-Leitungen
hochgezogen werden, während `AUD_P3.3V` abgeschaltet ist.

Nicht bewiesen ist damit, welcher Strom in diesem Zustand tatsächlich
durch die ES8328-I/O-Struktur fließt.

### Verbindung mit der gesicherten experimentellen Evidence

Die korrigierte Schaltungstopologie wird auf Syntheseebene mit der bereits
gesicherten H3-1R-Evidence zusammengeführt.

Der Baseline-Cold-FAIL zeigt:

`A=81/80 -> M0=93/80`

und die Abschaltung von `es8328-power` ungefähr 10,716 Sekunden vor dem
ersten beobachteten I2C3-Fehler.

Der erfolgreiche Same-Boot-LDB-Rebind zeigt:

`A=81/80 -> M0=81/a0`

H3-1R fügte ausschließlich `regulator-always-on` für `es8328-power`
hinzu und erreichte 5/5 erfolgreiche vorab festgelegte echte
POR-Cold-Boots.

In diesen fünf vollständigen Kernel-Logs fehlen die bekannte
`93/80`-Signatur und das untersuchte I2C3-`arbitration lost`.

Die neue Schaltplan-Evidence erklärt damit noch nicht den elektrischen
Mechanismus, macht aber die bereits priorisierten Kandidaten M1 bis M3
wesentlich konkreter:

1. Clamp- oder Bus-Loading-Effekt am unversorgten ES8328,
2. Rückspeisung des abgeschalteten Audio-Power-Domains,
3. Pull-up-/Power-Domain-Wechselwirkung.

M4, ein zeitabhängiger Abschalttransient, bleibt ebenfalls offen.

### Neue Messgrundlage

R26A und R27A besitzen für die weitere Untersuchung besonderen
Informationswert.

Jeweils eine Widerstandsseite gehört zum globalen I2C3, die andere zum
ES8328-/Audiozweig.

Für die weitere Messplanung werden deshalb mindestens folgende Größen
vorgesehen:

1. `AUD_P3.3V`,
2. globale Seite von R26A / `I2C3_SCL`,
3. ES8328-Seite von R26A / `AUD_I2C3_SCL`,
4. globale Seite von R27A / `I2C3_SDA`,
5. ES8328-Seite von R27A / `AUD_I2C3_SDA`,
6. `P3.3V_DELAYED`,
7. sichere gemeinsame Masse.

Eine messbare Spannungsdifferenz über R26A beziehungsweise R27A kann
Strom durch den jeweiligen Zweig anzeigen.

Eine solche Beobachtung allein beweist weder Backpower noch Clamp und
identifiziert noch keinen internen ES8328-Strompfad.

### Noch keine physische Messfreigabe

Die elektrischen Netze sind bestimmt, die konkreten sicheren
Platinenmesspunkte jedoch noch nicht.

Insbesondere ist noch nicht bewiesen, dass die Pads von R26A/R27A auf
der tatsächlich getesteten Novena mechanisch sicher mit einem
Messgerät erreichbar sind.

Vor einer Hardwaremessung müssen deshalb zunächst aus Layout-Unterlagen
und geeigneten Platinenabbildungen eindeutig bestimmt werden:

- physische Lage und Orientierung von R26A,
- physische Lage und Orientierung von R27A,
- geeigneter Messpunkt für `AUD_P3.3V`,
- geeigneter Messpunkt für `P3.3V_DELAYED`,
- sicherer Massepunkt.

Für jeden später verwendeten Messpunkt sind zusätzlich Messmittel,
erwarteter Zustand, erforderliche zeitliche Auflösung und
Kurzschluss-/Belastungsrisiko festzulegen.

### Nächster Teilschritt

Der nächste Teilschritt ist die read-only Erstellung einer physischen
Messpunktkarte aus den bereits vorhandenen Layout- und
Platinenunterlagen.

Es erfolgt noch keine Messung an der Novena.

Insbesondere wird derzeit nicht:

- `regulator-always-on` zurückgenommen,
- ein Audio-Power-OFF-Zustand erzeugt,
- R26A oder R27A mit einem Tastkopf kontaktiert,
- ein Logic Analyzer an I2C3 angeschlossen,
- Kernel-Patch `0007` begonnen.

Eine spätere temporäre Wiederherstellung eines Audio-Power-OFF-Zustands
wäre nur als separat vorbereiteter diagnostischer Vergleich zulässig
und stellt keine Rücknahme der dauerhaften Board-Konfiguration dar.

### Dokumentationsartefakte

Die korrigierte Originalquellenanalyse befindet sich unter:

`research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md`

Die Block-3.13-Synthese und Messplanung befindet sich unter:

`research/03-root-cause-synthesis/block-3.13-elektrische-messplanung.md`

### Aussagegrenze

Block 3.13D beweist weiterhin keinen einzelnen elektrischen
Root-Cause-Mechanismus.

Gesichert ist jetzt zusätzlich die für die Messplanung entscheidende
Schaltungstopologie:

Die ES8328-seitigen I2C-Netze werden aus `P3.3V_DELAYED` hochgezogen,
während die eigentliche Codec-Versorgung aus dem separat geschalteten
`AUD_P3.3V` erfolgt.

Clamp, Rückspeisung, Power-Domain-Wechselwirkung und Abschalttransient
bleiben voneinander zu unterscheidende Hypothesen.

## Checkpoint Block 3.13E – quellenbasierte Eingrenzung ohne Hardwaremessung – 2026-09-18

### Ausgangspunkt

Block 3.13D hatte die elektrische Topologie des ES8328-/I2C3-Pfads
aus den PVT2-A-Unterlagen weiter präzisiert und daraus eine physische
Messplanung abgeleitet.

Für die weitere Untersuchung stehen jedoch keine elektrischen Messgeräte
zur Verfügung. Es werden deshalb keine Hardwaremessungen durchgeführt und
keine Messwerte simuliert.

Block 3.13E wechselte folgerichtig auf eine rein quellenbasierte
Eingrenzung der noch offenen elektrischen Root-Cause-Kandidaten.

Die dauerhafte Board-Konfiguration

`es8328-power = regulator-always-on`

blieb während der gesamten Untersuchung unverändert.

Kernel-Patch `0007` wurde nicht begonnen.


### Weiterhin maßgebliche experimentelle Evidenz

Die aktuelle experimentelle Evidenz bleibt die kontrollierte H3-1R-Serie.

Im gesicherten Baseline-Cold-FAIL wird `es8328-power` während des
Bootvorgangs automatisch abgeschaltet.

Später tritt die instrumentierte I2C3-Fehlersignatur

`A=81/80 -> M0=93/80`

mit `IAL` und nicht gesetztem `MSTA` auf.

Der erfolgreiche Same-Boot-LDB-Rebind zeigt dagegen:

`A=81/80 -> M0=81/a0`.

H3-1R änderte als einzige semantische Device-Tree-Eigenschaft am
bestehenden Audio-Regulator die dauerhafte Aktivierung durch
`regulator-always-on`.

Unter dieser kontrollierten Intervention bestanden fünf von fünf
vorregistrierten echten POR-Cold-Boots.

In keinem dieser fünf vollständigen Kernel-Logs trat die untersuchte
`93/80`-/`arbitration lost`-Signatur auf.

IT6251 initialisierte erfolgreich und das Display funktionierte.

Damit bleibt ein kausaler Beitrag des Abschaltens des ES8328-Power-Domains
zur untersuchten Fehlerklasse stark experimentell gestützt.


### Relevante elektrische Topologie

Die zuvor korrigierte PVT2-A-Auswertung bleibt Grundlage der
Root-Cause-Eingrenzung:

- `I2C3_SCL -> R26A 330R -> AUD_I2C3_SCL -> ES8328E`,
- `I2C3_SDA -> R27A 330R -> AUD_I2C3_SDA -> ES8328E`,
- R10B zieht `AUD_I2C3_SCL` über 1 kOhm nach `P3.3V_DELAYED`,
- R11B zieht `AUD_I2C3_SDA` über 1 kOhm nach `P3.3V_DELAYED`,
- ES8328-DVDD liegt an `AUD_P3.3V`,
- ES8328-PVDD liegt an `AUD_P3.3V`,
- `AUD_P3.3V` ist separat geschaltet,
- die I2C-Pull-ups des Audiozweigs werden dagegen aus
  `P3.3V_DELAYED` versorgt.

Damit ist schaltungstechnisch ein Zustand möglich, in dem der Codec
unversorgt ist, während seine I2C-Leitungen weiterhin über Widerstände
in Richtung einer aktiven 3,3-V-Versorgung gezogen werden.

Diese Topologie macht Clamp-/Bus-Loading, Rückspeisung,
Pull-up-/Power-Domain-Wechselwirkung und Abschalttransient zu konkreten
Hypothesen.

Sie beweist keinen dieser Mechanismen.


### ES8328-Komponentendokumentation

Die Suche nach zusätzlicher frei zugänglicher ES8328-Dokumentation
lieferte keinen ausreichend belastbaren neuen Primärbeleg für die interne
I/O-Struktur im unversorgten Zustand.

Insbesondere wird aus sichtbaren absoluten Eingangsspannungsgrenzen nicht
auf eine konkrete interne Schutzdiode oder einen bestimmten
Backpower-Strompfad geschlossen.

Weiterhin nicht belegt beziehungsweise nicht gemessen sind:

- interne Clamp-Struktur des ES8328,
- konkreter Rückspeisepfad,
- tatsächliche SDA-/SCL-Spannung am abgeschalteten Codec,
- Strom durch R26A oder R27A,
- Spannung auf `AUD_P3.3V` während des kritischen Zustands.

M1 und M2 bleiben damit plausibel, aber nicht bewiesen.


### Historische ES8328-Power-Management-Commits

Im historischen Repository `xobs/novena-linux` wurden die Commits

`468b85c56f0942b518a8b03ac0b6293fe6df8408`

und

`ff010f4fa0b44fceefd60a3449cf80af4379ca1b`

gezielt untersucht.

`468b85c...` aktiviert die normale ASoC-Power-Management-Integration des
i.MX6-ES8328-Machine-Drivers.

Der Commit dokumentiert keine I2C3-Störung.

`ff010f4...` entfernt einen Codec-spezifischen Suspend-/Resume-Pfad.
Der entfernte Suspend-Code schaltete Clock und ES8328-Versorgungen
ausdrücklich ab; der Resume-Code aktivierte sie wieder und synchronisierte
den Codec-Zustand.

Die Commit-Nachricht begründet die Entfernung mit der bereits vorhandenen
ASoC-Power-Behandlung.

Damit ist historisch belegt, dass Softwarepfade das Abschalten der
ES8328-Versorgungen ausdrücklich vorsahen.

Diese beiden Commits belegen für sich jedoch keine I2C3-Störung und keine
Notwendigkeit von `regulator-always-on`.


### Ältere Novena-DTS-Stände

Direkt geprüfte Novena-DTS-Stände aus `xobs/novena-linux` zeigen für
`es8328-power` zunächst `regulator-boot-on`.

Dies gilt unter anderem für:

- `d0bbd1497c117cd9661bc98685e25a5db23506c8`,
- `70a8c03bd9eea54fcd2616302403b80c20729db9`,
- den untersuchten Repository-HEAD
  `d9d2e8b619f17e8394d62c08d60b4ea17154e9d6`.

Damit ist die frühe beziehungsweise ältere Novena-Konfiguration nicht als
generelles `regulator-always-on` zu interpretieren.


### Methodische Korrektur früherer Git-Suchen

Breite Git-Suchen nach der Zeichenfolge `regulator-always-on` hatten
zwischenzeitlich Treffer geliefert, die nicht zuverlässig dem
ES8328-Regulator zugeordnet werden konnten.

Insbesondere beweist

`git log -Sregulator-always-on`

nur, dass die Zeichenfolge im jeweiligen Diff vorkommt.

Sie beweist nicht, dass sie im `es8328-power`-Knoten geändert wurde.

Auch eine spätere Auswertung mit festem `grep`-Kontext um
`es8328-power` war ungeeignet, weil benachbarte Regulator-Knoten in den
Kontext geraten konnten.

Diese Treffer werden nicht als ES8328-spezifische Evidenz verwendet.

Die direkt geprüften Regulator-Knoten und vollständigen Commit-Diffs haben
Vorrang.


### Provenienz von e48619 erneut geklärt

Zwischenzeitlich entstand Unsicherheit über den bereits dokumentierten
historischen Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`.

Der Commit war zunächst weder im untersuchten `xobs/novena-linux` noch im
lokalen Objektbestand von `~/novena-linux` auffindbar.

Die Provenienzprüfung zeigte, dass diese Nichtfunde kein Gegenbeleg waren.

Der Commit stammt aus:

`https://github.com/novena-next/linux.git`

und nicht aus `xobs/novena-linux`.

Der vorhandene lokale Clone `~/novena-linux` zeigt zwar auf dieses
Repository, ist aber ein Shallow Clone und enthält lokal ausschließlich
den Branch `nvn_v5.7-rc2`.

Zum Prüfzeitpunkt galt:

`HEAD=1fda06deecb61538ca3d07d256eb7c43d4e3432a`

und

`SHALLOW=true`.

Der gesuchte Commit, sein Parent und der historische Repository-HEAD waren
deshalb nicht im lokalen Objektbestand vorhanden.


### Archivierter Primärquellen-Auszug

Im Projekt war der frühere Primärquellen-Auszug weiterhin vorhanden:

`research/02-historical-software/extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt`

SHA-256:

`35bbe085ce6c199f15596e0edaa5106e10b1ef1db4a8a416491e0437b27e19a4`

Er dokumentiert:

Repository:

`https://github.com/novena-next/linux.git`

Repository-HEAD:

`18bf34080c4c3beb6699181986cc97dd712498fe`

Commit:

`e48619edadbde342d79655e73654f0b21fc5e20b`

Parent:

`16aae414f47116b568c837a131ba9d9250cf3b48`

Autor:

Jookia

Datum:

2020-04-01

Betreff:

`ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

Die archivierte Commit-Nachricht beschreibt ausdrücklich, dass Linux den
`es8328-power`-Regulator abschaltet, wenn der Codec nicht verwendet wird,
und dass dies nach damaliger Beobachtung den I2C3-Bus beeinträchtigt und
dadurch unter anderem Bildschirm, EEPROM und Senoko stört.

Der archivierte DTS-Diff ersetzt für genau diesen Regulator

`regulator-boot-on`

durch

`regulator-always-on`.


### Erneute unabhängige Remote-Prüfung

Die Repository-Provenienz wurde am 2026-09-18 erneut gegen den öffentlich
erreichbaren Remote-Zustand geprüft.

`git ls-remote` für `novena-next/linux` meldete:

`18bf34080c4c3beb6699181986cc97dd712498fe refs/heads/master`

Der gleiche Commit wurde als Default-HEAD des Remote-Repositorys
zurückgegeben.

Damit stimmt der aktuelle Remote-`master` exakt mit dem im archivierten
Primärquellen-Auszug dokumentierten Repository-HEAD überein.

Die öffentliche Commitquelle bestätigte zusätzlich Commit-ID, Autor,
Parent, Betreff, Commit-Nachricht und DTS-Änderung.

`e48619edadbde342d79655e73654f0b21fc5e20b` wird deshalb als
verifizierter historischer Novena-Primärbeleg behandelt.


### Zusätzlicher Shallow-History-Versuch

Für eine zusätzliche lokale Reproduktion wurde ein separates temporäres
Repository unter `/tmp` verwendet.

Der gesuchte Commit war nach einem begrenzten Shallow-Fetch und einer
anschließenden Vertiefung noch nicht lokal vorhanden.

Wegen der stark verzweigten Linux-Kernel-Historie erzeugte diese Methode
bereits eine sehr große erreichbare Commitmenge und wurde nicht weiter
verfolgt.

Das temporäre Repository wurde anschließend entfernt beziehungsweise war
bei der abschließenden Aufräumkontrolle nicht mehr vorhanden.

Das Projekt-Repository und der vorhandene `~/novena-linux`-Clone blieben
unverändert.


### Quellenübergreifendes Ergebnis

Nach Block 3.13E stehen zwei unabhängige Evidenzlinien nebeneinander.

Historisch dokumentiert `e48619...` auf realer Novena-Hardware eine
Wechselwirkung zwischen dem Abschalten von `es8328-power` und der
Funktionsfähigkeit von I2C3.

Die historische Gegenmaßnahme war:

`regulator-always-on`.

Unabhängig davon zeigt die aktuelle kontrollierte H3-1R-Serie:

- Baseline mit Abschaltung von `es8328-power`,
- später untersuchte I2C3-Fehlersignatur,
- Änderung ausschließlich der Regulator-Policy,
- fünf von fünf erfolgreiche echte POR-Cold-Boots,
- kein Auftreten der untersuchten `93/80`-/`arbitration lost`-Signatur.

Historische Quelle und aktuelles Experiment stützen damit unabhängig
voneinander dieselbe boardspezifische Gegenmaßnahme.


### Aussagegrenze

Die quellenübergreifende Übereinstimmung beweist nicht den konkreten
elektrischen Mechanismus.

Insbesondere ist weiterhin nicht bewiesen:

- dass das historische Fehlerereignis exakt dem heutigen
  `A=81/80 -> M0=93/80` entspricht,
- dass M1, M2, M3 oder M4 der konkrete elektrische Mechanismus ist,
- dass eine interne ES8328-Schutzdiode beteiligt ist,
- dass tatsächlich Rückspeisung nach `AUD_P3.3V` stattfindet,
- welcher Strom durch R26A oder R27A fließt,
- welche Spannungsverläufe während des kritischen Zustands auftreten.

Ohne elektrische Messungen lassen sich M1 bis M4 aus der derzeitigen
Quellenlage nicht eindeutig voneinander trennen.


### Kandidatenstatus nach Block 3.13E

M1, Clamp-/Bus-Loading am unversorgten ES8328:

Hohe Priorität, mit der bekannten Schaltungstopologie vereinbar,
nicht bewiesen.

M2, Rückspeisung des Audio-Power-Domains:

Hohe Priorität, mit der bekannten Schaltungstopologie vereinbar,
nicht bewiesen.

M3, Pull-up-/Power-Domain-Wechselwirkung:

Hohe Priorität und durch die getrennten Versorgungen besonders konkret,
nicht bewiesen.

M4, Abschalttransient:

Weiterhin offen und nicht bewiesen.

M5, primärer IT6251-Power-/Reset-/POR-Fehler:

Als alleinige Primärursache gegenüber M1 bis M4 weniger naheliegend;
als nachgelagerter Zustand weiterhin möglich.

M6, primärer U-Boot-I2C3-Handoff-Fehler:

Als alleinige Primärursache gegenüber der Audio-Regulator-Abhängigkeit
weniger naheliegend; ein beitragender Ausgangszustand ist nicht vollständig
ausgeschlossen.

M7, interner i.MX6Q-Controller-/Clock-/Pinmux-Zustand:

Nicht vollständig ausgeschlossen, aber niedriger priorisiert.

M8, primäre Linux-6.18-START-/IAL-Regression:

Durch die historische unabhängige ES8328-/I2C3-Beobachtung und H3-1R
weiter geschwächt.

M9, STMPE811:

Für die untersuchte Fehlerklasse experimentell ausgeschlossen.

M10, Single-Master-Hypothese:

Für das untersuchte IAL-Ereignis experimentell ausgeschlossen.

M11, altes beziehungsweise stehengebliebenes IAL-Bit:

Durch Patch 0006 stark ausgeschlossen.

M12, separates I2C0-`arbitration lost`:

Bleibt ein eigenständiger offener Befund.


### Entscheidung nach Block 3.13E

Die dauerhafte Novena-Board-Konfiguration

`es8328-power = regulator-always-on`

bleibt bestehen.

Es wird derzeit nicht:

- zur Baseline-Regulator-Konfiguration zurückgekehrt,
- ein zusätzlicher IT6251-Retry eingeführt,
- eine zusätzliche Verzögerung als Ersatzfix eingebaut,
- STMPE811 erneut isoliert,
- die Single-Master-Hypothese erneut getestet,
- Kernel-Patch `0007` begonnen.

Die weitere Root-Cause-Arbeit darf sich auf die noch offene elektrische
Mechanismusfrage konzentrieren.

Da keine Hardwaremessungen durchgeführt werden, wird zunächst nur noch
geprüft, ob vorhandene Originalunterlagen, Komponentenquellen oder
historische Entwicklungsquellen M1 bis M4 weiter diskriminieren können.

Falls daraus keine zusätzliche belastbare Evidenz entsteht, wird die
Erkenntnisgrenze ausdrücklich akzeptiert:

Die boardspezifische Gegenmaßnahme ist experimentell und historisch stark
abgesichert; der genaue elektrische Mechanismus bleibt ohne Messung offen.


### Dokumentationsartefakte

Die vollständige Synthese von Block 3.13E befindet sich unter:

`research/03-root-cause-synthesis/block-3.13e-quellenbasierte-eingrenzung.md`

Der verifizierte historische Primärquellen-Auszug befindet sich unter:

`research/02-historical-software/extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt`

Die korrigierte elektrische Topologie befindet sich unter:

`research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md`

## Checkpoint Block 3.13F – Primärquellenbasierte Eingrenzung von FPGA und ES8328 – 2026-09-18

### Ausgangspunkt

Block 3.13F setzte die quellenbasierte Untersuchung aus Block 3.13E fort.

Da keine elektrischen Messgeräte zur Verfügung stehen und keine
Hardwaremessungen durchgeführt werden, wurden zwei verbleibende
Fragestellungen ausschließlich anhand von Schaltungsunterlagen,
Herstellerdokumentation, historischen Novena-Quellen und bereits
gesicherter experimenteller Evidence untersucht:

1. Kann der direkt an I2C3 angeschlossene Spartan-6-FPGA während
   Power-on, Initialisierung oder Konfiguration eine plausible primäre
   Busbelastung darstellen?

2. Lassen sich die verbleibenden ES8328-Mechanismuskandidaten M1 bis M4
   anhand belastbarer Quellen weiter voneinander trennen?

Die bereits in Block 3.13E erreichte Aussage zur kausalen Beteiligung
der ES8328-/Audio-Power-Domain wurde dabei nicht zurückgenommen.

### Weiterhin maßgebliche experimentelle Evidence

Der Versuch H3-1R änderte ausschließlich die ES8328-Versorgung auf
`regulator-always-on`.

Danach waren fünf von fünf vorregistrierten echten POR-Kaltstarts
erfolgreich.

Dabei verschwanden im untersuchten I2C3-Pfad:

- die bekannte Signatur `A=81/80 -> M0=93/80`,
- der untersuchte `arbitration lost`,
- der IT6251-Initialisierungsfehler.

Der Bildschirm funktionierte in allen fünf Läufen.

Diese experimentelle Evidence bleibt die maßgebliche aktuelle
Interventionsevidenz.

### Archivierte Spartan-6-Primärquellen

Für Block 3.13F wurden zwei offizielle AMD/Xilinx-Dokumente archiviert:

- Spartan-6 FPGA Configuration User Guide, UG380, Version 2.11,
  2019-03-22,
- Spartan-6 FPGA SelectIO Resources User Guide, UG381, Version 1.7,
  2015-10-21.

Archivdateien:

`research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.pdf`

SHA-256:

`4afb6472018a9b3fa3bc1be906a0d15f86a17567d1dd7e930debfce3de362c9b`

und

`research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.pdf`

SHA-256:

`4a0fc9078af54edc1104452fe5fa2bfaa82118b5501ba9fd000c1e1e2310821a`

Die HTTP-Header beider Downloads wurden ebenfalls archiviert.

Alle vier Dateien wurden in

`research/01-original-novena-docs/SHA256SUMS`

aufgenommen.

Die Gesamtprüfung des bestehenden Quellenarchivs ergab vor Erstellung
dieses Checkpoints:

- 55 geprüfte Einträge,
- 0 Prüfsummenfehler.

### FPGA-Verbindung mit I2C3

Die PVT2-A-Schaltung zeigt:

- FPGA-Pin P4 = `IO_L2P_3` = `I2C3_SCL`,
- FPGA-Pin P3 = `IO_L2N_3` = `I2C3_SDA`.

P3 und P4 sind damit direkt an I2C3 angeschlossene normale
Spartan-6-User-I/Os.

Die visuelle Schaltplanprüfung korrigierte außerdem ein früheres
Text-Extraktionsartefakt:

`FPGA_HSWAPEN` ist nicht mit `EIM_DA14` gleichzusetzen.

### HSWAPEN-Beschaltung

Die PVT2-A-Schaltung zeigt:

- R13F = 4,7 kOhm von `P3.3V_DELAYED` nach `FPGA_HSWAPEN`,
- R12F = 4,7 kOhm von `FPGA_HSWAPEN` nach GND,
- R12F ist `DNP`.

Damit wird `FPGA_HSWAPEN` in der dokumentierten Bestückung nach High
gezogen.

UG380 und UG381 dokumentieren:

- HSWAPEN Low aktiviert die internen Pull-ups der User-I/Os,
- HSWAPEN High deaktiviert diese Pull-ups.

Damit sind die HSWAPEN-gesteuerten internen User-I/O-Pull-ups auf
Novena während der relevanten Konfigurationsphase deaktiviert.

### FPGA-I/O-Zustand während Power-on und Konfiguration

UG380 und UG381 dokumentieren, dass die normalen User-I/O-Ausgangstreiber
während Power-on, Initialisierung und Konfiguration High-Z sind.

Für P3/P4 ergibt sich damit während dieser Phase:

- Ausgangstreiber High-Z,
- HSWAPEN-gesteuerte interne Pull-ups deaktiviert.

Ein einfacher Fehlermechanismus, bei dem der FPGA bereits durch seinen
normalen POR-/Initialisierungs-/Konfigurationszustand I2C3 aktiv treibt,
wird dadurch deutlich geschwächt.

### Grenze des FPGA-Befunds

Der High-Z-Befund gilt nicht automatisch für den gesamten Bootvorgang.

Nach Freigabe von GTS während der Spartan-6-Startup-Sequenz gehen die
User-I/Os in den vom geladenen User-Design bestimmten Zustand über.

Daher bleiben formal offen:

- Zeitpunkt und Zustand eines eventuell aktiven FPGA-Designs,
- Zustand von P3/P4 nach GTS-Freigabe,
- ein möglicher Post-Configuration-Einfluss auf I2C3.

Für einen solchen Post-Configuration-Fehler wurde in den untersuchten
Quellen jedoch kein konkreter positiver Novena-spezifischer Beleg
gefunden.

### Historischer U-Boot-Befund zum FPGA

Die untersuchten historischen Novena-U-Boot-Quellen definieren

`NOVENA_FPGA_RESET_N_GPIO`

als GPIO5_IO07.

Der historische SPL setzt dieses Signal auf Low.

In den untersuchten Novena-spezifischen U-Boot-Stellen wurde keine
FPGA-Bitstream-Ladeoperation gefunden.

`FPGA_RESET_N` darf anhand dieser Quellen nicht mit dem dedizierten
Spartan-6-Konfigurationssignal `PROGRAM_B` gleichgesetzt werden.

Aus `FPGA_RESET_N = 0` folgt deshalb nicht, dass P3/P4 dauerhaft High-Z
sein müssen.

### ES8328-Dokumentationslage

Im lokalen Quellenbestand wurde kein ES8328-Herstellerdatenblatt
gefunden.

Ein zusätzlicher direkter Downloadversuch über einen öffentlich
indexierten Datenblattspiegel endete mit HTTP 404.

Das dabei entstandene Artefakt verblieb ausschließlich unter `/tmp` und
wurde nicht archiviert.

Eine anschließende lokale Suche auf der foobox fand ebenfalls keine
ES8328-Datenblatt-PDF.

Weitere schwach nachvollziehbare Datenblattspiegel wurden bewusst nicht
akkumuliert.

### Lokale ES8328-Linux-Quelle

Untersucht wurde der lokale historische Quellbestand:

Repository:

`https://github.com/novena-next/linux.git`

HEAD:

`1fda06deecb61538ca3d07d256eb7c43d4e3432a`

Betreff:

`FIXME: ITE workaround`

Datum:

`2020-01-27T05:13:57+01:00`

Die daraus abgeleiteten Aussagen werden ausdrücklich diesem
untersuchten historischen Quellstand zugeordnet.

### ES8328-Supply-Modell

Die lokale Datei

`Documentation/devicetree/bindings/sound/es8328.txt`

beschreibt:

- `DVDD-supply` als Versorgung des digitalen Kerns mit 1,8 bis 3,6 V,
- `PVDD-supply` als Versorgung der digitalen I/Os mit 1,8 bis 3,6 V.

Für die Root-Cause-Frage ist insbesondere belegt:

**PVDD wird in diesem historischen Linux-Binding als Digital-I/O-Supply
des ES8328 modelliert.**

### ES8328-Regulator-Handling

Der lokale Codec-Treiber

`sound/soc/codecs/es8328.c`

verwaltet:

- DVDD,
- AVDD,
- PVDD,
- HPVDD

als Regulator-Supplies.

Der untersuchte Suspend-Pfad kann diese Supplies gemeinsam über

`regulator_bulk_disable()`

abschalten.

Beim Resume werden sie über

`regulator_bulk_enable()`

wieder eingeschaltet und der Regcache anschließend synchronisiert.

Im untersuchten I2C-Treiber wurde keine besondere Powered-off-Sequenz
für CCLK/CDATA gefunden.

Aus dem Fehlen einer solchen Sequenz folgt keine Aussage darüber, ob ein
unversorgter ES8328 die extern hochgezogenen I2C-Leitungen elektrisch
beeinflusst.

### Belegter Power-Domain-Grenzzustand

Die Kombination aus Linux-Supply-Modell und Novena-Schaltung ergibt
folgenden direkt gestützten Zustand:

- ES8328 DVDD liegt an `AUD_P3.3V`,
- ES8328 PVDD liegt an `AUD_P3.3V`,
- `AUD_P3.3V` kann abgeschaltet werden,
- PVDD wird im untersuchten Linux-Binding als Digital-I/O-Supply
  beschrieben,
- `AUD_I2C3_SCL` bleibt über R10B = 1 kOhm aus `P3.3V_DELAYED`
  hochgezogen,
- `AUD_I2C3_SDA` bleibt über R11B = 1 kOhm aus `P3.3V_DELAYED`
  hochgezogen.

Damit können Digital-Core- und Digital-I/O-Versorgung des ES8328
abgeschaltet sein, während die externen Codec-seitigen I2C-Pull-ups
weiter versorgt werden.

Der interne elektrische Effekt dieses Grenzzustands ist nicht bestimmt.

### M1 – Clamp oder statische Busbelastung

Status:

**offen; mit der Topologie vereinbar; nicht belegt.**

Nicht belegt sind insbesondere:

- interne CCLK/CDATA-Schutzstrukturen,
- ein konkreter Clamp-Pfad,
- resultierende Pinspannungen,
- resultierende Ströme.

### M2 – Backfeeding

Status:

**offen; mit der Topologie vereinbar; nicht belegt.**

Nicht belegt sind:

- ein konkreter interner Backfeed-Pfad,
- Stromrichtung und Stromgröße,
- eine mögliche Teilversorgung des Codecs,
- ein direkter Zusammenhang mit der beobachteten I2C3-Signatur.

### M3 – Pull-up-/Power-Domain-Interaktion

Status:

**stark gestützter Power-Domain-Grenzzustand; exakter elektrischer
Mechanismus nicht bewiesen.**

Von M1 bis M4 ist M3 am unmittelbarsten durch die Kombination aus
Novena-Schaltung und lokalem Linux-Supply-Modell gestützt.

Belegt ist dabei der Grenzzustand und nicht ein bestimmter interner
Clamp- oder Backfeed-Mechanismus.

### M4 – Abschalttransient

Status:

**offen; mit der Schaltung vereinbar; ohne Messung nicht nachgewiesen.**

Der aktive Audio-Power-Schaltkreis mit Q11A, Q10A und Q12A lässt einen
zeitabhängigen Abschaltmechanismus als Untersuchungsrichtung offen.

Mangels elektrischer Zeit-, Spannungs- und Strommessungen ist ein
solcher Transient nicht nachgewiesen.

### Quellenübergreifende Root-Cause-Eingrenzung

Nach Block 3.13F zeigen mehrere voneinander verschiedene Evidenzlinien
auf denselben kausalen Bereich:

1. Die Novena-Schaltung dokumentiert den ES8328-Power-Domain-Grenzzustand.

2. Der historische Novena-Commit
   `e48619edadbde342d79655e73654f0b21fc5e20b`
   dokumentiert, dass das Abschalten von `es8328-power` nach damaliger
   Beobachtung I2C3 störte und unter anderem Bildschirm, EEPROM und
   Senoko beeinträchtigte.

3. Der unabhängige aktuelle H3-1R-Versuch beseitigte mit
   `regulator-always-on` die untersuchte Störung in fünf von fünf
   vorregistrierten echten POR-Kaltstarts.

4. Die Spartan-6-Primärquellen schwächen den normalen
   FPGA-POR-/Konfigurationszustand als einfache konkurrierende
   Busbelastung deutlich.

Damit ist die ES8328-/Audio-Power-Domain-Abschaltung als kausaler Bereich
der untersuchten I2C3-Kaltstartstörung stark eingegrenzt.

### Aussagegrenze

Nicht bewiesen sind:

- ein interner ES8328-Clamp-Pfad,
- ein konkreter Backfeed-Pfad,
- bestimmte Spannungen oder Ströme,
- ein bestimmter Abschalttransient,
- die vollständige Ausschließung eines FPGA-Einflusses nach
  GTS-Freigabe,
- die elektrische Identität jedes historischen I2C3-Fehlers mit der
  aktuellen Signatur `A=81/80 -> M0=93/80`.

Ohne zusätzliche belastbare Herstellerinformation zum unversorgten
CCLK/CDATA-Verhalten oder elektrische Hardwaremessungen lassen sich M1,
M2, M3 und M4 nicht seriös bis auf einen einzelnen mikroskopischen
Mechanismus auflösen.

### Entscheidung nach Block 3.13F

Die praktische Board-Konfiguration bleibt unverändert:

**`es8328-power` bleibt dauerhaft eingeschaltet.**

Block 3.13F liefert keinen Anlass für:

- Patch 0007,
- eine erneute STMPE811-Isolation,
- eine Wiederholung der Single-Master-Isolation,
- zusätzliche IT6251-Delay-/Retry-Experimente,
- eine routinemäßige Rücknahme von `regulator-always-on`.

Die verbleibende Unsicherheit betrifft den exakten elektrischen
Mechanismus und nicht die praktische Board-Konfiguration.

### Dokumentationsartefakte

Neu beziehungsweise erweitert wurden:

- `research/01-original-novena-docs/SHA256SUMS`
- `research/01-original-novena-docs/notes/spartan6-i2c3-configuration-state.md`
- `research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.pdf`
- `research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.http-headers.txt`
- `research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.pdf`
- `research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.http-headers.txt`
- `research/03-root-cause-synthesis/block-3.13f-primaerquellen-eingrenzung.md`

Damit ist Block 3.13F dokumentarisch abgeschlossen.


## 2026-09-18 – Block 3.14A: Power-/I2C3-Zustandsrekonstruktion

Block 3.14A rekonstruiert den zeitlichen und logischen Zusammenhang
zwischen der Abschaltung von `es8328-power` und der späteren
I2C3-Fehlersignatur.

Die exakte Kernelquelle des untersuchten Linux-6.18.49-Stands ist:

`/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz`

Die direkte Analyse von `drivers/regulator/core.c` bestätigt:

- `regulator_init_complete()` plant den Cleanup mit 30000 ms
  Verzögerung nach dem Late-Initcall.
- `regulator_late_cleanup()` überspringt `always_on`-Regulatoren.
- Ein Regulator mit aktivem `use_count` wird nicht abgeschaltet.
- Für einen aktivierten, unbenutzten und abschaltbaren Regulator wird
  `disabling` protokolliert und danach `_regulator_do_disable()`
  ausgeführt.

Damit ist die Baseline-Meldung:

`[   33.761742] es8328-power: disabling`

direkt dem Linux-6.18.49-Regulator-Late-Cleanup zuzuordnen.

Die Schaltung zeigt gleichzeitig:

- ES8328-/Audio-Versorgung über die geschaltete Domain `AUD_P3.3V`,
- Codec-seitige I2C3-Pull-ups R10B/R11B aus `P3.3V_DELAYED`,
- I2C3-Anbindung des Codecs über R26A/R27A.

Damit kann nach der Audio-Regulator-Abschaltung ein
Power-Domain-Grenzzustand bestehen, in dem der Codec unversorgt ist,
während seine externen I2C-Leitungen weiterhin hochgezogen werden.

Die relevante Baseline-Reihenfolge lautet:

- 33.761742 s: `es8328-power: disabling`
- 42.097171 s: IT6251 `bridge_attach`
- 42.395240 s: `bridge_pre_enable`
- 42.395351 s: IT6251 `regulator_enable`
- 43.172998 s: `imx-es8328 sound: Unable to register: -517`
- 44.471549 s: IT6251-Regulator aktiviert
- 44.471609 s: `product ID attempt 1/5`
- 44.478167 s: erste log-sichtbare problematische I2C3-Transaktion,
  `arbitration lost`, `I2SR=0x93`
- 44.478235 s:
  `START failure A=81/80 B=93/80 C=83/80 ret=-11`

Der Abstand von 10.716425 s zwischen der Regulator-Abschaltung und der
ersten log-sichtbaren problematischen I2C3-Transaktion ist keine
nachgewiesene elektrische Fehlerlatenz.

Zwischen diesen Zeitpunkten ist im vorhandenen Log kein weiterer
I2C3-Transfer nachweisbar. Daraus darf nicht geschlossen werden, dass
garantiert kein Transfer stattgefunden hat.

`-517` entspricht `EPROBE_DEFER`, ist aber nicht als Ursache der
I2C3-Störung belegt.

Direkt nachgewiesen ist außerdem, dass `regulator-always-on` im exakten
Linux-6.18.49-Regulator-Core den Late-Cleanup dieses Regulators
verhindert.

Dies ist konsistent mit H3-1R, bei dem fünf von fünf vorregistrierten
echten POR-Kaltstarts ohne die bekannte `93/80`-/Arbitration-Loss-
Signatur erfolgreich waren.

Der genaue elektrische Mechanismus bleibt unbestimmt. Insbesondere
sind Clamp, Backfeeding, Teilversorgung und Abschalttransient nicht als
einzelner Mechanismus bewiesen.

Die bestehende Board-Entscheidung bleibt deshalb unverändert:

**`es8328-power` bleibt dauerhaft mit `regulator-always-on`
eingeschaltet.**

Block 3.14A ist damit inhaltlich abgeschlossen.

Vollständige Synthese:

`research/03-root-cause-synthesis/block-3.14a-power-i2c3-zustandsrekonstruktion.md`
