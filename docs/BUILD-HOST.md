# Build Host

Stand: 2026-09-14

## Zweck

Dieses Dokument beschreibt den verifizierten Entwicklungs- und
Build-Rechner für das Projekt des universellen NixOS-SD-Images für das
Kosagi Novena.

Die hier dokumentierte Konfiguration dient als Referenz für
reproduzierbare x86_64-zu-ARMv7-Cross-Builds sowie für den seriellen
Zugriff auf die Novena-Hardware.

Projektbezogene Kernel-, Device-Tree-, Display- und I2C-Teststände werden
nicht hier dokumentiert, sondern insbesondere in:

- `docs/PROJECT-STATE.md`
- `docs/GOLDEN-BUILD.md`
- `docs/TEST-LOG.md`

## Rechner

Hostname:

```text
foobox
```

Mainboard:

```text
ASRock X300-ITX
```

CPU:

```text
AMD Ryzen 7 5700G with Radeon Graphics
```

CPU-Konfiguration:

```text
8 Kerne
16 Threads
AMD-V vorhanden
```

Firmware:

```text
BIOS P1.70
Firmware-Datum: 2025-11-21
```

## Betriebssystem

Distribution:

```text
CachyOS
```

Architektur:

```text
x86_64
```

Zum Stand dieses Dokuments verwendeter Kernel:

```text
7.2.4-3-cachyos
```

Shell:

```text
fish 4.9.3
```

Git:

```text
2.55.0
```

## Arbeitsspeicher

Physisch installiert:

```text
64 GiB DDR4
```

Bestückung:

```text
Channel A DIMM1: 32 GiB Kingston KF3200C16D4/32GX
Channel B DIMM1: 32 GiB Kingston KF3200C16D4/32GX
```

Damit ist eine symmetrische Bestückung beider Speicherkanäle vorhanden.

Aktuell konfigurierte Speichergeschwindigkeit:

```text
2400 MT/s
```

Die Module gehören nominell zur DDR4-3200-Klasse. Eine Änderung der
Speichertaktung oder Aktivierung eines schnelleren Speicherprofils wurde
bewusst nicht im Rahmen des Novena-Projekts vorgenommen.

Linux meldet:

```text
MemTotal: 61582688 kB
```

Das entspricht ungefähr:

```text
58.7 GiB
```

nutzbarem Hauptspeicher.

Die Differenz zu den physisch installierten 64 GiB ist plausibel erklärt.

Die integrierte Radeon-Grafik reserviert:

```text
4096 MiB VRAM
```

Zusätzlich meldet der Kernel ungefähr:

```text
1373404 KiB reserved
```

für weitere Firmware-, Kernel- und Gerätebereiche.

Es gibt daher keinen Hinweis auf nicht erkannten oder fehlerhaft
adressierten Arbeitsspeicher.

## ZRAM

Aktives Swap-Gerät:

```text
/dev/zram0
```

Kompression:

```text
zstd
```

Größe:

```text
ca. 58.7 GiB
```

Priorität:

```text
100
```

Damit steht zusätzlich ein großer komprimierter Swap-Bereich im RAM zur
Verfügung.

## Stabilitätsbewertung

Ein separater synthetischer RAM- oder CPU-Stresstest mit Werkzeugen wie
`stress-ng` oder `memtester` wurde zum Stand dieses Dokuments nicht
durchgeführt.

Beide Programme waren auf der foobox nicht installiert.

Auf eine zusätzliche Installation nur für diesen Test wurde bewusst
verzichtet, da die Maschine bereits wiederholt reale und teilweise
umfangreiche Nix-, Kernel- und ARMv7-Cross-Builds ohne beobachtete
Speicher-, Compiler- oder Stabilitätsfehler durchgeführt hat.

Diese realen Builds gelten für den aktuellen Projektstand als
ausreichender praktischer Funktionstest.

Falls später nicht reproduzierbare Compilerfehler, Abstürze,
Speicherfehler oder andere Stabilitätsprobleme auftreten, soll ein
gezielter synthetischer CPU- und RAM-Test nachgeholt werden.

## Massenspeicher

Systemlaufwerk:

```text
Samsung SSD 970 EVO Plus 1TB
```

Gerät:

```text
/dev/nvme0n1
```

Partitionierung:

```text
nvme0n1p1   4 GiB       vfat
nvme0n1p2   ca. 927 GiB LUKS
```

Das verschlüsselte Hauptsystem verwendet Btrfs.

Zum überprüften Zeitpunkt standen ungefähr:

```text
302 GiB
```

noch nicht zugewiesener Btrfs-Speicherplatz zur Verfügung.

Hohe Belegung einzelner bereits allokierter Data- oder Metadata-Chunks
ist daher nicht als akuter Platzmangel zu interpretieren.

Ein Btrfs-Balance-Lauf ist allein aufgrund dieser Chunk-Auslastung nicht
erforderlich.

## NVMe-Gesundheitszustand

Der Zustand wurde mit `smartctl` überprüft.

Laufwerk:

```text
Samsung SSD 970 EVO Plus 1TB
```

Firmware:

```text
2B2QEXM7
```

SMART-Gesamtstatus:

```text
PASSED
```

Wesentliche Werte:

```text
Critical Warning:                 0x00
Temperature:                      46 C
Temperature Sensor 2:             49 C
Available Spare:                  100 %
Percentage Used:                  0 %
Data Units Read:                  ca. 6.39 TB
Data Units Written:               ca. 8.76 TB
Power Cycles:                     1284
Power On Hours:                   484
Unsafe Shutdowns:                 71
Media and Data Integrity Errors:  0
Error Information Log Entries:    1219
```

Die vorhandenen Error-Log-Einträge wurden als:

```text
Invalid Field in Command
```

gemeldet.

Es handelt sich damit nicht um dokumentierte NAND-, Medien- oder
Datenintegritätsfehler.

Der NVMe-Zustand gilt für den aktuellen Projektstand als gesund.

## Netzwerk

Ethernet-Controller:

```text
Realtek RTL8111/8168
```

Interface:

```text
enp2s0
```

Bei der Verifikation verwendete Adresse:

```text
192.168.1.10/24
```

Die konkrete IPv4-Adresse gehört nicht zur reproduzierbaren
Build-Konfiguration und kann sich unabhängig vom Projekt ändern.

## Serielle Novena-Konsole

Für den seriellen Zugriff auf die Novena ist ein FTDI-Adapter vorhanden.

USB-Gerät:

```text
FTDI FT232R
```

USB-ID:

```text
0403:6001
```

Kernelmodul:

```text
ftdi_sio
```

Das aktuelle TTY-Gerät ist:

```text
/dev/ttyUSB0
```

Für Projektbefehle soll nach Möglichkeit nicht der volatile
`/dev/ttyUSB0`-Name verwendet werden, sondern der persistente Pfad:

```text
/dev/serial/by-id/usb-FTDI_FT232R_USB_UART_BH00304A-if00-port0
```

Dieser verweist aktuell auf:

```text
/dev/ttyUSB0
```

Der persistente `by-id`-Pfad ist die bevorzugte Referenz für zukünftige
serielle Test- und Logging-Befehle.

## Nix

Installierte Version:

```text
nix 2.35.2
```

Die systemweite Nix-Konfiguration befindet sich unter:

```text
/etc/nix/nix.conf
```

Der verifizierte vollständige Inhalt lautet:

```text
build-users-group = nixbld
experimental-features = fetch-tree flakes nix-command
extra-platforms = armv7l-linux
extra-system-features = gccarch-armv7-a
```

Damit sind insbesondere folgende Nix-Funktionen dauerhaft aktiviert:

```text
fetch-tree
flakes
nix-command
```

Frühere Projektbefehle mussten regelmäßig beispielsweise mit:

```text
--extra-experimental-features 'nix-command flakes'
```

ausgeführt werden.

Dieser Zusatz ist mit der aktuellen systemweiten Konfiguration nicht
mehr erforderlich.

## Nix-Plattformen

Die native Plattform wurde verifiziert als:

```text
x86_64-linux
```

Die für das Novena verwendete Cross-Plattform wurde verifiziert als:

```text
armv7l-linux
```

Die ARMv7-Hard-Float-Cross-Umgebung:

```text
pkgsCross.armv7l-hf-multiplatform
```

ist auf der foobox funktionsfähig.

Ein ARMv7-Cross-GCC wurde erfolgreich durch Nix gebaut.

Verifizierte GCC-Version:

```text
15.3.0
```

Damit ist der grundlegende x86_64-zu-ARMv7-Cross-Build-Pfad bestätigt.

## Nix-Build-Einstellungen

Zum Stand dieses Dokuments gelten effektiv:

```text
cores = 0
max-jobs = 1
```

Zusätzlich meldet Nix unter anderem folgende System-Features:

```text
benchmark
big-parallel
gccarch-armv7-a
kvm
nixos-test
uid-range
```

`cores = 0` erlaubt einer Derivation die automatische Nutzung der
verfügbaren CPU-Kerne.

`max-jobs = 1` beschränkt die Anzahl gleichzeitig laufender
Derivationen.

Diese Werte wurden bewusst nicht geändert.

Während früherer Tests wurden auf der Kommandozeile unterschiedliche
Kombinationen von `--max-jobs` und `--cores` ausprobiert. Daraus ergibt
sich bisher keine ausreichend begründete optimale globale Einstellung.

Insbesondere bei einem einzelnen großen Kernel-Build kann
`max-jobs = 1` zusammen mit `cores = 0` sinnvoll sein, da die einzelne
Derivation die vorhandenen CPU-Ressourcen nutzen kann, ohne gleichzeitig
mit weiteren großen Derivationen um RAM und CPU-Zeit zu konkurrieren.

Eine Änderung dieser Parameter soll erst nach reproduzierbaren
Buildzeit-Messungen erfolgen.

## Flake-Verifikation

Das Repository:

```text
~/nixos-novena-image
```

wurde mit der dauerhaft aktivierten Flake-Unterstützung erfolgreich
ausgewertet.

Der überprüfte Git-Stand war:

```text
35357fd405259a24f18c86a6fd0dc673c98a12f5
```

`nix flake show` erkannte:

```text
nixosConfigurations
└── novena
```

als gültige NixOS-Konfiguration.

## Verifizierte Basisbefehle

Native Host-Plattform:

```fish
nix eval --impure --raw --expr 'builtins.currentSystem'
```

Erwartetes Ergebnis:

```text
x86_64-linux
```

ARMv7-Cross-Plattform:

```fish
nix eval --impure --raw --expr '
let
  pkgs = import <nixpkgs> {};
in
  pkgs.pkgsCross.armv7l-hf-multiplatform.stdenv.hostPlatform.system
'
```

Erwartetes Ergebnis:

```text
armv7l-linux
```

Flake-Struktur:

```fish
cd ~/nixos-novena-image
nix flake show
```

Diese Befehle benötigen mit der aktuellen Nix-Konfiguration keinen
zusätzlichen Parameter für `nix-command` oder `flakes`.

`--impure` ist davon unabhängig und bleibt bei Auswertungen notwendig,
die ausdrücklich eine impure Evaluation verwenden.

## Repository- und Backup-Zustand

Temporäre Arbeits- und Sicherungsdateien aus den vorherigen
Diagnoseschritten wurden aus dem unversionierten Repository-Arbeitsbaum
entfernt, nachdem sie separat gesichert und überprüft wurden.

Sicherungsverzeichnis:

```text
~/novena-backups/foobox-repo-backups-2026-09-13
```

Die dort gesicherten Dateien wurden:

1. kopiert,
2. mit SHA-256 erfasst,
3. mit `sha256sum -c` erfolgreich verifiziert,
4. byteweise mit den ursprünglichen Dateien verglichen.

Alle geprüften Kopien waren identisch.

Die bereits von Git verwaltete Datei:

```text
kernel/it6251.c.before-drm-lifecycle
```

wurde nicht aus der Versionsgeschichte entfernt und nach der
Bereinigung korrekt aus Git wiederhergestellt.

Der anschließend überprüfte Repository-Zustand war:

```text
## main...origin/main
```

Der Working Tree war damit sauber.

## Pflege dieses Dokuments

Dieses Dokument soll aktualisiert werden, wenn sich die für den
Novena-Build relevante Infrastruktur der foobox wesentlich ändert.

Dazu gehören insbesondere:

- CPU oder RAM
- Build-Betriebssystem
- Nix-Version
- `/etc/nix/nix.conf`
- Cross-Compilation-Plattform
- relevante Build-Parallelisierung
- serieller Adapter oder dessen persistenter Gerätepfad
- für Builds relevanter Massenspeicher
- dauerhaft eingesetzte zusätzliche Build-Werkzeuge

Kurzlebige Kernel-, Device-Tree- oder I2C-Diagnosezustände gehören
dagegen nicht in dieses Dokument.
