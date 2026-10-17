# Historische Novena-Medien und Image-Provenienz

Stand: 2026-09-28

## 2026-10-11 – DOC-UPDATE-02B: datierte Medien- und Imageidentitäten

Der Stand vom 28.09. und die Nachträge vom 05./06.10. bleiben historische
Untersuchungsstände. Grundlage dieser Präzisierung ist DOC-UPDATE-02A,
`/home/loomit/novena-backups/block-logs/DOC-UPDATE-02A-20261011-085639-6dYeUv41.txt`.
Die folgenden Prüfsummen sind übernommene historische Werte, keine heutige
Integritätsprüfung der Dateien oder Medien.

| Belegdatum | Artefakt | Größe in Byte | Historische SHA-256 | Identität und Grenze |
|---|---|---|---|---|
| 2026-09-12 | `novena-internal-mmc-2026-09-12.img` | `3965190144` | `c27f0afe75e1b7073bb8d0deed16880afcc6c402080cf9816cc130731d61e34e` | Interne MMC/microSD, NovM; Zustandsbackup, kein unverändertes Factory-R1-Release. |
| 2026-09-11 | `novena-sd-original.img` | `3963617280` | `17cccd572d3100fb6403946e4741639c76dd49481c3f9cc037269ca2dd23753a` | Älteres NixOS-Quellimage, Disk-ID `0x2178694e`; „original“ bezeichnet keine Factory-Provenienz. |
| 2026-09-11 | `novena-sd.img` | `3963617280` | `e8c910f7df4328530ef3d1b30ab272be6ccb9fb9eda8337ac4270449d6393f5a` | Historisch als bytegleicher Präfix von `novena-sd-full.img` dokumentiert; eigener Dateiumfang. |
| 2026-09-11 | `novena-sd-full.img` | `31914983424` | `df2013d4f68c9979d4e1b22b57bb1f40f219631e7a89810794c5d709509b5720` | Historischer NixOS-SD-Zustand, vom kleinen „original“-Image verschieden. |
| 2026-09-11 (Backupdatum 06.09.) | `novena-sd-32gb-backup-2026-09-06.img` | `31914983424` | `0e5a478b8cb7ee06e3647c096cbfbd6a50afdd92ec1f0769369ba3f0dc03d1d8` | Weiterer NixOS-Zustand; gleiche Größe beweist keine Identität mit `novena-sd-full.img`. |
| 2026-09-28 | `novena-historical-usb-full-2026-09-28.img` | `11098128384` | `660cf1f33ff4ce93de07f2b8a78488ab415eebee85eebf6b9a9d9764248835af` | Teilimage des historischen USB-Mediums, trotz „full“ im Dateinamen. |
| 2026-09-28 | `novena-historical-usb-ddrescue-2026-09-28.img` | `31666995200` | `7e9028d5b127ead9bf20867bd8d6882ae5ff6b256f0504c21de5fe8267f3ead9` | Vollständiger lokaler USB-Raw-Master; historischer Hybridzustand, kein unverändertes Factory-Image. |
| Quellencheckpoints 2026-09-28, 05./06.10. | Factory `novena-mmc-disk-r1.img` | Exakte Releasegröße offen; ungefähre Mindestmediumgröße `2.3GB` aus älterer Projektdokumentation übernommen, lokal primärquellenseitig nicht unabhängig bestätigt (siehe Präzisierung unten) | `26d368cb4b3aa43e411703f8c659d3e229deacfe75af38c1f82489dd9af80dbb` | Veröffentlichte Hashidentität; keine verifizierte wiedergefundene Binärkopie. MD5: `6923a145cbdc75b420408fc2d09ba4f8`. |

Präzisierung vom 2026-10-11 – DOC-FIX-02: Die Angabe `2.3GB` beziehungsweise
2,3 GB ist aus älterer Projektdokumentation übernommen und in der aktuellen
lokalen Primärquellenprüfung nicht unabhängig bestätigt. Dies gilt auch für
die entsprechenden historischen Größenangaben weiter unten. DOC-VERIFY-02
benennt diese Quellenlücke; auch hier wurde keine eindeutige lokale historische
Primärfundstelle gefunden. Die lokale
[Kosagi-Main-Page-Kopie](../research/01-original-novena-docs/sources/kosagi-novena-main-page.html)
belegt im Abschnitt „Disk Imaging“ Factory-Einsatz und Hashidentität, enthält
aber diese Mindestmediumgröße nicht. Die Angabe ist weder eine exakte
Factory-R1-Dateigröße noch eine verifizierte Releasegeometrie.

MMC-Quelle ist die separate
[Hardwarebaseline vom 12.09.](/home/loomit/novena-system/docs/NOVENA-HARDWARE-BASELINE-2026-09-12.md),
die Backup und unabhängigen Quellhash historisch dokumentiert. Der dortige
HDD-Pfad lautet
`/mnt/novena-backup-hdd/novena-archive/device-backups/novena-internal-mmc-2026-09-12.img`.
Diese HDD-Kopie wurde hier nicht angesprochen oder erneut geprüft.
Die vier NixOS-SD-Dateien sind im separaten
[Imageinventar](/home/loomit/novena-system/docs/IMAGE-INVENTORY.md) und der
[Prüfsummenausgabe vom 11.09.](/home/loomit/novena-system/snapshots/2026-09-11-foobox/image-checksums.txt)
belegt. Belegdaten sind keine pauschalen Image-Erstellungsdaten.
Diese Dateien, die interne MMC, der historische USB-Zustand und die externe
SD der späteren NixOS-/P_EXT-Tests sind getrennte Gegenstände; auch Golden
und G2X sind eigene [Buildreferenzen](GOLDEN-BUILD.md#2026-10-11--doc-update-02b-historische-identitäten-und-evidenzgrenzen).

Die dokumentierten gemeinsamen Dateisystem-UUIDs und frühen Partitionsmerkmale
von interner MMC und USB stützen eine gemeinsame historische Systemlinie.
Disk-ID, Rootpartitionsgröße und Gesamtmedium unterscheiden sich.
Ein individueller MMC-zu-USB-Kopiervorgang, seine Richtung, sein Zeitpunkt
oder die konkrete Transformation sind damit nicht nachgewiesen.

Die älteren Bezüge auf Fall `7C` und Patch0010 unten gelten für ihre
jeweiligen datierten Untersuchungsstände. Für DDR/SPL bleibt der
[A-B-B-A-Checkpoint vom 07.10.](PROJECT-STATE.md#maßgeblicher-ddr-testcheckpoint--block-9c-52q-r7b-3aaj-2026-10-07)
maßgeblich: Root Cause OPEN, Patch0012 kein allgemein nachgewiesener Fix.
I2C3/ES8328 bleibt funktional CLOSED. P_EXT bis SPL ist kein Nachweis des
vollständigen externen Bootpfads. Historische Medienbefunde erklären diese
DDR-/SPL-Beobachtungen ohne eigenen Nachweis nicht.

## Zweck und Abgrenzung

Dieses Dokument beschreibt die historische Medien-, Image- und Bootloader-Provenienz, die im Rahmen der Untersuchung des eigenständigen Novena-Bootpfads rekonstruiert wurde.

Gegenstand sind insbesondere:

* ein zusammen mit der Novena erhaltenes historisches USB-Medium;
* dessen Partitions-, Dateisystem-, Bootloader- und Rootfs-Zustand;
* die historische Novena-Image-Erzeugung;
* das veröffentlichte `novena-mmc-disk-r1.img`;
* historische U-Boot-Versionen;
* die spätere Novena-Debian-Installer-Linie;
* die Beziehung dieser Quellen zum beobachteten historischen Medium.

Diese Untersuchung ist von der P_EXT-/Allocator-Hardwarediagnose getrennt. Insbesondere wird aus historischen Medien- oder Quellcodebefunden keine Erklärung für den im aktuellen U-Boot v2026.07 beobachteten Q1/Q2/P0-Fall `7C` abgeleitet.

Die abgeschlossene ES8328/I2C3-Root-Cause-Untersuchung wird ebenfalls nicht wieder geöffnet.

## Aussageklassen

Für die historische Untersuchung werden unterschiedliche Evidenzstärken ausdrücklich getrennt.

### Direkt beobachtet

Ein Befund wurde unmittelbar am qualifizierten historischen Medium, dessen verifiziertem Raw-Master oder an einem lokal vorhandenen historischen Git-Objekt festgestellt.

### Primärquellenseitig bestätigt

Ein Befund wird durch zeitgenössischen Novena-/Kosagi-Quellcode, Dokumentation oder erhaltene Git-Historie bestätigt.

### Stark gestützt

Mehrere voneinander unabhängige Befunde sprechen für dieselbe Interpretation, ohne dass eine vollständige bytegenaue Provenienzkette vorliegt.

### Nicht nachgewiesen

Die vorhandenen Quellen reichen nicht aus, um eine konkrete Transformation, Benutzerhandlung, zeitliche Abfolge oder Byteidentität zu behaupten.

Diese Trennung ist insbesondere bei Begriffen wie „Originalimage“, „Factory Image“, „Auslieferungszustand“ und „unverändert“ einzuhalten.

## Historisches physisches USB-Medium

Das untersuchte historische USB-Medium wurde zusammen mit der Novena aufbewahrt.

Es ist nicht die externe SD-Karte der aktuellen NixOS-/P_EXT-Tests. Diese externe SD wurde wesentlich später für das laufende NixOS-Projekt erstellt und besitzt für die historische Medienprovenienz keine entsprechende Aussagekraft.

Das historische USB-Medium wurde anhand stabiler Geräteattribute qualifiziert:

* Herstellerkennung: `SMI`;
* Modell: `USB DISK`;
* USB VID:PID: `090c:2000`;
* Seriennummer: `CCYYMMDDXK9TMH40`;
* stabiler by-id-Pfad: `usb-SMI_USB_DISK_CCYYMMDDXK9TMH40-0:0`;
* Gesamtgröße: `31666995200` Byte;
* Sektoren: `61849600`;
* logische Sektorgröße: `512` Byte.

Die DOS/MBR-Disk-ID lautet `0x4e6f7653`.

Die vier Rohbytes an Offset `0x1b8` lauten `53 76 6f 4e`. Die numerische Little-Endian-Darstellung der Disk-ID ist `0x4e6f7653`. Die Bytes dürfen daher nicht als direkt im Medium stehende Zeichenfolge `NovS` beschrieben werden.

## Partitionstabelle des historischen USB-Mediums

Das Medium verwendet eine DOS/MBR-Partitionstabelle.

### Partition 1

* Start: Sektor `2048`;
* Größe: `65536` Sektoren = `32 MiB`;
* Typ: `0x0b`;
* Dateisystem: FAT16;
* UUID: `BB0B-A49F`;
* PARTUUID: `4e6f7653-01`.

### Partition 2

* Start: Sektor `67584`;
* Größe: `65536` Sektoren = `32 MiB`;
* Typ: `0x82`;
* Verwendung: Swap;
* UUID: `b8bad69c-5edf-42f2-85d6-ca1b5cc1bedf`;
* PARTUUID: `4e6f7653-02`.

### Partition 3

* Start: Sektor `133120`;
* Größe: `29296640` Sektoren;
* Größe in Byte: `14999879680`;
* Typ: `0x83`;
* Dateisystem: ext4;
* UUID: `fa7f372b-d890-4a8c-877e-7e201e997682`;
* PARTUUID: `4e6f7653-03`.

Partition 3 endet bei Sektor `29429759`.

Danach verbleiben `32419840` Sektoren beziehungsweise `16598958080` Byte unpartitioniert am Ende des Mediums.

Die aktuelle Größe des USB-Mediums darf deshalb nicht mit der Größe des darauf enthaltenen historischen Systemlayouts gleichgesetzt werden.

## MBR und SPL

Der MBR enthält generischen x86-BIOS-Code. Die MBR-Signatur `55 aa` ist vorhanden.

Im Bereich vor der ersten Partition befindet sich ein i.MX6-Bootloader. Die i.MX-IVT liegt bei physischem Offset `0x400`.

Der dort direkt im Rohmedium vorhandene SPL identifiziert sich als:

    U-Boot SPL 2014.10-rc3-00039-gc5efead (Oct 17 2014 - 19:41:27)

Der zugehörige historische Git-Commit wurde im Repository `xobs/u-boot-novena` exakt identifiziert als `c5efeadb913c8246be372054adcd6106ad4fe067`.

Commit-Datum: `2014-10-17`

Commit-Betreff: `novena: Parse EEPROM early, and boot to SATA if requested`

Historische Git-Beschreibung: `v2014.10-novena-rc12-0-gc5efeadb91`

## FAT-Bootpartition

Auf Partition 1 wurden genau sechs Dateien festgestellt:

* `u-boot.spl` – `35840` Byte;
* `u-boot.img` – `305148` Byte;
* `zimage` – `3602272` Byte;
* `novena.dtb` – `39800` Byte;
* `zImage.recovery` – `3602272` Byte;
* `novena.recovery.dtb` – `39800` Byte.

Die FAT-Zeitstempel liegen am 2015-02-05.

Der auf der FAT-Partition gespeicherte SPL identifiziert sich als:

    U-Boot SPL 2014.10-rc3-00045-g3d26413 (Dec 15 2014 - 13:43:05)

Der zugehörige Git-Commit in `xobs/u-boot-novena` ist `3d26413e65f985131a9573d99a51fd47e7253188`.

Commit-Datum: `2014-12-15`

Commit-Betreff: `debian: Update to rc14`

Historische Git-Beschreibung: `v2014.10-novena-rc14-0-g3d26413e65`

Damit sind bereits der tatsächlich rohe SPL und der auf der Bootpartition gespeicherte SPL unterschiedliche historische U-Boot-Generationen.

## Root-Dateisystem

Das ext4-Dateisystem von Partition 3 besitzt:

* UUID `fa7f372b-d890-4a8c-877e-7e201e997682`;
* Blockgröße `4096` Byte;
* Erstellungszeitpunkt `Fri Nov 14 13:55:12 2014`;
* Mount Count `58`;
* letzten dokumentierten Dateisystemcheck `Tue May 19 16:06:44 2015`;
* Lifetime Writes von ungefähr `709 GB`.

Der Dateisystemzustand war bei der read-only Untersuchung als sauber markiert.

Mehrere Verzeichniszeitstempel zeigen spätere Änderungen. Der Root-Verzeichniseintrag trägt unter anderem einen Zeitstempel vom 2023-01-28.

Daraus folgt: Der heute beobachtete Rootfs-Zustand darf nicht als unveränderter Auslieferungszustand bezeichnet werden.

## Betriebssystem und Novena-Pakete

Das Root-Dateisystem enthält `/etc/debian_version = 8.2` und den Hostnamen `novena`.

Die APT-Konfiguration verweist auf Debian Jessie einschließlich `main`, `contrib`, `non-free` und Jessie-Security. Zusätzliche historische Paketquellen beziehungsweise Listen umfassen unter anderem Kosagi sowie weitere damals auf dem System vorhandene Hardware-/SDR-bezogene Quellen.

Der vorhandene Kernelmodulbaum lautet `/lib/modules/3.19.0-00485-gc5f2138`.

Unter den installierten Novena-/Kosagi-Paketen befinden sich unter anderem:

* `linux-image-novena 3.19-novena-r39`;
* `linux-firmware-image-novena 3.19-novena-r39`;
* `linux-headers-novena`;
* `novena-eeprom`;
* `novena-eeprom-gui`;
* `novena-firstrun`;
* `novena-usb-hub`;
* `novena-disable-ssp`;
* `xorg-novena`;
* `pulseaudio-novena`;
* `irqbalance-imx`;
* `kosagi-repo`;
* `u-boot-novena 2014.10.r7-novena.1`.

Dieser Paketbestand stützt die Einordnung als echtes historisches Novena-/Kosagi-System. Er beweist jedoch nicht, dass der heute beobachtete Zustand exakt dem ursprünglich ausgelieferten Zustand entspricht.

## Historische fstab und MMC-Zielstruktur

Die historische `/etc/fstab` enthält für Root, Boot und Swap Gerätepfade auf Basis von `platform-2198000.usdhc`.

Insbesondere werden Root-Partition 3, Boot-Partition 1 und Swap-Partition 2 angesprochen. Damit trägt das Root-Dateisystem eine klare MMC-/USDHC-Zielsemantik.

Dieser Befund steht in einem wichtigen Spannungsverhältnis zur Disk-ID `0x4e6f7653`, die in der historischen Novena-Image-Erzeugung der SATA-Linie zugeordnet wird.

## Dritte U-Boot-Generation im Rootfs

Das installierte Paket `u-boot-novena` enthält eine weitere SPL-Generation.

Paketdatei: `/usr/share/u-boot-novena/u-boot.spl`

Größe: `44032` Byte

Der SPL identifiziert sich als:

    U-Boot SPL 2015.04-rc1-02853-gbd5c7d3 (Sep 04 2015 - 10:11:08)

Der zugehörige Git-Commit wurde als `bd5c7d3290bb94bf8af17760c321df67d2641343` identifiziert.

Commit-Datum: `2015-09-04`

Commit-Betreff: `debian: Update to v2014.10.r7-novena.1`

Historische Git-Beschreibung: `v2014.10.r7-novena-0-gbd5c7d3290`

Damit enthält das untersuchte System mindestens drei voneinander unterscheidbare U-Boot-/SPL-Generationen:

1. Rohmedium: rc12 vom Oktober 2014;
2. FAT-Bootpartition: rc14 vom Dezember 2014;
3. Rootfs-Paket: Stand vom September 2015.

Aus diesen drei Generationen wird keine konkrete Reihenfolge individueller Benutzeraktionen abgeleitet.

## Historisches SPL-Installationsskript

Das Root-Dateisystem enthält `/usr/sbin/novena-install-spl`.

Das Skript verwendet standardmäßig `file=/boot/u-boot.spl` und `disk=/dev/disk/by-path/platform-2198000.usdhc` und schreibt den SPL sinngemäß mit:

    dd if="${file}" of="${disk}" bs=1024 seek=1 conv=notrunc

Der Zieloffset beträgt damit ebenfalls `0x400`.

Dies erklärt den historischen Mechanismus zum Aktualisieren eines SPL auf dem internen MMC-/microSD-Ziel. Es beweist nicht, dass genau dieses Skript den aktuell im untersuchten USB-Medium vorhandenen Roh-SPL geschrieben hat.

## Externe-SD-Unterstützung im historischen U-Boot

Die historische U-Boot-Git-Historie zeigt, dass Unterstützung für den Boot von der externen SD bereits im Oktober 2014 eingeführt wurde.

Commit: `b5a15faf85a52789308f3f59898693b005532833`

Datum: `2014-10-15`

Betreff: `novena-spl: Add support for booting from external SD`

Der SPL unterscheidet dabei die ROM-Bootquelle und verwendet USDHC3 für die interne SD-/MMC-Linie sowie USDHC2 für die externe SD.

Ein zugehöriger Full-U-Boot-Commit `4facac7043bd45f57956b5a0084cb02aaea59bae` unterscheidet interne MMC, externe MMC und SATA als Bootquellen.

Beide Oktober-Commits sind Vorfahren des auf dem historischen Rohmedium identifizierten rc12-Commits `c5efead...`.

Damit enthält der rohe rc12-SPL bereits die Oktober-2014-Unterstützung für die externe SD.

Ein weiterer Commit vom Februar 2015 mit ähnlicher External-MMC-Thematik ist eine davon getrennte historische Änderung und darf nicht als Ursprung der bereits im Oktober vorhandenen Unterstützung behandelt werden.

Aus der Existenz dieser U-Boot-Unterstützung folgt außerdem nicht, dass das untersuchte Novena ursprünglich mit einer externen SD als Rootmedium ausgeliefert wurde.

## Historische Novena-Image-Erzeugung

Das historische Repository `xobs/novena-image` wurde separat und read-only untersucht.

Qualifizierter lokaler HEAD: `f62c4f3b452199ab882e2c3dba16d4e31dfb1d56`

Frühester untersuchter Repositorycommit: `a9d5066d55f2cdbdef1589f2bd12ca1e176ac2bb` vom 2014-10-10.

Bereits dieser frühe Stand unterscheidet zwei Image-Typen.

### MMC

Für `mmc` wird verwendet:

    Disk-ID = 0x4e6f764d
    Swap = 32 MiB

Die numerische ID entspricht der historischen `NovM`-Kennung. Die erzeugte `fstab` verwendet die Novena-USDHC-Pfade.

### SATA

Für `sata` wird verwendet:

    Disk-ID = 0x4e6f7653
    Swap = 4 GiB

Die numerische ID entspricht der historischen `NovS`-Kennung. Die SATA-Variante verwendet SATA-Gerätepfade.

### Gemeinsame Struktur

Die Builderlogik erzeugt grundsätzlich Bootpartition, Swap und Rootpartition. Die Bootpartition ist `32 MiB` groß. Der SPL wird bei Offset `0x400` geschrieben.

Präzisierung vom 2026-10-11: Die 32-MiB-Angabe gilt für den oben bezeichneten
frühen Builderstand `a9d5066d55f2cdbdef1589f2bd12ca1e176ac2bb` vom 2014-10-10,
nicht für jede Buildergeneration oder die unbelegte Factory-R1-Geometrie.
Die spätere Entwicklung `32M -> 64M -> 32M` ist im
[Quellencheckpoint vom 06.10.](#2026-10-06--block-9c-52e-builder--boot--und-ram-quellencheckpoint)
dokumentiert. Die 32-MiB-Bootpartition des USB-Masters ist ein separater
Medienbefund, kein Beweis identischer Releasegeometrie.

Das historische `mmc-install.sh` ruft den Builder für die MMC-Variante auf.

## Hybridzustand des historischen USB-Mediums

Das untersuchte USB-Medium kombiniert Merkmale, die in der unveränderten historischen Builderlogik nicht gemeinsam zu einem einzigen Image-Typ gehören.

Beobachtet wurden:

* Disk-ID `0x4e6f7653` – SATA-/`NovS`-Semantik;
* Swapgröße `32 MiB` – MMC-Semantik;
* `/etc/fstab` mit USDHC-Gerätepfaden – MMC-Semantik;
* Rootpartition als Partition 3 – gemeinsame historische Struktur;
* SPL ab `0x400` – kompatibel mit der historischen Builderlogik.

Daraus folgt: Der heutige Zustand des historischen USB-Mediums lässt sich nicht als unverändertes Ergebnis eines einzelnen normalen `--type mmc`- oder `--type sata`-Builderlaufs erklären.

Die konkrete Transformation, durch die dieser Hybridzustand entstand, ist nicht nachgewiesen.

Insbesondere wird nicht behauptet, dass eine bestimmte Benutzerhandlung, ein bestimmtes Installationsskript oder eine bestimmte spätere Anleitung die Disk-ID verändert hat.

## Das veröffentlichte novena-mmc-disk-r1.img

In der erhaltenen öffentlichen Novena-Dokumentation ist das Image `novena-mmc-disk-r1.img` als Factory-microSD-Image dokumentiert.

Historische URL: `http://repo.novena.io/novena/images/novena-mmc-disk-r1.img`

Dokumentierte SHA-256-Prüfsumme:

`26d368cb4b3aa43e411703f8c659d3e229deacfe75af38c1f82489dd9af80dbb`

Dokumentierte MD5-Prüfsumme:

`6923a145cbdc75b420408fc2d09ba4f8`

Die Dokumentation nennt für Revision r1 ungefähr `2.3GB` als erforderliche Mindestgröße des Zielmediums.

Das Image wird vollständig mit `dd` auf eine microSD geschrieben. Anschließend kann die Rootfs-Partition optional vergrößert werden.

Damit ist das dort beschriebene Vergrößern der Rootpartition eine nachgelagerte Installationshandlung und kein Nachweis dafür, dass dieser Schritt zur Erzeugung des veröffentlichten r1-Images verwendet wurde.

## Offene r1-Geometrie

Das historische `xobs/novena-image`-Repository enthält seit Commit `24a84d8c92e3583dffcbbe7dcc424dc94884e29f` vom 2014-10-15 eine feste Loopbackgröße von `3965190144` Byte.

Diese Größe erklärt die öffentlich dokumentierte Größenangabe von ungefähr `2.3GB` für `novena-mmc-disk-r1.img` nicht unmittelbar.

In der untersuchten Builderhistorie wurde kein separater `resize2fs`-/Shrink-/Release-Schritt gefunden, der daraus ein ungefähr 2,3-GB-r1-Image erzeugt.

Daraus wird nicht geschlossen, dass ein solcher externer Releaseprozess nie existiert hat.

Die exakte Gesamtgröße und die exakten Partitionsgrenzen des veröffentlichten r1-Images bleiben ohne eine verifizierte Kopie des Images oder gleichwertige Primärmetadaten offen.

## NovM und NovS

Die historische Builderquelle trennt die Kennungen eindeutig:

    MMC  -> 0x4e6f764d
    SATA -> 0x4e6f7653

Diese Zuordnung wird zusätzlich durch spätere historische Novena-Installerquellen gestützt.

Dort wird für den MMC-/Recovery-Pfad `PARTUUID=4e6f764d-03` und für den SATA-/SSD-Rootfs-Pfad `PARTUUID=4e6f7653-03` dokumentiert.

Damit gilt für die historische Untersuchung:

`Q2_NOVM_NOVS=RESOLVED`

Die normale historische MMC-Linie ist `0x4e6f764d`.

Die normale historische SATA-Linie ist `0x4e6f7653`.

## Widerspruch in einer späteren Novena-Anleitung

Die spätere öffentliche Anleitung zum Schreiben des Factory-microSD-Images verlangt für Recovery die Kennung `4e6f764d-03`, weist beim Bearbeiten der Partitionstabelle aber zusätzlich darauf hin, dass gegebenenfalls die Disk-ID neu gesetzt werden müsse, und nennt dabei als MBR-Beispiel `0x4e6f7653`.

Dies steht zur Buildersemantik der normalen MMC-Linie in Spannung.

Der Befund wird als historische Quellenabweichung dokumentiert. Er wird nicht als Nachweis dafür verwendet, dass genau diese Anleitung den Hybridzustand des untersuchten USB-Mediums verursacht hat.

## Rootfs-UUID-Spur

Die Rootpartition des historischen USB-Mediums besitzt `fa7f372b-d890-4a8c-877e-7e201e997682`.

Genau diese UUID erscheint auch in der historischen `novena-debian-installer`-Git-Historie.

Der früheste lokal identifizierte Commit mit dieser UUID ist `e31b7e30b946f7e9ca67e3920d2de9da6a8467c7`.

Datum: `2015-07-03`

Betreff: `Adding uEnv.txt`

Der dort eingeführte Installer-Bootpfad verwendet diese UUID als Rootziel, wenn das Installer-Initramfs geladen wurde.

Diese Übereinstimmung ist eine starke Provenienzkorrelation. Sie ist für sich allein kein bytegenauer Beweis, dass die Rootpartition eines unveränderten `novena-mmc-disk-r1.img` exakt diese UUID besaß.

## Novena-Debian-Installer

Das historische Repository `thesourcerer8/novena-debian-installer` wurde separat untersucht.

Qualifizierter lokaler Master-HEAD: `ebdb816b93f91bf32809a8c4c1454d4ef7f2bb5f`

Die Dokumentation des Installers beschreibt ausdrücklich die Verwendung eines fertigen Installer-Images auf einer microSD.

Der Installer ist eine von `novena-mmc-disk-r1.img` zu unterscheidende Image-Linie. Er darf deshalb nicht mit dem Factory-microSD-r1-Image gleichgesetzt werden.

## Installer-Image als r1-Derivat

Die historische Installer-Dokumentation beschreibt für den Selbstbau:

1. Ausgangspunkt ist das normale `novena-mmc-disk-r1.img`;
2. die Bootpartition wird vergrößert oder neu angelegt;
3. `makeimage.sh` wird auf einer Novena ausgeführt;
4. die erzeugten Dateien unter `boot/` werden auf die Bootpartition kopiert.

`makeimage.sh` erzeugt dabei kein vollständiges neues Root-Dateisystem und kein vollständiges Raw-Disk-Image.

Damit ist primärquellenseitig gestützt, dass die Installer-Image-Linie als Derivat der normalen r1-MicroSD-Basis konzipiert wurde.

Dies beweist nicht, dass das untersuchte historische USB-Medium aus einem solchen Installer-Image erzeugt wurde.

## Historische Installer-Image-Generationen

In der Git-Historie wurden mehrere dokumentierte Installer-Image-Prüfsummen festgestellt.

Frühere SHA-512-Prüfsumme:

`9a91f978d11ae698c2178033808d0324be43acc78fe7f2eef11d8f1a0aa013a37b808b6a9128898db53e4a63016039dce7e57f4ace56b331cee58822364c2ec4`

Spätere Futureware-Generation:

`fb4a59938ea2fea47af401825c532e5637e62e839abc4c638334e4aedb067e81b4eb886e61653c39d02c365dc55636c7f83c5b96f3ea1c48f6793c1c7efa42d0`

Eine weitere Generation wurde am 2016-03-28 gebaut und in PR #6 dokumentiert.

SHA-512:

`5a1a6c834ad4e315084b8051d7f8b4ac09df0936b6921e44c7424d4ae669310b65fa8846df057876a5f1a19654a886dfb2f61dda208061a3390cd7fc10d7878`

Diese Prüfsummen gehören zur historischen Installer-Image-Linie und nicht zur SHA-256-Prüfsumme des `novena-mmc-disk-r1.img`.

## PR #6 des Debian-Installers

Der historische PR-Head wurde über den GitHub-PR-Ref lokal gesichert.

PR-Head: `4f7263c60dc96746905ac347c57f075ecb3cd6aa`

GitHub-Merge-Testcommit: `94b7e44dec25e0863c50a1bf2547cbf6efe910ce`

Beide besitzen denselben Tree: `9796df819ca1d3d8d8e944a5074c6dd7cdf3e2ff`.

Der PR-Head stammt vom 2016-03-29 und dokumentiert die am 2016-03-28 gebaute Installer-Generation mit der SHA-512-Prüfsumme `5a1a6c...d7878`.

Der GitHub-Merge-Ref wird nur als historischer Test-Merge klassifiziert. Aus seiner Existenz wird nicht geschlossen, dass PR #6 in dieser Form in den normalen Master übernommen wurde.

## Spätere selektive Upstream-Übernahme

Der spätere Master-Commit `1a06a3fe1a0f2d84cc3b2c0b154b863a4cc14f96` trägt den Betreff `Cherry picking from pull request from chris4795`.

Er ist jedoch kein patch-identischer Cherry-Pick eines einzelnen untersuchten PR-#6-Commits.

Die Stable-Patch-ID des Master-Commits lautet `0d2f8e67e463676151927f5d590a86b9b546af37`.

Für die untersuchten einzelnen PR-Commits wurde kein identischer Einzelcommit-Patch gefunden.

Die historische Klassifikation lautet deshalb `SELECTIVE_UPSTREAM_REIMPLEMENTATION_OR_EXTRACTION`.

Der March-2016-SHA-512-Eintrag ist im PR-Head beziehungsweise dessen Tree vorhanden, nicht jedoch im untersuchten Tree dieses späteren Master-Commits.

Deshalb wird nicht behauptet, dass die March-2016-Imagezeile durch diesen Master-Commit unverändert übernommen wurde.

## Verifiziertes lokales Raw-Master-Image

Während der Preservation-Arbeit zeigte das physische USB-Medium bei einem ersten vollständigen `dd`-Versuch einen Transportfehler.

Der erste Leseversuch brach exakt bei `11098128384` Byte ab.

Das teilweise gelesene Image wurde bewusst erhalten und nicht überschrieben.

Dateiname: `novena-historical-usb-full-2026-09-28.img` im selben
Sicherungsverzeichnis wie der ddrescue-Master unten. Trotz `full` im Namen
ist dies das Teilimage mit `11098128384` Byte, kein vollständiges USB-Abbild.

SHA-256 des Partial-Images:

`660cf1f33ff4ce93de07f2b8a78488ab415eebee85eebf6b9a9d9764248835af`

Nach physischem Neuverbinden wurde das Medium anhand von Seriennummer, VID:PID und Größe erneut eindeutig identifiziert.

Anschließend wurde mit GNU ddrescue 1.30 ein neues vollständiges Raw-Image erstellt.

Der Lauf endete mit:

* `31666995200` Byte gerettet;
* `100.00%`;
* `0` Read Errors;
* `0` Bad Areas;
* Rückgabestatus `0`.

Das vollständige lokale Raw-Master-Image ist:

`/home/loomit/novena-backups/historical-novena-usb-2026-09-28/novena-historical-usb-ddrescue-2026-09-28.img`

Größe: `31666995200` Byte

SHA-256:

`7e9028d5b127ead9bf20867bd8d6882ae5ff6b256f0504c21de5fe8267f3ead9`

Die ddrescue-Map besitzt SHA-256:

`f3df1797140f6f1516e871ee85a87bbd5105a75ce74dbbf02e0f136d87c578b7`

Die Map beschreibt den gesamten Bereich als erfolgreich gerettet.

## Unabhängige Prefix-Verifikation

Der bereits beim fehlgeschlagenen ersten `dd`-Lauf gelesene `11098128384`-Byte-Präfix wurde nicht verworfen.

Der SHA-256-Wert desselben Präfixbereichs des vollständigen ddrescue-Images stimmt exakt mit dem SHA-256 des ursprünglichen Partial-Images überein.

Damit wurde der bereits zuvor unabhängig gelesene Medienpräfix bytegenau bestätigt.

Diese zusätzliche Prüfung erhöht die Vertrauenswürdigkeit des lokalen Raw-Masters, ohne einen weiteren vollständigen Lesedurchlauf über das physische USB-Medium zu erzwingen.

Präzisierung vom 2026-10-11: Der historisch dokumentierte Vergleich umfasst
ausschließlich die ersten `11098128384` Byte, also `[0, 11098128384)`.
Er ist keine zweite unabhängige Vollverifikation des `31666995200`-Byte-Masters.
Ein ursprüngliches Vergleichsrohprotokoll wurde in DOC-UPDATE-02A nicht
gefunden; die historische Gleichheitsangabe wurde hier nicht neu geprüft.

## 2026-10-11 – USB-Beweiskette und erhaltene Belege

Lokales Verzeichnis:
`/home/loomit/novena-backups/historical-novena-usb-2026-09-28/`.
DOC-UPDATE-02A fand dort beide Images mit den dokumentierten Größen,
`SHA256SUMS.local` und die vollständige ddrescue-Map. Deren historischer
SHA-256-Wert ist `f3df1797140f6f1516e871ee85a87bbd5105a75ce74dbbf02e0f136d87c578b7`.

| Schritt vom 2026-09-28 | Erhaltener Beleg | Aussagegrenze |
|---|---|---|
| Geräteidentifikation und Reidentifikation | Dokumentierte Seriennummer, VID:PID und Größe; Map-Befehlszeile mit stabilem by-id-Quellpfad | Eigenständiges ursprüngliches Reidentifikationsrohprotokoll nicht gefunden; kein heutiger Gerätezugriff. |
| Erster `dd`-Versuch mit Abbruch | Erhaltenes Teilimage und historischer Manifesthash | Ursprüngliches Fehlerrohprotokoll nicht gefunden; Dateigröße allein beweist weder Fehlerursache noch vollständigen Ablauf. |
| Vollständiger ddrescue-Lauf | Master und Map von GNU ddrescue 1.30; Start 14:29:16, Ende 14:32:51; gesamter Bereich `0x75F800000` als `+` markiert | Map belegt den erhaltenen Rettungsstatus. Sie ersetzt kein vollständiges Prozessrohprotokoll für die oben historisch genannten Read-Error-/Bad-Area-Statistiken und Exitcode 0. |
| Historische Hashbindung | `SHA256SUMS.local` enthält beide Imagehashes und den Maphash | Manifestwerte wurden gelesen; keine neue vollständige Hashprüfung der Images. |
| Unabhängiger Präfixvergleich | Historische Dokumentation und erhaltenes Teilimage | Vergleich nur über `11098128384` Byte; ursprüngliches Vergleichsrohprotokoll nicht gefunden. |
| Strukturqualifikation / `MASTER_SOURCE_GATE=PASS` | Historische Struktur- und Gate-Dokumentation in diesem Dokument und PROJECT-STATE.md | Eigenständiges ursprüngliches Qualifikationsrohprotokoll nicht gefunden; Gate nicht neu ausgeführt. |

Die historische Bezeichnung „verifiziertes vollständiges Raw-Master-Image“
bleibt erhalten. Sie bezeichnet den dokumentierten Checkpoint vom 28.09.,
keine heute erneut ausgeführte vollständige Akquisition oder Integritätsprüfung.

## Master-Image-Qualifikation

Das lokale vollständige Raw-Master-Image reproduziert die zuvor direkt am Medium ermittelten Strukturmerkmale:

* Gesamtgröße `31666995200` Byte;
* DOS-Disk-ID `0x4e6f7653`;
* identische Partitionsgeometrie;
* FAT16-/Swap-/ext4-UUIDs;
* i.MX-IVT bei `0x400`;
* rohen rc12-SPL mit `c5efead`.

Die Qualifikation endete mit `MASTER_SOURCE_GATE=PASS` und `HISTORICAL_ANALYSIS_SOURCE=LOCAL_VERIFIED_RESCUE_IMAGE`.

Seit diesem Checkpoint wird das physische historische USB-Medium nicht mehr als reguläre Analysequelle verwendet.

Weitere Untersuchungen erfolgen am lokalen verifizierten Raw-Master oder an davon kontrolliert abgeleiteten read-only Ansichten.

## Terminologie des Backups

Das lokale Abbild wird als `verifiziertes vollständiges Raw-Master-Image` bezeichnet.

Die Untersuchung erhebt nicht den Anspruch einer formalen forensischen Akquisition im kriminaltechnischen Sinn.

Diese Terminologie vermeidet eine stärkere Aussage, als durch den tatsächlich dokumentierten Erfassungsprozess gerechtfertigt ist.

## Zusammengeführte Provenienzbewertung

### Stark gestützt

Stark gestützt ist:

* Das Root-Dateisystem stammt aus der historischen Novena-/Kosagi-Systemlinie.
* Seine Struktur entspricht der historischen MMC-/USDHC-Linie.
* Die Builderhistorie ordnet `0x4e6f764d` der normalen MMC-Linie und `0x4e6f7653` der SATA-Linie zu.
* Die Installerhistorie bestätigt unabhängig dieselbe NovM-/NovS-Trennung.
* Die historische Installerlinie wurde auf Basis des normalen `novena-mmc-disk-r1.img` konzipiert.
* Die Rootfs-UUID des untersuchten Mediums besitzt eine auffällige exakte Entsprechung in der historischen Installer-Git-Historie.

### Direkt widersprochen beziehungsweise nicht unterstützt

Nicht unterstützt ist die Aussage: „Das heutige USB-Medium ist ein unverändertes Factory Image.“

Dagegen sprechen insbesondere:

* spätere Rootfs-Zeitstempel und Änderungen;
* mehrere unterschiedliche U-Boot-Generationen;
* die hybride Kombination aus SATA-Disk-ID und MMC-Struktur.

Ebenfalls nicht unterstützt ist: „Das Medium ist das unveränderte Ergebnis eines einzelnen normalen SATA-Builderlaufs.“

Die 32-MiB-Swapgröße und die USDHC-`fstab` widersprechen dieser einfachen Interpretation.

Auch ein unveränderter einzelner normaler MMC-Builderlauf erklärt den heutigen Zustand nicht, weil die Disk-ID `0x4e6f7653` der historischen SATA-Linie entspricht.

## Offene Frage Q1 – Entstehung des Hybridzustands

Offen bleibt, warum das historische Medium gleichzeitig folgende Merkmale besitzt:

* `NovS`-/SATA-Disk-ID `0x4e6f7653`;
* MMC-artige 32-MiB-Swappartition;
* MMC-/USDHC-`fstab`;
* ein historisches Novena-Rootfs.

Mehrere historische Mechanismen zeigen, dass Partitionstabellen, Bootloader und Bootdateien im Laufe der Zeit verändert werden konnten.

Keine der bisher gefundenen Quellen beweist jedoch die konkrete Transformation dieses individuellen Mediums.

Status: `Q1_HISTORICAL_USB_HYBRID_TRANSFORMATION=UNRESOLVED`

## Frage Q2 – NovM-/NovS-Zuordnung

Die Kombination aus historischer Builderquelle, `mmc-install.sh` und späterer Installerquelle reicht aus, um die normale historische Semantik zu klassifizieren:

    NovM / 0x4e6f764d = MMC beziehungsweise Recovery
    NovS / 0x4e6f7653 = SATA beziehungsweise SSD-Rootfs

Status: `Q2_NOVM_NOVS=RESOLVED`

Diese Klassifikation erklärt die Bedeutung der Kennungen. Sie erklärt noch nicht, warum das individuelle historische USB-Medium die SATA-Kennung mit einer MMC-artigen Struktur kombiniert.

## Offene Frage Q3 – exakte r1-Geometrie

Die öffentliche Dokumentation bestätigt:

* den Dateinamen `novena-mmc-disk-r1.img`;
* seine Prüfsummen;
* die Größenordnung von ungefähr `2.3GB`;
* das vollständige Schreiben per `dd`;
* die Möglichkeit einer anschließenden optionalen Rootfs-Vergrößerung.

Nicht bekannt sind weiterhin die bytegenau verifizierten Partitionsgrenzen des ursprünglichen veröffentlichten r1-Images.

Status: `Q3_R1_EXACT_PARTITION_GEOMETRY=OPEN`

Zur endgültigen Klärung wird eine verifizierte Kopie des Originalimages oder eine gleichwertige zeitgenössische Primärquelle mit exakter Partitionsgeometrie benötigt.

## r1-Retrievability

Die historischen Prüfsummen des r1-Images sind bekannt.

Eine bytegenau verifizierte vollständige lokale Kopie des veröffentlichten `novena-mmc-disk-r1.img` liegt zu diesem Checkpoint nicht vor.

Historische Webarchiv-Metadaten wurden gefunden, reichen jedoch nicht aus, um eine vorhandene Kopie als byteidentisch mit dem dokumentierten r1-Image zu qualifizieren.

Deshalb wird weder behauptet, dass das Image heute sicher öffentlich abrufbar ist, noch dass es in Webarchiven definitiv nicht vorhanden ist.

Die Suche nach einer erhaltenen Kopie bleibt zulässig, sofern eine gefundene Datei vor jeder weiteren Verwendung gegen die dokumentierte SHA-256-Prüfsumme geprüft wird.

## Aktuelle Schutzregeln

Für die weitere historische Medienuntersuchung gelten:

1. Das physische historische USB-Medium bleibt aus der normalen Analyse ausgeschieden.
2. Keine unnötigen weiteren Voll-Lesedurchläufe des physischen Mediums.
3. Primäre Analysequelle ist das verifizierte lokale Raw-Master-Image.
4. Das Partial-Image des ersten fehlgeschlagenen Leseversuchs bleibt als unabhängiger Präfixbeleg erhalten.
5. Das Raw-Master wird nicht in-place verändert.
6. Schreibende Dateisystemreparaturen am Master sind unzulässig.
7. Historische Git-Repositories werden getrennt vom aktiven NixOS-Projektrepository gehalten.
8. Beobachtete Tatsachen, Quelleninterpretationen und nicht nachgewiesene Kausalzusammenhänge werden getrennt dokumentiert.
9. Das historische USB-Medium wird nicht pauschal als „unverändertes Factory Image“ bezeichnet.
10. Die historische Medienuntersuchung darf die abgeschlossene I2C3-/ES8328-Untersuchung nicht wieder öffnen.
11. Historische Befunde dürfen nicht ohne eigenen Nachweis als Erklärung des aktuellen P_EXT-/Allocator-Falls `7C` verwendet werden.

## Aktueller Checkpoint

Historischer Checkpoint vom 2026-09-28; die späteren Präzisierungen vom
05./06.10. und DOC-UPDATE-02B vom 11.10. sind getrennte Nachträge.

Zum 2026-09-28 gilt:

    LOCAL_RAW_MASTER=VERIFIED
    PHYSICAL_HISTORICAL_USB=RETIRED_FROM_ACTIVE_ANALYSIS
    Q1_HISTORICAL_USB_HYBRID_TRANSFORMATION=UNRESOLVED
    Q2_NOVM_NOVS=RESOLVED
    Q3_R1_EXACT_PARTITION_GEOMETRY=OPEN
    R1_VERIFIED_LOCAL_COPY=NO
    CAUSAL_LINK_INSTALLER_TO_HISTORICAL_USB=NOT_PROVEN

Damit ist die historische Untersuchung ausreichend konsolidiert, um als dokumentierter Projektcheckpoint erhalten zu bleiben.

Eine spätere Fortsetzung soll nur erfolgen, wenn neue Primärquellen hinzukommen, insbesondere:

* eine gegen SHA-256 verifizierbare Kopie von `novena-mmc-disk-r1.img`;
* zeitgenössische exakte Partitionsmetadaten dieses Images;
* oder eine Quelle, die die konkrete Transformation des untersuchten historischen Mediums belegt.

## 2026-10-05 – BLOCK 9C-52C: Factory-R1-Präzisierung

Ergänzung zum historischen Stand oben, auf Basis des vom Benutzer bestätigten
Konsolidierungsstands 9C-51A/9C-52B gemäß Auftrag 9C-52C; keine neue
Onlineprüfung. Eigenständige lokale Originalberichte dieser beiden Blöcke
wurden hier nicht gefunden. Quellen- und Provenienzgrenzen stehen im
[kanonischen Quellenverzeichnis](QUELLENVERZEICHNIS.md#2026-10-05--block-9c-52c-konsolidierter-historischer-quellenstand).

BELEGT im bestätigten Quellenstand: `novena-mmc-disk-r1.img` wurde als
Novena-Auslieferungsimage verwendet. Name, historische URL und SHA256/MD5
sind im Abschnitt zum veröffentlichten Image festgehalten. Historisch belegt
ist das Shrink/Expand-Konzept sowie die Factory-Prozedur: `dd`, Neuerzeugung
von p3 bis zum Kartenende, `fsck`, `resize2fs`. Dies präzisiert den bisherigen
Hinweis auf nachgelagerte Vergrößerung, beweist aber keinen exakten
Release-Shrink-Schritt. Die OpenPGP-Signatur-Paketzeit ist
`2014-12-22 07:35:33 UTC`; kryptographische Gesamtprüfung bleibt offen.
Die Paketzeit belegt kein genaues Erstellungsdatum des Imageinhalts.

STARK BELEGT: Debian-U-Boot-Version `2014.10-novena-rc12` und
Factory-/shipped-Kernel `3.17.0-rc5-00217-gfd79638`.
Der stärkste U-Boot-Commit-Kandidat ist Tag `v2014.10-novena-rc12`,
`c5efeadb913c8246be372054adcd6106ad4fe067`. Seine belegte Tagzuordnung und
sein Nachweis im rohen SPL des separaten historischen USB-Mediums beweisen
nicht den exakten Commit im R1-Binärimage. Spätere Bootdateien, Kernel- oder
Bootloaderupdates sind kein rückwirkender Nachweis des R1-Inhalts.

```text
FACTORY_R1_BINARY_RECOVERED=NO
FACTORY_R1_EXACT_UBOOT_COMMIT=NICHT_BELEGT
```

Keine hashverifizierte wiedergefundene Binärkopie; exakte Original-
Partitionierungsgeometrie, bytegenaue Kernelzuordnung und exakter
Release-Shrink bleiben offen. Negative Suche beweist keinen Verlust des
Images. C3D2s Factory-Image-Verweis und der 31C3-Ereignisanker belegen
keinen heutigen Besitz oder ein dortiges Archiv des R1-Binärimages.
Historische Medienprovenienz und aktuelle Patch0010-DDR-Beobachtungen
bleiben getrennt; keine Root Cause folgt aus dieser Präzisierung.

## 2026-10-06 – BLOCK 9C-52E: Builder-, Boot- und RAM-Quellencheckpoint

Konsolidierung der abgeschlossenen lokalen Quellenanalysen 9C-52D-1/2R;
keine neue Medienanalyse und kein Hardwaretest. Vollständige Quellidentitäten,
Paketliste und Evidenzgrenzen im
[Quellencheckpoint](QUELLENVERZEICHNIS.md#2026-10-06--block-9c-52e-abgeschlossener-historischer-quellencheckpoint).

BELEGT: `/home/loomit/novena-historical-research/novena-image`, master HEAD
`f62c4f3b452199ab882e2c3dba16d4e31dfb1d56`, Origin
`https://github.com/xobs/novena-image.git`, vollständige lokale Historie
mit 36 Commits seit `a9d5066d55f2cdbdef1589f2bd12ca1e176ac2bb` (2014-10-10).
`24a84d8c92e3583dffcbbe7dcc424dc94884e29f` (2014-10-15) implementiert
`loopback_size=3965190144` und `truncate "-s${imgsize}" "${imgname}"`.
**BELEGT ist 3965190144 Byte als logische Gesamtgröße der erzeugten
Builder-Image-Datei. NICHT BELEGT ist diese Größe als veröffentlichte
Factory-R1-Release-Dateigröße.**

BELEGT im Builder: MBR, `fdisk -C32 -H32`, MMC `0x4e6f764d`, SATA
`0x4e6f7653`, p1 FAT `/boot`, p2 swap, p3 ext4 root bis zum verbleibenden
Ende; Entwicklung der Boot-Partitionsgröße `32M -> 64M -> 32M`.
QUELLENLUECKE: exakte Factory-R1-Sektorgeometrie, da fdisk-Defaults,
historisches Tooling und konkreter Release-Lauf nicht vollständig qualifiziert
sind. NEGATIVBEFUND der vollständigen lokalen Historie: kein implementierter
Release-Shrink mittels `resize2fs`, `sfdisk` o. ä. Das beweist keinen fehlenden
externen Releaseprozess. `dd` im Repository ist für SPL bzw. spätere
Stichprobenprüfung belegt, nicht als Release-Shrink.

`mmc-install.sh` in `8b64953` (2014-11-04) nennt u. a. U-Boot rc12,
Novena-Kernelpakete 1.14-r1, kosagi-repo 1.0-r1 und kosagi.key; vollständige
relevante Liste im Quellenverzeichnis. Paketchronologie ist kein vollständiges
Factory-R1-Manifest. Dezember-2014-Side-Branch-Commits `92ba999`, `444744c`,
`4baf00a`, `53e2521` gelangen erst Dezember 2015 über `29d5ed5`/`f62c4f3`
in master. Heutiges master entspricht damit nicht dem belegten Masterzustand
von Dezember 2014.

BELEGT durch die lokale Main-Page-Primärkopie im Abschnitt „Disk Imaging“:
Factory-Auslieferungsimage, Hashidentität, geschrumpftes ext4-Design und
Factory-Imaging/Expand-Prozedur (`dd`, p3-Neuerzeugung, `fsck`, `resize2fs`).
Signaturbytes liegen dort ebenfalls vor; Paketzeit 2014-12-22 07:35:33 UTC
ist kein Image-Erstellungsdatum und keine kryptographische Gesamtverifikation.

OpenBSD (`openbsd-src`, initial `31642defb5b3233700d202713f8a65d872d823c3`,
2015-05-08) liefert unabhängige reale Test-/Betriebsbelege und einen
4-GB-RAM-Bericht (`a5b7bc8b7df`; U-Boot-Start `0x10000000`, Größe
`0xf0000000`). Getestete SDHC-Unterstützung ist belegt, exakter vollständiger
SD-Bootablauf und vollständiger lokaler Bootlog fehlen. Der untersuchte Pfad
übernimmt bootloaderinitialisierten RAM; keine eigene Novena-DDR-Initialisierung.
Gentoo (`gentoo-on-novena`, HEAD `f49fcab518c5bb4b2fb0e90baecf8b504209bfff`)
dokumentiert spätere bootfähige microSD-Gesamtimages v1.0.0/v1.0.1,
libre/standard, v1.0.1 etwa 770/797 MiB komprimiert und 6 GiB entpackt,
p1 vfat `/boot`, p2 swap, p3 ext4 `/`, Linux 4.7.2 und reale Board-/Desktop-
Tests, ohne vollständigen lokalen seriellen Bootlog. U-Boot 2014.10.8 stammt
über novena-overlay aus der xobs-Linie; keine unabhängige DDR-Initialisierung.
Die deklarierte Tag-/Commitquelle des Overlays ist nachvollziehbar, das
ursprüngliche Release-Archiv damit nicht nachträglich kryptographisch verifiziert.

Diese Quellen beweisen keinen identischen Factory-R1- oder aktuellen
NixOS/Patch0010-Bootpfad, keine DIMM-/SPD-/DDR-Identität und keine C1/Q1-Ursache.

```text
FACTORY_R1_RELEASE_BUILDER_IDENTIFIED=PARTIAL
FACTORY_R1_RELEASE_SHRINK_PROCEDURE_IDENTIFIED=NO
FACTORY_R1_EXACT_IMAGE_SIZE_EXPLAINED=NO
FACTORY_R1_EXACT_PARTITION_GEOMETRY_EXPLAINED=NO
FACTORY_R1_BINARY_RECOVERED=NO
FACTORY_R1_FULL_CRYPTO_VERIFY=OPEN
FACTORY_R1_EXACT_UBOOT_COMMIT=NICHT_BELEGT
FACTORY_R1_EXACT_PACKAGE_MANIFEST=QUELLENLUECKE
FACTORY_R1_EXACT_PARTITION_GEOMETRY=QUELLENLUECKE
FACTORY_R1_RELEASE_SHRINK_PROCEDURE=QUELLENLUECKE
SIGNED_APT_INDEX_CHAIN=QUELLENLUECKE
FACTORY_DIMM_SPD_IDENTITY=QUELLENLUECKE
FACTORY_R1_IMAGE_EXISTENCE=PROVEN
FACTORY_R1_DEPLOYMENT=PROVEN
FACTORY_R1_HASH_IDENTITY=PROVEN
FACTORY_R1_SHRUNK_EXT4_DESIGN=PROVEN
FACTORY_IMAGING_EXPAND_PROCEDURE=PROVEN
```


## 2026-10-17 – NOVENA-DEBIAN-DOC-01B: Bootvertrag der Debian-Paketlinie 2015

Dieser additive Nachtrag integriert NOVENA-DEBIAN-REF-09 bis REF-14.
Exakte Originalprotokolle, Sidecarabgleich, Binärhashes, Herkunft und Git-Blobs
stehen im [Quellenregister](QUELLENVERZEICHNIS.md#2026-10-17--novena-debian-doc-01b-ref-09-bis-ref-14-quellenregister).
Die folgenden Aussagen unterscheiden Quellcodebefund, Binärindiz und offenen
Laufzeitnachweis. Es fand hier kein Build, Test, Medienzugriff oder Hardwareboot statt.

### Getrennte Generationen und Referenzbindung

Der USB-Raw-Master enthält keinen einheitlichen Bootzustand: roher rc12-SPL
`c5efeadb913c8246be372054adcd6106ad4fe067` vom Oktober 2014,
FAT-SPL rc14 `3d26413e65f985131a9573d99a51fd47e7253188` vom Dezember 2014
und spätere Rootfs-Paketdateien von September 2015 bleiben getrennt.
Die FAT-Zeitstempel von Februar 2015 datieren nicht automatisch deren Quellstand.
Die analysierten 2015-Dateien stammen aus `/usr/share/u-boot-novena/` auf p3,
nicht aus dem rohen Bootbereich oder der FAT-p1. Paketbezeichnung
`2014.10.r7-novena.1` und Banner `2015.04-rc1-02853-gbd5c7d3` bezeichnen hier
denselben untersuchten Paketkontext, keine Gleichsetzung mit rc12/rc14.

**Alle Quellstellen dieses Nachtrags gelten, soweit nicht ausdrücklich anders
angegeben, für `xobs/u-boot-novena` am exakten Commit
`bd5c7d3290bb94bf8af17760c321df67d2641343` vom 2015-09-04.**
Der separate SOURCE-18B-Befund zum 2020.07-SPL (USDHC3-Pfad belegt,
USDHC2-Pfad nicht belegt) bleibt unverändert und wird nicht durch 2015-Code widerlegt.

REF-09 §9 präzisiert die Abstammung: Die Oktober-2014-Commits
`b5a15faf85a52789308f3f59898693b005532833` und
`4facac7043bd45f57956b5a0084cb02aaea59bae` sind keine Vorfahren des
2015-Referenzcommits. Dagegen ist
`a0b19a414a2adf02dc11827f13ba9a035734d44a` vom 2015-02-12
(`novena: Support SPL loading from external MMC`) dessen Vorfahr;
der gleichbetitelte `b2ad57b4a3c1f47877c9f84ec21915d76fe14183` ist es nicht.
Die ältere Aussage zur Oktober-Unterstützung in der rc12-Linie bleibt bestehen;
sie ist keine identische Abstammungsbehauptung für die spätere Linie.

### SPL: externe SD, Controller und FAT-Dateivertrag

**Quellcodebelegt (REF-09/10):** `board/kosagi/novena/novena_spl.c:88–129`
liest `SRC_SBMR1` über `SRC_BASE_ADDR + 0x4`; neben den Boottypfeldern wird
Bit 12 ausgewertet. `board_mmc_init()` in derselben Datei, Zeilen 468–508,
registriert abhängig von dieser Quelle genau den gewählten Controller:
interne SD `MX6_SD0_BOOT` über USDHC3, externe SD `MX6_SD1_BOOT` über USDHC2.
Der SATA-Zweig meldet fehlende Unterstützung und hält an.

Davon getrennt bestimmt `arch/arm/imx-common/spl.c:18–90` die generische
Bootklasse aus SBMR1 beziehungsweise, bei gesetztem GPR10-Bit 28, GPR9.
SD/MMC wird als `BOOT_DEVICE_MMC1` behandelt. Diese Bezeichnung ist weder
eine USDHC-Controllernummer noch die spätere Proper-Gerätenummer.
`common/spl/spl_mmc.c:88–104` sucht nach der Boardinitialisierung
`find_mmc_device(0)`, weil nur ein Gerät registriert wird. Die externe SD
ist im SPL daher Gerät 0 des ausgewählten Controllers.

**Quellcodebelegt (REF-11/12):** `include/configs/novena.h:129–131` aktiviert
SPL-MMC und SPL-FAT und bindet `imx6_spl.h` ein. Dadurch liefert
`spl_boot_mode()` Dateisystemmodus. `include/configs/imx6_spl.h:45–61`
legt Partition **1** und Payload **`u-boot.img`** fest.
`common/spl/spl_mmc.c:125–149` führt zum FAT-Lader;
`common/spl/spl_fat.c:41–70` liest zunächst den Imageheader, wertet ihn aus
und lädt anschließend die Datei an die daraus bestimmte Adresse.
Der ebenfalls definierte Raw-Sektor 138 (69 KiB) ist kein Beleg für einen
hier primär ausgewählten Raw-Proper-Ladepfad. SPL bei Medienoffset `0x400`
und Proper als FAT-Datei sind verschiedene Verträge. Eine Datei
`u-boot.spl` auf FAT ersetzt nicht automatisch den ROM-geladenen rohen SPL.

**Binärindiz:** Die untersuchten SPL-Strings enthalten `External SD card`,
`Internal SD card` und den bd5-Banner. Sie stützen die Zuordnung, ersetzen
aber keinen vollständigen Binär-/Quellnachweis und keinen P_EXT-Lauf.

### U-Boot proper: andere MMC-Nummerierung und persistentes Environment

`board/kosagi/novena/novena.c:165–212` initialisiert USDHC3 vor USDHC2:
vorgesehen sind intern MMC 0, extern MMC 1. Ein Initialisierungsfehler kann
die Schleife vorzeitig beenden; daraus wird kein erfolgreicher Lauf ohne
interne Karte abgeleitet. `set_bootdev()` (Zeilen 509–537), aufgerufen aus
`misc_init_r()` (578–592), setzt intern `bootsrc=mmc, bootdev=0`, extern
`bootsrc=mmc, bootdev=1`, SATA `bootsrc=sata, bootdev=0` (REF-10).
Diese Proper-Reihenfolge darf nicht auf das einzelne SPL-Gerät übertragen werden.

`include/configs/novena.h:79–96` konfiguriert bei MMC-Unterstützung
persistentes Environment auf **MMC 0**, 16 KiB pro Kopie, primär bei
`0x80000` (512 KiB), redundant bei `0x84000` (528 KiB).
`common/env_mmc.c:205–289` behandelt die redundanten Kopien;
`common/board_r.c:772–806` ordnet das Environment-Laden vor `misc_init_r()`
ein (REF-14). Die externe Bootquelle schaltet den Environment-Speicherort
also nicht automatisch auf MMC 1 um. Default-Fallback bei fehlendem oder
ungültigem Environment ist Quelllogik, kein bewiesener unabhängiger Gesamtboot.
Die tatsächlich vorhandenen persistenten Werte eines künftigen Referenzmediums
und das Verhalten ohne interne MMC bleiben offen.

### EEPROM, doppelte Defaults und Root-Auswahl

`novena.c:372–400` definiert ein 16-Bit-Featurefeld und
`feature_rootsrc_sata=0x0100`; die Gültigkeitsprüfung vergleicht die sechs
Signaturbytes mit `Novena`. `set_bootdev()` setzt bei gültigen Daten und
gesetztem Feature `sata_root=yes` (533–534). Bei fehlendem Feature wird ein
bereits vorhandener Wert dort nicht ausdrücklich gelöscht. Das ist eine
mögliche Einflussgröße aus persistentem Environment, keine Beobachtung eines
konkreten EEPROM-/Environmentzustands (REF-13/14).

`include/configs/novena.h:263,313` enthält zwei `rootdev`-Defaults:
früh `/dev/mmcblk0p2`, später `PARTUUID=4e6f764d-03`; beide stehen auch in den
Binärstrings. Der ergänzend lokal gelesene Importpfad
`common/env_common.c:120–123` und `lib/hashtable.c:225–265,916–928` verwendet
Hash-Einträge mit `ENTER` und Überschreiben bestehender Werte. Unter erfolgreichem
Defaultimport ohne abweisenden Callback folgt daraus der spätere NovM-Wert.
Dies ist eine statische Schlussfolgerung über den Defaultimport, keine Messung
des effektiven Laufzeitwerts. Persistentes Environment, Boardlogik, spätere
Imports und Hooks bleiben zusätzlich zu berücksichtigen.

In `novena.h:370–379` führt ein nichtleeres `sata_root` ohne Recovery zur
SATA-Auswahl (`devtype=sata`, `devnum=0`) und setzt
`rootdev=PARTUUID=4e6f7653-03`; sonst werden Scanquelle und Gerätenummer aus
`bootsrc/bootdev` übernommen. Danach wird `root=${rootdev}` an `bootargs`
angehängt. Ein externer SPL-/Proper-Start garantiert somit kein externes Rootfs.

Die historische Semantik bleibt **NovM / `0x4e6f764d` = MMC/Recovery** und
**NovS / `0x4e6f7653` = SATA-/SSD-Rootfs**; `-03` bezeichnet Partition 3,
nicht eine Dateisystem-UUID. Die NovS-ID des USB-Masters beweist keinen
SATA-Boot und widerspricht seiner dokumentierten MMC-fstab nicht als Beobachtung:
beides gehört zum erhaltenen Hybridzustand. Für ein externes Referenzmedium
sind tatsächliche Partitionierung, effektives `root=` und mögliche
PARTUUID-Kollisionen mit anderen Medien gesondert zu qualifizieren.

### uEnv.txt, Hooks, extlinux und direkter Fallback

`novena.h:308–341` lädt `uEnv.txt` über
`load ${bootsrc} ${bootdev} ${loadaddr} ${bootenv}` und importiert bei
erfolgreichem Laden mit `env import -t -r $loadaddr $filesize`.
Die Hilfetextformulierung „internal MMC“ ersetzt diese variable Quellenwahl
nicht. Danach läuft `earlyhook`, sofern gesetzt. `dolatehook` (331–339),
aufgerufen in Zeile 381, importiert bei gesetztem `finalhook` den Speicherinhalt
erneut und führt dann `finalhook` aus. Fortbestehende Gültigkeit von
`loadaddr/filesize` und Wirkung des erneuten Imports sind ohne gesonderten
Nachweis nicht vorauszusetzen (REF-14).

`include/config_distro_bootcmd.h:155–193` definiert Präfixe `/` und `/boot/`;
pro Präfix wird zuerst `extlinux/extlinux.conf` über `sysboot` gesucht,
danach werden `boot.scr.uimg` und `boot.scr` geladen und ausgeführt.
Der Novena-Pfad ruft `scan_dev_for_boot` mit `bootpart=1` direkt auf;
das ist kein Nachweis eines Scans aller Partitionen oder sämtlicher Bootmedien.
`novena_boot` stellt dem extlinux-Kommando `usb start` voran. Ein vorhandener
USB-Startbefehl belegt weder die Herkunft des Rootfs noch einen USB-Gesamtboot.

Kehrt der Scan zurück, folgt in `novena.h:383–390` der direkte FAT-Fallback:
`zImage${rec}` an `kernel_addr_r=0x12000000`,
`novena${rec}.dtb` an `fdt_addr_r=0x11ff0000`, danach `fdt addr` und
`bootz` mit `${initrd_addr_r}` (Default `-`, also ohne initrd).
Recovery setzt `rec=.recovery`, entsprechend `zImage.recovery` und
`novena.recovery.dtb`. Anders als der Scan verwendet dieser Fallback weiterhin
`${bootsrc} ${bootdev}`, auch nach SATA-Root-Auswahl. Scanquelle, direktes
Kernel-/DTB-Lademedium und Rootquelle können daher auseinanderfallen.
Der erhaltene FAT-Dateiname `zimage` bleibt ein Medienbefund; er wird hier
nicht umbenannt. Die semikolongetrennte Folge belegt keinen erfolgreichen
Ladevorgang und keine validierten Kernel-/DTB-/initrd-Artefakte.

### Offene Voraussetzungen für einen eigenständigen P_EXT-Referenzboot

Statisch belegt ist der oben beschriebene **Quellvertrag** der 2015-Linie.
Nicht belegt sind insbesondere:

- vollständige Binär-zu-Quellcode-/Buildprovenienz des konkreten Paars;
- qualifizierter ROM-/SPL-/Proper-Übergang samt Header-, Lade-, Entry- und
  Speicherbereichsprüfung für das konkrete externe Referenzimage;
- erfolgreiche Proper-Initialisierung und Environmentbehandlung ohne erforderliche
  interne MMC oder interne Boot-/Root-Dateien;
- tatsächlich wirksames Environment einschließlich EEPROM-SATA-Einfluss,
  `uEnv.txt`, beider Hooks, Scanquelle und eindeutig externer Root-Auswahl;
- Kernel-, initrd- und DTB-Identitäten, Formate, Größen, Speicherbereiche und
  kompatibler Übergabevertrag bis zum Debian-Rootfs auf derselben externen SD;
- qualifiziertes externes Gesamtimage und vollständiger Hardware-P_EXT-Boot.

Die ursprünglichen Extraktionsbelege sind inzwischen im Quellenregister benannt;
die weitergehende ursprüngliche Build-/Paketprovenienz bleibt offen.
Factory-R1-Herkunft, Releasegeometrie und Gleichheit mit dem USB-Hybrid werden
nicht nachträglich bewiesen. Es wird kein Kandidat pauschal als Original oder
GOOD eingestuft. NOVENA-STRATEGY-02 bleibt maßgeblich; dieser Nachtrag trifft
keine neue Bootloaderentscheidung. Die moderne DDR-Root-Cause bleibt offen,
I2C3/ES8328 funktional geschlossen; `regulator-always-on` bleibt Boardanforderung.
