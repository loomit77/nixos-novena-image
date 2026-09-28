# Historische Novena-Medien und Image-Provenienz

Stand: 2026-09-28

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

Das historische `xobs/novena-image`-Repository enthält in seiner späteren Builderhistorie eine feste Loopbackgröße von `3965190144` Byte.

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
