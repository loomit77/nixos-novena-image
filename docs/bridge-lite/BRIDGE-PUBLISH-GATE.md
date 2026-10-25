# Novena Bridge Exchange – Publication Gate

## Zweck

Lokaler synthetischer Test der Git-basierten Erstveröffentlichung
von `novena/status/NOVENA-CURRENT-STATE.json`.

Das künftige private Zielarchiv lautet:
`loomit77/project-block-logs`.

## Aktueller Status

- Produktionsmodus deaktiviert.
- Ausschließlich `--self-test` verfügbar.
- Nur markiertes lokales Bare-Git-Testarchiv.
- Kein Zugriff auf den echten GitHub-Archivpfad.
- Kein Projekt-Commit und kein Projekt-Push.

## Sicherheitsregeln

- JSON-Schema Draft 2020-12 validieren.
- Initialer LAST_VERIFIED bleibt null.
- Veröffentlichungsclaim bleibt NOT_PUBLISHED.
- Erwarteten Ausgangs-HEAD prüfen.
- Ausschließlich den definierten JSON-Pfad hinzufügen.
- Kein Force-Push oder erzwungener Refspec.
- Remote-HEAD und Datei nach lokalem Push zurücklesen.
- Doppelte Initialisierung und veralteten HEAD ablehnen.

## Grenzen

- Keine produktive GitHub-Veröffentlichung.
- Keine automatische Checkpoint-Qualifikation.
- Keine produktive Konflikt- oder Berechtigungsprüfung.
- Keine Zwei-Dateien-Transaktion.
- Die lokale Simulation ersetzt keinen echten
  GitHub-Test und keine Veröffentlichungsfreigabe.

Produktive Veröffentlichung erfordert ein separates
Audit und ausdrückliche Freigabe.

## Block 154 – vorbereitete Publikationslogik (2026-10-25)

Der Produktionseinstieg `production_publish()` lehnt bedingungslos ab,
auch bei Änderung der Konstante `PRODUCTION_ENABLED`. Es gibt keinen
Produktionsschalter in CLI, Umgebung oder Konfiguration. Der bestehende
Uploadrunner bleibt unverändert.

Die bestehende Initialisierungslogik verwendet nun die Schema-, Claim- und
Serialisierungsfunktionen des Publishers. Initialer Checkpoint und
`publication.commit` bleiben null, der Payload bleibt NOT_PUBLISHED.
Ein vorhandener Zielpfad (auch Baum oder Symlink) wird abgelehnt.
Der Commit muss genau einen Elterncommit mit dem erwarteten vollständigen
HEAD und genau eine Änderung enthalten: Hinzufügen des Zielpfads.

Nur im markierten temporären Bare-Testarchiv werden Commitobjekte lokal
übertragen und `main` mit `git update-ref NEW EXPECTED_OLD` atomar gesetzt.
Dies ersetzt den früheren synthetischen Push: Ein normaler Fast-Forward-Push
allein garantiert keinen exakten Ausgangs-HEAD, etwa nach einem konkurrierenden
Rewind. Es gibt keinen Force-Push, Retry, Merge, Rebase oder Konfliktausgleich.
Dies ist ein lokales CAS-Modell, kein implementierter GitHub-Transport.
Git-Unterprozesse erlauben nur das lokale File-Protokoll; globale/systemweite
Git-Konfiguration und geerbte GIT-Variablen werden ausgeschlossen.
Testarchive und ihre lokale Konfiguration müssen vertrauenswürdig sein.

Rücklesen prüft den HEAD vor und nach dem bytegenauen Lesen des Blobs am
unveränderlichen Commit. Der Rückgabebeleg enthält außerhalb des Payloads:
Ausgangs-HEAD, resultierenden Commit, Zielpfad, SHA256 und Verifikationsart.
Kein zusätzlicher Beleg wird im Archivcommit abgelegt. Der Aufrufer muss diesen
Beleg später separat dauerhaft protokollieren. Der Kandidat 149 bleibt
unverändert; `latest_block=146R` gilt weiter nur für die ausgewählten neun
Evidenzpaare. Keine automatische Checkpoint-Promotion.

### Separate unabhängige GitHub-API-Verifikation – nur Entwurf

Nach künftig separat freigegebener Produktionstransaktion soll ein unabhängiger
Leser ohne Wiederverwendung des Publisher-Checkouts folgende GETs ausführen:

1. `repos/loomit77/project-block-logs/git/ref/heads/main`: SHA muss exakt dem
   außerhalb des Payloads protokollierten Publikationscommit entsprechen.
2. `repos/loomit77/project-block-logs/contents/novena/status/NOVENA-CURRENT-STATE.json?ref=COMMIT`:
   Typ file und Encoding base64 verlangen, Inhalt strikt dekodieren und die
   vollständigen Bytes sowie SHA256 mit dem freigegebenen Payload vergleichen.
3. Branch-Ref erneut lesen und denselben Commit verlangen. Commit-Eltern und
   Dateidiff unabhängig auf erwarteten Ausgangscommit und genau die eine
   Dateihinzufügung prüfen.

Fehler, fehlende Dateien, Berechtigungsprobleme, unerwartete API-Formate oder
abweichender HEAD ergeben keine Bestätigung. Kein automatischer Wiederanlauf.
Zeitpunkte, API-Belege, Commit und Payloadhash getrennt sichern. Die Prüfung
belegt nur ihren Beobachtungszeitraum; spätere Branchänderungen bleiben möglich.
Keine API-Anfrage wurde in Block 154 ausgeführt.

### Produktionsvoraussetzungen und Grenzen

Erforderlich bleiben Inhaltsfreigabe, unabhängiges Audit, ausdrückliche
Veröffentlichungsfreigabe und qualifizierter Transport mit serverseitig
atomarer Prüfung des exakt erwarteten alten HEAD. Ein gewöhnlicher Push oder
eine vorangestellte HEAD-Abfrage reicht dafür nicht. Dieser Transport ist
noch nicht implementiert. Keine Produktiv- oder Berechtigungsqualifikation.
Der ältere `publisher --remote-check` erwartet einen CONFIRMED-Payload und
ist deshalb **nicht** der neue unabhängige Readback dieses NOT_PUBLISHED-Payloads.
Das muss vor späterer produktiver Update-Unterstützung gesondert geklärt werden;
aktuell werden Updates immer abgelehnt. Kein Zwei-Dateien-Verfahren.

Nach einer erfolgreichen Ref-Änderung kann die Rückleseprüfung fehlschlagen;
dann könnte der Commit bereits veröffentlicht sein. Kein Rollback und kein
Retry: separat untersuchen. CAS erkennt keine zwischenzeitliche Änderung,
die vor der Transaktion exakt zum selben OID zurückkehrt (ABA). Für eine
strengere Ereignisgarantie wären serverseitige Sperren/Auditdaten erforderlich.

Lokale Tests:

```sh
PYTHONDONTWRITEBYTECODE=1 python scripts/novena-bridge-publication-test.py
PYTHONDONTWRITEBYTECODE=1 python scripts/novena-bridge-publish-gate.py --self-test
PYTHONDONTWRITEBYTECODE=1 python scripts/novena-bridge-publisher.py --self-test
```

Sechs fokussierte Tests prüfen Kandidat/Provenienz/Readback, doppelte und stale
Initialisierung, Claims/Schema/HEAD-Format, konkurrierenden Advance und Rewind,
zusätzliche Commitdateien, falsche Readbackbytes/HEAD sowie die bedingungslose
Produktionssperre. Die bestehenden Selftests decken zusätzlich Checkpointschutz ab.
