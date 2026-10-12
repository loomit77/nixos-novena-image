Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Projektgeschichte mit getrennten Bootgenerationen

## Ziel und frühe Grundlage

BELEGT als Projektziel: selbstständig bootfähiges, reproduzierbares externes NixOS-SD-Image für Kosagi Novena, einschließlich ARMv7-Kernel, Novena-DTB, IT6251 und Innolux-Panel (`README.md:6`). V1 definiert Unabhängigkeit von vorhandenen internen Boot-/Root-Dateien; dies ist kein bereits belegter Nachweis beliebiger Novena-Speichermodule oder jeder Boardrevision (`docs/PROJECT-STATE.md:91`).

Die frühe Kernelgeneration 5.7.0-rc2 erreichte nach Projektbericht Linux, scheiterte aber mit `unknown-block(0,0)` am Root-Mount (`docs/TEST-LOG.md:5`). Aussagegrenze: Kernelstart war damals möglich; weder derselbe Fehler im aktuellen initrd noch eine aktuelle Root-Ursache folgt daraus. Historische NixOS-Wiki-Revision 21513 und Novena-next-Quellen liefern Vergleichsmaterial, keine aktuelle Buildanweisung (`research/02-historical-software/notes/nixos-wiki-kosagi-novena-revision-21513.md:3`).

## Historische Novena-Bootsoftware 2014–2015

Die archivierte rc5-Version (`9402489e9a3e52a9df5464b4ce3c792cee2ba3a5`, Oktober 2014) liest SPD vor der MMDC-Konfiguration, kalibriert und führt einen eigenen CPU-Speichertest aus (`research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/novena_spl.c:551`; Provenienzdatei im Audit 11). Sie verwendet in diesem Quellstand USDHC3 für den zweiten Bootabschnitt. Das historische Git enthält externe-SD-Erweiterungen und spätere DDR-Kalibrierungsänderungen; konkrete lokale Git-Belege stehen in 14. rc5 darf nicht mit dem 2015 auf der Hardware beobachteten bd5c7d3-Stand gleichgesetzt werden.

BELEGT als historische Medienrekonstruktion: Roh-SPL rc12, FAT-SPL rc14 und Rootfs-Paket-SPL von 2015 sind drei verschiedene Generationen (`docs/HISTORICAL-MEDIA.md:104`, `docs/HISTORICAL-MEDIA.md:200`). Der installierte Paketbestand ist kein Nachweis des tatsächlich vom ROM ausgeführten SPL. Das historische USB-Medium ist zudem nicht die aktuelle P_EXT-SD.

Die ursprüngliche Debian-Imageerzeugung arbeitet mit debootstrap/qemu-user und getrennten MMC-/SATA-Konventionen; das aktuelle Projekt verwendet NixOS-Cross-Derivationen. Der lokal vorhandene Debian-Installer beschreibt SATA-Zielinstallation, angepasste uEnv-Dateien und Kernel-Hooks. Seine bekannten Clock-Probleme bleiben historisch berichtete Bedingungen, keine Diagnose für Fall 8B (14; `docs/HISTORICAL-MEDIA.md:389`).

## September 2026: Display, Boardversorgung und Golden-Stand

BELEGT als dokumentierter Entwicklungspfad: Linux 6.18.49, DRM-Bridge-Patch und Panelpatch; früh falsche aktive Bildgrößen, später 1920×1080. Die isolierte Wirkung einzelner Vor-Git-DT-Korrekturen ist nicht vollständig erhalten (`docs/TEST-LOG.md:74`). Die Golden-Sicherung vom 11. September und ein erfolgreicher Hardwarelauf vom 12. September sind Referenzen, aber kein universeller Cold-Boot-Beweis (`docs/GOLDEN-BUILD.md:412`).

Die I2C-Untersuchung verdichtete den Cold-FAIL/Same-Boot-Rebind-Kontrast auf den Master-START-Übergang. `single-master` unterdrückte bestimmte softwareseitige IAL-Prüfungen; es belegte kein Verschwinden des Hardwareereignisses (`docs/TEST-LOG.md:107`). Die H3-1R-Serie dokumentiert 5/5 POR-Erfolge nach isolierter Ergänzung des ES8328-Always-on. Die historische Leakage-/Pull-up-Evidenz stützt den funktionalen Abschluss am 20. September (`docs/DECISIONS.md:108`; 05 und 13). Diese Untersuchung ist geschlossen; der mikroskopische Analogpfad ist für den Projektfortschritt keine Pflichtlücke.

Produktiv verbleiben zwei Displaypatches und das dauerhafte Audio-Always-on. Die im Repository vorhandenen I2C-Diagnosepatches sind Archiv-/Entwicklungsbestand, nicht automatisch aktiver Kernel (`hardware/novena.nix:31`).

## 21.–22. September: Eigenständigkeit und Reproduzierbarkeit

Vor Integration des neuen Bootloaders waren nach Projektbericht drei POR-Boots des externen NixOS erfolgreich. Tatsächlich benutzt wurde jedoch die interne microSD mit SPL/U-Boot 2015 (`docs/PROJECT-STATE.md:86`). Der Erfolg von externem Kernel, initrd und Rootfs beweist daher keinen P_EXT-Gesamtboot.

U-Boot-Basis wird `ece349ade2973e220f524ce59e59711cc919263f`, bezeichnet als v2026.07. U1 wählt USDHC2 im SPL; U2 priorisiert mmc1 in proper; U3 beseitigt persistentes MMC-Environment (`docs/DECISIONS.md:205`; Patches 0001–0003). Nummern sind phasenabhängig und dürfen nicht direkt als Controlleridentität gelesen werden.

Die Clean-Store-Abweichung wurde auf hostabhängige Bash-Pipegrößen zurückgeführt, lokal auf 4096 im Cross-Build festgelegt und anschließend für den damaligen G2X-Stand byteidentisch qualifiziert (`docs/TEST-LOG.md:1371`, `docs/TEST-LOG.md:1424`). Aussagegrenze: gleicher Host, getrennte Stores und damaliger Quellstand; keine allgemeine Reproduzierbarkeit aller zukünftigen Images auf beliebigen Hosts.

## 22.–23. September: P_EXT und Allocator-Eingrenzung

Erhaltene Logs zeigen aktuellen SPL, zunächst Ende nahe MMC-Suche, später Eintritt in eSDHC-Initialisierung mit `ret=-12`. Weitere Diagnose lokalisiert inkonsistentes `mem_malloc_brk` vor erster Allokation, P0 bereits Null und schließlich Q1 vor Heap-Clear Null (05, 13). Die historische Korridorbeschreibung MMC ist ein Beobachtungsort; die spätere Evidenz verlegt die offene Frage vor den Heap-Clear.

Fall 7C und Fall 8B stammen aus verschiedenen Instrumentierungen und Läufen. Konsistente Nullbeobachtungen sind ein INDIZ für ein ähnliches frühes Problem; sie sind keine kontrollierte Wiederholungsserie und bestimmen keinen DDR-Mechanismus.

## 28. September bis 2. Oktober: getrennte Provenienz und Kontrolle

Historische Medienanalyse dokumentiert einen Hybridzustand mit SATA-Disk-ID und MMC-fstab, dessen Entstehung ungeklärt bleibt (`docs/HISTORICAL-MEDIA.md:296`). Das verifizierte Raw-Master ist die künftige Analysegrundlage; dieser Block hat keine Images gemountet oder erneut qualifiziert.

Patch 0008 wurde nach Dokumentation gebaut, statisch qualifiziert, ins vollständige Image integriert, geschrieben und vollständig rückverifiziert, danach genau einmal unter POR/P_EXT beobachtet (`docs/TEST-LOG.md:2209`). Die erhaltene Primärausgabe ist das 830-Byte-Seriellog. Build-/Write-/Readback-Angaben sind eigenständige Dokumentationsbelege; sie werden nicht durch das Seriellog allein bestätigt.

Am 2. Oktober folgte die statische Rekonstruktion: ignorierte WL-/DQS-/Wait-Rückgaben, SRAM-Stack, BSS-Clear nach Kalibrierung und DQS-Tail-Branch (`docs/PROJECT-STATE.md:5740`; research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md). Originalberichte 9C-33 bis 9C-42 fehlen im Repository; kein Ablauf wird aus Blocknummern erfunden.

## Aktuell belegter Endpunkt

ROM → externe SD/USDHC2 → SPL ist dokumentiert. SPL → aktuelles proper → boot.scr → Kernel/initrd/DTB → Rootfs unter eigenständigem P_EXT bleibt unbelegt. Die offene Root Cause gehört zu diesem Bootstrang, nicht zur abgeschlossenen ES8328-Untersuchung. Der heutige Arbeitsbaum enthält bereits Patches 0004–0008, obwohl frühere historische Abschnitte sie noch als nicht eingebunden beschreiben; das ist Versionsentwicklung, kein Grund zum Rücksetzen.
