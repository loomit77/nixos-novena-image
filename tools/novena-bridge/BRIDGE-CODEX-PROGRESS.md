# Novena Bridge – Entwicklungsfortschritt

## Aktueller Checkpoint – MASTER_RUN=217, 2026-10-26

Arbeitsblock MASTER-217; keine globale INDEX-Nummer beansprucht.
Erster vollständig geprüfter Teil: lokaler Dateibündeladapter und CLI
prepare-local-sources, drei neue Integrationstests PASS (positiver E2E-Pfad,
15 harte Negativvarianten und sieben Review-/Konfliktvarianten).
LOCAL_SOURCE_FILES_V1 bindet explizite absolute Pfade, Quellen-IDs, Rollen,
erwartete Original-SHA256 und Runner-Blockkennungen an LOCAL_SOURCE_MERGE_V1.
Manifestoriginal und dessen Hash stehen getrennt in der Provenienz; alle
Original-/Sidecarbytes bleiben erhalten. Keine mtime-/Verzeichnisauswahl.
Nur latest_block wird projiziert, bestehender Checkpoint bleibt unverändert.
Fehlende Originale, explizit benannte fehlende Sidecars, Hash-/Blockabweichung,
unvollständige Bündel und doppelte IDs ergeben harte Fehler ohne stdout.
Null-Sidecar, ungültige/abweichende Sidecars, unvollständige Logs und
konfliktäre doppelte Blockquellen bleiben getrennte REVIEW_REQUIRED-Befunde.
Vertauschte Git-/Inventurrollen werden als ungültige Beobachtungen markiert;
auch korrekt geformte Angaben beweisen keine reale Projekt-/Hardwarezuordnung.
Symlinks in allen Pfadkomponenten, Verzeichnisse und FIFO werden abgewiesen.
Linux-open nutzt O_NOFOLLOW/O_NONBLOCK; konkurrierender Austausch von
Elternverzeichnissen benötigt weiterhin eine externe Dateisystem-Vertrauensgrenze.
Andere Plattformen werden vom Dateiadapter abgewiesen.

Erster gezielter Testlauf: ein Positivtest PASS, zwei Tests FAIL: widersprüchliche
Block-ID war bereits harter Fehler statt Review; Sandbox verbot Unix-Socket-
Erzeugung. Testerwartung korrigiert und FIFO statt Socket verwendet; danach
alle drei Tests PASS. Keine private Archivquelle gelesen, nur synthetische
Dateien innerhalb tests/ angelegt und nach den Tests entfernt.
Schlussprüfungen und finale Dokumentation folgen; kein äußerer Runner gestartet.

### Schlussprüfung und Abschluss Run 217

Nach allen Ruständerungen tatsächlich ausgeführt, alle PASS:
- cargo test --locked --offline: 62 Entwicklungstests (59 bestehende + 3 neue).
- cargo fmt --all -- --check: PASS.
- cargo clippy --locked --offline --all-targets -- -D warnings: PASS.
Alle Befehle mit --manifest-path tools/novena-bridge/Cargo.toml aus dem
Repository; keine Abhängigkeiten verändert oder heruntergeladen.
Die sieben Konfliktvarianten enthalten sechs Reviewfälle und einen harten
Block-ID-Konflikt. Bei allen 16 harten Varianten wurde leeres stdout geprüft.
Checkpointpromotion bleibt durch bestehenden Status-/Transitionsschutz und
durch Ablehnung zusätzlicher Manifestfelder gesperrt. Variante A und strikte
Git-Objektprüfungen unverändert, gesamte Regression PASS.

Geändert ausschließlich unter tools/novena-bridge/:
src/local_sources.rs (neu), tests/local_sources.rs (neu), src/lib.rs,
src/main.rs, README.md, BRIDGE-CODEX-PROGRESS.md.
CARGO_TARGET_DIR extern: /tmp/novena-master-217.NuIv5Umn/target.
HEAD vorher/nachher df28dc50be1eea15339a8510a4aa450eb6b15b99;
Git-Index-SHA256 vorher/nachher
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Externer Git-Status gegenüber Eingang unverändert. Keine synthetischen
Restdateien, target-/pyc-/__pycache__-Dateien in der Crate gefunden.
Kein äußerer Runner; Werkzeugausgaben ersetzen kein vollständiges dauerhaftes
Runner-Rohprotokoll. Keine Netzwerkanfragen oder realen privaten Logarchive.

MASTER_RESULT=PASS im beauftragten lokalen Offline-Entwicklungsumfang.
M1/M2/M3 Entwicklungstests PASS; M4 synthetischer Umfang PASS;
M5 expliziter lokaler End-to-End-Entwicklungsumfang PASS.
Keine automatische Fachabnahme und keine unabhängige Runner-Abnahme behauptet.
Offen außerhalb dieses Entwicklungsabschlusses: unabhängige Runner-Abnahme,
authentische Erhebungs-/Zuordnungsevidenz und reale externe Qualifikation.
Korrekt geformte, aber sachlich falsche caller-supplied Zuordnungen sind aus
diesen Bytes nicht entscheidbar und bleiben ausdrücklich unbeglaubigt; kein
abhängiger Build-/Hardware-/Checkpointschluss wird daraus implementiert.
Automatische mtime-/Verzeichnisbeschaffung ist keine verbleibende M5-Pflicht.
Nächster Schritt: getrennte unabhängige Abnahme mit eigener Evidenz/Freigabe;
keine weiteren Entwicklungsänderungen für diesen lokalen Auftrag erforderlich.
GIT_COMMIT=NO; GIT_PUSH=NO; BRIDGE_PUBLICATION=BLOCKED.

---

## Historischer Checkpoint – MASTER_RUN=216, 2026-10-26

Arbeitsblock MASTER-216, keine globale INDEX-Nummer beansprucht.
Teilcheckpoint Quellenvertrag: src/sources.rs und vier gezielte Tests PASS.
Reine lokale API mit expliziter Logauswahl statt mtime-Heuristik; Quellenrollen,
IDs, Originalbytes/base64, SHA256 und Sidecar werden separat erhalten.
Git-/Inventurangaben bleiben im lokalen Provenienzbericht außerhalb Exchange-v1.
Konflikt gleicher Blockkennung unterdrückt Runner-PASS; fehlende/ungültige
Angaben und Inventurwarnungen erzeugen REVIEW_REQUIRED. Kein Build-, Hardware-,
Upload- oder Fach-PASS, keine Checkpointänderung. Nur latest_block wird projiziert.
Keine automatische Verzeichnisauswahl und kein Import vorgefertigter Lite-Snapshots.
Fünf Quellentests und ein neuer CLI-Transitionstest mit elf Negativvarianten
qualifizieren diesen lokalen Umfang. Quellen-IDs, Originalbytes und Sidecars
bleiben separat erhalten; Git-/Inventurhashes behaupten keine Integritätsprüfung.
Inventurzähler werden auf innere Widersprüche geprüft, ohne daraus Erfolg abzuleiten.

Zweiter Teilcheckpoint: verify-update-offline als strikter lesender Adapter für
verify_update_index abgeschlossen. Genau die sechs Initialbündelfelder plus
baseline_base64; alte Bytes werden an alten Blob/Commit gebunden, Kandidat wird
mit Checkpointerhalt vorbereitet. Beide rohe Commits, vollständige Trees/Blobs,
Parentlink, Nebenobjekterhalt und HEAD vor/nach Prüfung werden geprüft.
Neue CLI-Negativfälle: Zusatz-/fehlendes Feld, ungültige und byteabweichende
Baseline, HEAD-/CAS-Konflikt, fehlender alter Rohcommit, gekürzter neuer Rohcommit,
fehlender alter Blob, unvollständiger Tree, Checkpointpromotion sowie vollständig
neu gehashte Nebenobjektänderung. Keine Teil-stdout-Ausgabe bei Ablehnung.
Bestehende synthetische CAS-/Race-/Timeouttests bleiben PASS. Ein statisches
CLI-Bündel beweist keinen zeitlichen HEAD-Verlauf; diese Prüfung bleibt injiziert.
Snapshotbeleg und Transitionsbericht sind getrennte Verträge, gegenseitige
Bündelverwechslung negativ geprüft. Kein Mutationsadapter ergänzt.

### Schlussprüfung und Ergebnis Run 216

Alle vorgeschriebenen Checks nach den letzten Ruständerungen PASS:
- cargo test --locked --offline: 59 Tests (26 Lib, 2 CLI-Unit, 3 CLI,
  5 Logs, 3 Objekte, 14 Protokoll, 1 Schemaoracle, 5 Quellen).
- cargo fmt --all -- --check: PASS.
- cargo clippy --locked --offline --all-targets -- -D warnings: PASS.
Keine Testfehler oder Clippy-Befunde. Cargo.toml/Cargo.lock unverändert.
Externer CARGO_TARGET_DIR: /tmp/novena-master-216.QMoyn8Zd/target.
HEAD vorher/nachher df28dc50be1eea15339a8510a4aa450eb6b15b99;
Index-SHA256 vorher/nachher
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Externer Git-Status gegenüber Eingang unverändert. Keine target-/pyc-/
__pycache__-Dateien in der Crate gefunden. Kein äußerer Runner gestartet;
Werkzeugausgaben sind kein dauerhaftes vollständiges Runner-Rohprotokoll.

Geändert ausschließlich unter tools/novena-bridge/:
src/sources.rs (neu), tests/sources.rs (neu), src/lib.rs, src/main.rs,
src/protocol.rs, tests/protocol.rs, README.md, BRIDGE-CODEX-PROGRESS.md.

MASTER_RESULT=PARTIAL; MASTER_TEST_RESULT=PASS.
M1/M2/M3 PASS; M4 PASS im synthetischen Umfang; M5 weiterhin PARTIAL.
Lokaler Quellen-/Mergevertrag und strikter Update-CLI-Adapter sind im beschriebenen
Entwicklungsumfang abgeschlossen. Weiter offen sind automatische lokale
Verzeichnisbeschaffung und End-to-End-collect/snapshot-Orchestrierung einschließlich
CLI-Quellenbündeladapter. Die explizite Auswahl ist eine bewusste konservative
Vertragsänderung gegenüber Lite, keine qualifizierte mtime-Neueste-Ermittlung.
Git-/Inventurquellen sind caller-supplied; kein authentifizierter Erhebungsweg,
kein automatisch verifizierter Bezug zu Build, Image oder Hardware. Lite-Snapshots
werden nicht als Originale importiert. Keine externe Runner-Abnahme, kein echter
Remote-Transport/Readbacknachweis und keine produktive Veröffentlichung qualifiziert.
Variante A bleibt erhalten; Payload und separater Verifikationsbeleg bleiben getrennt.

Nächster Schritt: gezielt lokale Quellenbeschaffung/lesenden Bündeladapter auf
LOCAL_SOURCE_MERGE_V1 aufsetzen und separat End-to-End qualifizieren, wenn dieser
weitergehende Integrationsumfang beauftragt wird. Keine erneute Gesamtinventur.
Externe Qualifikation benötigt eigene Evidenz/Freigabe; kein technischer Test
ersetzt sie. Kein Download, Netzwerk, SSH, Hardware-/SD-Zugriff, Commit, Push,
Reset, Clean, Retry/Rollback oder automatische fachliche Freigabe ausgeführt.
GIT_COMMIT=NO; GIT_PUSH=NO; BRIDGE_PUBLICATION=BLOCKED.

---

## Historischer Checkpoint – MASTER_RUN=215, 2026-10-26

Arbeitsblock MASTER-215; keine globale INDEX-Nummer beansprucht.
M1/M2/M3 PASS; M4 PASS im synthetischen Umfang; M5 PARTIAL.
Variante A ausdrücklich durch den Auftrag freigegeben. Historische Aussagen
über eine noch ausstehende Entscheidung gelten nur für Run 214 und früher.

Abgeschlossener Implementierungsteil: separater typisierter Offline-Beleg
in src/attestation.rs mit privater Konstruktion nach unabhängigem Readback.
Repository/ID, Ref, Commit, Blob, SHA256 und tatsächliche geprüfte Bytes
werden verbunden; HEAD wird vorher/nachher geprüft. Zeit/Bedingungen sind
explizit caller-supplied, keine beglaubigte Uhr oder Remote-Frische.
Keine Statusbehauptung wird zum fachlichen PASS hochgestuft.
NOT_PUBLISHED/null bleiben unveränderte Payloadstatuswerte und sagen nichts
über eine spätere externe Verifikation des umschließenden Commits aus.
Dieser neue Belegvertrag ist ausdrücklich NICHT Python validate(remote=True).
Python und externe Schemata bleiben unverändert.

Erster Prüflauf: cargo test --locked --offline PASS, 49 Tests inklusive
zwei neuer Integrationstests mit Python-SHA256-Oracle und Negativvarianten.
Danach Fortschreibungs-/CAS-Integration abgeschlossen; Schlussprüfung siehe unten.
Fortschreibungsplanung bindet alte Originalbytes an den gehashten Baseline-
Commit/Tree/Blob und erhält Checkpoint und Geschwister. Nur der feste Zielpfad
ist ersetzbar. CLI prepare-update-objects erhält zusätzlich baseline_base64;
prüft außerdem separate alte Blobantwort und HEAD vor/nach Planung.

### M5-Teilcheckpoint: Fortschreibung und Offline-CLI abgeschlossen

Neue Transitionprüfung verify_update_index prüft beide rohe Commits,
Parentlink, alte und neue Trees/Blobs, exakte Baseline, Checkpointerhalt und
nur den erlaubten Zielpfad. simulate_update_graphql nutzt dieselbe Prüfung
vor dem einzigen synthetischen Versuch sowie unabhängigen Readback danach.
Tests: CAS-Konflikt mit null Aufrufen; Timeout/ungültige Antwort mit genau
einem; HEAD-Race im nachfolgenden Readback ergibt MutationUnknown.
Siblings, zusätzliche leere Verzeichnisse, Modusänderung, fehlender alter
Blob, falscher Parenthash und Baselinebytes werden negativ geprüft.
Nicht-null-Checkpoint bleibt erhalten; Löschen/Ersetzen wird abgewiesen.

CLI prepare-update-objects und verify-attestation sind lesend integriert.
Letzteres hat einen strikten eigenen Bündelvertrag und private Belegkonstruktion.
E2E-Tests prüfen beide CLI-Befehle, keine Teil-stdout-Ausgabe im Fehlerfall,
bytegenaue Belegidentität sowie alle erzeugten Updateobjekte gegen unabhängiges
`git --no-optional-locks hash-object` ohne -w. SHA256 gegen Python hashlib.
Bestehende initiale CLI-/Python-Referenzverträge bleiben erhalten.
Kein CLI-Bündeladapter für die vollständige Update-Transitionprüfung;
verify-attestation ist bewusst nur ein Snapshotbeleg und ersetzt sie nicht.

Schlussprüfungen nach allen Ruständerungen tatsächlich PASS:
- cargo test --locked --offline: 53 Tests (26 Bibliothek, 2 CLI-Unit,
  3 CLI-Integration, 5 Logs, 3 Objekte, 13 Protokoll, 1 Schemaoracle).
- cargo fmt --all -- --check: PASS.
- cargo clippy --locked --offline --all-targets -- -D warnings: PASS.
Die zusätzlichen sechs Protokolltests enthalten mehrere Negativvarianten.
Ein Editieraufruf verwendete zunächst fälschlich repository-relative Pfade
im Crate-Arbeitsverzeichnis: FileNotFound, keine Teiländerung; danach mit
korrektem Arbeitsverzeichnis ausgeführt. Keine fehlgeschlagenen Rusttests
oder Clippy-Befunde in diesem Lauf.

Quellenprüfung: scripts/novena-bridge-lite.py collect/image_inventory/snapshot
sowie exchange.create_index und publish-gate.payload ausgewertet. Lite erhebt
Git separat und fasst Inventurzähler zusammen; hardware_image_identity bleibt
UNKNOWN. Exchange-v1 besitzt keine Git-/Inventurfelder für diese Summen und
setzt build UNKNOWN/source null. Eine eindeutige automatische Zuordnung von
Inventur zu Build oder Hardware ist daraus NICHT ableitbar. Quellenmerge mit
separater Provenienz und Auswahl-/Konfliktregeln bleibt offen; kein erfundener
fachlicher Buildnachweis, keine Änderung am externen Schema.

Externer CARGO_TARGET_DIR aus Umgebung: /tmp/novena-master-215.c3uQmdq6/target.
HEAD df28dc50be1eea15339a8510a4aa450eb6b15b99; Index-SHA256 zu Beginn
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Keine Abhängigkeiten geändert. Kein äußerer Runner (Schreibgrenze Crate).
Keine externe Runner-Abnahme, Veröffentlichung oder reale Remote-Evidenz.

### Abschluss und exakter Fortsetzungspunkt

MASTER_RESULT=PARTIAL; MASTER_TEST_RESULT=PASS.
M5 bleibt PARTIAL wegen fehlender collect/snapshot-Quellenzusammenführung,
Git-/Inventurprovenienz und vollständiger CLI-Update-Transitionintegration.
Nächster Lauf: nur aktuellen Checkpoint und betroffene Module lesen;
separaten lokalen Provenienz-/Mergevertrag aus vorhandenen Referenzen
entwerfen, keine Inventurwerte als Build-/Hardware-PASS übernehmen.
Optional danach verify_update_index als strikten CLI-Bündeladapter anbinden;
Snapshotbeleg und Transitionsschutz ausdrücklich getrennt halten.
Keinen produktiven Transport, Retry, Rollback oder Publikationspfad ergänzen.
Externe Runner-Abnahme bleibt separat und unbehauptet.

Geändert (alle unter tools/novena-bridge/): src/attestation.rs (neu),
src/lib.rs, src/main.rs, src/objects.rs, src/planning.rs, src/protocol.rs,
tests/cli.rs, tests/protocol.rs, README.md, BRIDGE-CODEX-PROGRESS.md.
Cargo.toml/Cargo.lock unverändert, keine neue Abhängigkeit.
Abschließende lesende Kontrolle: HEAD und Git-Index-SHA256 identisch zum
Eingang; externer Git-Status unverändert. Keine target-/__pycache__-/pyc-
Dateien unter der Crate gefunden (rg ohne Treffer, Exit 1 ist hier erwartet).
Nur die oben genannten Cratedateien wurden in diesem Lauf bearbeitet.
GIT_COMMIT=NO; GIT_PUSH=NO; BRIDGE_PUBLICATION=BLOCKED.

---

## Historischer Checkpoint – MASTER_RUN=214, 2026-10-26

Arbeitsblock MASTER-214; keine neue globale INDEX-Nummer beansprucht.
M1 PASS; M2 PASS; M3 PASS; M4 PASS (synthetischer Umfang);
M5 PARTIAL; M6 PASS (historische Abschlussinventur aus Run 213).
MASTER_RESULT=PARTIAL; MASTER_TEST_RESULT=PASS.
Letzter abgeschlossener Entwicklungsteil: reiner Rust-Runner-Logeingang.
Die nachfolgenden Run-213-Abschnitte bleiben als historische Übergabe erhalten;
für die nächste Fortsetzung ist dieser neue Checkpoint maßgeblich.

### M5-Teilcheckpoint: Logeingang abgeschlossen

Neu: `src/logs.rs` mit reinem Byteparser, separater SHA256-Sidecarprüfung,
begrenztem lesendem Dateiadapter und lokaler `prepare_from_log`-Anbindung.
CLI `inspect-log FILE` gibt ausschließlich den Prüfbericht auf stdout aus.
Keine Originaldateien oder Prüfsummen werden geschrieben. Keine
Verzeichnissuche und kein Zugriff auf reale Runnerarchive erforderlich.

Vollständigkeit verlangt Blockkennung, gültigen Ergebniswert PASS/FAIL,
dezimalen Exitstatus 0..255, widerspruchsfreien LOG_COMPLETE=YES und
Integrität PASS. Konflikte werden auch in LOG_COMPLETE und UPLOAD_RESULT
erkannt; widersprüchliche Ergebnis-/Exitstatuspaare, fehlende Felder und
ungültiges UTF-8 ergeben REVIEW_REQUIRED und process_result=UNKNOWN.
Identische wiederholte Werte bleiben zulässig. Sidecar und Parser prüfen
dieselbe einmal gelesene Bytefolge; Sidecar fehlt/ungültig/Hashabweichung
bleiben getrennte MISSING/INVALID/FAIL. Finalen Symlinks und Spezialdateien
wird der Dateieingang verweigert, Größenlimits 32 MiB bzw. 8 KiB.
I/O-/Größenfehler des Logs brechen mit Fehler ohne Teil-stdout ab.

Bewusste konservative Abweichungen zur Python-Referenz: keine fachliche
Übernahme von BLOCK_RESULT; dessen letzte Behauptung steht nur im separaten
claimed_block_result. Technical, Professional und Upload bleiben UNKNOWN.
Unvollständige Logs werden einheitlich REVIEW_REQUIRED statt teilweise
IN_PROGRESS_OR_INCOMPLETE. Hashgleichheit belegt weder Prozesswahrheit noch
Vollständigkeit außerhalb des unterstützten Markerformats. Logmarker sind
unbeglaubigte Textangaben, auch wenn sie in Befehlsausgaben eingebettet sind.

`prepare_from_log` akzeptiert nur lokale geprüfte Statusdokumente und
Originalbytes. Es ändert ausschließlich latest_block; last_verified,
current, publication und sonstige Felder bleiben unverändert. Fehlende
Blockkennung wird abgewiesen. Keine automatische technische/fachliche
Freigabe, keine Remote-Vertragsänderung und keine Checkpointpromotion.

SHA256-Abhängigkeit vor Cargo-Änderung lokal unter
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sha2-0.10.9`
gefunden und Manifest geprüft; `cargo generate-lockfile --offline`
erfolgreich. `sha2 = "=0.10.9"` ergänzt, Cargo.lock offline neu aufgelöst.
Keine Downloads, Installationen oder eigene Kryptographieimplementierung.

47 Entwicklungstests PASS: bisherige 42 plus 5 neue Log-Testfunktionen
mit mehreren Negativvarianten. Python-Selftestfixture wird per AST lesend
aus der unveränderten Referenz extrahiert, ihr SHA256 unabhängig mit Python
berechnet; alle fünf Referenzfälle nachgebildet. Zusätzlich SHA256-abc-Vektor,
Abschluss-/ID-/Exitkonflikte, fehlende Pflichtfelder, ungültige Ergebniswerte,
Exitgrenzen, Integritätsfehler, falsche Dateinamen, mehrzeilige Sidecars,
UTF-8-/Größenfehler, Upload-PASS/Uploadkonflikt, Checkpointerhalt und lesende
CLI mit dauerhaftem synthetischem TXT-/Sidecarfixture geprüft.
Originalbytes von Fixture und Sidecar nach CLI-Aufruf erneut verglichen.
Ein erster Clippy-Lauf meldete zwei reine Teststilbefunde (unused import,
unused mut); gezielt korrigiert, danach sämtliche Schlussprüfungen PASS:

```bash
cargo test --locked --offline
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

CARGO_TARGET_DIR unverändert aus Umgebung übernommen:
`/tmp/novena-master-214.23S94nlU/target`. Keine Test-Tempdateien angelegt;
neue synthetische Testfixtures liegen ausschließlich innerhalb der Crate.
Kein äußerer Runner gestartet, da dessen Schreib-/Uploadziele nicht
freigegeben sind. Werkzeugausgaben sind kein vollständiges Runner-Rohlog.

Geänderte/ergänzte Dateien dieses Laufs (alle unter tools/novena-bridge/):
Cargo.toml, Cargo.lock, src/lib.rs, src/main.rs, src/logs.rs, tests/logs.rs,
tests/fixtures/SYNTHETIC.txt, tests/fixtures/SYNTHETIC.txt.sha256,
README.md, BRIDGE-CODEX-PROGRESS.md.

### Nächsten offenen Punkt geprüft: Publikationsfeldvertrag, Entscheidung offen

Primäre Evidenz: `scripts/novena-bridge-publisher.py`, validate(remote=True)
verlangt CONFIRMED und commit != null. Dagegen erzeugt der lokale Vertrag
in `scripts/novena-bridge-publish-gate.py`, payload(), ausschließlich
NOT_PUBLISHED mit null-Commit. Der bestehende Rust-Initialpfad erhält diese
Payloadbytes und bestätigt Objektidentitäten separat. Diese Aussagegrenzen
bleiben unverändert; es gibt hier keine produktive Vertragsentscheidung.

Konkrete Alternativen zur ausdrücklichen späteren Entscheidung:

| Alternative | Bedeutung und Auswirkungen |
|---|---|
| A: Unveränderte Payload plus separater Verifikationsbeleg | NOT_PUBLISHED/null bezeichnet nur die fehlende Selbstbestätigung der Payload. Ein unabhängiger Bericht bindet Repository, Ref, Commit-OID, Blob-OID und exakte Bytes sowie Beobachtungsbedingungen. Remote-Validator muss später in Payload- und Belegprüfung getrennt werden; Schema-/Benennungssemantik und Migration sind ausdrücklich zu klären. Keine Selbstreferenz, passend zum bereits vorhandenen Initial-Readback. |
| B: Separates Bestätigungsdokument im Folgecommit | Erster Commit enthält unveränderte Payload; zweiter Commit enthält eine Bestätigung des ersten Commits, außerhalb dessen Payload. Zusätzliche Dokument-/Pfad-Allowlist, zwei CAS-Schritte und Zustände für halbfertige Bestätigung nötig. Bestätigung betrifft ausdrücklich den ersten, nicht ihren eigenen Commit; realer Zusatzschritt und Fehlermodell werden komplexer. |
| C: CONFIRMED/commit in der Payload referenziert ausdrücklich einen früheren Commit | Vermeidet Selbstreferenz, behauptet aber nur eine historische Veröffentlichung. Feldbedeutung, verifizierte Vorgängerbytes und Aktualitätsdarstellung müssen geändert und unabhängig geprüft werden. Ohne klare Trennung kann die alte Bestätigung fälschlich den aktuellen Inhalt beglaubigen. |

Begründete Empfehlung, noch keine Entscheidung: A. Sie erhält die exakten
bereits gehashten Payloadbytes und trennt Inhaltsaussagen von dem erst
nach Veröffentlichung möglichen Identitäts-/Readbackbeleg. Sie benötigt
weniger neue Mutationszustände als B und vermeidet die Mehrdeutigkeit
historischer Bestätigungen aus C. Ob NOT_PUBLISHED als Feldname erhalten
bleibt oder in einer versionierten Schemaänderung ersetzt wird, muss
ausdrücklich Teil derselben Entscheidung sein. Keine Variante darf eine
erfolgreiche Veröffentlichung durch bloßes Setzen eines Feldes bestätigen.

Das nachträgliche Eintragen des eigenen umschließenden Commit-OID in
dieselbe Payload ist keine tragfähige Alternative: geänderte Blobbytes
ändern Blob-, Tree- und Commitidentität; ein weiterer Commit löst diese
Selbstreferenz nicht. Diese Begründung entscheidet nicht zwischen A/B/C.

### Exakter Fortsetzungspunkt und verbleibender Umfang

1. Diesen Checkpoint, Git-Zustand und nur betroffene Module lesen.
2. Remote-Publikationsfeldvertrag ausdrücklich entscheiden lassen;
   keine davon abhängige allgemeine Indexfortschreibung anfangen.
3. Weiter offen und unabhängig planbar: Git-/Image-Inventurzuordnung und
   Quellenzusammenführung einschließlich expliziter Provenienz. Reiner
   Logeingang ist fertig, vollständige Python-snapshot/collect-Kette nicht.
4. Nach Vertragsentscheidung: bestehende Indexdatei aus verifizierten alten
   Blobbytes fortschreiben, Nebenobjekte erhalten, Baseline-/CAS-/Racefälle
   negativ prüfen. Reale Transport-/Runner-Abnahme bleibt separat offen.

Keine Git-Mutation, Netzwerkanfrage, SSH-, Hardware-/Medienaktion,
Veröffentlichung, Retry oder Rollback. Bestehende fremde Änderungen erhalten.
Schlussprüfung: HEAD weiterhin df28dc50be1eea15339a8510a4aa450eb6b15b99,
Git-Index-SHA256 weiterhin
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Externer Git-Status entspricht den eingangs vorhandenen Einträgen.
Keine target-Verzeichnisse oder Python-Bytecode-Dateien in der Crate gefunden.
GIT_COMMIT=NO; GIT_PUSH=NO; BRIDGE_PUBLICATION=BLOCKED.

---

MASTER-Auftrag: NOVENA-BRIDGE-CODEX-MASTER, MASTER_RUN=213, 2026-10-26.
Arbeitsblock: MASTER-213 (keine neue globale INDEX-Nummer beansprucht).

Letzter abgeschlossener Meilenstein: M6 (Abschlussinventur; M5 bleibt PARTIAL).
M1 PASS; M2 PASS; M3 PASS; M4 PASS (synthetischer Umfang); M5 PARTIAL; M6 PASS.
MASTER_RESULT=PARTIAL. MASTER_TEST_RESULT=PASS.

## M6 – Abschlussinventur und exakte Übergabe

Erreicht: initialer Statusindex wird in Rust syntaktisch, strukturell und
fachlich geprüft, kanonisch vorbereitet, in einen Git-Objektplan überführt
und über einen getrennten Offline-Antwort-/Rohcommitvertrag verifiziert.
CAS-Konflikte und unklare Mutationsausgänge werden ausschließlich
synthetisch modelliert. Kein Gesamtabschluss der gesamten Quellen- und
Fortschreibungsintegration behauptet.

Schlussprüfungen erfolgreich, Arbeitsverzeichnis jeweils tools/novena-bridge:

```bash
cargo test --locked --offline
cargo test --locked --offline --quiet
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

Letzte Suite: 42 Tests (26 Bibliothek, 2 CLI-Unit, 3 CLI-Integration,
3 Objekt-, 7 Protokoll-/Planungs-/Bündeltests, 1 Schemaoracle).
Darunter 699 unabhängige Schemafälle, 81 Statuskombinationen,
11 strikte CLI-Negativvektoren je fünf Eingangswegen, 26 integrierte
Objekt-/REST-Negativvarianten, Headerinjektion, Baselinekonflikte und
Mutationsunklarheit. Schlussreview ergänzte Statusgrößenschutz auch beim
Value-Eingang und die erneute Nullcheckpointprüfung am initialen Readback.
Die letzten beiden Ergänzungen sind mit gezielten Negativassertionen
abgedeckt; anschließend volle Suite und Clippy erneut PASS.

Toolchain: rustc/cargo 1.99.1; verwendete std-APIs ab Rust 1.88.
CARGO_TARGET_DIR unverändert:
/tmp/novena-bridge-master-213.MwhKpPDT/target.
Keine Buildartefakte innerhalb der Crate gefunden.
Git-HEAD und Index am Ende unverändert gegenüber dem oben protokollierten
Ausgangspunkt. Git-Status außerhalb der Crate zeigt dieselben bestehenden
Einträge. Keine eigenen Projektänderungen außerhalb der Crate, keine
Git-Mutation und keine Netzwerk-/SSH-/Hardware-/Medienaktion ausgeführt.
Cargo.toml und Cargo.lock unverändert, keine Abhängigkeit hinzugefügt.

### Vollständiges Änderungsinventar dieses Laufs

- src/lib.rs: Module/Fehlertyp, Schema-LF-Korrektur und ihr Test,
  Formatierung; A1-Kontrollfluss unverändert (Clippy-Attribut).
- src/main.rs: sichere ausschließlich lesende CLI-Integration.
- src/status.rs: immutable Statusmodell, Schutz-/Vorbereitungsregeln.
- src/objects.rs: strikte OIDs, Gitbytes, Hashes, Tree-/Commitprüfung.
- src/protocol.rs: injizierte REST-Leser, base64, Readback, CAS-Modell.
- src/planning.rs: initiale reine Objektplanung und Bündeladapter.
- tests/schema_oracle.rs: unabhängiger Schema-Abgleich.
- tests/objects.rs: Python-/Git-Vektoren, Manipulations-/Profilfälle.
- tests/protocol.rs: vollständige synthetische Integration und Differenzen.
- tests/cli.rs: Prozessgrenzen, Negativfälle und Publikationssperre.
- README.md: Schnittstellen, Bündelformate und Nachweisgrenzen.
- BRIDGE-CODEX-PROGRESS.md: dieser fortlaufende Bericht.

### Technischer Restaufwand, bekannte Risiken und Nachweisgrenzen

1. **M5 offen: Rust-Quelleneingang.** Der Python-Lite-Parser für Runnerlogs,
   Konflikt-/Vollständigkeitsprüfung, SHA256-Sidecars und die Zuordnung
   zu Git-/Image-Inventurdaten sind noch nicht als Rust-Kette integriert.
   Aktuell werden vollständige vorgegebene Statusdokumente verarbeitet;
   deren Evidenzdateien und fachliche Angaben sind damit nicht beglaubigt.
   Für SHA256 ist eine geprüfte offline verfügbare Bibliothek zu prüfen;
   noch kein Bedarf an einer konkret fehlenden Cargo-Abhängigkeit festgestellt.
   Keine Downloads oder eigene Kryptographieimplementierung auf Verdacht.
2. **M5 offen: existierenden Index fortschreiben.** Dokumentvergleich schützt
   den Checkpoint, Objektplanung/Readback erlauben bislang nur eine initiale
   Hinzufügung. Nötig sind bytegenau verifizierte alte Blobbytes, konsistente
   Baseline-/Kandidatenprüfung, ausschließlich diese eine geänderte Datei,
   unveränderte Nebenobjekte/Verzeichnisse und CAS-/Race-Negativfälle.
3. **Ungeklärter Referenzvertrag für spätere Remote-Statuswerte.**
   Python publisher.validate(remote=True) verlangt CONFIRMED plus commit,
   gate.payload() erzeugt dagegen NOT_PUBLISHED plus null im initialen Blob.
   Ein eigener umschließender Commit-OID kann nicht einfach nachträglich
   in dieselben schon gehashten Statusbytes eingetragen werden.
   Der Rust-Initialpfad erhält die ursprünglichen Bytes und legt das Ergebnis
   in den separaten Prüfbericht. Vor einer allgemeinen Remote-Fortschreibung
   muss dieser Vertrag ausdrücklich entschieden werden; kein stilles Raten
   und keine produktive Anwendung der widersprüchlichen Übergangsannahmen.
4. **Transport/Originalbytes.** Kein echter HTTP- oder Rawcommit-Beschaffer
   implementiert. REST-Metadaten allein sind kein Commit-Hashnachweis.
   Spätere Beschaffung braucht einen eigenen freigegebenen Leseweg; solange
   Originalbytes fehlen, bleibt die starke Prüfung fail-closed. Reine
   REST-Objekterzeugungsanforderungen/-antworten sind noch nicht zu einem
   Offline-Uploadadapter verbunden; die Planung liefert rohe Objektbytes.
5. **Bewusste Profilgrenzen.** Signierte/Merge-/Encoding-Commits,
   Nicht-UTF8-Pfade und zu große Archive werden abgewiesen. Vor einer
   Erweiterung konkrete Primärbeispiele und neue diskriminierende Tests
   vorsehen. Die festen Budgets beweisen keine reale Archivabdeckung.
6. **Format-/Quellvertrauen.** Schaltsekunden nur durch Rust-Grenztests
   abgedeckt; unabhängig ergänzte Python-Datumsprüfung ist kein vollständiger
   RFC3339-Beweis. Offline-Bündel können Inhalt/Hashrelationen, aber keine
   reale Remote-Frische oder Authentizität ihrer Beschaffung beweisen.
   Finale Symlinkprüfung der CLI ist kein Betriebssystem-Sandboxersatz und
   bietet keine atomare Garantie gegen konkurrierenden Dateiaustausch.
7. **Externe Abschlussqualifikation offen.** Alle hier genannten Prüfungen
   sind Codex-Entwicklungstests. Keine externe Runner-Abnahme, kein realer
   GitHub-Schreib-/Readbacknachweis, kein neuer Chat-Nutzungsnachweis und
   keine Hardware-/Imagequalifikation. Der äußere Runner wurde wegen der
   ausschließlich auf die Crate begrenzten Schreibfreigabe nicht gestartet;
   kein vollständiges dauerhaftes Runner-Rohlog dieser Entwicklung erzeugt.
8. **Produktivsperre bleibt dauerhaft in diesem Stand.** Keine echte
   Veröffentlichung, Credentialanbindung, automatische Checkpointfreigabe,
   Retry- oder Rollbackautomation ergänzen. Eine spätere menschlich geprüfte
   Checkpointänderung ist kein vorhandener automatischer Codepfad.

### Exakte Fortsetzung für nächsten Lauf

1. Nur diesen Bericht, Git-Zustand und die betroffenen Rust-Module lesen;
   keine erneute breite Projektinventur. Unveränderten Index und externes
   CARGO_TARGET_DIR prüfen; vorhandene Nutzeränderungen erhalten.
2. Als nächste unabhängige M5-Teilaufgabe den reinen Rust-Logeingang anhand
   scripts/novena-bridge-lite.py implementieren: unvollständige/konfliktäre
   Logs UNKNOWN/REVIEW_REQUIRED, SHA256-Integrität separat, Upload und
   Fachabnahme ohne externe Evidenz UNKNOWN. Keine Originaldateien ändern.
   Offline-Verfügbarkeit einer SHA256-Abhängigkeit vor Cargo-Änderung prüfen;
   falls sie fehlt, Bedarf dokumentieren und nichts herunterladen.
3. Danach den oben benannten Publikationsfeldvertrag klären, bevor davon
   abhängige allgemeine Indexfortschreibung implementiert wird. Bis dahin
   bestehenden initialen Objekt-/Readbackvertrag und CP-Schutz erhalten.
4. M5-Lücken mit Offline-Differenz-/Regressionsprüfungen schließen,
   `cargo test --locked --offline` und erforderliche Checks ausführen,
   Fortschrittsbericht aktualisieren. Externe Runner-Abnahme folgt getrennt.

GIT_COMMIT=NO; GIT_PUSH=NO; BRIDGE_PUBLICATION=BLOCKED.

## M5 – Integration, Objektplanung und CLI (PARTIAL)

Sichere lesende CLI mit stdin/Datei-Eingang, Größenbegrenzung,
finaler Symlink-/Spezialdateisperre und keiner Teil-stdout-Ausgabe bei
Validierungsfehlern. Neue Befehle: prepare-initial, prepare-status,
prepare-from-archive, prepare-objects, plan-cas, verify-offline.
Alle Publish-Varianten bedingungslos vor Dateizugriff gesperrt.

Initiale Objektplanung vollständig im Speicher: Baseline-Commit hashen,
alle alten Trees prüfen, nötige Unterbäume ergänzen, Geschwister erhalten,
Blob-/Tree-/Commitbytes deterministisch erzeugen und eigenen Treevergleich
prüfen. Identity-Headerinjektion wird abgelehnt. Ausgabe sind nur Bytes/OIDs
und ein CAS-Anforderungsentwurf. Kein Git-Objekt wird gespeichert.

Offline-Bündelprüfung und Vorbereitung jeweils per CLI End-to-End getestet.
Direkter Vergleich gegen Python-Publisher, Publish-Gate und Transport-
Allowlist ergänzt; bestehende Python-Dateien bleiben unverändert.
README innerhalb der Crate beschreibt Befehle, Bündelformate und Grenzen.
Cargo.toml/Cargo.lock unverändert, keine neuen Laufzeitabhängigkeiten.

Erfolgreich zuletzt: `cd tools/novena-bridge && cargo test --locked --offline --test protocol`
(7 Tests). Davor volle Suite mit 41 Tests PASS; vollständige Schlussprüfung
nach dem letzten Vergleichstest folgt noch. `cargo clippy --locked --offline
--all-targets -- -D warnings` war vor den letzten Testergänzungen PASS.
Die anfänglichen Clippy-Stilbefunde wurden ausgewertet; A1-Scannerlogik
bleibt unverändert, ausschließlich lokales allow-Attribut ergänzt.

Zusätzlich geändert: src/planning.rs, src/protocol.rs, src/main.rs,
src/lib.rs, tests/cli.rs, tests/objects.rs, tests/protocol.rs, README.md,
dieser Bericht. Formatierung der Rust-Dateien innerhalb der Crate.

Noch nicht vollständig verbunden: Python-Lite-Quellenableitung aus
Originalprotokollen/SHA256-Sidecars und Git-/Image-Inventurdaten hat noch
keinen Rust-Eingangsadapter. Die Offline-Objektprüfung unterstützt die
initiale Hinzufügung, nicht die spätere Aktualisierung eines existierenden
Index. Statusfortsetzung ist bislang nur auf Dokumentebene implementiert.
Darum M5 PARTIAL und kein Gesamt-PASS.

## M4 – Offline-REST, GraphQL und unabhängiges Readback

REST-Allowlist auf ein Repository, main-Ref und immutable OID-Routen;
keine rekursive Tree-Route. Strikte JSON-Eingänge, Request-/Bytebudgets,
Normalisierung erlaubter size/url-Metadaten. Blob-base64 mit Größen-/
Hashprüfung, fail-closed bei nichtkanonischen Paddingbits.
Readback prüft Repository-ID, HEAD vor/nach der Prüfung, beide rohen
Commit-Identitäten und Links, alle Trees und exakt erwartete Blobbytes.
Rohe Commitbytes sind eine ausdrücklich getrennte injizierte Quelle;
sie werden niemals aus REST-Metadaten rekonstruiert.

Reiner GraphQL-updateRefs-Anforderungsbau mit beforeOid/afterOid und
force=false; synthetischer injizierter Ablauf prüft Objekte vor dem
einzigen Versuch und erneut unabhängig danach. Vorabkonflikt schreibt
nicht. Timeout, GraphQL-Fehler/Teilergebnis, malformed response oder
Readbackfehler danach ergeben MutationUnknown. Kein Retry/Rollback.
production_publish ist bedingungslos gesperrt; kein echter Transport.

Erfolgreich: `cd tools/novena-bridge && cargo test --locked --offline`
(26 Bibliotheks-, 2 CLI-Unit-, 3 Objekt-, 3 Protokoll-, 1 Oracle-Test).
Protokolltests enthalten 26 integrierte Negativvarianten und mehrere
CAS-/Allowlist-/Transport-/base64-Fälle; ausschließlich in-memory.
Zusätzlich geändert: src/protocol.rs, tests/protocol.rs, src/lib.rs, Bericht.
Fortsetzung: M5, sichere lesende CLI für Vorbereitung/Offline-Readback,
Regressionen und Nutzungsdokumentation innerhalb der Crate.

## M3 – Git-Objekte

Strikter OID-Typ, Blob-/Tree-/Commit-SHA1 einschließlich Git-Header,
UTF-8-Namen und Git-Verzeichnissortierung. Alle erreichbaren Trees werden
nichtrekursiv abgefragt und bytegenau gehasht; vollständiger Pfadvergleich
erlaubt nur die initiale Statusdatei und ihre nötigen Verzeichnisse.
Zusätzliche leere Verzeichnisse werden ebenfalls abgelehnt (Härtung
gegenüber der Python-Referenz). Traversierung ist mengen-/tiefenbegrenzt.
Rohe Commitbytes werden ohne Rekonstruktion geprüft, schmales unsigned
Profil mit null/einem Parent; neue Commits müssen genau den erwarteten
Parent und Tree referenzieren. Signaturen/Merge-/Encodingheader bleiben
absichtlich nicht unterstützt und werden abgewiesen.

Python-Tree-/Commit-Vektoren direkt lesend wiederverwendet; Rust-Tree und
Commit zusätzlich gegen `git --no-optional-locks hash-object -t TYPE --stdin`
ohne -w geprüft. Kein Git-Objekt oder Repositoryzustand geschrieben.
Tampering, falsche Links, Omission, Namen, Modi, Duplikate, zusätzliche
Header/Parents und falsche OIDs negativ geprüft.
Erfolgreich: `cd tools/novena-bridge && cargo test --locked --offline`
(26 Bibliotheks-, 2 CLI-Unit-, 3 Objekt-, 1 Oracle-Test).
Zusätzlich geändert: src/objects.rs, tests/objects.rs, src/lib.rs, Bericht.
Fortsetzung: M4, normalisierender REST-Lesevertrag, unabhängiges Readback,
GraphQL-CAS als reine Anforderung und synthetisches Zustandsmodell.

## M2 – Statusmodell und Schutzregeln

Unveränderliches geprüftes Statusmodell mit getrennten Runner-/Upload-/
Technik-/Fachresultaten. Vorbereitung erhält last_verified exakt;
auch viermal PASS erteilt keine Checkpointfreigabe. Initialstatus verlangt
null-Checkpoint. Falsche Publikations-/Livezustandsbehauptungen sowie
fachlich ungültige OIDs werden abgelehnt. Bytegenauer Baselinevergleich.
81 Ergebniskombinationen, Checkpoint-Löschung/-Ersetzung, falsche
Publikation und Konflikte getestet. Statuswerte sind übernommene Angaben,
keine automatische Beglaubigung ihrer Originalquellen.

Erfolgreich: `cd tools/novena-bridge && cargo test --locked --offline`
(26 Bibliotheks-, 2 CLI-Unit-Tests, 1 Oracle-Test mit 699 Fällen).
Zusätzlich geändert: src/status.rs, src/lib.rs, dieser Bericht.
Fortsetzung: M3, rohe Git-Objekte und verifiziertes Tree-/Commit-Readback.

## M1 – unabhängiger Schema-Abgleich

699 mutierte Dokumente gegen das tatsächliche Schema mit installiertem
Python-jsonschema geprüft. Keine Python-Datei verändert, Python mit -B.
Gefundene Korrektur: Schema-Regex `$` akzeptiert abschließendes LF bei
40-stelligen Hexwerten; rein strukturelle Rust-Prüfung daran angepasst.
Die fachliche/Protokollprüfung muss OIDs weiterhin strikt ablehnen.
Bestehender Negativvektor dafür wurde explizit in einen Schema-Positivvektor
umgewandelt; keine Änderung an JSON-Parser oder kanonischen Pilotbytes.
Die installierte jsonschema-Umgebung hat keinen date-time-Checker;
Oracle ergänzt für gewöhnliche Sekunden einen separaten stdlib-Checker.
Schaltsekunden bleiben durch vorhandene Rust-Grenztests abgedeckt,
nicht durch diesen unabhängigen Format-Oracle. Kein vollständiger RFC-Beweis.

Erfolgreich: `cd tools/novena-bridge && cargo test --locked --offline --test schema_oracle -- --nocapture`.
Vorherige volle Suite: 25 Bestandstests PASS, Oracle zunächst FAIL;
Ursachen (fehlender Python-Formatchecker, Regex-Differenz, Python akzeptiert
ISO-24:00) ausgewertet und gezielt korrigiert. Volle Regression folgt M2.
Geändert: src/lib.rs, tests/schema_oracle.rs, dieser Bericht.

Ausgangszustand geprüft: Branch main, HEAD
df28dc50be1eea15339a8510a4aa450eb6b15b99; Git-Index SHA256
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Vorhandene fremde Änderungen: TODO.md, docs/CHAT-STARTER.md,
docs/DECISIONS.md, docs/NOVENA-BRIDGE-LITE-PLAN.md; fünf unversionierte
Python-Referenzmodule und gesamte Rust-Crate. Diese werden erhalten.

## Sicherheitsgrenzen

Nur tools/novena-bridge/ ist für Projektänderungen freigegeben.
Buildartefakte ausschließlich im bereits gesetzten externen CARGO_TARGET_DIR.
Kein Netzwerk, SSH, Hardware-/Medienzugriff, Download, Installation,
Git-Mutation, produktives Publish oder automatische Checkpointfreigabe.
Kein äußerer Runner gestartet: dessen externe Schreib-/Uploadziele sind
nicht freigegeben. Werkzeugausgaben sind kein vollständiges Runner-Rohlog.
Codex-Tests sind Entwicklungstests; externe Abschlussqualifikation offen.

## Fortsetzung

Maßgeblich ist die exakte Übergabe in M6 oben; Schlussprüfungen sind PASS.
Im nächsten Lauf gezielt M5-Lücken bearbeiten, keine erneute Gesamtinventur.
Keine neue Abhängigkeit herunterladen; Cargo.lock und vorhandene
Abhängigkeiten bevorzugen. Detaillierte nächste Schritte folgen in M6.

---

## Persistent Workflow M1 – Engineering Continuation Checkpoint – 2026-10-26

**M1_RESULT=COMPLETE (Planung und Dokumentation); LIVE_HTTPS=NOT_IMPLEMENTED;
BRIDGE_PUBLICATION=BLOCKED.** Dieser angehängte Checkpoint ist der aktuelle
Wiedereinstieg für den persistenten Workflow. Das frühere „M1 – unabhängiger
Schema-Abgleich“ und M2–M6 bezeichnen historische Entwicklungsphasen, nicht
die neuen Workflow-Meilensteine. Die gesamte bisherige Historie bleibt erhalten.
Ältere Hinweise auf fehlenden Logeingang, Quellenadapter, Updateplanung oder
ungeklärte Variante A sind durch die späteren Implementierungen überholt.

### Auftrag, durchgeführte Arbeit und Evidenzbasis

M1 umfasst ausschließlich diese Dokumentergänzung; keine Rust-Quelldatei,
Cargo.toml, Cargo.lock oder andere Projektdatei wurde geändert. Git-Zustand
mit `git --no-optional-locks` gelesen; vorhandene Nutzeränderungen und die
unversionierte Rust-Crate bleiben erhalten. Gelesen wurden AGENTS.md,
Repository-README, Bridge-README, bisheriger Fortschrittsbericht,
docs/NOVENA-BRIDGE-LITE-PLAN.md, relevante Bridge-Entscheidungen in
docs/DECISIONS.md sowie alle Rust-Module, Integrationstests und Cargo-Verträge.
Projektstatus/Testlog auf relevante Bridge-Verweise geprüft; unter den
Dateinamen in test-logs/, evidence/ und research/ keine zu Bridge/274/275
passenden Rohbelege gefunden. Das ist eine begrenzte Suche, kein Beweis,
dass außerhalb dieser Auswahl keine Protokolle existieren.

Vom Auftrag vorgegebener jüngster Prüfstand: **Block 274: alle 66 Rust-Tests
PASS. Block 275: echte HTTPS-Implementierung durch fehlenden vollständigen
HTTP-/TLS-Abhängigkeitssatz im lokalen Cargo-Cache blockiert.** Diese Angaben
werden als übergebene Ergebnisse fortgeführt; M1 hat ihre Rohlogs nicht
erneut qualifiziert und den Cache nicht erneut vollständig inventarisiert.
Statische Zählung ergibt passend 66 `#[test]`-Funktionen: 26 Bibliothek,
2 CLI-Unit, 3 CLI-Integration, 3 lokale Quellen, 5 Logs, 3 Objekte,
18 Protokoll, 1 Schemaoracle und 5 Quellenmerge. Das ist kein Testlauf.
Die historischen 42/59/62-Testangaben bleiben historische Prüfpunkte.

Keine Builds, Tests, Formatierung, Installation, Netzwerkabfrage, Downloads,
SSH-, Git-Mutation, Veröffentlichung oder Hardware-/Medienaktion in M1.
Keine neue Python-Implementierung und kein Transport über Python, curl,
openssl-Unterprozesse oder Shell als Ersatz für Rust.

### 1. Bereits vorhandene Rust-Funktionen wiederverwenden

| Bereich / Einstieg | Implementierter Umfang und Aussagegrenze |
|---|---|
| src/lib.rs | Strikter UTF-8-/Integer-JSON-Parser, doppelte dekodierte Schlüssel und Tiefe begrenzt; Exchange-v1-Schema und RFC3339-Grenzfälle; separate allgemeine canonical_json-Funktion ist kein strikter Statusprüfer. |
| src/status.rs | Geprüfte unveränderliche Statuswerte, initialer Nullcheckpoint, Fortsetzung mit unverändertem last_verified, getrennte Runner-/Upload-/Technik-/Fachergebnisse; keine automatische Promotion. |
| src/logs.rs | Begrenztes Lesen und Parser für Original-TXT plus optionalen SHA256-Sidecar; Konflikte/unvollständige Logs ergeben Review/UNKNOWN. Hashgleichheit beglaubigt weder Prozessablauf noch Upload. |
| src/sources.rs, src/local_sources.rs | Explizite Runner-/Gitbeobachtungs-/Inventurquellen, Originalbytes und Manifest/SHA256 in separater Provenienz, Auswahl statt mtime-Heuristik; nur latest_block wird projiziert. Linux-Dateiadapter weist Symlinks/Spezialdateien ab; konkurrierender Austausch von Elternverzeichnissen bleibt externe Vertrauensgrenze. |
| src/objects.rs | Git-Blob-/Tree-/Commit-Identitäten, vollständige begrenzte Treeprüfung einschließlich leerer Verzeichnisse, Pfade/Modi und exakte Ein-Datei-Änderung; schmales unsigned Root-/Ein-Parent-Commitprofil, kein Merge-/Signatur-/Encodingprofil. |
| src/planning.rs | Deterministische initiale Hinzufügung und Ersetzung des bestehenden Statusindex im Speicher; exakte Baselinebytes, Rohcommit/REST-Metadatenbindung, Geschwistererhalt und HEAD-Prüfung; Objektbytes/OIDs plus CAS-Entwurf, keine Objektspeicherung. |
| src/protocol.rs | REST-Allowlist, striktes JSON/base64, Repository-/Refbindung, unabhängige Initial-/Update-Readbackprüfung mit Rohcommits, Parentlinks, Trees und exakten alten/neuen Blobbytes; nur injizierte Quellen. |
| src/attestation.rs | Variante A: unveränderte NOT_PUBLISHED/null-Payload und separater VARIANT_A_OFFLINE_V1-Beleg nach Snapshotprüfung; Zeitpunkt/Bedingungen CALLER_SUPPLIED, keine Quellenauthentifizierung. Snapshot ist kein Transitionsnachweis. |
| src/protocol.rs, src/main.rs | Reiner GraphQL-updateRefs-CAS-Entwurf (beforeOid/afterOid, force=false), synthetische Initial-/Updateabläufe mit höchstens einem Versuch und MutationUnknown bei Unklarheit; produktive Publish-Befehle vor Eingangslesen bedingungslos gesperrt. |
| src/main.rs | Lesende CLI für Schema, Status, Quellen, Objektplanung, Offline-Snapshot/Transition und CAS-Plan; Ausgabe erst nach vollständiger Prüfung, keine eigene Dateiveröffentlichung. |

Die manuellen Vorschau-/Viewer-/Shadow-Hüllen sind im Bridge-README und im
Lite-Plan als lokale Teilqualifikation dokumentiert, in M1 nicht ausgeführt.
Sie ersetzen keinen Rust-Transport. Bestehende Tests verwenden teilweise
installiertes Python als Entwicklungsoracle und Git hash-object ohne -w;
das ist kein produktiver Python-Implementierungspfad.

### 2. Exakte verbleibende Live-HTTPS-Lücke

`protocol::ResponseSource::get(endpoint) -> BridgeResult<Vec<u8>>` ist der
Wiederverwendungspunkt für einen späteren Rust-GET-Adapter unter RestReader.
Vor jedem Request gilt `allowed_endpoint`: ausschließlich
`repos/loomit77/project-block-logs`, dessen `git/ref/heads/main` sowie
`git/{commits,trees,blobs}/<strikter OID>`. Keine rekursive Tree-Route,
freien URLs, Queries oder beliebigen Repositories.

`OfflineHttpResponse` und `OfflineHttpSource` prüfen ausschließlich bereits
im Speicher liegende synthetische Envelopes: GET, exakter URL-String
`https://api.github.com/<endpoint>`, 200, keine Location/Redirects,
Body-/Versuchslimits und caller-supplied elapsed. Es gibt keinen DNS-, Socket-,
TLS-, HTTP-Empfangs-, Credential- oder CLI-Netzwerkpfad. elapsed ist keine
laufende Deadline; Bodyprüfung nach Bereitstellung ist kein Empfangslimit.
**Synthetisches HTTP ist kein Live-HTTPS und kein Authentizitätsnachweis.**

Es fehlen der etablierte HTTP-/TLS-Client samt qualifizierter Konfiguration,
kontrollierte Credentialzuführung für das private Archiv, echte Zeit-/Byte-
und Headerlimits während des Empfangs, sichere Fehlerabbildung sowie eine
ausdrücklich freigegebene lesende Integration. RestReader allein begrenzt
Antworten nach Rückgabe auf 8 MiB und Requests auf maximal 256; ein neuer
Empfänger muss zusätzlich ein gemeinsames Gesamtbudget durchsetzen.

Separat offen: `RawCommitSource::raw_commit(Oid)` beschafft derzeit nur
injizierte Originalbytes. JSON von git/commits ist keine rohe Git-Commitdatei.
Ein authentifizierter, OID-gebundener Beschaffungsweg für unveränderte
Commitbytes muss anhand verfügbarer Primärdokumentation und realer Bytes
qualifiziert werden. Keine Rekonstruktion aus API-Identitäten/Zeitstempeln,
kein Patch-/Difftext als Ersatz; bei fehlenden Originalbytes fail-closed.
Dieser Weg ist nicht durch die vorhandene REST-Allowlist automatisch erlaubt.

### 3. Anforderungen an HTTP-/TLS-Bibliothek und Adapter

Dies sind Auswahl-/Abnahmekriterien, keine in M1 geprüfte Aussage über eine
konkrete aktuelle Bibliotheksversion. Keine eigene TLS-/Zertifikats- oder
HTTP-Framing-Implementierung. Ein etablierter Rust-HTTP-Client mit etabliertem
TLS-Backend, etwa ein später zu prüfender rustls-basierter Stack, ist erst
nach Prüfung seiner Primärdokumentation und vollständigen Features auszuwählen.

- Zertifikatskette, Gültigkeitszeit und Hostname api.github.com verbindlich
  prüfen; SNI, sichere Protokollversionen (mindestens TLS 1.2), dokumentierte
  Root-CA-/Truststore-Herkunft und Aktualisierung. Keine insecure-Option,
  permissive Custom-Verifier oder Übernahme beliebiger CA-Umgebungsvariablen.
- Nur intern konstruierte feste HTTPS-Ziele; weder HTTP-Fallback noch
  Redirectverfolgung. Automatische Proxies, Cookie-/Credentialspeicher,
  Redirects und versteckte Retries deaktivieren. Zulässige Header und
  GitHub-Medientyp/API-Version/User-Agent explizit festlegen und später
  gegen Primärquellen qualifizieren, keine Annahme über Originalcommitbytes.
- Monotone Gesamtdeadline über DNS, Connect, TLS, Header und vollständigen
  Body sowie endliche Phasenlimits. Bibliotheksgrenzen gerade bei DNS und
  langsamem Body nachweisen; nur ein Socket-Readtimeout genügt nicht.
- Headergröße/-anzahl, pro Antwort höchstens 8 MiB, gemeinsames Bytebudget
  (Ausgangspunkt 32 MiB), maximal 256 Versuche und Gesamtlaufzeit begrenzen.
  Während des Lesens abbrechen, auch ohne/falsch mit Content-Length oder bei
  Chunking. Kompression zunächst deaktivieren oder Wire- und entpackte Bytes
  getrennt begrenzen. Rohcommits in das gemeinsame Budget einbeziehen und
  ihr bestehendes 1-MiB-Profil erhalten. Limits sind keine Archivabdeckung.
- Ausschließlich vollständige 200-Antworten im GET-Vertrag; 3xx/Location,
  204/206/304, Auth-/Rate-Limit-/Serverfehler, Abbruch und ungültiges Framing
  liefern Fehler ohne automatische Wiederholung oder Teil-Erfolg.
- Credentialvertrag getrennt festlegen: explizite Zuführung mit minimalem
  Leserecht, kein implizites Lesen lokaler Secrets oder CLI-Argument mit Token;
  keine Tokens, Header, Bodies oder private URLs in Debug/Fehlern/Protokollen.
  Bibliotheksfehler auf stabile BridgeError-Codes abbilden. Fehlende Credentials
  sind kein Anlass, Schreibrechte oder alternative Ziele anzufordern.
- Offline- und authentifizierte Erhebung typisiert auseinanderhalten.
  Vorhandene OFFLINE_SUPPLIED_RESPONSES-/VARIANT_A_OFFLINE_V1-Berichte nicht
  umetikettieren; ein späterer Live-Beleg braucht eigenen Herkunftsvertrag.

### 4. Fehlende Dependencies beschaffen und qualifizieren (späterer Auftrag)

Cargo.toml enthält derzeit nur sha1="0.10", sha2="=0.10.9" und
serde_json="1"; Cargo.lock enthält keinen HTTP-/TLS-Stack. Nach Block 275
ist kein vollständiger passender Satz lokal verfügbar. Exakte fehlende
Crates/Versionen sind ohne gewählten Stack und Featuregraph noch nicht
abschließend benennbar; ein erfundener Paket-/Versionsplan wäre unpräzise.

1. Im nächsten freigegebenen Entwicklungsschritt zunächst lesend vorhandene
   Registryquellen, Cache/Index und Toolchain prüfen, ohne Cargo-Auflösung mit
   Nebenwirkungen. Danach Kandidat, Versionen, TLS-Backend, Truststore,
   Crypto-Provider, Zielplattform und minimale Features festlegen. foobox
   ist der Bridge-Host; ARM/Novena-Support wird daraus nicht vorausgesetzt.
2. Vollständige direkte/transitive/runtime/build/test-Abhängigkeiten einschließlich
   TLS-Testfixtures/-Werkzeugen, Plattformanteilen und gegebenenfalls nativer
   Buildvoraussetzungen ermitteln. Bei unvollständigem lokalem Index die genaue
   Auflösung ausdrücklich als noch offen führen, nicht offline vortäuschen.
3. Beschaffung nur mit eigenem Auftrag für Netzwerk/Import und passende
   Schreibziele. Alternativ kontrollierter Offline-Import eines vollständigen
   Pakets; Quelle, Version, Features, Checksummen und Transportprovenienz
   dokumentieren. M1 erteilt diese Freigaben nicht.
4. Primärquellen, Wartungs-/Securitystand, RustSec/Upstream-Advisories, Lizenz,
   MSRV, Feature-/Buildskripte und native Abhängigkeiten prüfen. Cargo-Checksummen
   gegen vertrauenswürdigen Registry-/Lock-Kontext verifizieren; ein selbst
   mitgelieferter Hash allein authentifiziert kein importiertes Paket.
5. In separat autorisiertem Umfang Cargo.toml/Cargo.lock nachvollziehbar
   aktualisieren, vollständigen Featuregraph festhalten und Offline-Build aus
   vollständigem Cache oder vendortem Satz qualifizieren. Kein Umgehen von
   --locked, kein stiller TLS-Backendwechsel und kein Download während Tests.

### 5. Kleine nützliche Rust-Arbeiten ohne neue Dependencies

Nur Vorschläge für einen späteren Quelländerungsauftrag; M1 implementiert nichts:

- Priorität: reine gemeinsame Budget-/Deadline-Policy mit injizierbarer
  monotoner Uhr, Versuchszähler und begrenztem Read-Adapter entwerfen und
  mit kurzen/stückweisen std::io::Read-Eingängen testen. Sie qualifiziert
  Empfangslogik im Modell, noch keinen DNS-/TLS-Abbruch.
- Fehlende Randvektoren ergänzen statt implementierte Parser/Planer neu
  schreiben: 304/429, Fragmente/encodierte Pfade, Größen-/Gesamtbudgetgrenzen,
  Zeitgrenze exakt erreicht, Requestverbrauch bei Fehlern, leere/fehlende
  Rohbytes und unveränderte Checkpoint-/Publikationsfelder. Vorher bestehende
  tests/protocol.rs prüfen, um bereits abgedeckte Fälle nicht zu duplizieren.
- Separaten Rust-Vertrag für Erhebungsprovenienz, sichere Fehler und spätere
  Read-only-Orchestrierung definieren; Offline-Beleg bleibt unverändert.
  Keine Attrappe darf authentifizierte Herkunft oder realen Zeitverlauf melden.

Diese Aufgaben beseitigen den Dependencyblocker nicht. Kein Nachbau von TLS,
keine neue mtime-Auswahl, keine erneute lokale Gesamtimplementierung.

### 6. Prüfplan nach vollständiger Dependencybereitstellung

Reihenfolge und getrennte Evidenzstufen für spätere autorisierte Arbeit:

1. Bestehende 66 Tests als Regression sowie cargo fmt --all -- --check und
   cargo clippy --locked --offline --all-targets -- -D warnings. Tests mit
   `cargo test --locked --offline` im Crate-Verzeichnis und explizitem externem
   CARGO_TARGET_DIR; keinen historischen /tmp-Pfad als noch existent annehmen.
   Lokale Quellentests erzeugen/entfernen synthetische Dateien unter tests/;
   Testfreigabe muss diese Schreibzugriffe umfassen. Vorhandene Oracle-
   Voraussetzungen prüfen, keine Python-Nachimplementierung hinzufügen.
2. Rust-TLS-Integration an kontrolliertem lokalen Testserver mit gesonderter
   Socket-/Testfreigabe und isolierten Test-CAs: gültige Kette/Hostname sowie
   unbekannte CA, falscher Host, abgelaufenes/nicht gültiges Zertifikat,
   Handshakeabbruch und Protokolldowngrade. Test-Truststore darf nicht in die
   Produktivkonfiguration gelangen; kein frei wählbarer Host im Produktivpfad.
3. Echte Netzwerk-Lesegrenzen am Testserver: DNS-/Connect-/Handshake-/Header-/
   Bodytimeout, Slow-Drip, EOF/Trunkierung, Chunking, falsche Längen, zu große
   Header/Bodies, Kompression, Gesamtbudget und Versuchslimits. Keine Redirect-
   Folgeanfrage, keine Proxy-/Retryaktivierung, kein Secret in allen Fehlerwegen.
4. Vorhandene vollständige Git-Fixtures durch den neuen Adapter und dieselben
   RestReader-/Objektprüfer schicken: manipulierte/fehlende Rohcommits,
   Repo-ID/Ref/OID-Abweichung, fehlende/truncated Trees, Blobsubstitution,
   Baseline mit anderem Whitespace, Nebenobjektänderung und HEAD-Races müssen
   weiter scheitern. Snapshot und Transition getrennt prüfen; UNKNOWN/Review,
   Variante A und Checkpointerhalt unverändert. Publish-CLI bleibt gesperrt.
5. Erst mit ausdrücklicher GitHub-Lesefreigabe und Credentialvertrag: begrenzter
   realer GET-Lauf gegen das private Zielrepo, Repository-ID unabhängig pinnen,
   HEAD vorher/nachher, alle verwendeten OIDs und exakten Originalbytes binden,
   Beschaffungsbedingungen und redigierte Evidenz vollständig erhalten. Echte
   Rohcommitbeschaffung separat nachweisen. Ohne Statusziel kein erfolgreicher
   Status-Readback; kein Schreiben eines Testindex als implizite Voraussetzung.
6. Externe unabhängige Gesamtprüfung und neuer Chat: Statusindex plus referenzierte
   Originalprotokolle im vorgesehenen privaten Leseweg tatsächlich lesen,
   Berechtigungen/Verfügbarkeit, Integrität und praktischen Wiedereinstieg prüfen.
   Das ist weder durch Cargo-PASS noch durch lokale TLS-Tests erledigt.

Live-Objekterzeugung und GraphQL-Mutation sind spätere separate Arbeit: es fehlen
qualifizierte Rust-Upload-/API-Adapter und echte CAS-/Readbackevidenz. Keinen
Live-Transport in simulate_graphql/simulate_update_graphql einsetzen. Zuerst
separaten Produktivvertrag mit Preflight, exaktem beforeOid und unabhängigem
Readback entwerfen; erst nach eigener Freigabe qualifizieren. Unklare Mutation
bleibt MutationUnknown, ohne Retry/Rollback oder automatische Freigabe.

### 7. Offene Authentizitätsgrenzen und konkrete Wiederaufnahme

Fakt: Offline-Bündel beweisen Hash-/Link-/Bytebeziehungen innerhalb übergebener
Daten. Repository-ID, gepinnte OIDs, Originalquellen, Beobachtungszeit und
Bedingungen sind derzeit caller-supplied. SHA1 ist Git-Objektidentität,
SHA256 Byteintegrität; beides ist für sich kein Nachweis des Urhebers oder
des tatsächlichen Ablaufs. TLS würde Server-/Kanalvertrauen unter dem gewählten
CA-Modell ergänzen, keine Fachabnahme und keine Hardware-/Imagezuordnung.
Zwei gleiche HEAD-Lesungen beweisen keinen atomaren Snapshot oder lückenlosen
zeitlichen Verlauf (einschließlich zwischenzeitlicher Änderung mit Rückkehr).

Offen bleiben echte Rust-GitHub-Beschaffung, unveränderte Rawcommitbytes,
unabhängiger Readback des Statusindex und referenzierter Originalbelege,
gesicherte private Chat-Lesbarkeit und externe fachliche Gesamtqualifikation.
Historische Python-API-Ergebnisse und lokale Viewer-PASS sind kein Rust-
Live-HTTPS-Nachweis. Das aktuelle Vorhandensein des entfernten Statusindex
wurde in M1 nicht geprüft. Keine automatische last_verified-Promotion,
kein CURRENT-Livebeweis, kein Upload-/Technik-/Fach-PASS aus Transporterfolg.

**Restart-Reihenfolge:** Diesen Checkpoint zuerst lesen, dann lokalen Git-Zustand
und nur betroffene Rust-Schnittstellen prüfen. Alle Nutzeränderungen erhalten.
Die nächste eigenständige Aufgabe ist entweder ein ausdrücklich beauftragter
kleiner Offline-Rust-Policy-/Testschritt aus Abschnitt 5 oder die gesondert
freigegebene Dependencyauswahl/-beschaffung aus Abschnitt 4. Bei weiterhin
fehlendem vollständigem Stack Blocker präzise fortführen; keine Cargoänderung
auf Verdacht. Danach GET-Adapter und Rohcommitweg getrennt implementieren und
die Evidenzstufen aus Abschnitt 6 einzeln abnehmen. Fortschritt künftig mit
konkretem Umfang, ausgeführten Tests, Artefakt-/Quellenprovenienz und Grenzen
anhängen. Dieser Plan ist keine Freigabe für spätere Aktionen.

M1_DOCUMENTATION=COMPLETE; M1_TESTS_RUN=NO; SOURCE_CHANGES=NO;
DEPENDENCY_ACQUISITION=NOT_AUTHORIZED; NETWORK=NO; GIT_MUTATIONS=NO;
HARDWARE=NO; LIVE_HTTPS=BLOCKED_DEPENDENCIES; REAL_READBACK=UNQUALIFIED;
CHECKPOINT_PROMOTION=BLOCKED; BRIDGE_PUBLICATION=BLOCKED.
## NOVENA-BRIDGE-RUST-280-M2 – Implementierung durch fehlenden M1-Stand blockiert

### Vorhandene Bausteine und geprüfte Grundlage

Die lokale Codebasis enthält `ResponseSource`, `RestReader`, die bestehende
REST-Pfad-Allowlist, `OfflineHttpSource` und den getrennten `RawCommitSource`.
`production_publish` bleibt unbedingt gesperrt. Die 69 erfolgreichen Tests
sowie Rustfmt-/Clippy-PASS aus Block 278 sind übergebene Ausgangsevidenz;
sie wurden in diesem Auftrag nicht erneut ausgeführt.

Die konkrete M1-Voraussetzung des Auftrags ist im vorliegenden Arbeitsbaum
nicht vorhanden: `tools/novena-bridge/Cargo.toml` nennt ausschließlich sha1,
sha2 und serde_json. `Cargo.lock` enthält weder reqwest noch rustls; auch die
Abhängigkeitsliste des Bridge-Pakets enthält keinen HTTPS-Client. Die bisherige
Fortschrittsdatei endet mit Block 279 und dessen ausdrücklich noch nicht
implementiertem Live-HTTPS-Transport. Ein abgeschlossener Dependency-Schritt
280-M1 ist hier nicht dokumentiert. Damit besteht ein ungeklärter Widerspruch
zwischen Auftragsvoraussetzung und lokalem Stand.

### Tatsächliche Implementierung und geänderte Rust-Dateien

Keine Rust-Dateien geändert und kein HTTPS-Transport implementiert.
Gemäß AGENTS.md wird vor Änderungen auf widersprüchlicher Grundlage gestoppt.
M2 verbietet Dependencyänderungen und Netzwerkbeschaffung; der fehlende
Dependency-Schritt wird deshalb nicht eigenständig nachgeholt. Ausschließlich
dieser Checkpoint wurde ergänzt; ältere Checkpoints bleiben erhalten.

### Offene Sicherheitsanforderungen und vorbereitete Tests

Sämtliche Anforderungen an den echten Transport bleiben unimplementiert und
ungeprüft: HTTPS-/Hostbindung, GET-/Pfadbindung, Redirect-/Proxy-/Retry-Sperren,
Zertifikats-/Hostnamenprüfung, finite Deadlines sowie Empfangs-, Header-,
Fehlerantwort-, Anfrage- und Gesamtbytegrenzen. Die Offline-Hülle belegt diese
Eigenschaften einer echten Verbindung nicht. Credential-Unterstützung und
private Readback-Qualifikation bleiben gesperrt. REST-Commit-JSON belegt keine
originalen Git-Commitbytes; `RawCommitSource` bleibt live unangebunden.

Keine neuen Testfälle angelegt oder ausgeführt. Nach Bereitstellung des
geprüften M1-Stands sind deterministische Tests für URL-/Methodenrestriktionen,
Header-/Responsefehler, Anfrage-/Bytebudgets, Empfangsüberschreitungen,
fehlende/inkonsistente Antworten und den Erhalt der Publikationssperre zu
ergänzen. Stackabhängige TLS-/Deadline-/Retry-Eigenschaften benötigen eigene
Nachweise; reine synthetische Tests sind dafür kein Live-Nachweis.

### Nächste Schritte

Den zum Auftrag gehörenden M1-Stand mit gepinnter reqwest-/rustls-Auswahl und
lokal verfügbaren Quellen bereitstellen beziehungsweise die abweichende
Arbeitsgrundlage klären. Danach Manifest, Lockgraph und lokale Client-APIs
lesend prüfen und M2 innerhalb seiner unveränderten Schreibgrenzen fortsetzen.
Builds und Tests erst mit gesonderter Freigabe ausführen.

M2_RESULT=BLOCKED_MISSING_M1_DEPENDENCY; RUST_CHANGES=NO;
NEW_TESTS=NO; BUILDS_OR_TESTS_RUN=NO; NETWORK=NO;
DEPENDENCY_CHANGES=NO; LIVE_HTTPS=NOT_IMPLEMENTED;
RAW_COMMIT_ACQUISITION=UNQUALIFIED; LAST_VERIFIED=UNMODIFIED;
CHECKPOINT_PROMOTION=BLOCKED; BRIDGE_PUBLICATION=BLOCKED.

---

## M2 – REST-Gesamtbudget an der Antwortgrenze, 2026-10-26

Fortsetzung auf Grundlage des M1-Plans; dessen Historie bleibt erhalten.
Git-Zustand, AGENTS.md, Repository-/Bridge-README, M1, relevante bestehende
Rustquellen und Protokolltests sowie Bridge-Verweise in Projektstatus,
Testlog und Entscheidungen gelesen. Bestehende Nutzeränderungen erhalten.
Begrenzte Dateinamensuche unter test-logs/, evidence/ und research/ ergab
keine passenden Bridge-/274-/275-Rohbelege; keine neue Qualifikation dieser
historisch übergebenen Testergebnisse.

**Belegte Quelllücke:** RestReader begrenzte vor M2 ausschließlich einzelne
zurückgegebene Antworten auf 8 MiB und Aufrufe auf 256. Das 32-MiB-Limit
von OfflineHttpSource begrenzt dessen bereits geliefertes Transkript;
andere ResponseSource-Implementierungen waren daran nicht gebunden.
Damit fehlte am gemeinsamen Übergang zur REST-JSON-Verarbeitung ein
quellenunabhängiges kumulatives Antwortbudget. Keine neue Transportattrappe.

Exakt in M2 geänderte Pfade:
- tools/novena-bridge/src/protocol.rs
- tools/novena-bridge/tests/protocol.rs
- tools/novena-bridge/BRIDGE-CODEX-PROGRESS.md

RestReader besitzt nun pro Instanz ein festes Gesamtbudget von 32 MiB.
Alle erfolgreich von ResponseSource zurückgegebenen Bytes werden vor
Einzelgrößen-, JSON- und Formprüfung angerechnet, einschließlich abgewiesener
Antworten. checked_sub vermeidet Unterlauf; eine Überschreitung erschöpft
das Budget dauerhaft. Die exakt passende letzte Antwort darf noch geprüft
werden. Nach Erschöpfung wird vor einem weiteren Quellenaufruf mit
RESPONSE_BUDGET_EXCEEDED abgebrochen. Allowlist und Versuchslimit gelten
weiter; nicht erlaubte Pfade verbrauchen weder Versuch noch Antwortbudget.
Quellenfehler verbrauchen weiterhin einen Versuch, liefern aber keine
anrechenbaren Antwortbytes. Rückgaben können nicht als Teil-Erfolg passieren.

Drei fokussierte Rusttests vorbereitet, **nicht ausgeführt**:
- rest_response_budget_accepts_exact_boundary_and_stops_before_next_source_call:
  vier gültige JSON-Objekte von je 8 MiB einschließlich Whitespace, verschiedene
  erlaubte Endpunkte, exakt 32 MiB akzeptiert; keine weiteren Quellenaufrufe
  oder Versuche nach Erschöpfung, Allowlistfehler ohne Quellenaufruf.
- rest_response_budget_rejects_one_byte_over_and_remains_exhausted:
  nach 32 MiB minus einem Byte wird eine Zwei-Byte-Antwort abgewiesen;
  erneuter Aufruf bleibt ohne weiteren Quellenaufruf gesperrt.
- rest_response_budget_charges_rejected_bodies: ungültiges JSON,
  falsche JSON-Form und Antworten über dem Einzelgrößenlimit verbrauchen
  kumulatives Budget; nachfolgende Aufrufe umgehen die Erschöpfung nicht.

Validierungsstand: ausschließlich manuelle Quell-/Testdurchsicht und lesende
Zustandskontrolle. Kein cargo test, Build, Formatter oder Clippy ausgeführt;
keine Aussage über Kompilierung oder Test-PASS. Bestehender vollständiger
Offline-Readbacktest und Git-Objektprüfer werden wiederverwendet, nicht dupliziert.

Grenzen und verbleibende Arbeit: Dies begrenzt die Weiterverarbeitung bereits
zurückgegebener REST-Bytes. Es begrenzt weder Allokation in ResponseSource
noch Netzwerkempfang, Header, Laufzeit oder verborgene Bytes eines Quellenfehlers.
RawCommitSource bleibt außerhalb dieses Budgets. Ein späterer echter Empfänger
muss weiterhin ein gemeinsames Empfangsbudget einschließlich Rohcommits und
echte monotone Deadlines durchsetzen. HTTP-/TLS-Dependencies, Credentialvertrag,
authentifizierte Originalcommitbeschaffung und reale lesende GitHub-Qualifikation
bleiben gemäß M1 offen. Offline-Belege behalten ihre bisherige Herkunftsgrenze.
Nächster freizugebender Schritt: vorbereitete Tests und bestehende Regression
offline ausführen, Format-/Clippyprüfung; anschließend getrennte Fortsetzung
des M1-Transportplans. Keine Dependencyänderung oder neue Live-HTTPS-Behauptung.

M2_SOURCE_CHANGES=COMPLETE; M2_TESTS_PREPARED=YES; M2_TESTS_RUN=NO;
M2_BUILD_RUN=NO; NETWORK=NO; REGISTRY_ACCESS=NO; GIT_MUTATIONS=NO;
HARDWARE=NO; LIVE_HTTPS=BLOCKED_DEPENDENCIES; REAL_READBACK=UNQUALIFIED;
CHECKPOINT_PROMOTION=BLOCKED; BRIDGE_PUBLICATION=BLOCKED.

---

## M3 – M2-Grenzprüfung und übergebenes Offline-Testergebnis, 2026-10-26

AGENTS.md, README, M1-Transportplan, jüngsten M2-Budgetcheckpoint,
Bridge-README, relevante Rustquellen und Tests sowie die einschlägigen
Projektverweise lesend geprüft. Die älteren gleichnamigen M1-/M2-/M3-
Entwicklungsstufen bleiben historische Einträge; dieser Checkpoint setzt
den jüngsten M2-REST-Budgetschritt fort. Vorhandene Nutzeränderungen bleiben
erhalten. Da die Bridge im aktuellen Git-Status unversioniert ist, beruht
diese Prüfung auf den vorhandenen Quellbytes und dem dokumentierten
M2-Änderungsumfang, nicht auf einem unabhängigen M2-Commitdiff.

**Testergebnis und Provenienz:** Der Auftrag übergibt für

```text
cargo test --offline --locked --all-targets -- --test-threads=1
exit status: 0
```

einen erfolgreichen Offline-Rust-Testlauf. Das ist ein vom Nutzer gemeldetes
Ergebnis, kein in M3 selbst ausgeführter Lauf. Vollständige Cargoausgabe,
Testanzahl, Laufumgebung und Bindung an einen Quellhash wurden nicht
mitgeliefert; konkrete Einzelergebnisse werden deshalb nicht rekonstruiert.
Der Befehl umfasst die Cargo-Testtargets, beweist aber für sich keine
Format-/Clippyprüfung, Live-TLS-Integration oder externe Qualifikation.
Bestehende Tests enthalten Python-Entwicklungsoracles und lesende Git-
Objektoracles; M3 hat weder diese noch Python oder Git-Schreibaktionen
ausgeführt. In M3 wurden keine Tests, Builds, Formatter oder Clippy gestartet.

**Reviewbefund:** Die vorliegende M2-Implementierung entspricht der
dokumentierten begrenzten Aufgabe. RestReader startet je Instanz mit 32 MiB;
erlaubte Aufrufe verbrauchen vor dem Quellenzugriff einen Versuch. Erfolgreich
zurückgegebene Bodies werden vor Einzelgrößen-, JSON- und Formprüfung mittels
checked_sub angerechnet. Exaktes Ausschöpfen lässt die letzte Antwort noch
prüfen; Überschreiten erschöpft das Budget dauerhaft. Weitere Quellenaufrufe
werden verhindert. Allowlistprüfung und Versuchslimit bleiben vorgeschaltet;
wenn beide Limits erschöpft sind, kann REQUEST_LIMIT_EXCEEDED Vorrang haben.
Quellenfehler verbrauchen einen Versuch, aber keine zurückgegebenen Bodybytes.
Die drei fokussierten Tests prüfen exakte Grenze und ausbleibende Folgeaufrufe,
ein Byte Überschreitung mit dauerhafter Sperre sowie Anrechnung abgewiesener
JSON-/Form-/Übergrößenantworten. Dies ist eine Quellprüfung ihrer Assertions,
keine separat beobachtete Ausführung dieser drei Tests.

Kein spezifischer Rustdefekt festgestellt; keine Rustkorrektur vorgenommen.
Nur dieser Checkpoint wurde angehängt. Cargo.toml/Cargo.lock, Dependencies,
Publish-Sperre und last_verified bleiben unverändert. Sollte später eine
Quellkorrektur nötig werden, ist danach ein neuer Testlauf erforderlich;
das übergebene Ergebnis deckt eine solche Änderung nicht ab.

**Weiter bestehende Grenzen:** Das Budget gilt nur für die Weiterverarbeitung
bereits gelieferter REST-Bytes je RestReader. Es begrenzt weder Quellallokation,
Netzwerkempfang, Header, Zeit noch versteckte Fehlerbytes und schließt
RawCommitSource nicht ein. Ein neuer Reader beginnt ein neues Budget.
Synthetische HTTP-Envelopes und Offline-Git-Prüfungen authentifizieren keine
GitHub-Erhebung. Ein echter HTTP-/TLS-Stack, sichere Credentialzuführung,
authentifizierte unveränderte Rohcommitbytes und unabhängiger realer Readback
bleiben offen. Die produktive Publish-CLI bleibt bedingungslos gesperrt.
Es gibt keine produktive Publikationsqualifikation und keine Promotion.

**Exakte nächste Schritte – spätere, getrennt zu autorisierende Arbeit:**

1. Zuerst lokale Toolchain, Registry-/Cachebestand und die Primärdokumentation
   möglicher etablierter Rust-HTTP-/TLS-Stacks prüfen. Kandidat, genaue Version,
   minimale Features, TLS-Backend/Crypto-Provider, CA-Herkunft, Zielplattform
   und MSRV festlegen. Den vollständigen transitiven Build-/Runtime-/Testgraph
   einschließlich TLS-Fixtures bestimmen. Keine eigene TLS-, Zertifikats- oder
   HTTP-Framingimplementierung als Ersatz erstellen.
2. Mit eigener Beschaffungs-/Schreibfreigabe fehlende Pakete per kontrolliertem
   Import oder freigegebenem Download bereitstellen; Registry-/Lock-Vertrauen,
   Checksummen, Herkunft, Lizenzen und Security-Advisories prüfen. Erst im dafür
   erweiterten Schreibumfang Cargo.toml/Cargo.lock aktualisieren, Features und
   Provenienz dokumentieren und den vollständigen Satz offline qualifizieren.
   Diese Manifest-/Lockänderungen sind im M3-Auftrag nicht erlaubt.
3. Einen echten GET-Adapter unter ResponseSource mit festem api.github.com-
   Ziel und bestehender Allowlist integrieren: verbindliche Ketten-/Zeit-/
   Hostnamenprüfung, kein HTTP-Fallback, Redirect, automatischer Proxy oder
   versteckter Retry; minimaler expliziter Lesecredentialvertrag und sichere
   Fehlercodes ohne Secrets. Monotone Gesamt-/Phasendeadlines und Empfangs-
   limits für Header, Bodies, Versuche und Gesamtbytes während des Empfangs
   durchsetzen. Rohcommits in dasselbe Gesamtbudget einbeziehen. Den separaten
   authentifizierten Beschaffungsweg für unveränderte OID-gebundene Rohcommits
   anhand von Primärdokumentation und tatsächlichen Bytes nachweisen; REST-
   Commitmetadaten niemals zu angeblichen Originalbytes rekonstruieren.
4. Nach Implementierung die obige Offline-Regression erneut ausführen, dazu
   cargo fmt --all -- --check und
   cargo clippy --offline --locked --all-targets -- -D warnings
   im Bridge-Crate. Vorher Testschreibziele und vorhandene Oraclevoraussetzungen
   klären. Mit eigener lokaler Socket-/Testfreigabe den echten TLS-Adapter an
   kontrolliertem Server prüfen: gültige/ungültige Kette, Host/Zeitfehler,
   Handshakeabbruch, langsame Phasen/Bodies, Trunkierung, Größenlimits,
   Redirectablehnung, fehlende Retries und Secretfreiheit. Test-CAs isolieren;
   vorhandene Objekt-/Race-/Manipulationsfixtures durch denselben Leser prüfen.
5. Erst mit ausdrücklicher GitHub-Lesefreigabe und Credentialvertrag einen
   begrenzten authentifizierten GET-Lauf gegen loomit77/project-block-logs
   durchführen. Repository-ID unabhängig pinnen, main-HEAD vorher/nachher
   lesen, alle verwendeten OIDs, Originalcommits, vollständigen Trees und
   exakten Statusblobbytes prüfen. Herkunft und redigierte Erhebungsevidenz
   erhalten; keine Mutationsantwort als Readback verwenden. Fehlendes Statusziel
   oder fehlende Originalbytes bleiben ein Blocker, kein Anlass zur Publikation.
6. Danach in unabhängiger Prüfung und neuem Chat den Statusindex sowie seine
   referenzierten Originalprotokolle über den vorgesehenen privaten Leseweg
   tatsächlich lesen, Byteintegrität, Berechtigungen, Verfügbarkeit und den
   praktischen Wiedereinstieg belegen. Snapshotprüfung und Transitionprüfung
   getrennt halten. HEAD-Gleichheit beweist keinen atomaren Verlauf. Reales
   Readback ist noch keine fachliche Abnahme oder Publikationsqualifikation;
   Live-Mutation bleibt ein eigener späterer Auftrag gemäß M1.

M3_REVIEW=COMPLETE; M2_BOUNDARY_REVIEW=CONSISTENT;
OFFLINE_TEST_EXIT=0_USER_REPORTED; M3_TESTS_RUN=NO; RUST_CORRECTIONS=NONE;
M3_EDIT=CHECKPOINT_APPEND_ONLY; DEPENDENCY_CHANGES=NO; NETWORK=NO;
GIT_MUTATIONS=NO; PUBLISHING=NO; HARDWARE=NO; NIXOS_BUILD=NO;
LAST_VERIFIED_CHANGES=NO; LIVE_HTTPS=BLOCKED_DEPENDENCIES;
REAL_READBACK=UNQUALIFIED; CHECKPOINT_PROMOTION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED.

---

## NOVENA-BRIDGE-RUST-277 – Qualität und HTTPS-Plan

Datum: 2026-10-26. Umfang: ausschließlich angehängte Dokumentation;
keine Implementierung und keine technische Abnahme.

### Geprüfter Ausgangsstand und Evidenzgrenzen

AGENTS.md, Repository-README, Bridge-README, Fortschrittsdatei einschließlich
jüngstem M1/M2/M3, Lite-Plan, Cargo.toml/Cargo.lock sowie relevante Rust-
Schnittstellen und Protokolltests lesend geprüft. Projektstatus, Testlog und
Entscheidungen als Kontext herangezogen. Git-Zustand mit
`git --no-optional-locks status --short` gelesen: bestehende Änderungen und
unversionierte Dateien einschließlich der Bridge vorhanden und erhalten.
Die begrenzte Dateinamensuche unter test-logs/, evidence/ und research/ ergab
keinen passenden Bridge-/274-/276-/Format-/Clippy-Rohbeleg; das schließt
anders benannte oder anderweitig aufbewahrte Protokolle nicht aus.

Verbindlich vom Auftrag übernommen: Block 274 mit 66 erfolgreichen Rusttests;
Block 276 mit gesichertem M1/M2/M3 und erfolgreichen Rusttests einschließlich
der drei zusätzlichen REST-Budgettests. Die frühere M2-Angabe „nicht
ausgeführt“ beschreibt dessen damaligen Stand; M3 dokumentiert bereits einen
übergebenen erfolgreichen Offline-Testlauf. Hier gilt die jüngste explizite
Bestätigung des Auftrags. Keine neue Testanzahl oder Einzelausgabe aus diesen
Angaben rekonstruiert. Kein eigener Testlauf, keine erneute Cacheinventur.
Der bisher festgestellte unvollständige lokale HTTP-/TLS-Abhängigkeitssatz
bleibt der dokumentierte Beschaffungsblocker.

Vorhanden und offline teilqualifiziert sind strikte JSON-/Statusverarbeitung,
Originalquellen-/Logprüfung, Git-Objektprüfung, Objekt-/CAS-Planung und
unabhängige Offline-Snapshot-/Transitionsprüfung. Konkrete Anschlüsse:
`ResponseSource::get` unter `RestReader`, separat `RawCommitSource::raw_commit`,
`check_commit`, `verify_initial_index`, `verify_update_index` und
`attestation::verify_snapshot`. Die drei Budgettests prüfen exakte Ausschöpfung,
ein Byte Überschreitung mit bleibender Sperre sowie Anrechnung abgewiesener
Antworten. Ihre Assertions wurden gelesen, hier nicht ausgeführt.

`RestReader` begrenzt bereits zurückgegebene REST-Bodies auf 8 MiB einzeln,
32 MiB kumulativ je Instanz und höchstens 256 Versuche. Das schützt weder
Empfang noch Quellallokation, Header oder Zeit; Quellenfehler liefern keine
anrechenbaren Bodybytes. Rohcommits liegen außerhalb dieses Budgets.
`OfflineHttpSource` ist eine vorhandene synthetische Policyprüfung, kein
HTTP-Client. Es fehlt weiterhin ein echter Rust-HTTP-/TLS-Transport.

### Tatsächlicher Format-/Clippy-Status

Die äußere Prüfung meldet laut Auftrag:

| Prüfung | Tatsächlicher Exitstatus | Bewertung |
|---|---|---|
| `cargo fmt --all -- --check` | 1 | Fehlgeschlagen; Formatqualität nicht bestätigt |
| `cargo clippy --offline --locked --all-targets -- -D warnings` | 101 | Fehlgeschlagen; Clippy-Warnungsfreiheit nicht bestätigt |

Beide Ergebnisse sind Fehler, weder PASS noch lediglich „nicht geprüft“.
Ältere PASS-Einträge gelten für historische Stände und ersetzen diese jüngste
Meldung nicht. Erfolgreiche Rusttests heilen diese fehlgeschlagenen Prüfungen
nicht. Umgekehrt beweisen die zwei Exitstatus keinen fehlgeschlagenen
Rust-Testlauf und benennen keinen konkreten Rustdefekt.

**Ursachen: noch zu untersuchen.** Vollständige stdout-/stderr-Diagnosen,
Toolchain-/Komponentenversionen, Arbeitsverzeichnis und belastbare Bindung an
die geprüften Quellbytes fehlen in der hier verfügbaren Evidenz. Exit 1 allein
belegt keine bestimmte Formatdiffstelle; Exit 101 unterscheidet keine
Clippy-Lint-, Compiler-, Dependency- oder Werkzeugursache. Keine dieser
Möglichkeiten wird zur bestätigten Ursache erklärt. Keine Korrektur auf
Verdacht. In diesem Auftrag weder Formatter noch Clippy, Builds oder Tests
aufgerufen. Nächster Qualitätsschritt ist zuerst die Auswertung der erhaltenen
äußeren Originaldiagnosen; erst gesondert autorisiert Ursachen korrigieren und
betroffene Prüfungen wiederholen. Dokumentationsabschluss ist keine Abnahme.

### Technische Auswahl: bevorzugte Richtung, noch kein Dependencybeschluss

Bevorzugt wird ein schmaler echter GET-Adapter mit **reqwest und rustls**,
unter Wiederverwendung der vorhandenen Prüfmodule. Zunächst synchroner
Aufrufpfad passend zum bestehenden `ResponseSource`-Vertrag; ob die konkrete
blocking-API sämtliche Abbruch-/DNS-/Headeranforderungen erfüllt, muss vor
Festlegung nachgewiesen werden. Ein nicht abbrechbarer Hintergrundthread ist
kein Ersatz für eine wirksame Deadline. Bei Nichterfüllung ist die Auswahl
vor der Integration erneut zu entscheiden.

Begründung der Richtung: HTTP- und TLS-Protokollarbeit an etablierte Rust-
Bibliotheken delegieren, Bridge-Policy und Objektbeweise in der vorhandenen
Codebasis belassen. **ureq mit rustls** ist eine sinnvolle Alternative für
einen synchronen, möglicherweise kleineren Stack; tatsächlicher Featuregraph
und Ressourcenbedarf sind noch nicht verglichen. **hyper mit hyper-rustls**
ist eine weitere etablierte Richtung, verlangt aber mehr Integrationsarbeit
für Clientverhalten und Laufzeit. Kein eigener HTTP-Framing-/TLS-Nachbau,
keine zweite OfflineHttpSource, kein Python- oder Shell-Transport.

Diese Kandidaten sind Auswahlvorschläge, keine Bestätigung ihrer aktuell
verfügbaren Versionen, Sicherheitslage oder konkreten APIs. Wegen des
Netzwerkverbots wurden keine aktuellen Upstream-Primärquellen abgefragt.
Genaue Version, Featurebezeichnungen und Eignungsnachweise bleiben offen.
Cargo.toml enthält nur sha1, sha2 und serde_json; Cargo.lock enthält keinen
solchen HTTP-/TLS-Stack. Keine neue Version wird hier als gesichert genannt.

Vor einer Manifeständerung sind anhand der Primärdokumentation der gewählten
Version mindestens folgende Punkte festzulegen und nachvollziehbar abzulegen:

- `default-features = false` als Auswahlprinzip; nur benötigten GET-/Bytebody-
  Pfad, gegebenenfalls blocking und genau ein rustls-Backend freischalten.
  Exakte Feature-Namen erst nach Versionsprüfung festschreiben. Kein zweites
  TLS-Backend, Cookie-Jar, automatische Kompression oder unnötige Protokolle.
- Den tatsächlich aufgelösten transitiven Graph einschließlich Feature-
  Vereinigung, Resolver, Laufzeit, DNS, HTTP, TLS, Crypto-Provider, Root-CA-
  Quelle, Plattformanteilen, Buildskripten und Testabhängigkeiten erfassen.
  Wenige direkte Crates bedeuten keinen kleinen oder vollständigen Stack.
  Ob etwa Tokio, hyper, ring oder aws-lc-rs enthalten sind, ist erst am
  gewählten Graph zu bestätigen; keine davon gilt hier als bereits verfügbar.
- foobox als Ausführungshost, Toolchain/MSRV und native Buildvoraussetzungen
  prüfen; Novena/ARM-Unterstützung nicht voraussetzen. Vollständigen Satz aus
  vertrauenswürdigem Import oder gesondert freigegebener Beschaffung erhalten,
  Registry-/Lock-Herkunft, Checksummen, Lizenzen und Advisories prüfen.
  Reproduzierbare Offline-Qualifikation erst nach vollständiger Bereitstellung.

### Verbindliche Anforderungen an den späteren Empfangspfad

TLS muss Zertifikatskette, Hostname und Gültigkeit verbindlich prüfen, SNI
korrekt verwenden und mindestens TLS 1.2 erlauben, ohne unsicheren Fallback.
Truststore-Herkunft und Aktualisierung explizit entscheiden: gepflegtes
Rootbundle oder kontrollierter Systemtruststore, keine stillschweigende
Übernahme beliebiger CA-Umgebungsvariablen. Kein permissiver Verifier, kein
Abschalten von Zertifikatsprüfung; Test-CAs bleiben vom Produktivpfad getrennt.
Zertifikate sind keine Credentials, private Schlüssel und Tokens dagegen
Secrets. Für diesen Client ist kein eigener Client-Schlüssel vorausgesetzt.

Nur intern konstruierte `https://api.github.com/`-Ziele der bestehenden
Allowlist, keine freien URLs, Redirects, HTTP-Downgrades oder impliziten
Proxies. Nur GET, vollständiger Status 200; Location, 3xx, Teilantworten,
Auth-/Rate-Limit-/Serverfehler und Framingfehler führen zum Abbruch. Keine
versteckten Retries, Cookies oder Credentialweiterleitung. Medientyp,
API-Version und User-Agent später anhand der GitHub-Primärquellen festlegen.

Credentials ausschließlich explizit über einen separat vereinbarten sicheren
Eingang mit minimalen privaten Leserechten; keine implizite Suche nach lokalen
Tokens, keine Tokens in CLI-Argumenten oder URLs. Keine Secrets, Header,
privaten Bodies oder unredigierten Bibliotheksfehler in Debug/Logs. Fehler
auf stabile englische BridgeError-Codes abbilden. Speicherlebensdauer
minimieren; sichere Löschung nicht ohne Nachweis versprechen. Schutz und
zulässige Ablage privater Readback-Evidenz separat vereinbaren.

Als konkretes **zu qualifizierendes Startprofil**, nicht als implementierter
Schutz: höchstens 8 MiB REST-Body, 1 MiB Rohcommit, 32 MiB gemeinsames
Body-Empfangsbudget und 256 Versuche je gesamtem Lesevorgang; höchstens
32 KiB Header und 100 Headerfelder je Antwort. Headergrenzen müssen bereits
im Bibliotheksparser greifen, nicht erst nach unbeschränkter Allokation.
Vorschlag für monotone Zeitgrenzen: 10 s DNS/Connect/TLS zusammen, 10 s bis
vollständige Header, 30 s je gesamter Anfrage und 120 s je Lesevorgang;
es gilt stets die früheste Grenze. Diese Werte sind Planung und benötigen
belegte Durchsetzbarkeit am gewählten Stack und eine fachliche Limitentscheidung.

Stückweise mit begrenzten Puffern empfangen; Content-Length allein genügt
nicht. Chunking, fehlende/falsche Länge, EOF und Slow-Drip berücksichtigen.
Vor weiterer Allokation und während jedes Reads Budget/Deadline prüfen;
Fehlerbytes und Rohcommits ebenfalls anrechnen. Kompression zunächst aus,
unerwartete Content-Encoding zurückweisen. Bibliotheksinterne Puffer und
Framingaufwand separat nachweisen; 32 MiB Bodybudget ist kein exaktes
Prozessspeicherlimit. Ein neuer RestReader darf das gemeinsame Transportbudget
nicht zurücksetzen. Bei Überschreitung kein partieller Erfolg.

### Rohcommitbytes und authentifizierter unabhängiger Readback

REST-Commit-JSON, Patch-/Diffmedien oder rekonstruierte Author-/Zeitstrings
sind keine Git-Commit-Rohbytes. Ein geeigneter authentifizierter,
OID-gebundener Beschaffungsweg ist **weiterhin unbelegt**. Kein bestimmter
GitHub-Endpunkt oder Accept-Header wird ohne Primärnachweis als Lösung
behauptet. Dieser Weg benötigt einen eigenen engen Quellen-/Allowlistvertrag.
Falls kein solcher Weg qualifizierbar ist, bleibt vollständiger Readback
blockiert; kein Ausweichen auf synthetisierte Bytes.

Originalbytes unverändert als Bytes behandeln: keine UTF-8-Neukodierung,
Newline-Ergänzung, CRLF-Normalisierung oder JSON-Umserialisierung. Mit
`object_oid(ObjectKind::Commit, bytes)` die angeforderte OID prüfen und
Tree-/Parentmetadaten über `check_commit` abgleichen. Das vorhandene schmale
unsigned Root-/Ein-Parent-Profil bleibt erhalten; nicht unterstützte
Commitheader werden abgewiesen statt normalisiert.

Ein späterer authentifizierter Leseablauf muss separat zur Mutation erhoben
werden: unabhängig gepinnte Repository-ID, Ref/HEAD vorher und nachher,
Originalcommits, vollständige gehashte Trees und exakte Status-/Baseline-
Blobbytes über die bestehenden Prüfer binden. Mutationsantworten sind keine
Objektbelege. Snapshot und Transition getrennt halten. Für Live-Erhebung einen
eigenen Herkunftsvertrag mit Erhebungsbedingungen, CA-/Clientkonfiguration,
Zeiten, OIDs, Hashes und privat geschützten Originalbelegen definieren;
`OFFLINE_SUPPLIED_RESPONSES`/`VARIANT_A_OFFLINE_V1` nicht umetikettieren.
TLS authentifiziert den Serverkanal im gewählten Vertrauensmodell, nicht
fachliche Aussagen. Zwei gleiche HEAD-Lesungen beweisen keinen atomaren Verlauf.

### Präzise nächste Schritte und Wiedereinstieg

1. Diesen Checkpoint und M1/M2/M3 lesen, Git-Zustand erneut lesend prüfen und
   Nutzeränderungen erhalten. Zuerst äußere Format-/Clippy-Originaldiagnosen
   beschaffen/lesen, sofern innerhalb eines neuen Auftrags zugänglich;
   ohne diese Ursachen ausdrücklich offen lassen. Keine Wiederholung allein
   aus diesem Plan ableiten.
2. In einem gesonderten Dependencyauftrag lokale Primärquellen/Cache und
   Toolchain prüfen, danach genaue reqwest-/rustls-Auswahl gegen die genannten
   Alternativen bewerten. Feature-/Truststore-/Crypto-/Deadline-Nachweise und
   vollständigen Graph dokumentieren. Fehlende Quellen oder Pakete als Blocker
   nennen. Beschaffung und Cargoänderungen benötigen erweiterten Auftrag.
3. Nach dieser Entscheidung echten Rust-GET-Adapter an ResponseSource und
   gemeinsames Empfangsbudget anschließen. Bestehende Offlinequelle erhalten;
   Rohcommitweg separat nachweisen und implementieren. Kein Live-Transport in
   die ausschließlich synthetischen simulate_graphql-Verträge einsetzen.
4. In autorisiertem Entwicklungsauftrag Qualitätsschäden anhand der Diagnosen
   beheben. Danach Format, Clippy und bestehende Regression sowie lokale echte
   TLS-Tests qualifizieren: unbekannte CA, falscher Host, ungültige Zeit,
   Handshakeabbruch, Redirect/Proxy/Retry, Secretredaktion, Header-/Bytegrenzen,
   Chunking, Abbruch und echte Deadlines einschließlich DNS/Slow-Drip.
   Keine Test-CA im Produktivpfad. Erfolg jeder Stufe separat belegen.
5. Erst mit ausdrücklicher GitHub-Lesefreigabe und Credential-/Evidenzvertrag
   begrenzten authentifizierten Readback erheben. Fehlender Statusindex oder
   fehlende Rohcommits bleiben Blocker; nichts zu Testzwecken veröffentlichen.
   Danach unabhängige Gesamtprüfung und im neuen Chat Statusindex samt
   referenzierten Originalprotokollen tatsächlich über den privaten Leseweg
   lesen. Verfügbarkeit, Integrität und praktischen Wiedereinstieg belegen.

**Ausdrücklich gesperrt in diesem Auftrag:** Rust-/Python-/Bash-Änderungen,
Cargo.toml-/Cargo.lock-Änderungen, Builds und Tests einschließlich erneuter
Format-/Clippyaufrufe, Downloads und sämtliche Netzwerk-/GitHub-API-/SSH-
Zugriffe, Git Add/Commit/Push/Reset/Clean und sonstige Git-Mutationen,
Veröffentlichungen, last_verified-Änderungen, NixOS-Image-Builds,
Blockgerätezugriffe und Hardwaretests. Nur diese bestehende Fortschrittsdatei
wurde ergänzt; alle früheren Checkpoints bleiben vollständig erhalten.
Produktive Veröffentlichung und Checkpoint-Promotion bleiben gesperrt.

CHECKPOINT_277=DOCUMENTED; TECHNICAL_ACCEPTANCE=NOT_GRANTED;
TESTS_274=66_PASS_USER_REPORTED; TESTS_276=PASS_USER_REPORTED;
FMT_EXIT=1_USER_REPORTED; CLIPPY_EXIT=101_USER_REPORTED;
QUALITY_CAUSES=INVESTIGATION_REQUIRED; NEW_BUILDS_OR_TESTS=NO;
PREFERRED_DIRECTION=REQWEST_RUSTLS_PENDING_EVIDENCE;
DEPENDENCY_CHANGES=NO; NETWORK=NO; GIT_MUTATIONS=NO;
LIVE_HTTPS=NOT_IMPLEMENTED; REAL_READBACK=UNQUALIFIED;
LAST_VERIFIED_CHANGES=NO; CHECKPOINT_PROMOTION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED.

---

## NOVENA-BRIDGE-RUST-278 – Rust-Qualitätskorrektur, 2026-10-26

Ausgangslage: Block 277 ergab Format-Exit 1 und
Clippy-Exit 101 mit drop_non_drop in tests/protocol.rs.

Codex wurde mit einer eng begrenzten Rust-Korrektur
beauftragt. Die äußere Prüfung ergab:

- cargo fmt --all -- --check: Exit 0
- cargo clippy --offline --locked --all-targets -- -D warnings: Exit 0
- cargo test --offline --locked --all-targets -- --test-threads=1: Exit 0

Diese Werte stammen aus dem Runner von Block 278.
Das vollständige Testprotokoll liegt im separaten
Runner-Originalprotokoll.

Git-HEAD und geschützter Index wurden geprüft und
blieben unverändert.

Keine neuen Cargo-Abhängigkeiten. Kein Python-Ausbau,
kein Live-GitHub-Readback, keine Mutation und keine
Veröffentlichung.

Der echte HTTP-/TLS-Transport bleibt Gegenstand eines
späteren, gesondert qualifizierten Rust-Auftrags.

RUST_278_FMT_EXIT=0;
RUST_278_CLIPPY_EXIT=0;
RUST_278_TEST_EXIT=0;
LIVE_HTTPS=NOT_IMPLEMENTED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.


---

## NOVENA-BRIDGE-RUST-279 – konkrete HTTPS-Integrationsvorbereitung, 2026-10-26

**Ergebnis: lokale Bestandsprüfung und Integrationsplan abgeschlossen;
reqwest mit rustls bleibt bevorzugt, eine konkrete Dependencyauswahl ist
noch nicht qualifiziert.** Dieser Nachtrag ist der jüngste Wiedereinstieg.
Alle früheren Checkpoints bleiben unverändert erhalten. Ausschließlich diese
Datei darf in diesem Auftrag geändert werden. Der Plan erteilt keine Freigabe
für seine spätere Ausführung.

### Ausgangsstand und tatsächlich erhobene Evidenz

Block 278 ist laut Auftrag gesichert: 69 Rust-Tests PASS, Rustfmt PASS,
Clippy mit `-D warnings` PASS. Der vorhandene 278-Nachtrag bestätigt die
Exitwerte und verweist auf ein separates Runner-Originalprotokoll; dieses
wurde hier nicht erneut qualifiziert. Die älteren offenen Qualitätsbefunde
aus 277 sind damit als damaliger Stand zu lesen. In 279 keine Tests,
Format-/Clippyprüfungen, Builds, Cargo-Auflösung oder Downloads ausgeführt.

Gelesene Grundlagen: AGENTS.md, Repository-README, Bridge-README,
Fortschrittsdatei einschließlich M1/M2/M3 und 277/278, Lite-Plan,
Cargo.toml/Cargo.lock, relevante Rust-Schnittstellen und Protokoll-/Quellen-
Tests sowie Projektstand, Testlog und einschlägige Entscheidungen.
Dateinamensuche unter test-logs/, evidence/ und research/ ergab keine
passenden Bridge-/278-/279-/reqwest-/rustls-Belege; das ist keine Aussage
über außerhalb dieser Auswahl liegende Originalprotokolle oder Quellen.

Git wurde ausschließlich mit `--no-optional-locks` gelesen. Vorbestehend
geändert: TODO.md, docs/CHAT-STARTER.md, docs/DECISIONS.md,
docs/HANDOVER-LATEST.md und docs/NOVENA-BRIDGE-LITE-PLAN.md; unversioniert
sind die Rust-Crate und fünf novena-bridge-*-Pythondateien unter scripts/.
Diese Nutzerarbeit bleibt erhalten. HEAD:
`df28dc50be1eea15339a8510a4aa450eb6b15b99`.
Geschützter Index-SHA256:
`3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9`.
Manifest-SHA256:
`ccd4889f074a3323a9f2732ea10c3cdb004a33d3714d3331f66a94778b4f3931`;
Lockfile-SHA256:
`0f1b9406a003ac867b995c55781ef5605f0aae2b8935591c3a518abb493afb56`.

### Toolchain, Registry und verfügbare Pakete

Lesend ausgeführt: `command -v`, `cargo --version --verbose`, `rustc -vV`,
`rustc --print target-libdir`, begrenzte Umgebungs-/Konfigurationsprüfung,
Verzeichnislisten, `rg` und SHA256 der vorhandenen Cratearchive.
Keine geheimen Umgebungswerte oder Cargo-Credentials gelesen.

| Gegenstand | Verifiziert | Noch nicht verifiziert |
|---|---|---|
| Toolchain | /usr/bin/cargo und /usr/bin/rustc, beide 1.99.1, Arch-Paket 1:1.99.1-1; Host x86_64-unknown-linux-gnu, CachyOS; rustc LLVM 23.1.3 | MSRV und Buildfähigkeit eines neuen HTTP-/TLS-Graphs; README nennt für bestehenden Code Rust >=1.88, hier kein MSRV-Test |
| Cargo-Vertrag | Edition 2021, publish=false, Lockformat 4; sha1="0.10", sha2="=0.10.9", serde_json="1" | Kein expliziter rust-version-/Resolver-Eintrag im Crate-Manifest; künftige Featurevereinigung erst am aufgelösten Graph prüfen |
| Cargo-Umgebung | CARGO_HOME, RUSTUP_HOME und CARGO_TARGET_DIR in dieser Sitzung nicht gesetzt; rustup nicht im PATH gefunden | Keine Aussage über sämtliche möglichen Umgebungs-/Systemoverrides oder externe Stores |
| Konfiguration | Keine config/config.toml in untersuchten Cargo-Vorfahren ab Crate bis / und unter ~/.cargo gefunden; kein projektspezifischer Toolchain-Pin gefunden | Kein globaler Vollständigkeitsnachweis über andere Cache-/Nix-/Vendoringorte |
| Registry | ~/.cargo/registry/index/index.crates.io-1949cf8c6b5b557f: config.json nennt dl=https://static.crates.io/crates und api=https://crates.io; 22 lokale Sparse-Cacheeinträge | Aktualität, Herkunftsauthentizität und vollständiger Remote-Index; keine reqwest-/rustls-Metadaten lokal in diesem Index |
| Paketmaterial | Dieselben 22 Paketversionen als *.crate und ausgepacktes src vorhanden; SHA256 aller 22 Archive stimmen mit Cargo.lock überein | Ausgepackte Quellen nicht vollständig gegen Archive gehasht; Hashgleichheit zum lokalen Lock beweist keine unabhängige Registryauthentizität |
| Native Werkzeuge | cc, cmake und pkg-config im PATH; Host-stdlib unter /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib | Funktionsprüfung, Versionen und Eignung für gewählten Crypto-Provider; Linker-/C-/Assembler-/Headeranforderungen, DNS, Uhr und CA-Bereitstellung |

Verifizierte lokale Registryversionen (22; zusätzlich das lokale Paket
novena-bridge 0.1.0 im Lockfile): block-buffer 0.10.4, cfg-if 1.0.5,
cpufeatures 0.2.17, crypto-common 0.1.7, digest 0.10.7, generic-array 0.14.7,
itoa 1.0.18, libc 0.2.191, memchr 2.8.3, proc-macro2 1.0.107, quote 1.0.47,
serde 1.0.229, serde_core 1.0.229, serde_derive 1.0.229,
serde_json 1.0.154, sha1 0.10.7, sha2 0.10.9, syn 3.0.9,
typenum 1.20.1, unicode-ident 1.0.27, version_check 0.9.5, zmij 1.0.23.
Lockmitgliedschaft bedeutet nicht, dass jedes Paket im aktiven Featuregraph
kompiliert wird. Der HTTP-/TLS-Satz ist darin nicht enthalten.

### Dependencyentscheidung und konkrete offene Auswahlpunkte

Bevorzugt wird **reqwest mit genau einem rustls-Backend**, ohne Defaultfeatures.
Begründung ist der vorhandene synchrone ResponseSource-Vertrag und der Bedarf
an einem etablierten HTTP-/TLS-Client. **Unbewiesene Annahme:** Ein blocking-
Client der später gewählten Version kann alle nötigen Empfangs- und
Abbruchbedingungen mit vertretbarem Aufwand erfüllen. Das ist zuerst anhand
von Quellcode und lokalen echten TLS-Tests zu prüfen; blocking allein belegt
keine harte DNS-Deadline oder Beendigung interner Hintergrundarbeit.

Keine konkrete reqwest- oder rustls-Version, MSRV, Featurebezeichnung,
Rootbundle-Version, Crypto-Providerwahl oder Retry-API ist hier verifiziert.
Insbesondere keine vermeintlich aktuelle Versionsnummer aus Erinnerung pinnen.
`blocking` und `rustls-tls` sind zu prüfende Featurekandidaten, kein bereits
gültiger Manifestvorschlag für eine bestimmte Version. Beabsichtigt sind
GET und Byteempfang, kein JSON-Feature, Multipart, Cookie-Jar, automatische
Kompression, HTTP/3 oder zweites TLS-Backend. HTTP/1.1 als kleinstes
Startprofil prüfen; ob HTTP/2-Features zuverlässig ausgeschlossen werden
können und welche Headerparsergrenzen verfügbar sind, bleibt offen.

Noch zu ermitteln sind vollständige direkte, transitive, Build-, Test- und
plattformabhängige Pakete samt Features, Versionen, Checksummen, Lizenzen,
Buildskripten und MSRV. Namen wie tokio, hyper, hyper-util, http, http-body,
bytes, url, rustls, tokio-rustls, rustls-webpki, webpki-roots, ring oder
aws-lc-rs bezeichnen **Prüfkandidaten, keinen verifizierten Dependencygraph**.
Keiner dieser Kandidaten liegt im untersuchten Cache vor. Provider und
Rootquelle dürfen nicht durch Defaults oder Featurevereinigung unbemerkt
wechseln. Cargo selbst meldet libcurl mit OpenSSL; das sagt nichts über das
spätere TLS-Backend des Bridge-Binaries aus.

Ziel der ersten Qualifikation ist foobox/x86_64-unknown-linux-gnu.
ARM-/Novena-Ausführung, Cross-Build und Imageintegration sind weder nötig
noch nachgewiesen. Eine gepflegte, versionsgebundene Rootliste ist das
bevorzugte Trustmodell; konkrete Bezugsquelle, Aktualisierung und Umgang mit
Widerruf bleiben Prüf-/Entscheidungspunkte. Gültige Hostzeit ist Voraussetzung
der Zertifikatszeitprüfung. Kein Nachweis von Offline-Revokationsprüfung
behauptet. Aktuelle Security-/RustSec-/Upstream-Advisories fehlen wegen des
Netzwerkverbots.

Ein Alternativenvergleich wird jetzt nicht eröffnet: es gibt noch keinen
belegten technischen Ausschluss von reqwest. Falls die gewählte Version
Headerlimits vor Allokation, explizite Retry-Sperre oder echte Gesamtdeadlines
nicht bereitstellen kann, zuerst innerhalb reqwest den minimal nötigen
asynchronen Pfad prüfen; erst bei belegter Restlücke einen anderen Client
gezielt gegen genau dieses Kriterium vergleichen. Kein eigener HTTP-/TLS-Stack.

### Kleinste sichere Integration in bestehende Rust-Schnittstellen

1. Ein explizit konstruierter echter Rust-GET-Adapter implementiert direkt
   `protocol::ResponseSource::get(&mut self, endpoint) -> BridgeResult<Vec<u8>>`.
   RestReader, strikter JSON-/Base64-Pfad und Objektprüfer bleiben zuständig.
   OfflineHttpSource bleibt vorhandene synthetische Testquelle; keine zweite
   Offline-HTTP-Architektur und kein Live-Umweg über OfflineHttpResponse.
   Der Adapter besitzt Client, explizites Credential und einen pro gesamtem
   Lesevorgang erzeugten Empfangskontext. Kein impliziter Netzwerkdefault.
2. Vor Clientaufruf erneut `allowed_endpoint` verwenden. Exakt erlaubt sind
   ausschließlich GET auf folgende aus festem Ursprung und geprüften Pfaden
   konstruierte URLs:
   - https://api.github.com/repos/loomit77/project-block-logs
   - https://api.github.com/repos/loomit77/project-block-logs/git/ref/heads/main
   - https://api.github.com/repos/loomit77/project-block-logs/git/commits/ZONE
   - https://api.github.com/repos/loomit77/project-block-logs/git/trees/ZONE
   - https://api.github.com/repos/loomit77/project-block-logs/git/blobs/ZONE
   Dabei ist ZONE ausschließlich ein über Oid::parse geprüfter String aus
   genau 40 kleinen Hexzeichen, kein frei übergebener URLteil. Keine Queries,
   Fragmente, Ports im Eingabestring, Userinfo, Prozentkodierung, Contents-,
   recursive-Tree-, raw.githubusercontent.com- oder GraphQL-Route.
   URLnormalisierung darf eine unzulässige Eingabe nicht nachträglich erlauben.
3. TLS-Ketten-, Hostnamen- und Zeitprüfung für api.github.com erzwingen,
   korrektes SNI, mindestens TLS 1.2 ohne unsicheren Fallback. Keine permissiven
   Verifier oder danger-Schalter. Truststore explizit; keine fremden CA- oder
   Proxy-Umgebungswerte übernehmen. Redirectpolicy vollständig aus,
   automatische Proxies aus, kein Credential-/Cookie-Store. Nur vollständiger
   Status 200 ohne Location; 3xx, 204/206/304, 401/403/429/5xx und sonstige
   Abweichungen brechen ohne Folgeanfrage ab. API-Version, Accept und User-Agent
   vor Implementierung gegen GitHub-Primärquellen festlegen.
4. Credential ausschließlich als explizites, nicht Debug-/Serialize-fähiges
   Konstruktorargument aus einem gesondert freigegebenen privaten Eingang
   (etwa bereits geöffneter Deskriptor); keine Tokenentdeckung in Umgebung,
   Dateien, gh oder Git, keine Secrets in Argumenten/URL. Nur geringste nötige
   Leserechte des privaten Zielrepositories; konkreten GitHub-Rechteumfang
   noch prüfen. Authorization nur am geprüften Request setzen und als sensibel
   markieren, Headerinjektion abweisen. Keine unredigierten Clientfehler,
   Header oder privaten Bodies in Logs; stabile englische BridgeError-Codes.
   Speicherlebensdauer begrenzen, sichere Löschung nicht unbelegt zusagen.
5. Ein gemeinsamer Empfangskontext zählt REST und späteren Rohcommitweg:
   maximal 256 Transportversuche einschließlich Fehlversuchen, 32 MiB
   empfangene Bodybytes insgesamt, 8 MiB pro REST-Antwort, 1 MiB pro Rohcommit.
   Vor Sendung Versuch verbrauchen; keine parallelen Requests. Stückweise mit
   fest begrenztem Puffer lesen, verbleibendes Budget vor weiterer Allokation
   prüfen. Eine nötige Überschreitungsprobe muss explizit begrenzt werden;
   nach Erreichen eines Limits keinen weiteren Request beginnen. Fehler- und
   verworfene Antwortbytes mitzählen, Fehlerbodies nicht unbeschränkt abholen.
   RestReader behält seine eigene bisherige nachgelagerte 32-MiB-Prüfung;
   dieselben Bytes werden in zwei getrennten Schutzschichten geprüft, nicht
   zweimal vom gemeinsamen Empfangskonto abgezogen. Readerneubau oder Wechsel
   zu RawCommitSource darf dieses Konto nicht zurücksetzen.
6. Startprofil aus 277 zur Qualifikation: höchstens 32 KiB Header und 100
   Headerfelder vor unbeschränkter Parserallokation; monotone absolute
   Deadlines 10 s DNS/Connect/TLS zusammen, anschließend höchstens 10 s für
   vollständige Header, 30 s pro kompletter Anfrage und 120 s pro Lesevorgang.
   Immer früheste Grenze, auch während jedes Reads; Slow-Drip darf keine
   Gesamtdeadline erneuern. Diese Zahlen sind Planwerte, keine implementierte
   oder bereits fachlich abgenommene Durchsetzung. Fehlende Librarykontrolle
   ist ein Integrationsblocker; nachträgliches elapsed reicht nicht.
   Kompression aus, unerwartetes Content-Encoding außer identity abweisen;
   fehlende/falsche Content-Length, Chunking, EOF, Framingfehler und interne
   Puffer im Test nachweisen. Bodybudget ist kein Prozessspeicherlimit.
7. Retrypolicy explizit deaktivieren und tatsächlichen Stack prüfen: auch
   Verbindungswiederverwendung, Protokollfehler, Auth- und Rate-Limit-Verhalten
   dürfen keinen erneuten GET auslösen. Ein ResponseSource-Aufruf darf höchstens
   einen HTTP-Request erzeugen. TCP-Wiederübertragung ist davon zu unterscheiden.
   Bei unklarer Antwort Abbruch; kein Backoff-/Fallback-/Resume-Mechanismus.

### Eigene Rohcommit-Provenienz und bleibende Publikationssperre

`RawCommitSource::raw_commit(Oid)` bleibt separat. Die erlaubte REST-Commitroute
liefert Metadaten; ein Accept-Header mit angeblichem raw-Verhalten ist kein
Nachweis originaler Git-Commitbytes. Authentifizierte, OID-gebundene Beschaffung
dieser Bytes ist weiterhin **offen und blockiert vollständigen Live-Readback**.
Kein URL der obigen Allowlist wird dafür ohne Primärnachweis umgedeutet oder
um zusätzliche Ziele erweitert. Keine Rekonstruktion, Patch-/Differsatz oder
UTF-8-/Newline-/JSON-Normalisierung.

Ein späterer eigener Erhebungsvertrag muss pro Rohcommit Originalbytes,
angeforderte OID, SHA256, Quelle/Repositorybindung, Erhebungszeit/-bedingungen
und konkrete Client-/TLS-/CA-Konfiguration ohne Secrets binden. Ein separat
geliefertes lokales Original bleibt als solche Quelle gekennzeichnet und wird
nicht als HTTPS-Erhebung ausgegeben. `check_commit` und
`object_oid(ObjectKind::Commit, bytes)` prüfen Identität sowie Tree/Parents
gegen REST; vorhandenes unsigned Root-/Ein-Parent-Profil bleibt unverändert.
Echte Live-REST-Provenienz ebenfalls separat von Offlineberichten halten;
`OFFLINE_SUPPLIED_RESPONSES` und `VARIANT_A_OFFLINE_V1` nicht umetikettieren.

Unabhängig gepinnte Repository-ID, HEAD vorher/nachher, vollständige Trees,
Status-/Baselineblobs und Originalprotokolle bleiben eigene Readbackpflichten.
TLS beweist keine Fachabnahme; zwei gleiche HEAD-Werte keinen atomaren Verlauf.
Fehlender Rohcommit oder Statusindex ergibt keinen vollständigen Erfolg.
Keine Einbindung in simulate_graphql/simulate_update_graphql. Die unbedingte
Publish-CLI-/production_publish-Sperre, publish=false, Payload-/Belegtrennung
und last_verified-Erhalt bleiben bestehen. Keine API-Mutation, Veröffentlichung
oder Checkpoint-Promotion als Nebenwirkung eines GET-Erfolgs.

### Ausführbare Arbeitsfolge für einen später freigegebenen Auftrag

**Nicht in 279 ausgeführt.** Erforderlicher späterer Umfang: ausdrücklich
Registry-/Primärquellen-Downloads mit festgelegten Schreibzielen, Änderungen
an Cargo.toml/Cargo.lock und betroffenen Rust-Dateien, Cache-/Targetschreibrechte
sowie Tests einschließlich synthetischer Fixturedateien. Lokale Socket-/TLS-
Tests und echte private GitHub-GETs benötigen jeweils passende Freigaben.
Dependencybeschaffung erlaubt keine private GitHub-Abfrage oder Mutation.

1. Git/Index und vorhandene Nutzeränderungen erneut lesend prüfen. Cargo-
   Konfiguration und freigegebene Registryquelle festlegen; keine fremden
   Änderungen überschreiben. Ein freigegebenes externes CARGO_TARGET_DIR
   setzen, da hier keines gesetzt ist. Vor Cargo-Änderungen aktuelle offizielle
   reqwest-/rustls-Dokumentation und Quellmanifeste beschaffen; Version, MSRV,
   Feature-/Provider-/Rootauswahl, Parsergrenzen, Retry- und Deadline-APIs sowie
   Security-/Lizenzbefunde dokumentieren. GitHub-GET-Header separat belegen.
   Nur wenn alle Kriterien erfüllbar sind, genaue Version und Features setzen.
2. Folgende Befehlsfolge ist nach dieser Auswahl direkt ausführbar. Die drei
   Variablen enthalten ausschließlich zuvor geprüfte Werte; bewusst keine
   erfundene Versionsnummer oder unbestätigte Featureliste voreingetragen.
   Sie ist Dokumentation, keine neue Bash-Datei oder Bash-Implementierung:

   ```sh
   cd /home/loomit/nixos-novena-image/tools/novena-bridge
   : "${BRIDGE_REQWEST_VERSION:?exakte geprüfte Version erforderlich}"
   : "${BRIDGE_REQWEST_FEATURES:?geprüfte komma-separierte Features erforderlich}"
   : "${BRIDGE_QUAL_TARGET_DIR:?freigegebenes externes Targetverzeichnis erforderlich}"
   export CARGO_TARGET_DIR="$BRIDGE_QUAL_TARGET_DIR"
   cargo add "reqwest@=$BRIDGE_REQWEST_VERSION" --no-default-features --features "$BRIDGE_REQWEST_FEATURES"
   ```

   cargo add ist hier die bewusst autorisierte Manifest-/Lockauflösung mit
   möglichem Indexdownload; noch kein --locked vortäuschen. Danach anhalten
   und Manifest-/Lockdiff prüfen: bestehende Pakete nicht unbegründet ändern,
   genau einen TLS-/Crypto-/Rootpfad und minimale Features sichern. Kein
   pauschales cargo update und kein zweiter Lockfile-Neuaufbau ohne Anlass.
   Ein separat nötiges Runtime-/Testcrate erst nach belegtem Bedarf ergänzen.
3. Den aufgelösten Lockgraph als Freigabeartefakt behandeln, anschließend
   vollständige Pakete beschaffen und Graph erfassen:

   ```sh
   cargo fetch --locked
   cargo metadata --locked --offline --format-version 1
   cargo tree --locked --offline --target x86_64-unknown-linux-gnu -e features
   cargo tree --locked --offline --target all -e normal,build,dev
   cargo tree --locked --offline -i reqwest -e features
   ```

   fetch ohne Targeteinschränkung beschafft auch Lock-Plattformanteile;
   Netzwerk-/Speicherfreigabe muss das umfassen. Metadata- und tree-Ausgaben
   innerhalb freigegebener Evidenzziele erhalten, Paketarchive gegen den
   geprüften Registry-/Lock-Kontext hashen. Hostgraph, übrige Plattformen und
   tatsächlich aktive Features unterscheiden. Rustls/Crypto/Roots danach mit
   den tatsächlich aufgelösten Paketnamen gezielt invers prüfen. Konflikte,
   unerwünschte Defaults, native Anforderungen oder fehlende Prüfpakete zuerst
   klären. Kontrollierter Offlineimport ist nur mit vollständigem Graph,
   Metadaten, Herkunft und Checksummen gleichwertig, kein Teilpaketimport.
4. Erst nach Dependencyabnahme den einen echten Adapter und gemeinsamen
   Empfangskontext implementieren; bestehende Rust-Prüfer wiederverwenden.
   Ohne qualifizierten Rohcommitweg nur den begrenzten Transportumfang
   ausweisen. Keine weitere Offline-Architektur und keine Pythonimplementierung.
5. Mit vollständigem Cache und autorisierten Testschreibzielen ausführen:

   ```sh
   cargo check --offline --locked --all-targets
   cargo fmt --all -- --check
   cargo clippy --offline --locked --all-targets -- -D warnings
   cargo test --offline --locked --all-targets -- --test-threads=1
   ```

   Bisherige 69 Tests als Regression erhalten. Zusätzlich echte lokale Rust-
   TLS-Tests für unbekannte CA, falschen Host, Zertifikatszeit, Handshakefehler,
   Redirect ohne Folgeanfrage, Proxy-/Retry-Sperre und Secretredaktion;
   Header-/Bodygrenzen, Fehlerbytes, Chunking/Trunkierung, Slow-Drip,
   DNS-/Connect-/TLS-/Header-/Gesamtdeadlines und gemeinsames REST-/Rawbudget
   einschließlich Readerneubau prüfen. Kontrollierten Resolver verwenden,
   keine öffentlichen DNS-Zugriffe aus lokalen Tests. Test-CA/-Hosts dürfen
   den Produktivkonstruktor nicht aufweichen. Bestehende Objekt-/Race-/CLI-
   Sperrtests weiter verwenden; neue Fehlfälle gezielt ergänzen. Jeder Befehl
   muss erfolgreich sein; bei Fehlern keine Live-Fortsetzung.
6. Erst im separat freigegebenen privaten GET-Auftrag Credentials und
   Evidenzablage festlegen, begrenzte echte Erhebung durchführen und
   Transport-, Objekt- und vollständige Readbackqualifikation getrennt
   dokumentieren. Fehlende Originalcommits bleiben Blocker. Produktive
   Veröffentlichung und last_verified-Promotion bleiben auch dann gesperrt.

Abschlusskontrolle 279: historische erste 1373 Zeilen per SHA256 erhalten
(vorher `fdfb9577c376a1939043c79092a9edf37fb834d2b0d8b1d90838f3f8f94b7b50`);
HEAD, Index, Cargo.toml und Cargo.lock wurden abschließend erneut geprüft
und stimmen mit den obigen Eingangswerten überein; Git-Status unverändert.
Werkzeugausgaben dieses Auftrags stehen im Chat; kein separates Rohlog oder
temporäres Artefakt angelegt. Keine vollständige neue Laufzeitqualifikation.

CHECKPOINT_279=DOCUMENTED; LOCAL_INVENTORY=COMPLETE_WITH_STATED_SCOPE;
BASELINE_278=69_TESTS_FMT_CLIPPY_PASS_USER_CONFIRMED;
PREFERRED_DIRECTION=REQWEST_RUSTLS; EXACT_HTTP_TLS_VERSIONS=UNVERIFIED;
HTTP_TLS_CACHE=INCOMPLETE; DEPENDENCY_CHANGES=NO; RUST_CHANGES=NO;
NEW_TESTS_OR_BUILDS=NO; NETWORK=NO; DOWNLOADS=NO; GIT_MUTATIONS=NO;
LIVE_HTTPS=NOT_IMPLEMENTED; RAW_COMMIT_ACQUISITION=UNQUALIFIED;
REAL_READBACK=UNQUALIFIED; LAST_VERIFIED=UNMODIFIED;
CHECKPOINT_PROMOTION=BLOCKED; BRIDGE_PUBLICATION=BLOCKED.

---

## NOVENA-BRIDGE-RUST-280-M4 – qualifizierter Entwicklungsabschluss, 2026-10-26

**Ergebnis: Dokumentationsabschluss für den vorhandenen Rust-Quellstand mit
erfolgreich übergebenen M3-Qualitätsprüfungen. Kein abgeschlossener
HTTPS-Laufzeittransport und kein vollständiger realer Readback. Veröffentlichung
bleibt ausdrücklich gesperrt.** Dieser Nachtrag ist der jüngste Wiedereinstieg;
alle bisherigen Checkpoints bleiben unverändert als historische Evidenz erhalten.
Der Abschluss von 280 bezeichnet diesen begrenzten Entwicklungsstand, keine
produktive oder fachliche Abnahme.

### Tatsächlicher Stand und M3-Evidenz

Grundlage sind AGENTS.md, README, die bisherigen Checkpoints, aktuelle
Rust-Schnittstellen und Implementierungen einschließlich Cargo.toml/Cargo.lock,
die drei neuen REST-Budgettests in tests/protocol.rs sowie bestehende
Offline-HTTP-, Readback-, Checkpointerhalt- und Publikationssperrtests.
Git-Zustand und einschlägige Projektverweise wurden ausschließlich lesend
geprüft. Vorbestehende Nutzeränderungen einschließlich der unversionierten
Rust-Crate bleiben erhalten. Es liegt kein unabhängiger M2-/M3-Commitdiff vor.

Der Nutzer übergibt für M3 folgende tatsächlichen Exitstatus:

| Prüfung | Exitstatus | Aussage |
|---|---:|---|
| cargo check | 0 | erfolgreich laut übergebenem M3-Ergebnis |
| cargo fmt --check | 0 | erfolgreich laut übergebenem M3-Ergebnis |
| cargo clippy | 0 | erfolgreich laut übergebenem M3-Ergebnis |
| cargo test | 0 | erfolgreich laut übergebenem M3-Ergebnis |

Diese vier Ergebnisse werden als Nutzerangaben dokumentiert, nicht als in M4
selbst ausgeführte Prüfungen. Vollständige Primärausgaben, genaue Optionen,
Testanzahl, Laufumgebung und Quellhashbindung sind in diesem Auftrag nicht
mitgeliefert; insbesondere werden weder --all-targets noch -D warnings oder
einzelne Testergebnisse nachträglich unterstellt. Die ältere M3-Notiz über
einen übergebenen Offline-Testlauf bleibt historisch erhalten; die jetzige
Mitteilung ergänzt die vier Qualitätsausgänge. Es ist kein Testfehler gemeldet.
Ein pauschales PASS für die gesamte Bridge folgt daraus nicht.

### Quellimplementierung gegenüber Laufzeitqualifikation

Vorhanden ist der Rust-Offlinekern mit strikter JSON-/Base64-Verarbeitung,
Git-Objektprüfung, REST-Allowlist, separater Rohcommitprüfung und lesenden
Offline-CLI-Adaptern. Die jüngste Quellergänzung begrenzt RestReader pro
Instanz auf kumulativ 32 MiB zurückgegebene Antwortbytes, zusätzlich zu
höchstens 256 Versuchen und 8 MiB je Antwort. Bytes werden vor Größen-,
JSON- und Formprüfung angerechnet; Überschreitung erschöpft das Budget.
Nach Erschöpfung findet kein weiterer Quellenaufruf statt. Quellenfehler
verbrauchen einen Versuch, aber liefern keine anrechenbaren Bodybytes.

Die drei neuen Tests prüfen die exakte 32-MiB-Grenze mit ausbleibendem
Folgeaufruf, ein Byte Überschreitung mit dauerhafter Sperre und die Anrechnung
abgewiesener JSON-/Form-/Übergrößenantworten. Ihre Assertions wurden gelesen;
M4 hat sie nicht separat ausgeführt. Bestehende Offline-HTTP-Tests prüfen
synthetische Methoden-/URL-/Status-/Redirectregeln, aufgezeichnete Zeitwerte
und den Anschluss an die vorhandene Readbackprüfung.

**Ein echter HTTPS-Adapter ist im vorliegenden Quellstand nicht implementiert.**
Das Manifest enthält weiterhin nur sha1, sha2 und serde_json; der Lockstand
enthält keinen reqwest-/rustls-Stack. OfflineHttpSource verarbeitet bereits
im Speicher vorhandene, vom Aufrufer gelieferte Envelopes. Ein https-URLstring
und ein geliefertes elapsed-Feld beweisen weder TLS noch eine durchgesetzte
Deadline oder authentifizierte Herkunft. Der frühere 280-M2-Blocker wegen
fehlender 280-M1-Dependencies bleibt für die HTTPS-Integration ungelöst;
der spätere REST-Budgetschritt schließt ausschließlich die dokumentierte
nachgelagerte Budgetlücke. Erfolgreiche Cargo-Prüfungen ersetzen diese
fehlende Implementierung und deren Laufzeitqualifikation nicht.

### Verbleibende TLS-/HTTP-Risiken und Abnahmelücken

- Dependencyauswahl und Provenienz: konkrete Client-/TLS-Versionen, Features,
  Crypto-Provider, CA-Quelle, MSRV, vollständiger Lockgraph, Lizenzen und
  aktuelle Sicherheitsbefunde sind weiterhin zu qualifizieren. reqwest mit
  genau einem rustls-Backend bleibt eine bevorzugte Richtung aus 279.
- TLS-Vertrauen: Ketten-, Hostnamen- und Zeitprüfung, SNI, Mindestversion,
  Truststorepflege und Widerrufsgrenzen sind an einem echten Stack zu prüfen.
  Ebenso fehlen qualifizierte Credentialzuführung, Secretredaktion und die
  nachgewiesene Nichtübernahme fremder Proxy-/CA-Umgebungswerte.
- HTTP-Verhalten: feste GET-/Host-/Pfadbindung, vollständige Redirect-, Proxy-
  und Retry-Sperren sowie Status-/Headerprüfung müssen im tatsächlichen Client
  durchgesetzt werden. Die synthetische Ablehnung ist kein Nachweis über
  versteckte Folgeanfragen, Verbindungswiederverwendung oder interne Retries.
- Ressourcen und Zeit: RestReader begrenzt Weiterverarbeitung, keinen Empfang
  oder Quellallokationen. Headerlimits vor unbeschränkter Parserallokation,
  Fehlerbytes, Framing, Chunking, Trunkierung, Content-Encoding und interne
  Puffer bleiben ungeprüft. Ein Readerneubau setzt sein Budget zurück;
  RawCommitSource liegt außerhalb. Ein gemeinsames Empfangskonto über REST
  und Rohcommits sowie echte monotone DNS-/Connect-/TLS-/Header-/Anfrage-/
  Gesamtdeadlines einschließlich Slow-Drip-Abbruch fehlen. Die Zahlen aus 279
  bleiben Planwerte; ein Bodybudget ist kein Prozessspeicherlimit.

Lokale echte TLS-/Socket-Negativtests und ein separat autorisierter privater
GET-/Readbacklauf stehen aus. M4 hat weder Netzwerk noch Builds oder Tests
ausgeführt. Auch ein späterer TLS-Erfolg würde keine fachliche Abnahme oder
atomaren HEAD-Verlauf beweisen.

### Rohcommitbeschaffung und verbindliche Sperren

RawCommitSource ist als separater Vertrag vorhanden, live jedoch unangebunden.
REST-Commit-JSON liefert Metadaten und belegt keine originalen Git-Commitbytes.
Eine authentifizierte, OID- und repositorygebundene Beschaffung unveränderter
Originalbytes bleibt offen und blockiert den vollständigen Live-Readback.
Kein rekonstruierter Commit, Patch-/Differsatz, unbelegter Accept-Header oder
normalisiertes JSON darf diese Lücke überdecken; die Allowlist wird dafür
nicht stillschweigend erweitert.

Der spätere Beschaffungsnachweis muss Originalbytes, angeforderte OID, SHA256,
Quelle/Repositorybindung, Erhebungsbedingungen und konkrete Client-/TLS-/CA-
Konfiguration ohne Secrets verbinden. check_commit prüft bereits Git-Commit-OID
sowie Tree/Parents gegen REST-Metadaten, authentifiziert aber nicht die Quelle.
Unabhängig gepinnte Repository-ID, HEAD vorher/nachher, vollständige Trees,
Status-/Baselineblobs und Originalprotokolle bleiben eigene Nachweispflichten.
Offline-Belege behalten OFFLINE_SUPPLIED_RESPONSES beziehungsweise
VARIANT_A_OFFLINE_V1; Payload und Beleg bleiben getrennt.

**BRIDGE_PUBLICATION=BLOCKED gilt unbedingt weiter.** production_publish
liefert PRODUCTION_PUBLICATION_DISABLED; die Publish-CLI weist entsprechende
Aufrufe vor dem Öffnen von Eingaben ab. Cargo publish=false bleibt bestehen.
Keine API-Mutation, keine Veröffentlichung, keine last_verified-Änderung und
keine Checkpoint-Promotion werden durch M3 oder diesen Abschluss freigegeben.
Der Dokumentationscheckpoint M4 ist keine Promotion des Statuspayloads.

### Nächster Block und Fehlerbehandlung

Nächster sachlicher Schritt ist die Klärung/Bereitstellung des fehlenden
gepinnten 280-M1-Dependencystands mit nachvollziehbarer Provenienz; anschließend
die in 279 vorbereitete echte Transportintegration und getrennte Qualifikation,
jeweils nur mit passendem neuen Auftrag. M4 erteilt dafür keine Freigabe.
Falls vollständige M3-Primärausgaben später einen Fehler oder eine abweichende
Quellbindung zeigen: betroffenen Befehl, Optionen, Exitstatus und konkrete
Fehlerstelle erhalten, keine PASS-Aussage für diesen Umfang fortschreiben und
keine Live-Fortsetzung starten. Im nächsten autorisierten Block zuerst diesen
Befund gezielt behandeln und danach die betroffene Prüfung und erforderliche
Regression erneut ausführen; hier keine zusätzliche Rustkorrektur.

M4 ändert ausschließlich diese Fortschrittsdatei durch Ergänzung. Keine
Rust-/Python-/Cargoänderung, kein Test-/Buildlauf, Netzwerkzugriff, Git-Mutation
oder Veröffentlichung. Der unveränderte historische Präfix umfasst 1744 Zeilen
mit SHA256 48258ac47b66910f3ace39c88a2e14a3dcd98b56d0d54ec1895cd3229a585574.

CHECKPOINT=NOVENA-BRIDGE-RUST-280-M4; M4_DOCUMENTATION=COMPLETE;
M3_CHECK_EXIT=0_USER_REPORTED; M3_FMT_EXIT=0_USER_REPORTED;
M3_CLIPPY_EXIT=0_USER_REPORTED; M3_TEST_EXIT=0_USER_REPORTED;
SOURCE_SCOPE=OFFLINE_CORE_AND_REST_RESPONSE_BUDGET;
LIVE_HTTPS=NOT_IMPLEMENTED; HTTP_TLS_RUNTIME=UNQUALIFIED;
RAW_COMMIT_ACQUISITION=UNQUALIFIED; REAL_READBACK=UNQUALIFIED;
M4_BUILDS_OR_TESTS_RUN=NO; RUST_CHANGES=NO; PYTHON_CHANGES=NO;
NETWORK=NO; GIT_MUTATIONS=NO; LAST_VERIFIED=UNMODIFIED;
CHECKPOINT_PROMOTION=BLOCKED; BRIDGE_PUBLICATION=BLOCKED.

## NOVENA-BRIDGE-RUST-281-M2 – Explizite HTTPS-GET-Quelle

Implementierungscheckpoint vom 26.10.2026, **UNGETESTET**. Der vom Auftrag
gesicherte M1-Stand ist reqwest 0.13.5 mit `blocking,rustls`; Cargo.toml und
Cargo.lock bleiben unverändert. Frühere Checkpoints bleiben vollständig
erhalten. Die historischen PASS-Angaben gelten nicht als Qualifikation dieser
neuen Implementierung.

### Implementierter Transportumfang

`src/https.rs` ergänzt `HttpsGetSource` als explizit konstruierbare
`ResponseSource`. Kein Default, kein frei übergebbarer Client, kein freier URL
und keine CLI-Anbindung. Erst `get` führt Netzwerkcode aus. Die bestehende
`allowed_endpoint`-Prüfung erfolgt vor URLkonstruktion und Sendung: ausschließlich
HTTPS auf api.github.com für das feste Repository, den main-Ref und die
OID-gebundenen Git-Commit-/Tree-/Blob-Endpunkte. Ausschließlich GET; kein
GraphQL, Contents, recursive-Query, alternativer Host oder Rawcommitweg.

Der Client verwendet reqwest/rustls, HTTP/1, maximal 100 Headerfelder,
`Policy::none`, `no_proxy`, `retry::never`, keine idle Connection-Pools,
keine automatische Kompression, TLS mindestens 1.2, SNI und deaktiviertes
SSL-Keylogging. Zertifikats-/Hostnamenprüfung wird nicht abgeschaltet.
`tls_certs_only` verwendet ausschließlich explizite CA-DER-Eingaben statt
System-/Umgebungswurzeln. CA-Auswahl und Provenienz sind Aufruferpflicht;
hier wird kein freigegebenes produktives Rootbundle bereitgestellt.
Optionales Token ausschließlich als explizites Konstruktorargument, ohne
Credential-Erkennung; Headerwert wird auf ASCII ohne Whitespace begrenzt und
sensibel markiert. Kein Debug/Serialize des Quelltyps. Keine sichere
Speicherlöschung zugesagt. Ohne Token erfolgen anonyme GETs; der Zugriff auf
das private Archiv ist damit nicht nachgewiesen.

Feste Requestheader: Accept `application/vnd.github+json`, API-Version
`2022-11-28`, User-Agent `novena-bridge/0.1`, Accept-Encoding `identity`.
Ihre aktuelle GitHub-seitige Eignung wurde wegen des Netzwerkverbots nicht
gegen aktuelle Primärdokumentation verifiziert und bleibt zu qualifizieren.
Nur Status 200 ohne Location und ohne Content-Encoding außer identity wird
angenommen. Fehlerbodies werden nicht aktiv gelesen. Client-/I/O-Fehler werden
ausschließlich auf statische BridgeError-Codes abgebildet, ohne URL, Token,
Header, Body oder ursprüngliche Fehlermeldung auszugeben.

1..256 Versuche je Quelle, auch fehlgeschlagene Sendungen zählen; keine
Wiederholung im Adapter. Jeder Fehler schließt die Quelle für weitere
Versuche. Bodyempfang über festen 8192-Byte-Puffer, konfigurierbar 1..8 MiB
pro Antwort und 32 MiB insgesamt je Quellinstanz. Eine Überschreitungsprobe
liest höchstens ein zusätzliches Byte und belastet ebenfalls das Gesamtbudget.
Am exakten Gesamtlimit wird konservativ abgebrochen, wenn EOF einen weiteren
Read benötigen würde; es gibt keine ungezählte Probe über dieses Limit.
Content-Length ist nur eine frühe Ablehnungsprüfung, nicht die Empfangsgrenze.
Reader-Neubau mit derselben Quelle setzt ihr Budget nicht zurück; eine neue
Quelle ist ein neuer, ausdrücklich konstruierter Vorgang. Die zusätzliche
32-MiB-Grenze des RestReader bleibt unverändert.

`src/lib.rs` exportiert das neue Modul und korrigiert die bisher pauschale
Netzwerkfreiheitsaussage. `src/protocol.rs` aktualisiert ausschließlich die
Modul-/Quellkommentare. OfflineHttpSource, RestReader und sämtliche vorhandenen
Git-Objektprüfungen bleiben funktional unverändert. `src/main.rs`,
`src/objects.rs`, `src/planning.rs` und `src/attestation.rs` bleiben unverändert.
Die produktive CLI erhält keinen HTTP-Pfad. Rohcommitbeschaffung bleibt separat
und unqualifiziert; REST-Commit-JSON wird niemals zu Originalcommitbytes
rekonstruiert. Offlineberichte werden nicht zu Live-Attestationen umetikettiert.

### Vorbereitete deterministische Tests, nicht ausgeführt

Sieben Unit-Tests in `src/https.rs`: exakte Zielbindung und abgelehnte Pfade,
reqwest-Builderbeobachtung für Redirects/HTTP1/fehlende explizite Proxies,
Credentialvalidierung und sensible Header, Status-/Location-/Encodingablehnung,
endliche Versuchslimits und monotone Zeitprüfungen, stückweiser Bodyempfang
einschließlich Probe/Gesamtbudget sowie leere Bodies/I/O-Fehler/abgelaufene Reads.
Tests verwenden Speicherreader und injizierte Zeitbeobachtung; weder Socket
noch DNS noch externe Netzwerkzugriffe. Der Buildertest baut keinen Client.
Seine Debugbeobachtung beweist weder die Systemproxysperre noch Laufzeitverhalten.

Vorhandene Tests werden wiederverwendet: `tests/protocol.rs` prüft die separate
32-MiB-REST-Grenze, Git-Objekt-/Rohcommitverträge und ausbleibende Promotion;
`tests/cli.rs::production_disabled_before_any_input_is_opened` prüft die
unveränderte Publikationssperre. Keine dieser Prüfungen wurde in M2 gestartet.
Auch Cargo check, Rustfmt und Clippy wurden nicht ausgeführt: keine PASS-Aussage.

### Tatsächliche API-Evidenz und verbleibende Grenzen

Lesend geprüft wurden die lokalen beschafften reqwest-0.13.5-Quellen unter
`src/blocking/client.rs`, `src/blocking/request.rs`, `src/blocking/response.rs`,
`src/async_impl/client.rs`, `src/retry.rs` und `src/redirect.rs` im Cargo-Cache.
Sie bieten `no_proxy` einschließlich Abschaltung automatischer Systemproxies,
`retry::never` ausdrücklich auch gegen die voreingestellten Protokoll-NACK-Retries,
Redirectsperre, explizite Roots und `http1_max_headers`. Das ist eine
Quell-/API-Sichtung, kein Build-, Featuregraph- oder Wire-Nachweis.

Explizit konfiguriert: Connecttimeout 10 s und Request-/Blockingwartezeit 30 s.
Monotone Prüfungen setzen zwischen Reads eine Grenze von 30 s pro Anfrage
und 120 s seit erstem Versuch, mit verkürzter Requestwartezeit am Vorgangsende.
Der Blocking-Response-Read beginnt jedoch jeweils eine neue reqwest-Wartefrist.
Ein bereits blockierender Read kann die monotone Grenze überschreiten; ebenso
ist die Abbruchwirkung auf DNS-Hintergrundarbeit hier nicht nachgewiesen.
Damit sind die geforderte harte Gesamtdeadline und getrennte 10-s-Headerdeadline
**nicht durchgesetzt**. Nachträgliche Zeitprüfung verwirft verspätete Daten,
beweist aber keine rechtzeitige Unterbrechung.

Eine einstellbare 32-KiB-Headerbytegrenze vor Parserallokation fehlt in der
gesichteten Blocking-API. 100 Headerfelder sind kein Headerbyte-/Speicherlimit.
Interne TLS-/HTTP-Puffer und bereits vor Statusprüfung eingelesene oder beim
Abbruch verworfene Bytes sind nicht im Anwendungsbudget messbar; die 32 MiB
begrenzen zurückgelesene Bodybytes, nicht sämtliche Netzwerkbytes oder den
Prozessspeicher. Fehlerbodies werden nicht vollständig abgerufen/gezählt,
die Quelle wird stattdessen geschlossen. Kein gemeinsames Budget mit einem
künftigen Rohcommittransport implementiert. Framing/Chunking/EOF-Verhalten,
TLS-Ketten-/Hostname-/Zeitprüfung, tatsächliche Proxy-/Retryfreiheit und
Slow-Drip-Verhalten benötigen separate freigegebene Laufzeitqualifikation.
Keine externen Security-/Revokations-/Rootbundle-Prüfungen durchgeführt.

Diese Lücken bleiben Integrationsblocker gegenüber dem vollständigen Vertrag
aus 279; die explizite Bibliotheksquelle ist keine produktive Transportfreigabe.
Live-REST-Provenienz, OID-gebundene originale Rohcommitbeschaffung und vollständige
unabhängige Live-Readback-Evidenz fehlen weiterhin. last_verified unverändert,
Checkpoint-Promotion und produktive Veröffentlichung weiterhin gesperrt.

CHECKPOINT=NOVENA-BRIDGE-RUST-281-M2; IMPLEMENTATION=PREPARED_UNTESTED;
LIVE_HTTPS_SOURCE=EXPLICIT_LIBRARY_ONLY; CLI_HTTP=DISABLED;
TESTS=PREPARED_NOT_RUN; FMT=NOT_RUN; CLIPPY=NOT_RUN; BUILDS=NOT_RUN;
NETWORK=NO; CARGO_FILES=UNMODIFIED; GIT_MUTATIONS=NO;
RAW_COMMIT_ACQUISITION=UNQUALIFIED; REAL_READBACK=UNQUALIFIED;
HARD_DEADLINES=NOT_ENFORCED; HEADER_BYTE_LIMIT=NOT_ENFORCED;
LAST_VERIFIED=UNMODIFIED; CHECKPOINT_PROMOTION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED.


## NOVENA-BRIDGE-RUST-281-M4 – Dokumentation des HTTPS-Entwicklungsstands

Dokumentationscheckpoint vom 26.10.2026. Grundlage: aktuelle Rust-Quellen,
Tests, Cargo.toml/Cargo.lock, lokal vorhandene Cargo-Quellen und die vom
Auftrag gemeldeten äußeren Prüfergebnisse. Frühere Checkpoints einschließlich
der damaligen UNGETESTET-Aussage von 281-M2 bleiben historische Befunde.
Dieser Nachtrag ist keine Statuspayload-Promotion und keine Live-Qualifikation.

### Tatsächlich vorhandene Dependencies

Cargo.toml enthält vier direkte Dependencies; Cargo.lock löst sie wie folgt auf:

| Dependency | Manifest | Aufgelöste Version |
| --- | --- | --- |
| reqwest | exakt =0.13.5, default-features=false, blocking und rustls | 0.13.5 |
| sha1 | 0.10 | 0.10.7 |
| sha2 | exakt =0.10.9 | 0.10.9 |
| serde_json | 1 | 1.0.154 |

Alle vier Versionen sind als entpackte Quellen im lokalen Cargo-Registrycache
vorhanden. Ebenso lokal vorhanden und im Lockfile verzeichnet sind insbesondere
rustls 0.23.46, hyper-rustls 0.27.10, hyper 1.12.0, tokio 1.54.1 und
aws-lc-rs 1.18.2. Das konkretisiert den inzwischen bereitgestellten
Dependencystand; ältere Aussagen über fehlendes reqwest beschreiben nicht mehr
diesen Quellstand. Cachevorhandensein und Lockfile belegen weder den Zeitpunkt
oder Weg der Installation noch den vollständigen aktivierten Ziel-/Featuregraph.
Ein Lockfileeintrag allein beweist keine aktive Nutzung eines optionalen oder
plattformabhängigen Pakets. Kein cargo tree, Download oder Installationslauf
wurde für M4 ausgeführt; eine vollständige Beschaffungsprovenienz wird hier
nicht nachträglich behauptet.

### Implementierter HTTPS-Transportumfang

src/https.rs exportiert HttpsGetSource als explizite Bibliotheksquelle für
ResponseSource. Nur GET auf https://api.github.com, festes Repository
loomit77/project-block-logs, Repositorymetadaten, main-Ref und OID-gebundene
Git-Commit-/Tree-/Blob-Endpunkte sind zugelassen. Die Pfadprüfung erfolgt vor
URLkonstruktion und Sendung. Kein frei wählbarer Host, URL oder Client,
keine CLI-Anbindung, kein GraphQL-/Mutations- oder Rohcommittransport.

Der Builder konfiguriert reqwest/rustls mit HTTP/1, maximal 100 Headerfeldern,
TLS mindestens 1.2, SNI, expliziten CA-DER-Roots, deaktiviertem Keylogging,
Redirect-/Proxy-/Retry-Sperren und deaktivierter automatischer Dekompression.
Keine Abschaltung der Zertifikats-/Hostnamenprüfung. Optionales Bearertoken
kommt ausschließlich vom Aufrufer; sensible Header und statische Fehlercodes
vermeiden die direkte Ausgabe ursprünglicher Clientfehler. Kein produktives
CA-Bundle, keine automatische Credentialerkennung und keine zugesagte sichere
Speicherlöschung. Diese Konfiguration ist noch kein Nachweis ihres Wire-Verhaltens.

Nur HTTP 200 ohne Location und mit fehlendem oder ausschließlich identity-
Content-Encoding wird akzeptiert. Connecttimeout 10 s, Requestwartezeit 30 s;
monotone Prüfungen zwischen Reads berücksichtigen 30 s pro Anfrage und 120 s
pro Vorgang. 1..256 Versuche, 1..8 MiB Body pro Antwort, 32 MiB insgesamt je
Quellinstanz, 8192-Byte-Lesepuffer; die zusätzliche Größenprobe wird mitgezählt.
Nach jedem get-Fehler wird die Quelle geschlossen. Am ausgeschöpften
Gesamtbudget erfolgt kein zusätzlicher ungezählter EOF-Read. Die separate
RestReader-Grenze und bestehende Objekt-/Transitionsprüfungen bleiben bestehen.

### Neue Testfälle und konkrete äußere Ergebnisse

Sieben neue Unit-Testfunktionen in src/https.rs decken ab:

1. exact_targets_and_no_url_normalization: feste Ziele und negative URL-/Pfadvarianten.
2. reqwest_builder_has_redirects_disabled_and_no_explicit_proxies: Builder-Debugbeobachtung ohne Clientbau.
3. credentials_are_explicit_sensitive_and_errors_are_static: explizite Credentials, sensible Header und statische Fehler.
4. statuses_locations_and_encodings_fail_closed: Status-, Location- und Encodingablehnung.
5. finite_attempts_and_monotonic_deadline_checks: Limitvalidierung, Versuchszahl, Zeitgrenze und geschlossene Quelle.
6. streaming_limits_charge_probe_and_do_not_read_beyond_total_budget: Bodygrenze, gezählte Probe und Gesamtbudget.
7. empty_bodies_read_errors_and_expired_reads_are_rejected: leere Bodies, Lesefehler, Secretredaktion und verspätete Reads.

Die Tests nutzen Speicherreader und injizierte Zeitbeobachtungen. Sie führen
keinen echten TLS-Handshake, DNS- oder Sockettest aus. Der Buildertest beweist
insbesondere keine Laufzeit-Proxy-/Retryfreiheit. Vorhandene Protokolltests
prüfen weiterhin synthetischen Readback, Rohcommitverträge und Budgetgrenzen;
der CLI-Test production_disabled_before_any_input_is_opened prüft die Sperre.

| Äußere Prüfung laut Auftrag | Exitstatus | Einordnung |
| --- | --- | --- |
| cargo check | 0 | als erfolgreich gemeldet |
| cargo fmt | 1 | FEHLER; Formatierungsprüfung nicht erfolgreich |
| cargo clippy | 0 | als erfolgreich gemeldet |
| cargo test | 0 | als erfolgreich gemeldet |

**Kein Gesamt-PASS für 281.** Der Formatierungsfehler bleibt offen und wurde
hier nicht korrigiert. Vollständige Befehlszeilen, Optionen, Rohoutputs,
Testanzahl und eine Bindung des äußeren Laufs an einen unveränderlichen
Quellstand wurden mit diesen vier Exitcodes nicht übermittelt. Daher keine
Behauptung über konkrete Formatierungsdiffs, einzelne Testergebnisse,
Warnungsfreiheit, --locked/--offline oder eine vollständige Runner-Abnahme.
M4 hat keine dieser Prüfungen erneut gestartet und keine Rohausgaben erfunden.

### Verbleibende Risiken, Blockaden und fehlender echter Rohcommit-Readback

Harte abbrechbare Gesamtdeadlines einschließlich blockierender Reads und DNS
sowie eine getrennte Headerdeadline sind nicht nachgewiesen/durchgesetzt.
Die nachträgliche monotone Prüfung ersetzt keinen rechtzeitigen Abbruch.
Eine 32-KiB-Headerbytegrenze vor Parserallokation fehlt; 100 Headerfelder sind
kein Byte- oder Prozessspeicherlimit. Interne HTTP-/TLS-Puffer, vorab gelesene
und verworfene Fehlerbytes sind nicht vollständig im Bodybudget erfasst.
Ein gemeinsames Budget mit künftigem Rohcommittransport fehlt. Framing,
Chunking, Trunkierung, Slow-Drip-Abbruch, tatsächliche Redirect-/Proxy-/Retry-
Sperren und Zertifikatsketten-/Hostname-/Zeitprüfung bleiben lokal unqualifiziert.
CA-Provenienz, Truststorepflege, Widerrufsgrenzen und die konkrete Secretzuführung
benötigen eigene Evidenz. Auch die serverseitige Eignung der festen API-Header
wird hier ohne Netzwerkprüfung nicht neu bestätigt.

RawCommitSource existiert als separater Vertrag, ist aber nicht an eine echte
Beschaffung angebunden. REST-Commit-JSON liefert keine nachgewiesenen originalen
Commitbytes. Es fehlt ein authentifizierter, repository- und OID-gebundener
Rohcommit-Readback einschließlich Originalbytes, Git-OID, SHA256, Quelle und
Erhebungsbedingungen. Keine Rekonstruktion aus JSON oder Patch-/Diffdaten als
Ersatz zulässig. Damit fehlen vollständiger unabhängiger Live-Readback und
Live-Provenienz weiterhin; synthetische Offline-Belege schließen diese Lücke
nicht. last_verified bleibt unverändert, Checkpoint-Promotion gesperrt.
BRIDGE_PUBLICATION=BLOCKED; production_publish/Publish-CLI bleiben gesperrt,
Cargo publish=false bleibt bestehen.

### Nächste Schritte zur lokalen TLS-Qualifikation

Für einen gesondert autorisierten Folgeauftrag, hier nicht ausgeführt:

1. Äußeren fmt-Rohbefund und genaue Befehle sichern, Formatierung gezielt
   korrigieren und betroffene Checks mit erhaltenen Primärausgaben wiederholen.
2. Lokalen TLS-Testaufbau mit synthetischer CA, gültigem Serverzertifikat für
   api.github.com und kontrolliertem Loopbackserver entwerfen. Eine ausschließlich
   testseitige Zielauflösung muss die feste Host-/SNI-/Pfadbindung erhalten;
   keine freie URL oder Abschaltung der Zertifikatsprüfung als Produktoption.
   Socket-/Loopbackfreigabe und erforderliche Entwicklungsänderungen separat klären.
3. Positiven HTTPS-GET sowie unbekannte CA, falschen Hostnamen, abgelaufene/
   noch nicht gültige Zertifikate und unvollständige Ketten prüfen. Fremde
   Proxy-/CA-Umgebung injizieren; Server-/Proxybeobachtungen müssen belegen,
   dass keine fremden Roots, Folgeanfragen oder versteckten Retries greifen.
4. Redirects/Location, Fehlerstatus, Encoding, übergroße Header/Bodies,
   Chunking/Trunkierung, Verbindungsabbruch und Slow-Drip gezielt prüfen;
   Versuchszahl, empfangene Bytes, Abbruchzeiten und Geheimnisfreiheit der
   Ausgaben erfassen. Fehlende harte Deadline-/Headergrenzen gezielt beheben
   oder als fortbestehende Integrationsblocker dokumentieren.
5. Client-/TLS-/Featurestand, CA-Provenienz, Testbedingungen und vollständige
   lokale Rohlogs erhalten. Lokale TLS-Erfolge getrennt von privatem Remote-
   Zugriff, echter Rohcommitbeschaffung und fachlicher Abnahme bewerten.

Nur diese Fortschrittsdatei wurde ergänzt. Keine Builds, Tests, Formatierung,
Netzwerkzugriffe, Python-Arbeit, Git-Mutationen oder Veröffentlichung in M4.

CHECKPOINT=NOVENA-BRIDGE-RUST-281-M4; M4_DOCUMENTATION=COMPLETE;
OUTER_CHECK_EXIT=0_USER_REPORTED; OUTER_FMT_EXIT=1_USER_REPORTED;
OUTER_CLIPPY_EXIT=0_USER_REPORTED; OUTER_TEST_EXIT=0_USER_REPORTED;
OVERALL_RESULT=NOT_PASS; FORMAT_FAILURE=OPEN;
LIVE_HTTPS_SOURCE=EXPLICIT_LIBRARY_ONLY; CLI_HTTP=DISABLED;
LOCAL_TLS_RUNTIME=UNQUALIFIED; RAW_COMMIT_ACQUISITION=UNQUALIFIED;
REAL_READBACK=UNQUALIFIED; HARD_DEADLINES=NOT_ENFORCED;
HEADER_BYTE_LIMIT=NOT_ENFORCED; M4_BUILDS_OR_TESTS_RUN=NO;
NETWORK=NO; PYTHON_WORK=NO; GIT_MUTATIONS=NO;
LAST_VERIFIED=UNMODIFIED; CHECKPOINT_PROMOTION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED.

---

## NOVENA-BRIDGE-RUST-282 – HTTPS-Rust-Qualifikation, 2026-10-26

Ausgangslage: Block 281 hat reqwest 0.13.5 mit rustls
und die explizit aktivierbare Rust-HTTPS-Quelle
HttpsGetSource implementiert.

Block 281 meldete erfolgreiche Kompilierung, Clippy
und 76 bestandene Tests, jedoch Format-Exit 1.

Block 282 hat die Rust-Formatierung korrigiert und
anschließend sämtliche Qualitätsprüfungen erneut
ausgeführt:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --all-targets: Exit 0

Gesamtergebnis der lokalen Offlinequalifikation:
PASS

Die Originalausgaben befinden sich im archivierten
Runner-Protokoll NOVENA-BRIDGE-RUST-282.

Die Source-Snapshots BASELINE und FORMATTED sowie
der Abschluss-Snapshot sind separat mit SHA256
gesichert.

Gültigkeitsgrenze:
Ein bestandener Offline-Testlauf beweist keine
erfolgreiche echte TLS-Verbindung zu GitHub.
Lokale TLS-Laufzeittests und authentifizierte
Rohcommitbeschaffung bleiben eigenständige Aufgaben.

Die produktive Veröffentlichung bleibt gesperrt.
last_verified wird nicht fortgeschrieben.

RUST_282_CHECK=0;
RUST_282_FMT=0;
RUST_282_CLIPPY=0;
RUST_282_TEST=0;
RUST_282_QUALITY=PASS;
LIVE_GITHUB_READBACK=UNQUALIFIED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-283-M2 – Lokale TLS-Laufzeittests vorbereitet, 2026-10-26

### Technische Prüfung und Implementierung

Ausgangsstand ist der dokumentierte Block 282 mit 76 Tests PASS sowie
erfolgreichem check, Rustfmt und Clippy. Diese Ergebnisse gelten für dessen
Quellstand, nicht automatisch für die folgenden Änderungen. Vorhandene
Nutzeränderungen und ältere Checkpoints wurden erhalten.

Geänderte beziehungsweise ergänzte Dateien dieses Schritts:

- src/https.rs: ausschließlich testseitiger Konstruktor local_test mit fester
  Auflösung api.github.com auf 127.0.0.1 und einem lokalen Testport. Portfeld
  und URL-Portanpassung sind vollständig cfg(test). Zielprüfung, HTTPS,
  Hostname, SNI, Zertifikatsprüfung, gemeinsamer client_builder und echter
  fetch-/ResponseSource-Pfad bleiben erhalten. Keine neue produktive Option,
  kein neuer CLI-Befehl und kein frei übergebbarer Client oder Host.
- src/https_local_tls.rs: fünf Unit-Tests mit Präfix local_tls_, Rust-Testhülle
  für den lokal vorhandenen OpenSSL-TLS-Server und synthetische Zertifikate.
- BRIDGE-CODEX-PROGRESS.md: dieser Zwischenstand.

Cargo.toml und Cargo.lock bleiben unverändert. reqwest 0.13.5/blocking/rustls
ist der bestehende echte Client; dessen lokal vorhandene Builder-API wurde
lesend geprüft. Transitive rustls-Einträge im Lockfile ersetzen keine direkt
nutzbare Serverdependency. Statt einer zusätzlichen Crate verwendet die
Rust-Testhülle das bereits lokal verfügbare Programm openssl (lesend ermittelt:
OpenSSL 3.6.5). Es wurde keine Dependency beschafft und keine zweite
HTTP-Clientimplementierung hinzugefügt.

Der äußere Testlauf benötigt openssl im PATH mit req, x509 und s_server sowie
lokale Prozess-, Temporärdatei- und Loopbackrechte. Fehlt OpenSSL oder startet
der Server nicht, schlagen die Tests fehl; kein stilles Überspringen. Im
Testlauf erzeugt jeder Test zwei lokale selbstsignierte CAs und ein damit
signiertes Serverzertifikat mit SAN und serverAuth, Laufzeit zwei Tage ab
Erzeugung. Es werden keine gespeicherten Remoteantworten, produktiven
Credentials oder externen Zertifikatsquellen genutzt. Der Testtruststore
enthält genau die gewählte synthetische CA. Schlüssel/Dateien werden im
jeweiligen neu angelegten temporären Verzeichnis erzeugt und per Drop entfernt;
Serverprozesse werden auch bei normalen Testpanics beendet und eingesammelt.
Ein harter Prozessabbruch kann temporäre Dateien hinterlassen.

s_server erhält ausschließlich -4 -accept 127.0.0.1:PORT. Ein kurz reservierter
OS-Port wird an OpenSSL übergeben; die verbleibende Portübergabelücke kann
zu einem Testfehler führen. Serverbereitschaft wird über dessen ACCEPT-Ausgabe
mit fünf Sekunden Wartegrenze erkannt, ohne Probeverbindung. Die Hülle hängt
somit vom lokalen OpenSSL-Ausgabeformat ab. Reqwest löst den festen Host über
die testseitige resolve-Zuordnung auf; kein externer DNS-/GitHubzugriff ist
Teil des Testentwurfs. Redirectziele verwenden ebenfalls nur diesen Host.

### Durch den äußeren Lauf real prüfbare Eigenschaften

1. local_tls_trusted_ca_handshake_and_single_get: echter TLS-Handshake und
   HTTP-GET über reqwest, erfolgreicher Zwei-Byte-Body, fest gebundener
   Requestpfad/Hostheader und synthetischer Bearerheader. Ein zweiter expliziter
   Aufruf scheitert am Requestlimit; der Server muss genau einen GET sehen.
2. local_tls_unknown_ca_rejected_before_http: das Serverzertifikat ist von der
   anderen CA signiert als der explizit vertrauten; Requestfehler und kein
   entschlüsselter HTTP-Request am Server werden erwartet.
3. local_tls_wrong_hostname_rejected_before_http: vertraute Signatur, aber SAN
   wrong.invalid bei unverändertem URL-/SNI-Host api.github.com; Requestfehler
   und kein HTTP-Request werden erwartet. Keine deaktivierte Hostnamenprüfung.
4. local_tls_http_redirect_rejected_without_followup_or_secret_leak: über TLS
   empfangener HTTP-302 mit HTTP-Location wird statisch abgelehnt, ohne zweiten
   GET. Location und Antwort enthalten synthetische Geheimnismarker.
5. local_tls_malformed_http_rejected_without_retry_or_secret_leak: über TLS
   empfangene ungültige HTTP-Statuszeile wird vom echten Client abgelehnt;
   genau ein GET und keine Marker in Display/Debug des Bridgefehlers.

Alle Negativfälle prüfen zusätzlich HTTPS_SOURCE_CLOSED bei einem weiteren
expliziten Aufruf sowie Geheimnisfreiheit des ursprünglichen Bridgefehlers.
Serverbeobachtung zählt entschlüsselte GET-Anforderungen einschließlich
möglicher Folgeanfragen innerhalb desselben Requests. Das zusätzliche
Beobachtungsfenster beträgt 250 ms nach den Clientaufrufen. Dies ist kein
universeller Nachweis gegen beliebig spät stattfindende Anfragen oder gegen
weitere TCP-/TLS-Versuche ohne HTTP-Daten. Unbekannte CA und falscher Host
liefern absichtlich denselben statischen Clientfehler; der konkrete
Negativstimulus kommt aus dem kontrollierten Zertifikataufbau. Das Positiv-
gegenstück ist für die Bewertung der Negativfälle erforderlich.

### Prüfstatus und fortbestehende Grenzen

In diesem Auftrag wurden keine Cargo-Builds, Cargo-Checks, Cargo-Tests oder
TLS-Tests gestartet. Nur die geänderten Rust-Quelldateien wurden mit dem lokal
vorhandenen rustfmt formatiert. Die neuen Tests sind implementiert, aber
**nicht als PASS qualifiziert**. Die vier äußeren Runnerprüfungen einschließlich
Clippy und tatsächlicher TLS-Handshakes stehen aus. Es gab keine Netzwerk-
verbindung, Registrybeschaffung, Git-Schreibaktion, Publikation oder Promotion.

Die Tests beheben keine bestehenden Transportgrenzen: 100 Headerfelder sind
keine 32-KiB-Headerbytegrenze vor Parserallokation. Harte abbrechbare absolute
Gesamt-/Headerdeadlines einschließlich DNS und blockierender Reads bleiben
unbelegt; monotone Prüfungen zwischen Reads ersetzen keinen rechtzeitigen
Abbruch. Bodylimits begrenzen nicht sämtliche internen HTTP-/TLS-Puffer,
vorab gelesene Fehlerbytes oder den Prozessspeicher. Kein gemeinsames Budget
mit künftigem Rohcommittransport. Slow-Drip, große Header/Bodies, Trunkierung,
Chunking, Zertifikatszeitfehler, unvollständige Ketten, Widerruf und injizierte
Proxy-/CA-Umgebung werden durch diese fünf Tests nicht vollständig qualifiziert.
Die Secretprüfung umfasst Bridgefehler, keine Garantie sicherer Speicherlöschung
oder Geheimnisfreiheit beliebiger externer Prozessdiagnosen.

RawCommitSource bleibt ein separater, nicht an authentifizierte echte
Beschaffung angebundener Vertrag. Originale repository-/OID-gebundene
Commitbytes samt Git-OID, SHA256, Quelle und Erhebungsbedingungen fehlen
weiterhin; REST-JSON oder daraus rekonstruierte Bytes ersetzen sie nicht.
Lokale TLS-Tests belegen weder Zugriff auf das private GitHubarchiv noch
Remote-Frische, CA-Bundle-Provenienz, Credentialzuführung oder vollständigen
unabhängigen Live-Readback. Diese Blockaden und die Publikationssperre bleiben
bestehen. last_verified wird nicht fortgeschrieben.

CHECKPOINT=NOVENA-BRIDGE-RUST-283-M2;
LOCAL_TLS_TESTS=IMPLEMENTED_NOT_RUN; NEW_TLS_TEST_FUNCTIONS=5;
CARGO_BUILDS_OR_TESTS_RUN=NO; OUTER_RUNNER_QUALIFICATION=PENDING;
RAW_COMMIT_ACQUISITION=UNQUALIFIED; LIVE_READBACK=UNQUALIFIED;
HEADER_BYTE_LIMIT=NOT_ENFORCED; HARD_DEADLINES=NOT_ENFORCED;
EXTERNAL_NETWORK=NO; MANIFEST_CHANGES=NO; GIT_MUTATIONS=NO;
LAST_VERIFIED=UNMODIFIED; CHECKPOINT_PROMOTION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED.

---

## NOVENA-BRIDGE-RUST-283-M3 – Lokale TLS-Qualifikation, 2026-10-26

Ausgangsstand: Block 282 mit 76 bestandenen Rust-Tests,
erfolgreicher Formatierung und Clippy-Prüfung.

Codex erhielt den Auftrag, echte lokale TLS-
Integrationstests ohne öffentlichen Netzwerkzugriff
zu ergänzen.

Äußere Prüfungen:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --all-targets: Exit 0

Rust-Qualität: PASS

Anzahl gefundener Testdefinitionen mit Präfix
local_tls_: 5

Die tatsächliche Testausführung und die Grenzen
der einzelnen Tests sind ausschließlich anhand
des vollständigen Runner-Originalprotokolls
zu bewerten.

Ein positives Offline-Testergebnis beweist
nicht automatisch eine echte GitHub-Verbindung.

Original-Rohcommitbeschaffung, unabhängiger
privater Readback und produktive Veröffentlichung
bleiben gesondert ungeklärt beziehungsweise gesperrt.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-284-CODEX – HTTPS-Empfangsgrenzen vorbereitet, 2026-10-26

### Grundlage und Ist-Zustand

Auftragsgemäßer Ausgangsstand: Block 283 mit 81 erfolgreichen Rust-Tests,
darunter fünf lokalen TLS-Tests, sowie erfolgreichem check, Rustfmt und
Clippy. Der bestehende Checkpoint 283-M3 bestätigt die äußeren Prüfungen;
ein vollständiges Runner-Originalprotokoll wurde für diesen Schritt nicht
gefunden. Die Ausgangsqualifikation wird nicht auf neue Quellen übertragen.
Vorhandene Nutzeränderungen bleiben erhalten. README, Plan, bisherige
Checkpoints, HTTPS-Adapter, Protokollverträge und Protokolltests wurden gelesen.
Die ältere README-Einleitung ohne HTTP-Client beschreibt den jüngeren
expliziten Bibliotheksadapter nicht vollständig; maßgeblich für diese
Untersuchung sind src/https.rs und die späteren Transportcheckpoints.

**Tatsächlich im Adapter durchgesetzte Grenzen:** Nur allowlistengeprüfte
GET-Pfade am fest konstruierten HTTPS-Ziel api.github.com; keine freien URLs,
Redirects, automatischen Wiederholungen oder Proxyerkennung. Rustls prüft
gegen explizite Roots und den Hostnamen, TLS mindestens 1.2. Der Builder
konfiguriert höchstens 100 HTTP/1-Headerfelder. Vor Netzwerkausführung werden
Versuche (höchstens 256), Restbudget und abgelaufene Operationszeit geprüft.
Das Empfangsbudget begrenzt die aus Read übernommenen Bytes auf insgesamt
32 MiB; das Antwortlimit ist konfigurierbar bis 8 MiB. Eine zusätzliche
Byteprobe erkennt Überschreitung und wird ebenfalls berechnet. Am exakten
Gesamtlimit wird ohne ungezählte EOF-Probe konservativ abgebrochen. Fehler
schließen die Quelle dauerhaft. Statische Bridgefehler übernehmen weder
reqwest-Diagnosen noch Header-, Body-, URL- oder Tokeninhalte.

**Prüfungen nach vorgelagerter Verarbeitung:** Status exakt 200, keine Location
und ausschließlich fehlendes beziehungsweise identity-Content-Encoding
werden erst nach dem HTTP-Headerparser geprüft. Eine deklarierte zu große
Content-Length wird vor dem Bodylesen abgewiesen; Chunking benötigt dagegen
die tatsächliche Streamingprüfung. Reqwest kann intern bereits Bytes gepuffert
haben. Das Bodylimit ist kein Limit aller empfangenen TLS-/HTTP-Bytes oder
des Prozessspeichers. RestReader prüft erst bereits gelieferte Vektoren;
OfflineHttpSource prüft schon im Speicher befindliche Antworten und vom
Aufrufer eingetragene Laufzeiten. Beides beweist keine Empfangsdeadline.

**Zeitbehandlung und offene Anforderungen:** Konfiguriert sind zehn Sekunden
Connecttimeout, 30 Sekunden Anfragebudget und 120 Sekunden Operationsbudget
ab erstem Versuch. Pro Request wird das kleinere verbleibende Zeitbudget an
reqwest übergeben; monotone Prüfungen erfolgen vor und nach Reads. Sendefehler
werden HTTPS_REQUEST_FAILED, Readfehler HTTPS_BODY_READ_FAILED; nur eigene
Zeitprüfungen liefern HTTP_TIMEOUT. Eine präzise Timeoutursache wird dadurch
nicht zugesichert. Harte, unabhängig abbrechbare Header-/Gesamtdeadlines über
DNS, TLS und blockierende Reads sowie eine eigene 32-KiB-Headerbytegrenze vor
Parserallokation sind weiterhin nicht implementiert beziehungsweise nicht
qualifiziert. Ein Headeranzahllimit ersetzt diese Anforderungen nicht.

### Neue Rust-Integrationsfälle

Ausschließlich src/https_local_tls.rs wurde funktional erweitert. Die
bestehende Rust-Hülle um OpenSSL nimmt nun auch eigene Antwortvektoren an.
Optional schließt sie nach Übergabe der Antwort die Servereingabe; mit dem
vorhandenen -no_ign_eof entsteht ein echter TLS-Verbindungsabschluss. Die
Serverausgabe wird dabei bis EOF verworfen, damit eine geschlossene stdout-Pipe
den Abschluss nicht künstlich stört. Drop beendet und sammelt den Prozess
ein und wartet anschließend auf den Leserthread, auch bei normalen Panics.
Bestehende Startbereitschaftsgrenze und Portübergabelücke bleiben erhalten.
Es gibt keine neuen Hintergrundthreads, Sleeps oder eigenen Listenerloops.

Zehn neue local_tls_-Testfunktionen durchlaufen den echten TLS-Handshake,
die vorhandene synthetische Test-CA und den unveränderten fetch-Pfad:

- 16 Bodybytes bei Content-Length 16 werden akzeptiert.
- Zwei Chunks mit zusammen genau 16 Bodybytes werden akzeptiert.
- Content-Length 17 bei Limit 16 wird abgelehnt.
- Ein tatsächlich übertragener 17-Byte-Chunk wird beim Streaming abgelehnt.
- Content-Length 16 mit nur 14 Bodybytes vor TLS-Abschluss ergibt einen Readfehler.
- Chunkübertragung ohne abschließenden Nullchunk vor TLS-Abschluss ergibt einen Readfehler.
- Content-Encoding gzip wird ohne automatische Dekompression abgewiesen.
- HTTP 503 wird abgewiesen.
- Ein zusätzlicher Headerwert mit gut 8 KiB bei kleinem Body wird akzeptiert;
  dies qualifiziert keine harte obere Headerbytegrenze.
- 101 zusätzliche Headerfelder überschreiten die konfigurierte Anzahl und
  sollen bereits beim echten Parser als Requestfehler scheitern.

Alle Fälle verwenden ein Antwortlimit von 16 Byte und vier erlaubte Versuche,
keine 8-MiB-Antwort. Alle Negativfälle prüfen statische Fehlercodes, fehlende
synthetische Geheimnismarker in Display/Debug, HTTPS_SOURCE_CLOSED beim
Folgeaufruf und unverändert genau einen verbrauchten Versuch. Der Server
muss genau einen entschlüsselten GET beobachten. Das bestehende endliche
250-ms-Fenster ist kein universeller Nachweis gegen beliebig späte oder
reine TCP-/TLS-Verbindungsversuche. Bei den beiden EOF-Fällen endet der
Server selbst; Versuchszähler und Quellensperre ergänzen dort die Beobachtung.

Die Tests bleiben im Unit-Testmodul, weil nur dort der cfg(test)-Konstruktor
zugänglich ist; fachlich integrieren sie echten Client, TLS und HTTP-Empfang.
Nur 127.0.0.1 und die feste lokale Hostauflösung werden verwendet, kein
öffentlicher DNS-Zugriff. Zertifikate und Schlüssel entstehen erst beim
Runnerlauf mit der bestehenden lokalen OpenSSL-Infrastruktur. Keine echten
Credentials. Die produktive Rust-Implementierung und ihre Sicherheitsoptionen
wurden nicht verändert; keine nachgewiesene neue Sicherheitslücke behoben.

### Aussagegrenzen und nächste Schritte

Die EOF-Fälle sind kontrollierte TLS-Abschlüsse während einer unvollständigen
HTTP-Antwort, kein Nachweis eines abrupten TCP-Resets oder einer unvollständigen
TLS-Recordübertragung. Der erfolgreiche Chunkgrenzfall ist das Gegenstück zu
den Chunkfehlern; die genaue OpenSSL-EOF- und Parserwirkung muss der Runner
bestätigen. Slow-Drip und verzögerter Body wurden nicht als neue TLS-Tests
implementiert: Die vorhandene API bietet keine kleinen konfigurierbaren
Zeitbudgets, und die interaktive Serverhülle garantiert keine zeitgenaue
TLS-Auslieferung. Daraus wird kein PASS und keine harte Deadline abgeleitet.
Auch eine neue TLS-Qualifikation des 32-MiB-Gesamtbudgets ist nicht behauptet;
die vorhandenen injizierten Read-Budgettests bleiben davon getrennt.

Keine Builds, Tests, Formatierer oder Clippy ausgeführt. Die zehn Fälle sind
implementiert, nicht ausgeführt und nicht als PASS qualifiziert. Als Nächstes
soll der äußere Runner check, Formatprüfung, Clippy und alle Tests samt
vollständigem Rohprotokoll durchführen. Erwarteter Umfang nach erfolgreicher
Erkennung: 15 local_tls_-Funktionen; keine erfundene neue Gesamterfolgszahl.
Danach EOF-/Parserergebnisse bewerten und eine gesonderte, zuverlässig
steuerbare Zeitqualifikation sowie harte Headerbyte-/Deadlinegrenzen klären.

Originale repository-/OID-gebundene Rohcommitbytes samt Git-OID, SHA256,
Quelle und Erhebungsbedingungen fehlen weiterhin als authentifizierter
Beschaffungspfad. RawCommitSource ist nicht an diesen HTTPS-Adapter angebunden;
REST-JSON und rekonstruierte Commitbytes ersetzen keine Originale. Kein
zusammengeführtes Budget mit künftigem Rohcommittransport. Privater Live-
Readback, Remote-Frische, Credentialzuführung und CA-Provenienz bleiben offen.

Keine Manifest-/Lockfileänderung, Downloads, produktiven GitHub-Aufrufe,
Git-Mutationen, Veröffentlichung, Hardwarezugriffe oder Image-Builds.
Alle bisherigen Checkpoints bleiben vollständig erhalten.
last_verified bleibt unverändert. Die Publikationssperre bleibt aktiv.

Checkpoint: NOVENA-BRIDGE-RUST-284-CODEX.
Neue TLS-Testfunktionen: 10, implementiert, Runnerqualifikation ausstehend.
EOF-Tests: kontrollierter TLS-Abschluss, kein TCP-Reset-Nachweis.
Harte Headerbytegrenze und harte Gesamt-/Headerdeadlines: weiterhin offen.

---

## NOVENA-BRIDGE-RUST-284 – Äußere Testqualifikation

Datum: 2026-10-26

Ausgangsstand: Block 283 mit 81 erfolgreichen
Rust-Tests einschließlich fünf lokaler TLS-Tests.

Codex hat die vorhandene Rust-HTTPS-Implementierung
und ihre Fehlergrenzen weiterentwickelt.

Äußere Prüfung durch den Runner:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Gesamtstatus der Offline-Qualifikation: PASS

Die genauen Testnamen, Erfolge und Fehler sind
im archivierten Originalprotokoll zu prüfen.

Die Testqualifikation erlaubt keine Aussage
über vollständigen privaten GitHub-Readback.

Offen bleiben:
- nicht nachgewiesene harte Empfangsgrenzen;
- authentifizierte Git-Commit-Rohbytes;
- unabhängige private Readbackprüfung.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_ACCESS=NOT_EXECUTED.

## NOVENA-BRIDGE-RUST-285-CODEX

Datum: 2026-10-26. Sicherheitsanalyse und Implementierung; keine eigene
Build-/Testausführung. Ausgangsqualifikation laut Auftrag: Block 284 mit
91 Rust-Tests, darunter 15 echte lokale TLS-Tests, check/fmt/Clippy PASS.
Die nachstehenden neuen Fälle sind noch nicht qualifiziert.

### Lokale Primärquellen und Befund

Analysiert wurde der lokal vorhandene Registryquellcode von reqwest 0.13.5
unter `/home/loomit/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.13.5/`.
Maßgebliche Stellen: `src/blocking/client.rs` (`ClientHandle::execute_request`,
`forward`, `InnerClientHandle::drop`), `src/blocking/wait.rs` (`timeout`),
`src/blocking/response.rs` (`Read::read`, `body_mut`),
`src/blocking/request.rs` (`RequestBuilder::timeout`), `src/connect.rs`
(`ConnectorService::call`, `with_timeout`, `connect_with_maybe_proxy`),
`src/async_impl/client.rs` (`execute_request`, `PendingRequest::poll`) und
`src/async_impl/body.rs` (`response`, `TotalTimeoutBody::poll_frame`).
Keine Quellenbeschaffung und kein Netzwerkzugriff.

Wesentliche Korrektur der bisherigen Bewertung: Der explizite Timeout des
**Requests** bleibt im inneren asynchronen Request erhalten. Dessen Timer
wird beim Empfang der Header unverändert in die Response und schließlich
in TotalTimeoutBody verschoben. Er wird vor dem inneren Body-Poll geprüft
und weckt auch einen wartenden Read. Er startet nicht pro Bodybyte neu.
Der Timeout des **blockierenden Clientbuilders** allein wird dagegen als
Wartezeit an send und jeden Read übergeben; er allein beweist keine
gemeinsame Requestdeadline. Der vorhandene fetch-Pfad verwendete bereits
den entscheidenden expliziten Request-Override. Es wurde daher keine
belegte bisher unbegrenzte Bodywartezeit durch eine neue Architektur behoben.

| Anforderung | Tatsächliche Durchsetzung / Aussagegrenze |
|---|---|
| Connect | Bibliothek: 10 s um den Connector-Future, zusätzlich Requesttimer. Kein eigener Socketcode. |
| TLS-Handshake | Bibliothek: derselbe Connect-Timeout umfasst den rustls-HTTPS-Connector einschließlich Handshake; kein separater Handshaketimer. Kein neuer Laufzeitnachweis eines hängenden Handshakes. |
| Vollständige HTTP-Header | Bibliothek: Requesttimer und begrenztes blocking-send-Warten bis zur Response. Keine separate Headerdeadline. Neuer echter TLS-Test mit unvollständigen Headern. |
| Blockierender Body-Read | Bibliothek: blocking wait::timeout begrenzt jeden Read; zusätzlich gemeinsamer TotalTimeoutBody-Timer. Future wird bei Warte-Timeout verworfen, Response beim Fehler verworfen. Kein eigener zurückgelassener Clientthread. |
| Slow-Drip | Bibliothek: TotalTimeoutBody läuft trotz Fortschritt weiter. Eigener Code prüft außerdem vor/nach jedem Read; diese Prüfungen allein wären nur kooperativ. Neuer TLS-Test mit laufenden kleinen Sendungen. |
| Gesamtdauer eines Requests | Bibliothek: expliziter Request-Override bis Body-Ende, normalerweise höchstens 30 s ab bibliotheksinternem Timerstart. Abbruch während des Wartens, keine bloße elapsed-Auswertung. |
| Gesamtdauer eines Lesevorgangs | Eigener Code berechnet ab erstem get ein 120-s-Budget und beschränkt jeden Request auf das verbleibende Budget. Bibliothek erzwingt dessen relative Dauer auch im wartenden Body-Read. Neuer TLS-Test mit bereits fast verbrauchtem Operationsbudget. |
| Headerbytes vor Allokation | Weiterhin BLOCKED: 100 Headerfelder sind keine 32-KiB-Bytegrenze. reqwest 0.13.5 exponiert für diesen HTTP/1-Pfad keine passende Headerbyte-/Puffergrenze. Prüfung nach HeaderMap-Allokation würde die Anforderung nicht erfüllen. |

Die Bibliothek erzwingt endliche Wartefristen, keine Echtzeitgarantie unter
beliebiger Schedulerblockade. Das eigene absolute Instant wird als relative
Dauer weitergegeben; Zeit zwischen Restbudgetberechnung und internem
Timerstart kann hinzukommen. Vor-/Nachprüfungen verhindern Annahme verspäteter
Bodies, können aber keine exakt an diesem Instant stattfindende Präemption
beweisen. Eine streng absolute, ohne diese Übergabelücke durchgesetzte
120-s-Deadline bleibt als Sicherheitsblocker offen. Ebenso offen bleiben
separate frei konfigurierbare Header-/Handshakephasenfristen und deren
gesonderte Laufzeitqualifikation. Kein unbelegtes PASS für diese Anforderungen.
Clientkonstruktion gehört nicht zum ab erstem get gestarteten Budget.

Der lokale Hyper-HTTP/1-Unterbau besitzt zwar einen endlichen Standard-
Lesepuffer (`DEFAULT_MAX_BUFFER_SIZE = 8192 + 4096 * 100` in
`src/proto/h1/io.rs`), dieser ist keine projektspezifische 32-KiB-Grenze
vor Allokation und keine Obergrenze des gesamten Header-Speicherverbrauchs.
Parserpuffer, HeaderMap, Kopien und TLS-Puffer sind von den Bodybudgets
getrennt. Die Bodybudgets bleiben 8 MiB pro Antwort und 32 MiB kumuliert;
sie begrenzen weder alle Netzwerkbytes noch sämtliche Bibliotheksallokationen.

### Änderung und echte TLS-Testfälle

Geänderte Rust-Dateien: `src/https.rs` und `src/https_local_tls.rs`.
fetch weist ein schon vor send aufgebrauchtes Restbudget explizit ab und
dokumentiert den sicherheitsrelevanten Request-Override. ReceiveBudget hält
die weiterhin produktiv festen 30 s als internes Feld; nur das private
Testmodul verkürzt es. Keine neue öffentliche Timeout-/URL-/Client-Injektion.
Feste GitHub-Endpunkte, Zertifikats-/Hostnameprüfung, CA-Zufuhr,
Redirect-/Proxy-/Retry-Sperren und Größenbudgets bleiben erhalten.

Fünf neue `local_tls_`-Funktionen verwenden echten reqwest/rustls-Empfang
mit dem vorhandenen lokalen OpenSSL-Server und synthetischer CA:

- `local_tls_headers_then_delayed_body_aborts_while_waiting`: vollständige
  Header, danach Body erst nach 3 s.
- `local_tls_slow_drip_does_not_restart_request_deadline`: erster Bodybyte
  sofort, danach 50 einzelne Sendungen im Abstand von jeweils 50 ms;
  deklarierter Body bleibt unvollständig, tatsächlich gelesene Bytes werden geprüft.
- `local_tls_unterminated_chunked_response_aborts_while_waiting`: ein echter
  Chunk, anschließend kein Nullchunk und zunächst kein Verbindungsabschluss.
- `local_tls_operation_remainder_limits_blocking_body_read`: Requestbudget
  2 s, aber Operationsrest nur 600 ms; erster Bodybyte, danach Stillstand.
- `local_tls_missing_complete_headers_aborts_while_waiting`: unvollständiger
  Headerwert ohne Headerabschluss.

Vorzeitige Verbindungsenden bleiben durch die beiden vorhandenen echten
TLS-EOF-Fälle aus Block 284 abgedeckt (unvollständige Content-Length und
fehlender Chunkterminator). Das sind kontrollierte TLS-Abschlüsse, kein
behaupteter TCP-Reset-Test.

Neue Zeitfälle: Budget 600 ms, Rückkehr zwischen 300 und 1800 ms; damit
1,2 s obere Jittertoleranz. Der endliche Server-Fallback schließt frühestens
nach 3 s, beim verzögerten Body nach dessen Verzögerung. Er liegt außerhalb
der akzeptierten Clientlaufzeit. Ein allein durch Server-EOF gelöster Read
kann daher keinen Erfolg vortäuschen. Die Tests führen den Client direkt
aus, ohne detached Timeoutthreads und ohne äußeren Runner als Abbruchhilfe.
Alle Negativfälle prüfen Quellensperre und einen verbrauchten Versuch;
die Fortschrittsfälle prüfen zusätzlich verbrauchtes Bodybudget.

Der Server unterstützt endliche Antwortskripte; recv_timeout statt langer
ununterbrechbarer Sleeps erlaubt Drop, die Verzögerung abzubrechen. Drop
signalisiert Abbruch, beendet und reapet OpenSSL und joint den Leserthread.
Nur kleine Antwortstücke gelangen in die bestehende Pipe. Die vorhandene
5-s-Bereitschaftsgrenze und Portübergabelücke bleiben. Zertifikaterzeugung
und OpenSSL-Verhalten bleiben Voraussetzungen des äußeren Testlaufs;
die tatsächliche zeitliche TLS-Auslieferung ist noch vom Runner zu bestätigen.
Keine öffentlichen DNS-Anfragen, keine echten Credentials, nur 127.0.0.1
mit festem Test-Hostname/SNI. Keine neuen synthetischen Envelope-Tests.

### Offene Qualifikation und nächste Schritte

1. Äußerer Runner: Formatierung, check, Clippy und vollständige Tests;
   erwartete Erkennung 20 local_tls_-Funktionen, keine behauptete Erfolgszahl.
2. Originalausgaben und Laufzeiten prüfen; insbesondere sicherstellen, dass
   die Bodyfälle den Bodypfad erreichen und vor dem Server-Fallback abbrechen.
   Bis dahin Status der neuen Fälle: IMPLEMENTIERT, NICHT AUSGEFÜHRT.
3. Harte Headerbytegrenze vor Allokation und exakt absolute Operationsdeadline
   als offene Anforderungen gesondert lösen; keine Produktivfreigabe ableiten.
4. Authentifizierte repository-/OID-gebundene Rohcommitbeschaffung samt
   Originalbytes, Git-OID, SHA256 und Provenienz bleibt offen. REST-Metadaten
   ersetzen diese Bytes nicht; RawCommitSource bleibt getrennt. Privater
   Live-Readback und Remote-Frische sind weiterhin nicht qualifiziert.

Keine Builds, Tests, Formatierer oder Clippy durch Codex ausgeführt.
Keine Manifest-/Lockfileänderung, Downloads, produktiven GitHub-Zugriffe,
Git-Mutationen, Veröffentlichungen, Image-Builds oder Hardwaretests.
Frühere Checkpoints bleiben erhalten. last_verified bleibt unverändert.
BRIDGE_PUBLICATION=BLOCKED; LIVE_GITHUB_ACCESS=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-285 – Äußere Rust-Qualifikation

Datum: 2026-10-26

Ausgangsstand: 91 erfolgreiche Rust-Tests
einschließlich 15 lokaler TLS-Tests aus Block 284.

Codex erhielt den Auftrag, harte Netzwerk-
Zeitgrenzen und Slow-Drip-Abbruchverhalten
zu untersuchen und soweit möglich zu verbessern.

Prüfungen durch den Runner:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Rust-Qualitätsstatus: PASS

Dieser Status ist keine automatische Bestätigung
aller harten Zeitgrenzen. Deren tatsächlicher
Schutzumfang ist anhand von Implementierung,
Tests und Originalprotokoll gesondert zu bewerten.

Echter GitHub-Readback wurde nicht ausgeführt.

Rohcommitbeschaffung bleibt unqualifiziert.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_ACCESS=NOT_EXECUTED.

## NOVENA-BRIDGE-RUST-286-CODEX

Datum: 2026-10-26. Ergebnis: Sicherheitsanalyse mit konkretem Blocker;
**keine lokale Git-Produktivquelle implementiert**. Die ausdrücklich
vorgesehene Abbruchbedingung für nicht abgesicherte harte Grenzen greift.
Ausgangsqualifikation laut Auftrag: 96 Rust-Tests, davon 20 lokale TLS-Tests,
Rustfmt und Clippy bestanden. Diese Zahlen wurden hier nicht erneut geprüft.

### Quellenprüfung und Entscheidung

Gelesen wurden AGENTS.md, NOVENA-ARBEITSREGELN.txt, Repository-README,
Bridge-README und bisherige Checkpoints, NOVENA-BRIDGE-LITE-PLAN sowie die
relevanten Rust-Schnittstellen und Objekt-/Protokolltests. Der bestehende
Arbeitsbaum enthält Nutzeränderungen und die unversionierte Bridge; diese
bleiben erhalten. Keine neue globale INDEX-Kennung beansprucht.

Lokal verfügbar: `/usr/bin/git`, Ausgabe `git version 2.56.0`.
Primärbelege: installierte Handbücher unter `/usr/share/man/`, insbesondere
`man1/git-cat-file.1.gz` (OPTIONS, OUTPUT, BATCH OUTPUT, CAVEATS),
`man1/git.1.gz` (globale Optionen, Umgebungsvariablen),
`man1/git-config.1.gz` (ENVIRONMENT, core.packedGitLimit,
core.deltaBaseCacheLimit) und `man5/gitrepository-layout.5.gz`
(objects/info/alternates, config, commondir). Keine Netzwerkbeschaffung.
Implementierungsquellen von Git wurden bei der begrenzten lokalen Suche
unter /usr/src und /usr/share nicht gefunden; keine Quellcodequalifikation
des Git-Binaries behauptet. Es wurde kein cat-file-Versuch gestartet.

**Belegte Schnittstellen:** cat-file kann unkomprimierte Rohobjektbytes
ausgeben. Batch liefert OID, Typ und Größe vor dem Body. Der einfache
Aufruf mit vorgegebenem Typ erlaubt jedoch auch Typableitungen, etwa Tree
aus Commit; eine Typangabe allein ersetzt keine exakte Typprüfung.
`--no-replace-objects` und `--no-lazy-fetch` sind lokal dokumentiert.
Letzteres ist notwendig, weil selbst Objektlesezugriffe bei Promisor-
Repositories sonst fehlende Objekte beschaffen können. Keine textconv-,
filters-, mailmap-, Revisions- oder remote-object-info-Verwendung zulässig.

**Verbleibender Ressourcenblocker:** Ein auf 1 MiB begrenzter Rust-Reader
begrenzt nur angenommene Ausgabe, nicht Git-interne Dekompression,
Deltaauflösung, Pack-Mappings oder CPU-Verbrauch. Die dokumentierten
packedGitLimit-/deltaBaseCacheLimit-Werte begrenzen Mappings beziehungsweise
Cacheteilbereiche, nicht den gesamten Prozessspeicher. Eine vorgeschaltete
Größenabfrage bietet keine solche Prozessgrenze. Ein einzelner kleiner
Zielcommit begrenzt nicht automatisch seine Delta-Basisressourcen.
Das ist eine Sicherheitsanforderung an einen künftigen Adapter, kein hier
reproduzierter Ressourcenangriff. std::process::Command allein bietet keine
Speicher-/CPU-Isolation und keine komplette Deadline-/Pipe-/Reaping-Lösung.
Ein blockierendes read oder wait_with_output mit nachträglicher Zeitprüfung
wäre unzureichend. Unbegrenzte Readerthreads wären ebenfalls kein Ersatz.

**Verbleibende Vertrauensgrenze:** Bereinigte Umgebung und explizites
--git-dir verhindern noch nicht das Lesen repositoryeigener Konfiguration,
Includes, commondir und dateibasierter Alternates. Insbesondere ist ein
leeres GIT_ALTERNATE_OBJECT_DIRECTORIES keine dokumentierte Abschaltung von
objects/info/alternates. Nur vorherige Pfadprüfungen bieten bei gleichzeitig
veränderbarem Speicher keinen atomaren Schutz gegen Austausch/Umleitung.
Es wurde kein stabiler, ausschließlich lesbarer Snapshot oder isolierter
Dateisystemraum als Voraussetzung dieses Adapters eingerichtet oder belegt.

Für einen späteren Git-Adapter sind vor Implementierungsfreigabe konkret
abzusichern: expliziter absoluter Speicher- und vertrauenswürdiger Git-Pfad,
unterstütztes SHA1-Repositoryformat, kontrollierte Konfiguration ohne
Umleitungen, unveränderliche lokale Objektprovenienz, Ausschluss von
Alternates/Replacement/Promisor-Beschaffung, Betriebssystemgrenzen für
Speicher und CPU sowie eine endliche Prozessdauer einschließlich aller
Pipefehler und kill/wait-Aufräumwege. Netz und Schreibzugriffe müssen auch
im Child ausgeschlossen sein. Ein solcher Rust-/OS-Isolationspfad ist mit
der bestehenden Bridge noch nicht implementiert oder qualifiziert.
Dies behauptet keine grundsätzliche Unmöglichkeit einer sicheren Lösung.
Ein eigener Loose-/Pack-Reader mit begrenzter Dekompression wäre eine
separate Alternative; die direkten bestehenden Abhängigkeiten stellen
keinen solchen Git-Reader bereit. Keine neue Dependency aufgenommen.

### Tatsächlicher Implementierungs- und Testumfang

Produktive Quellen bleiben unverändert: explizit übergebene Rohcommitbytes
über den bestehenden RawCommitSource-Closureadapter und Offlinebündel.
HttpsGetSource bleibt ausschließlich ResponseSource für HTTPS-GET und ist
keine Rohcommitquelle. Keine automatische Auswahl des Projekt-Repositories,
kein lokaler Git-Speicherzugriff, Clone, Fetch oder neuer CLI-Pfad.
RawCommitSource und check_commit bleiben in ihrer Implementierung unverändert;
protocol.rs bindet lediglich ein privates cfg(test)-Modul ein.

Neu: acht Rust-Vertragstests in `src/raw_commit_tests.rs` mit ausschließlich
synthetischen Speicherfixtures, ohne Dateischreibzugriff oder Child-Prozess:

- Originalbytes mit Nicht-UTF8-Nachricht, mehrfachen finalen Newlines und
  ohne finale Newline; bytegleiche Rückgabe und bestehender check_commit.
- Passende/falsche OID und nachträglich entfernte finale Newline.
- Blob-/Tree-Hashdomänen gegen denselben gültigen Commitinhalt.
- Ungültige OIDs einschließlich Ref, Revisionsausdruck, Kurz-/Überlänge,
  Großbuchstaben, Unicode, LF/NUL und Optionsargument; kein Quellenaufruf.
- Beschädigte beziehungsweise nicht unterstützte Commitbytes trotz passender
  Inhalts-OID: CR, NUL, leere Nachricht und fehlende Commitstruktur.
- Exakt 1 MiB akzeptiert, ein Byte darüber abgewiesen.
- Fehlendes Objekt als injizierter Quellfehler ohne erfolgreiches Teilergebnis.
- Produktive Publikationssperre auch nach erfolgreicher Commitprüfung.

Diese Tests qualifizieren **keinen Git-Objektspeicheradapter**. Insbesondere
prüfen Hashdomänen keine reale cat-file-Typantwort, injizierte Fehlobjekte
keine lokale Git-Suche und die Parsergrößengrenze keine Allokationsgrenze
vor Beschaffung. Keine Prozess-/Alternates-/Konfigurations-/Netzwerksperre
als getestet ausgeben. Bestehende Offline- und CLI-Negativtests bleiben
ergänzende Regressionen; keine Teilstdout-Garantie aus diesen Unit-Tests
ableiten. Alle acht neuen Fälle: IMPLEMENTIERT, NICHT AUSGEFÜHRT.
Runner muss check, Rustfmt, Clippy und Tests gesondert ausführen.

### Provenienz, TLS und spätere GitHub-Rücklesung

Kein lokaler Objektspeicher wurde für diesen Auftrag als Quelle ausgewählt
oder gelesen; daher keine neue reale Objektprovenienz erhoben. Ein späterer
Adapter muss ausgewählten Speicher/Snapshot, dessen Herkunft, Originalbytes,
OID, ergänzenden SHA256 und Prüfbedingungen nachvollziehbar zuordnen.
Selbst korrekte Git-Hashidentität beweist nur Inhaltsintegrität relativ zur
angeforderten OID. Lokal verfügbare Bytes beweisen keine authentifizierte
GitHub-Herkunft, Repositoryzugehörigkeit, Ref-Erreichbarkeit oder Aktualität.
SHA1 ist die bestehende Objektidentität, keine neue Authentizitätsgarantie.
REST-JSON darf niemals zur Rekonstruktion vermeintlicher Rohbytes dienen.
Das bisherige schmale Commitprofil mit CR-/NUL-/Zusatzheaderablehnung bleibt.

TLS-Blocker aus 285 bleiben offen: keine harte 32-KiB-Headerbytegrenze vor
Allokation; 100 Headerfelder und der Bibliothekspuffer ersetzen sie nicht.
Die exakt absolute Operationsdeadline ohne Übergabelücke sowie separate
Phasengrenzen und deren Qualifikation bleiben ebenfalls offen.

Später, nur mit gesonderter Netzwerkfreigabe: authentifizierte private
GitHub-Repository-ID und Ref vor/nach der Prüfung zurücklesen; vollständige
Commitmetadaten, Trees und Originalblobs OID-gebunden prüfen; separat
qualifizierte Originalcommitbytes gegen OID und REST-Links prüfen. Beschaffung
und Provenienz dieser Bytes müssen ausdrücklich belegt sein. Zielpayload und
Baseline bytegenau vergleichen, vollständige erlaubte Transition und
unveränderten last_verified nachweisen; Konflikte oder fehlende Belege
geschlossen ablehnen. Lokale Integrität ersetzt diesen Readback nicht.
Kein bestehender Offlinebericht wird zu Live- oder Herkunftsevidenz umgedeutet.

Keine Builds, Tests, Formatierer oder Clippy ausgeführt. Keine Manifest-/
Lockfileänderung, keine Netzwerkverbindung, produktive Git-Mutation,
Veröffentlichung, last_verified-Änderung, Image-Build oder Hardwareaktion.
Kein äußerer Runner gestartet; Werkzeugausgaben sind kein vollständiges
archiviertes Runner-Rohprotokoll. Bisherige Checkpoints bleiben erhalten.

LOCAL_GIT_RAW_SOURCE=BLOCKED_NOT_IMPLEMENTED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_ACCESS=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-286 – Äußere Rust-Qualifikation

Datum: 2026-10-26

Ausgangsstand: 96 bestandene Rust-Tests aus Block 285.

Codex wurde mit einer ausschließlich lokalen,
lesenden Original-Git-Commitquelle beauftragt.

Äußere Prüfung:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Rust-Qualitätsstatus: PASS

Eine eventuell implementierte lokale Commitquelle
beweist nur die Integrität gegenüber der geprüften
OID, nicht die authentifizierte Herkunft aus dem
privaten GitHub-Repository.

GitHub-Readback und produktive Veröffentlichung
bleiben ausdrücklich gesperrt.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-287-CODEX

Datum: 2026-10-26. Ergebnis: Sicherheitsanalyse abgeschlossen;
konkrete lokale Git-RawCommitSource **BLOCKED, nicht implementiert**.
Keine globale INDEX-Kennung beansprucht. Alle bisherigen Checkpoints bleiben
erhalten. Ausschließlich dieser Dokumentationsnachtrag wurde ergänzt.

### Evidenz und tatsächlich vorhandene Quelle

Lesend geprüft: Arbeitsregeln, README und aktuelle Projekt-/Test-/
Entscheidungsnachträge, Bridge-README, Checkpoints, Lite-Plan, Rust-Verträge
in protocol.rs, objects.rs, lib.rs, raw_commit_tests.rs und tests/objects.rs.
Die Eingangslage enthält bereits Nutzeränderungen und die unversionierte
Crate; diese wurden erhalten. Branch main, HEAD
df28dc50be1eea15339a8510a4aa450eb6b15b99; Index-SHA256
3ddd11b86530f7ad82fa5568d5c603b1624a4ec2d0f0b2748f5ac21be3ea67e9.
Keine zusätzliche Sicherung angelegt; kein äußerer Runner gestartet.
Die Werkzeugausgaben sind kein archiviertes vollständiges Runner-Rohprotokoll.

Tatsächlich ausgeführt: lokale Versions-/Pfadinventur und lesende Auswertung
der installierten Git-Manpages. `/usr/bin/git --version` meldet Git 2.56.0
(Versionsabfrage erfolgte als `git --version`, aufgelöst über command -v).
`git --exec-path`: /usr/lib/git-core. /usr/bin/git ist laut ls eine reguläre
ausführbare Datei. Dies ist keine vollständige Prüfung seiner Vertrauenskette.
Primärquellen: /usr/share/man/man1/git.1.gz, git-cat-file.1.gz,
git-config.1.gz sowie /usr/share/man/man5/gitrepository-layout.5.gz.
Keine Netzrecherche, kein cat-file-Zugriff auf Projektobjekte, keine
Git-Fixture und kein Verhaltensexperiment ausgeführt. Manpageverträge sind
keine experimentelle Qualifikation der ausführbaren Instanz.

RawCommitSource bleibt ein injizierter Vertrag mit Closure-Implementierung;
es existiert weiterhin kein konkreter lokaler Git-Objektspeicheradapter und
keine automatische Arbeitsrepositoryauswahl. Oid besitzt ein privates
20-Byte-Feld; Oid::parse akzeptiert genau 40 ASCII-Zeichen aus 0..9/a..f.
HEAD, Branches, Refs, Abkürzungen, Optionen und Revisionsausdrücke scheitern
vor dem Quellaufruf. check_commit prüft originale Bytes unabhängig mit
SHA1 über `commit <Bytezahl>\0<Bytes>` und vergleicht Tree-/Parentlinks.
inspect_commit hat die nachgelagerte 1-MiB-Grenze und das bestehende schmale
Commitprofil; dies begrenzt keine Beschaffung.

### Git-Sicherheitsvertrag: Befunde und erforderliche Grenzen

Die folgenden Maßnahmen sind Anforderungen an einen künftigen Adapter,
keine bereits implementierten Schutzmaßnahmen:

| Gefahr | Befund / notwendige Behandlung |
|---|---|
| GIT_DIR, GIT_WORK_TREE | Verändern Repository-/Worktreeauswahl. Vollständig geleerte Child-Umgebung, explizites absolutes Git-Verzeichnis und kontrolliertes Arbeitsverzeichnis; keine Discovery. Auch GIT_COMMON_DIR, Namespace, Index- und sonstige Git-Variablen dürfen nicht geerbt werden. |
| GIT_CONFIG_* / globale Konfiguration | GIT_CONFIG_COUNT/KEY/VALUE injizieren Laufzeitkonfiguration. Keine geerbten Variablen einschließlich GIT_CONFIG_PARAMETERS; globale Konfiguration explizit auf /dev/null. HOME/XDG nicht übernehmen. |
| Systemkonfiguration | GIT_CONFIG_NOSYSTEM und GIT_CONFIG_SYSTEM=/dev/null sind zusätzliche kontrollierte Einstellungen. Sie deaktivieren keine Repositorykonfiguration oder deren include/includeIf. |
| Repositorykonfiguration | Keine beliebige config mit Remotes, Includes, Extensions oder worktree-Konfiguration akzeptieren. Ein enges Profil mit exakt begrenztem, geprüftem Inhalt oder kontrollierter isolierter Hülle wäre nötig; einzelne -c-Werte ersetzen keine Isolation. |
| GIT_OBJECT_DIRECTORY / GIT_ALTERNATE_OBJECT_DIRECTORIES | Nicht erben. Expliziter Objektspeicher allein verhindert dateibasierte Alternates nicht. |
| objects/info/alternates | Kann weitere Speicher außerhalb der Auswahl erschließen; auch http-alternates ausschließen. Keine rekursive Alternates-Auflösung im unterstützten Profil. |
| Replace refs | GIT_NO_REPLACE_OBJECTS explizit setzen und Replacementfreiheit des Profils prüfen, einschließlich gepackter Refs. Eine volle OID allein verhindert Replacements nicht. |
| Shallow / Promisor / Partial Clone | Shallow ist kein Nachweis fehlender Rawbytes, aber außerhalb eines zunächst engen Profils. shallow, Promisor-Konfiguration, partialClone-Extensions und .promisor-Packmarker ablehnen. Fehlende Objekte müssen Fehler bleiben. |
| Automatische Beschaffung | Git dokumentiert GIT_NO_LAZY_FETCH gegen Lazy-Fetch vom Promisor-Remote. Zusätzlich keine Remotes/Helper/Filter im Profil, keine geerbte Prozessumgebung und gegebenenfalls OS-Netzwerksperre. Die Variable allein ist keine Sandbox oder hinreichende Nur-lokal-Garantie. |
| Pfade / Symlinks | .git-Dateiumleitungen, commondir, Symlinks in allen Eltern-/Objektpfaden und spezielle Dateien ausschließen. canonicalize plus spätere Prüfung verhindert keinen konkurrierenden Austausch. Ein unveränderlicher, vertrauenswürdig bereitgestellter lokaler Snapshot oder atomare descriptorbasierte Isolation ist erforderlich. safe.directory=* wäre keine Lösung. |
| Objektformat / Speicher | Nur SHA1; SHA256-/Kompatibilitätsformate und unbekannte Extensions ablehnen. Als erster Kandidat ausschließlich eigenständiger Bare-Snapshot mit Loose Objects, ohne Packs/MIDX/Commitgraph/Alternates; auch Loose Objects benötigen sichere Dekompression und Größen-/Zeitgrenzen. Dieses Profil ist noch nicht implementiert oder qualifiziert. |

cat-file dokumentiert `<object>` als Revisionsausdruck. `-p` ist Pretty-Print,
kein allgemeiner Originalbytevertrag. `cat-file <type> <object>` liefert zwar
unkomprimierte Rawbytes, erlaubt jedoch auch Dereferenzierung: ein angeforderter
Typ allein beweist deshalb nicht den tatsächlichen Objekttyp. Nötig wären
eine exakte Typprüfung und unabhängige Commit-OID-Prüfung auf demselben
unveränderlichen Speicher. Keine Filter, Textconv oder Mailmap anwenden.
Die vorhandenen Originalbytes dürfen weder als Text umkodiert noch mit LF
ergänzt werden; auch ein fehlendes Schluss-LF ist Teil ihrer Identität.

Batch liest Objektausdrücke von stdin, liefert Typ/Größe und danach exakt
Größe Bytes plus einen zusätzlichen Ausgabe-LF. Dieser gehört nicht zum
Objekt. Fehler wie missing haben eigene Ausgaben. Batch-all-objects traversiert
auch Alternates; seine dokumentierte Replacementausnahme darf nicht auf
gewöhnliche Einzelabfragen übertragen werden. Batch passt außerdem nicht zum
geforderten geschlossenen stdin. Kein Batch- oder -p-Adapter implementiert.

### Prozessfrist: konkreter BLOCKED-Befund

Für die Git-Rawquelle gibt es derzeit **keine Prozessgrenzen**, weil sie
keinen Child startet. Die historischen hash-object-Oracletests verwenden
Command mit PATH-Auflösung, piped stdin/stdout und wait_with_output;
sie sind kein sicherer Prozessadapter und haben keine harte Frist.

Ein neuer Adapter müsste Command ohne Shell mit vertrauenswürdigem absolutem
Programmpfad, stdin=null, stderr=null, vollständiger Exitstatusprüfung und
einer während des Empfangs geltenden Grenze von 1 MiB plus höchstens einem
Überlauf-Erkennungsbyte verwenden. output()/wait_with_output() oder ein
unbegrenztes read_to_end sind hierfür ungeeignet. Eine vorherige Größenabfrage
ersetzt die Empfangsgrenze nicht.

Mit den hier vorhandenen synchronen std-Prozess-/Pipe-Primitiven ist kein
ausreichender harter Fristnachweis erbracht: ChildStdout::read kann blockieren,
auch bei take(1 MiB + 1); Instant-Prüfungen vor/nach read unterbrechen dies
nicht. try_wait-Polling allein entleert keine volle stdout-Pipe. Ein
Readerthread mit recv_timeout kann nach Rückgabe weiter blockieren;
Join kann seinerseits unbegrenzt warten. kill beendet nicht verlässlich
einen ganzen Nachkommenbaum; offene Pipehandles von Nachkommen verhindern
EOF. Child-Drop übernimmt weder Kill noch Reaping. Auch kill plus wait
belegt keine harte Rückkehrfrist bei blockierendem Kernel-I/O.

Dies ist kein allgemeiner Unmöglichkeitsbeweis für Rust ohne neue Crates:
plattformabhängige Nonblocking-/Poll-/Prozessgruppen-FFI oder ein gesondert
qualifizierter Supervisor wären denkbar. Eine solche Implementierung samt
Spawn-, Abbruch-, Reaping-, Nachkommen- und Dateisystemvertrag ist hier nicht
vorhanden; rohe FFI oder ein externes timeout werden nicht als ungeprüfter
Ersatz eingeführt. Daher gemäß Auftrag fail-closed keine konkrete Quelle,
keine detached Threads und kein scheinbar funktionierender Ersatzadapter.

### Tests, Grenzen und nächste Schritte

Neue echte Git-Integrationstestfälle: **keine**, weil keine echte Quelle
implementiert wurde. Die acht Vertragstests aus 286 bleiben unverändert:
Originalbytes/Nicht-UTF8/Schluss-LF, falsche OID/geänderte Bytes,
andere Hashdomänen, ungültige Revisionssyntax, fehlerhafte Commitbytes,
1-MiB-Parsergrenze, fehlendes Objekt und Publikationssperre. Sie belegen
weder Git-Typabfragen noch Repositoryisolation oder Child-Abbruch.
Die gemeldeten 104 Rust- und 20 lokalen TLS-Tests sind Eingangsevidenz,
kein in diesem Block erneut ermitteltes Ergebnis. Keine Builds, Tests,
Rustfmt oder Clippy ausgeführt.

Nach Qualifikation des Prozess-/Snapshotvertrags wären ausschließlich eigene
temporäre Git-Fixtures vorzusehen: echte Rawbytes samt unabhängiger Git-OID
und variierenden Schlussnewlines; Blob/Tree/Tag statt Commit; missing;
ungültige OIDs ohne Spawn; exakt 1 MiB und Überlauf während Empfang;
fehlende/manipulierte Struktur, Symlinks/FIFO und konkurrierender Austausch;
Replace refs/packed-refs, beide Alternatesarten, vergiftete Umgebung und
Config-Includes, Shallow/Promisor/Partial Clone; Child ohne Ausgabe, Ausgabe
über Limit, fehlerhafte/abgeschnittene Ausgabe, Nonzero-Exit nach gültigen
Bytes und Nachkommen mit offenem stdout; Abbruch und Reaping sowie weiterhin
gesperrte Publikation. Dies ist ein Testentwurf, keine vorhandene Qualifikation.

Lokale OID-Integrität beweist keine authentifizierte GitHub-Provenienz,
Repositoryzugehörigkeit, Ref-Erreichbarkeit oder Aktualität; SHA1 ist eine
Objektidentität, keine Herkunftsattestation. Kein lokaler Speicher wurde
als tatsächliche Rawquelle ausgewählt oder mit GitHub gleichgesetzt.
HTTPS bleibt separat begrenzt: keine harte 32-KiB-Headerbytegrenze vor
Allokation; 100 Headerfelder und Puffergrößen ersetzen sie nicht. Die exakt
absolute Operationsdeadline ohne Übergabelücke und separate Phasengrenzen
bleiben unqualifiziert. Dieser Block behebt keine dieser TLS-Grenzen.

Nächste Schritte: Prozess-/Dateisystemisolation ausdrücklich festlegen und
qualifizieren, dann schmale Quelle und temporäre Integrationstests umsetzen.
Für vollständigen Readback zusätzlich gesondert freigegebene authentifizierte
GitHub-Erhebung mit Repository-ID und Ref vor/nach der Prüfung, Originalcommit-
Beschaffungsprovenienz, vollständige Trees/Blobs und exakte Payload-/Baseline-
Transition nachweisen. last_verified bleibt unverändert; fehlende Evidenz
und Konflikte bleiben harte Fehler. Kein Offlinebericht wird zu Live-Evidenz.

LOCAL_GIT_RAW_SOURCE=BLOCKED_NOT_IMPLEMENTED;
PROCESS_DEADLINE=BLOCKED_NOT_QUALIFIED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-287 – Äußere Rust-Qualifikation

Datum: 2026-10-26

Ausgangsstand: 104 erfolgreiche Rust-Tests
einschließlich 20 lokaler TLS-Tests.

Auftrag: sichere lokale Git-Rohcommitquelle,
begrenzte Git-Prozesse und Schutz vor
Konfigurations-/Alternates-Umleitungen.

Äußere Prüfungen:

- cargo check: Exit 0
- cargo fmt: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test: Exit 0

Rust-Qualitätsstatus: PASS

Ein erfolgreicher Rust-Testlauf allein ist
kein Nachweis einer authentifizierten
GitHub-Herkunft.

Die konkrete Implementierung und etwaige
Sicherheitsblocker stehen im Codex-Checkpoint
NOVENA-BRIDGE-RUST-287-CODEX.

Originalprotokoll:
NOVENA-BRIDGE-RUST-287.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.


## NOVENA-BRIDGE-RUST-289-CODEX

Datum: 2026-10-26. Status: Offline-Integration implementiert, **noch nicht
ausgeführt oder qualifiziert**. Ausgangslage laut Auftrag: Block 288 mit
113 bestandenen Rust-Tests, darunter 20 echte lokale TLS-Tests, sowie
check/Rustfmt/Clippy PASS. Diese Vorqualifikation ist kein Ergebnis von 289.

### Quellvertrag und wiederverwendete Prüfer

`RawCommitFiles` implementiert bereits `RawCommitSource` und passt direkt
an die vorhandenen generischen Readback-Funktionen. `ResponseSource` liefert
REST-JSON-Bytes an `RestReader`; Rohcommits gehören ausschließlich in die
separate `RawCommitSource`. Keine Vermischung mit REST-Rekonstruktionen.
`check_commit` bleibt crate-intern und wird über die vollständigen öffentlichen
Prüfpfade erreicht; keine Sichtbarkeitserweiterung und kein zweiter Verifier.

Die Integrationstests übergeben `RawCommitFiles` ausdrücklich an
`verify_initial_index` beziehungsweise `verify_update_index`. Diese benutzen
unverändert `verify_transition_index`, `verify_transition_objects`,
`check_commit`, `inspect_commit`, `object_oid`, `flatten_verified`,
`verify_single_addition` beziehungsweise `verify_single_replacement` und
`RestReader::blob` mit dessen Blobhashprüfung. Repository-ID, Ref vor/nach
Readback, Rohcommit-OIDs, REST-Tree-/Parentlinks, vollständige Trees,
Änderungsumfang und exakte Zielbytes bleiben im bestehenden Prüfvertrag.
Der Updatepfad prüft zusätzlich die separat zurückgelesenen Baselinebytes.

### Neue Integrationstests und geprüfte Eigenschaften

Einzige geänderte Rust-Datei: `tests/protocol.rs`, mit dem Unix-Testmodul
`protocol_raw_files`. Keine Änderung an Produktionscode, CLI, Manifest,
Lockfile, Schema oder HTTP-Architektur. Die Unix-Beschränkung ermöglicht
explizite initiale Verzeichnisrechte 0700; auf anderen Plattformen werden
diese zwei Tests nicht kompiliert oder als Nachweis gezählt.

Wiederverwendet werden `Fixture`, dessen Git-Objektgenerator und
`update_fixture`. Jeder Fall erzeugt einen eigenen neuen privaten Baum unter
dem ausdrücklich gewählten `/tmp`; bestehende Pfade werden nicht übernommen.
REST-Antworten und originale synthetische Commitbytes liegen als Dateien
darin. Die REST-Testclosure liest ausschließlich ihre explizite Pfadzuordnung.
Die Commitdateien heißen exakt wie ihre OIDs. Die Rohbyte-Map wird vor dem
Verifieraufruf geleert: Originalbytes kommen ausschließlich aus
`RawCommitFiles`, niemals aus dem bisherigen Speicher-Rohcommitcallback.
Alle Einrichtung und Fehlerpräparation erfolgt vor Quellenkonstruktion;
während der Prüfung bleibt der Baum unverändert. Nachher räumt der Test
nur seinen eigenen Baum auf. Keine allgemeine Isolation damit behauptet.

Zwei neue Rust-Integrationstests mit insgesamt neun statischen Fällen:

- Vollständiger initialer Erfolg mit gültigen Dateibytes, passenden OIDs,
  REST-Metadaten, Trees und Blob. Assertions erfassen Repositoryabfrage,
  beide Commitabfragen, Zielblob und beide Ref-Abfragen bis zum Abschluss.
- Falsche Rohbytes unter unverändertem OID-Dateinamen:
  `COMMIT_HASH_MISMATCH`.
- Fehlende Baseline-Commitdatei: `RAW_COMMIT_PATH_MISSING`.
- Widersprechende REST-Treemetadaten: `COMMIT_METADATA_MISMATCH`.
- Widersprechende REST-Parentmetadaten: `COMMIT_METADATA_MISMATCH`.
- Gleich lange geänderte REST-Blobbytes: `BLOB_HASH_MISMATCH`.
- Geänderter REST-Treeeintrag: `TREE_HASH_MISMATCH`.
- Fehlende REST-Blobevidenz: `MISSING_RESPONSE`, kein Teil-PASS.
- Vollständiger Update-Erfolg über `verify_update_index`, einschließlich
  separatem Abruf des alten und neuen Blobs und abschließender Ref-Abfrage.

Diese Fälle prüfen die neue Kombination der Quellen mit den vorhandenen
Prüfpfaden. Bereits vorhandene Parser-, Größen-, Symlink-, CLI-, CAS- und
umfassende Update-Negativtests werden nicht erneut implementiert.
In Erfolgs- und Fehlerfällen prüfen Assertions unveränderte Payloadbytes,
`last_verified=null`, erhaltene UNKNOWN-Angaben und nichtleere Reviewgründe,
`NOT_PUBLISHED`/Commit-null und die Sperre `production_publish`.
Erfolgsberichte bleiben `OFFLINE_SUPPLIED_RESPONSES`,
`publication_performed=false` und `bridge_publication=BLOCKED`.
Fehlende Objekte liefern einen Fehler, keine erfundene UNKNOWN-Verifikation;
die fachlichen UNKNOWN-/Reviewangaben der Payload bleiben erhalten.

### Nachweisgrenzen und nächste Schritte

Alle genannten Assertions sind **implementierte Prüfziele**, noch keine
bestandenen Laufzeitnachweise. Gemäß Auftrag wurden weder Cargo-Builds noch
Tests, Rustfmt oder Clippy gestartet. Der äußere Runner muss beide neuen
Integrationstests tatsächlich ausführen und die Gesamtqualifikation liefern.
Die Zahl bestandener Tests wird hier nicht vorweggenommen.

Lokale Objektintegrität ist keine GitHub-Herkunftsattestation. Ein synthetischer
REST-Response ist keine authentifizierte HTTPS-Evidenz. Der Test beweist
keinen atomaren GitHub-Ref-Verlauf, keine Remote-Frische und keine fachliche
Abnahme. Die TOCTOU-Sicherheit von `RawCommitFiles` ist nicht allgemein
bewiesen; private, vertrauenswürdige, unveränderliche Dateien und Elternpfade
bleiben externe Voraussetzungen. Kein Race-Sicherheitsnachweis.
HTTPS-Headerbytegrenzen vor Parserallokation bleiben offen; Headeranzahl und
Bibliothekspuffer ersetzen diese Grenze nicht. Deadline-/Phasengrenzen und
Beschaffungsprovenienz werden durch diesen Offline-Test nicht qualifiziert.

Ein privater Live-Readback fehlt weiterhin und ist nicht freigegeben.
Keine Git-Prozesse, HTTP-/GitHub-Aufrufe, Credentials oder fremden
Rohcommitdateien in den neuen Tests. Keine Veröffentlichung, Promotion,
Git-Mutation oder Hardware-/Imagearbeit.

Nächster konkreter Integrationsschritt nach äußerer Qualifikation: einen
expliziten lokalen Readback-Aufrufer mit gepinnten Repository-/Commitwerten,
expliziter REST-Quelle und explizitem RawCommitFiles-Verzeichnis integrieren;
Bereitstellungsprovenienz und unveränderlichen Pfadvertrag separat belegen.
Ein authentifizierter privater GitHub-Readback benötigt einen eigenen Auftrag
und eine konkrete Netzwerkfreigabe einschließlich Originalcommitbeschaffung.

OFFLINE_RAW_FILES_READBACK=IMPLEMENTED_NOT_QUALIFIED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-288-CODEX

Datum: 2026-10-26. Implementiert, **nicht gebaut oder getestet**.
Die 104 erfolgreichen Tests einschließlich 20 lokaler TLS-Tests aus Block
287 sind Eingangsevidenz, kein erneut ermitteltes Ergebnis dieses Blocks.
Alle bisherigen Checkpoints bleiben erhalten. Bestehende Nutzeränderungen
einschließlich der unversionierten Bridge-Crate wurden erhalten.

### Vertrag und Implementierung

Neu: `raw_commit_files::RawCommitFiles`, öffentlich über `lib.rs`, als
kleiner konkreter Adapter des unveränderten `protocol::RawCommitSource`.
Der bisherige Closureadapter und `check_commit` bleiben erhalten.
Kein Git-Objektspeicher-, Loose-Object- oder Packfileparser.

- Auswahl ausschließlich durch
  `RawCommitFiles::from_trusted_immutable_directory(&Path)` mit ausdrücklich
  übergebenem absolutem Verzeichnis. Kein Default, keine `.git`-Auswahl,
  Discovery, Verzeichnisinventur, Umgebungs-/Tildeexpansion oder canonicalize.
  Elternkomponenten `..` werden abgewiesen. Die Rust-Path-Komponentenansicht
  kann redundante Separatoren und innere `.` bereits lexikalisch überspringen;
  dies ist keine zusätzliche Suche oder Pfaderweiterung durch den Adapter.
- Exakter Dateiname: die vollständige SHA1-OID als 40 ASCII-Zeichen
  `0..9/a..f`, ohne Suffix oder Unterverzeichnisse. Kein Fallback auf andere
  Namen, Refs, Abkürzungen oder Revisionen. Genau diese eine Datei wird pro
  erfolgreicher Anforderung einmal geöffnet; Metadatenabfragen lesen keinen
  zweiten Commitinhalt. `read_oid(&str)` validiert vor jedem Dateizugriff mit
  `Oid::parse`; der Trait nimmt bereits den privat konstruierten Typ `Oid` an.
- Dateiinhalt: originale unkomprimierte Git-Commitbytes ohne Objektheader
  und ohne REST-JSON. Keine Dekodierung, Normalisierung oder LF-Ergänzung.
  Nicht-UTF8-Nachricht und vorhandene/fehlende Schlussnewlines bleiben
  unverändert, soweit das bestehende schmale Commitprofil sie akzeptiert.
- Maximale Dateigröße: 1 MiB = 1.048.576 Byte. Vorprüfung per
  `symlink_metadata`, danach Typ-/Größenprüfung am geöffneten Handle.
  Einziger Inhaltsreader: `take(1.048.577).read_to_end`, somit höchstens
  1 MiB plus ein Überlauf-Erkennungsbyte pro Anforderung. Übergröße wird
  sowohl anhand der Metadaten als auch nach begrenztem Empfang abgewiesen.
  Größenabweichungen zwischen Vorprüfung, Handle und gelesenen Bytes sind
  Fehler. Keine unbeschränkte Inhaltsleseoperation im Adapter.
- Alle Verzeichniskomponenten einschließlich Eltern werden bei Konstruktion
  und vor jedem Lesen mit `symlink_metadata` auf echte Verzeichnisse geprüft.
  Der finale Pfad muss eine reguläre Datei sein: keine Symlinks, Verzeichnisse,
  Sockets oder sonstigen Spezialdateien im unterstützten unveränderlichen
  Zustand. Unter Unix werden zusätzlich Device/Inode von Vorprüfung und
  geöffnetem Handle verglichen. Dies ist eine zusätzliche Fehlererkennung,
  kein atomarer Race-Ausschluss.
- Wiederverwendung von `inspect_commit` und
  `object_oid(ObjectKind::Commit, &bytes)` aus `objects.rs`; keine neue
  Hashimplementierung. Ein richtiger Dateiname mit falschen Bytes scheitert.
  Die bestehende nachgelagerte `check_commit`-Prüfung bindet weiterhin
  Tree-/Parentlinks an die separat gelieferten Metadaten.
- Fehler liefern ausschließlich `Err(BridgeError)` ohne Pfad-/Inhaltsdetails
  und ohne Teilbytes, PASS-Bericht oder Ausgabe. Fehlender Pfad:
  `RAW_COMMIT_PATH_MISSING`; andere I/O-Fehler: `RAW_COMMIT_IO_ERROR`;
  Pfad-/Typfehler: `RAW_COMMIT_ABSOLUTE_DIRECTORY_REQUIRED`,
  `RAW_COMMIT_INVALID_DIRECTORY`, `RAW_COMMIT_DIRECTORY_TYPE_REJECTED`,
  `RAW_COMMIT_FILE_TYPE_REJECTED`; Grenzen/Änderungen:
  `RAW_COMMIT_TOO_LARGE`, `RAW_COMMIT_FILE_CHANGED`,
  `RAW_COMMIT_SIZE_CHANGED`. OID-/Parser-/Hashfehler stammen aus den
  vorhandenen Prüfern, insbesondere `INVALID_OBJECT_ID` und
  `COMMIT_HASH_MISMATCH`. Keine Wiederholung oder Ersatzbeschaffung.

### Tatsächlicher Sicherheitsvertrag und offene Grenzen

**Nur für vertrauenswürdige, unveränderliche und privat kontrollierte lokale
Verzeichnisse samt Dateien und sämtlichen Elternpfaden.** Diese Bedingung
muss der Aufrufer extern sicherstellen; der Konstruktorname bestätigt sie,
prüft oder erzeugt aber keine Isolation. Auch Änderungen über Hardlinks,
Mounts oder andere schreibfähige Handles müssen extern ausgeschlossen sein.
Rechtebits allein würden diesen Vertrag nicht beweisen; keine automatische
Snapshotanlage, chmod-Änderung oder Schreibaktion an der Quelle.

Die verwendeten portablen synchronen `std`-Pfad-/Dateioperationen bieten hier
keine atomare descriptorbasierte Pfadtraversierung. Zwischen Metadatenabfrage
und `File::open` bleiben TOCTOU- und Symlink-/Verzeichnisaustauschrennen.
Bei verletzter Voraussetzung könnte open einer Umleitung folgen oder bei
einer ausgetauschten Spezialdatei blockieren, bevor eine Handleprüfung greift.
Auch ein Inodevergleich schließt dies nicht aus. Keine Sicherheit für
feindlich veränderbare Verzeichnisse behauptet; kein Race-Test als solcher
ausgegeben. Netzwerkdateisysteme, Geräte und harte I/O-Zeitgrenzen sind nicht
qualifiziert. Die Bytegrenze ist keine Deadline oder Garantie gegen
blockierendes Kernel-I/O. Unveränderlichkeit und lokale reguläre Dateien
sind daher Vertragsvoraussetzungen, keine hier bewiesenen Eigenschaften.

### Neue echte Dateitests

Neun neue Tests in `src/raw_commit_tests.rs`, zusätzlich zu den acht
bisherigen Speichervertragstests. Eigener per `create_dir` neu angelegter
Tempbaum pro Test, unter Unix initial Modus 0700; keine Wiederverwendung
existierender Pfade, keine produktiven Repositories und keine Unterprozesse.
Fixtures schreiben nur innerhalb dieses eigenen Baums. Änderungen zwischen
Leseaufrufen modellieren getrennte statische Zustände, keine erlaubte
gleichzeitige Mutation während eines Quellenaufrufs.

- Byteidentität mit Nicht-UTF8, mehrfacher, einfacher und fehlender finaler
  Newline; vorhandene Commit-OID- und Metadatenprüfung; Quelldatei unverändert.
- Entfernte/zusätzliche Newline, andere Nachricht und Blob-/Tree-Hashdomäne
  bei richtig aussehender Commitstruktur: Fehler statt Teilresultat.
- Ungültige OIDs einschließlich Pfadversuch, Option, Revision, Kurz-/Überlänge,
  Großbuchstaben, Unicode, LF/NUL; Prüfung vor Zugriff auf entfernten Quellpfad.
- Fehlende Datei, NUL, CR, leere Nachricht, Objektheader und REST-JSON.
- Kein Dateinamenfallback auf `.commit`; nachgelagerter Metadatenwiderspruch.
- Exakt 1 MiB erfolgreich im Testentwurf, 1 MiB plus ein Byte abgewiesen.
- Relative/fehlende Verzeichnisauswahl, `..`, Datei statt Verzeichnis und
  Verzeichnis statt Rohcommitdatei.
- Unter Unix: Dateisymlink, Verzeichnissymlink, Symlink im Elternpfad und
  statischer Austausch eines ausgewählten Verzeichnisses nach Konstruktion.
- Unter Unix: echter Unix-Socket statt Datei, ohne open darauf.

Erfolgs- und Fehlerpfade prüfen weiterhin `production_publish` als gesperrt.
Fehlerprüfungen laufen auch über `check_commit` und erhalten nur `Err`, keinen
erfolgreichen Committeilbericht. Kein neuer CLI-Pfad; diese Tests belegen
keine neue CLI-Teilstdout-Eigenschaft. Keine deterministische Ausführung der
Race-/Read-I/O-Fehlerzweige behauptet. **Alle neuen Tests IMPLEMENTIERT,
NICHT AUSGEFÜHRT.** Keine Builds, Tests, Rustfmt, Clippy oder Runner gestartet.

### Provenienz, verbleibende Blockaden und Integration

`provenance()` klassifiziert ausschließlich als
`LOCAL_EXPLICIT_RAW_COMMIT_FILES`. Hashidentität belegt Inhaltsintegrität
relativ zur angeforderten SHA1-OID, keine authentifizierte GitHub-Herkunft,
Repositoryzugehörigkeit, Ref-Erreichbarkeit oder Aktualität. Keine automatische
Provenienzpromotion und keine Änderung bestehender Readback-Berichtsscopes.
Kein realer Rohcommitbestand wurde ausgewählt oder gelesen. Herkunft,
Bereitstellung und Unveränderlichkeit späterer Originaldateien müssen separat
belegt werden; REST-JSON ist keine Quelle rekonstruierter Originalbytes.

Die Git-Unterprozessquelle bleibt wegen fehlender qualifizierter Prozess-/
Dateisystemisolation **BLOCKED, nicht implementiert**. Dieser Dateiadapter
umgeht diese Blockade nicht durch einen versteckten Git-Aufruf.
HTTPS unverändert offen: keine harte 32-KiB-Headerbytegrenze vor Allokation;
Headeranzahl und Bibliothekspuffer ersetzen sie nicht. Exakt absolute Deadline
ohne Übergabelücke und separate Phasengrenzen bleiben unqualifiziert.

Nächster Integrationsschritt: unabhängige Runner-Qualifikation von check,
Formatierung, Clippy und Tests; danach explizite Verzeichnisübergabe an einen
gesondert beauftragten lokalen Readback-Aufrufer samt Bereitstellungsprovenienz
und belegtem unveränderlichem Pfadvertrag. Für feindlich veränderbare Quellen
zuerst atomare Dateisystemisolation separat entwerfen und qualifizieren.
GitHub-Erhebung benötigt weiterhin gesonderte Freigabe und vollständige
authentifizierte Repository-/Ref-/Tree-/Blob-/Originalcommit-Evidenz.

Keine Manifest-/Lockfileänderung, Dependencybeschaffung, Git-Objektbeschaffung,
Shell im Adapter, Netzwerkverbindung, Git-Schreiboperation, Veröffentlichung,
last_verified-Promotion oder Hardware-/Imagearbeit.

LOCAL_RAW_COMMIT_FILES=IMPLEMENTED_NOT_QUALIFIED;
LOCAL_GIT_RAW_SOURCE=BLOCKED_NOT_IMPLEMENTED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-288 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 104 erfolgreiche Rust-Tests.

Auftrag: direkte lokale Rohcommit-Dateiquelle,
ohne Git-Unterprozess und ohne Netzwerkzugriff.

Prüfergebnisse:

- cargo check: Exit 0
- cargo fmt: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Qualitätsstatus: PASS

Die tatsächlich implementierten Eigenschaften,
Tests und Einschränkungen stehen im zugehörigen
Codex-Checkpoint.

Lokale Rohcommitbytes beweisen keine
authentifizierte GitHub-Herkunft.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-289 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand:
113 erfolgreiche Rust-Tests aus Block 288.

Auftrag:
Integration der bestehenden RawCommitFiles-
Dateiquelle in den vorhandenen vollständigen
Offline-Readback-Verifier.

Prüfungen durch den Runner:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Gesamtqualifikation: PASS

Die Originalausgaben des Runners sind die
maßgebliche Quelle für konkrete Testergebnisse.

Ein erfolgreicher Offline-Test beweist keine
authentifizierte GitHub-Herkunft.

Die Rohcommit-Dateiquelle erfordert weiterhin
privat kontrollierte unveränderliche
Verzeichnisse.

Die produktive Veröffentlichung und die
Änderung von last_verified bleiben gesperrt.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-290-CODEX

Datum: 2026-10-26. Implementierungscheckpoint; keine Runner-Abnahme.
Ausgangslage laut Block 289: 115 Rust-Tests und 20 lokale TLS-Tests PASS.
Bestehende Checkpoints bleiben erhalten.

### Schnittstellen und verbundener Datenfluss

Die konkrete Implementierung von ResponseSource für HttpsGetSource passt
direkt zu RestReader<S: ResponseSource>::new. Kein Adapter und keine Änderung
am Produktivcode erforderlich. Die neuen Integrationsfälle liegen im
Unit-Testziel unter https::local_tls, damit die ausschließlich cfg(test)
verfügbare Loopbackauflösung privat bleiben kann. Kein neuer produktiver
Konstruktor, frei wählbarer Zielhost oder alternativer HTTP-Client.

Implementierter Pfad:
lokaler OpenSSL-TLS-Server -> HttpsGetSource::get/fetch ->
RestReader<HttpsGetSource> -> verify_initial_index -> check_commit und
RawCommitFiles -> vorhandene Commit-/Tree-/Blob- und Statusprüfung.
RawCommitFiles ist eine getrennte Dateiquelle für die beiden Originalcommits;
REST liefert ausschließlich Metadaten und Tree-/Blobbeschreibungen.
Die Speicherrohbytes werden vor dem Verifieraufruf entfernt.

Die bestehende Fixture aus tests/protocol.rs wurde ohne Änderung ihres
Objektmodells nach src/readback_fixture.rs verschoben und wird von beiden
Testzielen eingebunden. Der vorhandene OpenSSL-Testserver unterstützt jetzt
zusätzlich pfadabhängige Antworten und zwei zeitlich getrennte Refantworten.
Keine zweite Verifierimplementierung, HTTP-Architektur oder Git-Prozessquelle.
OpenSSL bleibt der vorhandene lokale TLS-Testgegenpart, die Testlogik ist Rust.

### Neue Integrationsfälle

Sieben neue Rust-Tests in src/https_local_tls.rs:

- local_tls_full_readback_raw_files: vollständige konsistente Repository-,
  Ref-, Commit-, Tree- und Blobkette, Commit-OIDs und exakte Statusbytes.
- local_tls_readback_wrong_raw_commit: COMMIT_HASH_MISMATCH.
- local_tls_readback_missing_raw_commit: RAW_COMMIT_PATH_MISSING.
- local_tls_readback_wrong_tree_metadata: COMMIT_METADATA_MISMATCH.
- local_tls_readback_wrong_blob: BLOB_HASH_MISMATCH bei gleicher Bytelänge.
- local_tls_readback_ref_changed: FINAL_HEAD_MISMATCH nach Objektprüfung.
- local_tls_readback_failed_get_no_retry: HTTP_STATUS_REJECTED bei 503;
  Folgeaufruf ergibt HTTPS_SOURCE_CLOSED, genau ein serverseitiger GET
  im bestehenden endlichen Beobachtungsfenster.

Alle Fälle prüfen unveränderte Kandidatenbytes, last_verified=null,
CURRENT_UNKNOWN, UNKNOWN für Build/Upload/Fachabnahme, bestehende
review_required-Einträge und die unveränderte Veröffentlichungssperre.
Positivfall und Refwechsel verlangen beide tatsächlichen Ref-GETs sowie
die beiden Commit- und die Blobabfrage in der serverseitigen Aufzeichnung.
Die bestehenden Tests für unbekannte CA und falschen Zertifikatshost
decken Zertifikatsfehler weiterhin ohne HTTP-Anfrage ab; keine Verdoppelung.

Die Tests sind auf echte lokale TLS-Handshakes und über TLS übertragene
HTTP-Antwortbytes angewiesen; es gibt keinen In-Memory-Ersatzpfad.
Jeder Fall erzeugt eine eigene synthetische Test-CA und ein Serverzertifikat
für api.github.com. Testdaten und Fehlervarianten sind fest vorgegeben;
Schlüssel, Tempnamen und Loopbackport entstehen zur Laufzeit.
Im neuen Readback-Test wird vor der ersten Anfrage Authorization entfernt;
der Server und die Assertions verlangen credentialfreie Requests.
Host und SNI bleiben api.github.com mit testseitigem Loopbackport.
Die Zertifikatsprüfung bleibt aktiv.

**Diese sieben Tests wurden hier NICHT AUSGEFÜHRT. Daher ist für Block 290
noch keine tatsächlich erfolgte TLS-Byteübertragung oder erfolgreiche
Qualifikation beobachtet.** Keine Cargo-Builds, Tests, Clippy, Downloads oder
Netzwerkaufrufe ausgeführt. Direkter rustfmt-Aufruf für die drei geänderten
Rust-Dateien erfolgreich; dies ist kein Kompilierungs- oder Laufzeitnachweis.

### Unveränderte Prüfer und Sicherheitsgrenzen

Wiederverwendet werden verify_initial_index, check_commit,
inspect_commit/verify_commit, object_oid, flatten_verified/tree_oid,
verify_single_addition, verify_blob und die Statusvorbereitung/-validierung.
RawCommitFiles bleibt unverändert. Berichtsscope bleibt absichtlich
OFFLINE_SUPPLIED_RESPONSES; lokale synthetische TLS-Daten werden weder zu
GitHub-Provenienz noch zu Live-Frische oder fachlicher Abnahme aufgewertet.

Produktiver Transport unverändert: feste URL-/Pfad-Allowlist und
api.github.com, GET-only, keine Redirects/Proxies/automatischen Retries,
aktive rustls-Zertifikatsprüfung, endliche Request-/Body-/Gesamtbudgets.
Kein unsicherer Client und keine produktive Hostkonfiguration hinzugefügt.
Die fehlende harte Headerbytegrenze vor Parserallokation bleibt separat offen;
100 Header und Bibliothekspuffer belegen keine 32-KiB-Headerbytegrenze.
Absolute Deadline ohne Übergabelücke und getrennte Phasengrenzen bleiben
weiterhin unqualifiziert.

RawCommitFiles setzt privat kontrollierte unveränderliche lokale Dateien,
Verzeichnisse und Elternpfade voraus. Die Testrohdateien werden vor
Quellenkonstruktion vollständig eingerichtet, anschließend nicht verändert.
TOCTOU bei veränderlichen Pfaden, Hardlink-/Mountaustausch und fehlende harte
Dateisystem-I/O-Deadline werden dadurch nicht gelöst oder qualifiziert.

### Offene Herkunftsbelege und nächste Schritte

Zuerst unabhängige Runner-Qualifikation: Offline-Kompilierung, Formatprüfung,
Clippy und gesamter Testsatz einschließlich der sieben neuen TLS-Fälle.
Der vorhandene lokale OpenSSL-Gegenpart wird dafür benötigt.
Danach Headerbytebegrenzung vor Parserallokation und Dateisystemisolation
als eigene Aufgaben behandeln. Authentifizierter GitHub-Live-Readback,
Bereitstellungsprovenienz echter Originalcommitdateien sowie authentifizierte
Repository-/Ref-/Tree-/Blobevidenz bleiben offen und benötigen eigene Freigabe.

Keine Manifest-/Lockfileänderung, externe Beschaffung, Git-Mutation,
Veröffentlichung, last_verified-Promotion oder Hardware-/Imagearbeit.

TLS_RAW_FILES_READBACK=IMPLEMENTED_NOT_RUN;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-290 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 115 bestandene Rust-Tests.

Auftrag: vollständige Verbindung von
lokalem TLS-Transport, REST-Reader,
Git-Objektprüfung und RawCommitFiles.

Äußere Prüfergebnisse:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Rust-Qualität: PASS

Die eigentliche End-to-End-Abnahme richtet
sich nach den konkret ausgeführten
TLS-Integrationstests im Originalprotokoll,
nicht allein nach den Exitstatus.

Unverändert offen:
- authentifizierte private GitHub-Erhebung;
- Originalcommit-Herkunftsnachweis;
- harte Headerbytebegrenzung;
- allgemeine TOCTOU-Sicherheit bei
  veränderlichen Rohcommit-Verzeichnissen.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-291-CODEX

Datum: 2026-10-26. Quellenprüfung und Rust-Testergänzung; keine Ausführung
von Cargo, Builds, Tests oder Netzwerkaktionen durch Codex. Ausgangsqualifikation
laut Auftrag/Block 290: 122 Rust-Tests, darunter 27 lokale TLS-Tests, sowie
Check/Rustfmt/Clippy und TLS-/REST-/Git-Readback PASS. Das ist keine neue
Qualifikation dieses Nachtrags; dessen Runner-Ergebnisse stehen noch aus.

### A – Lokale Originalquellen und tatsächliche APIs

Geprüfte Bibliotheksquellen unter
`/home/loomit/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`:
reqwest 0.13.5, hyper 1.12.0, hyper-util 0.1.21, httparse 1.10.1 und
bytes 1.12.1; Versionszuordnung anhand des unveränderten Cargo.lock.
Zentrale Fundstellen (Pfade relativ zum jeweiligen Crateverzeichnis):

| API / Fundstelle | Tatsächliche Bedeutung und Verfügbarkeit |
|---|---|
| reqwest `src/blocking/client.rs:474`, `src/async_impl/client.rs:1560`, Weiterleitung `:1003` | `http1_max_headers(usize)` setzt nur die Anzahl; Standard 100. Kein Mindestwertcheck im Setter. Im Bridge-Native-Build verfügbar. |
| reqwest `src/blocking/client.rs:537` | `http2_max_header_list_size(u32)` nur mit Feature `http2`; hier nicht aktiviert und wegen `http1_only()` ohnehin kein HTTP/1-Schutz. |
| hyper `src/client/conn/http1.rs:500` | `max_headers(usize)` begrenzt Parser-Slots, keine Bytezahl und keinen einzelnen Wert. Kein Mindestwertcheck im Setter. |
| hyper `src/client/conn/http1.rs:517` | `max_header_size(usize)` umfasst den vollständigen Antwortkopf einschließlich Statuszeile; Standard `None`, kein Mindestwertcheck. Auch Chunk-Trailer-Limit. HTTP/1-Client-API, kein zusätzliches Sonderfeature. |
| hyper `src/client/conn/http1.rs:554` | `max_buf_size(usize)` setzt adaptive Empfangsstrategie und Schreibpuffergrenze; Minimum 8192, darunter Panic. Standard 417792 Bytes (`8192 + 4096 * 100`). |
| hyper `src/client/conn/http1.rs:539` | `read_buf_exact_size(Option<usize>)` setzt die Lesestrategie, löscht `max_buf_size`; umgekehrt löscht letzteres die Exact-Einstellung. Kein Mindestwertcheck im Setter. Keine separate Headerbytegarantie. |
| hyper-util `src/client/legacy/client.rs:1107,1123,1297` | `http1_read_buf_exact_size`, `http1_max_buf_size`, `http1_max_headers` reichen an Hyper durch, Feature `http1`; zusätzlich Legacy-Client-Modul. Keine Legacy-Weiterleitung von `max_header_size` gefunden. |

Entscheidende Lücke: reqwest exponiert in weder Blocking- noch Async-Builder
`http1_max_buf_size`, `http1_read_buf_exact_size` oder eine
HTTP/1-`max_header_size`-Weiterleitung. Die private Clientkonfiguration und
die Build-Weiterleitung bestätigen dies. Die vorhandene Abhängigkeit nutzt
HTTP/1 und Client/Client-Legacy; das bloße Vorhandensein tieferer APIs macht
sie am reqwest-Builder nicht verfügbar. Keine erfundene Methode ergänzt.

### B – Durchgesetzte Grenzen und Allokationsgrenze

`HEADER_MEMORY_BOUNDARY=BLOCKED` für die Anforderung maximal 32768
HTTP-Response-Headerbytes möglichst vor unbeschränkter Parserallokation.
Keine nachträgliche HeaderMap-Prüfung als vermeintliche Lösung eingeführt.
Keine zweite HTTP-Implementierung, kein eigener Parser, keine Dependency-
oder Manifeständerung. Produktive Verhaltensänderung: keine; lediglich
präzisierte Kommentare am Client und zusätzliche Rust-Tests.

Quellenbefunde, ausdrücklich getrennt:

- **Anzahl:** Bridge setzt weiterhin 100 Headerfelder. Hyper baut daraus
  begrenzte temporäre Parser-/Indexfelder; httparse meldet TooManyHeaders.
  Dies beschränkt keine Länge eines Feldes oder der Statuszeile.
- **Einzelwert:** Kein konfiguriertes eigenes Byte-Limit. Auch unbekannte
  Header durchlaufen den normalen Parser und die HeaderMap-Konstruktion.
- **Gesamtkopf:** Kein durch reqwest konfiguriertes 32-KiB-Limit.
  Statuszeile, Namen, Werte, Trennzeichen und abschließende Leerzeile sind
  Wirebytes; die HeaderMap bildet diese Summe nicht verlustfrei ab.
- **Interner Empfangspuffer:** Hyper `src/proto/h1/io.rs:179–258` versucht
  zuerst `parse_headers`; nur bei unvollständigem Kopf prüft es aktuelle
  Länge gegen `max_header_size` und `ReadStrategy::max`. Der Standardwert
  417792 ist daher keine exakte Grenze vollständig empfangener Köpfe.
  `poll_read_from_io` begrenzt den Reservewunsch anhand der Strategie,
  übergibt dem Transport aber die gesamte freie `BytesMut`-Kapazität.
  `bytes/src/bytes_mut.rs:610–790` erlaubt Kapazitätswachstum über den Wunsch
  hinaus. Daraus folgt keine harte Heap- oder Wirebyteobergrenze von 32 KiB;
  auch TLS-Puffer und weitere Parserobjekte zählen separat.
- **Parserinterne Kopfprüfung:** Hyper `src/proto/h1/role.rs:1037–1180`
  prüft ein konfiguriertes `max_header_size` nach erfolgreichem httparse,
  aber vor Reason-Phrase-Kopie und HeaderMap-Aufbau. Empfangspuffer und
  temporäre Parserfelder existieren dann bereits. Bei unvollständigem Kopf
  greift zusätzlich die genannte Prüfung in `io.rs`. Auch diese API wäre
  deshalb kein Beleg für eine Gesamtprozess-Speichergrenze von 32768 Bytes.
- **Unvollständige Köpfe / Slow-Drip:** EOF oder vorhandene Request-Deadline
  beendet den Request; die Deadline wird nicht durch jeden Headerbyteempfang
  neu gestartet. Zeitgrenze und Speichergrenze bleiben verschiedene Aussagen.
  Informational Responses können Köpfe verbrauchen und die aktuelle
  Pufferlänge reduzieren; daraus entsteht kein kumulatives Headerbudget.

TLS-Verifikation, feste Host-/Pfad-Allowlist, GET-only, Redirect-/Proxy- und
Retry-Sperren, Request-/Operationsdeadline, Bodybudgets, Reader-/Verifier-
Verträge und CLI-Publikationssperre bleiben erhalten. Die bisher offene
absolute Deadline ohne Übergabelücke und getrennte Phasenlimits werden
hierdurch nicht qualifiziert.

### C – Neun ergänzte lokale TLS-Fälle, noch nicht ausgeführt

Die vorhandene OpenSSL-Loopbackinfrastruktur und der unveränderte echte
reqwest-fetch-Pfad werden wiederverwendet. Alle Daten sind synthetisch;
keine Verbindung zu GitHub. Die neu angelegten Fälle erwarten:

1. Kopf mit exakt 32767 Wirebytes: angenommen.
2. Kopf mit exakt 32769 Wirebytes: derzeit ebenfalls angenommen.
3. Einzelner langer unbekannter Header in 65536-Byte-Kopf: angenommen.
4. Lange Statuszeile in 40960-Byte-Kopf: angenommen.
5. Genau 100 Felder (99 unbekannte plus Content-Length), insgesamt über
   32768 Kopfbytes: angenommen.
6. Genau 101 kleine Felder, insgesamt unter 32768 Bytes: Parserfehler.
7. Unvollständiger 65536-Byte-Kopf und TLS-EOF: Fehler.
8. Unvollständiger 524288-Byte-Kopf mit EOF-Fallback: Fehler; der Test allein
   unterscheidet Pufferablehnung nicht von EOF und misst keinen Heap-Peak.
9. Header-Slow-Drip (höchstens 50 einzelne Bytes im 50-ms-Abstand): Abbruch
   unter verkürzter 600-ms-Requestdeadline, erlaubtes Messfenster 300–1800 ms,
   vor dem separaten 3-s-Serverfallback.

Die Annahme oberhalb 32 KiB ist eine präregistrierte Charakterisierung der
Lücke, kein erwünschter Sicherheits-PASS. Erst Runner-Ausführung kann die
erwarteten Laufzeitergebnisse bestätigen. Die Eingaben sind auf höchstens
512 KiB begrenzt, ohne unendlichen Generator; vorhandene Clienttimeout- und
Server-Abbruchmechanismen bleiben aktiv. Die Fehlerfälle prüfen über die
bestehenden Hilfen statische Fehler, keine Geheimnisse, genau einen
beobachteten GET und geschlossene Quelle beim expliziten Folgeaufruf.
Der Slow-Drip-Fall erwartet ebenfalls exakt den statischen Requestfehler.
Das endliche 250-ms-Beobachtungsfenster ist kein universeller Retrybeweis.
Redirects bleiben durch den vorhandenen echten TLS-Redirecttest abgedeckt.
Keiner dieser Fälle misst Parserallokationen oder beweist eine vorgezogene
32-KiB-Grenze. Neu erwarteter Gesamtumfang: 131 Rust-Tests, 36 TLS-Tests;
keine neue PASS-Behauptung vor äußerer Qualifikation.

### D – Integrität, Restrisiken und nächster Integrationsschritt

Veränderte Dateien ausschließlich:
`src/https.rs`, `src/https_local_tls.rs`, `BRIDGE-CODEX-PROGRESS.md`
(jeweils unter `tools/novena-bridge/`). Alle früheren Checkpoints bleiben
erhalten. Keine Änderung an `protocol.rs`, RawCommitFiles, unabhängiger
Git-Objektprüfung, Statusdaten oder `last_verified`.

Die sieben vorhandenen TLS-/REST-/Git-/RawCommitFiles-Readbackfälle bleiben
als Regressionskette erhalten. Statische Bewertung: mangels funktionaler
Clientänderung keine neue Verzweigung in dieser Kette. Laufzeitintegrität
dieses Arbeitsstands ist erst vom äußeren Runner zu bestätigen:
Offline/Locked Check, Rustfmt-Prüfung, Clippy und vollständiger Testsatz.
Codex hat diese Werkzeuge ausdrücklich nicht gestartet.

Offen bleiben authentifizierte private GitHub-Erhebung, unabhängiger
GitHub-/Repository-/Ref-/Tree-/Blob-Herkunftsnachweis und die Herkunft echter
Originalcommitdateien. Lokale TLS-Fixtures und Hashgleichheit ersetzen diese
Belege nicht. RawCommitFiles erfordert weiterhin privat kontrollierte,
während Konstruktion und Nutzung unveränderliche Dateien und Elternpfade;
Vorprüfung, open und Metadatenabgleich lösen TOCTOU, Hardlink-/Mountaustausch
oder eine harte Dateisystem-I/O-Deadline nicht.

Nächster klarer Schritt: äußere Qualifikation dieses unveränderten
Transportpfads einschließlich der neun Charakterisierungsfälle. Danach in
einem gesonderten Integrationsauftrag eine reqwest-Weiterleitung für
HTTP/1-Kopf- und Empfangspufferlimits anhand konkreter verfügbarer Quellen
bewerten; ein Dependencywechsel oder Patch ist hier nicht freigegeben.
Vor einer Sicherheitszusage zusätzlich Reserve-/Kapazitätsüberschreitungen,
vollständige/partielle Köpfe, Statuszeile und Informational Responses prüfen.
Bis zur nachgewiesenen Lösung bleibt die Headeranforderung blockiert und
produktiver GitHub-Zugriff beziehungsweise Veröffentlichung gesperrt.

HEADER_MEMORY_BOUNDARY=BLOCKED;
HEADER_TLS_CHARACTERIZATION=IMPLEMENTED_NOT_RUN;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-291 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand:
122 erfolgreiche Rust-Tests aus Block 290.

Auftrag:
Tatsächliche HTTP-Header- und Speichergrenzen
des HTTPS-Adapters prüfen und soweit möglich
verbessern.

Prüfungen:
- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Rust-Gesamtqualität: PASS

Ein erfolgreicher Regressionstest beweist
keine vollständig begrenzte Parserallokation.

Konkrete Schutzgarantien und Blocker sind
dem Codex-Checkpoint und Originalprotokoll
zu entnehmen.

Produktive GitHub-Herkunft weiterhin nicht
nachgewiesen.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

---

## NOVENA-BRIDGE-RUST-292-CODEX

Datum: 2026-10-26. Ergebnis: konkreter Architektur- und Implementierungsplan,
keine Implementierung oder neue Laufzeitqualifikation. Ausschließlich dieser
Nachtrag wurde ergänzt; sämtliche vorherigen Checkpoints bleiben erhalten.
Veröffentlichung und `last_verified` bleiben gesperrt.

### Grundlage und Evidenzstatus

Arbeitsgrundlage: AGENTS.md, NOVENA-ARBEITSREGELN.txt, Repository-/Bridge-README,
Fortschrittscheckpoints bis einschließlich 291, Bridge-Plan und einschlägige
Projekt-/Test-/Entscheidungsverweise; insbesondere die Grenzen aus 286/287,
der Dateivertrag aus 288 und die Integrationen 289/290 sowie Headeranalyse 291.
Quellgrundlage sind die vorhandenen Rust-Verträge in objects.rs, protocol.rs,
https.rs, raw_commit_files.rs, attestation.rs, status.rs und planning.rs samt
Quellen-/Logeingängen und den zugehörigen Testfällen. Lokale Git-Manpages
bestätigen den Rawinhalt-/Typvertrag von cat-file und die lokalen Schreibfolgen
von fetch. Das ist keine Qualifikation eines Git-Erhebungsadapters.

Verbindlicher verifizierter Ausgangsstand laut Auftrag: 131 Rust-Tests und
36 lokale TLS-Tests bestanden; vollständige lokale TLS-Readback-Kette bestanden.
In 292 nicht wiederholt. Die äußeren Checkpoints enthalten zusammenfassende
Qualitätsmeldungen; die begrenzte Dateinamensuche unter test-logs/, evidence/
und research/ lieferte keine passenden Bridge-Rohprotokolle. Daher keine neuen
Einzelergebnisse oder nachträglich erfundenen Primärausgaben. Lokale TLS-Daten
beweisen weiterhin keine private GitHub-Erhebung. Aktuelle offizielle
GitHub-Verträge wurden wegen des ausdrücklichen Netzwerkverbots nicht abgerufen;
die unten genannten GitHub-seitigen Fähigkeiten sind extern zu verifizieren.

Der Eingangsarbeitsbaum enthält bereits Nutzeränderungen und die unversionierte
Bridge-Crate. Keine dieser Arbeiten wurde bereinigt oder in Git aufgenommen.
Kein äußerer Runner wurde gestartet, weil nur diese Dokumentdatei schreibend
freigegeben ist. Keine Live-Anfrage, API-Abfrage, SSH-Verbindung, Clone/Fetch,
Git-Mutation, Cargo-Ausführung, Dependencyänderung oder Pythonimplementierung.

### Höchstens drei Varianten und Architekturentscheidung

| Variante | Technische Eignung | Entscheidung |
|---|---|---|
| A: isolierter Git-Erheber über SSH, anschließend unabhängiger Rust-REST-Readback | Git transportiert Originalobjekte; Git übernimmt Pack-/Deltaauflösung. Gepinnter SSH-Hostschlüssel und repositorygebundener Nur-Lese-Deploy-Key authentifizieren den getrennten Beschaffungsweg. Rust übernimmt bestehende Objektprüfung. Benötigt einen qualifizierten Supervisor und eine neue, enge SSH-Freigabe. | **Bevorzugt**, unter den nachstehenden zwingenden Freigabegates. Kein heute freigegebener oder bereits implementierter Pfad. |
| B: isolierter Git-Erheber über Smart HTTPS plus Rust-REST-Readback | Ebenfalls Originalobjekte, etablierter Git-Transport. Benötigt zusätzlich separat qualifizierte Git-/HTTP-/CA-/Credentialkonfiguration, Headergrenzen und Retry-/Redirectkontrolle; die reqwest-Qualifikation gilt nicht für Gits HTTP-Stack. | Realistische Alternative, für diesen Plan nicht weiter verfolgt; vermeidet die vorhandenen Sicherheitslücken nicht. |
| C: nativer Rust-Git-Transport mit etablierter Git-Bibliothek plus REST | Vermeidet Git-Unterprozesse, benötigt aber neue Dependencies und Qualifikation von SSH/HTTPS, Pack-/Deltaauflösung, Dekompressionsgrenzen und Objektformaten. Aktuelle direkte Dependencies liefern diesen Adapter nicht. | Realistisch, aber größerer unqualifizierter Umfang; nicht gewählt. Kein eigener Pack-/TLS-Nachbau. |

Eine reine REST-JSON-Lösung ist kein vierter Beschaffungsweg: Die vorhandene
Route `/git/commits/<oid>` liefert Metadaten. Weder JSON noch Patch/Diff,
Commitansicht oder ein unbewiesener Accept-Header garantieren originale
Git-Commitbytes. Author-/Committertexte, Zeitzonen, Headerreihenfolge,
Signaturfortsetzungen und Nachricht einschließlich Schluss-LF dürfen niemals
aus REST rekonstruiert werden. Die gewählte Architektur braucht keinen
angeblichen REST-Rawcommit-Endpunkt und keine Erweiterung auf freie Raw-URLs.

### Konkrete Komponenten, Datenfluss und Verträge

Neue Komponentenbezeichnungen in diesem Abschnitt sind Entwurfsnamen,
keine bereits vorhandenen Rust-APIs.

1. **ReadbackSpec und RunBudget (Rust):** Explizite Eingabe bindet
   `loomit77/project-block-logs`, unabhängig bestätigte Repository-`node_id`
   und numerische `id`, `refs/heads/main`, erwartete Commit-OID C sowie bei
   Transitionsprüfung Baseline-OID B und exakte alte/neue Payloadbytes.
   Repository-IDs, Hostschlüssel und erwartete Hashes dürfen nicht erst aus
   derselben ungeprüften Erhebung als Vertrauensanker übernommen werden.
   C ist der erwartete bereits vorhandene Remotecommit; B ist dessen Parent,
   nicht die erste Refbeobachtung. Snapshot und Transition sind getrennte Modi.

2. **Authentifizierter REST-Einstieg:** Nach Schließung der Transportblocker
   liefert HttpsGetSource -> RestReader Repositorymetadaten und Ref R0.
   Erforderlich sind exakter Name, gepinnte IDs, `private=true`, richtiger
   Reftyp und R0=C. Der aktuelle Reader prüft bereits full_name und node_id;
   Prüfung von numerischer ID/private und Repositorywiederholung am Ende
   sind noch zu ergänzen. Ein fehlender unabhängiger ID-Pin blockiert den Lauf.
   Die heutige Existenz des Statuszieles wird nicht vorausgesetzt.

3. **GitAcquirer unter begrenztem Supervisor:** Genau ein neu angelegter,
   privater isolierter Bare-Speicher ohne Vorbestand, Checkout oder Verbindung
   zum Projekt- oder Transportrepository. Feste Zielidentität
   `git@github.com:loomit77/project-block-logs.git`, keine frei übergebene URL.
   SSH ausschließlich auf github.com:22, exakter separat bestätigter
   Hostschlüssel, StrictHostKeyChecking, eigenes unveränderliches known_hosts,
   kein Accept-new/TOFU, Agent, Agentforwarding, ProxyCommand, ProxyJump,
   Multiplexing, TTY, Passwortdialog oder lokaler SSH-Konfigurationsimport.
   Nur expliziter Nur-Lese-Deploy-Key für dieses Repository; keine Suche nach
   vorhandenen privaten Schlüsseln. SSH-Hostauthentifizierung ersetzt nicht
   die REST-Repository-ID-Bindung.

   Ein einzelner begrenzter Fetch der gepinnten C-OID, ohne Tags und ohne
   Submodule, mit Historientiefe höchstens zwei für C und B. GitHub-Unterstützung
   für diesen genauen OID-/Shallow-Vertrag ist vor Live-Einsatz zu bestätigen.
   Ablehnung, abweichender Tip oder fehlendes B führen zum Abbruch; kein
   Fallback auf einen inzwischen anderen Branch, Full-Clone oder Deepening.
   Der Erwerbsspeicher darf hierbei Packs und eine kontrolliert erzeugte
   Shallow-Grenze enthalten. Das ist ausdrücklich ein **neuer** Erwerbsvertrag,
   keine Erfüllung des engeren historischen Loose-only-Kandidaten aus 287.
   Shallow begrenzt Historie, nicht Packgröße, Deltaressourcen oder Blobbytes.

   Netzfreigabe gilt nur während der Erwerbsphase; danach wird dieser Speicher
   unveränderlich bereitgestellt und der Exportprozess erhält kein Netzwerk.
   Git-/SSH-Binaries, Helper, Konfiguration und Umgebung sind exakt kontrolliert:
   keine Includes, Alternates/http-alternates, commondir, Replace-Refs,
   Promisor-/Partial-Clone-Funktion, Lazy-Fetch, Hooks, Filter, Textconv,
   Mailmap, automatische Wartung oder Submodulrekursion. Nur SHA1-Objektformat.
   Kein fremder lokaler Objektspeicher wird übernommen. Der Erwerb schreibt
   ausschließlich in seinen eigens freigegebenen isolierten Speicher.

4. **RawCommitExport:** Im unveränderlichen Erwerbsspeicher für genau C und
   gegebenenfalls B zunächst exakte Typ-/Größenprüfung, dann
   `git cat-file commit <vollständige OID>` ohne Pretty-Print/Filter.
   Keine Shell und kein Revisionsausdruck. stdin und stderr geschlossen;
   stdout während des Empfangs begrenzt, Exitstatus vollständig geprüft.
   Typprüfung ist erforderlich, weil die Typform von cat-file auch triviale
   Dereferenzierung erlaubt. Kein Batch-Framing und kein angehängtes Ausgabe-LF.
   Maximal 1 MiB unveränderte unkomprimierte Commit-Nutzbytes je Objekt, ohne
   `commit <Länge>\0`-Präfix. Kein Trimmen, UTF8-Umschreiben oder LF-Ergänzen.
   Rust prüft `object_oid(Commit, bytes)` und inspect_commit. Unterschriebene,
   Merge- oder sonstige vom aktuellen schmalen Profil nicht unterstützte
   Commits bleiben harte Fehler; nicht normalisieren und nicht neu erzeugen.

   Export in ein neues privates versiegeltes Verzeichnis, exakte OID-Dateinamen,
   plus separater Erwerbsbeleg. Erst nach vollständigem erfolgreichem Abschluss
   übergibt der Supervisor das unveränderliche Verzeichnis an RawCommitFiles.
   Auch Elternpfade, Mounts, Hardlinks und offene Schreibhandles müssen unter
   seiner Kontrolle sein. chmod allein ist kein Unveränderlichkeitsbeweis.
   Der Bridge-Verifier liest keinen Git-Speicher und startet selbst kein Git.

5. **Unabhängige REST-Objektprüfung:** REST liefert Commitmetadaten, jeden
   nichtrekursiven Tree und jeden im Prüfumfang referenzierten Blob separat.
   RawCommitFiles -> check_commit bindet Originalcommit-OID, Tree und Parents
   an REST. flatten_verified prüft beide vollständigen Trees für Transitionen,
   einen für Snapshots. verify_initial_index beziehungsweise verify_update_index
   prüft ausschließlich die erlaubte Hinzufügung/Ersetzung, Parent C->B,
   Zielbytes und gegebenenfalls exakte Baseline. attestation::verify_snapshot
   ist ausschließlich Snapshotprüfung und ersetzt keinen Transitionsnachweis.
   Für Snapshotbytes keine zusätzliche kanonische Umserialisierung einführen.

   **Zusätzlich erforderlich für vollständige Blob-Verifikation:** Alle
   eindeutigen Blob-OIDs aus sämtlichen geprüften Trees aufzählen und mit
   RestReader::blob abrufen, Größe/base64/OID prüfen und SHA256 erfassen;
   dies umfasst unveränderte Geschwister, ausführbare Dateien und Symlinkbytes.
   Symlinks werden als Blobinhalt gelesen, nie verfolgt. Gitlinks sind fremde
   Commitreferenzen, keine Blobs: im ersten vollständigen Profil hart ablehnen,
   keine automatische Submodulbeschaffung. Nicht unterstützte Pfadkodierungen
   ebenfalls ablehnen. Vollständigkeit bezieht sich auf diese ein/zwei
   Commit-Trees, nicht auf die gesamte Historie oder unerreichbare Objekte.
   Ein fehlender oder zu großer Nebenblob verhindert Gesamt-PASS.

6. **Abschluss und privater Beleg:** Nach dem zusätzlichen Blobdurchlauf
   erneuter authentifizierter Repository- und Refabruf R1; dieselben IDs und
   R1=R0=C verlangen. Die heutigen internen Refprüfungen allein würden einen
   erst danach laufenden Zusatzdurchlauf nicht einklammern. Git-seitig
   beobachtete Refwerte, sofern vorhanden, müssen ebenfalls zu C passen;
   ein Fetchergebnis ersetzt keinen unabhängigen Refabruf.
   Anschließend separater geplanter Beleg `AUTHENTICATED_READBACK_V1`, der
   Erwerbsbeleg, REST-Originalantworten, Objektinventar und Prüfergebnis bindet.
   Bestehende `OFFLINE_SUPPLIED_RESPONSES`-/`VARIANT_A_OFFLINE_V1`-Berichte
   bleiben unverändert als innere Objektbelege erhalten und werden nicht
   umetikettiert. Kein Livebeleg allein aus caller-supplied JSON konstruierbar.

Der Erwerbsbeleg muss Lauf-ID, festes Repository/Ziel, ID-Pins, C/B, R0/R1,
SSH-Hostschlüsselfingerprint, Client-/Supervisor-/Quellidentitäten,
CA-Bundle-Identität für REST, Konfigurationshashes, monotone Reihenfolge und
lokale Beobachtungszeiten enthalten; ferner Objektart, OID, Länge und SHA256
jedes exportierten Originalcommits. Token/Schlüssel/Authorization/Traceausgaben
sind ausgeschlossen. Privates Originalmaterial bleibt in ausdrücklich
freigegebener geschützter Ablage; öffentliche Zusammenfassungen sind keine
Originale. Bei Übergabe zwischen Vertrauensbereichen muss ein separat
vertrauenswürdig bestätigter Manifesthash oder eine authentifizierte Signatur
mit unabhängig gepinntem Schlüssel die Übergabe binden. Ein selbst mitgelieferter
Sidecar allein authentifiziert den Exporteur nicht.

Git-OID bedeutet hier SHA1 über `<Typ> <dezimaler Bytezahl>\0<Nutzbytes>`.
SHA256-Belege hashen ausdrücklich die unveränderten Nutzbytes; Hashdomäne und
Länge werden mitgeführt, Wire-/Packhashes separat bezeichnet. Für Rohcommits
und alle Blobinhalte Rust-sha2 verwenden und später unabhängig mit einem
zweiten freigegebenen Werkzeug, beispielsweise sha256sum, auf denselben
versiegelten Originalen gegenprüfen. Trees erhalten zusätzlich SHA256 ihrer
kanonischen, bereits OID-verifizierten Git-Treebytes; diese sind aus vollständigen
Treeeinträgen serialisiert und werden nicht als original transportierte
Rohcommitbytes ausgegeben. Die bestehende Attestation hasht nur Zielbytes,
nicht schon sämtliche Commit-/Tree-/Blobbelege. SHA256 ersetzt weder Git-OID
noch authentifizierte Herkunft oder Commit-Signaturprüfung.

### API-, Transport- und Credentialvertrag

REST bleibt auf der vorhandenen Allowlist: Repository, main-Ref,
`git/commits/<oid>`, `git/trees/<oid>` ohne recursive-Query und
`git/blobs/<oid>`. Nur HTTPS/api.github.com:443, explizite geprüfte Roots,
Host-/Ketten-/Zeitprüfung und mindestens TLS 1.2. Nur vollständige 200-Antworten,
keine Redirects, Proxies, Retries, alternative Hosts oder Cacheantwort 304.
API-Version `2022-11-28`, Accept `application/vnd.github+json`, User-Agent
und identity-Encoding sind der heutige Clientvertrag; aktuelle Unterstützung,
Berechtigungsmodell und Tree-/Blobantwortformen bleiben extern zu bestätigen.
Kein 404 wird als Abwesenheitsbeweis von fehlenden Rechten unterschieden.
401/403/404/429/5xx sind Abbruch, kein Wiederholungsauftrag.

REST-Credential: explizit bereitgestelltes, auf dieses private Repository
begrenztes Token mit Contents-Leserecht und erforderlichem Metadatenzugang;
konkrete Tokenart/SSO-/Organisationsvorgaben extern bestätigen. Kein Schreibrecht.
SSH-Credential: getrennt freigegebener read-only Deploy-Key. Beide Eingänge
über vorab vereinbarte geschützte Deskriptor-/Credentialbereitstellung,
keine Geheimnisse in URL, argv, allgemeiner Umgebung oder Logs. Kein implizites
Auslesen von gh/Git/SSH-Agent. Notwendige Git/SSH-Hilfsprozesse müssen in der
Sandbox fest erlaubt sein. Deren exakter Credentialübergabevertrag ist vor
Implementierung abzunehmen; keine sichere Löschung oder Geheimnisfreiheit
ungeprüfter externer Diagnosen behaupten.

Die feste SSH-Upload-Pack-Sitzung verwendet serverseitig eine lesende
Git-Operation; Smart-Git-Protokollverkehr ist keine GitHub-REST-Mutation.
Der lokale Fetch ist trotzdem eine Git-Mutation im isolierten Erwerbsspeicher
und benötigt ausdrücklich eine eigene Freigabe. Niemals Push/receive-pack,
GraphQL, REST-Schreibmethoden oder Schreibcredential im Readbackpfad.

### Bereits abgedeckt, implementierbar und weiterhin blockiert

| Prüfpunkt | Bereits vorhanden / qualifizierter lokaler Umfang | Fehlender Umfang und Status |
|---|---|---|
| 1 Repositoryidentität | Fester ARCHIVE/PREFIX, full_name und erwartete node_id im RestReader | Unabhängige Pinprovenienz, numerische ID/private sowie reale Authentifizierung: extern zu verifizieren; Zusatzprüfungen implementierbar. |
| 2 Ref vorher/nachher | head(), verify_initial/update_index und verify_snapshot mit zwei Refprüfungen; lokaler Refwechseltest | Einklammerung der gesamten Erwerbung/Blobprüfung und echte Remote-Beobachtungen fehlen. |
| 3 Originalcommitbytes | RawCommitSource, RawCommitFiles, inspect_commit, byteerhaltende Tests | Authentifizierter Git-Erheber/Export nicht implementiert; bis Supervisorqualifikation BLOCKED. |
| 4 OID und SHA256 | object_oid/check_commit; sha2 vorhanden, Ziel-SHA256 in OfflineAttestation | Commit-/Gesamtobjektmanifest und unabhängige SHA256-Gegenprüfung implementierbar, noch nicht qualifiziert. |
| 5 Trees und Blobs | flatten_verified hasht alle traversierten Trees; Übergangsprüfung bindet alle Pfade/Modi/OIDs. blob() prüft abgerufene Inhalte. | Bestehende Verifier lesen nur Zielblob und bei Updates Baselineblob, nicht alle Nebenblobs. Gesamter Blobdurchlauf und Vollständigkeitsinventar fehlen. |
| 6 Ressourcen | HTTPS-/REST-Versuchs- und Bodylimits, Treegrenzen, 1-MiB-Dateigrenze; lokale Zeit-/Grenztests bestanden | Gemeinsamer Vorgang über Erwerb/REST/Export, Supervisor, Disk-/CPU-/RAMgrenzen und absoluter Abbruch nicht qualifiziert; Headergrenze BLOCKED. |
| 7 Herkunft der Rohdaten | RawCommitFiles klassifiziert korrekt LOCAL_EXPLICIT_RAW_COMMIT_FILES | SSH-/Repository-/Runbindung und authentifizierte Übergabe extern zu belegen; Hashgleichheit allein unzureichend. |
| 8 REST gegen Objektinhalt | Strikte getrennte Schnittstellen, check_commit gleicht nur Tree/Parents ab | Keine REST-Rohcommitgarantie vorhanden; gewählter Git-Weg erhält Originale. |
| 9 Refwidersprüche | INITIAL_HEAD_MISMATCH/FINAL_HEAD_MISMATCH und lokale Negativtests | Laufweiter gemeinsamer Konfliktzustand noch zu implementieren; keine automatische Neuauswahl. |
| 10 Readback gegen Veröffentlichung | production_publish/CLI gesperrt, publication_performed=false, Statusschutz erhält last_verified | Live-Readbackbeleg separat implementierbar; Veröffentlichung und Promotion bleiben gesondert gesperrt. |

Besonders verbindlich: **32-KiB-Headergrenze vor Parserallokation weiterhin
BLOCKED.** reqwest 0.13.5 exponiert die benötigte HTTP/1-Kontrolle nicht.
100 Felder, nachträgliche HeaderMap-Prüfung, ein Prozess-RAM-Limit oder SSH
für den Rohdatenweg lösen diese REST-Lücke nicht. Ein künftig gesondert
freigegebener Bibliothekspatch muss Kopf-/Empfangsgrenzen einschließlich
vollständiger und partieller Köpfe, langer Statuszeilen, Informational
Responses und Reservekapazität nachweisen. Ohne Nachweis kein produktiver
GitHub-Readback. Die exakt absolute Deadline ohne Übergabelücke sowie separate
Phasengrenzen bleiben ebenfalls unqualifiziert.

### Endliche Ressourcen und verbindliche Abbruchregeln

Folgendes ist das feste erste Abnahmeprofil des Plans, keine heutige Garantie:

- Ein serieller Lauf, keine Wiederholung, kein Resume/Deepening/Rollback.
  Höchstens zwei Commits, eine SSH-Erwerbssitzung, maximal 256 REST-Anfragen
  einschließlich Vor-/Nachprüfung und aller Blobs. Ein gemeinsamer Kontext
  überlebt Readerwechsel; kein Budgetneustart durch neue Quelleninstanzen.
- REST maximal 8 MiB je Antwort und 32 MiB kumulativ; Blob und Rohcommit
  jeweils maximal 1 MiB im vorhandenen Profil. Rohcommitexport maximal 2 MiB
  insgesamt; REST-Bodies plus exportierte Rohcommits zusätzlich gemeinsam
  maximal 32 MiB. Separat SSH-Eingang maximal 64 MiB einschließlich
  Protokoll-/Packverkehr, Erwerbsspeicher maximal 128 MiB, private Evidenz
  maximal 64 MiB. Quoten müssen während Empfang/Schreiben greifen.
- Gesamtlauf maximal 120 s ab vor Client-/Supervisorstart; pro Netzoperation
  maximal 30 s, DNS/Connect/SSH- beziehungsweise TLS-Aufbau zusammen maximal
  10 s, vollständige HTTP-Köpfe maximal weitere 10 s, Export je Commit maximal
  5 s. Supervisorabrechnung umfasst Aufbau, Reads, Abbruch und Aufräumen;
  keine nachträgliche elapsed-Prüfung als harte Unterbrechung ausgeben.
- Isolierter Prozessbaum maximal 512 MiB RAM, kein Swap, 60 s kumulierte
  CPU-Zeit und höchstens 16 Prozesse; zusätzlich gesamte Verifier-Ausführung
  unter demselben Ressourcenrahmen. Git-Konfigurationscachelimits ersetzen
  keine OS-Grenzen. Exakte cgroup-/Quota-/Supervisor-Durchsetzung auf foobox
  vor Nutzung belegen, keine Verfügbarkeit hier behauptet.
- Zusätzlicher Traversalvertrag vor Allokation: Tiefe höchstens 128,
  Pfadlänge 4096 Byte, höchstens 4096 Treebesuche, 100000 Einträge und 32 MiB
  kanonische Treebytes über den gesamten Lauf. Vorhandene flatten-Grenzen
  wirken je Aufruf und teilweise nach Materialisierung; sie sind noch keine
  Umsetzung dieses gemeinsamen strikten Profils. REST-Anfragelimit kann
  früher greifen. Überschreitung bedeutet unvollständig/Fehler, nie Teil-PASS.

SSH-Wirezählung, Diskquota, kumulierte CPU- und präemptive Prozessbaumbeendigung
sind neue Anforderungen an den Supervisor, keine Eigenschaften von Command,
cat-file oder RawCommitFiles. Ein externer timeout-Aufruf allein genügt nicht.
Bei Timeout/OOM/Quota/Nonzero-Exit, Pipefehler oder nicht bestätigtem Reaping
keine Übernahme von Teilbytes und kein erfolgreicher Beleg. Nicht abbrechbares
Kernel-I/O bleibt außerhalb einer unbelegten Echtzeitgarantie: nach Ablauf
keine neue Arbeit, Isolationsraum gesperrt halten und Lauf als BLOCKED melden;
fehlenden Aufräumnachweis nicht in Erfolg umdeuten.

Jede ID-/OID-/Typ-/Hash-/Metadatenabweichung, fehlende Originaldatei,
truncated Tree, fehlender Nebenblob oder nicht unterstütztes Commitprofil
bricht geschlossen ab. R0!=R1 oder ein Widerspruch zwischen Git und REST
wird mit beiden Beobachtungen und Reihenfolge erhalten: kein Mehrheitsentscheid,
kein „neuerer Wert gewinnt“, kein automatisches Re-Pinning und kein Retry.
Ein späterer Lauf braucht einen neuen ausdrücklich freigegebenen Auftrag.
R0=R1 beweist nur zwei gleiche Endpunktbeobachtungen; zwischenzeitliches
Wechseln mit Rückkehr (ABA) und ein atomarer Remote-Snapshot sind unbewiesen.
OID-gebundene Objekte bleiben davon getrennte Inhaltsbelege.

### Endlicher Implementierungsplan mit Akzeptanzkriterien

1. **Verträge festschreiben:** ReadbackSpec, Erwerbsmanifest und typisierten
   Livebeleg sowie RunBudget implementieren; bestehende Offlineverträge nicht
   ändern. Akzeptanz: fehlende Pins/Credentials, zusätzliche Ziele, ungültige
   OIDs und Modusverwechslung scheitern vor Netz/Spawn; kein caller-supplied
   Herkunftsfeld kann einen Livebeleg erzeugen. Limits entsprechen obigem Profil.
2. **Sicherheitsgates schließen:** Qualifizierten Linux-Supervisor und
   unveränderliche Exportübergabe bereitstellen; erforderlichen HTTP-Patch
   separat umsetzen. Akzeptanz: übergroße Köpfe scheitern vor der verbotenen
   Parserallokation; Budget-/Phasentests schließen Übergabelücken. Synthetische
   Prozesse mit blockierter Pipe, Nachkommen, Überausgabe, Nonzero-Exit,
   Pack-/Deltalast und vergifteter Konfiguration zeigen wirksame Grenzen,
   keinen Fremdpfadzugriff, keine Folgeanfrage und bestätigte Beendigung.
   Scheitert ein Nachweis, hier stoppen; kein Live-Einsatz und kein Ersatzweg.
3. **Git-Erheber und Export anbinden:** Festes SSH-/OID-Profil, isolierter
   Erwerb, netzfreier Export und RawCommitFiles-Übergabe implementieren.
   Akzeptanz an ausschließlich synthetischen lokalen Git-/SSH-Fixtures:
   Originalbytes samt Nicht-UTF8 und Schluss-LF-Varianten bleiben identisch;
   Typ, OID und unabhängiger SHA256 stimmen. Falscher Hostschlüssel/Repo,
   missing, Übergröße, Alternates/Replace/Promisor, Pfadaustausch und ungültige
   Ausgabe liefern keinen fertigen Export. Keine impliziten Credentials.
4. **Rust-Orchestrierung und Gesamtprüfung:** Bestehende Snapshot-/Transitions-
   prüfer verwenden; alle Nebenblobs ergänzen, äußere Repo-/Refklammer und
   getrennten Livebeleg einbauen. Akzeptanz: manipulierte Nebenblobs, fehlende
   Bäume, Gitlinks, ID-/Refkonflikte und Budgeterschöpfung ergeben keinen
   Gesamtbeleg; Snapshot/Transition bleiben verschieden. Alle erfolgreichen
   Fälle lassen Payload, UNKNOWN/Review, last_verified und Publish-Sperre
   unverändert. Danach freigegebene Offline-/Locked-Qualität und Regression
   einschließlich der bestehenden 131 Tests und 36 TLS-Fälle; neue Fälle
   separat zählen, keine Zahl bestandener Tests vorwegnehmen. Keine neue
   Pythonimplementierung; bestehende Entwicklungsoracles sind kein Transport.
5. **Genau eine extern freigegebene Erhebung:** Nach Abnahme von 1–4 und
   Bestätigung der GitHub-Verträge genau einen begrenzten privaten Lauf
   durchführen. Akzeptanz: unabhängige Repo-/Hostpins, echte authentifizierte
   Originalcommits, vollständiges Objektinventar, zweite SHA256-Prüfung,
   R0/R1, vollständige private Erwerbs-/REST-Belege und unabhängige Prüfung
   stimmen überein. Fehlt der Statusindex, endet dies als nachgewiesener
   fehlender Readback; keinen Testindex veröffentlichen. Kein zweiter Versuch
   aus einem Fehlschlag ableiten. Praktische Lesbarkeit über eine spätere
   Chat-Anbindung wird durch diesen foobox-Lauf nicht mitqualifiziert.

Dieser Plan endet mit dem begrenzten privaten Readbacknachweis oder einem
präzisen BLOCKED-/Fehlerbefund. Keine anschließenden spekulativen Untersuchungs-
oder Veröffentlichungsphasen sind Bestandteil dieses Auftrags.

### Ausdrücklich erforderliche zukünftige Freigaben

- Rust-Implementierungsauftrag für die genannten Komponenten und zusätzliche
  lokale Tests einschließlich synthetischer Git-/SSH-Prozesse, Loopback,
  Testdateien, privater Export-/Evidenzablage und externem Cargo-Target.
- Gesonderter Auftrag für HTTP-/Dependencypatch oder nötige neue Dependencies,
  Änderungen an Cargo.toml/Cargo.lock, geprüfte Quellbeschaffung und Build-/
  Testausführung. Dieser Plan trifft keine pauschale Dependencyfreigabe.
- Gesonderte Hostfreigabe für Supervisor, cgroups/Quoten/Namespaces und
  unveränderliche Dateisystembereitstellung; keine Hostkonfiguration ändern,
  bevor der konkrete Mechanismus und sein Umfang freigegeben sind.
- Explizite Freigabe zur Prüfung aktueller offizieller GitHub-/SSH-Verträge
  beziehungsweise Bereitstellung dieser Primärquellen. Bestätigung der IDs,
  Hostschlüsselprovenienz, Credentialrechte und des OID-/Depth-Verhaltens.
- Konkrete Netzwerk-/Credentialfreigabe für genau einen privaten Lauf:
  REST api.github.com:443 sowie SSH github.com:22 zum festen Repository,
  notwendiges DNS, read-only Token/Deploy-Key, isolierter lokaler Fetch samt
  dessen Schreibbereich und geschützte Originalbelege. Keine bestehenden
  Repositories verändern; Import/Export, Hashprüfung und Aufbewahrung müssen
  ausdrücklich umfasst sein. Keine heutige Freigabe dafür vorhanden.
- Veröffentlichung, Upload eines Statusindex, Push/API-Mutation,
  Runnerintegration und jede last_verified-Promotion benötigen unabhängige
  spätere Einzelaufträge. Sie bleiben auch nach erfolgreichem Readback
  **BLOCKED** und gehören nicht zum hier geplanten Erhebungslauf.

CHECKPOINT=NOVENA-BRIDGE-RUST-292-CODEX;
RESULT=ARCHITECTURE_PLAN_DOCUMENTED;
PREFERRED_ARCHITECTURE=ISOLATED_SSH_GIT_ACQUISITION_PLUS_RUST_REST_READBACK;
BASELINE=131_RUST_TESTS_36_LOCAL_TLS_TESTS_PASS_USER_VERIFIED;
LOCAL_FULL_READBACK=QUALIFIED_BASELINE;
NEW_IMPLEMENTATION=NO; NEW_BUILDS_OR_TESTS=NO; PYTHON_IMPLEMENTATION=NO;
LIVE_REQUESTS=NO; GITHUB_API_ACCESS=NO; CLONE_OR_FETCH=NO;
GIT_MUTATIONS=NO; CARGO_DEPENDENCY_CHANGES=NO;
AUTHENTICATED_RAW_COMMIT_ACQUISITION=BLOCKED_PENDING_QUALIFICATION;
HEADER_MEMORY_BOUNDARY=BLOCKED; LIVE_PRIVATE_READBACK=NOT_VERIFIED;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

## NOVENA-BRIDGE-RUST-293-CODEX

Datum: 2026-10-26. Auftrag/Arbeitsblock: NOVENA-BRIDGE-RUST-293.
Ergebnis: Rust-Implementierung und deterministische Integrationstests ergänzt;
**Kompilierung, Tests und Laufzeitqualifikation ausstehend beim äußeren Runner.**
Keine neuen bestandenen Tests behauptet. Der vom Auftrag bestätigte historische
Ausgangsstand bleibt 131 Rust-Tests und 36 lokale TLS-Tests; diese Zahlen sind
kein Prüfergebnis des hier geänderten Quellstands. Alte Checkpoints erhalten.

### Grundlage und Änderungsumfang

Grundlage sind AGENTS.md, NOVENA-ARBEITSREGELN.txt, Repository-/Bridge-README,
Bridge-Plan samt aktuellen Nachträgen, Projekt-/Test-/Entscheidungsverweise,
Checkpoint 292 sowie die vorhandenen Objekt-, REST-, Rohcommit-, Status- und
Testverträge. Die lokale Suche unter test-logs/, evidence/ und research/
lieferte weiterhin keine passenden Bridge-Rohprotokolle; keine Primärausgaben
rekonstruiert oder erfunden. Keine externen Quellen beschafft.

Geändert wurden ausschließlich `src/lib.rs`, `src/protocol.rs`, neu
`src/offline_readback.rs`, `tests/protocol.rs` und dieser Nachtrag.
Die zuvor vorhandenen Nutzeränderungen und unversionierten Dateien bleiben
erhalten. Cargo.toml, Cargo.lock und Git-Index haben nach der Bearbeitung
dieselben SHA256-Werte wie bei der Eingangsinventur. Keine Git-Mutation.
Direktes rustfmt wurde für die bearbeiteten Rust-Dateien ausgeführt; das
belegt weder Typprüfung noch erfolgreiche Kompilierung oder Testausführung.
Kein Cargo-Aufruf, kein äußerer Runner, keine neuen Dependencies, keine
Netzwerk-/SSH-/Credential-/Hardware-/Medienaktion und kein Image-Build.

### Implementierte Offline-Verträge

- `ReadbackSpec`: privater, nur durch geprüften Konstruktor erzeugbarer
  Laufvertrag mit festem Repository `loomit77/project-block-logs`, festem
  `refs/heads/main`, explizitem nichtleerem node_id-Pin und positiver numerischer
  ID, typisierter Commit-OID und getrenntem Snapshot-/Transitionsmodus.
  Transition enthält Baseline-OID sowie geborgte exakte alte/neue Bytes;
  gleiche C/B und Payloads über 1 MiB werden abgewiesen. Snapshot enthält
  geborgte erwartete Bytes. Dies ist noch kein ausführender Gesamtverifier
  und keine Statusvalidierung oder Bestätigung der Pinherkunft.
- Fester REST-Ursprung `https://api.github.com:443`, Wiederverwendung der
  bestehenden Endpunkt-Allowlist, nur als Zukunftsvertrag der feste SSH-Git-
  Zielname aus 292. Keine freien Hosts, Git-Verzeichnisse, Credentialfelder,
  Discovery oder Prozessstarts. Sämtliche OIDs benutzen den vorhandenen
  strikten Parser für genau 40 kleine Hexziffern.
- `RunScope` besitzt ausschließlich `Offline`. Eine LIVE-Variante, ein
  LIVE-Konstruktor und neue CLI-/Transportaktivierung existieren nicht.
  Fremde Behauptungen können keinen authentifizierten Laufbeleg erzeugen.
- `AcquisitionManifest` bewahrt geborgte Originalbytes und höchstens zwei
  eindeutige typisierte `ExportClaim`-Einträge mit OID, Nutzbytelänge und
  SHA256. Klassifikation stets `UNTRUSTED_ACQUISITION_CLAIM`, auch bei einer
  LIVE-Behauptung in den Originalbytes. Die Originalbytes werden nicht als
  authentifiziertes Manifest interpretiert. Einzelclaim höchstens 1 MiB,
  Originalmanifest höchstens 64 MiB, keine eigene Materialisierung.
  Manifestfelder, Selbsthash oder gleiche OIDs belegen keine Exporteuridentität.
- `RunBudget` ist ein gemeinsamer, nicht klonbarer Kontext mit geborgten
  Quellenhüllen für REST und Rohcommits. Neue Reader erhalten keinen neuen
  gemeinsamen Zähler. Quellen-/Budgetfehler und Fehler der vollständigen
  Blobprüfung sperren diesen Kontext. Fehler separat aufgerufener bestehender
  Verifier muss der Aufrufer weiterhin selbst als endgültigen Abbruch behandeln.

### Vollständige Nebenblobprüfung und unveränderte Verifier

`verify_complete_blobs` erstellt einen **getrennten** Offline-Vollständigkeits-
beleg über genau die explizit übergebenen ein oder zwei Root-Trees. Leeres
oder größeres Rootinventar und eine fremde Budgetzuordnung werden abgewiesen.
Der Beleg enthält die geprüften Root-OIDs, die Zahl der Blobpfadreferenzen
und für jede eindeutige Blob-OID Nutzbytelänge und SHA256 der unveränderten
Bytes. Seine Felder sind privat; es gibt keine Publikations- oder Status-API.
Er belegt allein die Vollständigkeit dieser Trees, keine vollständige
Historie und keine eigenständig bestätigte Auswahl der Commit-Trees.

Jeder nichtrekursive Tree läuft durch den echten RestReader und die bestehenden
Mode-/Namens-/Duplikat-/Git-Serialisierungs-/OID-Prüfungen. Fehlende Antworten,
abgeschnittenes JSON, truncated=true, ausgelassene Einträge, falsche OIDs,
Modi oder Typen verhindern den Gesamtbeleg. Jeder tatsächlich referenzierte
Blob wird über `blob_bounded` mit vorhandener strikter Base64-, Größen- und
Git-Blob-OID-Prüfung gelesen. Gleiche OIDs an mehreren Pfaden beziehungsweise
in beiden Roots benötigen innerhalb dieses Durchlaufs nur einen Blobabruf;
jede Pfadreferenz bleibt gezählt. Es werden keine vollständigen Blobinhalte
im Inventar gesammelt, sondern jeweils nur ein Blob verarbeitet und danach
Länge/SHA256 aufbewahrt. Ein Fehler gibt keinen Teilbeleg zurück.

Reguläre Dateien und ausführbare Dateien werden geprüft. Symlinks bleiben
Symlinks: ausschließlich ihre Blobbytes werden gelesen, niemals ein lokales
oder entferntes Linkziel verfolgt. Gitlinks/Submodule führen im neuen
Vollständigkeitsprofil zu `GITLINK_UNSUPPORTED`, ohne Fremdcommit-/Submodul-
erwerb. Sie werden nicht zu Blobs oder regulären Dateien umgedeutet.

`verify_initial_index`, `verify_update_index`, ihre Transitionslogik,
ReadbackReport und die bestehende Snapshot-Attestation behalten ihren
Ergebnisvertrag und ihre bisherigen Aussagen. Insbesondere bestätigt ein
alter ReadbackReport weiterhin nicht automatisch sämtliche Nebenblobbytes.
`flatten_verified` und die Status-/Publikationssperren wurden nicht geändert.
Gemeinsame RestReader-Eingänge erhielten zusätzliche begrenzte Varianten;
die bisherigen blob-/tree-Eingänge bleiben verfügbar. Strengere Größen- und
Eintragsvorprüfungen können fehlerhafte Eingänge früher zurückweisen, ohne
deren Fehler in Erfolg umzudeuten.

### Tatsächlich implementierte Grenzen und ihre Aussagegrenzen

Die gemeinsamen Quellenhüllen begrenzen vor Weitergabe an den JSON-/Commit-
Parser auf 256 REST-Versuche über Readerwechsel, 8 MiB je REST-Body,
32 MiB REST-Bodies einschließlich Rohcommit-Nutzbytes zusammen, höchstens
zwei Rohcommitabrufe, 1 MiB je Rohcommit und 2 MiB Rohcommits insgesamt.
Die REST-Summe ist damit ebenfalls höchstens 32 MiB. Ein ausgeschöpfter
Versuchs-/Gesamtbytezähler verhindert den nächsten Quellenaufruf; Fehler
werden nicht durch neue Reader oder neue Quellenhüllen zurückgesetzt.

Der neue Tree-Durchlauf begrenzt gemeinsam über alle übergebenen Roots und
weitere Durchläufe mit demselben Kontext: Tiefe 128, Pfadlänge 4096 Byte,
4096 Treebesuche, 100000 Einträge und 32 MiB kanonische Treebytes.
`tree_bounded` prüft Eintragszahl, Namenslängen und exakte kanonische Bytezahl
vor zusätzlichem Kopieren der Einträge/Namen und vor Git-Serialisierung.
Pfadsummen werden vor dem Anlegen zusammengesetzter Pfade geprüft.
`blob_bounded` prüft deklarierte Größe, Base64-Zeichenzahl und Paddinglänge
vor Decoder-Materialisierung; pro Blob höchstens 1 MiB, dekodierte Blobbytes
kumulativ höchstens 32 MiB im Kontext. Vor Inventareinträgen gilt zusätzlich
höchstens 256 eindeutige Blobnachweise je Durchlauf. REST-Limits greifen
gegebenenfalls früher. Gemeinsam benutzte Subtrees zählen bei jedem Besuch.

**Konkreter verbleibender Allokationsblocker:** ResponseSource und
RawCommitSource liefern bereits allokierte Vecs. Die neue Hülle kann die
vorgelagerte Quellenallokation oder den Empfang nicht präemptiv begrenzen.
Der JSON-Wertebaum existiert unter den REST-Bodygrenzen bereits vor der
Treevorprüfung; diese ist keine path-/entryweise Grenze vor jeder internen
JSON-Parserallokation und keine harte maximale Prozess-RAM-Garantie.
Für streng während Empfang/Export erzwungene gemeinsame Bytegrenzen braucht
es künftig einen begrenzten Streaming-/Übergabevertrag plus Supervisor.
Ebenso betreffen die neuen Traversalzähler den Zusatzdurchlauf, nicht
rückwirkend die gesonderten alten flatten-Aufrufe eines Initial-/Update-
Verifiers. Deren Integration in einen einzigen durchgehend budgetierten
Gesamtlauf bleibt offen. Kein vollständig durchgesetztes 292-Laufprofil behauptet.

`SupervisorLimits` hält die übrigen festen 292-Obergrenzen als ausdrücklich
**nicht durchgesetzten Typvertrag** fest: eine SSH-Sitzung, 64 MiB SSH-Eingang,
128 MiB Erwerbsspeicher, 64 MiB Evidenz, 120 s Gesamtlauf, 30 s Netzoperation,
10 s Verbindungsaufbau, weitere 10 s Köpfe, 5 s Export je Commit,
512 MiB RAM, kein Swap, 60 s kumulierte CPU, 16 Prozesse. Keine OS-Quoten,
cgroups, präemptiven Netzdeadlines oder Prozessbeendigung implementiert.

### Neue deterministische Rust-Integrationstests

Elf neue `#[test]`-Funktionen in `tests/protocol.rs`, mit Varianten innerhalb
einzelner Tests. Alle verwenden vorhandene synthetische Git-Fixtures und/oder
die realen REST-/Objektprüfer, keine vorgegebenen Verifier-Ergebnisattrappen:

1. Vollständiger initialer Readback über originale synthetische Commitbytes,
   gemeinsame REST-/Rohcommitbudgets und anschließende beide Treeinventare:
   zwei eindeutige Blobs, drei Pfadreferenzen, Nebenblob-SHA256/Länge,
   unveränderte Payloadbytes und last_verified=null, NOT_PUBLISHED und
   unverändert gesperrte production_publish-Funktion.
2. Manipuliertes beziehungsweise fehlendes Nebenblob; kein Teilbeleg und
   keine Fortsetzung desselben fehlgeschlagenen Kontexts.
3. Gleiche Blob-OID an regulärem, ausführbarem und Symlinkpfad: drei
   Referenzen, ein Blobabruf, kein Verfolgen des enthaltenen relativen Ziels.
4. Sieben Tree-Negativvarianten: falscher Typ, truncated=true, ausgelassener
   Eintrag, doppelter Name, ungültiger Modus, fehlender Tree und Gitlink.
5. Readerwechsel mit 31 MiB REST-Bodies plus genau 1 MiB gültigen Rohcommit-
   Nutzbytes; nächster Abruf und jede Fortsetzung werden abgewiesen.
6. Gemeinsame 256-Anfragen-Grenze über neue Reader hinweg; der folgende
   echte Vollständigkeitsdurchlauf erhält keinen Erfolg.
7. Gesamtbodybudget während echter Tree-/Blobprüfung: vier gültige auf
   jeweils 8 MiB aufgefüllte Antworten, danach Abbruch ohne Teilbeleg.
8. Abgeschnittenes Tree-JSON und anderer Budgetkontext; letzterer wird vor
   dem ersten Quellenaufruf erkannt.
9. Treeeintrags- und Treebytegrenze vor zusätzlicher Materialisierung;
   Nebenblob mit acht Byte Restbudget abgewiesen, mit genau neun angenommen.
10. 130 verschachtelte synthetische Trees sowie zwei je 3000 Byte lange
    Verzeichnisnamen: Tiefen- beziehungsweise Pfadsummenabbruch.
11. Typisierter Offline-Spec, feste Identität/Ref, strikte OID-Syntax,
    Ablehnung freier HTTP-Ziele, ungültiger Pins und leerer Exportlisten;
    LIVE-Text im unbeglaubigten Manifest erzeugt keine LIVE-Klassifikation.

Diese Fälle sind implementiert und quelltextlich durchgesehen, **nicht
ausgeführt**. Kein neuer TLS-Test und kein GitHub-Live-Test qualifiziert.

### Offene Herkunfts- und Sicherheitsgrenzen; nächster Schritt

Privater GitHub-Live-Readback, SSH-/Git-Prozesserwerb, echte Credentials,
Veröffentlichung und last_verified-Promotion bleiben gesperrt.
**Die 32-KiB-Headergrenze vor Parserallokation bleibt BLOCKED.**
Host-Supervisor, cgroup-/Disk-/CPU-/RAMquoten, unveränderliche Exportübergabe,
unabhängig bestätigte Pinherkunft, authentifizierte Provenienz und eine
unabhängige SHA256-Gegenprüfung sind nicht qualifiziert. Manifestclaims,
SHA256 und Git-OID ersetzen diese Nachweise nicht. Numerische ID/private
und abschließende Repository-/Refprüfung um den gesamten Zusatzdurchlauf
sind noch nicht als gemeinsamer Ablauf verdrahtet.

Nächster konkreter Schritt: Der äußere Runner kompiliert und qualifiziert
diesen Quellstand offline einschließlich der elf neuen Tests und der
vorhandenen Regressionen. Danach ein gesonderter Rust-Auftrag für einen
einzigen Offline-Orchestrator, der ReadbackSpec tatsächlich konsumiert,
Originalcommit-/Manifestrelationen prüft, daraus zwingend C/B-Roots ableitet,
alle Traversierungen unter demselben Budget ausführt und die vollständige
Objektprüfung mit Repository-/Ref-Vor-/Nachprüfung einklammert. Dabei die
bisherigen inneren Berichte erhalten; kein LIVE-Etikett und keine Publikation.
Erst separat die weiter blockierten Transport-/Supervisorfragen bearbeiten.

CHECKPOINT=NOVENA-BRIDGE-RUST-293-CODEX;
RESULT=RUST_IMPLEMENTATION_PENDING_EXTERNAL_QUALIFICATION;
NEW_RUST_INTEGRATION_TESTS=11_NOT_EXECUTED;
FULL_BLOB_VERIFICATION=IMPLEMENTED_OFFLINE_SEPARATE_SCOPE;
BASELINE=131_RUST_TESTS_36_LOCAL_TLS_TESTS_PASS_USER_VERIFIED;
NEW_BUILDS_OR_TESTS=NO; CARGO_DEPENDENCY_CHANGES=NO;
LIVE_REQUESTS=NO; SSH_FETCH=NO; REAL_CREDENTIALS=NO;
SUPERVISOR_AND_IMMUTABLE_EXPORT=NOT_QUALIFIED;
AUTHENTICATED_PROVENANCE=NOT_QUALIFIED; HEADER_MEMORY_BOUNDARY=BLOCKED;
LIVE_PRIVATE_READBACK=NOT_VERIFIED;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-293 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 131 Rust-Tests und
36 lokale TLS-Tests aus Block 291.

Auftrag: typisierte Readback-Laufverträge,
gemeinsame Budgetgrundlage und Prüfung
sämtlicher relevanten Nebenblobs.

Äußere Rust-Prüfungen:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

Rust-Qualitätsstatus: PASS

Die konkret implementierten Komponenten und
Testfälle sind anhand des Codex-Checkpoints
und des Originalprotokolls zu bewerten.

Ein Offline-PASS ist kein Nachweis einer
authentifizierten privaten GitHub-Erhebung.

Die vollständige Headerbytegrenze vor
Parserallokation bleibt BLOCKED.

SSH-Erhebung und Host-Supervisor bleiben
nicht freigegeben.

BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED;
GITHUB_LIVE_READBACK=NOT_EXECUTED.

## NOVENA-BRIDGE-RUST-294-CODEX

Datum: 2026-10-26. Implementierung ergänzt; Kompilierung und Laufzeitprüfung
bleiben dem äußeren Runner vorbehalten. Ausgangsstand laut Auftrag:
142 bestandene Rust-Tests, darunter 36 lokale TLS-Tests. Keine neue
Bestanden-Zahl für diesen Quellstand behauptet. Frühere Checkpoints erhalten.

### Ist-Zustand und tatsächlicher Datenfluss

Die bisherige lokale TLS-Kette verwendete HttpsGetSource -> RestReader ->
verify_initial_index -> check_commit mit RawCommitFiles -> flatten_verified
für beide Trees -> Zielblobprüfung -> abschließende Refprüfung ->
ReadbackReport. RawCommitFiles ist dabei eine separate Quelle für originale
Commit-Nutzbytes, kein nachgeschalteter REST-Parser. Ein gewöhnlicher
ReadbackReport belegt nicht die Nutzbytes sämtlicher Nebenblobs.

293 ergänzte verify_complete_blobs und RunBudget getrennt davon; die
TLS-Readbacktests nutzten diese Verbindung bisher nicht. HttpsGetSource zählt
Empfangsbytes/Versuche und Zeit selbst, RestReader zählt Bodies/Anfragen pro
Reader, flatten_verified zählt Traversierungen pro Aufruf. Diese Zähler sind
keine gemeinsame Abrechnung von REST und Rohcommitdateien.

Neu ist verify_complete_readback mit ausschließlich budgetierten Quellen:
HttpsGetSource -> RunBudget.responses -> RestReader, parallel RawCommitFiles
-> RunBudget.commits. Repository-/Refvorprüfung, bestehende
verify_transition_objects-Prüfung, vollständige Blobprüfung über deren beide
aus verifizierten Originalcommits abgeleitete Roots und abschließende
Refprüfung benutzen denselben Kontext. Roots werden nicht frei vom Aufrufer
für diesen Gesamtbeleg ausgewählt. Keine zusätzlichen Rohcommitabrufe.

Der gemeinsame Transitionskern gibt intern zusätzlich seine geprüften Roots
zurück; die bisherigen Initial-/Update-APIs behalten ihre Signaturen und
Berichtssemantik. baseline=None bedeutet weiterhin initiale Hinzufügung,
Some(exakte alte Bytes) weiterhin Ersetzung mit bestehender Status-, Parent-,
Pfad-, Modus- und Byteprüfung. Kein neuer HTTP-Client oder Objektverifier.
ReadbackSpec/AcquisitionManifest werden hier noch nicht konsumiert.

### Ergebnisarten und Fehlergrenzen

- Gewöhnlicher Readback: bestehender ReadbackReport, unveränderte Aussage.
- Vollständige Blobprüfung: bestehender CompleteBlobs-Beleg über seine Roots.
- Vollständiger integrierter Readback: neuer CompleteReadback mit privaten
  Feldern, nur nach beiden Prüfungen und erfolgreicher letzter Refprüfung.
- Unvollständige Evidenz: Err, kein Teilbeleg; jeder Fehler im neuen Ablauf
  sperrt den gemeinsamen Kontext. Fremde Budgetzuordnung wird vor I/O erkannt.
- Authentifizierte Live-Herkunft: weiterhin nicht konstruierbar;
  RunScope ausschließlich Offline. Lokales TLS und lokale Originaldateien
  erhalten keine GitHub-Herkunftsklassifikation.

### Gemeinsame Budgets und weiterhin getrennte Grenzen

RunBudget erzwingt im integrierten Pfad über Vorprüfung, Transitionsprüfung,
Zusatzdurchlauf und Nachprüfung insgesamt höchstens 256 REST-Versuche,
8 MiB je Body, 32 MiB REST-Bodies plus Rohcommit-Nutzbytes, zwei Rohcommits,
1 MiB je Rohcommit und 2 MiB Rohcommits insgesamt. Bei bereits ausgeschöpftem
Anfragen-/Gesamtbytebudget erfolgt kein weiterer Quellenaufruf. Quellen- und
Verifierfehler erlauben keine Fortsetzung mit einem neuen Reader im Kontext.

Die vollständige Blobprüfung behält ihre kontextweiten Grenzen: Tiefe 128,
Pfadlänge 4096 Byte, 4096 Treebesuche, 100000 Einträge, 32 MiB kanonische
Treebytes, 1 MiB je Blob, 32 MiB dekodierte Blobbytes und 256 eindeutige
Blobbelege je Durchlauf. Gleiche Blob-OIDs in beiden Roots werden in diesem
Durchlauf einmal gelesen; Pfadreferenzen bleiben einzeln gezählt. Der
gewöhnliche Readback liest den Zielblob zusätzlich; kein laufweiter Blobcache
eingeführt und keine neue Cache-/Frischesemantik behauptet.

Die beiden alten flatten_verified-Aufrufe behalten eigene Traversalzähler.
Damit sind Treebesuche/Einträge/Treebytes noch nicht über ALLE Traversierungen
gemeinsam begrenzt. HTTPS-Empfangsbudget bleibt ebenfalls separat: RunBudget
prüft kombinierte Bytebudgets erst nach Rückgabe des bereits empfangenen Vec,
vor JSON-Verarbeitung. Es begrenzt nicht den nächsten Body während Empfang
auf den um Rohcommitbytes verminderten Rest. JSON-Allokation, Prozess-RAM und
vollständige präemptive Gesamtlaufzeit werden dadurch nicht garantiert.

### Neue lokale TLS-Tests

Sieben neue Tests im vorhandenen cfg(test)-TLS-Integrationsmodul, sämtlich
mit Präfix local_tls_, vorhandenem OpenSSL-Loopbackserver und unverändertem
HttpsGetSource-Handshake-/GET-Pfad; keine In-Memory-HTTP-Quelle als TLS-Beleg:

1. complete_readback_multiple_blobs_raw_files: zwei vollständige Commit-Trees,
   zwei eindeutige Blobs, drei Pfadreferenzen, originale Commitdateien ohne
   Speicherfallback; echte GETs für alle Blobs und Offline-Gesamtbeleg.
2. complete_readback_tampered_side_blob: manipulierte Nebenblobbytes scheitern
   am vorhandenen Git-Blobhashprüfer; Folgeaufruf sendet nichts.
3. complete_readback_missing_side_blob: echte 404-Antwort, kein Gesamtbeleg,
   keine Fortsetzung oder automatische Wiederholung.
4. complete_readback_repeated_oid_single_side_get: dieselbe Nebenblob-OID in
   beiden Roots erzeugt genau einen GET, Zielblob insgesamt zwei GETs.
5. shared_budget_exhaustion_prevents_next_get: vier echte GET-Antworten zu je
   8 MiB erschöpfen gemeinsam 32 MiB; neuer RestReader mit demselben Kontext
   startet keine fünfte Anfrage. Erneuter Aufruf bleibt ohne Anfrage gesperrt.
   Nur dieser Test gibt dem separaten ReceiveBudget ein zusätzliches Byte
   für dessen erforderliche EOF-Probe, damit tatsächlich RunBudget und nicht
   das konservative Empfangsende den Folgeabruf sperrt. Produktionslimits
   werden nicht verändert.
6. complete_readback_final_ref_changed: vollständige Blobprüfung genügt bei
   abschließend abweichendem Ref nicht für einen Gesamtbeleg.
7. complete_readback_incomplete_tree_no_evidence: truncated Tree führt über
   den bestehenden Treeprüfer zu Fehler, ohne Vollständigkeitsbeleg.

Alle neuen Fälle prüfen unveränderte Payloadbytes, last_verified=null,
NOT_PUBLISHED und die gesperrte production_publish-Funktion. Erfolgsfälle
prüfen Offline-Klassifikation, publication_performed=false und gezählte echte
Blobrequests. Ausschließlich synthetische Daten und Loopback; neue Aufrufe
ohne Authorization. Tests implementiert, hier ausdrücklich nicht ausgeführt.

Geänderte Rust-Dateien: src/protocol.rs, src/offline_readback.rs und
src/https_local_tls.rs. Zusätzlich ausschließlich dieser Dokumentnachtrag.
Direktes rustfmt ausgeführt, keine Cargo-Builds, Cargo-Tests oder andere
Laufzeitqualifikation. Nutzeränderungen erhalten. Keine neuen Dependencies,
externen Verbindungen, SSH-/GitHub-Anfragen, Git-Mutationen, Hardwareaktionen,
Veröffentlichung oder last_verified-Promotion. Die lokale Dateinamensuche
unter test-logs/, evidence/ und research/ ergab keine Bridge-Rohprotokolle;
äußere Qualifikationsmeldungen und Auftragszahlen ersetzen keine Primärausgaben.

### Konkrete Blocker und nächster Schritt

32-KiB-Headergrenze vor Parserallokation weiterhin BLOCKED. Linux-Supervisor,
cgroup-/CPU-/RAM-/Diskgrenzen, manipulationssichere Rohdatenübergabe,
authentifizierte GitHub-Provenienz und vollständige präemptive Gesamtdeadline
bleiben unqualifiziert. Numerische Repository-ID/private, unabhängige Pins,
Manifest-/ReadbackSpec-Bindung und erneute Repositoryprüfung am Ende fehlen
weiterhin als gemeinsamer authentifizierter Vertrag. Kein atomarer Remote-
Snapshot und kein Schutz gegen zwischenzeitlichen Refwechsel mit Rückkehr.

Als Nächstes qualifiziert ausschließlich der äußere Runner diesen Stand mit
den bestehenden Regressionen und sieben neuen TLS-Fällen. Danach sinnvoll:
budgetierte Tree-Traversierung im vorhandenen Objektprüfer gemeinsam nutzen,
damit auch die bisherigen flatten-Aufrufe denselben Traversalkontext belasten;
dazu Update- und Budgetkontext-Negativfälle des neuen Gesamteingangs ergänzen.
Empfangsseitige kombinierte Restbudgets und die separaten Transport-/Supervisor-
Blocker benötigen eigene klar begrenzte Implementierung und Qualifikation.

CHECKPOINT=NOVENA-BRIDGE-RUST-294-CODEX;
RESULT=IMPLEMENTED_PENDING_EXTERNAL_QUALIFICATION;
BASELINE=142_RUST_TESTS_36_LOCAL_TLS_TESTS_USER_CONFIRMED;
NEW_LOCAL_TLS_TESTS=7_NOT_EXECUTED;
COMPLETE_READBACK=OFFLINE_ONLY;
HEADER_MEMORY_BOUNDARY=BLOCKED;
AUTHENTICATED_PROVENANCE=NOT_QUALIFIED;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-294 – Äußere Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 142 Rust-Tests,
darunter 36 lokale TLS-Tests.

Auftrag: vollständiger lokaler TLS-Readback
mit gemeinsamer Budget- und Nebenblobprüfung.

Prüfergebnisse:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

RUST_QUALITY=PASS

Ein Test-PASS ist keine authentifizierte
GitHub-Live-Erhebung.

HEADER_MEMORY_BOUNDARY=BLOCKED;
SSH_ACQUISITION=BLOCKED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.

## NOVENA-BRIDGE-RUST-295-CODEX

Datum: 2026-10-26. Ergebnis: implementiert, äußere Qualifikation ausstehend.
Ausgangsstand laut Auftrag: 149 bestandene Rust-Tests, darunter 43 lokale
TLS-Tests. Keine neuen bestandenen Tests behauptet; keine Cargo-Aufrufe.
Die Checkpoints 292, 293 und 294 samt ihren Aussagegrenzen bleiben erhalten.

### Evidenzvertrag und Anschluss an den vorhandenen Prüfer

Neues Rust-Modul `object_evidence`: `OfflineObjectEvidence` mit privaten
Feldern, ohne öffentlichen Konstruktor oder Deserialisierung. Zugriff über
`CompleteReadback::evidence()` erst nach erfolgreicher bestehender
Transitionsprüfung, vollständiger Nebenblobprüfung und letzter Refprüfung.
Kein zweiter Verifier, HTTP-Client, Erwerbsadapter oder CLI-Ausgabemodus.
Der bestehende vollständige TLS-Readback erhält damit denselben Evidenzpfad.

Das Manifest `NOVENA_OFFLINE_OBJECT_EVIDENCE_V1` bindet festes Repository und
Ref, den geprüften node_id-Erwartungswert, Baselinecommit sowie initial und
abschließend beobachteten Head. Die Baseline ist kein beobachteter Refstand.
Die Pinherkunft ist nicht beglaubigt; numerische Repository-ID/private und
erneute Repositoryprüfung am Ende bleiben wie in 294 offen.

Inventaridentität ist (Objekttyp, Git-OID), Reihenfolge Commit/Tree/Blob und
innerhalb jeder Klasse aufsteigende OID. Jede Identität erscheint einmal,
auch bei wiederholten Pfaden/Roots; es wird keine Zahl unabhängiger Erhebungen
behauptet. Gesamtzahl und vollständiges eindeutiges Inventar sind enthalten.
JSON verwendet die vorhandene kanonische Serialisierung: kompakt, rekursiv
lexikalische Schlüssel, genau ein abschließendes LF. Keine Uhrzeit oder
Zufallswerte im Kernmanifest. OID ist Git-SHA1 einschließlich Git-Objektheader;
das getrennte `payload_sha256` betrifft ausschließlich Nutzbytes ohne Header.

### Objektklassen, SHA256 und Byteprovenienz

- Commits: Die budgetierte Rohquelle bewahrt nach `inspect_commit` und
  unabhängiger Git-OID-Prüfung genau die zurückgegebenen Bytes. Der bestehende
  Prüfer gleicht deren Tree/Parents mit REST ab. Im Erfolgsmanifest stehen
  beide Original-Nutzbytefolgen verlustfrei als Base64 sowie OID, Typ, Länge
  und in Rust berechneter SHA256. REST-JSON wird nie als Rohcommit verwendet.
  `ORIGINAL_COMMIT_PAYLOAD` beschreibt diese Bytequelle, keine authentifizierte
  GitHub-Beschaffung. Das bestehende schmale unsigned Commitprofil bleibt.
- Trees: SHA256 wird direkt nach der bestehenden vollständigen
  Tree-/OID-Prüfung über `tree_bytes` berechnet, bevor dieser Puffer entfällt.
  Kennzeichnung ausschließlich `CANONICALLY_RECONSTRUCTED_TREE`. Keine
  Behauptung original übertragener Treebytes. Treeinventar ist OID-dedupliziert.
- Blobs: bestehendes `blob_bounded` prüft Base64, Länge und Git-OID;
  anschließend berechnet Rust SHA256 über die geprüften dekodierten Inhalte.
  Ziel- und Nebenblobs stehen als `DECODED_BLOB_PAYLOAD` im Inventar. Kein
  zusätzlicher Cache sämtlicher Blobinhalte und kein zusätzlicher Abruf.

Ungeprüfte AcquisitionManifest-Claims werden nicht konsumiert. Der Scope
bleibt typisiert ausschließlich Offline, im Manifest OFFLINE_SUPPLIED_RESPONSES;
Transportprovenienz bleibt NOT_AUTHENTICATED. Auch Nutzbytes mit einer
LIVE_PROVENANCE-Behauptung können diese festen Felder nicht verändern.

### Budgets und Fehlersemantik

Alle bisherigen RunBudget-Grenzen gelten unverändert: 256 REST-Versuche,
8 MiB je Body, zusammen 32 MiB REST- und Rohcommitbytes; maximal zwei
Rohcommitabrufe, je 1 MiB und insgesamt 2 MiB. Zusätzlich gehalten werden
höchstens diese 2 MiB geprüften Commitbytes, keine weiteren Rohabrufe.
Der vollständige Treepfad bleibt auf 4096 Besuche, Tiefe 128, 100000 Einträge,
32 MiB rekonstruierte Treebytes und 4096 Byte Pfadlänge begrenzt; damit auch
das neue Tree-Digestinventar auf höchstens 4096 Einträge. Blobgrenzen bleiben
256 eindeutige OIDs, 1 MiB je Blob und 32 MiB dekodierte Inhalte.

Die Serialisierung prüft zusätzlich `SupervisorLimits::EVIDENCE_BYTES`
(64 MiB), Fehler EVIDENCE_BUDGET_EXCEEDED. Diese Prüfung erfolgt nach der
Serialisierung, ist keine Empfangs-/Allocatorgrenze. Bereits die begrenzten
Inventare und höchstens 2 MiB Base64-Eingangsbytes verhindern eine neue
unbeschränkte Gesamtobjektmaterialisierung. Serialisierungszwischenpuffer
sind dennoch zusätzliche, nicht als Prozess-RAM-Limit qualifizierte Speicher.
Ein falsches Commitinventar ergibt EVIDENCE_COMMIT_INVENTORY_MISMATCH;
Serialisierungsfehler EVIDENCE_SERIALIZATION. Jeder Fehler im Gesamtpfad
vergiftet wie bisher den Kontext und liefert Err ohne Teil-Erfolgsmanifest.

### Neue Integrationstests und ausstehende Qualifikation

Drei gezielte Tests in `tests/protocol.rs`, ausschließlich vorhandene
synthetische Fixture und echter vollständiger `verify_complete_readback`:

1. Determinismus und Inventar: zwei vollständig geprüfte Commits, vier Trees,
   zwei Blobs einschließlich Nebenblob, fünf Blobpfadreferenzen. Prüft Typ,
   eindeutige Sortierung, Länge, SHA256 jeder Objektklasse, originale
   Commit-Base64 und zweimal identische Manifestbytes. Commitnachricht enthält
   Nicht-UTF8, mehrere LF und LIVE_PROVENANCE-Text; Scope bleibt Offline.
2. Ein zusätzliches Commit-LF ändert die geprüfte Commit-OID, SHA256 und
   Evidenzbytes. Keine Normalisierung der Originalnachricht.
3. Manipulierter Nebenblob, fehlender Nebenblob, fehlender Rohcommit und
   erschöpftes gemeinsames Anfragebudget verhindern Erfolg; jeder Fall prüft
   zusätzlich den gesperrten Folgeaufruf. Kein partielles Manifest zugänglich.

Alle Fälle prüfen unveränderte Payloadbytes, last_verified=null,
NOT_PUBLISHED und die unveränderte production_publish-Sperre. Bestehende
Tree-Unvollständigkeits-, Bytebudget- und TLS-Fehlerregressionen werden
wiederverwendet statt dupliziert. Die neuen Tests sind implementiert, nicht
ausgeführt. Sie qualifizieren weder reale Beschaffung noch Transportherkunft.
Direktes rustfmt ausgeführt; keine Kompilierung oder Laufzeitprüfung.

### Sicherheitsblocker und nächster konkreter Schritt

Das Manifest nennt ausdrücklich: keine authentifizierte GitHub-Herkunft,
keine atomare Remote-Snapshot-Konsistenz, keine spätere Ref-Erreichbarkeit,
keine sichere SSH-Erhebung, keine unveränderliche Dateisystemübergabe,
keine HTTP-Header-Speichergrenze und keine vollständige präemptive Deadline.
Keine Historienprüfung außerhalb der beiden tatsächlich geprüften Commits.
Headergrenze vor Allokation bleibt BLOCKED, Supervisor und manipulationssichere
Übergabe unqualifiziert. Die früheren getrennten flatten-Traversalbudgets und
empfangsseitigen kombinierten Restbudgetlücken aus 294 bleiben bestehen.
Publikation und last_verified-Promotion bleiben unverändert gesperrt.

Nächster Schritt: äußere Runnerqualifikation der drei Evidenztests sowie
bestehenden Rust-/TLS-Regressionen. Danach den in 294 benannten gemeinsamen
Traversalkontext in den vorhandenen Treeprüfer integrieren; keine Live-Erhebung
aus diesem Offline-PASS ableiten.

Grundlage: Arbeitsregeln, README, Projekt-/Test-/Entscheidungsdokumentation,
Bridge-Plan, Checkpoints 292–294 und relevante Rust-Quellen/Integrationstests.
Keine passenden Bridge-Rohprotokolle unter test-logs/, evidence/, research/
gefunden; Ausgangszahlen beruhen auf Auftrag und äußerem Checkpoint, keine
Primärausgaben rekonstruiert. Bestehende Nutzeränderungen erhalten.
Änderungen ausschließlich src/*.rs, tests/protocol.rs und dieser Nachtrag.
Keine Dependencyänderung, Netzwerk-/SSH-/Git-Mutation oder Hardwareaktion.

CHECKPOINT=NOVENA-BRIDGE-RUST-295-CODEX;
RESULT=IMPLEMENTED_PENDING_EXTERNAL_QUALIFICATION;
BASELINE=149_RUST_TESTS_43_LOCAL_TLS_TESTS_USER_CONFIRMED;
NEW_INTEGRATION_TESTS=3_NOT_EXECUTED;
OBJECT_EVIDENCE=OFFLINE_ONLY;
HEADER_MEMORY_BOUNDARY=BLOCKED;
AUTHENTICATED_PROVENANCE=NOT_QUALIFIED;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-295 – Äußere Rust-Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 149 Rust-Tests und
43 lokale TLS-Tests aus Block 294.

Auftrag:
Offline-Objektevidenz mit eindeutiger
Git-OID, Typ, Bytelänge und SHA256;
vollständige Inventur geprüfter
Commits, Trees und Blobs.

Äußere Prüfungen:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

RUST_QUALITY=PASS

Ein erfolgreicher lokaler Evidenztest
beweist keine authentifizierte
GitHub-Herkunft.

HEADER_MEMORY_BOUNDARY=BLOCKED;
SSH_ACQUISITION=BLOCKED;
HOST_SUPERVISOR=NOT_QUALIFIED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-296-CODEX

Datum: 2026-10-26. Ergebnis: **IMPLEMENTED_PENDING_EXTERNAL_QUALIFICATION**.
Keine Cargo-Builds, Cargo-Checks, Clippy- oder Testläufe durch Codex.
Direktes rustfmt wurde ausgeführt. Parsererfolg von rustfmt ist kein
Compiler-, Laufzeit- oder Sicherheitsnachweis. Sämtliche alten Checkpoints
bleiben erhalten. Architekturentscheidung 292 bleibt verbindlich:
isolierte SSH-Git-Erhebung plus unabhängiger Rust-REST-Readback.

### 1. Gesicherter Ausgangsstand

Ausgangsqualifikation laut Auftrag und äußerem Checkpoint 295: 152 bestandene
Rust-Tests, darunter 43 lokale TLS-Tests; Rustfmt, Cargo Check und Clippy PASS.
Diese Zahlen werden hier weder neu gemessen noch als Qualifikation des neuen
Codes fortgeschrieben. Ausgangsevidenz sind die vorhandenen äußeren
Zusammenfassungen; keine fehlenden Primärausgaben nachträglich rekonstruiert.

Git-Zustand vor Änderungen gelesen: vorhandene Nutzeränderungen in TODO,
CHAT-STARTER, DECISIONS, HANDOVER-LATEST und Bridge-Plan sowie unversionierte
Bridge und Pythonreferenzen. Diese Arbeiten bleiben erhalten. Der Git-Index
wurde ausschließlich gelesen, nicht aktualisiert. Lokale Arbeitsregeln,
Repository-/Bridge-Dokumentation, relevante Checkpoints 292–295, Manifest,
Rohcommitquelle, Status-, Objekt-, REST-, Budget- und TLS-Prüfer sowie ihre
bestehenden Fixtures bilden die Implementierungsgrundlage. Keine neue
Bewertung der abgeschlossenen Untersuchungen 286, 287 und 291.

Befund zu 295: V1 bewahrt zwei Originalcommits, enthält für Trees und Blobs
aber nur Inventardaten. Dieses Manifest allein kann keinen unabhängigen
Bytevollständigkeitsnachweis liefern. Seine Ausgabe bleibt unverändert;
zusätzliche Vorabgrenzen begrenzen nur bisher nicht zulässige Inventargrößen.

### 2. Neue Rust-Komponenten

Neues Modul `src/offline_package.rs`: `OfflinePackage`, `verify_package`,
`export_private`, `import_private`. `CompleteReadback::package()` liefert das
neue Paket erst nach dem vollständigen erfolgreichen bisherigen Prüflauf.
`RunBudget` bewahrt die tatsächlich zurückgegebenen REST-Bodies samt
Endpoint in monotoner Abrufreihenfolge zusätzlich auf. Rohcommitbytes stammen
weiterhin ausschließlich aus `BudgetedCommits::verified`.

Der Base64-Decoder in protocol.rs erhielt einen intern begrenzbaren Kern;
die bisherige öffentliche `decode_base64` behält ihre bisherigen Grenzen und
Fehlersemantik. Damit sind REST-Antwortbytes bis 8 MiB wieder einlesbar,
während Blobs und Commits weiterhin höchstens 1 MiB haben dürfen. Kein neuer
JSON-, Git-Tree- oder Vollständigkeitsparser; keine neue Dependency.

### 3. Daten- und Sicherheitsverträge

Neuer eigenständiger Vertrag **NOVENA_OFFLINE_REPLAY_V1**. V1 bedeutet genau
folgendes Format UND den vollständigen Initial-/Update-Transitionsprüfumfang:

- Genau `contract`, `scope`, `repository_id`, `before`, `after`,
  `expected_base64`, `baseline_base64`, `responses`, `original_commits`,
  `inventory`. Scope zwingend `OFFLINE_ONLY`. Baseline null bedeutet initiale
  Hinzufügung, Base64 bedeutet Update mit exakten alten Bytes.
- `responses`: geordnete Liste mit genau `endpoint` und `body_base64`.
  Bodies sind die tatsächlich von der Quelle gelesenen Antwortbytes,
  einschließlich ursprünglicher JSON-Whitespacebytes. Nur vorhandene feste
  REST-Allowlist; keine URL-, Datei-, Shell- oder Credentialinterpretation.
- `original_commits`: genau zwei nach OID streng aufsteigend sortierte
  Datensätze mit genau `git_oid`, `payload_base64`.
- `inventory`: vollständiges neu nachzurechnendes
  NOVENA_OFFLINE_OBJECT_EVIDENCE_V1 aus Block 295, kein vertrauenswürdiges
  Prüfergebnis. Darin feste Repository-/Refbindung, Baseline und geordnete
  initiale/finale Headbeobachtung; feste Publikations-/Promotionssperren.
- Kanonisches kompaktes JSON mit lexikalisch sortierten Schlüsseln und genau
  einem Schluss-LF, kanonisches gepaddetes Base64, kleine vollständige OIDs.
  Bestehender strikter Integer-JSON-Parser: doppelte dekodierte Schlüssel,
  Tiefe ab 128 Containern, Mehrdeutigkeiten und nachfolgende Daten scheitern.
  Unbekannte Versionen, zusätzliche/fehlende Vertragsfelder, Inventarfelder
  und nicht kanonische Paketdarstellungen scheitern geschlossen.

Die ursprünglichen REST-JSON-Bodies werden nicht selbst kanonisiert: ihre
verlustfreie Speicherung ist Teil des Vertrages. Pfade, Modi und OIDs in
Treeeinträgen sowie Originalcommit-Tree-/Parentlinks tragen die vollständigen
Referenzen; diese werden erneut mit den bestehenden Verifiern traversiert.
Keine Dateipfade werden aus dem Paket interpretiert. Ein reines 295-Inventar
wird von verify_package abgewiesen, nicht als byteverifiziert aufgewertet.
Keine importierbare LIVE-Variante, kein neuer Produktiv-/CLI-Modus.

Private Dateiübergabe: expliziter absoluter Pfad ohne `..`, keine Symlinks in
Elternkomponenten, privates Verzeichnis (Unix: keine Gruppen-/Fremdrechte).
Export ausschließlich in ein leeres Verzeichnis, genau der feste Dateiname
`novena-offline-replay-v1.json`, create_new und Unix-Modus 0600. Bestehende
Dateien oder Verzeichnisinhalte werden nicht überschrieben oder bereinigt.
Import verlangt genau diese eine reguläre Datei; Symlinks, Spezialdateien,
weitere Dateien, .git-Verzeichnisse im Übergabeverzeichnis und Unix-Hardlinks
werden abgewiesen. Es werden keine bestehenden Git-Speicher geöffnet.

**Vertrauensvoraussetzung:** Verzeichnis, sämtliche Eltern, Mounts und Dateien
müssen während des Zugriffs privat kontrolliert und unveränderlich sein,
einschließlich offener Schreibhandles. Metadaten-/Inodevergleich ist keine
race-freie Übergabe. TOCTOU zwischen Prüfung/open, Eltern-/Mountaustausch,
Inhaltsänderung über offene Handles und fehlende präemptive I/O-Deadline
bleiben offen. Kein Einsatz in feindlich veränderbaren Verzeichnissen.
Exportfehler können eine unvollständige Datei hinterlassen; kein automatisches
Aufräumen, kein Erfolg und keine zugesagte Crash-/Verzeichnisatomarität.
Ein solcher Rest blockiert erneuten Export. Unvollständige Dateien bestehen
den Import nicht. Kein ZIP/TAR, keine Git-/HTTP-/Shellausführung im Adapter.

### 4. Implementierte Prüfschritte

`verify_package` begrenzt Eingangsbytes, parst strikt und prüft kanonische
Darstellung und exakten Vertrag. Base64-Längen und kumulative Rohgrößen
werden vor Dekodierung geprüft; die Rückkodierung verlangt kanonisches Base64.
Status und Baseline werden erneut mit den vorhandenen Statusfunktionen
vorbereitet; erwartete neue Bytes müssen exakt übereinstimmen.

Ein frischer RunBudget gilt für den gesamten unabhängigen Wiederprüflauf.
Die Replayquelle liefert ausschließlich den nächsten gespeicherten Endpoint
und Body, kein Lookup mit implizitem letzten Wert und kein Netzwerkfallback.
Die Rohquelle liefert jede der beiden Originalbytefolgen genau einmal.
`verify_complete_readback` prüft erneut Repositoryerwartung, erste Ref,
Originalcommit-OID/Profil/REST-Metadaten, Parentbeziehung, vollständige alte
und neue Trees, einzig erlaubte Zieländerung, exakte Ziel-/Baselineblobs,
sämtliche Nebenblobs und letzte Ref. Unverbrauchte Antworten/Commits sind
Fehler. Geänderte Refbeobachtungen bleiben Konflikt; keine Neuauswahl.

Der vorhandene Prüfer berechnet Git-SHA1, Länge und SHA256 aller Objekte neu.
Vergleich der vollständig neu erzeugten kanonischen Paketbytes gegen den
Eingang prüft jedes Inventarfeld, sämtliche Identitäten und ihre Reihenfolge,
Anzahl, Typ, Byteprovenienz, Hashes, Referenzbindung und Vollständigkeit.
Zusätzliche globale OID-Eindeutigkeit schließt widersprüchliche Typidentitäten
im neuen Paket aus. Kein Vertrauen in ein `verified`-Feld; unbekannte Felder
werden auch im eingebetteten Inventar durch den Neuberechnungsvergleich
abgewiesen. Keine zweite Tree-Traversierungsimplementierung.

### 5. Deterministische Testfälle

Vier neue Rust-Testfunktionen in tests/protocol.rs, **nicht ausgeführt**:

1. Vollständiger Initialreadback mit Nicht-UTF8-Commitnachricht und
   LIVE_PROVENANCE-Text, deterministische Paketbytes, unabhängiges Replay,
   identisches neu berechnetes Inventar; reines Inventar scheitert.
   Die gemeinsame Matrix umfasst 23 einzelne Mutationen: unbekannte Version,
   LIVE-Scope, Erfolgsfeld, falsche Länge/SHA256/OID/Typ, fehlendes/doppeltes/
   umsortiertes Inventar, falsche Herkunft, manipulierte/fehlende/doppelte/
   unsortierte Rohcommits, fehlende/umgeordnete Antworten, falsche Anzahl,
   manipulierter Tree/Nebenblob, finaler Refkonflikt, falscher Reftyp und
   unbekanntes Inventarfeld. Zusätzlich doppelte JSON-Schlüssel,
   nichtkanonische Darstellung, Tiefe, Paketübergröße, Commitübergröße,
   257 Antworten. Ein zulässiger REST-Body über 1 MiB prüft den gemeinsamen
   begrenzbaren Decoder. Last_verified und Publikationssperren bleiben geprüft.
2. Update-Fixture mit drei vollständigen Blobs einschließlich altem Zielblob,
   vollständigem Replay, identischem Inventar und unverändertem Checkpoint.
3. Private Export-/Importdatei mit identischen Bytes, fehlender Datei,
   Überschreibversuch, fremder Zusatzdatei, Verzeichnis anstelle Datei,
   Symlink und .git-Inhalt; Fehler liefern keinen CompleteReadback.
4. Bekannte SHA256-Vektoren für leere Bytes und `abc`; für jeden originalen
   synthetischen Commit, Blob und rekonstruierten Tree Gegenprüfung des
   Inventardigests mit getrenntem lokalem `sha256sum` über stdin. Dieses
   Werkzeug ist Testvoraussetzung; Fehlen ist ein Testfehler, kein Skip/PASS.
   Keine Shell, kein produktiver Git-/SSH-Prozess und kein Herkunftsschluss.

Die vorhandenen erfolgreichen vollständigen Loopback-TLS-Fälle in
src/https_local_tls.rs wurden um die anschließende unabhängige Paketprüfung
erweitert. Das sind echte bestehende TLS-Sitzungen mit synthetischen Daten,
kein In-Memory-HTTP als TLS-Ersatz. Rohcommits kommen dort weiter aus der
privaten RawCommitFiles-Fixture. Es ist keine neue TLS-Testanzahl behauptet.
Bestehende Initial-/Update-, Nebenblob-, Ref-, Status-, Budget- und
Fehlerregressionen bleiben bestehen und müssen der äußere Runner qualifizieren.

### 6. Byteprovenienz und SHA256

Commit: tatsächlich gelesene Original-Nutzbytes, unverändert und verlustfrei
Base64; schmaler unsigned Git-Committyp inklusive Nicht-UTF8-Nachricht.
Tree: Original-REST-Antwortbytes im Replay; Git-Nutzbytes ausschließlich aus
vollständigen REST-Einträgen kanonisch rekonstruiert und OID-verifiziert.
Keine Behauptung original transportierter Git-Treebytes. Blob: tatsächlich
abgerufene und erneut dekodierte, längen- und OID-geprüfte Inhalte.

Git-OID ist SHA1 über Git-Header plus Nutzbytes; unabhängiger SHA256 ist
SHA256 der Nutzbytes OHNE Git-Header. Typ und Länge bleiben getrennte Felder.
Bekannte Vektoren und Werkzeuggegenprüfung stärken die Hashqualifikation,
authentifizieren aber weder GitHub, Repository, Refbeobachtung noch Exporteur.
Auch ein vollständig konsistent neu erzeugtes Fremdpaket bleibt OFFLINE.

### 7. Gemeinsame Ressourcenbudgets

Neue explizite Paketgrenzen: 64 MiB Paketdatei/-eingang vor JSON-Parsing;
256 geordnete Antwortdatensätze, je höchstens 8 MiB dekodiert; zwei Commits,
je höchstens 1 MiB; zusammen höchstens 32 MiB dekodierte Transport- und
Commitbytes. Neue Statusbytes und optionale Baseline je höchstens 1 MiB.
Maximal 4354 Inventarobjekte (2 Commits + 4096 Trees + 256 Blobs).
Ein-/Ausgabe-Vektoren sind durch diese Byte-/Anzahlgrenzen beschränkt.

Paketbildung prüft vor Base64-/JSON-Aufbau eine konservative Größenobergrenze
mit Escapingreserve. V1-Inventarbildung prüft zusätzlich Anzahl, Commitgrößen
und eine konservative Serialisierungsobergrenze vor den Zwischenpuffern.
Import zählt Base64-Nutzlängen vor Decoderallokation kumulativ; 8-KiB-Lesepuffer
und Grenze vor jedem Anwachsen, kein unbeschränktes read_to_end. Keine
Vollmaterialisierung über die Paketgrenze hinaus. Strikter Parser begrenzt
JSON-Tiefe auf den vorhandenen Umfang. Geparste JSON-Bäume, Kopien,
Serialisierungspuffer und Originalantworttranskript verbrauchen zusätzlich
RAM; 64 MiB Paketgröße bedeutet ausdrücklich KEIN 64-MiB-Prozesslimit.

Bestehender RunBudget bleibt gemeinsam über alle Reader hinweg: 256 Abrufe,
8 MiB/Body, 32 MiB REST plus Rohcommits, zwei Rohcommits und 2 MiB roh;
Tree-/Blob-/Pfadlimits und Vergiftung bleiben erhalten. Das neue Transkript
ist durch denselben Abruf-/Bytezähler begrenzt. Paket-Replay ist ein eigener
vollständiger Lauf, innerhalb dessen kein Budget neu gestartet wird.
Das Paket exportiert genau das Antwortsegment dieses vollständigen Readbacks.
Vorausgehende Abrufe desselben Kontexts belasten weiterhin dessen Laufzähler;
die Segmentauswahl setzt kein Budget zurück. Das unabhängige Replay belegt
den vollständigen Readback, nicht vorausgehende fremde Operationen.
Keine Übernahme als LIVE-Beleg und kein Fortsetzen nach Importfehler.

Unverändert offen: getrennte flatten-Traversalzähler und empfangsseitige
kombinierte Restbudgetlücken aus 294. Die neuen Paketgrenzen ersetzen weder
OS-cgroups noch Dateisystemquoten, Supervisorisolation oder eine harte Deadline.

### 8. Ergebnis- und Fehlersemantik

Nur vollständiger Erfolg gibt CompleteReadback mit Paket und neu berechnetem
Inventar zurück. Fehler sind Err, keine Teil-PASS-Berichte, keine partielle
Erfolgsevidenz. Datei-Schreibreste sind ausschließlich fehlgeschlagene Exporte.
Bestehender äußerer RunBudget wird bei Fehler der Paketbildung vergiftet.
Import gibt bei späterem Inventar-/Restdatenfehler ebenfalls keinen bereits
intern erzeugten Beleg zurück. Kein Fehler verändert den ursprünglichen
Paketwert, Statuspayload, last_verified oder Publikationszustand.

Kein Statusindex, keine fachliche Abnahme, kein Promotionsrecht, kein neuer
CLI-Schalter und kein Livezugriff. production_publish bleibt bedingungslos
gesperrt. Alte Offlineausgaben werden nicht umetikettiert.

### 9. Restliche technische Blocker – verbindliche Bestandsaufnahme

| Kategorie | Bestand und Aussagegrenze |
|---|---|
| A – implementiert, mit vorhandenen Offline-Tests qualifizierbar | Initial-/Update-Readback, vollständige Tree-/Blobnachweise im begrenzten Zwei-Commitprofil, Git-OID-Prüfung, Nutzbyte-SHA256, neues deterministisches Offlinepaket, unabhängige Neuberechnung, private eingeschränkte Dateiübergabe, Manipulationsmatrix, bekannte Hashvektoren und externe lokale Testgegenprüfung, TLS->Replay-Verbindung, Status-/Publikationssperren. Neue Fälle warten auf äußere Ausführung. |
| B – implementiert, durch diese Matrix nicht vollständig nachgewiesen | Alle maximalen Ressourcenrandkombinationen; nicht-Unix-Dateiverhalten; Abbruch bei realem Disk-full/Syncfehler; Crashdauerhaftigkeit; große Speicher-/CPU-Last; sämtliche vom vorhandenen Parser unterstützten Varianten. Metadatenprüfungen sind keine Racequalifikation. HttpsGetSource existiert, aber kein authentifizierter GitHub-Lauf ist nachgewiesen. |
| C – technisch offen/BLOCKED | Sicherer isolierter SSH-Git-Erheber; qualifizierter Host-Prozess-Supervisor; cgroups, Prozessbaumabschluss und Quoten; unveränderliche Rohdatenübergabe; 32-KiB-Headerbytegrenze VOR Parserallokation; vollständige präemptive Gesamtdeadline; gemeinsamer strikter Traversal-/Empfangsrestkontext. Authentifizierte Repositoryidentität einschließlich unabhängiger numerischer Pins/private und Endwiederholung fehlt. |
| D – spätere Einzel-Freigabe | Echte SSH-Git-Erhebung, isolierter lokaler Fetch, getrennte REST-/SSH-Nur-Lese-Credentials und deren Übergabe, unabhängig bestätigtes Hostschlüsselpinning/Repositorypins, Host-/cgroup-/Quotenkonfiguration, authentifizierte Refbeobachtung vor/nach der gesamten Erwerbung, echte REST-Anfragen, unabhängige SHA256-Gegenprüfung auf realen Originalen, produktiver Readback. Veröffentlichung und last_verified-Promotion brauchen darüber hinaus eigene Aufträge. |

Lokale Refbeobachtungen vor/nach der vollständigen Objektprüfung sind bereits
im Prüfer und Replay enthalten. Sie beweisen keine echte Erwerbsklammer,
keinen atomaren Snapshot und keinen Ausschluss zwischenzeitlicher ABA-Wechsel.
Die vollständigen lokalen Tree-/Blobnachweise und Hashes schließen keine
historische Ancestorprüfung oder authentifizierte Herkunft ein.

### 10. Konkrete nächste Implementierung

Zuerst äußere Offline-/Locked-Runnerqualifikation der neuen vier Tests,
der erweiterten bestehenden TLS-Fälle sowie Rustfmt/Check/Clippy. Die
Quelländerungen dieses Blocks sind bis dahin unqualifiziert. Bei Fehlern
zuerst erhaltene Runnerausgabe auswerten, keine PASS-Zahl vorwegnehmen.
Anschließend den bereits in 294 benannten gemeinsamen Traversal-/Empfangs-
restkontext in den vorhandenen Verifier integrieren. Supervisor,
unveränderliche Übergabe und Headergrenze bleiben getrennte konkrete
Implementierungsblocker vor jedem Live-Lauf; keine neue Architekturvariante.

### 11. Nicht freigegebene Tätigkeiten

Keine Tests/Builds/Clippy/Cargo-Checks durch Codex, keine Dependencybeschaffung,
keine Änderung an Cargo.toml oder Cargo.lock, kein Python-/Bash-Quellcode,
kein externer Netzwerkzugriff, kein GitHub-Live-Readback, SSH/Clone/Fetch,
kein Git add/commit/push/reset/clean oder Indexschreiben, keine Publikation,
kein last_verified-Eingriff, Hardware-/Blockgerätezugriff, Imagebuild oder
Host-Sicherheitsänderung. Die neuen Dateitests wurden nur implementiert;
Codex hat keine Export-/Import-Testverzeichnisse erzeugt und sha256sum nicht
als neuen Testlauf gestartet. Bestehende Dateien und Checkpoints erhalten.

CHECKPOINT=NOVENA-BRIDGE-RUST-296-CODEX;
RESULT=IMPLEMENTED_PENDING_EXTERNAL_QUALIFICATION;
NEW_TEST_FUNCTIONS=4_NOT_EXECUTED;
EXISTING_FULL_TLS_CASES=EXTENDED_WITH_INDEPENDENT_OFFLINE_REPLAY;
CONTRACT=NOVENA_OFFLINE_REPLAY_V1;
PROVENANCE=OFFLINE_ONLY;
HEADER_MEMORY_BOUNDARY=BLOCKED;
SSH_ACQUISITION=BLOCKED_PENDING_SUPERVISOR_AND_HANDOFF;
AUTHENTICATED_GITHUB_READBACK=NOT_QUALIFIED;
ATOMIC_HANDOFF=NOT_CLAIMED;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-296 – Äußere Qualifikation

Datum: 2026-10-26.

Ausgangsstand:
152 bestandene Rust-Tests,
43 bestandene lokale TLS-Tests.

Auftrag:
Große integrierte Rust-Entwicklungsrunde
für Offline-Evidenzvertrag, Import,
unabhängige Wiederprüfung,
Byteintegrität, Budgets und Regression.

Äußere Qualitätsprüfungen:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

RUST_QUALITY=PASS

Die tatsächliche Abdeckung der Arbeitspakete
und ihre Restrisiken ergeben sich aus dem
Checkpoint NOVENA-BRIDGE-RUST-296-CODEX
und dem archivierten Originalprotokoll.

Kein Rust-Test-PASS belegt eine
authentifizierte GitHub-Live-Erhebung.

HEADER_MEMORY_BOUNDARY=BLOCKED;
SSH_ACQUISITION=BLOCKED;
HOST_SUPERVISOR=NOT_QUALIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-297-CODEX

Datum: 2026-10-26. Implementierung vorhanden, äußere Qualifikation ausstehend.
Alle bisherigen Checkpoints einschließlich Architekturentscheidung 292 bleiben
erhalten. Ausgangsstand laut Auftrag: 156 Rust-Tests, 43 lokale TLS-Tests,
Check/Rustfmt/Clippy PASS. Diese Zahlen werden hier nicht als neue Ergebnisse
fortgeschrieben. Keine Cargo-Ausführung durch Codex.

### Verträge und Vertrauensstufen

Grundlage: Git-Zustand, AGENTS.md, NOVENA-ARBEITSREGELN.txt, Repository- und
Bridge-README, Projekt-/Test-/Entscheidungsdokumente, Bridge-Plan, Checkpoints
292–296 und vorhandene Rust-Quellen, Fixtures, Cargo.toml/Cargo.lock. Die lokale
Dateinamensuche unter test-logs/, evidence/ und research/ lieferte keine
passenden Bridge-Rohprotokolle. Der äußere Qualifikationsnachtrag 296 ist eine
Zusammenfassung; hier wird keine neue Primärausgabe dafür behauptet.
Das historische README behauptet eingangs noch das Fehlen eines HTTP-Clients;
maßgeblich für den heutigen Implementierungsbestand sind https.rs und die
neueren Checkpoints. README blieb außerhalb des erlaubten Schreibumfangs.

Neues kleines Modul `offline_run.rs`, kein zweiter Objekt-/Paketprüfer:

- `OfflineRun` ergänzt den vorhandenen `ReadbackSpec` um erwartete Basis-OID,
  lokalen Laufnamen, erwarteten Paket-SHA256 und eine direkte Referenz auf den
  vorhandenen `RunBudget`. Snapshot bedeutet in diesem Vertrag initiale
  Hinzufügung, Transition Update mit exakten alten/neuen Statusbytes.
  Die erwartete Basis ist auch initial ausdrücklich vorgegeben. Beim Update
  muss sie zusätzlich mit dem Baselinepin im ReadbackSpec übereinstimmen.
- Der vorhandene `AcquisitionManifest` und `ExportClaim` bleiben die einzige
  untrusted Erwerbshülle. Kein neuer konkurrierender Erwerbsmanifesttyp.
- `CompleteReadback`, `OfflinePackage`, `OfflineObjectEvidence` und die
  vorhandenen Git-/REST-/Statusverifier bleiben die Byteprüfgrundlage.
- `BoundOfflineReplay` ist nur durch vollständigen Erfolg konstruierbar,
  besitzt private Felder und keine JSON-Deserialisierung oder LIVE-Variante.

Die fünf Kategorien bleiben getrennt: angeforderter Lauf (`OfflineRun`),
untrusted Erwerbsbehauptung (`AcquisitionManifest`), geprüfte Originalcommit-
Nutzbytes und OIDs (bestehender Verifier), lokal neu geprüfte Evidenz
(`BoundOfflineReplay`) und authentifizierte Remote-Herkunft. Letztere bleibt
unimplementiert und über diese APIs unerreichbar. Numerische Repository-ID
ist ausdrücklich nur eine gebundene Erwartung; der bestehende REST-Verifier
prüft sie nicht als unabhängigen Identitätsbeleg. Node-ID und full_name werden
weiterhin gegen die gelieferten Offlineantworten geprüft, nicht authentifiziert.

### Deterministische Lauf- und Paketbindung

Manifestvertrag `NOVENA_OFFLINE_RUN_V1`: feste Repository-/Refbezeichnung aus
ReadbackSpec, erwartete Node-ID/numerische ID, Basis-/Ziel-OID, initial/update,
expliziter Laufname, geordnete Beobachtungen mit Phasen initial/final, genau
zwei sortierte Commitreferenzen mit OID/Nutzbytelänge/SHA256, Paketversion und
SHA256 der gesamten kanonischen Paketdatei einschließlich Schluss-LF sowie
`EXISTING_RUN_BUDGET_V1`. Scope ausschließlich OFFLINE_ONLY, Provenienz
ausschließlich UNTRUSTED_ACQUISITION_CLAIM. Die Phasen unterscheiden die zwei
Beobachtungen auch dann, wenn ihre OIDs erwartungsgemäß identisch sind.

Laufname: 1–128 ASCII-Zeichen aus Buchstaben, Ziffern, Punkt, Unterstrich und
Bindestrich; kein Zufall, keine Uhrzeit. Der Aufrufer muss Namen für logisch
verschiedene Läufe eindeutig vergeben. Es gibt weder eine globale Registry
noch eine Behauptung globaler Eindeutigkeit aus untrusted JSON.

`manifest_bytes` bildet eine deterministische Exportvorlage aus einem bereits
geprüften CompleteReadback und vergleicht dessen Paket mit den Erwartungen.
Sie führt selbst keinen neuen Byte-Replay aus und ist kein neuer Erfolgsbeleg.
Beim Import wird auch diese Vorlage ausschließlich als Behauptung behandelt.

Manifest höchstens 16 KiB vor dem strikten JSON-Parser. Dieser verwirft doppelte
dekodierte Schlüssel, ungültige Zahlen und übermäßige Tiefe. Exakter Vergleich
mit den kanonisch aus neu geprüften Daten erzeugten Manifestbytes verwirft
fehlende/zusätzliche/unbekannte Felder, Versionen, Herkunft, Reihenfolge,
OID-Schreibweisen, Längen und Hashes. ExportClaim wird zusätzlich separat mit
den neu geprüften Commitobjekten verglichen; eine passende JSON-Behauptung
kann widersprechende ExportClaims nicht verdecken. Die bestehenden strikten
OID-Parser und Originalcommit-Prüfer bleiben im Paketpfad maßgeblich.

### Vollständiger Datenfluss und Budget

Explizite Laufvorgabe -> begrenztes untrusted Manifest -> Paketgrößen-/SHA256-
Prüfung -> bestehender strikter Paketparser -> Statusvorbereitung -> geordnete
Replayquelle und originale Commitquelle -> vollständiger bestehender Readback
mit Repository, erster Ref, Originalcommit-OIDs und Metadaten, Parentbeziehung,
vollständigen Trees, Ziel-/Baselineblobs und sämtlichen Nebenblobs, letzter Ref
-> neu berechnetes Inventar/Paket -> exakte Lauf-/Manifest-/ExportClaimprüfung
-> erst dann BoundOfflineReplay.

`verify_package_in_run` ist eine interne Erweiterung des vorhandenen Pfades.
Sie verwendet den vom OfflineRun referenzierten RunBudget unverändert über
alle Quellen. Der öffentliche `verify_package` bleibt kompatibel und erzeugt
wie bisher einen frischen Kontext für einen vollständigen unabhängigen Lauf.
Paket-/Manifestfehler vergiften den Kontext; Folgeversuche scheitern mit
RUN_ALREADY_FAILED. Vorher verbrauchte Abrufe werden nicht zurückgesetzt.
Keine neuen frei konfigurierbaren Budgetzahlen im Manifest, keine frei
gewählten Objektpfade und keine neue Transportquelle. Bestehende Paket-,
Decoder-, Objekt- und Traversalgrenzen bleiben erhalten.

Zwei getrennte Aufrufe verwenden getrennte Budgets, Replayqueues und
Originalcommitmaps und rechnen erneut Git-SHA1, Nutzbyte-SHA256 und Längen.
Commitbytes stammen unverändert aus dem Paket; Treebytes werden wie bisher
aus vollständigen REST-Einträgen rekonstruiert, nicht als originale transportierte
Treebytes ausgegeben. Blobs werden erneut dekodiert und geprüft.

`compare_replays` vergleicht die komplette Bindung, vollständige Paketbytes und
vollständige neu berechnete Inventarbytes einschließlich Repository, Ref,
Commit-/Treeidentitäten, Blobinventar, SHA256, Beobachtungsfolge, Paketversion
und Offline-Sperren. Derselbe Ergebniswert als beide Argumente ist ein Fehler.
Gleichheit liefert ausschließlich `REPLAY_CONSISTENT_OFFLINE`. Verschiedene
gültige Pakete liefern OFFLINE_REPLAY_CONFLICT; manipulierte Pakete scheitern
schon vor einem gebundenen Ergebnis. Keine zwei GitHub-Erhebungen behauptet.

### Implementierte Tests, noch nicht ausgeführt

Zwei neue Testfunktionen in tests/protocol.rs; bestehender Update-Test und
bestehender lokaler TLS-Positivfall erweitert. Keine neue PASS-Zahl behauptet.

| Geforderter Fall | Implementierter Nachweiszweck |
|---|---|
| 1 gültige Bindung | Initialbindung, deterministisches Manifest; bestehende Updatefixture zusätzlich mit zwei gebundenen Replays |
| 2 falsches Repository | Repositoryname und Node-ID im Manifest; zusätzlich numerischer Erwartungspin |
| 3 falscher Ref | Manifestmutation, außerdem gezielter TLS-Folgefall |
| 4/5 falsche Basis/Ziel-OID | Je eigene Manifestmutation |
| 6 vertauschte Beobachtungen | Vertauschte initial/final-Phasen, zusätzlich umgeordnete Paketantworten |
| 7 manipuliertes Rohcommit | Andere Nutzbytes bei ursprünglicher OID |
| 8 fehlendes Rohcommit | Originalcommitdatensatz entfernt |
| 9 manipuliertes Nebenblob | Tatsächlicher Base64-Inhalt verändert |
| 10 unvollständiger Tree | Eintrag aus tatsächlichem Treeantwortbody entfernt |
| 11 widersprüchlicher SHA256 | Paketbindung, Inventarhash und separater ExportClaim jeweils verändert |
| 12 falsche Paketversion | Paket- und Manifest-Paketversionsmutation, zusätzlich Manifestversion |
| 13 doppeltes Objekt mit Konflikt | Zusätzliches Inventarobjekt mit abweichender Länge; zusätzlicher unerlaubter Rohcommit; zusätzliche Manifestcommitreferenz |
| 14 Budgeterschöpfung | 256 gültige vorherige Repoabrufe im selben Kontext; anschließender Paketlauf scheitert |
| 15 zwei konsistente Replays | Getrennte Budgets und vollständige Byteprüfung, Initial/Update/TLS |
| 16 widersprüchliche Replays | Zwei jeweils gültige Pakete mit verschiedenen originalen Commitnachrichten; Vergleich scheitert |
| 17 keine LIVE-Promotion | LIVE in Paket/Manifest und authentifizierte Herkunftsbehauptung scheitern |
| 18 keine Veröffentlichung | publication_performed=false, Status NOT_PUBLISHED, production_publish weiterhin gesperrt |
| 19 kein last_verified-Eingriff | Initial null; Update exakt unverändert; vorhandene Statusprüfer wiederverwendet |
| 20 keine Teilerfolgsevidenz | Alle Mutationen ergeben Err; zweiter Versuch mit vergiftetem Budget scheitert; ursprüngliche Paketbytes unverändert |

21 einzelne Manifest-/ExportClaim-Mutationen und 11 einzelne Paketmutationen
in den neuen Matrixfällen. Weitere Mutationen: Laufname, Modus, Budgetversion,
ExportClaim-Reihenfolge und Objektlänge. Die Objektmatrix passt den erwarteten
Paketdigest absichtlich an die manipulierten Bytes an: die Ablehnung darf dort
nicht allein auf einem alten äußeren Digest beruhen. So wird der bestehende
Byteverifier erreicht. Kein Mock-Ergebnisvergleich ersetzt die Byteprüfung.
Bestehende 296-Fälle für Parsermehrdeutigkeit und Paketgrenzen bleiben erhalten.

TLS-Anschluss im bestehenden Fall 0: vorhandener Loopback-TLS-Server ->
HttpsGetSource/RestReader -> vollständiger Readback -> Inventar/Paket ->
privater bestehender Exportadapter -> Importadapter mit eigener Wiederprüfung
-> zwei zusätzliche unabhängige gebundene Replays -> Vergleich. Negativfall:
falscher Ref im Manifest mit eigenem Budget, ohne weitere TLS-/Remoteanfrage.
Keine neuen Hosts oder Credentials. Die Dateianlage ist nur implementierter
Testcode; Codex hat diesen Test und seine Dateioperationen nicht ausgeführt.

### Erreichte Implementierung und offene Garantien

Implementiert sind deterministische explizite Offlinebindung, strikter
Manifestabgleich, unabhängige vollständige Objektbyte-Wiederprüfung und
Vergleich zweier getrennter lokaler Replayvorgänge. Laufzeitqualifikation bleibt
bis zur Runnerausführung offen. Rustfmt wurde direkt auf den bearbeiteten
Rust-Dateien ausgeführt, Exit 0. Keine Cargo-Builds, Tests, Cargo-Checks oder
Clippy. Git diff --check Exit 0 betrifft nur getrackte Änderungen; die Crate
war bereits unversioniert, deshalb kein vollständiger Diffnachweis ihrer
Änderungen. Cargo.toml, Cargo.lock und Git-Index vor/nach anhand SHA256 identisch.
Vorhandene Nutzeränderungen außerhalb des erlaubten Umfangs blieben erhalten.

Unverändert BLOCKED: authentifizierte GitHub-Erhebung, qualifizierter isolierter
SSH-Git-Erwerb, sicherer Host-Supervisor und 32-KiB-HTTP-Headerbytegrenze VOR
Parserallokation. Offlinebudgets ersetzen weder cgroups/Quoten noch Prozessbaum-
abschluss oder präemptive Gesamtdeadline. TOCTOU bei veränderlichen lokalen
Verzeichnissen, Eltern, Mounts und offenen Schreibhandles bleibt offen.
Gemeinsame Traversal-/Empfangsrestgrenzen aus 294 bleiben separat offen.
Keine atomare Remoteaufnahme, kein ABA-Ausschluss, keine historische
Ancestorprüfung und keine authentifizierte Provenienz numerischer Pins.

Konkrete nächste Aufgabe: äußere Offline-/Locked-Runnerqualifikation der zwei
neuen Tests und der erweiterten Update-/TLS-Fälle einschließlich Check,
Rustfmt und Clippy. Danach den bestehenden gemeinsamen Traversal-/Empfangs-
restkontext schließen; Supervisor und unveränderliche Übergabe sowie die
Headergrenze separat implementieren und qualifizieren. Vor einem möglichen
authentifizierten Erwerbslauf zusätzlich unabhängig bestätigte Repository-/
Hostschlüsselpins, getrennte Nur-Lese-Credentials und konkrete Einzelfreigabe
benötigt. Architektur bleibt isolierter SSH-Git-Erwerb plus unabhängiger
Rust-REST-Readback; hier kein SSH-Erwerber implementiert.

Keine Netzwerkverbindung, Git-/SSH-Unterprozesse, Dependencybeschaffung,
Hoständerung, Statusindex-Veröffentlichung, last_verified-Promotion, Hardware-
oder Imagearbeit durch diesen Auftrag. Änderungen ausschließlich an erlaubten
Rust-Dateien und diesem Fortschrittsdokument; keine Git-Mutation.

CHECKPOINT=NOVENA-BRIDGE-RUST-297-CODEX;
RESULT=IMPLEMENTED_PENDING_EXTERNAL_QUALIFICATION;
CONTRACT=NOVENA_OFFLINE_RUN_V1;
CONSISTENCY_RESULT=REPLAY_CONSISTENT_OFFLINE_ONLY;
NEW_TEST_FUNCTIONS=2_NOT_EXECUTED;
EXISTING_UPDATE_AND_TLS_CASES=EXTENDED;
AUTHENTICATED_GITHUB_ACQUISITION=BLOCKED;
SSH_ACQUISITION=BLOCKED; HOST_SUPERVISOR=NOT_QUALIFIED;
HEADER_MEMORY_BOUNDARY=BLOCKED; TOCTOU=OPEN;
BRIDGE_PUBLICATION=BLOCKED; LAST_VERIFIED=UNMODIFIED.

---

## NOVENA-BRIDGE-RUST-297 – Äußere Qualifikation

Datum: 2026-10-26.

Ausgangsstand: 156 Rust-Tests,
darunter 43 lokale TLS-Tests.

Auftrag: deterministische Offline-Laufbindung,
strikte Erwerbsmanifestprüfung und
unabhängiger Replay-Vergleich.

Prüfergebnisse:

- cargo check: Exit 0
- cargo fmt --check: Exit 0
- cargo clippy -D warnings: Exit 0
- cargo test --offline --locked: Exit 0

RUST_QUALITY=PASS

Tatsächlich implementierte Garantien
sind dem Codex-Checkpoint und dem
Originalprotokoll zu entnehmen.

Offline-Replay ist kein Nachweis
authentifizierter GitHub-Herkunft.

HEADER_MEMORY_BOUNDARY=BLOCKED;
SSH_ACQUISITION=BLOCKED;
HOST_SUPERVISOR=NOT_QUALIFIED;
LIVE_GITHUB_READBACK=NOT_EXECUTED;
BRIDGE_PUBLICATION=BLOCKED;
LAST_VERIFIED=UNMODIFIED.
