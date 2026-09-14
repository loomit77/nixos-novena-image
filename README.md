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

Stand: 2026-09-14

Aktueller Kernel:

- Linux 6.18.49

Aktuelle Patchserie:

1. `kernel/0001-drm-bridge-it6251.patch`
2. `kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`
3. `kernel/0003-i2c-imx-debug-arbitration-lost.patch`
4. `kernel/0004-i2c-imx-debug-start-state.patch`
5. `kernel/0005-i2c-imx-debug-start-error.patch`

Die Patches `0001` und `0002` bilden die Display-Unterstützung für
IT6251 und Innolux N133HSE-EA1 ab.

Die Patches `0003`, `0004` und `0005` sind Diagnoseinstrumentierung für
das aktuelle I2C-Problem und gehören nicht zu einem finalen
Produktionskernel.

Der aktuelle Diagnosefokus liegt auf einem reproduzierbaren
Cold-Boot-Fehler auf dem i.MX6-I2C-Bus.

Die bisherigen Messungen zeigen, dass der Fehler beim ersten
Master-START-Übergang auftritt:

- vor dem START ist der Controller aktiviert
- beim Setzen von MSTA wird unmittelbar `IAL` beobachtet
- `IBB` wird im Fehlerfall nicht stabil aufgebaut
- derselbe Bus funktioniert nach einem Rebind im selben Boot

Die nächste geplante Diagnosephase soll den MSTA/START-Übergang noch
feiner zeitlich auflösen.

Workarounds wie zusätzliche Delays, Retries, Bus-Recovery oder
Controller-Reset werden bis zur weiteren Eingrenzung bewusst nicht
eingebaut, damit die eigentliche Fehlerursache nicht verdeckt wird.

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

Ein reproduzierbarer Image-Build wurde erfolgreich abgeschlossen.

Build-Datum:

`2026-09-07`

Nix-Store-Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

Die späteren Kernel- und I2C-Diagnosearbeiten bauen auf diesem
reproduzierbaren Projektstand auf, dürfen aber nicht automatisch als
Bestandteil dieses ursprünglichen Image-Builds interpretiert werden.

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
- `boot/reference/`
  Referenzdateien eines bekannten Novena-Bootzustands
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

Der zum Stand dieses Dokuments aktuelle Commit ist:

`35357fd405259a24f18c86a6fd0dc673c98a12f5`

Commit-Beschreibung:

`Document and instrument Novena I2C START failure`

Der Stand wurde auf beiden verwendeten Remotes verifiziert:

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
