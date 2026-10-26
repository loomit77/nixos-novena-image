# Novena Bridge (Rust)

Offline-Entwicklungskern für `novena/status/NOVENA-CURRENT-STATE.json`.
Produktive Veröffentlichung ist bedingungslos gesperrt. Es gibt weder
HTTP-/SSH-Client noch Credentialzugriff, Git-Schreiboperation oder Dienst.
Alle CLI-Ausgaben erfolgen auf stdout, erst nach vollständiger Prüfung.

## Tests

Im Crate-Verzeichnis mit dem bereits gesetzten externen `CARGO_TARGET_DIR`:

```bash
cargo test --locked --offline
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

Laufzeitabhängigkeiten: `sha1`, `sha2` (=0.10.9), `serde_json`. SHA256 wurde
vor der Ergänzung lokal gefunden und ausschließlich offline aufgelöst. Entwicklungstests
benutzen zusätzlich vorhandenes `python3`, `jsonschema` und `git`.
Python läuft mit `-B`, Referenzmodule bleiben unverändert. Git wird nur mit
`hash-object --stdin` ohne `-w` aufgerufen. Es wird nichts heruntergeladen.
Verwendete neuere std-APIs benötigen Rust >= 1.88 (lokal Rust 1.99 geprüft).

## Lesende CLI

Alle FILE-Argumente außer `inspect-log` dürfen `-` für stdin sein. Reguläre Dateien sind
erforderlich; finale Symlinks und spezielle Dateien werden abgewiesen.
Keine gemeinsame transaktionale Dateiveröffentlichung: die CLI schreibt
selbst keine Dateien. Umleitungen durch einen Aufrufer benötigen dessen
eigene Freigabe. Statusdaten sind auf 1 MiB, allgemeine Eingänge und
Belegbündel auf 32 MiB begrenzt.

| Befehl | Bedeutung |
|---|---|
| `inspect-log FILE` | Vorhandenes Runner-TXT samt optionalem `FILE.sha256` lesen; getrennter JSON-Prüfbericht, kein Statuseingriff |
| `blob-id FILE` | Git-SHA1 eines beliebigen Blob-Inhalts |
| `canonical-json FILE` | Allgemeines JSON kanonisieren; **keine Statusprüfung**, historisch auch Fließzahlen/mehrfache Schlüssel erlaubt |
| `exchange-v1-json FILE` | Striktes Integer-JSON ohne doppelte dekodierte Schlüssel |
| `validate-exchange-v1 FILE` | Zusätzlich strukturelles Schema v1; keine Evidenz-/Freigabeprüfung |
| `prepare-initial FILE` | Lokaler Status, null-Checkpoint und keine Publikationsbehauptung; kanonische Statusbytes |
| `prepare-status PREVIOUS CANDIDATE` | Lokale Statusfortsetzung bei exakt unverändertem Checkpoint |
| `prepare-from-archive PREVIOUS CANDIDATE` | Dasselbe mit strukturell als CONFIRMED markierter Baseline; diese Markierung allein beweist keine Quelle |
| `prepare-objects FILE` | Rein lokale Blob-/Tree-/Commitplanung aus einem Vorbereitungsbündel |
| `plan-cas REPOSITORY_ID BEFORE_OID AFTER_OID` | GraphQL-Anforderung mit exaktem beforeOid und force=false, keine Ausführung |
| `verify-update-offline FILE` | Vollständige Update-Transition mit exakten Baselinebytes prüfen; kein Snapshotbeleg |
| `verify-offline FILE` | Unabhängige Objektprüfung eines lokalen Readbackbündels |

`publish`, `production-publish` und `--publish` schlagen auch mit weiteren
Argumenten vor jeglichem Dateizugriff fehl. Kein Flag kann die Sperre lösen.
Ein CAS-Plan allein bedeutet weder Objektqualifikation noch Publikationsrecht.

## Reiner Logeingang (M5-Teilumfang)

`logs::inspect_bytes` prüft bis zu 32 MiB Originalbytes im Speicher;
`logs::inspect_file` und `inspect-log` lesen ausschließlich reguläre Dateien.
Der optionale SHA256-Sidecar ist auf 8 KiB begrenzt. Eine einzelne
Prüfsummenzeile mit passendem Basisnamen wird gegen genau dieselben Bytes
geprüft, die der Parser untersucht. Fehlende Sidecars bleiben `MISSING`,
unlesbare/ungültige Sidecars `INVALID`, abweichende Hashes `FAIL`.

Konflikte in Blockkennung, Ergebnis, Exitstatus, Abschlussmarker und
`UPLOAD_RESULT`, ungültiges UTF-8, fehlende Felder und widersprüchliche
Ergebnis-/Exitstatuspaare ergeben `REVIEW_REQUIRED` und Runner `UNKNOWN`.
Identische Wiederholungen sind erlaubt; Exitstatus wird als Dezimalzahl
0 bis 255 geprüft. `COMPLETE` bezeichnet nur einen kohärenten, mit Sidecar
integritätsgeprüften Runnerdatensatz. Die Angaben stammen weiterhin aus
unbeglaubigtem Logtext; auch kopierte oder eingebettete Marker sind kein
unabhängiger Nachweis des tatsächlichen Prozessablaufs.

`claimed_block_result` bewahrt die Runnerbehauptung separat auf. Upload,
technisches Ergebnis und Fachabnahme bleiben ohne unabhängige Evidenz
`UNKNOWN`, auch bei `UPLOAD_RESULT=PASS`. Anders als die Python-Referenz
werden konflikthafte letzte Werte nicht als belastbare Ergebnisse übernommen;
unvollständige Logs erhalten einheitlich `REVIEW_REQUIRED`.

`logs::prepare_from_log` aktualisiert ausschließlich `latest_block` eines
lokalen geprüften Statusdokuments. `last_verified`, `current`, Publikation
und alle übrigen Felder bleiben gleich. Fehlende Blockkennung wird abgewiesen.
Die API akzeptiert Originalbytes, keinen vom Aufrufer veränderbaren Prüfbericht.
Remote-Baselines sind in diesem Logadapter gesperrt; Variante A verwendet
lokale Payloadstatuswerte. Der lokale Quellenmerge ist unten beschrieben;
automatische Verzeichnis-/Neueste-Auswahl bleibt außerhalb dieses Vertrags. Es wird keine Freigabe erteilt und kein Original
verändert. Finale Symlinkprüfung bietet keinen atomaren Schutz gegen
konkurrierenden Dateiaustausch und prüft keine übergeordneten Pfadkomponenten.

## Offline-Bündel

`verify-offline` erwartet genau diese Felder:

```json
{
  "repository_id": "SYNTHETIC_REPOSITORY_ID",
  "before": "40 lowercase hex characters",
  "after": "40 lowercase hex characters",
  "candidate": {},
  "responses": {},
  "raw_commits": {}
}
```

`candidate` enthält ein vollständiges Exchange-v1-Dokument. Es muss initial
`last_verified=null`, `publication.state=NOT_PUBLISHED` und
`publication.commit=null` besitzen. Ein publiziertes Blob kann nicht den
OID seines eigenen umschließenden Commits behaupten; Commitidentität steht
im unabhängigen Prüfbericht, nicht als nachträgliche Änderung am Payload.

`responses` bildet exakte REST-Pfade auf vollständige JSON-Antworten ab:

- `repos/loomit77/project-block-logs`: `full_name`, `node_id`.
- `repos/loomit77/project-block-logs/git/ref/heads/main`: exakter Ref und
  Commitobjekt. Im statischen Bündel muss HEAD `after` sein.
- `.../git/commits/OID`: `sha`, `tree.sha`, vollständige `parents[].sha`.
- `.../git/trees/OID`: `sha`, `truncated=false`, vollständige direkte
  `tree[]`-Einträge mit `path`, `mode`, `type`, `sha`.
  Optionale `size`/`url`-Metadaten werden geprüft und nicht mitgehasht.
- `.../git/blobs/OID`: `sha`, `encoding=base64`, `size`, `content`.

`raw_commits` bildet beide Commit-OIDs auf base64-kodierte **Originalbytes**
ab. Sie dürfen nicht aus REST-Metadaten rekonstruiert werden. Fehlende
Originalbytes führen zum Abbruch. Roots und Commits mit genau einem Parent
werden im dokumentierten schmalen unsigned Profil unterstützt:
`tree`, optional `parent`, `author`, `committer`, Leerzeile, nichtleere
Nachricht. Merge-/Signatur-/Encodingheader und CR/NUL werden abgewiesen.
Nicht-UTF8-Gitpfade werden nicht unterstützt; Blobbytes dürfen beliebig sein.

`prepare-objects` erwartet dieselbe Bündelstruktur, jedoch ohne `after` und
zusätzlich `identity` und `message`. HEAD muss hier `before` sein.
`identity` ist die exakte Git-Identität, beispielsweise
`Novena Test <test@localhost.invalid> 1760000000 +0000`; `message` wird
ohne implizite Newline-Ergänzung übernommen. Die Ausgabe enthält alle neuen
Objektbytes als base64, ihre OIDs und eine reine CAS-Anforderung. Vorhandene
Geschwister und Unterbäume bleiben erhalten; eine existierende Zieldatei
oder ein Nichtverzeichnis in ihren Eltern wird abgewiesen.

Ein ausführbares End-to-End-Beispiel dieser Formate steht in
`tests/protocol.rs` (`object_plan_matches_independent_graph_and_cli`,
`offline_bundle_cli_end_to_end`). Die dortigen Daten werden vollständig
im Speicher erzeugt und nie in ein Git-Repository geschrieben.

## Nachweis- und Sicherheitsgrenzen

Geprüft werden Hashidentität, Objektlinks, vollständiger Treevergleich
einschließlich leerer Verzeichnisse, Zielmodus 100644 und exakte Payloadbytes.
Initiale Hinzufügung und die unten beschriebene feste Zielpfad-Ersetzung
werden als Objektänderungen unterstützt.
Die begrenzte Traversierung erlaubt maximal 4096 Treebesuche, Tiefe 128,
32 MiB Treebytes, 4096 Byte je Pfad und rund 100000 Einträge. REST erlaubt
maximal 256 Antworten von jeweils höchstens 8 MiB; große Archive werden
abgewiesen und nie als vollständig geprüft gemeldet.

Der Prüfbericht sagt `OFFLINE_SUPPLIED_RESPONSES` und
`publication_performed=false`. Ein synthetisches PASS belegt keine
Remote-Frische, Quellechtheit, produktive Veröffentlichung oder fachliche
Abnahme. Die Repository-ID und gepinnten OIDs sind Vertrauensanker des
Aufrufers. Ein einzelnes statisches Bündel kann keinen realen zeitlichen
HEAD-Verlauf beweisen; Sequenz-/Racefälle werden mit injizierten Antworten
getestet. Der Prüfer verwendet niemals eine Mutationsantwort als Objektbeleg.

`simulate_graphql` ist ausschließlich ein injiziertes synthetisches Modell.
Es prüft vorab Objekte und Baseline, ruft den Testtransport höchstens einmal
auf und prüft danach über den separaten Leser erneut. Jede unklare Antwort
oder spätere Prüfstörung ergibt `MutationUnknown`. Keine automatische
Wiederholung und kein Rollback. Der Aufrufer darf in diesen Testvertrag
keinen produktiven Transport einsetzen.

last_verified bleibt bei jeder Statusfortsetzung unverändert. Runner,
Upload, technisches Ergebnis und fachliche Akzeptanz werden getrennt
übernommen; keine Kombination wird automatisch als Checkpoint qualifiziert.
Evidenznamen und Statusangaben werden strukturell geprüft, ihre tatsächlichen
Originaldateien werden dabei nicht automatisch gelesen oder beglaubigt.

Die Schemaprüfung bildet auch den abschließenden LF-Treffer des Schema-Regex
`$` ab; die fachliche OID-Prüfung lehnt ihn strikt ab. Der unabhängige
Python-Schemaoracle benötigt einen ergänzten stdlib-Datumschecker, weil
das installierte jsonschema keine optionale RFC3339-Implementierung besitzt.
Schaltsekunden sind durch Rust-Grenztests, nicht durch diesen Oracle geprüft.

Weitere Arbeit und externe Abschlussqualifikation:
[BRIDGE-CODEX-PROGRESS.md](BRIDGE-CODEX-PROGRESS.md).


## Variante A: Payload und separater Beleg

`publication.state=NOT_PUBLISHED` und `publication.commit=null` sind
Statuswerte der Exchange-v1-Payload, keine Aussage über eine spätere externe
Verifikation des umschließenden Git-Commits. Diese Felder werden weder
umbenannt noch nachträglich umgeschrieben. Eine Feldbelegung CONFIRMED
ist kein Veröffentlichungsnachweis. Fachliche Angaben bleiben unbeglaubigt.

Der neue Rust-Vertrag `attestation::OfflineAttestation` ist ausdrücklich vom
Python-Vertrag `publisher.validate(remote=True)` getrennt. Letzterer verlangt
historisch CONFIRMED/commit und wird hier nicht verändert. Der neue Typ ist
nur durch `verify_snapshot`/`verify_bundle` konstruierbar: Repository-ID,
Ref/HEAD vor und nach dem Readback, rohe Commitbytes, vollständige gehashte
Trees und exakte Blobbytes werden geprüft. Der Beleg enthält die Originalbytes,
Blob-/Commit-OID und SHA256; keine Umserialisierung der geprüften Payload.
`professional_acceptance=NOT_ATTESTED` bleibt unabhängig von Nutzdaten.

`verify-attestation FILE` erwartet ein striktes Offline-Bündel mit genau:
`repository_id`, `commit`, `expected_bytes_base64`, `responses`, `raw_commits`,
`recorded_at_utc`, `conditions`. REST-/Rawcommit-Mappings entsprechen dem
oben beschriebenen Format; HEAD muss `commit` sein. Erwartete Bytes müssen
dem unveränderten lokalen Exchange-v1-Payloadvertrag entsprechen, dürfen aber
einen bestehenden Checkpoint enthalten. Zeitpunkt wird als Exchange-v1-
Datum validiert; Bedingungen müssen nichtleer und höchstens 4096 UTF-8-Bytes
sein. Beide sind ausdrücklich **CALLER_SUPPLIED**, keine unabhängig
beglaubigte Uhrzeit oder Beschaffungsprovenienz. Der Beleg bestätigt allein
die geprüften Relationen im Offline-Bündel und bleibt
OFFLINE_SUPPLIED_RESPONSES/BRIDGE_PUBLICATION=BLOCKED. Authentische externe
Readback-Evidenz und deren unabhängige Abnahme bleiben ein separater Schritt.

## Existierenden Statusindex fortschreiben

`prepare-update-objects FILE` nutzt das `prepare-objects`-Bündel mit dem
zusätzlichen Pflichtfeld `baseline_base64`. Es enthält die exakten alten
Blobbytes, einschließlich ursprünglicher Whitespace-/Newlinebytes.
Der alte Blob muss separat in `responses` enthalten sein. Die Bytes werden
gegen Blobantwort sowie gehashten Baseline-Commit/Tree geprüft. Gleichwertiges
JSON mit anderen Bytes ist keine passende Baseline. Der Kandidat wird mit
dem bestehenden Statusvertrag kanonisch vorbereitet; `last_verified` bleibt
unverändert. Alle Dateien und Verzeichnisse außer dem festen Statusziel
bleiben identisch; der Zielmodus ist 100644. HEAD muss vor und nach der
Planung `before` entsprechen. Ausgabe enthält ausschließlich Objektbytes,
OIDs und CAS-Entwurf, keine Mutation.

Bibliotheksfunktionen `prepare_update_commit`, `verify_update_index` und
`simulate_update_graphql` unterstützen Vorbereitung, unabhängige Prüfung der
vollständigen Transition und synthetische CAS-Fälle. Der Snapshotbeleg allein
prüft keine Transition und ersetzt deren Checkpoint-/Änderungsprüfung nicht.
`verify-offline` behält seinen bestehenden Initialvertrag.
`verify-update-offline FILE` nutzt dessen sechs Pflichtfelder plus
`baseline_base64` mit exakten alten Blobbytes. Zusätzliche oder fehlende Felder
werden abgewiesen. Der Adapter bereitet den Kandidaten mit Checkpointerhalt
vor und prüft anschließend die gesamte Transition über `verify_update_index`: beide
Originalcommits, Parentlink, alte/neue Trees und Blobs, unveränderte Nebenobjekte
und HEAD vor/nach der Prüfung. Fehlende Rohbytes/Antworten werden niemals ersetzt.
Der Bericht ist ein Transitionsprüfergebnis, kein Snapshot-Verifikationsbeleg. Die synthetische Mutation
hat höchstens einen Versuch; Konflikt, Timeout, ungültige Antwort und spätere
HEAD-Races führen zu Konflikt bzw. MutationUnknown ohne Retry oder Rollback.

## Lokaler Provenienz- und Mergevertrag

`sources::merge(previous, sources, selected_log)` ist eine reine Bibliotheks-API.
Jede Quelle erhält eine eindeutige caller-supplied ID, eine Rolle (`RunnerLog`,
`GitObservation`, `ImageInventory`), Originalbytes und optional Sidecarbytes.
256 Quellen und zusammen 32 MiB Eingangsbytes sind erlaubt. Doppelte/leere IDs
werden abgewiesen. Das Ergebnis enthält getrennt `payload` und `provenance`;
der lokale Bericht `LOCAL_SOURCE_MERGE_V1` ist ausdrücklich kein Exchange-v1-
Dokument und kein `OfflineAttestation`-Beleg. Er bewahrt Originalbytes und Sidecar
als base64, SHA256, Rolle, ID und die direkt daraus ermittelten Prüfergebnisse.
Ein berechneter Hash belegt keine unabhängige Quellechtheit. Git-/Inventur-
Integrität bleibt UNKNOWN; Sidecars werden nur für Runnerlogs ausgewertet.

Runnerlogs werden erneut aus Originalbytes geprüft. Die explizite Auswahl
per Quellen-ID ersetzt Pythons collect-Sortierung nach mtime/Name und deren
Limit auf zwölf fertige Logs. Es findet keine automatische Neueste-Auswahl statt.
Alle übergebenen Quellen werden geprüft; fehlerhafte Quellen werden nicht durch
Sortierung oder Limit versteckt. Unterschiedliche Logbytes derselben Blockkennung
führen zu REVIEW_REQUIRED und unterdrücken Runner-PASS des betroffenen Blocks.
Identische Originalbytes mehrerer IDs bleiben getrennt erhalten. Fehlende Auswahl
oder fehlende Blockkennung erhält den bisherigen latest_block, mit Reviewgrund.
Bei kohärentem ausgewähltem Log wird ausschließlich latest_block geändert;
Upload, Technik und Fachabnahme bleiben UNKNOWN. Alle anderen Payloadfelder,
insbesondere last_verified, build und current, bleiben unverändert.

GitObservation erwartet genau die vier snapshot.git-Schlüssel head, branch,
worktree, remote_sync; head muss ein strikter OID sein und remote_sync UNKNOWN.
Dies ist eine übergebene Beobachtung, keine eigene Git-Erhebung. ImageInventory
nimmt die Original-JSON-Datei an: record_count, unique_file_hashes, host_counts,
scan_warnings und path_conflicts werden typgeprüft. Widersprüchliche Zähler,
Warnungen oder Pfadkonflikte verlangen Review. Weitere Originalfelder bleiben
als Quelldaten erhalten, werden aber nicht interpretiert oder in Exchange-v1
projiziert. Unterschiedliche Beobachtungen derselben Rolle bleiben erhalten und
führen zu REVIEW_REQUIRED; keine Last-wins-Auswahl. Fehlende/ungültige Beobachtungen
werden ebenfalls markiert. Der Bericht leitet keine Git-/Image-/Log-Kausalität ab.

Lite snapshot selbst ist eine abgeleitete Zusammenfassung und wird nicht als
Originallog akzeptiert oder als unabhängig verifiziert importiert. Dessen
snapshot_id und Zähler ersetzen keine Originalbelege. Der unten beschriebene CLI-Dateibündeladapter verbindet diese API mit
ausdrücklich ausgewählten Originaldateien. Es gibt keine automatische
Verzeichnisbeschaffung, authentifizierte Erhebung oder verifizierte Hardware-/
Imagezuordnung. Der explizite lokale End-to-End-Pfad ist synthetisch geprüft;
die unabhängige Runner-Abnahme bleibt offen.

## Explizite lokale Quellen bis zum vorbereiteten Status

`prepare-local-sources FILE` (auch stdin `-`) liest ein striktes Manifest mit
exakt `contract`, `previous`, `selected_log`, `sources`. `contract` ist
`LOCAL_SOURCE_FILES_V1`, `previous` ein lokales vollständiges Exchange-v1-
Dokument, `selected_log` die ausdrücklich gewählte Runner-Quellen-ID.
`sources` enthält 1..256 Einträge mit genau diesen Feldern:

```json
{
  "id": "SYNTHETIC.txt",
  "kind": "RunnerLog",
  "path": "/explicit/synthetic/SYNTHETIC.txt",
  "sha256": "expected lowercase SHA256 of original bytes",
  "sidecar_path": "/explicit/synthetic/SYNTHETIC.txt.sha256",
  "block_id": "SYNTHETIC"
}
```

Mindestens eine Quelle jeder Rolle `RunnerLog`, `GitObservation` und
`ImageInventory` ist erforderlich. Git-/Inventurquellen verwenden dieselben
sechs Felder mit `sidecar_path=null` und `block_id=null`; Originalformate
entsprechen dem Mergevertrag oben. SHA256 ist für jedes Original Pflicht.
Die Runner-ID muss denselben Basisnamen wie der Originalpfad haben. Sidecars
werden ausschließlich über den expliziten Pfad gelesen; `null` bedeutet
fehlender Integritätsbeleg, ein nicht lesbarer benannter Pfad ist ein harter
Fehler. Es gibt keine implizite Nachbar-, Verzeichnis- oder Neueste-Auswahl.
Manifestlimit 1 MiB, Originale plus Sidecars zusammen 32 MiB, Sidecars 8 KiB.

Die Ausgabe ist eine einzelne JSON-Hülle mit getrennten `payload` und
`provenance`. Nur `payload` ist Exchange-v1 und kann an die vorhandenen
Status-/Objektplaner weitergegeben werden. Der Provenienzbericht enthält
LOCAL_SOURCE_MERGE_V1 sowie unter `file_selection` die exakten Manifestbytes
als base64 und ihren SHA256: Pfade, Rollen, Auswahl und erwartete Hashes
bleiben dadurch erhalten. Er ist weder Statuspayload noch Attestation.
Keine Dateien werden geschrieben; auch ein Review-Ergebnis ist keine
Fachabnahme. Ein vollständiges synthetisches Beispiel einschließlich CLI
steht in `tests/local_sources.rs`.

Dateiquellen müssen absolute reguläre Pfade ohne Symlinkkomponenten sein.
Der Adapter unterstützt Linux mit O_NOFOLLOW/O_NONBLOCK beim finalen open;
konkurrierender Austausch von Elternverzeichnissen erfordert weiterhin eine
externe vertrauenswürdige Dateisystemumgebung. Andere Plattformen werden
abgewiesen. Inhalt und Sidecars werden einmal begrenzt gelesen und genau
gegen diese Bytes geprüft. Harte Fehler liefern keinerlei Teil-stdout.

Erwartete Hashes und Blockkennungen binden Bytes an die explizite Auswahl des
Aufrufers. Sie authentifizieren weder den Aufrufer noch eine reale Git-/
Inventur-/Imagezuordnung. Rollenverwechslungen und widersprüchliche Angaben
werden zurückgewiesen oder als Review bewahrt; fachlich unentscheidbare
Zuordnungen bleiben UNKNOWN. Insbesondere entsteht daraus kein Build- oder
Hardwareerfolg und keine Checkpointpromotion.

Der frühere Hinweis auf den fehlenden CLI-Quellenbündeladapter beschreibt den
Stand vor Run 217. Der explizite lokale End-to-End-Entwicklungsweg ist jetzt
vorhanden; automatische Verzeichnisbeschaffung ist kein erforderlicher Teil
dieses Vertrags. Unabhängige Runner-Abnahme bleibt separat offen.

## Optionaler Shadow Adapter

Der Shadow Adapter wird ausschließlich manuell, optional und lesend aufgerufen:

```bash
bash tools/novena-bridge/shadow-inspect.sh /absolute/path/to/RUNNER.txt
```

`NOVENA_BRIDGE_BINARY` kann optional den Pfad zur ausführbaren Rust-CLI
vorgeben; der aktuelle `/tmp`-Fallback ist nur ein temporärer Entwicklungspfad.
Die Eingabe muss eine reguläre Datei und darf kein Symlink sein. Einen
optionalen Original-SHA256-Sidecar (`RUNNER.txt.sha256`) behandelt die Rust-CLI.
Die ausgegebenen Berichtstatus bewahren die Bedeutung von `UNKNOWN` und
`REVIEW_REQUIRED`. Ein Runner-/Prozess-`PASS` ist keine technische oder
fachliche Abnahme. Fehler-Exitcodes müssen beachtet werden; ein erfolgreicher
Adapteraufruf allein bedeutet ebenfalls keine Abnahme.

Die fünf gezielten Laufzeitprüfungen in Block `NOVENA-BRIDGE-RUST-250`
ergaben `PASS`. Es gibt keine Integration in `run-and-upload.bash`, keine
Veröffentlichung und keine Promotion von `last_verified`. Die produktive
Abnahme bleibt gesperrt.

## Manuelle Statuserstellung und Anzeige

Im Repository-Hauptverzeichnis `NOVENA_BRIDGE_BINARY` auf den absoluten Pfad
des vorhandenen geprüften Rust-Executables setzen. Die Platzhalter in den
folgenden Befehlen sind durch die tatsächlichen Pfade und Hashwerte zu ersetzen:

    export NOVENA_BRIDGE_BINARY=/absoluter/pfad/zum/geprueften/novena-bridge
    bash tools/novena-bridge/prepare-status.sh \
      ABSOLUTES_RUNNERLOG ERWARTETER_LOG_SHA256 NEUES_ABSOLUTES_ZIELVERZEICHNIS

Das Runnerlog muss eine `.txt`-Datei mit passendem benachbartem
`.txt.sha256`-Sidecar sein. Der Preparer verbindet das explizit gewählte
Runnerprotokoll mit der lokalen Git-Beobachtung und Image-Inventur. Er erzeugt
`source-merge.json` samt Provenienz in einem neuen privaten Zielverzeichnis
(Verzeichnisrechte `0700`, Dateirechte `0600`); dessen Elternverzeichnis muss
bereits existieren. Die Ausgabe ist eine historische lokale Vorschau.
Exitstatus `3` bedeutet `REVIEW_REQUIRED`: Die Vorschau bleibt erhalten,
eine Freigabe entsteht daraus nicht.

Anschließend den Viewer separat manuell aufrufen. Als zweiten Parameter den
vom Preparer unter `SHA256=` ausgegebenen Hash der erzeugten
`source-merge.json` angeben, nicht den Hash des Runnerlogs:

    bash tools/novena-bridge/bridge-status.sh \
      NEUES_ABSOLUTES_ZIELVERZEICHNIS/source-merge.json SHA256_DER_ERZEUGTEN_DATEI

Keine automatische Aktualisierung, Runner-Integration, Checkpoint-Promotion
oder Veröffentlichung; technische und fachliche Abnahme bleiben `UNKNOWN`.

## Privater Status-Viewer

Der manuell aufrufbare Viewer `bridge-status.sh` liest eine bereits
vorbereitete lokale Statusvorschau. Er erzeugt keine neuen Statusdaten,
führt keine Git- oder Hardwareaktionen aus und veröffentlicht nichts.

Manueller Aufruf auf foobox:

    cd ~/nixos-novena-image
    bash tools/novena-bridge/bridge-status.sh \
      "$HOME/novena-backups/bridge-local/preview-253/source-merge.json" \
      6adb858d12d9c5db4642ecf8cab36c2be84549cb1ed607c4a9a15578dc174dc8

Der erwartete SHA256-Wert muss aus einem zuvor bestätigten Nachweis
stammen. Das Werkzeug prüft Dateiidentität, restriktive Zugriffsrechte,
JSON-Struktur und Provenienz vor der Ausgabe ausgewählter Statusfelder.

Die Ausgabe ist ausdrücklich eine historische lokale Vorschau,
kein automatisch aktualisierter Projektzustand und keine Freigabe.
UNKNOWN und REVIEW_REQUIRED bleiben erhalten.

Der gezielte Funktionstest NOVENA-BRIDGE-RUST-256 bestand vier von
vier Fällen. Der Viewer ist nicht in run-and-upload.bash integriert.
Produktive Veröffentlichung und automatische Checkpoint-Promotion
bleiben gesperrt.
