# Novena – Abschluss Dual-Build-Reproduzierbarkeitsvergleich

Kennung: NOVENA-DUAL-REPRO-2026-10-20

Datum: 2026-10-20

Status: Vergleich abgeschlossen; Ursachenklärung offen.

## 1. Untersuchungsgegenstand

Vergleich zweier vollständig erzeugter NixOS-SD-Images
für Kosagi Novena auf foobox und VM102.

Beide Builds waren erfolgreich. Beide erzeugten denselben
Nix-Store-Ausgabepfad für das SD-Image.

## 2. Image-Ergebnisse

Imagegröße: 2.588.422.144 Bytes.

SHA256 foobox:
5e4413cba9d284041fe3b1ca42e403e46d1cc53abdadc6c9346c62214a8d961c

SHA256 VM102:
67f7899000584867467c8740e39df46ef67d3320fd06ca69d0607b01c8af528c

Die vollständigen Images sind nicht byteidentisch.

Die ersten 8 MiB, die Partitionstabelle und die
128-MiB-FAT32-Bootpartition wurden byteidentisch verglichen.

Die ext4-Rootpartitionen sind nicht byteidentisch.

## 3. Vollständiger Root-Dateivergleich

Nachweis: NOVENA-DUAL-EXT4-MOUNT-COMPARE-48B.

foobox: 155.047 Dateisystemeinträge.
VM102: 155.045 Dateisystemeinträge.

Nur foobox: 2 Einträge.
Nur VM102: 0 Einträge.
Unterschiedliche gemeinsame Einträge: 2.

Die zwei ausschließlich auf foobox vorhandenen Einträge:

- Boost 1.89.0:
  lib/libboost_stacktrace_addr2line.so
- Boost 1.89.0:
  lib/libboost_stacktrace_addr2line.so.1.89.0

Unterschiedliche gemeinsame Dateien:

- /nix-path-registration
- Coreutils 9.11: bin/coreutils

Geprüft wurden Dateipfade, Dateitypen, reguläre Dateiinhalte
mittels SHA256, Symlink-Ziele, Modi und UID/GID.

Nicht Gegenstand des vollständigen Vergleichs waren
sämtliche ext4-Metadaten, ACLs und Extended Attributes.

## 4. Coreutils 9.11

Identischer Store-Pfad und identische Derivation.

foobox: 1.700.268 Bytes.
VM102: 1.700.016 Bytes.

SHA256 foobox:
bddd98853367b2502c81a3213a4fb37f9e50a096954fc90f47eaad475cdbc5f3

SHA256 VM102:
f0032c98b4e7b8d6f10a449f0000aae5abce2326119e361b0bb7013daadc42e8

Die ELF-Build-IDs unterscheiden sich.

## 5. Boost 1.89.0

Identischer Store-Pfad und identische Derivation.

Auf foobox existieren zusätzlich eine reguläre Bibliothek
mit 46.680 Bytes und ein darauf verweisender Symlink.

Auf VM102 fehlen beide Einträge.

## 6. Nix-Registrierung und Store

Die Unterschiede in /nix-path-registration entsprechen
den abweichenden NAR-Hashes und NAR-Größen von Coreutils
und Boost.

Block NOVENA-DUAL-REPRO-50 bestätigte:

- Identische Derivationspfade beider Store-Objekte.
- Byteidentische .drv-Dateien auf beiden Hosts.
- Unterschiedliche NAR-Hashes.
- Unterschiedliche Store-Inhalte.
- nix-store --verify-path: PASS auf beiden Hosts.
- Lokal berechnete NAR-Hashes stimmen jeweils mit
  den registrierten NAR-Hashes überein.

Die Unterschiede sind bereits in den lokalen Nix-Stores
vorhanden; sie entstehen nicht erst durch das Kopieren
der Store-Objekte in die SD-Images.

## 7. Build-Umgebungen

foobox:
- Nix 2.35.2
- CachyOS, Kernel 7.2.11
- extra-platforms = armv7l-linux

VM102:
- Nix 2.31.2
- Kernel 6.12.69
- andere extra-platforms-Konfiguration

Auf beiden Hosts:
- sandbox = true
- substituters = https://cache.nixos.org/
- builders-use-substitutes = false

Die tatsächliche historische Herkunft der beiden
abweichenden Store-Objekte ist nicht abschließend belegt.

## 8. Bewertung und Abgrenzung

Beide SD-Image-Builds: PASS.

Byteidentische Bootpartition: PASS.

Vollständige Bitreproduzierbarkeit des Images:
NICHT NACHGEWIESEN.

Inhaltsunterschiede: auf Coreutils und Boost eingegrenzt.
Die abweichende Registrierungsdatei bildet diese Unterschiede ab.

Die konkreten Ursachen für die unterschiedlichen
Store-Ausgaben trotz identischer Derivationen bleiben offen.

Insbesondere darf aus unterschiedlichen Nix-Versionen
allein keine Kausalität abgeleitet werden.

## 9. Abschlussentscheidung

Die lesende Vergleichsuntersuchung wird hier abgeschlossen.

Weitere Wiederholungen derselben Image-, Dateisystem-
oder Store-Vergleiche sind ohne neue Evidenz nicht vorgesehen.

Ein gesonderter Ursachenauftrag kann bei Bedarf
kontrollierte Paket-Rebuilds und Build-Umgebungsanalysen
umfassen. Dafür ist eine neue ausdrückliche Freigabe nötig.

Die eigentliche Novena-Boot- und Hardwarequalifikation
ist von dieser Reproduzierbarkeitsuntersuchung zu trennen.

## 10. Maßgebliche Originalprotokolle

GitHub-Repository: loomit77/project-block-logs

Verzeichnis: novena/2026/10/

- NOVENA-DUAL-EXT4-FILES-47A
- NOVENA-DUAL-EXT4-FILES-47B
- NOVENA-DUAL-EXT4-SYMLINK-47C
- NOVENA-DUAL-EXT4-MOUNT-COMPARE-48
- NOVENA-DUAL-EXT4-MOUNT-COMPARE-48B
- NOVENA-DUAL-REPRO-49
- NOVENA-DUAL-REPRO-50

Frühere Blöcke 45 bis 47 sind ebenfalls Teil der
Untersuchung. Historische FAIL-Ergebnisse bleiben FAIL.

Die Originalprotokolle und ihre SHA256-Sidecars bleiben
unverändert.
