# Novena: Personality-EEPROM und Ethernet-MAC

Stand: 2026-10-22

## Aktive Bootloader-Basis

Das Projekt verwendet U-Boot 2026.07 als aktive
Bootloader-Basis im NixOS-SD-Image-Builder.

Die historische Bootkette bleibt als Referenz
für Vergleiche und Provenienzuntersuchungen erhalten.

## Nachgewiesene EEPROM-Daten

Der Novena-Personality-EEPROM befindet sich auf
I2C-Bus 2 unter Adresse 0x56.

Ein lesender Zugriff mit zwei internen Adressbytes
ab Offset 0x0000 ergab:

- Signatur: Novena
- Version: 2
- Seriennummer: 632
- Ethernet-MAC: 00:1f:11:02:17:77
- Feature-Flags: 0x01f7

Der EEPROM wurde nicht verändert.

## Fehler im modernen U-Boot

Die historische Novena-Implementierung verwendete
zwei interne EEPROM-Adressbytes.

Der moderne U-Boot verwendet hingegen:

    i2c_get_chip_for_busnum(2, 0x56, 1, &eeprom);

Der dritte Parameter legt die interne Adresslänge
auf ein Byte fest.

Die moderne novena_defconfig enthält weiterhin
CONFIG_SYS_I2C_EEPROM_ADDR_LEN=2. Dieser Wert
korrigiert den expliziten DM-I2C-Aufruf nicht.

## Laufzeitbeobachtungen

Mehrere ältere NixOS-Protokolle zeigen die originale
Ethernet-MAC 00:1f:11:02:17:77.

Ein späterer U-Boot-2026.07-Boot meldete:

    Invalid I2C EEPROM signature.

Linux meldete anschließend:

    Invalid MAC address: 00:00:00:00:00:00
    Using random MAC address

Die falsche interne EEPROM-Adresslänge ist eine
stark belegte Ursache. Ein erfolgreicher Boot mit
korrigiertem U-Boot steht noch aus.

## Korrektur

Der Patch

    boot/u-boot/0015-novena-eeprom-two-byte-address.patch

ändert ausschließlich den dritten Parameter von
i2c_get_chip_for_busnum() von 1 auf 2.

Er wird in boot/u-boot/default.nix eingebunden.

Damit bleibt U-Boot 2026.07 unsere aktive
Bootloader-Basis.

## Grenzen dieses Arbeitsschritts

- Kein EEPROM-Schreibzugriff
- Keine fest eingetragene MAC-Adresse
- Kein Build
- Kein Hardwaretest
- Kein Commit oder Push
- Keine Änderung der historischen Referenz-Bootkette

## Belegprotokolle

- NOVENA-EEPROM-READONLY-04
- NOVENA-MAC-PROVENIENZ-01
- NOVENA-MAC-PROVENIENZ-02
- NOVENA-MAC-PROVENIENZ-03
- NOVENA-MAC-PROVENIENZ-04

Ablage: project-block-logs/novena/2026/10/
