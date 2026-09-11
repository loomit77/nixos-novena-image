# Image Layout

## Referenz

Das Projekt basiert auf der Analyse eines bekannten Novena-NixOS-Images.

Das ursprüngliche Referenzimage besaß eine DOS/MBR-Partitionstabelle mit
der Disk-ID:

`0x2178694e`

## Referenzpartitionen

### Partition 1

Label:

`FIRMWARE`

Dateisystem:

FAT

Größe beim ursprünglichen Referenzimage:

ungefähr 30 MiB

Start:

Sektor 16384

PARTUUID:

`2178694e-01`

UUID:

`2178-694E`

### Partition 2

Label:

`NIXOS_SD`

Dateisystem:

ext4

PARTUUID:

`2178694e-02`

UUID:

`44444444-4444-4444-8888-888888888888`

## Bereich vor der ersten Partition

Im ursprünglichen Referenzimage befanden sich Bootloader-Daten vor der
ersten Partition.

Aus diesem Bereich wurden unter anderem U-Boot/SPL-Strings gefunden.

Die korrekte Reproduktion dieses Bereichs ist entscheidend dafür, dass
das Novena direkt von der SD-Karte booten kann.

## Referenz-Bootdateien

Unter `boot/reference/` befinden sich:

- `novena.dtb`
- `novena-imx6-spl.bin`
- `u-boot-dtb.img`
- `zImage`

Diese Dateien dienen als bekannte Referenz und dürfen nicht mit den
aktuellen Build-Ergebnissen verwechselt werden.

## Historischer FIRMWARE-Inhalt

Beim analysierten Referenzimage waren unter anderem vorhanden:

- `boot.cmd`
- `boot.scr`
- `initrd.uimg`
- `novena.dtb`
- `u-boot-dtb.img`
- `zImage`

Historisch bekannte Ladeadressen:

- Kernel: `0x12000000`
- initrd: `0x13000000`
- Device Tree: `0x11ff0000`

Boot erfolgte anschließend mit `bootz`.

## Aktuelles Image

Das aktuelle Projekt erzeugt ein neues NixOS-SD-Image aus den Dateien:

- `configuration.nix`
- `hardware/novena.nix`
- `image/novena-image.nix`
- aktuellem Kernel
- aktueller Patchserie

Der genaue Image-Aufbau ist in `image/novena-image.nix` definiert.
