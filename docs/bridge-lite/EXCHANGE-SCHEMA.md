# NOVENA-BRIDGE-LITE – Exchange-Datenmodell v1

Stand: 2026-10-25

Status: Entwurf, strukturell getestet.
Keine Freigabe zur automatischen Veröffentlichung.

## 1. Zweck

NOVENA-CURRENT-STATE.json soll im bestehenden
privaten GitHub-Archiv den letzten ausdrücklich
qualifizierten Novena-Projektcheckpoint referenzieren.

Der Index ist eine abgeleitete Lesehilfe.
Originale bleiben maßgeblich.

Das JSON-Schema befindet sich in:
`docs/bridge-lite/exchange-schema-v1.json`.

## 2. Struktur

- `schema_version`: Formatversion, zunächst 1
- `project`: immer `novena`
- `generated_at_utc`: Erzeugungszeitpunkt
- `last_verified`: qualifizierter Checkpoint oder null
- `current`: Kenntnisstand des aktuellen Zustands
- `latest_block`: letzter erfasster Block oder null
- `build`: separat belegter Buildstatus
- `evidence`: Datei- und Prüfreferenzen
- `publication`: Status der Index-Veröffentlichung
- `review_required`: offene Prüfhinweise

Fehlende Angaben werden nicht geraten.

## 3. LAST_VERIFIED

`last_verified` ist kein Synonym für den zuletzt
ausgeführten Block.

Ein Checkpoint darf nur nach gesonderter
Qualifikationsentscheidung eingetragen werden.

Er muss mindestens eine Git-Identität und
einen nachvollziehbaren Quellenverweis besitzen.

Die Struktur allein bestätigt nicht, dass die
referenzierten Dateien tatsächlich existieren,
gültig sind oder zu genau diesem Checkpoint gehören.

## 4. CURRENT_UNKNOWN und IN_PROGRESS

`current.state` beschreibt nur den Kenntnisstand.

LAST_VERIFIED bedeutet, dass der ausgewiesene
letzte bestätigte Stand bekannt ist, nicht dass
sich die laufende Maschine weiterhin darin befindet.

CURRENT_UNKNOWN bedeutet, dass ein aktueller
Live-Zustand nicht bestätigt wurde.

IN_PROGRESS benötigt eine konkrete Quelle.
Ein alter Snapshot ist kein Beweis, dass ein
Prozess noch läuft.

## 5. BLOCK_FAILED

BLOCK_FAILED ist ein Ereignis, kein Grund zum
Überschreiben von LAST_VERIFIED.

Im v1-Schema wird ein solcher Fall durch
`latest_block.runner=FAIL` oder
`latest_block.technical_result=FAIL` abgebildet.

Die Ursache muss anhand der Originalquelle
geprüft werden.

## 6. Runner, Upload, Fachabnahme

`runner`, `upload`, `technical_result` und
`professional_acceptance` sind getrennte Werte.

Beispiel:

- Runner: FAIL
- Upload: PASS
- Fachabnahme: UNKNOWN

Daraus darf kein erfolgreicher technischer
Block oder neuer qualifizierter Checkpoint
abgeleitet werden.

Der lokale SHA256-Vergleich beweist nicht
automatisch die Authentizität des Archivs.

## 7. Buildstatus

`build.state` wird unabhängig vom Runner erfasst.

Ein Runner-PASS kann zusammen mit
`build.state=RUNNING` auftreten.

Ohne aktuellen Beleg gilt UNKNOWN.

## 8. Publication

Die Veröffentlichung eines neuen Index
ist gesondert zu qualifizieren.

Im vorliegenden v1-Entwurf bezieht sich
`publication` auf den bereits nachgewiesenen
Veröffentlichungszustand. Vor einer ersten
Publikation bleibt NOT_PUBLISHED beziehungsweise
UNKNOWN zu verwenden.

Ein endgültiges Verfahren zur selbstreferenziellen
Bestätigung des Index-Commits ist noch zu entwickeln.

Das Schema stellt keine sichere atomare
GitHub-Veröffentlichung bereit.

## 9. Sicherheitsregeln

- Bestehenden Runner nicht verändern.
- Originalprotokolle nicht überschreiben.
- Keine privaten Schlüssel oder Zugangsdaten.
- Keine automatische Git-Schreibberechtigung.
- Keine Builds, Hardwaretests oder SD-Schreibzugriffe.
- Keine neue VM, API oder MCP-Shell.
- Historische FAIL-Ergebnisse erhalten.
- Fachliche Abnahme nicht aus PASS-Markern ableiten.

## 10. Noch erforderliche Prüfungen

1. Echte historische PASS-/FAIL-Fälle auswerten.
2. Quell- und Commit-Identitäten prüfen.
3. Sicherstellen, dass Fehler LAST_VERIFIED
   nicht überschreiben.
4. Unabhängigen Archiv-Leseweg qualifizieren.
5. Schema gegen echte Archivfälle testen.
6. Veröffentlichungsverfahren getrennt freigeben.

Der heutige Block 129 implementiert ausschließlich
das Datenmodell und seine strukturellen Tests.
