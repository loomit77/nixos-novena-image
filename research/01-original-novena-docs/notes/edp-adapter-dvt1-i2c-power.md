# Novena eDP-Adapter DVT1 – I2C-, Reset- und Versorgungstopologie

## Zweck

Diese Notiz dokumentiert die Befunde aus dem originalen Novena-Schaltplan
„LVDS to eDP adapter DVT1“.

Ziel dieser Untersuchung ist es, die für den aktuellen I2C3-Kaltstartfehler
relevante Hardwaretopologie und die möglichen Hardwarezustände zu
rekonstruieren, ohne Kernel, Device Tree, Bootloader, Test-SD oder Hardware
zu verändern.

Dieses Dokument gehört zu Forschungsblock:

1. Originale Novena-Dokumentation und Schaltpläne

Die hier dokumentierten Befunde beweisen noch nicht die Ursache des aktuellen
I2C-Arbitration-Loss-Fehlers und rechtfertigen für sich allein keinen
Kernel-Patch 0007.

---

## Quellenherkunft

Offizielle Kosagi-Seite:

    Novena PVT Design Source

Die archivierte Seite enthält einen eigenen Abschnitt für das eDP-Board und
verlinkt dort den DVT1-Schaltplan sowie die zugehörigen Designquellen.

Archivierte Design-Source-Seite:

    research/01-original-novena-docs/sources/kosagi-novena-pvt-design-source.html

SHA-256:

    b138095a4b0d9d1ad0086a97de3d886b01cbd85f83f37d3306adacc257fafbca

Archivierte HTTP-Header:

    research/01-original-novena-docs/sources/kosagi-novena-pvt-design-source.http-headers.txt

SHA-256:

    495c31bc460f757963531af19455c29468faff176c38c04b2ee060c3f5576d9b

Offizieller eDP-DVT1-Schaltplan:

    research/01-original-novena-docs/sources/eDP_adapter_dvt1.PDF

SHA-256:

    ca9aa00cf64190d63469795f84381890928b48a98cd8d13298ecfcde46861f2a

PDF-Eigenschaften:

- PDF-Version: 1.3
- Seiten: 2
- Dateigröße: 631643 Byte
- Erzeugt mit: Altium Designer
- Producer: llPDFLib 3.x
- von pdfinfo gemeldetes Erstellungsdatum:
  Thu Mar 27 01:00:00 2014 CET

Der Titelblock des Schaltplans bezeichnet das Design als:

    LVDS to eDP adapter DVT1

Datum im Titelblock:

    3/27/2014

Die separat archivierte Kosagi-Seite zum eDP-Adapter EVT bestätigt unabhängig
davon, dass der Produktions-eDP-Adapter den IT6251 verwendet.

---

## Erzeugte Forschungsartefakte

Textextraktion:

    research/01-original-novena-docs/extracts/edp-adapter-dvt1-layout.txt

SHA-256:

    3f9a984960e91d7929858391300dddae4678d25c25f2e18afccfdf0ac9bf0118

Gerenderte Schaltplanseite 1:

    research/01-original-novena-docs/extracts/edp-adapter-dvt1-pages/page-1.png

SHA-256:

    44f29c98b47885ec2af5ae70b81e636ca8a344ff097002d2b20deb9a915d8683

Gerenderte Schaltplanseite 2:

    research/01-original-novena-docs/extracts/edp-adapter-dvt1-pages/page-2.png

SHA-256:

    fd80950f0d4088808697e18518a49e8a32b0fba22f6437ba674e1b794f4d9b79

Der SHA-256-Wert des Quell-PDFs wurde nach Textextraktion und Rendering
erneut überprüft und blieb unverändert.

---

# 1. Host-I2C-Anbindung des IT6251

Auf Schaltplanseite 1 befindet sich U10C, ein IT6251.

Die relevanten Host-Steuerpins des IT6251 sind:

- Pin 15: PCSDA
- Pin 14: PCSCL
- Pin 13: PCADR

Der Schaltplan verbindet:

    PCSDA -> AUX_I2C_SDA

und:

    PCSCL -> AUX_I2C_SCL

Der Adapteranschluss JP10C führt:

    Pin 47 -> AUX_I2C_SDA
    Pin 48 -> AUX_I2C_SCL

Damit beweist der eDP-Adapter-Schaltplan, dass die externe I2C-Verbindung des
Adapters elektrisch mit der Host-Steuerschnittstelle des IT6251 verbunden ist.

Die Bezeichnung „AUX_I2C“ darf dabei nicht mit dem differentiellen
DisplayPort-AUX-Kanal verwechselt werden.

Die eigentlichen eDP-AUX-Signale sind separat als differentielle Signale
eingezeichnet, unter anderem:

    EDP_TXAUX_P
    EDP_TXAUX_N

sowie auf der DisplayPort-Seite des IT6251:

    TXAUXP
    TXAUXN

AUX_I2C_SDA/SCL und EDP_TXAUX_P/N sind somit unterschiedliche
Schnittstellen.

---

# 2. Rekonstruierter Signalweg vom Mainboard zum IT6251

Der zuvor untersuchte PVT2-A-Mainboard-Schaltplan zeigt am
Novena-LCD-Anschluss:

    JPLCD Pin 6 -> I2C3_SDA
    JPLCD Pin 7 -> I2C3_SCL

Der eDP-Adapter-Schaltplan zeigt:

    JP10C Pin 47 -> AUX_I2C_SDA -> IT6251 PCSDA
    JP10C Pin 48 -> AUX_I2C_SCL -> IT6251 PCSCL

Zusammen mit dem dokumentierten kundenspezifischen Flexkabel zwischen
Novena-Mainboard und eDP-Adapter ergibt sich damit folgender rekonstruierter
Hardwarepfad:

    i.MX6Q I2C3
        |
        +-- EIM_D17 / I2C3_SCL
        +-- EIM_D18 / I2C3_SDA
        |
        v
    Novena-Mainboard I2C3
        |
        v
    JPLCD
        |
        v
    Display-Flexkabel
        |
        v
    eDP-Adapter JP10C
        |
        +-- Pin 48 AUX_I2C_SCL
        +-- Pin 47 AUX_I2C_SDA
        |
        v
    IT6251
        |
        +-- PCSCL Pin 14
        +-- PCSDA Pin 15

Damit ist der zuvor noch nicht vollständig belegte elektrische Pfad zwischen
dem I2C3-Bus der Novena und der Host-I2C-Schnittstelle des IT6251 geschlossen.

---

# 3. I2C-Pull-ups auf dem eDP-Adapter

Auf dem zweiseitigen DVT1-Schaltplan wurde kein eigener Pull-up-Widerstand von
AUX_I2C_SCL oder AUX_I2C_SDA zu einer Versorgungsschiene identifiziert.

R16C und R17C in der Nähe des IT6251 gehören zur Adressbeschaltung um
PCADR / I2C_ADR und dürfen nicht als SCL-/SDA-Pull-ups interpretiert werden.

Die derzeit verfügbaren Schaltplanbelege sprechen somit dafür, dass der
eDP-Adapter kein offensichtliches eigenes Pull-up-Paar für die
Host-I2C-Schnittstelle hinzufügt.

Der Mainboard-Schaltplan bleibt damit der bislang identifizierte Ort der
wesentlichen I2C3-Pull-ups.

Der tatsächlich bestückte Widerstandswert auf der getesteten
Produktionshardware ist weiterhin nicht geklärt.

In der Mainboard-Dokumentation besteht momentan folgende Diskrepanz:

- Dokumentübersicht: I2C3 mit 2,2-kOhm-Pull-up
- detailliertes Audio-Blatt: R10B/R11B mit 1 kOhm

Diese Diskrepanz muss anhand von Revisionen, ECO-Dokumenten und gegebenenfalls
den Designquellen untersucht werden, bevor eine Aussage über die tatsächlich
bestückte Produktionshardware getroffen wird.

---

# 4. Reset-Topologie des IT6251

IT6251 U10C Pin 7 ist:

    SYSRSTN

Dieser Pin ist mit folgendem Netz verbunden:

    RESET_N

RESET_N ist mit U11C verbunden.

U11C ist im Schaltplan bezeichnet als:

    APX803-29-SAG-7

mit:

    2.93V setpoint

Der Schaltplan bezeichnet diese Schaltung ausdrücklich als:

    Reset monitor (may not be necessary)

Der Reset-Monitor wird aus P3.3V versorgt.

Damit hängt der Reset-Zustand des IT6251 während eines echten Power-on von
der lokalen 3,3-V-Versorgung und dem Reset-Monitor ab.

Dies ist ein realer Hardwarezustand, der sich zwischen

1. einem echten Kaltstart/Power-on und
2. einem späteren Display-/LDB-Rebind innerhalb desselben Boots

unterscheiden kann.

Dieser Befund beweist NICHT, dass das Reset-Verhalten des IT6251 den
beobachteten I2C-Arbitration-Loss verursacht.

---

# 5. Versorgungsschienen des IT6251

Der IT6251 verwendet laut den Schaltplanseiten 1 und 2 mehrere
Versorgungsbereiche.

Relevante Netze sind:

    P3.3V
    P3.3VA
    P1.8VD
    P1.8VA
    P1.8VA_DP_PLL
    P1.8VA_LVDS_PLL

Diese Versorgungsschienen werden nicht alle als unabhängige externe
Spannungen eingespeist.

Mehrere davon werden lokal auf dem eDP-Adapter aus P3.3V erzeugt oder
gefiltert.

---

# 6. Digitale 1,8-V-Versorgung

Seite 2 zeigt U10P:

    LMR10510YSD

Eingang und Enable dieses Reglers sind mit P3.3V verbunden.

Der Wandler erzeugt den als

    P1.8VD

bezeichneten 1,8-V-Versorgungszweig.

Zum Schaltwandler gehören unter anderem L11P, D10P und die zugehörigen
Filter- und Abblockbauteile.

Für U10P ist kein separat softwaregesteuertes Enable-Signal eingezeichnet.

Der EN-Eingang liegt an P3.3V.

P1.8VD beginnt daher mit dem Anlegen von P3.3V hochzulaufen, abhängig vom
elektrischen Startverhalten des Reglers.

---

# 7. Analoge 1,8-V-Versorgung

Auf Seite 2 wird

    P1.8VA

über eine Filterung einschließlich L12P aus dem 1,8-V-Versorgungszweig
abgeleitet.

P1.8VA wird damit lokal auf dem Adapter erzeugt bzw. gefiltert und ist keine
separat gesteuerte externe Versorgung.

---

# 8. PLL-Versorgungen

Seite 2 zeigt U11P:

    MIC5319-1.8YML

Eingang und Enable dieses Reglers sind mit P3.3V verbunden.

Sein Ausgang ist bezeichnet als:

    P1.8VA_REG

Aus diesem Netz werden getrennte IT6251-PLL-Versorgungen erzeugt.

Über L13P:

    P1.8VA_REG
        ->
    P1.8VA_DP_PLL

Über L14P:

    P1.8VA_REG
        ->
    P1.8VA_LVDS_PLL

Auch für U11P ist kein separat softwaregesteuertes Enable-Signal
eingezeichnet.

Der EN-Eingang liegt an P3.3V.

---

# 9. Analoge 3,3-V-Versorgung

Seite 2 zeigt:

    P3.3V
        ->
    L15P
        ->
    P3.3VA

Die analoge 3,3-V-Versorgung des IT6251 wird somit lokal aus P3.3V gefiltert.

---

# 10. Bedeutung des Power-on-Ablaufs

Der Schaltplan belegt, dass nach dem Anlegen von P3.3V mehrere analoge
Vorgänge stattfinden:

1. der LMR10510-1,8-V-Wandler startet,
2. P1.8VD steigt an,
3. die gefilterte Versorgung P1.8VA wird aufgebaut,
4. der MIC5319-1,8-V-Regler startet,
5. P1.8VA_REG steigt an,
6. die DP- und LVDS-PLL-Versorgungen werden über ihre Filter aufgebaut,
7. P3.3VA wird über seine Filterung aufgebaut,
8. der APX803-Reset-Monitor steuert RESET_N,
9. der IT6251 verlässt seinen Hardware-Reset-Zustand.

Der Schaltplan allein belegt NICHT die exakte zeitliche Reihenfolge, die
Anstiegszeiten oder die Spannungsschwellen aller Vorgänge.

Dafür wären Datenblätter und/oder physikalische Messungen erforderlich.

Ein späterer LDB-Unbind/Rebind innerhalb desselben Boots wiederholt diesen
vollständigen Power-on-Ablauf des Adapters nicht, sofern P3.3V und die lokalen
Versorgungsschienen eingeschaltet bleiben.

Damit besitzt der eDP-Adapter Hardwarezustände, die sich zwischen einem
echten POR und einem späteren Display-Rebind unterscheiden können.

---

# 11. Zusammenhang mit dem aktuellen I2C3-Fehler

Die aktuelle Kernel-Instrumentierung hat folgenden Zustand nachgewiesen.

Kaltstartfehler:

    A=81/80 -> M0=93/80

Erfolgreicher LDB-Rebind innerhalb desselben Boots:

    A=81/80 -> M0=81/a0

Beim Kaltstartfehler gilt:

- IAL ist bereits im ersten beobachtbaren Post-MSTA-Sample gesetzt.
- MSTA ist in diesem Sample bereits wieder gelöscht.
- Die Abweichung entsteht somit im sehr kleinen Zeitfenster zwischen dem
  Pre-MSTA-Snapshot A und dem ersten Post-MSTA-Snapshot M0.

Der sichtbare I2SR-/I2CR-Zustand unmittelbar vor MSTA ist in beiden Fällen
gleich.

Der eDP-Schaltplan liefert nun einen zusätzlichen wichtigen Hardwarebefund:

Der IT6251 ist physikalisch mit diesem I2C-Bus verbunden und besitzt ein
POR-spezifisches Versorgungs- und Reset-Verhalten.

Damit ist der elektrische Zustand des IT6251 während eines Kaltstarts eine
relevante Variable für die weitere Untersuchung.

Dies beweist NICHT, dass der IT6251 die Quelle des IAL ist.

---

# 12. Weitere bekannte physikalische Teilnehmer an I2C3

Der eDP-Adapter darf nicht isoliert betrachtet werden.

Die bisherige Untersuchung des PVT2-A-Mainboard-Schaltplans hat weitere
elektrische Teilnehmer an I2C3 identifiziert.

Bekannte Teilnehmer bzw. elektrische Knoten sind:

- i.MX6Q-I2C3-Controller
- FT24C512A Utility-EEPROM
- FPGA-I/O-Pins
- eDP-Adapter / IT6251
- ES8328-Audiozweig hinter 330-Ohm-Serienwiderständen

Die wesentlichen I2C3-Pull-ups liegen laut Mainboard-Schaltplan außerdem an
einer als P3.3V_DELAYED bezeichneten Versorgung.

Der ES8328-Zweig besitzt ein eigenes geschaltetes Versorgungsverhalten.

Das FPGA besitzt eigene Reset-, Konfigurations- und Versorgungszustände.

Damit können mehrere physikalische Knoten POR-spezifische elektrische
Zustände besitzen.

---

# 13. Aktuelle Interpretation der Ursachenanalyse

Folgendes ist belegt:

- Kaltstart und erfolgreicher Same-Boot-Fall zeigen unmittelbar vor MSTA
  denselben sichtbaren I2SR-/I2CR-Snapshot.
- Die beiden Fälle unterscheiden sich unmittelbar nach dem Versuch des
  Controllers, Master zu werden.
- Der IT6251 ist direkt mit dem betroffenen physikalischen I2C-Bus verbunden.
- Der IT6251 besitzt einen Hardware-Reset-Monitor.
- Der IT6251 verwendet mehrere lokal erzeugte Versorgungsschienen.
- Diese Versorgungsschienen durchlaufen bei POR einen echten
  Einschaltvorgang.
- Ein späterer LDB-Rebind findet statt, nachdem diese Versorgungsschienen
  bereits stabilisiert sind.

Folgendes ist NICHT belegt:

- dass der IT6251 während des Kaltstarts SDA oder SCL fehlerhaft treibt,
- dass das RESET_N-Timing falsch ist,
- dass eine IT6251-Versorgung gegen Datenblattanforderungen verstößt,
- dass der eDP-Adapter den i.MX6-IAL verursacht,
- dass das FPGA den Fehler verursacht,
- dass der ES8328 den Fehler verursacht,
- dass das Mainboard-Pull-up-Netzwerk den Fehler verursacht,
- dass Linux den Transfer zu früh startet,
- dass U-Boot den Bus in einem problematischen Zustand hinterlässt.

An diesem Zwischenstand wird keine Ursache behauptet.

---

# 14. Offene Fragen

Die weitere Untersuchung der Originaldokumentation soll nach Möglichkeit
folgende Fragen beantworten:

1. Gibt es PVT2-ECO-Änderungen an I2C3?
2. Gibt es ECO-Änderungen an R10B/R11B oder den I2C3-Pull-ups?
3. Warum nennt die Dokumentübersicht 2,2 kOhm, während das detaillierte
   Schaltplanblatt 1 kOhm zeigt?
4. Gibt es ECO-Änderungen an den FPGA-Pins P3/P4?
5. Gibt es ECO-Änderungen am LCD-/eDP-I2C-Pfad?
6. Sind Probleme mit Reset oder Start des eDP-Adapters dokumentiert?
7. Sind bekannte Probleme bei der Initialisierung des IT6251 dokumentiert?
8. Enthält das PVT-Issue-Log Einträge zu I2C3, Audio, FPGA, LCD, eDP oder
   IT6251?
9. Initialisiert oder resettet der historische Bootloader den IT6251
   absichtlich vor Linux?
10. Welchen elektrischen Zustand besitzen die FPGA-Pins auf I2C3 während POR
    und FPGA-Konfiguration?
11. Welchen elektrischen Zustand besitzt der ES8328-Zweig, während seine
    Audio-Versorgung abgeschaltet ist?
12. Welche exakten Start- und Reset-Timing-Anforderungen besitzen IT6251 und
    die APX803-Schaltung?

---

# 15. Nächste Forschungsquellen

Wir bleiben zunächst in Forschungsblock 1.

Die nächsten Primärquellen sind:

1. Novena PVT2 ECO List
2. Novena PVT Issue Log
3. U-boot PVT Notes
4. IT6251-Diagnosedokumentation
5. originale Altium-Quellen, soweit sie zur Klärung von
   Schaltplanmehrdeutigkeiten erforderlich sind

Erst wenn der Block mit Originaldokumentation ausreichend abgeschlossen ist,
folgt:

2. historische Internet-, Kosagi- und Kernelquellen

und anschließend:

3. Vergleich und kontrollierte Tests der Kernelreihen 3.x, 5.x und 6.18

---

# Checkpoint-Regel

Aus diesen Befunden allein wird kein Kernel-Patch 0007 erstellt.

Retry-, Delay-, Bus-Recovery- oder Reset-Workarounds werden nicht als Ersatz
für die Ursachenanalyse eingeführt.

Die Novena wird nicht allein zur Fortsetzung dieser Dokumentationsphase
eingeschaltet.

Die aktuelle externe Test-SD und der dokumentierte 0006-Zustand bleiben die
experimentelle Ausgangsbasis.
