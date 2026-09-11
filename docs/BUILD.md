# Build

## Build-System

Primärer Entwicklungsrechner:

`foobox`

Hostarchitektur:

`x86_64-linux`

Zielarchitektur:

`armv7l-linux`

Das Image wird mit Nix gebaut.

## Flake

Projektwurzel:

`~/nixos-novena-image`

Die Abhängigkeiten sind in:

- `flake.nix`
- `flake.lock`

festgelegt.

## Build-Ergebnisse

Aktueller Kernel:

`6.18.49`

Aktuelles Kernelresultat:

`result-kernel`

Aktueller Device Tree:

`result-dtb`

Aktuelle Kernelkonfiguration:

`result-kconfig`

Aktuelles SD-Image:

`result`

Die `result*`-Einträge sind Nix-Symlinks und werden nicht in Git
versioniert.

## Aktuell erfolgreicher Image-Build

Build abgeschlossen:

2026-09-07

Exit-Code:

`0`

Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

## Kernel-Arbeitsverzeichnisse

Während der Entwicklung existieren lokal unter anderem:

- `kernel/linux-6.18.49`
- `kernel/linux-6.18.49-orig`
- `kernel/panel-patch`

Diese Verzeichnisse sind Arbeitskopien und werden nicht versioniert.

Die dauerhaft relevanten Änderungen müssen als Patchdateien unter
`kernel/` vorliegen.
