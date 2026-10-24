# Novena – Display-Übergabecheckpoint

Datum: 2026-10-24
Status: QUALIFIED – statische Offline-Analyse abgeschlossen
Root-Cause der Dunkelphase: nicht nachgewiesen

## Beobachtung

Bei historischen Debian-Systemen, einschließlich des Systems
auf der internen MMC, wurde keine entsprechende Dunkelphase
der Hintergrundbeleuchtung beobachtet.

Beim aktuellen NixOS-Boot wird das Display während des
Bootvorgangs vollständig dunkel und zum Login wieder aktiv.

Die Novena ist derzeit nicht aufgebaut. Die Untersuchungen
71–75 erfolgten deshalb auf Basis archivierter Images,
Device Trees und Quelltexte auf foobox.

## Historische U-Boot-Stände

Vorhandene Binärartefakte:

- U-Boot 2014.10-rc3
- U-Boot 2015.04-rc1
- U-Boot 2020.07

Der genaue Zusammenhang zwischen allen historischen
Binärartefakten und konkreten MMC-Bootvorgängen ist
nicht vollständig nachgewiesen.

U-Boot 2015 enthält die Bootumgebungslogik mit
`lcddet`, `keep_lcd` und `video`.

Historische U-Boot-Quellen zeigen außerdem:

- Initialisierung der IT6251-Bridge.
- GPIO-gesteuertes Ausschalten und Wiedereinschalten
  ihrer Versorgung mit Wartezeiten von 10 und 20 ms.
- Einschalten von NOVENA_BACKLIGHT_PWR_GPIO.
- Setzen von NOVENA_BACKLIGHT_PWM_GPIO auf HIGH
  nach erfolgreicher Bridge-Initialisierung.

Die tatsächliche Ausführung bei einem konkreten
historischen Boot wurde damit nicht bewiesen.

## Historischer Device Tree gegenüber NixOS

Der historische DTB-Kandidat und untersuchte NixOS-DTBs
verwenden `pwm-backlight` und den PWM-Controller
`pwm@2080000`.

Die PWM-Periode beträgt 10.000.000 ns.
Die Helligkeitstabelle besitzt 13 Einträge.
Der Standardhelligkeitsindex ist 12.

Ein funktionierender NixOS-Displaystand vom 2026-09-11
besitzt `regulator-always-on` für `reg_display`
und `reg_lvds_lcd`.

Spätere untersuchte NixOS-DTBs besitzen diese
Eigenschaft nicht. Zugleich wurde die
`reg_display`-Einschaltverzögerung von 200 ms
auf 2.000 ms erhöht.

Der historische DTB-Kandidat ist nicht zweifelsfrei
als der ursprüngliche MMC-DTB identifiziert.

## Aktueller Linux-Treiber

Der DRM-IT6251-Treiber verwendet das Regulator-Framework:

- `it6251_power_up()` mit `regulator_enable()`.
- `it6251_power_down()` mit `regulator_disable()`.
- Erneute Power-Cycles bei bestimmten Fehlern.
- DRM-Bridge-Callbacks zur Aktivierung und Abschaltung.

Die Hintergrundbeleuchtung wird separat über
`pwm-backlight`, Panel und die zugehörigen
Versorgungsbeziehungen gesteuert.

## Technische Bewertung

Die plausibelste derzeitige Arbeitshypothese ist
eine Unterbrechung bei der Übergabe der Display-,
Panel- oder Backlight-Steuerung von U-Boot an
den modernen Linux-Kernel.

Die konkrete Ursache und der Zeitpunkt des
Abschaltens sind nicht nachgewiesen.

Insbesondere darf weder eine fehlende
`regulator-always-on`-Eigenschaft noch ein
bestimmter Treiber-Callback ohne weitere Evidenz
als alleinige Ursache gelten.

Die abgeschlossene I2C3/ES8328-Root-Cause
wird durch diese Untersuchung nicht wieder eröffnet.

## Entscheidung

Die statische Untersuchung wird hier qualifiziert
abgeschlossen.

Keine weiteren gleichartigen Suchblöcke ohne neue
diskriminierende Evidenz.

Beim nächsten ohnehin genehmigten Hardwareeinsatz
gegebenenfalls einen gezielten Bootversuch mit
zeitlichem Abgleich von Bildschirmzustand,
Backlight und DRM-Meldungen durchführen.

Kein Kernelpatch, kein Build und kein Hardwaretest
aus dieser Dokumentation automatisch freigegeben.

## Zurück zum Hauptziel

Priorität erhält wieder das vollständige,
reproduzierbare NixOS-SD-Image.

Bestätigter VM102/VM104-Imagecheckpoint:

- Größe: 3240607744 Bytes
- SHA256:
  40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a

Die byteidentischen Imageartefakte wurden bestätigt.
Die vollständige Unabhängigkeit sämtlicher
Abhängigkeits-Builds ist nicht bewiesen.

Nächster Arbeitsgegenstand: bestehende
Image-Qualifikation, Bootdateien, Image-Provenienz
und offene Release-Anforderungen auswerten,
bevor neue Builds oder Medienzugriffe geplant werden.

## Originalnachweise

Alle Protokolle mit TXT und SHA256-Sidecar im
GitHub-Archiv `loomit77/project-block-logs`,
Verzeichnis `novena/2026/10/`:

- NOVENA-DISPLAY-CHAIN-70
- NOVENA-DISPLAY-HISTORY-71 – PARTIAL/FAIL,
  SSH-Timeout bei nicht aufgebauter Novena.
- NOVENA-DISPLAY-HISTORY-72 – PASS
- NOVENA-DISPLAY-HISTORY-73 – PASS
- NOVENA-DISPLAY-HISTORY-74 – PASS
- NOVENA-DISPLAY-HISTORY-75 – PASS

Außerdem:
- NOVENA-VM-IMAGE-INVENTORY-66
- NOVENA-IMAGE-QUALIFICATION-68/69
- docs/VM102-VM104-REPRODUCIBILITY-2026-10-24.md

Dieser Checkpoint ersetzt keine Originalprotokolle.
