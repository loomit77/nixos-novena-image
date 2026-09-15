# Quellenübergreifende Sichtung der historischen Novena-Hardware und -Software

## 1. Zweck und Abgrenzung

Diese Auswertung fasst die Ergebnisse der quellenübergreifenden historischen
Novena-Sichtung aus Block 2.5 zusammen.

Sie verbindet die bereits archivierten Hardware-Unterlagen aus Block 1 mit
historischen Linux-, U-Boot- und NixOS-Quellen aus Block 2. Zusätzlich werden
vier für Block 2.5 erzeugte reproduzierbare Primärquellen-Auszüge
berücksichtigt.

Ziel ist nicht, aus historischen Quellen nachträglich eine Ursache für den
aktuellen Linux-6.18.49-Kaltstartfehler zu konstruieren. Stattdessen sollen
historisch belegte Eigenschaften, bekannte Wechselwirkungen und frühere
Robustheitsmaßnahmen identifiziert werden, die für die weitere
Root-Cause-Untersuchung relevant sind.

Der aktuelle experimentelle Ausgangspunkt bleibt der mit Patch 0006
dokumentierte Unterschied:

    Kaltstart FEHLER:    A=81/80 -> M0=93/80
    LDB-Rebind ERFOLG:   A=81/80 -> M0=81/a0

Die historischen Quellen beweisen nicht, dass ein früher dokumentiertes
Novena-Problem mit diesem aktuellen Fehler identisch ist.

Insbesondere wird aus zeitlicher Nähe, ähnlichen Symptomen oder historischen
Workarounds keine Kausalität für Linux 6.18.49 abgeleitet.


## 2. Quellenbasis

### 2.1 Hardware-Unterlagen aus Block 1

Die zentrale Hardware-Basis bilden die bereits archivierten und analysierten
Novena-PVT-Unterlagen und die Dokumentation des eDP-Adapters.

Bestehende Analysen:

- `research/01-original-novena-docs/notes/edp-adapter-dvt1-i2c-power.md`
- `research/01-original-novena-docs/notes/novena-pvt2-source-metadata.md`
- `research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md`
- `research/01-original-novena-docs/notes/pvt2-eco-analysis.md`
- `research/01-original-novena-docs/notes/pvt-issue-log-analysis.md`
- `research/01-original-novena-docs/notes/u-boot-documentation-analysis.md`

Diese Quellen liefern insbesondere die physische I2C3-Topologie, die
Stromversorgungsstruktur des Audio- und Displaypfads, die Verbindung zum
IT6251 sowie Hinweise auf Produktionsänderungen und ECOs.


### 2.2 Bereits archivierte Software-Analysen aus Block 2

Bestehende Analysen:

- `research/02-historical-software/notes/linux-i2c-imx-5.7-vs-6.18-analyse.md`
- `research/02-historical-software/notes/nixos-wiki-kosagi-novena-revision-21513.md`
- `research/02-historical-software/notes/u-boot-v2014.10-novena-rc5-i2c-analysis.md`
- `research/02-historical-software/notes/xobs-it6251-dump-dptx-70b07ef-analysis.md`

Diese Analysen bleiben maßgeblich für ihre jeweiligen Detailbereiche. Die
vorliegende Datei wiederholt ihre vollständigen Befunde nicht.


### 2.3 Neue reproduzierbare Belege aus Block 2.5

Für die quellenübergreifende Sichtung wurden vier kompakte Primärquellen-
Auszüge erzeugt:

1. `extracts/quellenuebergreifend/xobs-novena-linux-it6251-powerup-historie.txt`
2. `extracts/quellenuebergreifend/xobs-u-boot-novena-it6251-power-ready-sequenz.txt`
3. `extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt`
4. `extracts/quellenuebergreifend/novena-next-linux-dts-power-policy-matrix.txt`

Sie sichern die für die weitere Untersuchung wichtigsten historischen
Software-Belege reproduzierbar im Projekt.


## 3. Physische I2C3-Topologie

Die Hardware-Unterlagen zeigen, dass I2C3 auf der Novena kein isolierter
Display-Bus ist.

Die CPU-Pads EIM_D17 und EIM_D18 bilden SCL beziehungsweise SDA von I2C3.
An diesem Bus beziehungsweise seinen direkt verbundenen Zweigen befinden
sich mehrere funktional unterschiedliche Komponenten.

Dazu gehören insbesondere:

- ES8328-Audiocodec an Adresse `0x11`,
- Novena-EEPROM an Adresse `0x56`,
- IT6251-Displaybridge an Adresse `0x5c`,
- weitere historisch dokumentierte I2C3-Komponenten beziehungsweise
  Erweiterungen.

Der FPGA ist ebenfalls direkt mit den I2C3-Leitungen verbunden.

Der Displaypfad führt I2C3 vom Novena-Mainboard über den LCD-Anschluss und
das Flexkabel zum eDP-Adapter und dort zum IT6251.

Damit können elektrische oder versorgungsabhängige Zustände eines
Teilnehmers grundsätzlich Auswirkungen auf andere Teilnehmer desselben
physischen Busses haben.

Diese Aussage beschreibt die Hardware-Topologie. Sie beweist nicht, dass
eine bestimmte Komponente den aktuellen Linux-6.18.49-IAL-Zustand erzeugt.


## 4. Besonderheit des Audiozweigs

Die PVT-Unterlagen zeigen eine für die weitere Untersuchung wichtige
Eigenschaft des ES8328-Zweigs.

Die I2C3-Leitungen besitzen mainboardseitige Pull-ups zur verzögerten
3,3-V-Versorgung. Auf der Codec-Seite existiert zusätzlich eine
versorgungsabhängige Beschaltung, wobei insbesondere die SDA-Seite mit der
geschalteten Audio-Versorgung zusammenhängt.

Damit ist die Stromversorgung des Audiopfads elektrisch nicht vollständig
von den I2C3-Signalen unabhängig.

Die historischen Unterlagen enthalten außerdem unterschiedliche Angaben
zur Pull-up-Bestückung. Die tatsächliche Produktionsbestückung darf daher
nicht ausschließlich aus einer einzelnen Schaltplan- oder BOM-Version
abgeleitet werden.

Der PVT-Issue-Log zeigt allgemein, dass laufende Produktionsänderungen nicht
zwingend in allen veröffentlichten Unterlagen synchron nachgeführt wurden.


## 5. Historisch belegte ES8328-/I2C3-Wechselwirkung

Ein besonders wichtiger Primärbeleg ist der Linux-Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

vom 1. April 2020.

Der Commit änderte den Regulator `es8328-power` von

`regulator-boot-on`

auf

`regulator-always-on`.

Die Commit-Nachricht dokumentiert ausdrücklich, dass das Abschalten dieser
Versorgung offenbar den I2C3-Bus beeinträchtigte und dadurch unter anderem
Display, EEPROM und Senoko ausfielen.

Der vollständige reproduzierbare Beleg befindet sich in:

`extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt`

### Bewertung

Dies ist ein starker historischer Beleg dafür, dass auf realer
Novena-Hardware eine Wechselwirkung zwischen der Audio-Stromversorgung und
dem gemeinsam genutzten I2C3-Bus beobachtet wurde.

Es handelt sich nicht lediglich um eine aus dem Schaltplan abgeleitete
Möglichkeit. Die Wechselwirkung wurde von einem historischen
Novena-Entwickler ausdrücklich als Grund für eine DTS-Änderung dokumentiert.

Der Beleg zeigt jedoch nicht:

- welchen exakten elektrischen Zustand die I2C-Leitungen dabei annahmen,
- ob der i.MX6-I2C-Controller dabei IAL setzte,
- ob der Fehler nur beim Kaltstart auftrat,
- ob der Zustand dem heutigen `A=81/80 -> M0=93/80` entspricht.

Die historische ES8328-Wechselwirkung ist deshalb eine hochrangige
Untersuchungsspur, aber noch keine Root-Cause-Feststellung für Linux 6.18.49.


## 6. Historische IT6251-Power-up-Robustheit unter Linux

Die Entwicklung des historischen xobs/novena-linux-Treibers zeigt, dass
das Einschalten und Erkennen des IT6251 wiederholt angepasst wurde.

Die relevante Commit-Kette lautet:

`7fdfaf801018ff03aa6497deb4ef15e2426a4486`
→ `73a0667a7994b92d674a073bc38dc1e553f4dbbd`
→ `fc52d5f71a01541572adf259b0cc174dc3df45ce`
→ `3d85836ee1377d445531928361809612aa0a18db`

Der ursprüngliche Treiber enthielt bereits den Hinweis, dass mehrere
Versuche zum Erkennen des IT6251 notwendig sein konnten.

Später wurde die Zahl der Versuche erhöht.

Der Commit

`fc52d5f71a01541572adf259b0cc174dc3df45ce`

trägt ausdrücklich den Betreff:

`it6251: Attempt to make powerup more robust`

Dabei wurden die Abstände zwischen fehlgeschlagenen Erkennungsversuchen
deutlich vergrößert.

Der spätere Commit

`3d85836ee1377d445531928361809612aa0a18db`

verbesserte weitere Aspekte der Stabilitätsprüfung und des
Wiederanlaufverhaltens.

Der vollständige reproduzierbare Verlauf befindet sich in:

`extracts/quellenuebergreifend/xobs-novena-linux-it6251-powerup-historie.txt`

### Wichtige Abgrenzung

`regulator_enable()` wird im historischen Linux-Treiber vor dem ersten
Product-ID-Leseversuch aufgerufen. Es existiert dort jedoch keine feste
initiale Wartezeit vor diesem ersten I2C-Zugriff.

Die längeren Wartezeiten folgen erst nach fehlgeschlagenen
Erkennungsversuchen.

Eine früher im Quelltext vorhandene Ausgabe mit einem Ausdruck auf Basis
von `150000` war keine feste 150-ms-Verzögerung vor dem ersten Zugriff.

Daraus darf deshalb nicht abgeleitet werden, dass der historische Treiber
vor jedem ersten IT6251-Zugriff pauschal 150 ms wartete.


## 7. Historische U-Boot-Power-/Ready-Sequenz des IT6251

Der U-Boot-Commit

`331ae846ad9fee532cf04268da76701093e4e4ed`

vom 16. Dezember 2014 führte die eigentliche LVDS-Unterstützung für Novena
ein.

Der Quelltext dokumentiert eine explizite IT6251-Power-Sequenz über
GPIO5_28:

1. Versorgung beziehungsweise Power-GPIO auf LOW,
2. 10 ms warten,
3. GPIO auf HIGH,
4. 20 ms warten,
5. anschließend die IT6251-Bereitschaft wiederholt per I2C prüfen.

Die Bereitschaftsprüfung kontrolliert die erwarteten Chip-IDs. Die
Erkennung wird über einen Zeitraum wiederholt, bevor der Displaypfad
aufgegeben wird.

Erst nach erfolgreicher Bereitschaft folgen die eigentliche
IT6251-Initialisierung und schließlich die Freigabe der
Hintergrundbeleuchtung.

Der reproduzierbare Beleg befindet sich in:

`extracts/quellenuebergreifend/xobs-u-boot-novena-it6251-power-ready-sequenz.txt`

Der U-Boot-Quelltext verweist selbst auf Sean Cross' historischen
Novena-Linux-Code aus Commit

`3d85836ee1377d445531928361809612aa0a18db`.

Damit ist die Provenienzkette zwischen dem historischen Linux-Treiber und
der späteren U-Boot-Implementierung direkt im Quelltext dokumentiert.

### Bewertung

U-Boot behandelt den Power-/Reset-/Ready-Zustand des IT6251 ausdrücklich
als relevante Voraussetzung für eine zuverlässige Initialisierung.

Die U-Boot-Sequenz ist jedoch nicht identisch mit der historischen
Linux-Strategie.

Insbesondere verwendet U-Boot einen expliziten LOW-HIGH-Power-Zyklus,
während der historische Linux-Treiber hauptsächlich über Regulatorsteuerung
und wiederholte Erkennungsversuche arbeitet.

Diese Unterschiede sind für die weitere Untersuchung relevant, dürfen aber
nicht ohne Test als notwendiger Fix für Linux 6.18.49 übernommen werden.


## 8. Entwicklung der DTS-Power-Policies

Die DTS-Zustände von Linux 4.4, 4.19-WIP3, 5.7-rc2 und 6.6 zeigen, dass die
Power-Policy des Novena-Display- und Audiopfads historisch mehrfach geändert
wurde.

Der reproduzierbare Vergleich befindet sich in:

`extracts/quellenuebergreifend/novena-next-linux-dts-power-policy-matrix.txt`

### Linux 4.4

Audio:

- `es8328-power`
- GPIO5_17
- `startup-delay-us = <400000>`
- `regulator-boot-on`

Display:

- GPIO5_28
- `startup-delay-us = <2000000>`
- weder `regulator-boot-on` noch `regulator-always-on`

LVDS:

- GPIO4_15
- keine feste Boot-/Always-on-Policy


### Linux 4.19-WIP3

Audio:

- GPIO5_17
- 400 ms Startverzögerung
- `regulator-always-on`

Display:

- GPIO5_28
- 2 s Startverzögerung
- `regulator-boot-on`

LVDS:

- GPIO4_15
- keine feste Boot-/Always-on-Policy


### Linux 5.7-rc2

Audio:

- GPIO5_17
- 400 ms Startverzögerung
- `regulator-boot-on`

Display:

- GPIO5_28
- 200 ms Startverzögerung
- `regulator-always-on`

LVDS:

- GPIO4_15
- `regulator-always-on`


### Linux 6.6

Der für diese Untersuchung relevante Zustand entspricht im Wesentlichen
dem 5.7-rc2-Zustand:

Audio:

- GPIO5_17
- 400 ms Startverzögerung
- `regulator-boot-on`

Display:

- GPIO5_28
- 200 ms Startverzögerung
- `regulator-always-on`

LVDS:

- GPIO4_15
- `regulator-always-on`


## 9. Historische Änderungen der Displayversorgung

Mehrere DTS-Commits zeigen, dass die Display-Stromversorgung wiederholt
Gegenstand gezielter Änderungen war.

Der Commit

`5f3c4528c7714e23b174c232af70fcaafecf2ba1`

trägt den Betreff:

`ARM: dts: restart the display power supplies on boot`

Dabei wurde die Power-Policy so verändert, dass die Displayversorgung beim
Booten vollständig neu gestartet werden konnte.

Der spätere Commit

`29f549a50d76fb73f88cbcdc4a4391c266f4f557`

trägt den Betreff:

`ARM: dts: imx6q-novena: Bring up reg_display on boot`

und änderte die Policy erneut.

Zusammen mit den späteren `always-on`-Zuständen zeigt dies, dass es über die
Lebensdauer der Novena-Kernelquellen keine einzige unveränderte
Display-Power-Policy gab.

### Bewertung

Die historischen Änderungen sind ein Beleg dafür, dass die
Display-Stromversorgung für die Funktionsfähigkeit praktisch relevant war.

Sie beweisen nicht, dass eine bestimmte historische Policy für alle
Novena-Revisionen und alle Kernelstände optimal ist.

Für ein universelles NixOS-Image darf deshalb nicht einfach ein einzelner
historischer DTS-Zustand als endgültige Referenz übernommen werden.


## 10. Hardwareseitiger Display-Power-/Reset-Pfad

Die Block-1-Unterlagen ergänzen die Softwaregeschichte um die physische
Displayarchitektur.

Der IT6251 auf dem eDP-Adapter besitzt einen eigenen Resetpfad über einen
APX803-Spannungsmonitor.

Die Adapterversorgung und der Reset hängen damit vom tatsächlichen
Spannungsaufbau ab.

Ein echter Kaltstart und eine erneute Initialisierung innerhalb eines
bereits laufenden Systems sind elektrisch nicht notwendigerweise
gleichwertige Ausgangszustände.

Dies ist besonders relevant, weil der aktuelle 0006-Versuch genau einen
solchen Unterschied zeigt:

- Kaltstart: erster Master-/START-Übergang schlägt fehl,
- erneutes LDB-Binden im selben Boot: derselbe sichtbare Prä-START-Zustand
  führt zu einem erfolgreichen Master-/START-Übergang.

Die Hardwareunterlagen liefern dafür einen plausiblen Kontext, aber keinen
Beweis für die Ursache des beobachteten IAL.


## 11. Weitere Hardwarehinweise

Der PVT-ECO-Verlauf enthält mindestens einen dokumentierten Fall einer
Kaltstart-/Power-on-Stabilitätskorrektur an der Novena-Stromversorgung.

ECO23 änderte die Beschaltung zur Verbesserung des Power-on-Verhaltens eines
Regulators.

Dieser Befund zeigt, dass reale Novena-Hardware historisch
Power-on-spezifische Probleme besaß.

Es gibt jedoch keinen Beleg dafür, dass ECO23 unmittelbar den heutigen
I2C3-/IT6251-Fehler erklärt.

Der Befund bleibt deshalb Kontext und darf nicht als Root-Cause-Beweis
verwendet werden.


## 12. U-Boot und frühe Hardwareinitialisierung

Die ursprüngliche Novena-Bootarchitektur verwendet SPL und anschließend
das vollständige U-Boot.

Historische U-Boot-Versionen initialisieren mehrere für die aktuelle
Untersuchung relevante Hardwarezustände bereits vor Linux.

Dazu gehören unter anderem:

- Audio-Power über GPIO5_17,
- FPGA-Zustand,
- I2C-Controller und I2C-Pads,
- später der Display-/IT6251-Pfad.

Der historische U-Boot-I2C-Code besitzt außerdem eine Bus-Recovery-
Funktion, die bei nicht freigegebenen SDA-/SCL-Leitungen den Controller
temporär auf GPIO umschaltet und SCL pulst.

Dies zeigt, dass die historische Bootsoftware einen blockierten I2C-Bus
ausdrücklich als möglichen Betriebszustand berücksichtigt.

Es beweist nicht, dass diese Recovery im heutigen Kaltstartfall benötigt
wird oder dass sie den aktuellen IAL verhindern würde.


## 13. NixOS- und novena-next-Kontext

Die historischen novena-next-Quellen zeigen den späteren Versuch, Novena
mit neueren Kernel- und NixOS-Versionen weiterzubetreiben.

Die archivierte NixOS-Wiki-Revision verweist auf:

- generische ARMv7-NixOS-Images,
- `novena_defconfig`,
- SPL,
- U-Boot,
- extlinux,
- SATA-Boot,
- das separate Repository `novena-next/docs`.

Die quellenübergreifende Sichtung bestätigte, dass
`novena-next/docs` tatsächlich ein separates Repository ist.

Dieses Repository ist vor allem als späteres Archiv historischer
Novena-Unterlagen und als Wartungskontext zu bewerten. Darin enthaltene
Design-Dateien, die bereits aus ursprünglicheren Quellen in Block 1
archiviert wurden, stellen keine unabhängige zweite Bestätigung desselben
Hardwarebefunds dar.

Die Textdokumentation zeigt außerdem, dass spätere Novena-Maintainer den
älteren 4.4-Kernel weiterhin als funktionale Referenz gegenüber
4.19-Varianten verwendeten.

Dies ist für spätere Kernelvergleiche relevant.


## 14. Was die Quellen gemeinsam belegen

Über Hardware-, Linux- und U-Boot-Quellen hinweg ergibt sich ein
konsistentes Gesamtbild:

Die zuverlässige Initialisierung des Novena-Displaypfads hängt nicht nur
von der eigentlichen IT6251-Registerprogrammierung ab.

Historisch relevant waren mindestens:

- Zustand der gemeinsamen I2C3-Infrastruktur,
- Audio-Stromversorgung,
- Display-Stromversorgung,
- LVDS-Stromversorgung,
- IT6251-Power-/Ready-Zustand,
- zeitliche Abfolge der Initialisierung,
- Reset- und Power-on-Zustände,
- frühe U-Boot-Hardwareinitialisierung.

Mehrere dieser Punkte wurden unabhängig voneinander zu unterschiedlichen
Zeitpunkten von Novena-Entwicklern geändert oder mit
Robustheitsmaßnahmen versehen.

Das rechtfertigt, diese Bereiche bei der heutigen Root-Cause-Suche höher
zu priorisieren.


## 15. Was die Quellen ausdrücklich nicht beweisen

Keiner der untersuchten historischen Belege dokumentiert den heutigen
Linux-6.18.49-Zustand

`A=81/80 -> M0=93/80`

oder einen exakt gleichwertigen Registerübergang.

Insbesondere ist historisch nicht belegt, dass:

- derselbe IAL unmittelbar beim Setzen von MSTA auftrat,
- MSTA dabei sofort wieder gelöscht wurde,
- der Fehler ausschließlich nach POR auftrat,
- eine ES8328-Versorgungsänderung genau diesen Registerzustand erzeugte,
- eine IT6251-Verzögerung genau diesen Registerzustand verhinderte,
- ein U-Boot-Power-Zyklus genau diesen Registerzustand verhinderte.

Die historischen Quellen dürfen deshalb nicht als Beweis für eine bereits
gefundene Root Cause interpretiert werden.


## 16. Bedeutung für den aktuellen 0006-Befund

Patch 0006 hat den aktuellen Fehler wesentlich genauer lokalisiert.

Vor dem Master-/START-Übergang ist der sichtbare Controllerzustand bei
Kaltstart und erfolgreichem Same-Boot-LDB-Rebind gleich:

`A=81/80`

Direkt nach dem Setzen von MSTA divergieren die Zustände:

Kaltstart:

`M0=93/80`

Erfolgreicher Same-Boot-Rebind:

`M0=81/a0`

Damit liegt die entscheidende Abweichung spätestens beim ersten
beobachtbaren Zustand nach der Master-Anforderung vor.

Die historische Sichtung ändert diese experimentelle Aussage nicht.

Sie erweitert jedoch die Menge der Zustände, die vor diesem Übergang
unterschiedlich sein können, obwohl sie in den sichtbaren I2C-Controller-
Registern A noch nicht erscheinen.

Dazu gehören insbesondere:

- physischer Zustand von SDA und SCL,
- Versorgung eines Bus-Teilnehmers,
- Pull-up- beziehungsweise Clamp-Zustände,
- IT6251-Power-/Reset-Zustand,
- Audio-Power-Zustand,
- Pin-/Pad-Zustand,
- Clock-/Reset-/Runtime-PM-Zustand des Controllers,
- von U-Boot hinterlassene Hardwarezustände.


## 17. Priorisierung für die nächste Root-Cause-Phase

Aus der kombinierten historischen und aktuellen Evidenz ergibt sich eine
Priorisierung für die nächste Untersuchung.

### Priorität 1: ES8328-/Audio-Power und I2C3

Begründung:

- direkte elektrische Verbindung zum gemeinsamen I2C3-Bus,
- historische Commit-Nachricht dokumentiert ausdrücklich eine
  I2C3-Störung durch Abschalten von `es8328-power`,
- aktueller Fehler unterscheidet Kaltstart und Same-Boot-Zustand.

Als Nächstes sollte deshalb der tatsächliche Zustand von GPIO5_17 und des
zugehörigen Regulators im aktuellen Linux-6.18.49-Bootpfad exakt bestimmt
werden.

Noch ist nicht bewiesen, dass dies die Ursache ist.


### Priorität 2: IT6251-/Display-Power-Sequenz

Begründung:

- historische Linux-Treiber benötigten wiederholte Power-up-/Readiness-
  Versuche,
- U-Boot verwendet ausdrücklich LOW → 10 ms → HIGH → 20 ms und
  anschließendes Ready-Polling,
- historische DTS-Power-Policies wurden mehrfach verändert,
- der aktuelle Fehler tritt beim Kaltstart auf, während ein Same-Boot-
  Rebind funktioniert.

Als Nächstes ist daher zu klären, welcher tatsächliche Zustand von
GPIO5_28, Displayregulator und IT6251-Reset beim ersten fehlgeschlagenen
Zugriff vorliegt.

Auch dies ist zunächst eine Untersuchungsspur, kein vorweggenommener Fix.


### Priorität 3: früher Bootloader-Zustand

Die heute verwendete U-Boot-Version 2020.07 kann andere Hardwarezustände
hinterlassen als die historischen xobs-Versionen.

Vor einem Kernel-Workaround sollte daher geklärt werden, welche
I2C3-, Audio-, FPGA- und Displayzustände das tatsächlich eingesetzte
U-Boot an Linux übergibt.


### Priorität 4: Controller-/Pinctrl-/Runtime-PM-Zustand

Falls die Versorgungsspuren den Fehler nicht erklären, bleiben die bereits
aus der 5.7-vs-6.18-Analyse bekannten Controllerzustände hoch relevant:

- Pinctrl-Umschaltung,
- Clock-Aktivierung,
- Reset-/Registerzustand,
- Runtime-PM-Resume,
- Zustand unmittelbar vor dem ersten Transfer.

Der gleiche sichtbare A-Snapshot schließt Unterschiede in diesen
nicht vollständig sichtbaren Zuständen nicht aus.


### Priorität 5: kontrollierter Kernelvergleich

Die historische DTS- und Kernelentwicklung zeigt, dass mindestens folgende
Stände als Vergleichspunkte sinnvoll sind:

- 4.4,
- 4.19-WIP3,
- 5.7-rc2,
- 6.6,
- aktueller 6.18.49-Teststand.

Ein solcher Vergleich sollte jedoch erst nach Definition einer konkreten
Fragestellung erfolgen und nicht als ungerichtete Suche.


## 18. Konsequenz für Patch 0007

Aus Block 2.5 folgt noch kein Patch 0007.

Insbesondere wird jetzt nicht vorsorglich:

- ein Retry eingebaut,
- eine zusätzliche Wartezeit eingefügt,
- Bus-Recovery aktiviert,
- der IT6251 hart power-gecycelt,
- `es8328-power` dauerhaft erzwungen,
- eine historische DTS-Power-Policy kopiert.

Solche Änderungen könnten den Fehler maskieren, ohne seine Ursache zu
erklären.

Der nächste Test muss stattdessen eine konkrete Hypothese falsifizierbar
prüfen.

Der dokumentierte 0006-Zustand bleibt bis dahin der experimentelle
Ausgangspunkt.


## 19. Schlussfolgerung

Die quellenübergreifende Sichtung hat keinen historischen Beleg gefunden,
der den aktuellen Linux-6.18.49-Kaltstartfehler unmittelbar erklärt.

Sie hat jedoch mehrere bislang getrennte Befunde zu einem deutlich
schärferen Gesamtbild verbunden.

Besonders wichtig sind drei historisch belastbare Erkenntnisse:

Erstens wurde auf realer Novena-Hardware ausdrücklich eine Wechselwirkung
zwischen `es8328-power` und der Funktionsfähigkeit von I2C3 dokumentiert.

Zweitens wurde die IT6251-Power-up-/Readiness-Behandlung sowohl im
historischen Linux-Treiber als auch in U-Boot mehrfach beziehungsweise
explizit auf Robustheit ausgelegt.

Drittens änderte sich die Power-Policy von Audio-, Display- und
LVDS-Versorgung über die historischen Kernelstände erheblich.

Diese Befunde passen grundsätzlich dazu, dass ein echter Kaltstart einen
anderen versteckten Hardwarezustand besitzen kann als ein erfolgreicher
Same-Boot-Rebind.

Sie beweisen jedoch nicht, welcher dieser Zustände den aktuellen
`A=81/80 -> M0=93/80`-Übergang verursacht.

Damit ist Block 2.5 keine Fix-Entscheidung, sondern eine
Priorisierungshilfe für die nächste experimentelle Root-Cause-Phase.
