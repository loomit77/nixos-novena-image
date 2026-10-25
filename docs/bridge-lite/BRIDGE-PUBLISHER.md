# Novena Bridge Exchange – Publisher

## Zweck

Separater, noch nicht produktiver Publisher für den
maschinenlesbaren Statusindex.

Archiv: `loomit77/project-block-logs` (privat)

Vorgesehener Zielpfad:

`novena/status/NOVENA-CURRENT-STATE.json`

## Unterstützte Betriebsarten

- `--self-test`
- `--dry-run`
- `--dry-run --previous-file DATEI`
- `--dry-run --remote-check`

Der Remote-Check nutzt ausschließlich `gh api` mit GET.
Er benötigt GitHub CLI, Leseberechtigung und einen
bereits vorhandenen, bestätigten Remote-Index.

Ein fehlender Index, Zugriffsfehler und Netzwerkfehler
werden nicht als erfolgreicher Remote-Test ausgegeben.

## Schutzregeln

- Vollständige JSON-Schema-Validierung.
- Keine automatische Checkpoint-Hochstufung.
- `LAST_VERIFIED` darf nicht geändert oder entfernt werden.
- Keine unbestätigten Veröffentlichungsclaims.
- Keine Schreibzugriffe auf das GitHub-Archiv.
- Keine Änderung des bestehenden Uploadskripts.
- Keine Git-Commits oder Git-Pushes.

## Noch nicht implementiert

- Produktive Erstveröffentlichung.
- Atomare GitHub-Compare-and-Swap-Aktualisierung.
- Remote-Locking und parallele Schreibzugriffe.
- Qualifikation eines neuen Checkpoints.
- Rückleseverifikation nach einer Veröffentlichung.
- Automatische Ausführung.
- Zwei-Dateien-Transaktion.

Der Publisher ist ein Dry-Run-Prototyp. Vor dem
produktiven Einsatz sind ein gesondertes Audit,
eine Sicherheitsprüfung und eine ausdrückliche
Freigabe erforderlich.

## Nachtrag Block 154 (2026-10-25)

Die vorbereitete Initialisierungslogik im Gate verwendet die bestehenden
Publisher-Validatoren und die kanonische Serialisierung. Der Publisher bleibt
selbst ein Dry-Run-Werkzeug. Lokales atomisches CAS und bytegenaues Git-Readback
sind synthetisch testbar; Produktionstransport und unabhängiger API-Readback
bleiben gesperrt beziehungsweise als separater Schritt entworfen.
Details, Produktionsvoraussetzungen und die Inkompatibilität des älteren
`--remote-check` mit einem NOT_PUBLISHED-Payload stehen im
[Gate-Nachtrag Block 154](BRIDGE-PUBLISH-GATE.md#block-154--vorbereitete-publikationslogik-2026-10-25).
