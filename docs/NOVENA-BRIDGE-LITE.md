# NOVENA-BRIDGE-LITE – Bedienung und Architektur

Stand: 2026-10-25
Status: lokaler Pilot, technische Teilqualifikation PASS.

## 1. Zweck

NOVENA-BRIDGE-LITE erstellt eine kompakte Übersicht
des Novena-Projekts aus vorhandenen lokalen Quellen.

Die Bridge ersetzt weder Git noch Originalprotokolle,
Testberichte oder fachliche Entscheidungen.

## 2. Dateien

Generator:

`scripts/novena-bridge-lite.py`

Ausgaben:

- `docs/PROJECT-STATUS.md`
- `docs/bridge-lite/current-state.json`

Planungsgrundlage:

`docs/NOVENA-BRIDGE-LITE-PLAN.md`

## 3. Datenfluss

Lokaler Git-Zustand
          |
Lokale Runner-Protokolle und Sidecars
          |
Image-Inventur
          |
          v
novena-bridge-lite.py
          |
          +-- JSON-Snapshot
          |
          +-- Markdown-Übersicht

Der Generator startet keine Builds und greift
nicht auf die Novena-Hardware zu.

## 4. Bedienung

Synthetische Tests:

`python3 -B scripts/novena-bridge-lite.py --self-test`

Snapshot-Erzeugung:

`python3 -B scripts/novena-bridge-lite.py --generate`

Die Snapshot-Erzeugung verändert die zwei
vorgesehenen Ausgabedateien.

Sie darf nur innerhalb eines ausdrücklich
freigegebenen Arbeitsblocks erfolgen.

Technische Arbeitsblöcke werden mit dem
bestehenden run-and-upload.bash-Wrapper
protokolliert und archiviert.

## 5. Bedeutungen

COMPLETE:
Die vom Parser geforderten Abschlussmerkmale
und Integritätsbedingungen sind erfüllt.

REVIEW_REQUIRED:
Die Quelle benötigt weitere Prüfung.

IN_PROGRESS_OR_INCOMPLETE:
Die Quelle ist noch nicht abgeschlossen
oder ihr Abschluss konnte nicht belegt werden.

UNKNOWN:
Ein Sachverhalt ist durch die verfügbaren
Quellen nicht nachgewiesen.

PASS:
Nur innerhalb des konkret geprüften
technischen Umfangs interpretieren.

Ein Runner-PASS ist keine automatische
fachliche Abnahme.

## 6. Quellen und Evidenz

Die lokale SHA256-Prüfung vergleicht den Inhalt
einer TXT-Datei mit dem vorhandenen Sidecar.

Ein übereinstimmender Hash bestätigt nicht
automatisch die unabhängige Vertrauenswürdigkeit
oder Vollständigkeit der Quelle.

Die Bridge bestätigt gegenwärtig keinen
GitHub-Upload unabhängig.

Die verarbeiteten Protokolle bilden nur
einen begrenzten Ausschnitt des Gesamtarchivs.

## 7. Snapshot-Veröffentlichung

JSON und Markdown enthalten eine gemeinsame
Snapshot-ID.

Die Ausgabedateien werden jeweils einzeln
durch vorbereitete Dateien ersetzt.

Die Veröffentlichung beider Dateien ist
noch keine echte atomare Gesamttransaktion.

Bei einem Widerspruch der Snapshot-IDs
dürfen die Dateien nicht gemeinsam als
konsistenter Stand behandelt werden.

## 8. Grenzen

- Keine automatische fachliche Abnahme.
- Keine automatische Releasefreigabe.
- Keine Hardware- oder Imageidentität ohne Beleg.
- Keine eigenständige Git-Synchronisierung.
- Keine automatische Veröffentlichung.
- Keine automatische Tagesabschlussintegration.
- Kein dauerhafter Hintergrunddienst.
- Keine zusätzliche VM erforderlich.

## 9. Qualitätsnachweise

Block 122:
Erstimplementierung und lokaler Pilot PASS.

Block 123:
Härtung wegen Patch-Kontextfehler FAIL.

Block 123R:
Überarbeitung und synthetische Tests PASS.

Originale und SHA256-Sidecars:
Privates GitHub-Repository project-block-logs.

## 10. Weitere Arbeit

Zuerst die Ergebnisinterpretation und
Snapshot-Konsistenz vollständig qualifizieren.

Danach über kontrollierte Git-Integration
und den möglichen späteren Betrieb entscheiden.

Die bestehende Bridge-Lite-Planung bleibt
maßgeblich. Keine automatische Erweiterung
des Arbeitsumfangs.
