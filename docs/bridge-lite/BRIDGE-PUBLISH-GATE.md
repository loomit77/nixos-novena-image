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
