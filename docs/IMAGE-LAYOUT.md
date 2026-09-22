# Image Layout

Stand: 2026-09-22

## Historische Referenz

Das Projekt entstand aus der Analyse eines bekannten
Novena-NixOS-Images.

Die historischen Bootdateien unter `boot/reference/` bleiben zur
Provenienz und zum Vergleich im Repository erhalten. Sie sind jedoch
nicht mehr die aktive Quelle des produktiven Bootloaders.

## Partitionstabelle des aktuellen Images

Das aktuelle Image verwendet eine DOS/MBR-Partitionstabelle.

Disk-ID:

`0x2178694e`

Logische Sektorgröße:

`512` Byte

Gesamtgröße:

`2581291008` Byte

Gesamtzahl der Sektoren:

`5041584`

### Partition 1

Typ:

`W95 FAT32 (LBA)`, MBR-Typ `0x0c`

Label:

`FIRMWARE`

Start:

Sektor `16384`

Größe:

`262144` Sektoren = `128 MiB`

Die FAT-Partition enthält die Bootdateien für U-Boot.

### Partition 2

Typ:

Linux, MBR-Typ `0x83`

Label:

`NIXOS_SD`

Start:

Sektor `278528`

Größe:

`4761008` Sektoren

Die Partition ist im MBR als bootfähig markiert.

Sie enthält das ext4-Root-Dateisystem des NixOS-Systems.

## Bereich vor der ersten Partition

Der Bereich vor Sektor `16384` enthält den SPL.

Der projektlokal gebaute SPL wird ab Byte-Offset:

`1024`

in das Image geschrieben.

Aktueller SPL:

* Größe: `52224` Byte
* SHA-256:
  `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0`

Der SPL basiert auf U-Boot `v2026.07` mit den projektlokalen
Novena-Anpassungen und ist für den externen V1-Pfad auf USDHC2
ausgerichtet.

## Inhalt der FIRMWARE-Partition

Die statische Prüfung des aktuellen qualifizierten Images ergab:

* `u-boot-dtb.img`
* `zImage`
* `initrd.uimg`
* `novena.dtb`
* `boot.cmd`
* `boot.scr`

Das eingebettete `u-boot-dtb.img` ist das projektlokal gebaute
U-Boot-v2026.07-Artefakt:

* Größe: `602120` Byte
* SHA-256:
  `c5a2d0e7f2b9ebeae6387eee98630a43944f2cd2b52fc8a5a98a2e82ba55f9f7`

Weitere beim statischen Test extrahierte Artefakte:

### Kernel

Datei:

`zImage`

Größe:

`17457664` Byte

SHA-256:

`20a10cebceb35e44440f626a80d1b69245b21d917023dc31998841569af21bc6`

### Initrd

Datei:

`initrd.uimg`

Größe:

`27925712` Byte

SHA-256:

`55609b167c97ee15afb06075b4b3e8127a60e8f1c0298960fc1849843b32a3bc`

### Device Tree

Datei:

`novena.dtb`

Größe:

`59628` Byte

SHA-256:

`d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1`

## Bootskript

Das aktuelle `boot.scr` setzt den NixOS-Systempfad und lädt alle
Bootdateien ausdrücklich von der externen SD über `mmc 1:1`.

Verwendete Ladeadressen:

* Kernel: `0x12000000`
* Initrd: `0x20000000`
* Device Tree: `0x1f000000`

Der abschließende Start erfolgt mit:

`bootz 0x12000000 0x20000000 0x1f000000`

Der im statisch qualifizierten Image enthaltene Systemabschluss ist:

`/nix/store/axbsx57xnrbqdya5bk877g21g87kish7-nixos-system-novena-26.05.20260903.a5cc6f2`

## Aktuelles qualifiziertes Image

Nix-Store-Pfad:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Größe:

`2581291008` Byte

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Ein vollständiger Neubau aus demselben Git-Commit in einem separaten
isolierten Nix-Store erzeugte exakt dieselbe Größe und denselben
SHA-256-Wert.

Ein direkter `cmp`-Vergleich der beiden Images ergab Byteidentität.

## Historische Referenz-Bootdateien

Unter `boot/reference/` befinden sich weiterhin:

* `novena.dtb`
* `novena-imx6-spl.bin`
* `u-boot-dtb.img`
* `zImage`

Diese Dateien dokumentieren einen bekannten historischen
Novena-Bootzustand.

Sie dürfen nicht mit den aktiven, aus dem aktuellen Projekt gebauten
Bootartefakten verwechselt werden.

## Hardware-Aussagegrenze

Die statische Image-Prüfung bestätigt den Aufbau und die eingebetteten
Artefakte.

Noch nicht durch einen realen Hardwaretest bestätigt ist die vollständige
Bootkette:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

Dieser P_EXT-Test bleibt der nächste Hardware-Nachweis.
