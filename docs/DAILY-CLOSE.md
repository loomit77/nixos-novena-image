# Novena – Tagesabschlussverfahren

Stand: 2026-10-12

Status: Technischer Probelauf erfolgreich; fachliche Gesamtqualifikation noch offen.

## 1. Zweck

Dieses Verfahren dient dazu, einen Arbeitstag im Projekt
„Novena – universelles NixOS SD-Image“ nachvollziehbar
abzuschließen und die spätere Fortsetzung vorzubereiten.

Es ersetzt keine technischen Prüfungen und verändert
keine bereits qualifizierten historischen Ergebnisse.

Die maßgeblichen Originalprotokolle bleiben die Grundlage
für technische Aussagen und Abnahmeentscheidungen.

## 2. Verbindliche Grundregeln

1. Vorhandene Projektdokumentation und Originalprotokolle
   zuerst auswerten.
2. Keine abgeschlossenen technischen Untersuchungen
   ohne konkreten Anlass wiederholen.
3. Technische Shell-Blöcke ausschließlich über den
   vorhandenen Runner und Protokolltransport ausführen.
4. Jeder ausgeführte Block benötigt ein vollständiges
   TXT-Protokoll und einen SHA256-Nachweis.
5. Blockergebnis, Protokollintegrität und Transportergebnis
   getrennt bewerten.
6. Historische FAIL-Ergebnisse bleiben FAIL.
7. Neue Projektdateiänderungen benötigen eine
   ausdrückliche Freigabe.
8. Git-Commits und Projekt-Pushes benötigen eine
   gesonderte ausdrückliche Freigabe.
9. Keine Builds, Hardwaretests oder Medien-Schreibzugriffe
   ohne eigene Freigabe.
10. Vorhandene Intent-to-add-Einträge und der Git-Index
    dürfen nicht versehentlich verändert werden.

## 3. Zwei Abschlussarten

### 3.1 Regulärer Tagesabschluss

Standardverfahren für einen normalen Arbeitstag.

Erfasst:

- Datum und betroffenen Projektbereich.
- Zuletzt bestätigten Projektcheckpoint.
- Tatsächlich ausgeführte Arbeitsblöcke.
- Erfolgreiche und fehlgeschlagene Ergebnisse.
- Noch offene Aufgaben und Entscheidungen.
- Relevante neue Quellen oder Erkenntnisse.
- Aktuellen Git-Zustand, soweit erforderlich.
- Status vorhandener Sicherungsnachweise.
- Den nächsten konkreten Arbeitsschritt.

Nicht Bestandteil eines normalen Tagesabschlusses:

- Vollständige Wiederholung der DDR-/SPL-Untersuchung.
- Vollständige Neubewertung der Dateiinventur.
- Unnötige erneute Hash-Prüfungen historischer Daten.
- Automatische Backups oder Restore-Tests.
- Automatische Git-Commits oder Pushes.

### 3.2 Erweiterter Meilensteinabschluss

Wird nur bei begründetem Bedarf durchgeführt,
beispielsweise nach:

- neuen qualifizierten technischen Ergebnissen,
- größeren Änderungen am Boot- oder Imagepfad,
- wichtigen Git- oder Dokumentationsmeilensteinen,
- neuen Inventur- oder Backup-Checkpoints,
- relevanten Änderungen der Sicherheitsregeln.

Zusätzlich können erforderlich sein:

- gezielter Quellen- und Evidenzabgleich,
- Projektindex-Fortschreibung,
- Inventur- und Datenbanknachweise,
- Backup- und Integritätsnachweise,
- Prüfung der Wiederherstellbarkeit,
- dokumentierte Freigabe für Git-Synchronisierung.

Der Umfang wird vor der Durchführung festgelegt.

## 4. Ablauf des regulären Tagesabschlusses

### Schritt A – Geltungsbereich festlegen

Festhalten:

- Datum.
- Projekt und gegebenenfalls Teilprojekt.
- Beginn und Ende des betrachteten Zeitraums.
- Letzten qualifizierten Ausgangscheckpoint.
- Welche Arbeiten tatsächlich zum Abschluss gehören.

Parallele Projektchats und eigenständige Teilprojekte
dürfen nicht stillschweigend zusammengeführt werden.

### Schritt B – Originalprotokolle auswerten

Für jeden relevanten Block erfassen:

- Blockkennung.
- Zeitpunkt.
- Originalprotokoll und SHA256-Sidecar.
- Technisches Ergebnis PASS, FAIL oder UNGEKLÄRT.
- Protokollvollständigkeit und Integritätsstatus.
- Transportstatus.
- Tatsächlich vorgenommene Änderungen.
- Noch offene Folgefragen.

Ein erfolgreicher GitHub-Upload bedeutet nicht,
dass der technische Block erfolgreich war.

Ein fehlgeschlagener Block darf nicht durch einen
späteren erfolgreichen Block historisch umgedeutet werden.

### Schritt C – Fortschritt und offene Punkte trennen

Ergebnisse in drei Gruppen einordnen:

1. Abgeschlossen und anhand von Nachweisen bestätigt.
2. Fehlgeschlagen oder nur teilweise erfolgreich.
3. Offen, ungeprüft oder für später zurückgestellt.

Unbelegte Annahmen dürfen nicht als bestätigte
technische Tatsachen dargestellt werden.

### Schritt D – Projektdokumentation abgleichen

Bei fachlichem Bedarf prüfen:

- TODO.md
- docs/PROJECT-STATE.md
- docs/TEST-LOG.md
- docs/DECISIONS.md
- docs/project-index/
- docs/HANDOVER-LATEST.md

Historische Abschnitte erhalten.

Neuere datierte Nachträge müssen ältere Checkpoints
eindeutig einordnen, nicht stillschweigend ersetzen.

Nicht jede Datei muss täglich geändert werden.

Änderungen erst nach ausdrücklicher Freigabe durchführen.

### Schritt E – Git-Zustand erfassen

Soweit für den Abschluss erforderlich:

- Branch und HEAD.
- Git-Indexzustand.
- Beabsichtigte Nutzeränderungen.
- Relevante Worktree-Änderungen.
- Codeberg- und GitHub-Checkpoint.

Bereits qualifizierte Prüfungen dürfen wiederverwendet
werden, solange keine neue Änderung ihre Aussagekraft
aufgehoben hat.

Den ursprünglichen Git-Index nicht bereinigen,
zurücksetzen oder unbeabsichtigt neu schreiben.

Insbesondere Intent-to-add-Einträge erhalten.

Remote-Synchronisierung nur nach gesonderter Freigabe.

### Schritt F – Backupstatus einordnen

Vorhandene Backup- und Integritätsnachweise
dokumentieren, ohne Sicherungsmedien automatisch
zu beschreiben oder zu verändern.

Für den vorhandenen R6-HDD-Checkpoint gilt:

- Qualifizierter Sicherungscheckpoint vorhanden.
- Vollständige Wiederherstellungsprobe noch offen.
- Keine erneute HDD-Prüfung ohne konkreten Bedarf
  und entsprechende Freigabe.

### Schritt G – Übergabe vorbereiten

Die Übergabe soll enthalten:

- Letzten bestätigten Projektcheckpoint.
- Wichtige neue Ergebnisse.
- Fehlgeschlagene oder unvollständige Arbeiten.
- Relevante offene Aufgaben.
- Bestehende Sicherheits- und Freigabegrenzen.
- Nächsten empfohlenen Arbeitsblock.
- Verweise auf Originalprotokolle.

Die aktuelle Übergabe befindet sich unter:

docs/HANDOVER-LATEST.md

Der Chat-Starter befindet sich unter:

docs/CHAT-STARTER.md

Datierte Übergaben können nach Freigabe unter
docs/handovers/ archiviert werden.

## 5. Abschlussstatus

Ein Tagesabschluss erhält einen der folgenden Zustände:

### PASS

Alle für diesen Abschluss ausdrücklich festgelegten
Pflichtnachweise sind vorhanden und konsistent.

PASS bedeutet nicht, dass alle offenen Projektaufgaben
erledigt sind.

### PARTIAL

Der Abschluss ist nachvollziehbar, aber mindestens
ein vorgesehener Teil fehlt oder bleibt offen.

### FAIL

Ein wesentlicher Nachweis widerspricht den Angaben,
eine notwendige Integritätsprüfung schlägt fehl oder
die definierten Sicherheitsgrenzen wurden verletzt.

### UNGEKLÄRT

Die verfügbaren Informationen reichen für eine
belastbare Entscheidung nicht aus.

Der Status des Tagesabschlusses ist getrennt von
den Ergebnissen seiner einzelnen Arbeitsblöcke.

## 6. Fehlerbehandlung

Bei einem fehlgeschlagenen Arbeitsblock:

1. Das vollständige Originalprotokoll auswerten.
2. Technischen Fehler und Transportfehler unterscheiden.
3. Bereits erfolgreiche Schritte feststellen.
4. Mögliche Teiländerungen dokumentieren.
5. Keine unkontrollierte Wiederholung ausführen.
6. Gegebenenfalls einen begrenzten Folgeblock
   mit neuem Nachweiszweck vorbereiten.

Bei Widersprüchen zwischen Dokumentation und
Originalprotokollen hat der belegte Originalbefund
Vorrang; die Dokumentation wird nicht ohne
Freigabe verändert.

## 7. Sicherheits- und Freigabegrenzen

Der Tagesabschluss ist zunächst ein Auswertungs-
und Dokumentationsverfahren.

Er erteilt keine pauschale Freigabe für:

- Änderungen am NixOS-Image.
- U-Boot- oder Kernel-Patches.
- Hardwaretests.
- Schreibzugriffe auf SD, SSD oder externe HDD.
- Änderungen an Git-Index oder Nutzerdateien.
- Commits und Projekt-Pushes.
- Installation zusätzlicher Werkzeuge.

Der GitHub-Protokolltransport ist ausschließlich
im Rahmen seiner bereits erteilten Freigabe zu nutzen.

Sensible Inhalte dürfen nicht unkontrolliert
in das Protokollrepository gelangen.

## 8. Praktische Qualifikation

Vor der endgültigen Freigabe des Verfahrens:

1. Einen abgeschlossenen Arbeitstag auswählen.
2. Die vorhandenen Originalprotokolle heranziehen.
3. PASS, FAIL und offene Arbeiten korrekt erfassen.
4. Die belegten Git-Checkpoints zuordnen.
5. Den Backupstatus korrekt einordnen.
6. Eine nachvollziehbare Übergabe entwerfen.
7. Prüfen, ob ein neuer Chat damit fortsetzen kann.
8. Fehlende Anforderungen dokumentieren.
9. Erst danach den Verfahrensstatus aktualisieren.

Die erste praktische Qualifikation soll anhand
der vorhandenen Protokolle vom 2026-10-12 erfolgen.

Keine historischen technischen Tests erneut ausführen.

## 9. Bezug zu anderen Projektunterlagen

- NOVENA-ARBEITSREGELN.txt
- TODO.md
- docs/CHAT-STARTER.md
- docs/HANDOVER-LATEST.md
- docs/PROJECT-STATE.md
- docs/TEST-LOG.md
- docs/DECISIONS.md
- docs/project-index/README.md

Die jeweiligen Originalprotokolle bleiben
maßgebliche technische Nachweise.

## 10. Aktueller Status

Stand der Verfahrensentwicklung am 2026-10-12:

- Dokumentationsentwurf erstellt.
- Erster Probelauf DAY-CLOSE-03: FAIL.
- Ursache des Fehlers nachvollziehbar festgestellt.
- Korrigierter Probelauf DAY-CLOSE-04: PASS.
- Zehn abgeschlossene Originalprotokolle erfolgreich geprüft.
- Fachliche Prüfung DAY-CLOSE-06 durchgeführt.
- Drei daraus abgeleitete Verbesserungen eingearbeitet.

Die technische Probelaufprüfung ist erfolgreich.

Die fachliche Gesamtqualifikation des Verfahrens
bleibt bis zur abschließenden Bewertung offen.

Noch offen:

- Abschließende fachliche Bewertung der Ergänzungen.
- Entscheidung über den Qualifikationsstatus.
- Gesondert freigegebener Dokumentationscommit.
- Synchronisierung zu Codeberg und GitHub.
- Praktische Anwendung auf einen vollständigen
  Tagesabschluss einschließlich aktueller Übergabe.

## 11. Erkenntnisse aus der ersten praktischen Erprobung

Prüfdatum: 2026-10-12.

### 11.1 Abgeschlossene Protokolle eindeutig auswählen

Ein laufender Arbeitsblock darf sein eigenes, noch
unvollständiges Protokoll niemals als abgeschlossenen
Originalnachweis auswerten.

Bei der Protokollauswahl muss deshalb vor Beginn
des Prüfblocks ein eindeutiger Nachweisumfang
festgelegt werden.

Zulässige Verfahren sind beispielsweise:

- eine explizite Liste bereits abgeschlossener Dateien,
- ein belegter Abschlusszeitpunkt als Auswahlgrenze,
- ein zuvor qualifizierter Protokollmanifest-Stand.

Eine bloße Dateisuche nach Tagesdatum reicht nicht,
wenn sie das aktuelle Runner-Protokoll einschließt.

Die SHA256-Sidecars müssen für sämtliche ausgewählten
Protokolle bereits vollständig vorliegen.

### 11.2 Historische Fehler erhalten

Die erste Erprobung zeigte:

- NOVENA-DAY-CLOSE-03: FAIL.
- Ursache: laufendes Eigenprotokoll wurde mitgeprüft.
- NOVENA-DAY-CLOSE-04: PASS.
- Zehn abgeschlossene Protokolle wurden geprüft.
- Sechs historische PASS-Ergebnisse.
- Vier historische FAIL-Ergebnisse.
- Alle zehn Protokolle mit gültigen SHA256-Nachweisen.

Der erfolgreiche Folgeblock korrigiert die Prüflogik,
ändert jedoch nicht das historische FAIL-Ergebnis.

### 11.3 Projektzustand getrennt bewerten

Ein erfolgreicher Tagesabschluss bedeutet nicht,
dass das gesamte Novena-Projekt fehlerfrei ist.

Insbesondere bleiben getrennt:

- Ergebnis des aktuellen Abschlussblocks.
- Ergebnisse der historischen Arbeitsblöcke.
- Qualifikation des Abschlussverfahrens.
- Offene technische und organisatorische Aufgaben.
- Status von Git-Commits und Remote-Synchronisierung.

Der ursprüngliche Git-Index und die vorhandenen
Intent-to-add-Einträge bleiben geschützt.

### 11.4 Backupstatus nicht überinterpretieren

Die dokumentierte R6-Sicherung gilt als qualifizierter
Sicherungscheckpoint.

Die vollständige Wiederherstellungsprobe bleibt offen.

Ein Tagesabschluss darf aus einem vorhandenen
Backupnachweis keine erfolgreich getestete
Wiederherstellbarkeit ableiten.

### 11.5 Erste praktische Qualifikation

Der korrigierte Probelauf NOVENA-DAY-CLOSE-04
hat die vorgesehene historische Protokollauswertung,
die Trennung von PASS und FAIL sowie die Erhaltung
des Git-Checkpoints erfolgreich demonstriert.

Damit ist die technische Probelaufprüfung erfolgreich.

Die abschließende fachliche Qualifikation des gesamten
Verfahrens bleibt eine gesonderte Entscheidung.

Vor dieser Entscheidung sind insbesondere die
Vollständigkeit der Übergabe, die Dokumentationsregeln
und die Freigabegrenzen zu bewerten.

### 11.6 Vollständigkeit des Nachweisumfangs

Ein technisch erfolgreicher Probelauf darf nur
Aussagen über den tatsächlich ausgewählten und
nachweislich geprüften Protokollumfang treffen.

Für einen vollständigen Tagesabschluss müssen
zusätzlich erfasst werden:

- Der betrachtete Zeitraum mit Anfang und Ende.
- Alle für diesen Zeitraum relevanten Arbeitsblöcke.
- Begründete Ausschlüsse und separate Teilprojekte.
- Vorhandene Originalprotokolle und SHA256-Nachweise.
- Fehlende, unvollständige oder ungeklärte Nachweise.
- Nachweis, ob die Auswahl vollständig oder begrenzt ist.

Ein begrenzter Prüfdatensatz muss ausdrücklich
als Teilumfang bezeichnet werden.

DAY-CLOSE-04 bestätigte zehn ausgewählte,
abgeschlossene Originalprotokolle.

Daraus folgt keine automatische Bestätigung,
dass sämtliche Arbeitsblöcke des gesamten
2026-10-12 erfasst wurden.

Bei unvollständiger Quellenlage ist der
Tagesabschluss entsprechend als PARTIAL
oder UNGEKLÄRT zu kennzeichnen.

### 11.7 Übergabe und Wiederaufnehmbarkeit

Ein Tagesabschluss ist erst vollständig, wenn
die vorgesehene Übergabe tatsächlich erstellt
oder eine vorhandene, nachweislich aktuelle
Übergabe ausdrücklich bestätigt wurde.

Eine vollständige Übergabe muss mindestens
folgende Angaben enthalten:

- Datum und Geltungsbereich.
- Letzten qualifizierten Projektcheckpoint.
- Branch, HEAD und geschützten Indexzustand.
- Relevante vorhandene Worktree-Änderungen.
- Ergebnisse einschließlich historischer FAILs.
- Offene Aufgaben und Entscheidungen.
- Sicherheits- und Freigabegrenzen.
- Verweise auf die maßgeblichen Originalprotokolle.
- Den nächsten konkreten Arbeitsschritt.

Die Wiederaufnehmbarkeit muss praktisch
oder anhand eines ausdrücklich definierten
und dokumentierten Abnahmekriteriums
überprüft werden.

Eine nur geplante oder entworfene Übergabe
ist kein abgeschlossener Übergabenachweis.

Änderungen an docs/HANDOVER-LATEST.md oder
anderen Projektdateien erfolgen ausschließlich
nach entsprechender Freigabe.

### 11.8 Noch offene Schritte

- Abschließende Bewertung der drei Ergänzungen.
- Entscheidung über den Qualifikationsstatus.
- Gesondert freigegebener Dokumentationscommit.
- Synchronisierung zu Codeberg und GitHub.
- Vollständigen Tagesabschluss mit aktueller
  Übergabe gesondert praktisch nachweisen.
- Anschließende Auswertung von NOVENA-TOOLS-AUDIT-01.
