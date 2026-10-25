# Novena – Tagesabschlussverfahren

> Aktueller Workflowstand: siehe Nachtrag vom 2026-10-17 am Dateiende (Zwei-Skripte-Workflow, E2E PASS; Konkurrenztest offen). Fruehere Statusangaben sind datierte historische Staende.

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

## Nachtrag 2026-10-12 – NOVENA-DAY-CLOSE-11: belegter Abschlussstand

Dieser Nachtrag ergänzt die historischen offenen Punkte in Abschnitt 10
und 11.8. Der Verfahrenscommit und seine damalige Synchronisierung sind
inzwischen belegt; die fachliche Gesamtqualifikation bleibt offen.

### Originalnachweise

Die vollständigen lokalen Originalprotokolle unter
`~/novena-backups/block-logs/` wurden einschließlich ihrer eingebetteten
Ausgaben berücksichtigt. Ihre SHA256-Werte stimmen mit den Sidecars überein:

| Block | Originalprotokoll | SHA256 |
|---|---|---|
| DAY-CLOSE-08 | `NOVENA-DAY-CLOSE-08-20261012-195400-vMn3ybjc.txt` | `cbdfbb179e01edea38c7d60ac1580be6d0a6f84af1d376cbae630a17e60346f1` |
| DAY-CLOSE-09 | `NOVENA-DAY-CLOSE-09-20261012-195619-CrrMHwn2.txt` | `c516addeae07d465f6319c9133c10365cb9ab8fe4febfbca299c0d1928d5baaf` |
| DAY-CLOSE-10 | `NOVENA-DAY-CLOSE-10-20261012-195933-nEOtxUXi.txt` | `75081136a4cff11275c97ce85851400bb3524d1bb606d7efb2c4bdbce9f6f4db` |

### Commit, Push und zeitlich begrenzte Zählungen

DAY-CLOSE-08 (19:54:00–19:54:14 CEST) erstellte erfolgreich den Commit
`5e2f7afbad802b49b8305dd2a5437c8550bae2be` für `docs/DAILY-CLOSE.md`
und pushte ihn erfolgreich zu Codeberg und GitHub. Beide Remote-HEADs
entsprachen beim damaligen Abschluss diesem Commit. Der Originalindex
blieb bytegleich. Das ist ein historischer Synchronisierungsnachweis,
keine neue Remote-Abfrage und keine fachliche Gesamtqualifikation.

| Erhebung am 2026-10-12 | Abgeschlossene Originalprotokolle | Runner-PASS | Runner-FAIL |
|---|---:|---:|---:|
| DAY-CLOSE-09, 19:56:19–19:56:20 CEST | 173 | 135 | 38 |
| DAY-CLOSE-10, Erhebung ab 19:59:33 CEST; Blockende 20:03:05 CEST | 174 | 136 | 38 |

Jede Zählung gilt ausschließlich für den jeweils erfassten lokalen
abgeschlossenen Bestand vor dem laufenden Erhebungsblock. DAY-CLOSE-09
ist in seiner eigenen Zählung ausgeschlossen; DAY-CLOSE-10 erfasst den
inzwischen abgeschlossenen DAY-CLOSE-09, schließt sein Eigenprotokoll
jedoch aus. Eingebettete historische Abschlussmarker machen ein laufendes
Eigenprotokoll nicht zu einem abgeschlossenen Nachweis.

Alle erfassten Protokolle bestanden laut den Erhebungen SHA256-Abgleich
und Abschlussmarkerprüfung. Das ersetzt weder die fachliche Abnahme jedes
Blocks noch einen unabhängigen Sollbestand aller Tagesarbeiten oder
Einzeltransportnachweise. Eine vollständige Tagesabdeckung ist nicht belegt.
Die 38 historischen FAILs bleiben unverändert FAIL, auch bei späteren
erfolgreichen Folgeblöcken.

### Analyse und verbleibende Abnahme

DAY-CLOSE-10 ist als Analyseblock erfolgreich: `LOCAL_ANALYSIS=PASS`,
`CODEX_EXIT_STATUS=0`, `CODEX_REVIEW=COMPLETED`, `BLOCK_RESULT=PASS`.
Die protokollierte Codex-Auswertung ist eine belegte Analyse mit Zuordnung
der 38 FAILs und Bewertung des Tagesabschlusses als **PARTIAL**.
`CODEX_REVIEW=OPTIONAL` beschreibt die Rolle des Reviews im Skript und
hebt dessen tatsächlich protokollierte Durchführung nicht auf. Weder
Exitcode noch Runner-PASS erteilen eine endgültige Gesamtqualifikation.

Bei Abschluss von DAY-CLOSE-09 und DAY-CLOSE-10 war die Übergabeaktualisierung
noch nicht ausgeführt (`HANDOVER_UPDATE=NOT_YET_PERFORMED`). Mit diesem
Dokumentationsauftrag werden Übergabe, Starter und TODO datiert ergänzt.
Das belegt die Dokumentationsaktualisierung, keine praktische Wiederaufnahme.

Offen bleiben der definierte Tages-Sollumfang samt Zeitraum und Ausschlüssen,
die Abnahme der aktualisierten Übergabe, die praktische Wiederaufnahmeprüfung
und die abschließende Verfahrens-/Tagesqualifikation. R6 ist als
Sicherungscheckpoint qualifiziert; sein vollständiger Restore bleibt offen.
Der separate Cleanup-Restore schließt ihn nicht ab. Keine neue technische
Freigabe, kein neuer Commit oder Push folgt aus diesem Nachtrag.

## 2026-10-12 – Tagesabschluss nach TOOLS-CLOSE-02

Regulärer, begrenzter Tagesabschluss. Die dokumentierten
DAY-CLOSE- und TOOLS-Arbeiten wurden abgeschlossen; eine
vollständige fachliche Gesamtqualifikation sämtlicher
historischer Tagesprotokolle wird nicht behauptet.

Letzter bestätigter Git-Checkpoint:
`17524d75222ee8301efac3ce97e84c6a31640877`.

Der ursprüngliche Index-SHA256 bleibt
`6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
Die neun Intent-to-add-Patches 0004–0012 bleiben erhalten.

Ergebnisse:
- DAY-CLOSE-12: Dokumentationscommit und Synchronisierung
  auf Codeberg und GitHub erfolgreich.
- TOOLS-CLOSE-01: Werkzeugentscheidungen in TODO und
  DECISIONS dokumentiert.
- TOOLS-CLOSE-02: Commit `17524d7`, beide Remotes
  erfolgreich synchronisiert.
- Vorhandene Werkzeuge weiterverwenden; Recoll, Restic,
  Borg und weitere Ergänzungen nur bei konkretem Bedarf.
- Keine Builds oder Hardwaretests im Abschlussverfahren.

Offen bleiben:
- Praktischer Chat-Starter- und Wiederaufnahmetest.
- Vollständige Tagesqualifikation außerhalb dieses
  begrenzten Abschlusses.
- Vollständiger R6-Restore.
- DDR/SPL, Bootprovenienz und externer Bootpfad.

Die 38 historischen FAILs der früheren Tageserhebung
bleiben FAIL. I2C3/ES8328 bleibt abgeschlossen.

Nächster Schritt am 2026-10-13:
Neuen Projektchat starten, die aktualisierte Übergabe
lesen und den aktuellen Git-Checkpoint bestätigen.
Keine neue allgemeine Auditserie.

## NOVENA-DAY-CLOSE-13 – 2026-10-13

Geltungsbereich: DDR-07H bis DDR-07K, vollständiger
NixOS-Image-Build, Imageprüfung, SD-Identifikation,
SD-Schreibvorgang und vollständiger Readback.

### Bestätigte Ergebnisse

- DDR-07H: vollständiger Image-Build PASS.
- DDR-07I: Imageintegrität und eingebetteter SPL PASS.
- DDR-07J: Novena-Test-SD identifiziert, PASS.
- DDR-07K: vollständiges Image geschrieben, PASS.
- Imagegröße: 2581291008 Bytes.
- Image- und SD-Readback-SHA256:
  937776dbb17f0a870fca22519c20a04845dd530de7db8c0496b3e5d286cc2934
- Zielmedium beim Schreibvorgang: /dev/sda,
  USB-Kartenleser, 31914983424 Bytes.
- Git-HEAD und Index während der vier Blöcke unverändert.
- Kein Hardware-Bootversuch in diesen vier Blöcken.

### Originalnachweise

- NOVENA-DDR-07H-20261013-195302-cVlOKEgi.txt
- NOVENA-DDR-07I-20261013-195503-b83bvlPP.txt
- NOVENA-DDR-07J-20261013-195952-XHFMQdKS.txt
- NOVENA-DDR-07K-20261013-200119-LJXUcDmY.txt

Jeweils mit SHA256-Sidecar unter
~/novena-backups/block-logs/ und im privaten
GitHub-Protokollrepository loomit77/project-block-logs.

### Bewertung und Grenzen

Die Image-Erstellung, Medienbeschreibung und
vollständige Rückleseprüfung sind PASS.

Die DDR/SPL-Root-Cause bleibt offen.
Patch 0013 ist noch nicht auf der Novena gebootet.
Eine Bootfähigkeit wird aus dem erfolgreichen
Image-Readback nicht abgeleitet.

Dieser Abschluss bezieht sich auf die genannten
vier Arbeitsblöcke, nicht auf sämtliche Aktivitäten
des Kalendertags oder die vollständige fachliche
Qualifikation des Tagesabschlussverfahrens.

Der formale Gesamtstatus bleibt deshalb PARTIAL,
bis die Übergabe und der Abschlussblock geprüft sind.

### Nächster Schritt

Kontrollierter Kaltstart der Novena mit der
vorbereiteten Test-SD und vollständiger serieller
Aufzeichnung. Nur nach eigener Testfreigabe.

Keine automatische Git-Synchronisierung.


## Nachtrag 2026-10-14 – Vollständiger Tagesabschluss nach Arbeitsregel 14

Der Tagesabschluss muss einen ausdrücklich abgegrenzten Zeitraum und Sollumfang erfassen: alle zugehörigen Arbeitsblöcke, begründete Ausschlüsse, separate Teilprojekte sowie vorbereitete, laufende, fehlgeschlagene und abgeschlossene Arbeiten. Ein begrenzter Protokollbestand oder ein erfolgreicher Abschlussrunner belegt keine vollständige Tagesabdeckung. Die fachliche Gesamtqualifikation des Verfahrens wird durch diesen Nachtrag nicht erteilt.

Ein vollständiger Abschluss dokumentiert folgende Identitäten mit Prüfzeitpunkt, Herkunft und Aussageumfang; fehlende oder nicht zugängliche Werte werden ausdrücklich als offen gekennzeichnet:

- Projekt, Host, Repository, Branch, HEAD, Index-SHA256, relevante Index- und Worktree-Zustände einschließlich Nutzeränderungen und Intent-to-add-Einträgen. Lokaler Stand, dokumentierte Pushes und tatsächlich geprüfte Remote-Identitäten bleiben getrennt.
- Relevante Datei- und Artefaktpfade, Größen und SHA256-Werte sowie Quellstände, Konfigurationen und Patches. Vorher- und Nachherstände werden bei Änderungen eindeutig zugeordnet; historische Identitäten werden nicht ersetzt.
- Blockkennungen, Ausführungszeiten, vollständige Originalprotokollnamen und Fundorte, zugehörige Sidecars und geprüfte Hashwerte, Abschlusszustände und Exitcodes. Auch Vorbereitung und Erzeugung von Kommandodateien gehören zum Nachweisumfang; unprotokollierte Schritte bleiben Nachweislücken.
- Backup-Checkpoint, Sicherungsziel, Sicherungszeitpunkt, Manifest- und Prüfsummenidentitäten, gesicherter Umfang und vorhandene Integritätsnachweise. Sicherung, Rückleseverifikation und vollständige Wiederherstellungsprobe werden getrennt bewertet. R6 bleibt der dokumentierte qualifizierte Sicherungscheckpoint; sein vollständiger Restore bleibt offen. Eine neue Sicherung oder Medienprüfung wird hier nicht behauptet.
- Geltende Regeln mit Versionsstand, technische Resultate, fachlicher Abnahmestand, offene Aufgaben, Risiken, Abhängigkeiten und nächster empfohlener Schritt samt benötigten Freigaben. Maßgeblich sind insbesondere Arbeitsregeln 13 und 14 vom 2026-10-14 und AGENTS.md; dessen Identität ist bei einer späteren qualifizierten Erhebung gesondert festzuhalten.

Für jeden Block sind Upload, Runner, Integrität und Fachresultat vier eigenständige Bewertungen. Upload-PASS belegt den Transport im nachgewiesenen Umfang, Runner-PASS den entsprechenden Ausführungsabschluss und Hashgleichheit Byteübereinstimmung. Keines dieser Ergebnisse ersetzt Protokollvollständigkeit oder fachliche Abnahme. Ein beendeter Runner-FAIL kann auswertbare Teilresultate liefern; seine fachlich offenen Teile bleiben offen.

Originale und Sidecars werden zuerst lokal ausgewertet. Bei berechtigter GitHub-Anbindung und konkreter Netzwerkfreigabe werden benötigte Dateien selbstständig in `loomit77/project-block-logs`, `novena/YYYY/MM/`, anhand der Blockkennung gesucht und vollständig abgerufen. Bereits zugängliche Dateien werden nicht erneut beim Nutzer angefordert. Fehlende Originale, Sidecars oder Abrufmöglichkeiten sind benannte Lücken, keine bestandenen Prüfungen.

Als übergebener Stand vom 2026-10-14 gelten HEAD `7d98296db282aed0cadbf1d6cf9545ccb591f8a6` und Index-SHA256 `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`. ARCHIVE-08 ist in der Ausführung beendet, mit Runner-FAIL und zwei Downloadfehlern; ein fachlicher Archivabschluss liegt nicht vor. DDR-08H zeigt Q1/C1 ZERO und D32 `ret=-12`; vollständiger Boot und Patchfix sind nicht bestätigt. I2C3/ES8328 bleibt geschlossen. Diese Angaben stammen aus dem Auftrag; Originalnamen, Sidecarhashes, Uploadstatus und unabhängige Integritätsprüfung dieser beiden Blöcke wurden hier nicht erhoben und werden nicht ergänzt oder erfunden.

Laufende Eigenprotokolle und noch entstehende Sidecars dürfen nicht als abgeschlossene Nachweise gelten. Parallele Arbeiten erhalten getrennte Zuständigkeiten und abgegrenzte Ressourcen; ein Tagesabschluss darf ihren Zustand weder überschreiben noch als erfolgreich beendet darstellen. Bestehende Nutzeränderungen und der Originalindex bleiben erhalten. Bei wesentlichen ungeklärten Widersprüchen ist vor darauf aufbauenden Änderungen anzuhalten.

Die Übergabe muss tatsächlich vorliegen und die Wiederaufnahmekriterien nach Arbeitsregel 14 erfüllen. Vor neuen technischen Empfehlungen folgt die Prüfung der Übergabe, des Abschlusses, der relevanten Projektquellen und Originale. Bei fehlenden Pflichtnachweisen bleibt der Abschluss PARTIAL oder UNGEKLÄRT entsprechend der verfügbaren Evidenz. Dieser Nachtrag definiert die Anforderungen, bestätigt aber weder einen vollständigen Tagesabschluss vom 2026-10-14 noch eine praktische Wiederaufnahme oder fachliche Gesamtfreigabe.


---

## Tagesabschluss 2026-10-14 – NOVENA-DAY-CLOSE-15D

Kennung: NOVENA-DAY-CLOSE-15D / 2026-10-14

### Protokollinventur

- NOVENA-DAY-CLOSE-15A: PASS.
- 75 lokale TXT-Protokolle wurden während der Inventur erfasst.
- 74 bereits abgeschlossene Protokolle hatten gültige
  SHA256-Begleitdateien.
- Darunter waren 67 PASS und 7 FAIL.
- Das gleichzeitig erzeugte 15A-Protokoll war bei seiner
  eigenen Inventur noch nicht abgeschlossen.
- Nach Abschluss von 15A liegt auch dessen TXT-Protokoll
  mit SHA256-Begleitdatei auf GitHub vor.
- Somit sind für die erfassten 75 Blöcke 68 PASS und
  7 FAIL dokumentiert.
- Die technische Bewertung der FAIL-Blöcke bleibt
  anhand ihrer jeweiligen Originalprotokolle maßgeblich.

### Archiv und historische Quellen

- ARCHIVE-08: FAIL wegen zweier fehlgeschlagener Downloads.
- ARCHIVE-09A: PASS, reine Bestandsaufnahme.
- Thread 488: Wayback HTTP 404.
- Thread 561: Wayback HTTP 302.
- 165 erfolgreiche Downloads in ARCHIVE-08.
- 343 vorhandene HTML-Dateien.
- Die beiden Threads bleiben als gezielte Nachrecherche offen.
- Kein vollständiger erneuter Archivdownload freigegeben.

### DDR/SPL und Boot

- DDR-08H: Q1/C1 ZERO.
- D32: ret=-12.
- Ein vollständiger erfolgreicher Boot mit dem zuletzt
  untersuchten Stand ist nicht nachgewiesen.
- DDR/SPL bleibt offen.
- Keine neue Build-, Patch-, Hardwaretest- oder
  SD-Schreibfreigabe durch diesen Tagesabschluss.
- I2C3/ES8328 bleibt abgeschlossen und wird nicht
  erneut als offene Ursache behandelt.

### Git und Dokumentation

- NOVENA-DAY-CLOSE-15B: PASS.
- NOVENA-DAY-CLOSE-15C: PASS.
- Ausgangs-HEAD:
  7d98296db282aed0cadbf1d6cf9545ccb591f8a6
- Ausgangs-Index-SHA256:
  6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937
- Der ursprüngliche Index enthält absichtlich vorbereitete
  Änderungen und Intent-to-add-Patches 0004 bis 0012.
- docs/DAILY-CLOSE.md war im Originalindex als
  Löschung vorgemerkt, liegt im Worktree aber vor.
- Kein pauschales git add -A, kein Reset und keine
  ungeprüfte Übernahme dieser Indexeinträge.
- Der isolierte Commitumfang aus 15C umfasst genau:
  NOVENA-ARBEITSREGELN.txt,
  TODO.md,
  docs/CHAT-STARTER.md,
  docs/DAILY-CLOSE.md,
  docs/HANDOVER-LATEST.md.
- Die beiden zuletzt genannten Dateien werden
  durch 15D um diesen Abschlussstand ergänzt.

### Abschlusskriterium

Der Tagesabschluss ist erst vollständig, wenn der
freigegebene Dokumentationsstand committet, zu Codeberg
und GitHub gepusht und anschließend gegen beide
Remote-Referenzen verifiziert wurde.

Bis dahin lautet der Status:
DOKUMENTATION VORBEREITET – GIT-PUSH AUSSTEHEND.

Die TXT-Protokolle mit SHA256-Begleitdateien werden
getrennt im GitHub-Repository project-block-logs
archiviert.

> Aktueller Abschluss: siehe den Nachtrag „2026-10-15 – Abschluss und Übergabe nach Borg 35P-19“ am Dateiende; ältere Stände sind historisch.

## 2026-10-15 – Abschluss und Übergabe nach Borg 35P-19

Dieser datierte Nachtrag ist für den hier beschriebenen Abschluss maßgeblich.
Ältere Statusangaben bleiben historische Checkpoints. Grundlage sind die
vollständig ausgewerteten lokalen TXT-Originale samt übereinstimmenden
SHA256-Sidecars; das Nachweisregister steht im gleichnamigen Nachtrag in
`docs/TEST-LOG.md` (aus `docs/` heraus: `TEST-LOG.md`).

- **Borg-27:** nachgewiesene reale Sicherung am 15.10.2026,
  13:29:56–13:40:47 CEST. `BORGMATIC_CREATE_EXIT=0`, genau ein neues Archiv
  (Bestand 2 → 3), `ARCHIVE_CHECK=PASS` und
  `ARCHIVE_DATA_VERIFICATION=PASS`. Dies ist keine Restore-Abnahme.
- **35O-R5:** 64 erfolgreiche Offline-Tests in fünf Suiten
  (8 + 22 + 10 + 15 + 9); keine reale Borg-Ausführung in diesem Testblock.
- **35P-12:** R4-Laufzeitnachweis erfolgreich: Downstream betreten,
  äußere Sperre während der Bridge gehalten, konkurrierender R4-Aufruf
  abgewiesen, Bridge abgeschlossen, Sperre nach Rückkehr freigegeben.
  **Der R4-Laufzeitnachweis ist nicht mehr offen.** R4 ist ein
  Offline-Prüfmechanismus; daraus folgt keine produktive Repository-Sperre
  für eine reale Sicherung und keine produktive Gesamtfreigabe.
- **35P-14:** FAIL, Exitstatus 1 wegen eines fehlerhaft formatierten
  SHA256-Prüfblocks. Kein daraus belegter Quelldefekt. **35P-14R1:**
  korrigierte Aufrufpfadanalyse PASS; der Testdateihash wurde erfasst,
  aber ausdrücklich nicht mit einem historischen Sollhash verglichen.
  Der frühere FAIL bleibt erhalten.
- **35P-15/16:** Kontrollfluss-, Syntax- beziehungsweise Strukturanalyse
  des bestehenden Tagesabschlusses erfolgreich; keine reale Sicherung.
- **35P-17:** Offline-Borg-Nachweisprüfer erfolgreich gegen Borg-27,
  einschließlich Protokollhash, Sidecar, Archivanzahl, Archiv-ID,
  Archivprüfung, Datenverifikation und Runnernachweis.
- **35P-18:** Offline-Integrationstest und Abweisung einer nicht
  autorisierten Tagesabschlussausführung erfolgreich. Der Starter
  `~/novena-backups/workflow-improvement-35/borg/novena-dayclose-with-borg-35p18.bash`
  prüft den historischen Borg-27-Nachweis und startet keine Sicherung.
  Die Nachweisauswahl ist noch fest gebunden, nicht dynamisch.
- **35P-19:** integrierter Tagesabschluss erfolgreich ausgeführt
  (16:06:05 CEST; `DAYCLOSE_EXECUTED=YES`,
  `DAYCLOSE_INTEGRATION=PASS`, Runner-PASS, Exitstatus 0).
  Dabei kein Borg-/borgmatic-Aufruf und kein Borg-Repositoryzugriff.

**Korrektur veralteter Statusausgaben:** Der in 35P-19 eingebettete ältere
Tagesabschluss meldet weiterhin `BORG_FULL_LOCK_DURATION_PROOF=OPEN` und
führt den Laufzeitnachweis als offene Aufgabe. Das ist durch 35P-12 überholt,
kein neuer Fehlschlag. Auch die damaligen pauschalen offenen Integrationspunkte
sind anhand 35P-17/18/19 auf den tatsächlich erreichten Offline-Nachweisumfang
zu begrenzen. Die Skriptausgabe bleibt unverändert als Originalevidenz erhalten.
Der belegte Ausführungserfolg von 35P-19 ersetzt weder eine unabhängige
Vollständigkeitsinventur aller Tagesarbeiten noch eine produktive Borg-Abnahme.
Upload beziehungsweise Remote-Archivierung wurden in diesem Dokumentationsauftrag
nicht erneut geprüft; lokale Logintegrität und fachliche Ergebnisse sind davon getrennt.

**Offen:** Restore-Abnahme, Offsite-Schlüsselsicherung einschließlich
Wiederherstellbarkeit und produktive Gesamtfreigabe. Die dynamische Auswahl
eines geeigneten aktuellen Sicherungsnachweises sowie die produktive
Sperr-, Freigabe-, Provenienz- und Nachweiserfassung bleiben gesondert zu
qualifizieren. Der vollständige R6-Restore bleibt eine separate offene Aufgabe.

**Nächste Projektpriorität: Novena – universelles NixOS SD-Image.**
Borg-Arbeiten sind für die Wiederaufnahme zurückgestellt. Vor weiteren Builds,
Hardwaretests oder Änderungen zuerst den aktuellen Projektstand, TODO,
Artefaktprovenienz und vollständige vorhandene Boot-/Testnachweise prüfen.
DDR/SPL und der vollständige externe Bootpfad bleiben offen; der dokumentierte
DDR-08H-Befund ist Q1/C1 ZERO und D32 `ret=-12`, kein bestätigter Patchfix
oder vollständiger Boot. I2C3/ES8328 bleibt abgeschlossen;
`regulator-always-on` bleibt dauerhafte Boardanforderung.

### Tatsächlich lokal erhobener Git-Zustand

Prüfung am 2026-10-15T16:13:14+02:00,
Repository `/home/loomit/nixos-novena-image`, Branch `main`:
HEAD `ee99ba0c296626e63c214ec91f5b7d771e35d1f7`, Index-SHA256 `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
Dies ist eine lokale lesende Erhebung, keine Remote-Synchronisationsprüfung.
Der Status vor den Dokumentationsergänzungen lautet:

```text
MM NOVENA-ARBEITSREGELN.txt
MM TODO.md
 A boot/u-boot/0004-novena-uboot-diag-d0-d36.patch
 A boot/u-boot/0005-novena-diag-d21a-d21d.patch
 A boot/u-boot/0006-novena-diag-brk-p0-p1.patch
 A boot/u-boot/0007-novena-diag-brk-q1-q2.patch
 A boot/u-boot/0008-novena-diag-control-c0-c2.patch
 A boot/u-boot/0009-novena-diag-write-level-status.patch
 A boot/u-boot/0010-novena-diag-dqs-status.patch
 A boot/u-boot/0011-novena-spl-disable-bank-interleaving.patch
 A boot/u-boot/0012-novena-ddr-read-specific-dummy-write.patch
 M boot/u-boot/default.nix
MM docs/CHAT-STARTER.md
D  docs/DAILY-CLOSE.md
MM docs/DECISIONS.md
MM docs/HANDOVER-LATEST.md
MM docs/PROJECT-STATE.md
MM docs/QUELLENVERZEICHNIS.md
MM docs/TEST-LOG.md
 M docs/en/docs/BUILD-HOST.md
 M docs/en/docs/BUILD.md
 M docs/en/docs/GOLDEN-BUILD.md
 M docs/en/docs/IMAGE-LAYOUT.md
 M docs/en/docs/QUELLENVERZEICHNIS.md
 D docs/en/research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md
?? boot/u-boot/0013-novena-spl-rtt-nom-rzq2.patch
?? docs/DAILY-CLOSE.md
?? research/nxp-reference-manuals/
?? test-logs/2026-10-05/
?? test-logs/2026-10-07/
```

Die neun Patches 0004–0012 sind vorhandene Intent-to-add-Einträge.
Der Index enthält bereits Dokumentationsänderungen gegenüber HEAD;
`MM` bezeichnet zusätzlich abweichende Worktree-Inhalte.
`docs/DAILY-CLOSE.md` ist im Index zur Löschung vorgemerkt und liegt zugleich
als unversionierte Worktree-Datei vor. Keine Indexreparatur ableiten.
Vorhandene U-Boot-, Übersetzungs-, Forschungs- und Logänderungen gehören
nicht zum Schreibumfang dieses Auftrags. Die historischen HEAD-/Indexwerte
weiter oben sind keine aktuellen Wiederaufnahmevorgaben.

Dieser Auftrag ergänzt ausschließlich sieben bestehende Dokumentationsdateien.
HEAD und Index bleiben erhalten; keine Git-Mutationen, Commits, Pushes,
Builds, Tests, Borg-Ausführungen, Netzwerk-, Blockgeräte- oder Hardwarezugriffe.
Die neue Dokumentation ist lokal uncommittet; eine Veröffentlichung oder
aktuelle Übereinstimmung mit Codeberg/GitHub wird nicht behauptet.

<!-- BORG-INTEGRATION-24J -->

## Borg-Integration: Abschluss der Offline-Pruefphase (2026-10-15)

Die technische Pruefserie BORG-INTEGRATION-24A bis 24I
ist erfolgreich abgeschlossen.

### Nachgewiesene Ergebnisse

- BORG-INTEGRATION-16: realer Borg-Sicherungslauf PASS.
- BORG-INTEGRATION-22: gezielte Archivintegritaetspruefung PASS.
- 35P-17 bis 35P-19: historische Offline-Nachweispruefung
  und Tagesabschlussintegration erfolgreich dokumentiert.
- BORG-INTEGRATION-24G: Status-30 mit 15/15 Tests PASS.
- BORG-INTEGRATION-24G: Sperrmechanismus mit 9/9 Tests PASS.
- BORG-INTEGRATION-24I: erneute Gesamtpruefung
  mit 24/24 Offline-Tests PASS.
- Quellintegritaet und Git-Checkpoint in 24I PASS.

### Verbleibende Anforderungen

1. Nachweis, dass die produktive Aufrufkette die Sperre
   ueber die gesamte Laufzeit einer echten Borg-Sicherung haelt.
2. Nachweis, dass der vollstaendige produktive Tagesabschluss
   bei Fehlern zuverlaessig abbricht und keinen falschen
   Erfolgsstatus meldet.

Diese beiden Anforderungen sollen gemeinsam in einem
gezielten Integrationstest behandelt werden.

Die bisherigen Offline-Tests sind abgeschlossen.
Ohne neue technische Erkenntnisse ist ihre Wiederholung
nicht erforderlich.

BORG_FULL_LOCK_DURATION_PROOF=OPEN
PRODUCTIVE_END_TO_END_PROOF=OPEN
PRODUCTIVE_DAYCLOSE_RELEASE=BLOCKED


## 2026-10-17 – Zwei-Skripte-Workflow: realer E2E-Test PASS

Dieser Nachtrag ist für den aktuellen Bedienungsworkflow maßgeblich.
Ältere Planungs-, Sperr- und Integrationshinweise bleiben historische Stände.
Die älteren `integration/*`- und V8-Dateien bleiben unverändert erhalten.

Aktuelle Bedienungsskripte auf foobox:

- Projektstart: `~/novena-backups/workflow-dev/chat-start.bash`.
- Tagesabschluss: `~/novena-backups/workflow-dev/daily-close.bash`.

Beide werden über den vorhandenen GitHub-Wrapper
`~/novena-backups/github-transport/run-and-upload.bash` ausgeführt.
Die Dokumentation erteilt keine Ausführungs- oder Netzwerkfreigabe.
Der Tagesabschluss führt eine reale Borg-Sicherung aus; sein Aufruf benötigt
entsprechende Freigabe und ist keine rein lesende Statusabfrage.

- **NOVENA-WORKFLOW-TWO-SCRIPTS-04: PASS** – beide Skripte installiert.
- **NOVENA-CHAT-START-TEST-05: PASS** – Projektstart, Git-HEAD,
  Index-SHA256, Worktree, Dokumentinventur und Übergabe erfolgreich.
- **NOVENA-DAILYCLOSE-HARDEN-06: PASS** – Installation mit flock-Sperre,
  Borg-Identitätsprüfung und Archivvergleich am 17.10., 09:45:53 CEST.
  Ein separater Wiederholungsaufruf um 09:47:46–09:47:47 endete mit
  `BASELINE_INTEGRITY=FAIL`, Exit 22; dieser FAIL bleibt erhalten.
- **NOVENA-DAILYCLOSE-E2E-07: PASS** – realer End-to-End-Lauf am
  17.10.2026, 09:54:18–09:54:55 CEST. Exklusive Sperre erworben,
  Borg-Identitätsprüfung und Borgmatic-Konfiguration erfolgreich,
  Backup erstellt. Archivbestand **5 → 6**, genau **1 neues Archiv**,
  **0 verschwundene Archive**. Git-HEAD und Index unverändert.
  Vollständiges TXT-Protokoll und SHA256 vorhanden; GitHub-Upload laut
  verifiziertem Benutzerergebnis erfolgreich, hier nicht erneut abgefragt.

**Abgrenzung:** Der reale E2E-Nachweis ist erfolgreich; ältere pauschale
`PRODUCTIVE_END_TO_END_PROOF=OPEN`- beziehungsweise daraus abgeleitete
Sperrhinweise beschreiben den damaligen Workflow. Ein separater
Parallelstart-/Konkurrenztest der aktuellen flock-Sperre wurde noch nicht
durchgeführt und bleibt offen. Weder der Sperrerwerb im Einzellauf noch
historische R4-/Offline-Tests belegen diesen Konkurrenzfall. Eine umfassende
Fehlerfallqualifikation oder produktive Gesamtfreigabe folgt daraus nicht.

Evidenz: lokal vollständig gelesene Originalprotokolle und jeweils
übereinstimmende SHA256-Sidecars; das Nachweisregister steht im
[TEST-LOG](TEST-LOG.md#2026-10-17--zwei-skripte-workflow-realer-e2e-test-pass).

### Bedienung und Aussageumfang

Bei freigegebener Ausführung wird `daily-close.bash` als Befehlsdatei an den
Wrapper übergeben: `bash "$WRAPPER" "$BLOCK_ID" "$COMMAND_FILE"`.
Dabei bezeichnet `WRAPPER` den obigen Wrapperpfad, `COMMAND_FILE` das
Tagesabschlussskript und `BLOCK_ID` die konkrete neue Arbeitsblockkennung.
Keine alte Testkennung wiederverwenden. Skripte als separate Prozesse starten.
Die früheren Abschnitte ohne automatische Sicherung beschreiben das damalige
Verfahren; für das aktuelle Skript gilt der reale Backupumfang dieses Nachtrags.
Restore, vollständige Tagesabdeckung und Konkurrenzverhalten sind durch den
Einzellauf nicht zusätzlich qualifiziert.

## 2026-10-18 – Tagesabschluss und Borg-Sicherung

Kennung: NOVENA-DAY-CLOSE-2026-10-18-BORG

- Projektpriorität: universelles NixOS-SD-Image (NIXOS-FIRST).
- Git-HEAD vor der Dokumentationsaktualisierung:
  `2c33a6cf4b0454fc2dae22e0a3c3cf62e9737281`.
- Vorhandene Nutzeränderungen: `configuration.nix` und `modules/`;
  nicht in den Tagesabschluss übernommen.
- NOVENA-NIXOS-114A/114B: statische Prüfungen PASS.
- NOVENA-NIXOS-114C/114D: growpart nicht verfügbar;
  keine erfolgreiche Partitionserweiterungsprüfung.
- NOVENA-NIXOS-114E: Offline-Build fehlgeschlagen;
  SHA256-Abweichung beim tinycc-musl-Quellbezug.
- NOVENA-NIXOS-114F: Nix-Cache-Verfügbarkeit geprüft.
- NOVENA-NIXOS-114G: cloud-utils/growpart aus dem
  offiziellen Nix-Cache verfügbar gemacht; PASS.
- NOVENA-NIXOS-114H: Ergebnis hier nicht nachgewiesen;
  gesondert anhand des Originalprotokolls auswerten.
- Root-Partitionserweiterung: noch nicht für den
  produktiven SD-Boot freigegeben.
- ALSA-Build: zum Zeitpunkt von NOVENA-DAY-CLOSE-18D
  weiterhin aktiv; Ergebnis offen.
- Novena: ausgeschaltet; kein weiterer Hardwaretest.
- NOVENA-DAY-CLOSE-18C: Borg-Sicherung erfolgreich.
  Archivbestand 8 auf 9; ein neues Archiv nachgewiesen,
  kein vorheriges Archiv verschwunden.
- Borg-Identität, Konfiguration und Git-Checkpoint:
  PASS. Git-HEAD und Index blieben bei der Sicherung
  unverändert.
- Dokumentations-Commit und Push: zum Zeitpunkt dieses
  Nachtrags noch ausstehend.

Evidenz: Originalprotokolle im GitHub-Repository
`loomit77/project-block-logs`, Verzeichnis `novena/2026/10/`.

Dieser Nachtrag bestätigt keine erfolgreiche ALSA-Erstellung,
keine Root-Erweiterung auf der echten SD und keinen neuen
Hardwareboot.

## Nachtrag 2026-10-21: Audio, Borg VM103 und Tagesabschluss
### Tagesabschluss 2026-10-21

- Audio: Kopfhörer PASS; interne Lautsprecher ohne Ton,
  weitere Untersuchung vorerst eingestellt.
- Borg VM103: Repository erreichbar, erste Sicherung FAIL
  (Exitcode 105), null Archive, Restore offen.
- Block 131 war wegen abweichender SSH-Konfiguration
  keine gültige Prüfung des ursprünglichen Backupzugangs.
- Originalauftrag und Originalprotokoll wurden anschließend
  identifiziert und in Block 135 ausgewertet.
- Dokumentationsänderung und Git-Synchronisierung werden
  im Abschlussblock 136 ausgeführt und sind erst nach
  dessen Ergebnis als abgeschlossen zu bewerten.

### Audio / Novena

- NixOS 26.05, Linux 6.18.49 ARMv7, Patch 0007.
- Kopfhörerausgabe funktioniert.
- Interne Lautsprecher sind angeschlossen, bleiben aber ohne hörbare Ausgabe.
- Während der Wiedergabe waren Speaker, LOUT2, ROUT2 und
  audio-amp im ASoC-DAPM-Pfad aktiv.
- `es8328-power` war enabled, GPIO5_17 stand auf HIGH.
- Das belegt keine physisch gemessene Spannung an den
  Lautsprecherverstärkern U10A/U12A.
- Originalschaltplan: NS4890B/TS4990IST, Versorgung
  `P5.0V_DELAYED`, Steuerung `AUD_PA_SHDN`.
- Kein Multimeter vorhanden. Hardwaremessungen und weitere
  Lautsprecherdiagnose auf unbestimmte Zeit zurückgestellt.
- Keine weitere Kernel-, DTS- oder GPIO-Änderung beauftragt.

### Borg / VM103

- Backupziel: Debian-13-VM103, 3-TB-Datenträger.
- Repository:
  `ssh://borgadmin@192.168.178.20/srv/borg-storage/novena/foobox`
- Das verschlüsselte Borg-Repository ist über die bestehende
  SSH-Konfiguration erreichbar.
- Die erste Sicherung wurde am 2026-10-21 durch Block 97
  als temporärer Dienst gestartet.
- Originalprotokoll:
  `~/novena-backups/vm103-backup-state/first-backup-20261021-073016.log`
- Borgmatic scheiterte bei `borg create --dry-run --list`
  mit Exitcode 105 (Backup permission issue).
- Block 135 bestätigte: Repository erreichbar, aber null Archive.
- Deshalb ist noch keine erfolgreiche VM103-Sicherung nachgewiesen.
- Restore-Test offen.
- Das bisherige USB-Borg-Repository bleibt unverändert.
- Nächster Backupschritt: Berechtigungsursache gezielt untersuchen,
  anschließend erst nach Freigabe Sicherung erneut starten.
- Wichtig: Für Borg die vorhandene Datei
  `~/.config/novena-borg/vm103-known-hosts` verwenden.
  Der direkte Borg-Aufruf in Block 131 nutzte diese nicht.

### Nachweise

- NOVENA-PATCH-0007-SPEAKER-POWER-LIVE-130
- NOVENA-DAY-CLOSE-PRECHECK-131
- NOVENA-BORG-SSH-PRECHECK-132
- NOVENA-BORG-ORIGINAL-RUN-133
- NOVENA-BORG-FIRST-BACKUP-STATUS-134
- NOVENA-BORG-ARCHIVE-VERIFY-135

Die TXT-Protokolle und SHA256-Sidecars liegen im privaten
GitHub-Repository `loomit77/project-block-logs` unter
`novena/2026/10/`. Wrapper-PASS ist kein Beweis
für einen fachlich erfolgreichen Borg-Lauf.

<!-- NOVENA-DAYCLOSE-20261021-239 -->
## 2026-10-21 – Image 2 und Übergabe für den 22.10.

- Image 1: Kernelpatch 0007 und U-Boot 2026.07.
  Realer Novena-Test durchgeführt.
  Manueller Start von `boot.scr` auf `mmc 1:1` erfolgreich;
  Linux und NixOS erreichten die serielle Login-Anzeige.
  Dies ist kein Nachweis für automatischen Boot.
- U-Boot-Diagnose Block 190: Die automatische
  Partitionssuche berücksichtigt MBR-Bootmarkierungen.
- Block 193: Korrektur im Image-Builder:
  FAT32/FIRMWARE (p1) bootable, ext4/NIXOS_SD (p2)
  nicht bootable.
- Image 2: vollständiger foobox-Build Block 195 PASS,
  Git-Quellstand `6fce37e5498fcbc1c3bf78685dd5baf7d900f346`.
- Image 2:
  `/nix/store/n7bm1lbqbhxk62h1n45k0xwis9nzk091-nixos-novena-sd-image.img`
- Imagegröße: `2588422144` Bytes.
- SHA256:
  `144d71c37536d09e5e47d41eb270fa131bdd154244fdd2d21a37a4b0d958777d`
- Block 237: Image vorhanden, Größe und SHA256
  stimmen, MBR-Signatur und Partitionseinträge PASS.
  p1 Bootflag 0x80, p2 Bootflag 0x00.
- Block 238: Tagesabschluss-Vorprüfung PASS;
  Git-Worktree sauber, Workflow-Skripte vorhanden.
- Mesa-Commit `1cefe299` liegt nach dem Image-2-
  Quellstand. Mesa ist damit nicht als Bestandteil
  dieses Images nachgewiesen.
- Automatischer Hardwareboot von Image 2:
  NICHT GETESTET.
- Hardwaretest auf 2026-10-22 verschoben.
  Vorher SD-Zielgerät eindeutig bestimmen.
  Schreiben nur nach gesonderter Freigabe.
- Kein neuer Build und keine weiteren Patches
  vor diesem Hardwaretest.
- Borg: reguläres Daily-Close-Backup und separates
  VM103-Backup sind getrennt nachzuweisen.
  VM103 war zuvor mit Exitcode 105 fehlgeschlagen;
  kein erfolgreicher neuer Lauf wird hier behauptet.
- Der Dokumentationsabschluss ist nicht selbst
  der Nachweis einer Borg-Sicherung.

<!-- NOVENA-DAY-CLOSE-20261025-BRIDGE -->
## 2026-10-25 – Bridge-Exchange-Entwicklungscheckpoint

Stand vor Dokumentationsabschluss:

- Projekt-HEAD: `29107f15b628abb0b18a34369c3957da75cd0b99`
- Block 157: synthetisches Publisher-Gate committed,
  Codeberg/GitHub PASS.
- Block 158: privates Archiv über REST und SSH gelesen;
  Statusindex nicht vorhanden.
- Block 159: GraphQL-updateRefs-Schema PASS.
- Block 160: isolierter Codex-Patch erstellt.
- Block 161: Patchaudit PASS; 17 Tests PASS.
- Block 162: Patchintegration PASS.
- Block 163: vier Dateien committed, Remotes PASS.
- Produktionsveröffentlichung weiterhin gesperrt.
- Kein Bridge-bezogener Build oder Hardwaretest.
- Produktiver API-Adapter und Commitbereitstellung offen.

Dieser Eintrag belegt keine abgeschlossene Borg-Sicherung,
keinen abgeschlossenen Dokumentations-Push und keine
vollständige Tagesabschlussabnahme.
Diese Ergebnisse müssen gesondert protokolliert werden.
