Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Quellcode und Wechselwirkungen im Bootpfad

## 1. ROM, SPL und U-Boot proper sind getrennte Phasen

U1 ändert SPL-Controller und Clock von USDHC3 auf USDHC2 (`boot/u-boot/0001-novena-spl-boot-second-stage-from-external-sd.patch:7`). SPL registriert einen Controller; das Log `MMC1` und `dev=0` widerspricht deshalb nicht automatisch dem proper-Script `mmc 1:1`. Proper und Linux enumerieren eigenständig. BELEGT: externe-SD-ROM/SPL-Einstieg laut Testdokumentation; vollständiger proper-Einstieg unter P_EXT unbelegt.

REF `common/spl/spl_mmc.c` lädt nach erfolgreicher Gerätinitialisierung im FS-Modus aus Partition 1; .config nennt `u-boot-dtb.img`. `spl_mmc_boot_mode()` wählt bei FAT/EXT4-Unterstützung FS. Die vorhandene RAW-Sektoroption 0x8a allein beweist keinen tatsächlich ausgeführten RAW-Boot. Bootfähigmarkierung von P2 im MBR ersetzt nicht die explizite SPL-FS-Partition 1 (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/common/spl/spl_mmc.c:290`; `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/.config:659`; `image/novena-image.nix:124`).

## 2. DDR: feste aktuelle Vorgaben und historisches SPD

BELEGT: aktueller Boardpfad setzt I/O für 64 Bit, feste sysinfo und `elpida_4gib_1600`, verzögert 100 µs und ruft WL/DQS (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:544`, `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:583`). Die Boarddatei enthält in diesem Pfad keinen SPD-Aufruf. Historical rc5 liest SPD zuerst (`research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/novena_spl.c:551`).

| Parameter | Aktueller REF | Historischer rc5-Grundwert / SPD-Überschreibung |
|---|---|---|
| dsize | 2 / 64 Bit | zunächst 2; SPD akzeptiert 64-Bit-Module |
| ncs | 1 | zunächst 1, dann aus organization |
| cs_density | 32 | zunächst 32, dann Kapazitätsberechnung |
| rtt_wr | 0 | Grundwert 1, nach SPD 2 |
| rtt_nom | 1 | Grundwert 2, nach SPD 2 |
| walat / ralat | 0 / 5 | Grundwert 3 / 7, nach SPD 3 / 3 |
| bi_on | 1 | Grundwert 1, nach SPD 0 |
| trcd / trcmin / trasmin | 1375 / 4875 / 3500 | Grundwert 1300 / 4900 / 3590, dann SPD-berechnet |
| rows / cols / pagesz | 16 / 10 / 2 | Grundwert 16 / 10 / 2; rc5 pagesz pauschal 1 |

Fundstellen: `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:493`, `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:513`, `research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/ddr3.c:805`; historische und aktuelle vollständige Boardkonstanten im jeweiligen novena_spl.c. Tabelle beschreibt Quellwerte, keine gemessenen Register oder das konkrete DIMM. Initialwerte, nach-SPD-Werte und nach-Kalibrierung-Werte dürfen nicht vermischt werden.

Die gleichen vorkonfigurierten WL-/Gating-/Read-/Write-Werte finden sich in beiden Boarddateien. Das beweist keinen gleichen Endzustand, weil Geometrie, Terminierung, Helfer und Kalibrierung unterschiedlich wirken können. `mem_speed=1600` ist ein Bausteinrating: der aktuelle Helfer begrenzt MX6Q/D auf 1066 MT/s (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/arch/arm/mach-imx/mx6/ddr.c:1299`). Eine Behauptung tatsächlichen DDR-1600-Betriebs wäre falsch.

WIDERSPRUCH innerhalb von Kommentaren und Werten: aktuelles `rtt_wr=0` ist mit Kommentar RZQ/4 und `rtt_nom=1` mit RZQ/2 versehen; die Helferkodierung nennt für nominal 1 RZQ/4, 2 RZQ/2. Es werden Zahlen in Registerfelder umgesetzt, keine Kommentare (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:493`; `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/arch/arm/mach-imx/mx6/ddr.c:1472`, MR2/MR1-Kodierung ab Zeile 1544). Das ist eine Dokumentationsinkonsistenz; Herstellerdekodierung und tatsächlich geeignete Werte bleiben zu verifizieren. Kein automatischer Parameterfix.

Historisches SPD hat selbst Annahmen: rc5 pauschales pagesz=1, kein vor der MTB-Division erkennbarer Nullschutz des Divisors, akzeptierte Busbreite hart eingeschränkt. Spätere Git-Stände korrigieren page size und enthalten `ddr3_spd_check()` sowie canned SPD (14). Ein Rückport des alten rc5-Pfads wäre deshalb keine selbstverständlich robuste Universalitätslösung (`research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/ddr3.c:903`; `research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/ddr3.c:879`).

## 3. Kalibrierung und Fehlerverträglichkeit

BELEGT: WL-/DQS-int-Rückgaben werden vom Board verworfen. Die elf einschlägigen Wait-Quellstellen liefern potentiell Timeout, werden aber ebenfalls ohne Auswertung fortgesetzt. Fehlerbitmasken und Wait-Vertrag sind getrennt; eine Maske 0 wäre kein vollständiger PASS (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena_spl.c:583`; `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/arch/arm/mach-imx/mx6/ddr.c:19`; `/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/include/wait_bit.h:34`; research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md:106 ff.). Keine realen Masken/Timeouts im Fall-8B-Log.

WL kann bei Soft-Fail alte Werte wiederherstellen und Fehlerbit 4 setzen. Deshalb bedeutet erfolgreicher Fortgang weder erfolgreiche Kalibrierung noch völliges Fehlen einer Rückfalllogik. Der heutige Boardpfad meldet diese Unterscheidung nicht. Historischer post-config CPU-Test und aktuelle MMDC-Kalibrierung sind unterschiedliche Prüfformen; der aktuelle Boardpfad enthält keinen gleichwertigen `novena_memory_test()`.

DQS `modify_dg_result()` subtrahiert 0xc0, der Kommentar nennt 0x80 (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/arch/arm/mach-imx/mx6/ddr.c:74`). BELEGT als Text/Code-Differenz, technische Richtigkeit ohne Herstellerbeleg unbekannt. MPDGHWST-Lesewerte werden verarbeitet; zusätzliche Diagnose-Lesungen sind ohne geklärte Semantik nicht automatisch passiv. ERR050070 bleibt eine Quellenfrage.

## 4. SRAM, BSS und Heap

REF/.config setzt SPL-Text 0x00908000, Stack 0x0091ffb8, BSS-Basis 0x18200000, Full-Malloc 1 MiB ab 0x18300000 und frühes malloc_f 8 KiB. Stack_R/Early_BSS sind nicht gesetzt. Die erhaltene Rekonstruktion belegt frühen SRAM-Stack und BSS-Clear nach board_init_f/Kalibrierung (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/.config:247`; `research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md:143`).

| Bereich | Intervall bzw. Wert | Aussage |
|---|---|---|
| gelinkte tatsächliche BSS | [0x18200000,0x1820015c) | laut Referenzmap/Checkpoint, 348 Byte |
| max_total_mem | 0x18200030 | Control, hartcodiert im Patch |
| mem_malloc_brk | 0x1820003c | Target, 12 Byte Abstand |
| Full-Malloc | [0x18300000,0x18400000) | Heap-Clear adressiert logisch keine BSS |
| SPL Raw-Payload | 49480 Byte | Wrapper ist davon verschieden |
| SPL auf Medium | [0x400,0xe000) | aus Offset und 56320 Byte berechnet |
| P1 | ab 0x800000 | kein logischer SPL-Partitionsüberlapp |

Keine offensichtliche virtuelle/physische Adressintervallüberschneidung beweist keine korrekte physische DDR-Adressdekodierung. Q1=0 wird vor Heap-Clear beobachtet; der spätere Heap-Clear erklärt alleine diese Beobachtung nicht. Frühere BSS-Stores könnten relevant sein, sind aber nicht als Ursache gemessen. SRAM-Stackreserve einer zukünftigen Instrumentierung wurde nicht neu qualifiziert.

Die DQS-Tail-Branch-Maschinenfolge ist im Checkpoint rekonstruiert, kein fehlender DQS-Aufruf. Eine Nach-DQS-Ausgabe würde Code und Timing ändern. Globale BSS-Diagnosewerte vor BSS-Clear könnten verloren gehen; Architektur A/B ist noch keine implementierte Lösung (07).

## 5. MMC und Allokation

REF `fsl_esdhc_initialize()` ruft zuerst priv-calloc auf; bei NULL folgt -ENOMEM, vor plat und Controller-Fortgang (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/drivers/mmc/fsl_esdhc_imx.c:1322`). Rohlogs zeigen D21A/B, danach D32/D33=-12, kein D21C. Der konkrete NULL-Pfad ist durch Code und Kontrollfluss gestützt; `priv=?` bleibt kein numerisch lesbarer Zeiger. Der Clockwert 198 MHz wird vor diesem Punkt gemeldet, ist kein SD-Kartentransfertakt und kein DDR-Taktnachweis.

Die erste beobachtete Störung liegt an CPU-Speicher-/Allocatorbeobachtungen; aus -ENOMEM folgt keine physische RAM-Erschöpfung und kein fehlendes SD-Protokollfeature. MMC-/FAT-/Payloadfehler nach erfolgreicher Allokation bleiben für den künftig noch unerreichten Pfad offen, sind jedoch keine Erklärung für die schon frühere Q1-Nullbeobachtung.

## 6. U-Boot proper, DT und PMIC

proper `dram_init()` nutzt `imx_ddr_size()` zur gemeldeten Größe; das ist keine SPD-Auswertung und kein voller CPU-Speichertest (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena.c:93`). EEPROM in misc_init_r setzt die MAC nach Signaturprüfung; I2C-/Lesefehler werden gemeldet und als nichtfatal behandelt. Hardwarefeature-/DDR-Erkennung folgt daraus nicht automatisch.

Zusätzlicher statischer Robustheitsbefund: `power_init_board()` prüft pmic_get/probe, wertet aber `power_pfuze100_init`, mehrere pmic_reg_read und pmic_reg_write nicht aus (`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree/board/kosagi/novena/novena.c:100`). HYPOTHESE eines späteren Fehler-/Recoveryrisikos, keine Fall-8B-Erklärung: proper ist in diesem Lauf nicht erreicht. Ob alle betreffenden API-Fehler real auftreten können, ist noch zu prüfen.

U-Boots eigener DT für Treiber und später geladener Linux-DTB sind getrennte Artefakte. Linux-DTB wird aus config.hardware.deviceTree.package übernommen; Displayoverlays enthalten Zahlen für Clocks/Pins, Graphverbindungen, STMPE-Disable und Audio-Always-on. Bei Kernelwechseln müssen Bindings, Clock-IDs und Symbole erneut geprüft werden (`hardware/novena.nix:48`).

## 7. Kernel/initrd/Rootfs

boot.cmd setzt systemConfig/init absolut auf die zugehörige Closure und lädt zImage nach 0x12000000, DTB nach 0x1f000000, initrd.uimg nach 0x20000000 (`image/novena-image.nix:22`). Legacy-Header wird mit mkimage -T ramdisk -C gzip erstellt, passend zum konfigurierten gzip-initrd. Bootz-Kompatibilität war historisch mit U-Boot 2015 beobachtet; damit ist aktuelles proper unter P_EXT noch nicht qualifiziert.

BELEGT: Script enthält drei unbedingte fatload-Kommandos, danach bootz; keine explizite if/&&-Prüfung. HYPOTHESE: bei fehlender/trunkierter Datei könnte ein Folgekommando mit ungeeigneten RAM-Inhalten arbeiten. Exakte Hush-Fehlerfortsetzung ist vor einer Root-Cause-Aussage am gepinnten Parser zu prüfen. Kein solcher Fehler ist im SPL-Lauf erreicht.

Die Ladeadressen liegen weit auseinander, aber vollständige Grenzen benötigen tatsächliche Dateigrößen, kernel decompression/relocation, proper reservierte Bereiche und RAM-Ende je Modul. Das DTB→initrd-Startintervall beträgt 16 MiB; `0x20000000` liegt 256 MiB über RAM-Basis `0x10000000`. Bei kleinen RAM-Varianten kann dies relevant werden. Keine behauptete aktuelle Überschneidung.

Root wird per Label NIXOS_SD gemountet (`hardware/novena.nix:7`); ein root= in bootargs ist daher nicht automatisch erforderlich. Welche Treiber die tatsächliche initrd enthält und wie die gepinnte NixOS-Stage1 daraus Mountparameter erzeugt, ist noch im Store zu qualifizieren. Doppeltes Label/UUID mehrerer angeschlossener Klone ist ein konkretes zukünftiges Auswahlrisiko.

## 8. Cross-Build und Imageerzeugung

BuildPlatform x86_64, HostPlatform armv7l; Werkzeugtypen buildPackages sind im Image explizit. Das Bashoverlay wirkt auf beide Crossvarianten, Kernelpatchliste enthält produktiv zwei Patches. `system.stateVersion` ist nicht der nixpkgs-Pin (`flake.lock:9`; configuration.nix).

MBR/Disk-ID konstant, P1 128 MiB ab 8 MiB, P2 direkt danach; rootBytes wird auf Sektoren aufgerundet, danach 1 MiB Reserve. SPL wird mit dd ab 1024 eingebettet. BELEGT: in dieser Image-Derivation kein explizites Größen-Gate gegen P1 für zukünftige SPLs und kein dynamisches Vergrößern von P1. Aktuelle dokumentierte Artefakte passen; zukünftiges Wachstum bleibt Risiko (`image/novena-image.nix:124`).

ext4 setzt Zeit, UUID und Verzeichnishashseed, shrinkt und ergänzt 16 MiB (`image/make-ext4-fs-reproducible.nix:109`). Das genügt nicht zum allgemeinen Reproduzierbarkeitsbeweis; dafür ist der damalige Bytevergleich maßgeblich. Im fsck-Fehlerpfad folgt `cat errorlog` ohne hier erkennbare Erstellung dieser Datei, danach `return 1` innerhalb eines BuildCommand-Shelltextes (`image/make-ext4-fs-reproducible.nix:96`). Der Fehlerpfad ist statisch fragwürdig und ungetestet; ein erfolgreicher Imagebuild validiert ihn nicht. Keine neue Ausführung.

FIRMWARE ist noauto/nofail. Ein späteres Kernelupdate aktualisiert die manuell erzeugten FAT-Bootdateien damit nicht allein durch einen normalen Systemwechsel. Recovery-/Updatevertrag muss separat entwickelt werden; aktuelle Imageerzeugung liefert nur den initialen zusammengehörigen Satz.
