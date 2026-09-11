# Test Log

## Frühere NixOS-Tests

Ein älterer NixOS-Ansatz verwendete den Novena-spezifischen Kernel:

`5.7.0-rc2`

Ein früherer Bootversuch erreichte den Kernel, endete jedoch mit:

`VFS: Unable to mount root fs on unknown-block(0,0)`

Damit war nachgewiesen, dass Kernel und Device Tree grundsätzlich
gestartet werden konnten, die Root-/Initrd-Kette aber noch nicht
korrekt war.

## Analyse des bekannten funktionierenden Images

Das bekannte Novena-NixOS-Image wurde untersucht.

Festgestellt wurden:

- Bootloaderbereich vor Partition 1
- FAT-Partition `FIRMWARE`
- ext4-Partition `NIXOS_SD`
- SPL/U-Boot
- Kernel
- Device Tree
- initrd
- U-Boot-Bootskript

Diese Erkenntnisse bilden die Grundlage für das neue universelle Image.

## Linux 6.18.49

Für das aktuelle Image wurde Linux 6.18.49 verwendet.

Patchserie:

1. IT6251 DRM Bridge
2. Innolux N133HSE-EA1 Panel
3. I2C-IMX Debugging für Arbitration Lost

## 2026-09-07

Aktuelles NixOS-SD-Image erfolgreich gebaut.

Exit-Code:

`0`

SHA-256 des Build-Ergebnisses:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

## Nächster Test

Realer Boot-Test des aktuellen Images auf dem Novena.

Dabei vollständigen seriellen Bootlog erfassen.

Bis zur Auswertung dieses Logs keine Schlussfolgerung über den
tatsächlichen Hardwarestatus des 6.18.49-Kernels ziehen.
