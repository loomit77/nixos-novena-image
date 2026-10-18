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

Dokumentarischer Wegweiser vom 2026-10-11: Maßgeblich ist der
[DDR-A-B-B-A-Testcheckpoint vom 2026-10-07](docs/PROJECT-STATE.md#maßgeblicher-ddr-testcheckpoint--block-9c-52q-r7b-3aaj-2026-10-07).
Alle vier Läufe zeigen Q1/C1 ZERO vor MMC1 und D32 `ret=-12`.
Patch0012 wird unter diesen Testbedingungen nicht als isolierter Fix
unterstützt; die DDR-Root-Cause bleibt offen. Die vollständige Provenienz
und Aussagegrenzen stehen im
[Experimentnachtrag](docs/TEST-LOG.md#2026-10-07--block-9c-52q-r7b-3aaj-frisches-a-b-b-a-experiment).

Der [formale qualifizierte Projektindexabschluss vom 2026-10-09](docs/PROJECT-STATE.md#2026-10-09--index-1k-65-formaler-qualifizierter-projektindex-abschluss)
ist ein separater Dokumentations-/Abnahmecheckpoint für den genehmigten
endlichen Indexumfang. Er belegt keine DDR-, SPL- oder Hardwarequalifikation.

Der ältere [Dokumentationscheckpoint vom 02.10.](docs/PROJECT-STATE.md#2026-10-02--block-9c-44-statischer-dokumentationscheckpoint)
ordnet den einmaligen Patch-0008-Test vom 01.10. mit
`8B = CONTROL_FAIL_TARGET_FAIL` ein und bleibt als historischer Nachweis erhalten.

Die nachfolgenden G2X-Buildangaben beschreiben die historische Referenz
vom 2026-09-22 und keine neueren Diagnoseimages.

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

Der historische G2X-Referenzbuild enthält den projektlokalen U-Boot
`v2026.07`. Seine Image-Identität und der bytegenaue Same-Host-/Separate-
Store-Nachweis stehen unten unter „Historisches G2X-Referenzimage“.

Der reale P_EXT-Hardwaretest hat inzwischen den ersten Teil des
eigenständigen externen Bootpfads bestätigt:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07`

Noch nicht hardwareseitig nachgewiesen ist der anschließende Pfad:

`SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

Der beobachtete MMC-Endpunkt folgt auf die vorgelagerte Q1/C1-Zero-Signatur.
Er erklärt diese Signatur nicht und belegt keine DDR-Ursache.

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

## Historisches G2X-Referenzimage – 2026-09-22

Das historisch qualifizierte G2X-Image wurde aus Commit

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

Der reale P_EXT-Hardwaretest dieses Stands wurde begonnen. Der i.MX6-ROM-Code
lädt den projektlokalen SPL von der externen SD; U-Boot proper v2026.07 wurde
über diesen eigenständigen P_EXT-Pfad bisher noch nicht erreicht.

## Build-Host

Primäre Build-Maschine:

`foobox`

Betriebssystem:

`CachyOS`

Architektur:

`x86_64`

Die foobox wird für native Nix-Builds und für
x86_64-zu-ARMv7-Cross-Compilation verwendet.

Die datierte historische Build-Host-Bestandsaufnahme mit separaten Nachträgen ist
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
  datierte foobox-Build-Host-Bestandsaufnahme
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

## Historischer Git-Stand vom 2026-09-22

Der für den damaligen G2X-Reproduzierbarkeitsnachweis verwendete
Softwarecheckpoint war:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

Commit-Beschreibung:

`Make cross-built Bash reproducible`

Die Synchronisierung dieses Commits mit Codeberg und GitHub ist im
Projektstand vom 2026-09-22 dokumentiert. Diese historische Angabe
bezeichnet weder den heutigen HEAD noch eine erneute Prüfung der
Remotes.

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

## Operative Regeln und Protokollarchivierung

Die lokalen Worktree-Dateien `AGENTS.md` und `NOVENA-ARBEITSREGELN.txt`
regeln Arbeitsablauf und Freigabegrenzen; ihre Verfügbarkeit im veröffentlichten
Repository ist nicht gewährleistet. Die getrennte Protokollarchivierung ist in
den Direktupload-Arbeitsregeln unter dem lokalen foobox-Pfad
`/home/loomit/novena-backups/github-transport/DIREKTUPLOAD-ARBEITSREGELN.md`
beschrieben. Originale liegen unter `~/novena-backups/block-logs/`, das
Transportrepository unter `~/novena-backups/github-transport/project-block-logs/`.
Technisches Ergebnis und Transportergebnis bleiben getrennt. Die belegten
Teilfälle sind keine vollständige Uploaderqualifikation; Wiederaufnahme,
Netzwerkfehler und Konfliktfälle bleiben offen. Die Regeln erteilen keine
zusätzliche Aktionsfreigabe.

## Verwandtes Projekt

Die allgemeine Dokumentation der realen Novena-Hardware, Bootpfade,
Recovery-Strategie und historischen Systeme befindet sich separat unter:

`~/novena-system`

## Historische Medien und Image-Provenienz

Die historische Novena-Medien-, Bootloader- und Image-Provenienz ist separat
vom aktuellen P_EXT-/Allocator-Untersuchungsstrang dokumentiert:

- [`docs/HISTORICAL-MEDIA.md`](docs/HISTORICAL-MEDIA.md) – historische
  USB-Medienstruktur, U-Boot-Generationen, NovM-/NovS-Zuordnung,
  `novena-mmc-disk-r1.img`, Debian-Installer-Linie, verifiziertes lokales
  Raw-Master-Image sowie offene Provenienzfragen.

## Experimenteller Debian-Jessie-Erstboot-Modus

Der separate [Debian-Assembler](tools/novena-image-assembler/README.md)
unterstützt neben dem unveränderten Referenzmodus die explizite Auswahl
`mode = "jessie-firstboot-experimental"`. Er dokumentiert offene Bootbedingungen
getrennt von Nachweisen und kennzeichnet Image und Ergebnismanifest als
experimentell unqualifiziert. Die Integritätsprüfungen bleiben erhalten.
[Schnittstelle, offene Bedingungen und Prüfgrenzen](docs/DEBIAN-REFERENCE-IMAGE.md#2026-10-18--novena-jessie-20-experimenteller-erstboot-modus)
sind dokumentiert. Keine Aussage über Bootfähigkeit oder bestätigte Jessie-Provenienz.
