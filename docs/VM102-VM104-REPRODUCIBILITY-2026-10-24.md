# Novena – VM102/VM104-Reproduzierbarkeitscheckpoint

Datum: 2026-10-24 (Europe/Berlin)

## Gegenstand

Vollständiges NixOS-SD-Image für Kosagi Novena,
nachgewiesen auf zwei getrennten NixOS-Build-VMs:

- VM102: nixos-base
- VM104: novena-clean-builder

Beide Buildprotokolle melden PASS.

## Bestätigtes Referenzimage

Store-Ausgabe auf beiden VMs:

`/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img`

Imagegröße: 3240607744 Bytes

SHA256:

`40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a`

## Identische Derivation

Derivationspfad:

`/nix/store/jnnw8x82dlkj79vcybcsz4rxy1nc7ljf-nixos-novena-sd-image.img.drv`

SHA256 der Derivationsdatei:

`5142b14b4da32984955671b999ef231488ae9e42beba8e3f218d476c69ff9629`

## Ergebnis

Die auf VM102 und VM104 vorliegenden vollständigen
SD-Image-Dateien sind byteidentisch.

Die Buildprotokolle dokumentieren erfolgreiche,
zeitlich getrennte Buildabläufe.

Die vollständige Unabhängigkeit aller Abhängigkeits-Builds
ist nicht gesondert bewiesen.

Der Nachweis gilt für die dokumentierte Derivation und
die untersuchten NixOS-Buildumgebungen.

Er beweist keine allgemeine Byteidentität auf beliebigen
Linux-Buildhosts.

## Weitere Bestandsaufnahme

NOVENA-VM-IMAGE-INVENTORY-66 fand:

- Vier vorhandene SD-Images auf VM102.
- Ein vorhandenes SD-Image auf VM104.
- Ein identisches Imagepaar.
- Keine unterschiedlichen Image-Hashes bei einer
  auf beiden VMs vorhandenen identischen Derivation.

Drei weitere VM102-Images besitzen eigene Derivationen
und kein vorhandenes Vergleichsartefakt auf VM104.

## Abgrenzung foobox

foobox verwendet CachyOS und besitzt eine aktive
ARM-QEMU-Registrierung über binfmt_misc.

Für einzelne ARMv7-Pakete wurden unterschiedliche
Konfigurationstestergebnisse gegenüber VM102/VM104
festgestellt.

Die foobox-Abweichung bleibt ein separater
Reproduzierbarkeits- und Portabilitätsfall.

Es wurde hierfür kein neuer Paketpatch beschlossen.

## Primärnachweise

GitHub-Protokollrepository:

`loomit77/project-block-logs`

- NOVENA-IMAGE-COHORT-54, 2026-10-24:
  vollständige Image-Hashes auf beiden VMs.
- NOVENA-IMAGE-THREEWAY-56, 2026-10-24:
  VM102_VM104_BYTE_IDENTICAL=YES.
- NOVENA-VM-IMAGE-INVENTORY-66, 2026-10-24:
  vollständige Store-Inventur und Derivationsvergleich.

Zu jedem Prüfblock existieren TXT-Protokoll
und SHA256-Sidecar.

## Schlussfolgerung

Der byteidentische vollständige VM102/VM104-Image-Build
ist ein bestätigter Referenzcheckpoint.

Für diesen Nachweis sind keine weiteren Builds erforderlich.

Eine Veröffentlichung der Buildanleitung sollte den
festgelegten Quellstand, flake.lock und die
Buildumgebung der Referenz-VMs dokumentieren.
