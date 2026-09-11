# Project State

Stand: 2026-09-11

## Ziel

Erstellung eines universellen und reproduzierbaren NixOS-SD-Images für
das Kosagi Novena.

Das Image soll die für das Novena notwendigen Boot-, Kernel- und
Hardwarekomponenten enthalten und auf anderen Novena-Geräten verwendbar
sein.

## Entwicklungsrechner

Primäre Build-Maschine:

- Host: foobox
- Betriebssystem: CachyOS
- Architektur: x86_64

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

### 0002

`kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

Ziel:

Unterstützung des Innolux N133HSE-EA1 Panels.

### 0003

`kernel/0003-i2c-imx-debug-arbitration-lost.patch`

Ziel:

Zusätzliche Diagnose für I2C-Arbitration-Lost-Probleme auf dem Novena.

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

## Aktuelles Testmedium

Auf der foobox wurde am 2026-09-11 eine 29.7-GiB-SD-Karte erkannt.

Partitionierung:

### Partition 1

- Gerät auf foobox: `/dev/sda1`
- Größe: 128 MiB
- Dateisystem: FAT
- Label: `FIRMWARE`
- UUID: `2178-694E`
- PARTUUID: `2178694e-01`

### Partition 2

- Gerät auf foobox: `/dev/sda2`
- Größe: 2.3 GiB
- Dateisystem: ext4
- Label: `NIXOS_SD`
- UUID: `44444444-4444-4444-8888-888888888888`
- PARTUUID: `2178694e-02`

## Aktueller Arbeitsstand

Der aktuelle 6.18.49-Build ist erfolgreich erzeugt.

Der entscheidende nächste Schritt ist der reale Boot-Test auf dem
Novena.

Beim Boot-Test müssen insbesondere geprüft werden:

1. SPL/U-Boot startet korrekt.
2. Kernel wird geladen.
3. Device Tree wird geladen.
4. initrd bzw. NixOS Stage 1 startet.
5. Root-Dateisystem wird gefunden.
6. I2C-Verhalten wird protokolliert.
7. IT6251 wird erkannt.
8. Panel wird erkannt.
9. Displayinitialisierung wird bewertet.
10. vollständiger serieller Bootlog wird gespeichert.

## Sicherheitsregel

Ein fehlgeschlagener Boot-Test darf nicht durch spontane Änderungen am
Testmedium überschrieben werden.

Zuerst wird der serielle Log gesichert und ausgewertet.

Erst danach wird die nächste Änderung vorgenommen.
