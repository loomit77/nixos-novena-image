# Experimentelles historisches Novena-Bootloaderpaar

NOVENA-NIXOS-72, 2026-10-18. Dieser Modus ist **experimentell und für NixOS
nicht bootqualifiziert**. Standard bleibt `current-2026`.

Auswahl in einem zusätzlichen NixOS-Modul:

```nix
{ novena.image.bootloader = "historical-2015"; }
```

## Eingabebytes und Herkunft

Die Dateien wurden unverändert aus
`/home/loomit/novena-backups/historical-uboot-2015-analysis/` übernommen:

| Repository-Datei | Ursprünglicher Dateiname | Byte | SHA256 |
|---|---|---:|---|
| `SPL` | `u-boot-2015.04-bd5c7d3.spl` | 44032 | `e2f2b80692c3ca55d29bce39fc98efd44c09046959acda46c50675ef3728e276` |
| `u-boot.img` | `u-boot-2015.04-bd5c7d3.img` | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` |

Die lokale Größen-/Hashprüfung vor und nach der Übernahme stimmt mit den
Auftragswerten überein. `historical-2015.nix` prüft beide Größen und Hashes
vor Ausgabe des gemeinsamen Paars. Keine getrennte Auswahl, kein Ersatzpaar,
kein Download; externe Backup-Pfade sind keine Buildabhängigkeit.

Die dokumentierte Extraktionsherkunft ist `/usr/share/u-boot-novena/` auf p3
des historischen USB-Raw-Masters, Paketkontext `2014.10.r7-novena.1`.
NOVENA-DEBIAN-REF-08 FAIL und REF-08R PASS bleiben getrennte Ergebnisse.
Siehe [historische Medienanalyse](../../../docs/HISTORICAL-MEDIA.md#2026-10-17--novena-debian-doc-01b-bootvertrag-der-debian-paketlinie-2015)
und [Quellenregister](../../../docs/QUELLENVERZEICHNIS.md#2026-10-17--novena-debian-doc-01b-ref-09-bis-ref-14-quellenregister).

Referenzlinie: `xobs/u-boot-novena`, Commit
`bd5c7d3290bb94bf8af17760c321df67d2641343`, Banner
`2015.04-rc1-02853-gbd5c7d3`. Versionsstrings sind kein vollständiger
Binär-/Quellabstammungsnachweis. Ursprüngliche Buildkonfiguration, Toolchain,
vollständige Paket-/Buildprovenienz und reproduzierbarer Quellbuild bleiben offen.
Der Import fixiert historische Eingabebytes; er rekonstruiert deren Build nicht.
Die Referenzen unter `boot/reference/` gehören einer anderen Generation an.

## MMC- und Skriptvertrag

Am Referenzcommit belegen `board/kosagi/novena/novena_spl.c`,
`common/spl/spl_mmc.c`, `common/spl/spl_fat.c` und
`include/configs/imx6_spl.h`: SPL registriert den ausgewählten Controller
als MMC 0 und lädt von FAT-p1 `u-boot.img`. Proper initialisiert gemäß
`board/kosagi/novena/novena.c` USDHC3 vor USDHC2: intern MMC 0, extern MMC 1.
Deshalb lädt das historische NixOS-Skript ausdrücklich von `mmc 1:1`.
Dies bestimmt keine Linux-`mmcblk`-Nummer.

`include/config_distro_bootcmd.h` scannt `/` und `/boot/`, jeweils extlinux
vor `boot.scr.uimg` und `boot.scr`. Das Image stellt `/boot.scr` bereit,
ohne konkurrierende extlinux-, uEnv- oder Skriptdateien. SPL bleibt roh bei
`0x400`; der zusätzliche Größencheck begrenzt es auf den Bereich vor FAT-p1.
Partitionsgrenzen und Kernel-/initrd-/DTB-Artefakte bleiben unverändert.

## Statischer Kommandonachweis und Fehlerpfad

Für NOVENA-NIXOS-72 wurde der vorhandene lokale Git-Quellbestand
`/home/loomit/novena-historical-research/u-boot-novena` am obigen exakten
Commit lesend ausgewertet:

- `include/configs/novena.h` aktiviert `CONFIG_SYS_HUSH_PARSER`,
  `CONFIG_CMD_MMC`, `CONFIG_CMD_FAT` und `CONFIG_CMD_BOOTZ`.
- `common/cli_hush.c` implementiert `if`, `while`, `do`, `done` und `&&`.
- `common/Makefile` bindet `cmd_test.o` an `CONFIG_SYS_HUSH_PARSER`;
  `common/cmd_test.c` registriert `test` samt Zeichenkettenvergleich `=`.
- `include/config_cmd_default.h` aktiviert `CONFIG_CMD_MISC` und
  `CONFIG_CMD_ECHO`; `common/cmd_misc.c` registriert `sleep`.
- Das konkrete Proper enthält die Kommando-/Hilfetextstrings für `bootz`,
  `fatload`, `setenv`, `echo`, `sleep`, `mmc rescan` sowie das reservierte
  Wort `while`. Sein eingebettetes Environment verwendet bereits `if`,
  `test`, `&&`, `mmc dev`, `mmc rescan`, `sleep`, `setenv` und `fatload`.

Das sind übereinstimmende Quellbelege und Binärindizien, kein ausgeführter
Kommandotest und keine vollständige Rekonstruktion des Binärbuilds.
Alle drei Ladevorgänge müssen erfolgreich sein, bevor `bootz` ausgeführt wird.
Bei Fehler oder zurückkehrendem `bootz` gibt das Skript einen Fehler aus und
wartet in `while test 1 = 1; do sleep 1; done`. Es verwendet keinen ungesicherten
`exit`-/`reset`-Fehlerpfad. Ohne Bedienereingriff kehrt es nicht zum historischen
Direktlade-Fallback ohne passende initrd zurück. **Ctrl-C kann Hush/Sleep
abbrechen**; dies ist keine gegen Bedienereingriffe gesicherte Haltfunktion.

## Evidenz und offene Nachweise

Entwurf: lokales Originalprotokoll
`NOVENA-NIXOS-71-20261018-132751-RYuTyjSX.txt` unter
`/home/loomit/novena-backups/block-logs/`; darin die Ergebnisse von
NOVENA-NIXOS-67 bis -70. NOVENA-NIXOS-70 und -71 unterscheiden direkte
Dateiladebereiche von noch unqualifizierter Dekomprimierung/Laufzeitbelegung.
Der [Jessie-Bootnachweis](../../../docs/NOVENA-JESSIE-BOOT-20261018.md)
belegt den dortigen historischen Pfad, keinen NixOS-Boot mit diesem Image.

Persistentes Environment auf interner MMC 0, EEPROM, SATA-Root-Auswahl und
Hooks können den Skriptaufruf vorher beeinflussen. Das Image allein garantiert
weder das Erreichen von `boot.scr` noch Unabhängigkeit vom internen System.
Es schreibt kein Environment und übernimmt keine Jessie-p3-Rootidentität.

Offen bleiben Nix-Evaluierung, Image-Build, Artefaktabnahme, Byteidentität
wiederholter Gesamtimage-Builds (einschließlich mkimage-/FAT-Zeitstempel),
moderner Kernel mit historischem Proper, initrd-/DTB-Übergabe, Relokation,
Dekomprimierung/Laufzeitbelegung, Rootauflösung und vollständiger externer
NixOS-Hardwareboot. In NOVENA-NIXOS-72 wurden keine Builds, Medienzugriffe
oder Hardwaretests durchgeführt. Neue Dateien sind absichtlich nicht im
Git-Index; für einen späteren Git-Flake-Build müssen sie gesondert autorisiert
aufgenommen werden.
