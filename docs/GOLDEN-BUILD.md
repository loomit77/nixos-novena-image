# Golden Build – funktionierendes Novena-Display

Stand: 2026-09-11

## Zweck

Dieses Dokument beschreibt den gesicherten und verifizierten Golden Build
des reproduzierbaren NixOS-Images für das Kosagi Novena.

Dieser Build ist besonders wichtig, weil er den aktuellen Linux-6.18.49-
Kernelstand mit funktionierendem internem Novena-Display repräsentiert.

Der Golden Build dient als bekannte funktionierende Referenz, auf die bei
späteren Kernel-, Device-Tree-, DRM-, Display- oder Bootloader-Experimenten
zurückgegriffen werden kann.

## Zugehöriger Git-Stand

Repository:

    ~/nixos-novena-image

Ursprünglicher Golden-Build-Commit:

    f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7

Kurzform:

    f26464f

Commit:

    Add reproducible Novena NixOS image project

Der exakte Repository-Inhalt dieses Commits wurde zusätzlich als
unabhängiges tar.gz-Archiv gesichert.

## Kernel

Kernel-Version:

    Linux 6.18.49

Gesicherter Kernel:

    kernel/zImage

SHA-256:

    30a63036889af44301f4821f76ca66a7b8f428c18e725ca5ec29775b6d5fbf5c

Der komplette Kernel-Ausgabebaum wurde im Golden Backup übernommen.
Dazu gehören insbesondere:

- zImage
- System.map
- die erzeugte DTB-Sammlung

## Novena Device Tree

Separat gesicherter Novena-DTB:

    dtb/imx6q-novena.dtb

SHA-256:

    e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f

Dieser DTB gehört zum funktionierenden Linux-6.18.49-Stand.

## Kernel-Konfiguration

Gesicherte Kernel-Konfiguration:

    config/linux-6.18.49.config

SHA-256:

    cef1365fe594704ec72394e8abbe2b96cc8ccad37090fcff42aa8d221aaa5ef3

Damit ist neben dem fertigen Kernel auch die Konfiguration des bekannten
funktionierenden Buildstands separat erhalten.

## Fertiges SD-Image

Gesichertes Image:

    image/nixos-novena-sd-image.img

SHA-256:

    5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b

Dieses Image entspricht dem am 2026-09-07 erfolgreich fertiggestellten
NixOS-Novena-Build.

Es ist die primäre binäre Wiederherstellungsreferenz für diesen Stand.

## Projektarchiv

Archiv des zugehörigen Git-Commits:

    metadata/nixos-novena-image-f26464f.tar.gz

SHA-256:

    75779166a971aba60003c0599294e3f69389bce789375e1e47d3baf8d295f6b3

Das Archiv wurde mit `git archive` direkt aus Commit `f26464f` erzeugt.

Dadurch existiert zusätzlich zu den Git-Remotes eine unabhängige Kopie
der exakten Projektquellen dieses Golden Builds.

## Golden-Backup-Pfad auf foobox

Lokaler Backup-Pfad:

    ~/novena-backups/nixos-display-working-2026-09-11/

Gesamtgröße nach Abschluss der Nix-Closure-Sicherung:

    6,2G

Aufbau:

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

## Integritätsprüfung

Für sämtliche normalen Dateien des Golden Backups wurde eine gemeinsame
SHA-256-Prüfsummenliste erzeugt:

    SHA256SUMS

Nach Abschluss aller Sicherungsschritte wurde das komplette Backup erneut
mit folgendem Befehl geprüft:

    sha256sum -c SHA256SUMS

Finales Ergebnis am 2026-09-11:

    Fehlerhafte Einträge: 0
    Geprüfte Dateien: 1398

Damit waren zum Zeitpunkt der Prüfung alle 1398 gesicherten Dateien
bitgenau konsistent mit der Prüfsummenliste.

## Nix-Store-Ursprung

Zum Zeitpunkt der Sicherung zeigten die Projekt-Symlinks auf folgende
Nix-Store-Objekte:

Image:

    /nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img

Kernel:

    /nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49

Novena-DTB:

    /nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb

Kernel-Konfiguration:

    /nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49

Die unmittelbaren Nix-Store-Referenzen von Image und Kernel wurden
zusätzlich gespeichert:

    metadata/image-references.txt
    metadata/kernel-references.txt

## Nix-Closure-Sicherung

Zusätzlich zu den fertigen Binärartefakten wurden die vorhandenen
Nix-Store-Closures exportiert.

### Image-Closure

Datei:

    nix/image-closure.nar

Größe:

    3833159224 bytes

Enthaltene Store-Pfade zum Zeitpunkt des Exports:

    605

SHA-256:

    8228f00e3f9f85046a55621727e82f70395125e75f00ee0fc497402879464b02

### Kernel-Closure

Der Kernel-Store-Pfad war nicht Bestandteil der zuvor ermittelten
Image-Closure und wurde deshalb separat exportiert.

Datei:

    nix/kernel-closure.nar

Größe:

    96724392 bytes

SHA-256:

    5f44cfedcdea2d98cb3b8ea4b56fae35e5c0894a8ca5beffb8f78e080e8b069f

Die Details der Closure-Sicherung sind zusätzlich dokumentiert in:

    metadata/nix-closure-info.txt

Die Archive können später in einen kompatiblen Nix Store importiert werden
mit:

    nix-store --import < archive.nar

Die Closure-Archive ergänzen die binären Golden-Build-Artefakte und die
Projektquellen. Sie ersetzen weder das fertige SD-Image noch das
Git-Repository.

## Display-relevante Projektdateien

Der Git-Stand enthält insbesondere die für den aktuellen Display-Port
wichtigen Dateien:

    kernel/0001-drm-bridge-it6251.patch
    kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch
    kernel/0003-i2c-imx-debug-arbitration-lost.patch
    kernel/it6251.c
    kernel/it6251.c.before-drm-lifecycle

Diese Dateien müssen zusammen mit `flake.nix`, `flake.lock`,
`hardware/novena.nix`, `image/novena-image.nix` und der Kernel-
Konfiguration als zusammengehöriger Stand betrachtet werden.

## Wiederherstellungsstrategie

Es existieren mehrere voneinander unabhängige Wiederherstellungswege.

### 1. Direkte binäre Wiederherstellung

Das fertig gebaute SD-Image kann auf einen geeigneten Datenträger
zurückgeschrieben werden.

Vorher muss dessen SHA-256 geprüft werden:

    5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b

Damit kann der bekannte Buildstand verwendet werden, ohne zuerst einen
neuen Kernel oder ein neues NixOS-Image bauen zu müssen.

### 2. Wiederherstellung der Projektquellen

Die Projektquellen sind durch folgende Ebenen abgesichert:

1. lokales Git-Repository auf foobox
2. Codeberg-Repository
3. GitHub-Repository
4. unabhängiges tar.gz-Archiv des Commits `f26464f`

`flake.lock` ist Teil des gespeicherten Git-Stands und fixiert die
verwendeten Flake-Inputs.

### 3. Wiederherstellung von Nix-Store-Ausgaben

Die exportierten Nix-Closures erlauben zusätzlich die Wiederherstellung
der gesicherten Store-Ausgaben, selbst wenn diese später nicht mehr aus
einem Binär-Cache verfügbar sein sollten.

Ein zukünftiger Neubau ist trotzdem erst dann als vollständig bestätigt
anzusehen, wenn das resultierende System erneut auf echter Novena-
Hardware getestet wurde.

## Sicherheitsregel für zukünftige Änderungen

Dieser Golden Build darf durch spätere Experimente nicht überschrieben
oder ersetzt werden.

Vor Änderungen an:

- Kernel
- DRM
- IT6251
- Panel-Unterstützung
- I2C
- Device Tree
- U-Boot
- SPL
- Bootskripten

muss dieser Build als bekannte Rückfallposition erhalten bleiben.

Neue erfolgreiche Stände sollen als neue Golden Builds gesichert werden,
anstatt diesen Stand zu verändern.

## Status

Stand 2026-09-11:

- Git-Quellen gesichert
- Codeberg-Remote vorhanden
- GitHub-Remote vorhanden
- fertiges SD-Image gesichert
- Kernel 6.18.49 gesichert
- Kernel-Konfiguration gesichert
- Novena-DTB gesichert
- komplette Kernel-DTB-Sammlung gesichert
- Git-Projektarchiv gesichert
- Nix-Referenzlisten gesichert
- Image-Closure gesichert
- Kernel-Closure gesichert
- SHA256SUMS neu erzeugt
- 1398 Dateien erfolgreich verifiziert
- vollständiges Golden Backup: 6,2G
- funktionierender Display-Stand als Golden Build konserviert
