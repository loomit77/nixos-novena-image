# Decisions

## 2026-09-11 – Eigenes Git-Repository

Das universelle NixOS-SD-Image wird als eigenständiges Projekt
versioniert.

Es wird nicht mit dem allgemeinen Novena-Systemrepository vermischt.

Begründung:

Das SD-Image soll als reproduzierbares Softwareprodukt unabhängig von
der individuellen Installation des aktuell verwendeten Novena
betrachtet werden.

## 2026-09-11 – Patches statt kompletter Kernelquellen

Komplette ausgepackte Kernelquellen werden nicht in Git gespeichert.

Dauerhaft relevante Änderungen müssen als nachvollziehbare Patchdateien
gesichert werden.

Aktuelle Patchserie:

- `0001-drm-bridge-it6251.patch`
- `0002-drm-panel-add-innolux-n133hse-ea1.patch`
- `0003-i2c-imx-debug-arbitration-lost.patch`

## 2026-09-11 – Referenz-Bootdateien werden versioniert

Die Dateien unter `boot/reference/` werden im Repository behalten.

Begründung:

Sie dokumentieren einen bekannten Novena-Bootzustand und sind klein
genug, um sinnvoll in Git gesichert zu werden.

Sie sind ausdrücklich nicht als aktuelle Build-Ergebnisse zu verstehen.

## 2026-09-11 – Nix result-Symlinks werden nicht versioniert

Folgende Einträge bleiben außerhalb von Git:

- `result`
- `result-kernel`
- `result-dtb`
- `result-kconfig`

Sie verweisen lediglich auf lokale Nix-Store-Ergebnisse.

## 2026-09-11 – Bootlogs vor Änderungen auswerten

Bei zukünftigen realen Boottests wird zuerst der vollständige serielle
Bootlog gesichert.

Erst nach dessen Auswertung werden Kernel, Device Tree, Bootskript oder
Image verändert.
