# Build

Stand: 2026-09-22

## Build-System

Primärer Entwicklungs- und Build-Rechner:

`foobox`

Hostarchitektur:

`x86_64-linux`

Zielarchitektur:

`armv7l-linux`

Das vollständige Novena-SD-Image wird mit Nix als Cross-Build erzeugt.

Die detaillierte und verifizierte Konfiguration der foobox ist in
`BUILD-HOST.md` dokumentiert.

## Flake und Quellstand

Projektwurzel:

`~/nixos-novena-image`

Die Nix-Abhängigkeiten sind durch `flake.nix` und `flake.lock` festgelegt.

Der für den aktuellen Reproduzierbarkeitsnachweis verwendete
Projektcheckpoint ist:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

Tree:

`4fb8a30d07273d2eb288f6aed49423ea30eb12a1`

Der gepinnte Nixpkgs-Stand ist:

`a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4`

NAR-Hash:

`sha256-r2f1oUwixlgq9zOdYLqJLfS/lWBT60/IITjhTKI59JU=`

## Produktiver Kernel

Aktueller Kernel:

`6.18.49`

Die produktive Kernel-Patchserie enthält nur die für den Betrieb
benötigten Novena-Anpassungen:

1. `kernel/0001-drm-bridge-it6251.patch`
2. `kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch`

Die früheren I2C-Diagnosepatches gehören nicht mehr zum produktiven
Kernelstand.

`es8328-power` mit `regulator-always-on` ist eine dauerhafte
Novena-Board-Anforderung.

## Produktiver U-Boot

Der Bootloader wird projektlokal aus U-Boot `v2026.07` gebaut.

Upstream-Commit:

`ece349ade2973e220f524ce59e59711cc919263f`

Versionsidentität:

`2026.07-00003-gf8baca04e22d`

`SOURCE_DATE_EPOCH`:

`1789984363`

Der Build erfolgt über den normalen gepinnten
`pkgs.buildUBoot`-Baustein.

Qualifizierte Artefakte:

* `SPL`
  * Größe: `52224` Byte
  * SHA-256:
    `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0`
* `u-boot-dtb.img`
  * Größe: `602120` Byte
  * SHA-256:
    `c5a2d0e7f2b9ebeae6387eee98630a43944f2cd2b52fc8a5a98a2e82ba55f9f7`

Das Image-Rezept bindet diese projektlokal gebauten Artefakte direkt ein.
Die historischen Dateien unter `boot/reference/` sind nur noch
Referenzartefakte.

## Deterministischer Bash-Cross-Build

Bei einem vollständigen Neubau in einem separaten Nix-Store wurde eine
Nichtreproduzierbarkeit im ARM-Bash-Binary gefunden.

Bash erzeugte während des Cross-Builds `builtins/pipesize.h` mit einem
auf dem Build-Host ausgeführten Hilfsprogramm. Dadurch konnte der
dynamische Pipe-Zustand des Build-Hosts als `PIPESIZE` in das
ARM-Zielbinary gelangen.

Beobachtet wurden insbesondere die Werte:

* `65536`
* `8192`

Die Ursache wurde auf die hostseitige Pipe-Kapazität und deren Änderung
bei Erreichen des Linux-Pipe-Soft-Limits zurückgeführt.

Der Projektfix liegt unter:

`patches/bash/0001-bash-cross-build-use-deterministic-pipe-size.patch`

und wird über:

`overlays/reproducible-bash.nix`

nur bei Cross-Builds aktiviert.

Für den Cross-Build wird fest gesetzt:

`NIX_CROSS_PIPESIZE=4096`

`4096` wird hierbei als deterministischer konservativer Linux-Zielwert
verwendet und nicht als Behauptung über die dynamische tatsächliche
Pipe-Kapazität verstanden.

Der Fix gilt sowohl für den interaktiven als auch den
nichtinteraktiven Bash-Build.

Native Bash-Builds bleiben durch das Cross-only-Overlay unverändert.

Die erzeugten ARM-Binaries wurden separat geprüft. In beiden Varianten
wurde im relevanten Code der Wert `4096` nachgewiesen; die frühere
hostseitige Erzeugung über `psize.aux` und `psize.sh` findet im
qualifizierten Cross-Build nicht mehr statt.

Der zugehörige Projektcommit ist:

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

## Aktuelles qualifiziertes SD-Image

Normal-Store-Ausgabe:

`/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`

Größe:

`2581291008` Byte

SHA-256:

`8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Die statische Image-Prüfung bestätigte unter anderem:

* DOS/MBR-Disk-ID `0x2178694e`;
* Partition 1 ab Sektor `16384`;
* Partition 2 ab Sektor `278528`;
* projektlokalen SPL am Offset `1024`;
* projektlokales `u-boot-dtb.img` in der FAT-Partition;
* `boot.scr`, Kernel, Initrd und DTB in der FAT-Partition;
* explizite `mmc 1:1`-Ladevorgänge im Bootskript.

## Reproduzierbarkeitsnachweis

Das vollständige Image wurde anschließend aus exakt dem Commit

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

in einem neu angelegten isolierten Nix-Store erneut gebaut.

Der Clean Store verwendete keine bereits realisierten
Novena-spezifischen Outputs aus dem normalen `/nix/store`.
Normale Cache-Abhängigkeiten waren zulässig.

Clean-Store-Ergebnis:

* logischer Nix-Pfad:
  `/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img`
* Größe:
  `2581291008` Byte
* SHA-256:
  `8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Der direkte Bytevergleich zwischen Normal-Store- und
Clean-Store-Image ergab:

`cmp = 0`

Damit ist die Reproduzierbarkeit auf demselben Build-Host über zwei
getrennte Nix-Stores bytegenau nachgewiesen.

Dieser Nachweis ist noch kein Cross-Host-Reproduzierbarkeitsnachweis.
Ein späterer unabhängiger Build auf dem L14 ist davon getrennt.

## Historischer result-Symlink

Der vorhandene Symlink `result` wird bewusst nicht auf das aktuelle
Image umgebogen.

Er zeigt weiterhin auf den historischen Golden Build:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

Dadurch bleibt die historische Referenz unverändert erhalten.

## Hardware-Qualifikation

Der Software-Build und die Same-Host-/Separate-Store-Reproduzierbarkeit
sind qualifiziert.

Der reale P_EXT-Test auf der Novena weist inzwischen folgenden Teil der
Bootkette nach:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL v2026.07`

Noch nicht nachgewiesen ist der anschließende vollständige Pfad:

`SPL v2026.07 -> U-Boot v2026.07 -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

Der SPL erreicht bei der laufenden Diagnose die MMC-Initialisierung.
Die weitere Hardwarequalifikation erfolgt mit einer getrennt
vorregistrierten seriellen Diagnoseinstrumentierung.

## Kernel-Arbeitsverzeichnisse

Während der Entwicklung können lokal unter anderem existieren:

* `kernel/linux-6.18.49`
* `kernel/linux-6.18.49-orig`
* `kernel/panel-patch`

Diese Verzeichnisse sind Arbeitskopien und werden nicht versioniert.

Dauerhaft relevante Kerneländerungen müssen als Patchdateien unter
`kernel/` vorliegen.
