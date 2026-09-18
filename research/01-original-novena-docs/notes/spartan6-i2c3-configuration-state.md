# Spartan-6-Zustand an I2C3 während Power-on und Konfiguration

Stand: 2026-09-18

## Zweck

Diese Notiz dokumentiert die für die Novena-I2C3-Root-Cause-Analyse
relevanten Aussagen der offiziellen AMD/Xilinx-Spartan-6-Dokumentation.

Untersucht wird insbesondere, welchen elektrischen Zustand die mit I2C3
verbundenen FPGA-Pins während Power-on, Initialisierung und Konfiguration
haben und welche Rolle `HSWAPEN` dabei spielt.

Die Notiz dient der Eingrenzung einer möglichen FPGA-Beteiligung am
untersuchten I2C3-Kaltstartfehler. Sie soll ausdrücklich nicht über die
Primärquellen hinaus einen elektrischen Fehlermechanismus ableiten.

## Archivierte Primärquellen

Folgende offizielle AMD/Xilinx-Dokumente wurden lokal archiviert:

- `sources/amd-xilinx-ug380-spartan6-configuration.pdf`
  - Spartan-6 FPGA Configuration User Guide
  - UG380
  - Version 2.11
  - Datum: 2019-03-22
  - SHA-256:
    `4afb6472018a9b3fa3bc1be906a0d15f86a17567d1dd7e930debfce3de362c9b`

- `sources/amd-xilinx-ug381-spartan6-selectio.pdf`
  - Spartan-6 FPGA SelectIO Resources User Guide
  - UG381
  - Version 1.7
  - Datum: 2015-10-21
  - SHA-256:
    `4a0fc9078af54edc1104452fe5fa2bfaa82118b5501ba9fd000c1e1e2310821a`

Die zugehörigen HTTP-Header wurden ebenfalls archiviert:

- `sources/amd-xilinx-ug380-spartan6-configuration.http-headers.txt`
  - SHA-256:
    `a7c2ee078a5e6428d8b12f6880b068e5fb4e5ae21ea79bb49ee1e3f0d42995d9`

- `sources/amd-xilinx-ug381-spartan6-selectio.http-headers.txt`
  - SHA-256:
    `7abac9a97d15f938c9b205410057da82d4df6d53c3efb2504d264122819bef7a`

Alle vier Dateien sind zusätzlich in
`research/01-original-novena-docs/SHA256SUMS` erfasst.

## Novena-Schaltungstopologie

Die bereits ausgewertete PVT2-A-Schaltung zeigt:

- `I2C3_SCL` ist direkt mit FPGA-Pin P4 verbunden.
- P4 besitzt die Spartan-6-Funktion `IO_L2P_3`.
- `I2C3_SDA` ist direkt mit FPGA-Pin P3 verbunden.
- P3 besitzt die Spartan-6-Funktion `IO_L2N_3`.
- Zwischen diesen FPGA-Pins und dem globalen I2C3-Bus befindet sich im
  Schaltplan kein Serienwiderstand und kein Pegelwandler.

P3 und P4 sind damit normale Spartan-6-User-I/Os auf dem I2C3-Bus.

Die ausführliche Bus-Topologie ist separat dokumentiert in:

`notes/pvt2-a-i2c3-topology.md`

## HSWAPEN-Beschaltung auf Novena

Der PVT2-A-Schaltplan zeigt für `FPGA_HSWAPEN`:

- R13F = 4,7 kOhm von `P3.3V_DELAYED` nach `FPGA_HSWAPEN`.
- R12F = 4,7 kOhm von `FPGA_HSWAPEN` nach GND.
- R12F ist als `DNP` gekennzeichnet.
- `FPGA_HSWAPEN` führt zum Spartan-6-Pin D4 mit der Funktion
  `IO_L1P_HSWAPEN_0`.

Für die dokumentierte Bestückung ist `FPGA_HSWAPEN` damit über R13F nach
`P3.3V_DELAYED` hochgezogen.

Eine frühere Textauswertung hatte `NLEIM0DA14` beziehungsweise `EIM_DA14`
in räumlicher Nähe zu diesem Bereich ausgegeben. Die visuelle Kontrolle
des Schaltplans zeigt jedoch, dass dies ein Extraktionsartefakt war:
`EIM_DA14` liegt auf einem anderen FPGA-Pin und ist nicht mit
`FPGA_HSWAPEN` gleichzusetzen.

## Herstellerangabe zu HSWAPEN

UG381 beschreibt `HSWAPEN` als Steuerung der internen Pull-up-Widerstände
der User-I/O-Pins vom Einschalten bis zum Abschluss der Konfiguration.

Die dokumentierte Polarität ist:

- `HSWAPEN = Low`: interne Pull-ups der User-I/Os aktiviert.
- `HSWAPEN = High`: interne Pull-ups der User-I/Os deaktiviert.

UG380 bestätigt diese Zuordnung in der Tabelle zu den
Spartan-6-Konfigurations-Pin-Terminierungen:

- `HSWAPEN = 0`: Pull-up aktiviert.
- `HSWAPEN = 1`: keine entsprechende Terminierung der User-I/Os.

Da Novena `FPGA_HSWAPEN` über R13F nach `P3.3V_DELAYED` zieht und der
alternative Pulldown R12F nicht bestückt ist, ist für die dokumentierte
PVT2-A-Bestückung während dieser Phase `HSWAPEN = High` vorgesehen.

Damit sind die HSWAPEN-gesteuerten internen Pull-ups der User-I/Os
während Power-on und Konfiguration deaktiviert.

## Zustand der User-I/O-Ausgangstreiber

UG381 beschreibt den Zustand der I/O-Ausgangstreiber während Power-on
und Konfiguration.

Belegt ist:

1. Nachdem die für den internen POR relevanten Versorgungsspannungen ihre
   Mindestwerte erreicht haben, befinden sich alle Ausgangstreiber in
   einem hochohmigen Zustand.

2. Während der FPGA-Konfiguration bleiben die I/O-Treiber hochohmig.

3. Ob während dieser Zeit interne Pull-ups vorhanden sind, wird durch
   `HSWAPEN` bestimmt.

UG380 beschreibt ergänzend die Initialisierungsphase nach Power-on oder
einer erneuten Initialisierung. Während das Konfigurationsspeicher-Array
gelöscht beziehungsweise initialisiert wird, werden die I/Os mit Ausnahme
der dedizierten Konfigurations- und JTAG-Pins in den High-Z-Zustand
versetzt.

Für die Novena-Pins P3/P4 folgt daraus:

- P3/P4 sind User-I/Os.
- Ihre Ausgangstreiber sind während der dokumentierten
  Initialisierungs-/Konfigurationsphase High-Z.
- Wegen der Novena-HSWAPEN-Beschaltung sind die HSWAPEN-gesteuerten
  internen Pull-ups dabei deaktiviert.

## Ende des High-Z-Konfigurationszustands

Der High-Z-Zustand darf nicht auf den gesamten Bootvorgang übertragen
werden.

UG381 beschreibt, dass beim Ende der Konfiguration das globale
3-State-Signal GTS während der Startup-Sequenz freigegeben wird. Danach
gehen die User-I/Os in ihren durch das geladene Design bestimmten Zustand
über.

UG380 beschreibt denselben Übergang:

- Die Startup-Sequenz besteht aus mehreren Phasen.
- Das Negieren von GTS aktiviert die I/Os.
- Die konkrete Startup-Phase kann durch BitGen-Optionen bestimmt werden.
- `GTS_CYCLE` legt fest, in welcher Startup-Phase die I/Os vom
  3-State-Zustand in den Zustand des User-Designs wechseln.

Damit ist durch UG380/UG381 nicht belegt, dass P3/P4 nach Eintritt in den
User-Mode weiterhin High-Z bleiben.

Ihr Post-Configuration-Zustand hängt vom tatsächlich geladenen
FPGA-Design beziehungsweise Bitstream ab.

## Historischer Novena-U-Boot-Befund

Die bereits archivierten historischen Novena-U-Boot-Quellen wurden
ergänzend nach FPGA-Konfiguration und FPGA/I2C3-Interaktionen durchsucht.

Für Novena wurde gefunden:

- `NOVENA_FPGA_RESET_N_GPIO` ist als GPIO5_IO07 definiert.
- `FPGA_RESET_N` wird über `DISP0_DAT13__GPIO5_IO07` gemultiplext.
- Der Novena-SPL setzt dieses Signal mit
  `gpio_direction_output(NOVENA_FPGA_RESET_N_GPIO, 0)` auf Low.

In den untersuchten Novena-spezifischen U-Boot-Stellen wurde keine
Bitstream-Ladeoperation für den Spartan-6 gefunden.

Insbesondere darf `FPGA_RESET_N` nicht ohne zusätzlichen Beleg mit dem
dedizierten Spartan-6-Konfigurationssignal `PROGRAM_B` gleichgesetzt
werden.

Aus `FPGA_RESET_N = 0` kann deshalb anhand der bisher untersuchten Quellen
nicht abgeleitet werden, dass sich der FPGA dadurch im
Konfigurationszustand befindet oder dass P3/P4 dadurch High-Z bleiben.

## Bedeutung für die I2C3-Root-Cause-Analyse

Die Herstellerdokumentation schwächt eine einfache FPGA-Hypothese, nach
der der Spartan-6 bereits während seines normalen Power-on- oder
Konfigurationszustands I2C3 über P3/P4 aktiv treiben würde.

Für die dokumentierte Novena-Beschaltung ist stattdessen belegt:

- P3/P4 sind User-I/Os.
- Die User-I/O-Ausgangstreiber sind während Initialisierung und
  Konfiguration High-Z.
- `HSWAPEN` ist für die dokumentierte Bestückung High.
- Die HSWAPEN-gesteuerten internen User-I/O-Pull-ups sind damit
  deaktiviert.

Das ist keine vollständige Ausschließung des FPGA als mögliche
I2C3-Einflussquelle.

Offen bleibt insbesondere:

- ob und wann im konkreten Novena-Bootpfad ein FPGA-Bitstream geladen
  beziehungsweise bereits vorhanden ist;
- welchen Zustand das konkrete User-Design P3/P4 nach Freigabe von GTS
  zuweist;
- ob ein Post-Configuration-Zustand des FPGA den I2C3-Bus beeinflussen
  könnte.

Für einen solchen Post-Configuration-Einfluss wurde in den bislang
untersuchten Quellen jedoch kein konkreter Novena-spezifischer Beleg
gefunden.

## Ergebnis

### Belegt

Der normale Spartan-6-Power-on-/Initialisierungs-/Konfigurationszustand
setzt die User-I/O-Ausgangstreiber auf High-Z.

Die dokumentierte Novena-HSWAPEN-Beschaltung deaktiviert gleichzeitig die
HSWAPEN-gesteuerten internen Pull-ups der User-I/Os.

Damit liefern die untersuchten Herstellerquellen keine Stütze für eine
Hypothese, nach der P3/P4 während dieser Phase durch normale
FPGA-Ausgangstreiber aktiv auf I2C3 treiben.

### Offen

Der elektrische Zustand von P3/P4 nach der GTS-Freigabe und dem Eintritt
in den User-Mode ist ohne Kenntnis des tatsächlich geladenen
FPGA-Designs nicht bestimmt.

Ein Post-Configuration-Einfluss des FPGA bleibt daher formal offen.

### Nicht bewiesen

Aus dieser Analyse folgt nicht, dass der FPGA unter allen Umständen als
I2C3-Einflussquelle ausgeschlossen ist.

Ebenso folgt daraus keine Aussage über den exakten elektrischen
Mechanismus des beobachteten I2C3-Kaltstartfehlers.

Die Analyse grenzt lediglich den normalen
Power-on-/Initialisierungs-/Konfigurationszustand des Spartan-6 als
Erklärung deutlich ein.
