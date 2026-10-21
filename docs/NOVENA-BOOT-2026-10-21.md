# Novena-Bootstand – 2026-10-21

Die Angaben zu Block 190–192 stammen aus dem Arbeitsauftrag; die zugehörigen
Rohprotokolle lagen für diese Änderung im Repository nicht vor.

- **Block 190:** U-Boot bevorzugt als `bootable` markierte Partitionen.
- **Block 191:** Manueller Start von `boot.scr` auf `mmc 1:1`.
  Kernel und systemd starteten erfolgreich; `NIXOS_SD` auf `mmcblk1p2`
  wurde erfolgreich eingebunden. Die serielle Konsole zeigte:

  ```text
  Welcome to NixOS 26.05.20260903.a5cc6f2 (armv7l)
  novena login:
  ```

- **Block 192:** Die Fehlerstelle im Image-Builder wurde bestätigt:
  In der DOS-Partitionstabelle war Partition 2 statt Partition 1 als
  `bootable` markiert. In `image/novena-image.nix` erhält Partition 1
  (FAT32/FIRMWARE) diese Markierung; Partition 2 (ext4/NIXOS_SD) verliert sie.

Treibermeldungen zu IT6251, ES8328, USB und VPU bleiben separat zu bewerten.
Die Login-Anzeige belegt keinen getesteten Login; auch ein Netzwerktest ist
damit nicht belegt.

Automatischer SD-Boot nach der Korrektur ist noch ungetestet. Ein neuer
Image-Build und erneutes Flashen stehen noch aus.
