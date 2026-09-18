# Block 3.13E – Quellenbasierte Eingrenzung ohne Hardwaremessung

## 1. Zweck

Block 3.13E setzt die elektrische Root-Cause-Untersuchung nach Block 3.13D
unter einer zusätzlichen praktischen Randbedingung fort:

Für die weitere Untersuchung stehen keine elektrischen Messgeräte zur
Verfügung und es werden keine Hardwaremessungen durchgeführt.

Die in Block 3.13 vorbereitete elektrische Messplanung wird deshalb nicht
simuliert und nicht durch unbelegte Annahmen ersetzt.

Stattdessen wird untersucht, welche der offenen Root-Cause-Kandidaten durch
bereits vorhandene Originalquellen, historische Softwarequellen und
reproduzierbare Git-Historie weiter gestützt, geschwächt oder eingegrenzt
werden können.

Die dauerhafte Novena-Board-Konfiguration

`es8328-power = regulator-always-on`

bleibt dabei unverändert.

Es wird kein Kernel-Patch `0007` begonnen.


## 2. Ausgangslage

Vor Block 3.13E waren folgende experimentelle Befunde gesichert:

- Im instrumentierten Baseline-Cold-FAIL wird `es8328-power` während des
  Bootvorgangs automatisch abgeschaltet.
- Später tritt auf I2C3 die untersuchte Fehlersignatur
  `A=81/80 -> M0=93/80` auf.
- Der fehlgeschlagene M0-Zustand enthält `IAL`; gleichzeitig ist `MSTA`
  nicht gesetzt.
- Ein erfolgreicher Same-Boot-LDB-Rebind zeigt dagegen
  `A=81/80 -> M0=81/a0`.
- H3-1R änderte als einzige semantische Device-Tree-Eigenschaft am
  Audio-Regulator die dauerhafte Aktivierung durch
  `regulator-always-on`.
- Unter H3-1R bestanden fünf von fünf vorregistrierten echten
  POR-Cold-Boots.
- In diesen fünf vollständigen Kernel-Logs trat die untersuchte
  `93/80`-/`arbitration lost`-Signatur nicht auf.
- IT6251 initialisierte erfolgreich und das Display funktionierte.

Damit ist ein kausaler Beitrag des Abschaltens des ES8328-Power-Domains zur
untersuchten Fehlerklasse stark experimentell gestützt.

Nicht entschieden ist dadurch der genaue elektrische Mechanismus.


## 3. Elektrische Ausgangstopologie

Die Auswertung der PVT2-A-Schaltplanunterlagen hatte zuvor folgende
relevante Topologie ergeben:

- `I2C3_SCL -> R26A 330R -> AUD_I2C3_SCL -> ES8328E`
- `I2C3_SDA -> R27A 330R -> AUD_I2C3_SDA -> ES8328E`
- R10B zieht `AUD_I2C3_SCL` über 1 kOhm nach `P3.3V_DELAYED`.
- R11B zieht `AUD_I2C3_SDA` über 1 kOhm nach `P3.3V_DELAYED`.
- ES8328-DVDD liegt direkt an `AUD_P3.3V`.
- ES8328-PVDD liegt direkt an `AUD_P3.3V`.
- `AUD_P3.3V` ist ein separat geschalteter Audio-Power-Domain.
- Die I2C-Pull-ups des Audiozweigs werden dagegen aus
  `P3.3V_DELAYED` versorgt.

Damit ist schaltungstechnisch ein Zustand möglich, in dem die
ES8328-Versorgung abgeschaltet ist, während seine I2C-Leitungen weiterhin
über Widerstände in Richtung einer aktiven 3,3-V-Versorgung gezogen werden.

Diese Topologie macht insbesondere folgende Kandidaten konkret:

1. M1 – Clamp- oder Bus-Loading-Effekt am unversorgten ES8328,
2. M2 – Rückspeisung des abgeschalteten Audio-Power-Domains,
3. M3 – Pull-up-/Power-Domain-Wechselwirkung,
4. M4 – zeitabhängiger Abschalttransient.

Die Topologie allein beweist keinen dieser Mechanismen.


## 4. Prüfung der ES8328-Dokumentationslage

Für Block 3.13E wurde versucht, die elektrische Eingrenzung durch
ES8328-Komponentendokumentation zu verbessern.

Direkte Archivierungsversuche über frei zugängliche Datenblattseiten
lieferten jedoch keinen ausreichend belastbaren neuen Primärbeleg:

- ein Datasheet4U-Ziel lieferte HTTP 404,
- ein AllDatasheet-Ziel war für den direkten Abruf mit HTTP 403 gesperrt.

Aus Suchergebnissen waren Angaben zu zulässigen Eingangsspannungen und
Versorgungsspannungen sichtbar. Diese Angaben reichen jedoch nicht aus, um
die interne I/O-Struktur des ES8328 zu bestimmen.

Insbesondere darf aus einer absoluten Eingangsspannungsgrenze allein nicht
geschlossen werden, dass SDA oder SCL bei abgeschaltetem DVDD zwingend über
eine bestimmte interne Schutzdiode rückspeisen.

Für Block 3.13E gilt deshalb weiterhin:

- eine konkrete interne Clamp-Struktur ist nicht belegt,
- ein konkreter Backpower-Strompfad ist nicht belegt,
- eine tatsächliche Pinspannung am abgeschalteten Codec ist nicht gemessen,
- ein tatsächlicher Strom durch R26A oder R27A ist nicht gemessen.

M1 und M2 bleiben deshalb plausible, aber nicht bewiesene Mechanismen.


## 5. Historische ES8328-Power-Management-Commits

Im historischen Repository `xobs/novena-linux` wurden zwei
ES8328-Power-Management-Änderungen genauer untersucht.


### 5.1 Commit 468b85c56f0942b518a8b03ac0b6293fe6df8408

Betreff:

`ASoC: imx-es8328: add power management support`

Autor:

Sean Cross

Datum:

2015-04-09

Der Commit aktiviert die normale ASoC-Power-Management-Integration des
i.MX6-ES8328-Machine-Drivers.

Er dokumentiert keine I2C3-Störung und keine Forderung, den
ES8328-Regulator dauerhaft eingeschaltet zu lassen.


### 5.2 Commit ff010f4fa0b44fceefd60a3449cf80af4379ca1b

Betreff:

`ASoC: es8328: remove power management`

Autor:

Sean Cross

Datum:

2015-05-21

Der entfernte Codec-spezifische Suspend-Pfad enthielt ausdrücklich:

- Abschalten des ES8328-Clocks,
- `regulator_bulk_disable()` für die ES8328-Versorgungen.

Der zugehörige Resume-Pfad aktivierte Clock und Versorgungen wieder und
synchronisierte anschließend den Regcache.

Die Commit-Nachricht begründet die Entfernung damit, dass die
ES8328-Definition ihre Power-Eigenschaften ausreichend beschreibt und ASoC
die erforderliche Suspend-/Resume-Behandlung bereits bereitstellt.

Dieser Commit belegt daher, dass historische Softwarepfade das Abschalten
der ES8328-Versorgungen ausdrücklich vorsahen.

Er belegt nicht, dass dieses Abschalten I2C3 beeinträchtigt, und er fordert
kein `regulator-always-on`.


## 6. Historische Device-Tree-Entwicklung

Mehrere Novena-DTS-Stände aus `xobs/novena-linux` wurden direkt
gegengeprüft.

Der frühe Novena-DTS-Stand aus Commit

`d0bbd1497c117cd9661bc98685e25a5db23506c8`

verwendet für `es8328-power`:

`regulator-boot-on`

und nicht `regulator-always-on`.

Auch der 2015er Stand

`70a8c03bd9eea54fcd2616302403b80c20729db9`

verwendet `regulator-boot-on`.

Der untersuchte HEAD von `xobs/novena-linux`

`d9d2e8b619f17e8394d62c08d60b4ea17154e9d6`

verwendet ebenfalls `regulator-boot-on`.

Damit ist belegt, dass die frühe beziehungsweise ältere Novena-Konfiguration
den ES8328-Regulator nicht generell als `regulator-always-on` deklarierte.


## 7. Methodische Korrektur früherer Git-Suchen

Zwei Zwischenergebnisse aus Block 3.13E dürfen nicht als Beleg für eine
ES8328-spezifische `regulator-always-on`-Historie verwendet werden.

Eine breite Suche mit

`git log -Sregulator-always-on`

findet Commits, sobald die Zeichenfolge irgendwo im jeweiligen
Device-Tree-Diff vorkommt. Daraus folgt nicht, dass die Änderung den
`es8328-power`-Knoten betrifft.

Ebenso war eine spätere Auswertung mit einem festen `grep`-Kontext um
`es8328-power` methodisch ungeeignet. Der Kontext konnte benachbarte
Regulator-Knoten einschließen und dadurch dort vorkommendes
`regulator-always-on` fälschlich dem ES8328-Regulator zuordnen.

Diese Treffer werden deshalb nicht als Beleg verwendet.

Die direkt extrahierten ES8328-Regulator-Knoten der geprüften Revisionen
haben Vorrang.


## 8. Wiederauffinden des historischen Novena-Primärbelegs

Während Block 3.13E entstand vorübergehend Unsicherheit über den bereits in
der Projektdokumentation verwendeten Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`.

Der Commit ließ sich zunächst weder in `xobs/novena-linux` noch im lokalen
Objektbestand von `novena-next/linux` finden.

Die anschließende Provenienzprüfung zeigte, dass diese Nichtfunde zwei
unterschiedliche Ursachen hatten:

1. `xobs/novena-linux` ist nicht das Repository, aus dem der dokumentierte
   Commit stammt.
2. Das lokale Repository `~/novena-linux` zeigt zwar auf
   `https://github.com/novena-next/linux.git`, ist aber ein Shallow Clone,
   der lokal ausschließlich den Branch `nvn_v5.7-rc2` enthält.

Der lokale `novena-next/linux`-Clone hatte zum Prüfzeitpunkt:

`HEAD=1fda06deecb61538ca3d07d256eb7c43d4e3432a`

und

`SHALLOW=true`.

Der gesuchte Commit, sein Parent und der historische Repository-HEAD waren
deshalb nicht im lokalen Objektbestand vorhanden.

Der lokale Nichtfund war somit kein Gegenbeleg gegen den historischen
Commit.


## 9. Archivierter Primärquellen-Auszug

Im Projekt war bereits ein reproduzierbarer Primärquellen-Auszug vorhanden:

`research/02-historical-software/extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt`

SHA-256:

`35bbe085ce6c199f15596e0edaa5106e10b1ef1db4a8a416491e0437b27e19a4`

Der Auszug dokumentiert folgende Repository-Provenienz:

Repository:

`https://github.com/novena-next/linux.git`

damaliger Repository-HEAD:

`18bf34080c4c3beb6699181986cc97dd712498fe`

Commit:

`e48619edadbde342d79655e73654f0b21fc5e20b`

Parent:

`16aae414f47116b568c837a131ba9d9250cf3b48`

Autor:

Jookia

Datum:

2020-04-01

Betreff:

`ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

Die archivierte Commit-Nachricht beschreibt ausdrücklich, dass Linux den
`es8328-power`-Regulator abschaltet, wenn der ES8328-Codec nicht verwendet
wird, und dass dies nach damaliger Beobachtung den I2C3-Bus beeinträchtigt
und dadurch unter anderem Bildschirm, EEPROM und Senoko stört.

Der archivierte DTS-Diff ersetzt für genau diesen Regulator:

`regulator-boot-on`

durch:

`regulator-always-on`.

Damit dokumentiert der historische Commit eine reale Novena-Beobachtung,
die unmittelbar den Audio-Regulator und I2C3 miteinander verknüpft.


## 10. Unabhängige erneute Provenienzprüfung

Die Repository-Provenienz des archivierten Auszugs wurde in Block 3.13E
erneut gegen den öffentlich erreichbaren Remote-Zustand geprüft.

`git ls-remote` für

`https://github.com/novena-next/linux.git`

lieferte:

`18bf34080c4c3beb6699181986cc97dd712498fe refs/heads/master`

Der gleiche Commit wurde außerdem als Default-HEAD des Remote-Repositorys
gemeldet.

Damit stimmt der heutige Remote-`master` exakt mit dem im archivierten
Primärquellen-Auszug dokumentierten Repository-HEAD überein.

Zusätzlich wurde der Commit erneut über die öffentliche
`novena-next/linux`-Commitquelle geprüft.

Commit-ID, Autor, Parent, Betreff, Commit-Nachricht und DTS-Änderung stimmen
mit dem archivierten Primärquellen-Auszug überein.

Der historische Commit `e48619edadbde342d79655e73654f0b21fc5e20b` wird
deshalb wieder als verifizierter historischer Primärbeleg behandelt.


## 11. Fehlgeschlagener zusätzlicher Shallow-History-Versuch

Zur zusätzlichen lokalen Reproduktion wurde vorübergehend ein separates
Repository unter `/tmp` angelegt.

Ein Fetch mit `--depth=300` enthielt den gesuchten Commit noch nicht.

Eine anschließende Vertiefung um weitere 700 Ebenen enthielt ihn ebenfalls
noch nicht.

Wegen der stark verzweigten Linux-Kernel-Historie führte die
Reachability-Auswertung dabei bereits zu sehr großen Commitmengen.

Diese Methode wurde deshalb beendet und nicht weiter vertieft.

Das temporäre Repository wurde anschließend entfernt.

Das eigentliche Projekt-Repository sowie der vorhandene
`~/novena-linux`-Clone wurden dadurch nicht verändert.


## 12. Quellenübergreifende Bewertung

Nach Block 3.13E liegen zwei voneinander unabhängige Evidenzlinien vor.


### 12.1 Historische Evidenz

Der verifizierte Novena-Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

dokumentiert eine damalige reale Hardwarebeobachtung:

Das Abschalten von `es8328-power` schien I2C3 zu beeinträchtigen.

Als Board-Konfigurationsänderung wurde deshalb
`regulator-always-on` verwendet.


### 12.2 Aktuelle experimentelle Evidenz

Im aktuellen Linux-6.18.49-Teststand wurde unabhängig davon beobachtet:

- Baseline-Cold-FAIL mit Abschaltung von `es8328-power`,
- später untersuchte I2C3-Fehlersignatur,
- kontrollierte H3-1R-Änderung ausschließlich am Regulator,
- anschließend fünf von fünf erfolgreiche echte POR-Cold-Boots,
- kein Auftreten der untersuchten `93/80`-/`arbitration lost`-Signatur.

Die historische Quelle und die aktuelle Versuchsserie stützen damit
unabhängig voneinander dieselbe boardspezifische Gegenmaßnahme.


## 13. Aussagegrenze

Die Übereinstimmung ist stark, darf aber nicht überinterpretiert werden.

Belegt ist:

- Auf historischer Novena-Hardware wurde eine
  ES8328-Power-/I2C3-Wechselwirkung beobachtet.
- Der historische Fix verwendete `regulator-always-on`.
- Der aktuelle Baseline-Test schaltet denselben Audio-Regulator ab.
- Die aktuelle kontrollierte H3-1R-Serie verwendet ebenfalls
  `regulator-always-on`.
- Unter H3-1R bestanden fünf von fünf echte POR-Cold-Boots ohne die
  untersuchte I2C3-Fehlersignatur.

Nicht bewiesen ist:

- dass das historische Fehlerereignis exakt dem heutigen
  `A=81/80 -> M0=93/80` entspricht,
- dass M1, M2, M3 oder M4 der konkrete elektrische Mechanismus ist,
- dass eine interne ES8328-Schutzdiode beteiligt ist,
- dass tatsächlich Backpower in `AUD_P3.3V` auftritt,
- welcher Strom während des kritischen Zustands durch R26A oder R27A fließt,
- welche Spannungsverläufe an den beteiligten Netzen auftreten.

Ohne elektrische Messung lassen sich M1 bis M4 aus den derzeit verfügbaren
Quellen nicht eindeutig voneinander trennen.


## 14. Konsequenz für die Root-Cause-Untersuchung

Die dauerhafte Board-Konfiguration

`es8328-power = regulator-always-on`

bleibt bestehen.

Sie wird durch zwei unabhängige Evidenzlinien gestützt:

1. die aktuelle kontrollierte H3-1R-POR-Serie,
2. den erneut verifizierten historischen Novena-Primärbeleg.

Es wird deshalb nicht:

- zur fehlerhaften Baseline-Konfiguration zurückgekehrt,
- ein zusätzlicher IT6251-Retry eingebaut,
- eine zusätzliche Wartezeit als Ersatzfix eingeführt,
- STMPE811 erneut isoliert,
- die bereits ausgeschlossene Single-Master-Hypothese erneut getestet,
- Kernel-Patch `0007` begonnen.

Die weitere Untersuchung darf sich auf die noch offene Frage nach dem
elektrischen Mechanismus konzentrieren.


## 15. Status der Root-Cause-Kandidaten nach Block 3.13E

M1 – Clamp-/Bus-Loading-Effekt am unversorgten ES8328:

Weiterhin hohe Priorität und mit der bekannten Topologie vereinbar.
Nicht bewiesen.

M2 – Rückspeisung des Audio-Power-Domains über I2C:

Weiterhin hohe Priorität und mit der bekannten Topologie vereinbar.
Nicht bewiesen.

M3 – Pull-up-/Power-Domain-Wechselwirkung:

Weiterhin hohe Priorität. Durch die getrennten Versorgungen von
I2C-Pull-ups und Codec besonders konkret.
Nicht bewiesen.

M4 – Abschalttransient:

Weiterhin offen. Die zeitliche Trennung zwischen Regulator-Abschaltung und
später sichtbarer I2C3-Fehlersignatur schließt einen dadurch erzeugten
persistierenden oder nachwirkenden Hardwarezustand nicht aus.
Nicht bewiesen.

M5 – primärer IT6251-Power-/Reset-/POR-Fehler:

Als alleinige Primärursache gegenüber M1 bis M4 weniger naheliegend, da die
Audio-Regulator-Intervention das untersuchte Fehlerbild beseitigte.
Als nachgelagerter Zustand weiterhin möglich.

M6 – primärer U-Boot-I2C3-Handoff-Fehler:

Als alleinige Primärursache gegenüber der nachgewiesenen
Audio-Regulator-Abhängigkeit weniger naheliegend.
Ein beitragender versteckter Ausgangszustand ist nicht vollständig
ausgeschlossen.

M7 – primärer interner i.MX6Q-Controller-/Clock-/Pinmux-Zustand:

Weiterhin nicht vollständig ausgeschlossen, aber gegenüber der
Audio-Power-Spur niedriger priorisiert.

M8 – primäre Linux-6.18-I2C-START-/IAL-Regression:

Durch die historische unabhängige ES8328-/I2C3-Beobachtung und den
aktuellen H3-1R-Erfolg weiter geschwächt.

M9 – STMPE811:

Für die untersuchte Fehlerklasse experimentell ausgeschlossen.

M10 – Single-Master-Hypothese:

Für das untersuchte IAL-Ereignis experimentell ausgeschlossen.

M11 – altes beziehungsweise stehengebliebenes IAL-Bit:

Durch Patch 0006 stark ausgeschlossen.

M12 – separates I2C0-`arbitration lost`:

Bleibt ein eigenständiger offener Befund und wird nicht mit der
I2C3-/ES8328-Ursache gleichgesetzt.


## 16. Abschluss von Block 3.13E

Block 3.13E liefert keinen direkten elektrischen Nachweis für M1 bis M4.

Er beseitigt jedoch die zwischenzeitliche Unsicherheit über die wichtigste
historische Softwarequelle.

Der Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

ist als historischer Novena-Primärbeleg verifiziert.

Damit stehen die historische Beobachtung und die aktuelle kontrollierte
Versuchsserie konsistent nebeneinander, ohne dass aus ihrer Übereinstimmung
ein nicht gemessener elektrischer Mechanismus abgeleitet wird.

Da keine Hardwaremessungen durchgeführt werden, ist vor weiteren
Experimenten zunächst zu prüfen, ob vorhandene Schaltplan-, Layout-,
Komponenten- oder historische Entwicklungsquellen M1 bis M4 noch weiter
voneinander trennen können.

Bleibt auch diese Quellenlage ohne zusätzliche diskriminierende Evidenz,
muss die Grenze ausdrücklich dokumentiert werden:

Die boardspezifische Gegenmaßnahme ist experimentell und historisch stark
abgesichert, während der genaue elektrische Mechanismus ohne Messung offen
bleibt.
