# NixOS Wiki: NixOS on ARM/Kosagi Novena – Revision 21513

## Status

Historische Quellenuntersuchung für das Projekt des reproduzierbaren
NixOS-SD-Images für die Novena.

Diese Quelle wird als historische Begleitdokumentation archiviert. Sie
belegt für sich allein nicht die Ursache des Cold-POR-I2C3/IT6251-
START-/Arbitration-Loss-Fehlers unter Linux 6.18.49.

Aus dieser Quelle allein wird weder ein Kernel-Workaround noch eine
Patch-Entscheidung abgeleitet.

## Archivierte Quelle

Lokal gespeichertes HTML-Original:

    research/02-historical-software/sources/nixos-wiki-kosagi-novena-revision-21513.html

SHA-256:

    ab3855d4dbc0190a2d43cf1c7ec5855ba70db5f5da8eab76b1aeecf39a5ac2b6

Die im Projekt archivierte Kopie wurde als byteidentisch mit der ursprünglich
lokal gespeicherten Quelldatei verifiziert. Der maschinenspezifische absolute
Pfad dieser ursprünglichen Arbeitskopie wird nicht als Bestandteil der
dauerhaften Forschungsdokumentation geführt.

## Provenienz

Das gespeicherte HTML identifiziert die MediaWiki-Seite als:

    NixOS_on_ARM/Kosagi_Novena

Titel:

    NixOS on ARM/Kosagi Novena

MediaWiki-Revision:

    21513

Artikel-ID:

    404

Inhaltssprache der Seite:

    en

Der Seiten-Footer gibt als letzte Bearbeitung an:

    2025-05-18 05:36

Die gespeicherte Seite enthält außerdem einen Permanent-Link mit:

    oldid=21513

Die im HTML enthaltenen MediaWiki-Parser- und Cache-Zeitstempel sind
Rendering-/Cache-Metadaten. Sie dürfen nicht als Veröffentlichungsdatum
oder Datum der letzten Bearbeitung des Artikels interpretiert werden.

## Abgeleitete Belegdateien

Provenienz-Extrakt:

    research/02-historical-software/extracts/nixos-wiki-kosagi-novena-revision-21513-provenance.txt

SHA-256:

    ba3e5f929ff2f78b9df36cacc2dc47f4c4c0412db12d695f8d3932cf272cade1

Schlagwortorientierter relevanter Extrakt:

    research/02-historical-software/extracts/nixos-wiki-kosagi-novena-revision-21513-relevant.txt

SHA-256:

    7a39944b1b6b1d2ce03a659105c4bfd0db6fd281b0f1f56fb1ebaaa29d0214da

Diese Extrakte dienen als abgeleitete Such- und Analysehilfen.
Maßgebliche archivierte Quelle bleibt das unveränderte HTML.

## Verweise auf NixOS-/Novena-Repositories

Die Seite verweist ausdrücklich auf:

    https://github.com/novena-next/nixos-novena

Dieses Repository wird als Möglichkeit beschrieben, einen
Novena-spezifischen Kernel und zusätzliche Werkzeuge zu bauen, darunter:

    novena-eeprom
    novena-usb-hub

Die Seite verweist außerdem ausdrücklich auf:

    https://github.com/novena-next/docs

für allgemeine Dokumentation und erwähnt, dass die Anleitung möglicherweise
zukünftig dorthin verschoben werden soll.

Diese Repository-Verweise sind wichtige Spuren für die Untersuchung der
historischen Software. Die jeweiligen Git-Repositories und ihre Historie
müssen jedoch eigenständig untersucht werden. Die Beschreibung im Wiki darf
nicht als alleiniger Beleg für deren tatsächlichen Inhalt verwendet werden.

## Generisches ARM-Image

Die Seite weist auf die Verwendung des generischen armv7l-Images hin:

    sd-image-armv7l-linux.img

Dies ist eine historische Installationsanweisung. Daraus folgt nicht, dass
ein beliebiges aktuelles generisches armv7l-NixOS-Image ohne weitere
Anpassungen auf einer Novena bootfähig ist.

## U-Boot-Anleitung

Für U-Boot nennt die Seite das Novena-Konfigurationsziel:

    make novena_defconfig

Anschließend wird SPL mit einem Offset von 1 KiB auf das Zielmedium
geschrieben:

    dd if=SPL of=/dev/sdc seek=1 bs=1k

Weiterhin soll:

    u-boot.bin

unter:

    /boot

des eingehängten SD-Images abgelegt werden.

Dies beschreibt ein historisches Novena-Bootmodell. Zum jetzigen Zeitpunkt
darf nicht angenommen werden, dass dieses Modell identisch ist mit:

1. dem bereits untersuchten älteren xobs/u-boot-novena-Stand rc5 bis rc12;
2. dem U-Boot-2020.07-Binary aus dem originalen funktionierenden
   Novena-NixOS-Image;
3. dem Boot-Aufbau unseres aktuellen reproduzierbaren Testimages.

Diese Zusammenhänge müssen anhand der jeweiligen Quellen und ihrer
Provenienz separat untersucht werden.

## SATA-Boot

Für eine Installation auf Festplatte beschreibt die Seite, dass weiterhin
eine SD-Karte zum Start von SATA benötigt wird und dass extlinux anstelle
von GRUB verwendet werden soll.

Um SATA als Standard-Bootziel zu aktivieren, soll mit:

    novena-eeprom

das Feature:

    sataroot

aktiviert werden.

Anschließend nennt die Seite folgende U-Boot-Befehle:

    printenv
    setenv boot_targets sata0
    saveenv
    reset

Die Seite enthält in diesem Abschnitt selbst noch einen TODO-Hinweis zum
Disk-ID-Thema. Dieser Abschnitt ist deshalb als historische Anleitung und
nicht als vollständig reproduzierbare Installationsanweisung zu behandeln.

## Bedeutung für die aktuelle Cold-POR-I2C3-Untersuchung

Der gespeicherte Wiki-Artikel enthält keine direkte technische Erklärung für
den aktuellen Fehler unter Linux 6.18.49:

    Cold FAIL:
        A=81/80
        M0=93/80
        IAL bereits gesetzt
        MSTA bereits gelöscht

Der Artikel dokumentiert insbesondere nicht direkt:

- die IT6251-Initialisierungsreihenfolge;
- den elektrischen Zustand von I2C3;
- die Beteiligung des FPGA an I2C3;
- das I2C-Verhalten des ES8328;
- eine I2C-Bus-Recovery;
- das Reset-Timing des IT6251;
- die LVDS-/eDP-Power-Sequenz.

Die Wiki-Seite beweist oder widerlegt deshalb keine unserer derzeitigen
Root-Cause-Hypothesen.

Ihr wesentlicher Wert für die aktuelle Untersuchung liegt in der Provenienz
und der Identifikation weiterer historischer Primärquellen. Insbesondere
verweist sie ausdrücklich auf die historischen novena-next-Repositories.

## Nächste historische Quellen

Als Nächstes sollen unabhängig und unter Einbeziehung ihrer Git-Historie
untersucht werden:

    novena-next/nixos-novena
    novena-next/docs

Danach soll die gesamte Organisation novena-next auf weitere relevante
Repositories untersucht werden, insbesondere auf historische Kernel- und
Novena-spezifische Supportquellen.

Besonders zu untersuchen sind:

- Kernelquellen und Kernel-Patches;
- IT6251-Code und Register-Dumps;
- Device-Tree-Historie;
- U-Boot-Annahmen;
- novena-eeprom und sataroot;
- FPGA-Initialisierung;
- I2C3-Initialisierung und Bus-Recovery;
- Display-Power- und Reset-Sequenzen.

## Aktuelles Fazit

Revision 21513 ist eine nützliche historische Dokumentations- und
Quellenfindungsquelle.

Sie belegt, dass die Novena-NixOS-Dokumentation Novena-spezifischen
Kernel-/Tooling-Support vorsah und ausdrücklich auf das Projekt
novena-next verweist.

Sie dokumentiert außerdem ein U-Boot-/SPL-/extlinux-/SATA-Bootmodell.

Sie belegt nicht den Mechanismus hinter dem aktuellen Cold-POR-
I2C3/IT6251-Arbitration-Loss-Fehler unter Linux 6.18.49.

Aus dieser Quelle ergibt sich keine Rechtfertigung für einen Patch 0007.
