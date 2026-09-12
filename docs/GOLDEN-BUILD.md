# Golden Build – funktionierendes Novena-Display

Stand: 2026-09-12

## Zweck

Dieses Dokument beschreibt den gesicherten und verifizierten Golden Build
des reproduzierbaren NixOS-Images für das Kosagi Novena.

Der Golden Build dient als bekannte funktionierende Referenz, auf die bei
späteren Kernel-, Device-Tree-, DRM-, Display-, I2C- oder
Bootloader-Experimenten zurückgegriffen werden kann.

Es ist wichtig, zwei Ebenen voneinander zu unterscheiden:

1. die am 2026-09-11 angelegte und verifizierte Golden-Build-Sicherung
2. die am 2026-09-12 erfolgte erfolgreiche Hardware-Validierung des
   aktuellen Images auf echter Novena-Hardware

Die Sicherung vom 2026-09-11 wird nicht verändert oder überschrieben.

## Zugehöriger Git-Stand der ursprünglichen Golden-Build-Sicherung

Repository:

```
~/nixos-novena-image
```

Ursprünglicher Golden-Build-Commit:

```
f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7
```

Kurzform:

```
f26464f
```

Commit:

```
Add reproducible Novena NixOS image project
```

Der exakte Repository-Inhalt dieses Commits wurde zusätzlich als
unabhängiges tar.gz-Archiv gesichert.

## Zusätzlicher Git-Sicherungspunkt vom 2026-09-12

Nach der erfolgreichen Hardware-Validierung wurde ein zusätzlicher
annotierter Git-Tag angelegt:

```
novena-display-working-2026-09-12
```

Der Tag zeigt auf Commit:

```
1d43425
```

Commit-Beschreibung:

```
Document Nix closure backup for golden build
```

Dieser Tag markiert den aktuell bestätigten funktionierenden
Display-Zwischenstand vor weiteren Änderungen an der
I2C-Diagnoseinstrumentierung.

## Kernel

Kernel-Version:

```
Linux 6.18.49
```

Gesicherter Kernel:

```
kernel/zImage
```

SHA-256:

```
30a63036889af44301f4821f76ca66a7b8f428c18e725ca5ec29775b6d5fbf5c
```

Der komplette Kernel-Ausgabebaum wurde im Golden Backup übernommen.

Dazu gehören insbesondere:

* zImage
* System.map
* die erzeugte DTB-Sammlung

## Novena Device Tree

Separat gesicherter Novena-DTB:

```
dtb/imx6q-novena.dtb
```

SHA-256:

```
e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f
```

Dieser DTB gehört zum gesicherten Linux-6.18.49-Stand.

## Kernel-Konfiguration

Gesicherte Kernel-Konfiguration:

```
config/linux-6.18.49.config
```

SHA-256:

```
cef1365fe594704ec72394e8abbe2b96cc8ccad37090fcff42aa8d221aaa5ef3
```

Damit ist neben dem fertigen Kernel auch die Konfiguration des bekannten
Buildstands separat erhalten.

## Fertiges SD-Image

Gesichertes Image:

```
image/nixos-novena-sd-image.img
```

SHA-256:

```
5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b
```

Dieses Image entspricht dem am 2026-09-07 erfolgreich fertiggestellten
NixOS-Novena-Build.

Nix-Store-Ursprung:

```
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

Dieses Image wurde am 2026-09-12 auf echter Novena-Hardware erfolgreich
mit funktionierender Linux-seitiger Displayinitialisierung getestet.

## Hardware-Validierung am 2026-09-12

Der aktuelle Build wurde am 2026-09-12 auf echter Novena-Hardware kalt
gestartet.

Der Test erreichte:

* SPL/U-Boot
* Linux 6.18.49
* NixOS Stage 1
* Root-Dateisystem
* regulären Login
* erfolgreiche Linux-seitige IT6251-Stromversorgung
* erfolgreiche IT6251-Product-ID-Erkennung
* erfolgreiche IT6251-Initialisierung
* erfolgreiches DisplayPort-Linktraining
* stabile interne Displayausgabe mit 1920x1080

Die Product-ID wurde bereits beim ersten Versuch korrekt gelesen.

Gelesene Register:

```
0x00 = 0x15
0x01 = 0xca
0x02 = 0x51
0x03 = 0x62
```

Kernelmeldung:

```
IT6251 detected: vendor ca15 device 6251
```

## Display-Link

Die IT6251-Initialisierung war bereits im ersten Versuch erfolgreich.

Das Linktraining endete nach zehn Iterationen.

Systemstatus:

```
0x3e
```

Gemessene aktive Auflösung:

```
hactive: 1920
vactive: 1080
```

Erfolgsindikatoren:

```
is_stable: stable 1920x1080
display link stable
bridge_enable: exit success
```

Damit ist bestätigt, dass der aktuelle Linux-6.18.49-Stand den internen
Novena-Displaypfad grundsätzlich vollständig initialisieren kann.

## Regulatorstatus nach erfolgreicher Initialisierung

Nach dem erfolgreichen Displaystart wurden die GPIO-Zustände geprüft.

Ergebnis:

```
gpio-15 (regulator-lvds-lcd) out hi
gpio-28 (regulator-display) out hi
```

Regulator-Zusammenfassung:

```
lcd-lvds-power
```

* aktiv
* 3300 mV
* Backlight-Consumer aktiv

und:

```
lcd-display-power
```

* aktiv
* 3300 mV
* Consumer `2-005c-power` aktiv

Damit waren sowohl Display- als auch Backlight-Versorgung aktiv.

## Relevante Device-Tree-Korrekturen

Der aktuell funktionierende Displaystand enthält unter anderem:

* LDB-Clock-Zuweisung auf PLL2 PFD2 396 MHz
* `single-master;` auf dem Display-I2C-Bus
* Novena-spezifisches IT6251-Pinmux
* Novena-spezifisches Backlight-Pinmux
* `startup-delay-us = <2000000>` für `reg_display`
* kein `regulator-always-on` auf `reg_display`

Die Clock-Korrektur wurde durch stabile Erkennung von 1920x1080
bestätigt.

`single-master;` beseitigte die zuvor beobachteten
Arbitration-Lost-/EAGAIN-Fehler auf dem Display-I2C-Bus.

## Erwartete IT6251-Reset-NACKs

Während der IT6251-Reset-Sequenzen wurden temporär folgende Meldungen
beobachtet:

```
error -6 writing to eDP addr 0x5
error -6 writing to LVDS addr 0x5
```

Diese Meldungen entsprechen dem bekannten Verhalten während der
IT6251-Resetsequenz und verhinderten den erfolgreichen Displaystart
nicht.

Die Initialisierung lief anschließend vollständig weiter und endete mit
einem stabilen 1920x1080-Link.

## Aktuelle Einschränkung

Der erfolgreiche Test vom 2026-09-12 wurde mit einem stark
instrumentierten I2C-Diagnose-Kernel durchgeführt.

Insbesondere enthält der aktuelle Kernel zusätzliche Logausgaben direkt
im i.MX-I2C-Interruptpfad.

Diese Instrumentierung kann das Timing des Systems beeinflussen.

Deshalb ist der Stand zwar als funktionierender Entwicklungsstand
bestätigt, aber noch nicht als endgültig reproduzierbarer
Produktionsstand einzustufen.

## Separates i2c-0-Diagnoseproblem

Der aktuelle Diagnose-Patch protokolliert Arbitration-Lost-Ereignisse
auch auf anderen i.MX-I2C-Controllern.

Auf `i2c-0` wurden nach dem Boot große Mengen folgender Meldung erzeugt:

```
NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
```

Zusätzlich wurde beobachtet:

```
<i2c_imx_write> write timedout
```

Dieses Problem ist getrennt vom erfolgreichen IT6251-Betrieb auf
`i2c-2` zu betrachten.

Vor weiteren Reproduzierbarkeitstests muss der Diagnose-Patch
entschärft werden.

## Projektarchiv

Archiv des ursprünglichen Golden-Build-Git-Commits:

```
metadata/nixos-novena-image-f26464f.tar.gz
```

SHA-256:

```
75779166a971aba60003c0599294e3f69389bce789375e1e47d3baf8d295f6b3
```

Das Archiv wurde mit `git archive` direkt aus Commit `f26464f` erzeugt.

Dadurch existiert zusätzlich zu den Git-Remotes eine unabhängige Kopie
der exakten Projektquellen dieses ursprünglichen Golden-Build-Stands.

## Golden-Backup-Pfad auf foobox

Lokaler Backup-Pfad:

```
~/novena-backups/nixos-display-working-2026-09-11/
```

Gesamtgröße nach Abschluss der Nix-Closure-Sicherung:

```
6,2G
```

Aufbau:

```
nixos-display-working-2026-09-11/
├── SHA256SUMS
├── config/
│   └── linux-6.18.49.config
├── dtb/
│   └── imx6q-novena.dtb
├── image/
│   └── nixos-novena-sd-image.img
├── kernel/
│   ├── System.map
│   ├── zImage
│   └── dtbs/
├── metadata/
│   ├── build-info.txt
│   ├── image-references.txt
│   ├── kernel-references.txt
│   ├── nix-closure-info.txt
│   └── nixos-novena-image-f26464f.tar.gz
└── nix/
    ├── image-closure.nar
    └── kernel-closure.nar
```

## Integritätsprüfung

Für sämtliche normalen Dateien des Golden Backups wurde eine gemeinsame
SHA-256-Prüfsummenliste erzeugt:

```
SHA256SUMS
```

Nach Abschluss aller Sicherungsschritte wurde das komplette Backup erneut
mit folgendem Befehl geprüft:

```
sha256sum -c SHA256SUMS
```

Finales Ergebnis am 2026-09-11:

```
Fehlerhafte Einträge: 0
Geprüfte Dateien: 1398
```

Damit waren zum Zeitpunkt der Prüfung alle 1398 gesicherten Dateien
bitgenau konsistent mit der Prüfsummenliste.

## Nix-Store-Ursprung

Zum Zeitpunkt der Sicherung zeigten die Projekt-Symlinks auf folgende
Nix-Store-Objekte.

Image:

```
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

Kernel:

```
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Novena-DTB:

```
/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb
```

Kernel-Konfiguration:

```
/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49
```

Die unmittelbaren Nix-Store-Referenzen von Image und Kernel wurden
zusätzlich gespeichert:

```
metadata/image-references.txt
metadata/kernel-references.txt
```

## Nix-Closure-Sicherung

Zusätzlich zu den fertigen Binärartefakten wurden die vorhandenen
Nix-Store-Closures exportiert.

### Image-Closure

Datei:

```
nix/image-closure.nar
```

Größe:

```
3833159224 bytes
```

Enthaltene Store-Pfade zum Zeitpunkt des Exports:

```
605
```

SHA-256:

```
8228f00e3f9f85046a55621727e82f70395125e75f00ee0fc497402879464b02
```

### Kernel-Closure

Der Kernel-Store-Pfad war nicht Bestandteil der zuvor ermittelten
Image-Closure und wurde deshalb separat exportiert.

Datei:

```
nix/kernel-closure.nar
```

Größe:

```
96724392 bytes
```

SHA-256:

```
5f44cfedcdea2d98cb3b8ea4b56fae35e5c0894a8ca5beffb8f78e080e8b069f
```

Die Details der Closure-Sicherung sind zusätzlich dokumentiert in:

```
metadata/nix-closure-info.txt
```

Die Archive können später in einen kompatiblen Nix Store importiert
werden mit:

```
nix-store --import < archive.nar
```

Die Closure-Archive ergänzen die binären Golden-Build-Artefakte und die
Projektquellen.

Sie ersetzen weder das fertige SD-Image noch das Git-Repository.

## Display-relevante Projektdateien

Der aktuelle Projektstand enthält insbesondere:

```
kernel/0001-drm-bridge-it6251.patch
kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch
kernel/0003-i2c-imx-debug-arbitration-lost.patch
kernel/it6251.c
kernel/it6251.c.before-drm-lifecycle
```

Diese Dateien müssen zusammen mit `flake.nix`, `flake.lock`,
`hardware/novena.nix`, `image/novena-image.nix` und der
Kernel-Ko
