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
- reproduzierbare x86_64-zu-ARMv7-Cross-Build-Umgebung

## Aktueller Stand

Stand: 2026-09-22

Aktueller produktiver Kernel:

- Linux 6.18.49

Produktive Kernel-Patchserie:

1. `kernel/0001-drm-bridge-it6251.patch`
2. `kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

Die Patches bilden die Display-Unterstützung für IT6251 und
Innolux N133HSE-EA1 ab.

Die frühere I2C-Diagnoseinstrumentierung gehört nicht mehr zum
produktiven Kernelstand.

Die funktionale ES8328/I2C3-Root-Cause-Untersuchung ist abgeschlossen.
Für Novena gilt dauerhaft:

`es8328-power = regulator-always-on`

Der aktuelle Entwicklungsschwerpunkt ist das universelle und
reproduzierbare externe NixOS-SD-Image.

Der projektlokale U-Boot `v2026.07` ist in das Image integriert.
Der vollständige Image-Build wurde auf der foobox zusätzlich in einem
getrennten isolierten Nix-Store wiederholt. Normal-Store- und
Clean-Store-Image waren byteidentisch.

Aktuelles qualifiziertes Image:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Größe:

`2581291008` Byte

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Noch nicht hardwareseitig qualifiziert ist der vollständige
eigenständige externe Bootpfad:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

Dieser vorregistrierte P_EXT-Test ist der nächste Hardware-Nachweis.

## Golden Build

Ein funktionierender Display-Zwischenstand ist dokumentiert und
gesichert.

Detaillierte Informationen befinden sich in:

`docs/GOLDEN-BUILD.md`

Dort sind insbesondere dokumentiert:

- funktionierender Kernel
- DTB-Referenzen
- SHA-256-Werte
- Nix-Store-Ausgaben
- Golden-Backup
- Git-Sicherungspunkt
- Abgrenzung zwischen Golden Build und späteren Diagnosekerneln

## Aktuelles Image

Das aktuell qualifizierte Image wurde aus Commit

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

gebaut.

Nix-Store-Ausgabe:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Größe:

`2581291008` Byte

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Das Image enthält den projektlokal gebauten U-Boot `v2026.07`.

Ein vollständiger Neubau in einem getrennten isolierten Nix-Store
erzeugte ein byteidentisches Image mit demselben SHA-256-Wert.

Der historische `result`-Symlink bleibt bewusst unverändert und dient
weiterhin als Referenz auf den früheren Golden Build.

Der reale P_EXT-Hardwaretest dieses aktuellen Images steht noch aus.

## Build-Host

Primäre Build-Maschine:

`foobox`

Betriebssystem:

`CachyOS`

Architektur:

`x86_64`

Die foobox wird für native Nix-Builds und für
x86_64-zu-ARMv7-Cross-Compilation verwendet.

Die vollständige und verifizierte Build-Host-Konfiguration ist
dokumentiert unter:

`docs/BUILD-HOST.md`

Dort sind unter anderem festgehalten:

- CPU und RAM
- Nix-Version
- dauerhafte Experimental Features
- `armv7l-linux`
- ARMv7-Hard-Float-Cross-Toolchain
- serielle FT232R-Anbindung
- persistenter serieller Gerätepfad
- NVMe-Zustand
- relevante Nix-Build-Einstellungen

Die systemweite Nix-Konfiguration enthält dauerhaft:

```text
experimental-features = fetch-tree flakes nix-command
extra-platforms = armv7l-linux
extra-system-features = gccarch-armv7-a
```

Damit sind zusätzliche Kommandozeilenparameter wie:

`--extra-experimental-features 'nix-command flakes'`

für die normalen Projektbefehle nicht mehr erforderlich.

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
- `boot/u-boot/`
  reproduzierbarer projektlokaler U-Boot-v2026.07-Baustein
- `boot/reference/`
  historische Referenzdateien eines bekannten Novena-Bootzustands
- `overlays/`
  projektlokale Nixpkgs-Overrides, darunter der Cross-only-Bash-Fix
- `patches/bash/`
  Patch für den deterministischen Bash-Cross-Build
- `novena-i2c-test/`
  ARMv7-Testpaket für I2C-Diagnose
- `docs/BUILD.md`
  Build-Dokumentation
- `docs/BUILD-HOST.md`
  verifizierte foobox-Build-Host-Konfiguration
- `docs/DECISIONS.md`
  dokumentierte Projektentscheidungen
- `docs/GOLDEN-BUILD.md`
  gesicherter funktionierender Display-Referenzstand
- `docs/IMAGE-LAYOUT.md`
  Image- und Partitionsaufbau
- `docs/PROJECT-STATE.md`
  aktueller technischer Projektzustand
- `docs/TEST-LOG.md`
  Hardware- und Diagnosetests

## Nicht versionierte Arbeitsartefakte

Die folgenden Arbeitsartefakte werden bewusst nicht in Git gespeichert:

- `result`
- `result-kernel`
- `result-dtb`
- `result-kconfig`
- weitere temporäre `result-*`-Symlinks
- ausgepackte Linux-Kernelquellen

Diese Dateien können aus dem dokumentierten Build reproduziert werden.

Temporäre Sicherungs- und Zwischenstände werden nicht dauerhaft im
Working Tree gesammelt.

Vor ihrer Entfernung werden relevante Dateien separat gesichert und
überprüft.

## Git-Stand

Der aktuelle qualifizierte Softwarecheckpoint ist:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

Commit-Beschreibung:

`Make cross-built Bash reproducible`

Der Commit ist auf beiden verwendeten Remotes synchronisiert und verifiziert:

- Codeberg
- GitHub

## Dokumentation

Der jeweils aktuelle technische Zustand wird nicht ausschließlich aus
dem README abgeleitet.

Für detaillierte und überprüfte Informationen sind insbesondere folgende
Dateien maßgeblich:

- `docs/PROJECT-STATE.md`
- `docs/GOLDEN-BUILD.md`
- `docs/TEST-LOG.md`
- `docs/BUILD-HOST.md`
- `docs/DECISIONS.md`

Das README dient als Einstieg und Überblick.

## Verwandtes Projekt

Die allgemeine Dokumentation der realen Novena-Hardware, Bootpfade,
Recovery-Strategie und historischen Systeme befindet sich separat unter:

`~/novena-system`
