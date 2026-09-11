# Universal NixOS SD Image for Novena

Dieses Repository enthält den aktuellen Entwicklungsstand eines
reproduzierbaren NixOS-SD-Images für das Kosagi Novena.

## Ziel

Das Image soll auf Novena-Systemen booten, ohne von einem bestimmten
bereits installierten Novena-System abhängig zu sein.

Dazu werden insbesondere benötigt:

- passender U-Boot/SPL-Bootpfad
- Novena Device Tree
- aktueller ARMv7-Kernel
- IT6251 Display-Bridge-Unterstützung
- Unterstützung für das verbaute Innolux-Panel
- reproduzierbarer NixOS-Image-Build

## Aktueller Stand

Stand: 2026-09-11

Aktueller Kernel:

- Linux 6.18.49

Aktuelle Patchserie:

1. `kernel/0001-drm-bridge-it6251.patch`
2. `kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`
3. `kernel/0003-i2c-imx-debug-arbitration-lost.patch`

Der aktuelle Image-Build wurde am 2026-09-07 erfolgreich abgeschlossen.

Nix-Store-Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

## Repository-Struktur

- `configuration.nix`
  zentrale NixOS-Konfiguration

- `flake.nix`
  Flake-Einstiegspunkt

- `flake.lock`
  festgelegte Nix-Abhängigkeiten

- `hardware/novena.nix`
  Novena-Hardwarekonfiguration

- `image/novena-image.nix`
  Definition des SD-Images

- `kernel/`
  Kernelpatches und Entwicklungsdateien

- `boot/reference/`
  Referenzdateien eines bekannten Novena-Bootzustands

- `novena-i2c-test/`
  ARMv7-Testpaket für I2C-Diagnose

- `docs/`
  Projektzustand, Image-Aufbau, Build- und Testdokumentation

## Nicht versionierte Dateien

Die folgenden Arbeitsartefakte werden bewusst nicht in Git gespeichert:

- `result`
- `result-kernel`
- `result-dtb`
- `result-kconfig`
- ausgepackte Linux-Kernelquellen

Diese Dateien können aus dem dokumentierten Build reproduziert werden.

## Verwandtes Projekt

Die allgemeine Dokumentation der realen Novena-Hardware, Bootpfade,
Recovery-Strategie und historischen Systeme befindet sich separat unter:

`~/novena-system`
