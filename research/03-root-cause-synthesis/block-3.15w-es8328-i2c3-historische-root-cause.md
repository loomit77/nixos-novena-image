# Block 3.15W – Historische Root-Cause-Synthese ES8328/I2C3

## 1. Zweck und Status

Dieser Bericht führt die bis Block 3.15W gewonnenen historischen, elektrischen und experimentellen Befunde zur I2C3-Kaltstartstörung der Novena zusammen.

Er ersetzt nicht die Einzelberichte der vorausgehenden Untersuchungsblöcke. Ziel ist die kontrollierte Beantwortung der Frage, ob die funktionale Root Cause des untersuchten Fehlers hinreichend bestimmt ist oder ob weitere elektrische Messungen erforderlich sind.

Ergebnis dieses Blocks:

- Die funktionale Root Cause ist hinreichend bestimmt.
- Das Abschalten der ES8328-Audio-Power-Domain ist kausal mit der untersuchten I2C3-Kaltstartstörung verbunden.
- Historische Novena-Primärquellen dokumentieren eine elektrische Rückspeisung beziehungsweise Leakage-Wechselwirkung des abgeschalteten Audio-Domains über I2C und mit den I2C-Pull-ups.
- `regulator-always-on` für `es8328-power` bleibt eine dauerhafte Novena-Board-Anforderung.
- Eine weitere analoge Detailmessung ist für diese funktionale Board-Entscheidung nicht erforderlich.
- Der exakte interne Leckstrompfad im ES8328 und der konkrete analoge Spannungs- und Stromverlauf unseres PVT2-Boards während des Fehlers bleiben nicht direkt gemessen.

## 2. Methodische Trennung der Evidenz

Für die Schlussfolgerung werden drei Ebenen ausdrücklich getrennt:

1. historisch direkt dokumentierte beziehungsweise gemessene Befunde;
2. im aktuellen Projekt direkt experimentell beobachtete Befunde;
3. aus der Übereinstimmung dieser unabhängigen Befunde abgeleitete Schlussfolgerungen.

Diese Trennung ist notwendig, damit historische EVT-Messwerte nicht fälschlich als Messwerte unseres heutigen PVT2-Boards behandelt werden.

## 3. Historisch direkt belegte Hardwarebefunde

### 3.1 Novena Issue Log: Leakage im abgeschalteten Audio-Domain

Der historische Kosagi-`Novena Issue Log` dokumentiert einen zu schwachen Audio-Power-off-Pulldown.

Für den damaligen Zustand mit R21A = 100 Ohm wurden ungefähr 10 mA über Leckpfade beobachtet. Der Eintrag nennt als Folge ungefähr 1 V auf der Audio-Versorgung.

Als unmittelbare Hardwareänderung wurde R21A auf 10 Ohm reduziert. Der Issue Log hält fest, dass diese Änderung zu diesem Zeitpunkt nur auf einem Board durchgeführt worden war.

Diese ungefähr 10 mA und ungefähr 1 V sind historische EVT-Messwerte. Sie wurden im aktuellen Projekt nicht auf dem vorhandenen PVT2-Board nachgemessen.

### 3.2 EVT-zu-DVT-ECO8: Leakage über I2C

Die historische Kosagi-Seite `Novena EVT to DVT changes` beschreibt unter ECO8 (`Audio chip sucks (power)`) die Ursache genauer.

Die Novena-Entwickler dokumentieren dort, dass der Audiochip während des Power-down Leistung über den I2C-Bus zurückleckt. Der Power-off-Pulldown müsse verstärkt werden, um den Chip vollständig zurückzusetzen und gegen die I2C-Pull-ups anzukämpfen.

Die ECO-Tabelle dokumentiert:

- EVT: R21A = 100 Ohm;
- auf EVT1A wurde 10 Ohm eingesetzt;
- für DVT wurde R21A = 20 Ohm vorgesehen;
- 20 Ohm wurde als voraussichtlich ausreichend bewertet.

Damit bilden die Werte 100 Ohm, 10 Ohm und 20 Ohm keinen belegten Widerspruch. Sie beschreiben aufeinanderfolgende Entwicklungszustände beziehungsweise einen Versuch und den anschließend vorgesehenen DVT-Wert.

### 3.3 ECO20 als historische Querverifikation

ECO20 derselben EVT-zu-DVT-Dokumentation greift die Erfahrung mit dem Audio-Codec erneut auf. Dort wird ein vorhandener 330-Ohm-Widerstand bei einer anderen Abschaltfunktion aufgrund der Erfahrung mit erheblichem Leakage als wahrscheinlich zu schwach bewertet.

Dieser Eintrag beweist keinen zusätzlichen ES8328-Strompfad. Er zeigt jedoch unabhängig davon, dass die Entwickler die zuvor beobachtete Audio-Leakage als reale elektrische Hardwareerfahrung behandelten.

## 4. PVT2-Hardwaretopologie

Die im Projekt untersuchten originalen PVT2-Boardquellen sind mit der historischen Beschreibung konsistent.

Relevante Elemente sind:

- R26A = 330 Ohm zwischen I2C3_SCL und AUD_I2C3_SCL;
- R27A = 330 Ohm zwischen I2C3_SDA und AUD_I2C3_SDA;
- R10B = 1 kOhm als Pull-up von AUD_I2C3_SCL nach P3.3V_DELAYED;
- R11B = 1 kOhm als Pull-up von AUD_I2C3_SDA nach P3.3V_DELAYED;
- eine separat schaltbare Audio-Versorgung AUD_P3.3V;
- Q11A als High-Side-Schaltelement der Audio-Versorgung;
- Q10A im AUD_PWRON-Steuerpfad;
- Q12A mit R21A als aktivem Power-off-Pulldown;
- R21A = 20 Ohm im untersuchten PVT2-Stand.

Damit existiert auf PVT2 weiterhin eine reale Power-Domain-Grenze: Die I2C-Signalseite des Audio-Codecs besitzt Pull-ups zur verzögerten 3,3-V-Versorgung, während die Audio-Versorgung selbst abgeschaltet werden kann.

Die PVT2-Topologie beweist allein nicht, welcher interne Halbleiterpfad im ES8328 den historischen Leckstrom verursacht.

## 5. Historischer Linux-Befund von 2020

Der verifizierte Novena-next-Linux-Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

trägt den Titel:

`ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

Der Commit änderte die Novena-DTS-Konfiguration von `regulator-boot-on` auf `regulator-always-on`.

Die Commit-Beschreibung hält fest, dass Linux die ES8328-Versorgung abschaltete, wenn der Codec nicht benutzt wurde, und dass dies offenbar I2C3 beeinträchtigte beziehungsweise Funktionen wie Bildschirm, EEPROM und Senoko störte.

Dieser Befund entstand Jahre nach den EVT/DVT-Hardware-ECOs und zeigt, dass die praktische Wechselwirkung zwischen ES8328-Power und I2C3 trotz der Hardwareänderungen weiterhin softwareseitig relevant war.

Der Commit ist eine historische Novena-Primärquelle. Seine Provenienz wurde in Block 3.15W gegen das Repository `novena-next/linux` verifiziert.

## 6. Direkt im aktuellen Projekt beobachtete Befunde

### 6.1 Cold-Fail vor H3-1R

Im untersuchten POR-Cold-Fail wurde `es8328-power: disabling` bei ungefähr 33,761742 s protokolliert.

Der erste relevante I2C3-Arbitration-Lost-Befund trat bei ungefähr 44,478167 s auf; der zugehörige START-Fehler folgte unmittelbar danach.

Der zeitliche Abstand zwischen dem Abschalten der Audio-Versorgung und dem ersten untersuchten I2C3-Fehler beträgt damit ungefähr 10,716 s.

Dieser zeitliche Zusammenhang allein beweist noch keine Kausalität.

### 6.2 H3-1R: isolierte Intervention

Für H3-1R wurde ausschließlich `regulator-always-on` für `es8328-power` hinzugefügt.

Die vorregistrierte POR-Serie ergab 5/5 PASS.

In diesen fünf Cold-Boots trat weder das zuvor charakteristische unmittelbare M0=`93/80`-Muster noch der untersuchte I2C3-Arbitration-Lost-Fehler auf.

Damit ist die Power-Policy der ES8328-Audio-Domain für den untersuchten Fehler experimentell kausal relevant.

### 6.3 Block 3.14B: zeitliche Lokalisierung des Controllerfehlers

Block 3.14B lokalisierte den primären Controllerfehler auf das enge Fenster zwischen dem Schreiben von MSTA und der ersten unmittelbar folgenden I2SR-Abfrage M0.

Cold-Fail:

- vor dem MSTA-Schreiben: A=`81/80`;
- unmittelbar danach: M0=`93/80`;
- damit ist IAL bei der ersten Messmöglichkeit bereits gesetzt und MSTA bereits wieder gelöscht.

PASS:

- vor dem MSTA-Schreiben: A=`81/80`;
- unmittelbar danach: M0=`81/a0`;
- anschließend normale START-Fortsetzung.

Zwischen MSTA-MMIO-Schreibzugriff und M0 befindet sich in der Instrumentierung keine weitere Softwareoperation, die MSTA löschen könnte.

### 6.4 Block 3.14C: i.MX-Arbitration-Lost-Semantik

Die historische i.MX-Treiberanalyse zeigte, dass ein fehlgeschlagener Master-Erwerb vom Controller als Arbitration Lost signalisiert werden kann und dabei MSTA ohne einen softwareseitigen STOP wieder gelöscht werden kann.

Der historische Linux/Freescale-Commit `639a26cf0771cb5a4d61a0f7777882cbda989753` dokumentiert die entsprechende Arbitration-Lost-Behandlung.

Damit sind MSTA=0 und IAL=1 unmittelbar nach dem START-Versuch mit dokumentiertem i.MX-Hardwareverhalten vereinbar.

Das später sichtbare `-EAGAIN` beziehungsweise `-11` ist eine nachgelagerte Softwarefolge und nicht der Ursprung des Fehlers.

## 7. Gegenhypothese: zweiter physischer I2C3-Master

Ein tatsächlich konkurrierender zweiter physischer Master auf I2C3 wurde im Projekt nicht nachgewiesen.

Die bereits durchgeführte isolierte Untersuchung der `single-master`-Konfiguration beseitigte den beobachteten Hardware-IAL nicht.

Der betreffende Test ist damit bereits durchgeführt und soll nicht lediglich zur Wiederbestätigung wiederholt werden.

`IAL` darf in diesem Fall nicht automatisch als Beweis für einen zweiten physischen Master interpretiert werden.

## 8. Zusammenführung der unabhängigen Evidenz

Die einzelnen Befunde stammen aus unterschiedlichen Zeiten und Quellen:

1. EVT-Hardwaremessung: relevanter Leakage-Zustand der abgeschalteten Audio-Versorgung;
2. EVT-zu-DVT-ECO: ausdrückliche Zuordnung des Problems zu Rückspeisung über I2C und zur Wechselwirkung mit den I2C-Pull-ups;
3. PVT2-Schaltung: weiterhin vorhandene getrennte Audio-Power-Domain, I2C-Kopplung, Pull-ups und verstärkter Power-off-Pulldown;
4. Linux 2020: Abschalten von `es8328-power` beeinträchtigt weiterhin I2C3; `regulator-always-on` wird eingeführt;
5. eigener Cold-Fail 2026: Fehler tritt nach dem Abschalten von `es8328-power` auf;
6. eigene isolierte Intervention H3-1R: `regulator-always-on` beseitigt den untersuchten Fehler in 5/5 POR-Läufen;
7. 3.14B/3.14C: der Controllerfehler entsteht unmittelbar beim Hardware-Master-Erwerb nach MSTA und ist mit dokumentierter i.MX-IAL-Semantik vereinbar.

Keine einzelne dieser Beobachtungen allein bestimmt den vollständigen analogen Mechanismus des heutigen Fehlers.

In ihrer Kombination bilden sie jedoch eine konsistente, voneinander weitgehend unabhängige Evidenzkette für eine elektrische Wechselwirkung zwischen der abgeschalteten ES8328-Audio-Power-Domain und I2C3 als funktionale Root Cause.

## 9. Neubewertung der Mechanismuskandidaten M1 bis M4

### M1 – Clamp beziehungsweise Bus-Loading

Ein belastender elektrischer Effekt auf den I2C-Bus ist mit den historischen Leakage-Befunden und der PVT2-Topologie vereinbar und wird dadurch stark gestützt.

Nicht bestimmt ist ein konkreter interner Clamp-, Schutzdioden- oder sonstiger Halbleiterpfad innerhalb des ES8328. Ein solcher Detailpfad darf ohne Herstellerquelle oder direkte elektrische Messung nicht als bewiesen bezeichnet werden.

### M2 – Backfeeding der abgeschalteten Audio-Domain

Die grundsätzliche Rückspeisungs-/Leakage-Wechselwirkung über I2C ist für historische Novena-Hardware direkt dokumentiert.

M2 ist damit nicht mehr lediglich eine aus dem aktuellen Fehlerbild abgeleitete Hypothese.

Nicht direkt gemessen wurde, welcher Strom während unseres konkreten PVT2-Cold-Fails über welchen internen ES8328-Pfad fließt.

### M3 – Pull-up-/Power-Domain-Wechselwirkung

Die historische ECO8-Dokumentation nennt ausdrücklich die I2C-Pull-ups als Gegenlast des Power-off-Pulldowns.

Die PVT2-Schaltung besitzt weiterhin die dafür relevante getrennte Versorgungs- und Pull-up-Topologie.

Die grundsätzliche M3-Wechselwirkung ist daher historisch direkt belegt und auf PVT2 topologisch weiterhin relevant.

### M4 – Power-down-Transient

Ein besonderer transienter Verlauf beim Abschalten ist nicht ausgeschlossen.

Ein solcher zusätzlicher Mechanismus ist jedoch nicht erforderlich, um die funktionale Root Cause und die notwendige Board-Konfiguration zu erklären.

M4 bleibt deshalb als mögliche analoge Detailfrage offen, ist aber kein Blocker für den Abschluss.

## 10. Fortschreibung der früheren Bewertung

Die früheren Blöcke 3.13E/3.13F und frühe Teile von 3.15 behandelten M1 bis M4 mangels direkter historischer beziehungsweise elektrischer Evidenz vorsichtig als offene Mechanismuskandidaten.

Diese damalige Bewertung war auf Basis des damaligen Wissensstands korrekt.

Block 3.15W erweitert die Evidenzbasis durch wiederentdeckte und archivierte Kosagi-Primärquellen. Dadurch werden insbesondere M2 und M3 wesentlich stärker bestimmt.

Die älteren Aussagen werden deshalb nicht als Fehler gelöscht, sondern als früherer Wissensstand betrachtet und durch diesen Bericht fortgeschrieben.

## 11. Funktionale Root Cause

Die Novena besitzt eine historisch dokumentierte elektrische Wechselwirkung zwischen der abschaltbaren ES8328-Audio-Power-Domain und I2C3.

Bereits auf EVT-Hardware wurde beim Abschalten des Audio-Domains ein relevanter Leckstromzustand beobachtet. Die EVT-zu-DVT-Dokumentation ordnet die Wechselwirkung ausdrücklich einer Rückspeisung über I2C zu und nennt die I2C-Pull-ups als Gegenlast des Power-off-Pulldowns.

Die spätere PVT2-Hardware behält einen verstärkten aktiven Pulldown bei. Dennoch dokumentiert der Novena-next-Linux-Commit von 2020, dass das Abschalten der ES8328-Versorgung weiterhin I2C3-Funktionen beeinträchtigte und führt deshalb `regulator-always-on` ein.

Unsere unabhängige H3-1R-Intervention bestätigt die kausale Abhängigkeit für den untersuchten heutigen Kaltstartfehler: Das isolierte Verhindern der ES8328-Abschaltung beseitigte die Störung in 5/5 POR-Läufen.

Block 3.14B/3.14C lokalisiert die sichtbare Controllerreaktion auf den Hardware-Master-Erwerb unmittelbar nach dem Setzen von MSTA und erklärt IAL/MSTA=0 als mit dokumentiertem i.MX-Hardwareverhalten vereinbar.

Damit gilt die elektrische Wechselwirkung der abgeschalteten ES8328-Audio-Power-Domain mit I2C3 als funktionale Root Cause der untersuchten Kaltstartstörung.

## 12. Wissenschaftliche Grenze

Nicht bestimmt wurden:

- der exakte interne Leckstrompfad im ES8328;
- ein möglicher konkreter Schutzdioden- oder Clamp-Pfad;
- der exakte analoge Spannungsverlauf von AUD_P3.3V während unseres PVT2-Cold-Fails;
- die exakten Ströme über R26A/R27A während dieses Fehlers;
- die exakten analogen SCL/SDA-Pegel auf beiden Seiten der 330-Ohm-Widerstände während des Fehlerereignisses.

Die historischen Werte von ungefähr 10 mA und ungefähr 1 V gehören zum dokumentierten EVT-Zustand mit R21A = 100 Ohm und dürfen nicht als Messwerte unseres PVT2-Boards ausgegeben werden.

Diese offenen analogen Detailfragen ändern die funktionale Board-Entscheidung nicht.

## 13. Konsequenz für die Board-Konfiguration

Für Novena bleibt `es8328-power` dauerhaft `regulator-always-on`.

Diese Einstellung ist nicht lediglich ein Workaround für einen einzelnen modernen Kernel. Sie ist durch historische Hardwarebefunde, historische Novena-Softwareerfahrung und die unabhängige aktuelle POR-Testserie begründet.

Die bereits getroffene H3-1R-Entscheidung wird dadurch bestätigt und stärker begründet.

## 14. Konsequenz für weitere Tests

Für den funktionalen Root-Cause-Abschluss sind derzeit nicht erforderlich:

- Patch 0007 oder noch dichteres START-Sampling;
- Wiederholung der bereits abgeschlossenen A/B/C/D/E- oder M0..M7-Tests;
- erneute `single-master`-Versuche zur bloßen Wiederbestätigung;
- weitere POR-Serien nur zur erneuten Bestätigung von H3-1R;
- PCB-/Via-Mapping für SCL/SDA;
- Anschaffung eines Oszilloskops oder Logic Analyzers;
- direkte analoge Messung von AUD_P3.3V oder den I2C3-Leitungen.

Solche Untersuchungen können später als freiwillige elektrische Charakterisierung durchgeführt werden, falls der interne Mechanismus wissenschaftlich weiter aufgelöst werden soll. Sie sind keine Voraussetzung für die funktionale Novena-Konfiguration.

## 15. Abschlussentscheidung Block 3.15W

Block 3.15W schließt die funktionale Root-Cause-Untersuchung der hier untersuchten ES8328/I2C3-Kaltstartstörung ab.

Festgehalten wird:

1. ES8328 `regulator-always-on` bleibt permanente Novena-Board-Konfiguration.
2. Die Audio-Power-Domain/I2C3-Wechselwirkung ist funktional kausal ausreichend belegt.
3. Historische Kosagi-Quellen belegen Leakage beziehungsweise Rückspeisung über I2C und die Pull-up-Wechselwirkung auf Novena-Hardware.
4. Der genaue interne ES8328-Strompfad bleibt offen und wird nicht spekulativ festgelegt.
5. Weitere elektrische Messungen sind optional und nicht erforderlich, um die Board-Konfiguration zu rechtfertigen.
6. Bereits abgeschlossene Tests werden ohne neue falsifizierbare Fragestellung nicht wiederholt.

## 16. Archivierte Primärquellen und Referenzen

### Kosagi-Hardwarehistorie

- `research/02-historical-software/sources/kosagi-hardware-history/Novena_EVT_to_DVT_changes.html`
  - SHA-256: `98626b8de199d68449c86225994802ed302ea03f7335153199bb12e5954bcb42`
- `research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.html`
  - SHA-256: `02a2314f7b3548d625b75719815e9d05c5f04012cb5173702abb8a253920d6d4`
- `research/02-historical-software/sources/kosagi-hardware-history/README.md`

### Historischer Linux-Commit

- Repository: `novena-next/linux`
- Commit: `e48619edadbde342d79655e73654f0b21fc5e20b`
- Titel: `ARM: dts: imx6q-novena: Always enable the es8328-power regulator`

### i.MX-Arbitration-Lost-Referenz

- Commit: `639a26cf0771cb5a4d61a0f7777882cbda989753`
- Titel: `i2c: imx: Add arbitration lost check`

### Projektinterne Vorarbeiten

- Block 3.13: elektrische und historische Eingrenzung
- H3-1R: isolierte ES8328-Always-on-POR-Intervention
- Block 3.14B: zeitliche Eingrenzung MSTA bis M0
- Block 3.14C: i.MX-START-/Arbitration-Lost-Semantik
- Block 3.15A: Falsifizierbarkeit der elektrischen Mechanismuskandidaten
- Block 3.15W: historische Quellenrecherche und Root-Cause-Synthese
