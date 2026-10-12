# Verbindlicher Übersetzungsworkflow

## Geltung und Rollen

Dieses Verwaltungsdokument regelt die Dokumentation, keine Aktionsfreigaben.
Das deutsche Original ist bei einer Übersetzung maßgeblich; die englische
Übersetzung ist ihm untergeordnet. Ein englisches Originaldokument hat keine
erfundene deutsche Quelle: `original_language=en`; eine spätere deutsche
Übersetzung wäre diesem englischen Original untergeordnet. Ein englisches
Einstiegs-/Begleitdokument dient der Navigation und Einordnung und ist ohne
belegte Zuordnung keine Übersetzung eines deutschen Dokuments.
Ein historischer Snapshot ist eine fest an einen konkreten Quellstand gebundene
Abbildung. Ein Entwurf ist ausdrücklich unvollständig oder noch in Bearbeitung.

Dokumentklassen werden getrennt geführt: `maintained_document` bezeichnet
fortgeführte Dokumentation, ohne jede darin enthaltene Aussage als aktuell zu
bestätigen; `historical_analysis` eine historische Untersuchung;
`historical_reference` eine historische Referenz; `draft` einen Entwurf;
`snapshot` einen konkret gebundenen historischen Snapshot. `historical` beschreibt
das Dokument als Ganzes, nicht einzelne alte Aussagen. Unbekannte Eigenschaften
werden mit `null` erfasst. Ein altes Inhaltsdatum allein beweist keinen Snapshot.
Rolle und Dokumentklasse sind getrennte Angaben.

## Statusachsen

Die Achsen sind unabhängig; Dateiexistenz beweist weder Synchronität noch Qualität.

| Achse | Wert | Verbindliche Bedeutung |
|---|---|---|
| translation_status | MISSING | Zielübersetzung fehlt. |
| translation_status | DRAFT | Ziel ist als Übersetzungsentwurf eingeordnet. |
| translation_status | PRESENT | Zielübersetzung existiert; keine Qualitätsaussage. |
| synchronization_status | UNKNOWN | Synchronität für den gegenwärtigen Stand ist nicht ausreichend belegt. |
| synchronization_status | CURRENT | Vollständiger Bedeutungsabgleich belegt Synchronität für exakt das aktuelle Quell-/Zielhashpaar; technische Teilprüfungen reichen nicht. |
| synchronization_status | STALE | Ziel bezieht sich nachweislich auf einen überholten lebenden Quellstand. Eine bloße Hashänderung ohne Bedeutungsabgleich reicht für diesen Befund nicht. |
| synchronization_status | TARGET_MODIFIED | Zielbytes unterscheiden sich vom dokumentierten geprüften oder übernommenen Zielstand; Nutzerarbeit und Konflikt müssen geprüft werden. |
| verification_status | VERIFIED | Technische und vollständige semantische Prüfung desselben exakten Hashpaars dokumentiert. |
| verification_status | PARTIALLY_VERIFIED | Nur begrenzte Prüfungen dokumentiert; Umfang und Lücken ausdrücklich angegeben. |
| verification_status | NOT_VERIFIED | Keine ausreichende dokumentierte Prüfung für diesen Stand. |
| verification_status | NOT_APPLICABLE | Übersetzungsprüfung auf ein eigenständiges Original oder Begleitdokument nicht anwendbar; keine Aussage über dessen fachliche Qualität. |

Bei zugleich geändertem Original und Ziel hat `TARGET_MODIFIED` Vorrang;
die Quelländerung wird zusätzlich dokumentiert. Unbekannte historische
Provenienz bleibt `null`.

## Hash- und Reviewregeln

SHA-256 wird über rohe Dateibytes ohne Normalisierung berechnet, einschließlich
Zeilenenden und Kodierung. Jedes Prüfergebnis wird an Quell- UND Zielhash
gebunden. Ein geänderter Quellhash macht die bisherige Synchronitätsprüfung
für den aktuellen Stand unzureichend. Ein geänderter Zielhash macht die bisherige
Zielprüfung unzureichend. Neue Prüfungen werden als neue Revieweinträge angefügt;
alte Prüfhistorie darf niemals mit neuen Hashes umgeschrieben werden.
Hashgleichheit oder ein gespeicherter Hash beweist Byteidentität, keine
semantische Korrektheit. Aktuelle Datei-Hashes sind von Review-Hashes getrennt.

`VERIFIED` ist für eine Übersetzung nur zulässig, wenn für exakt dasselbe
Quell-/Zielhashpaar mindestens technische Prüfung UND vollständige semantische
Prüfung mit Verfahren, Umfang, Datum, Ergebnis und Beleg dokumentiert sind.
Die semantische Prüfung umfasst alle relevanten Aussagen, insbesondere
Negationen, Ursache und Folge, belegt gegenüber hypothetisch, Unsicherheit und
Aussagegrenzen, historische gegenüber aktuellen Aussagen, Testbedingungen sowie
Zahlen und Einheiten in ihrem jeweiligen Aussagekontext. Auslassungen sind zu
prüfen. Regex-, Hash-, Token-, Struktur-, Markdown- oder Codeblockprüfungen allein
dürfen niemals `VERIFIED` begründen. Reviewstatus und Synchronität sind gesondert
zu entscheiden. Fehlende Belege oder unbekannte Prüfdetails bleiben `null`;
rekonstruierte Prüfungen werden ausdrücklich als Rekonstruktion gekennzeichnet.
Dokument-Inhaltsdaten sind keine Übersetzungsdaten.

## Nutzerarbeit und geschützte Inhalte

Bestehende Übersetzungen niemals ungefragt überschreiben. Bei geändertem Ziel
`TARGET_MODIFIED` verwenden, vorhandene Nutzerarbeit erhalten und Unterschiede
prüfen. Keine automatische Komplett-Neuübersetzung als Konfliktlösung.
Das Glossar unterstützt kontextbezogene Entscheidungen und definiert keine globale
automatische Ersetzungslogik.

Dateipfade, Befehle, Hashes, Commit-IDs, Registerbezeichnungen, Variablennamen,
Device-Tree-Properties, technische Labels, technische Identifikatoren,
Rohlog-Ausgaben und Originalzitate dürfen grundsätzlich nicht sprachlich verändert
werden, soweit sie technische Identität tragen. Erläuterungen dürfen außerhalb
dieser Inhalte übersetzt werden. Zitate bleiben Originalzitate; eine zusätzliche
Übersetzung ist gesondert als solche zu kennzeichnen.

## Historische Dokumente und englische Originale

Snapshots an konkrete Quellbytes, Hash und belegte Quellidentität binden.
Spätere Änderungen eines lebenden Dokuments machen einen korrekt gebundenen
historischen Snapshot nicht automatisch veraltet. Historische Aussagen nicht
stillschweigend aktualisieren. Historische offene Root-Cause-Aussagen gehören zu
ihrem damaligen Untersuchungsstrang und dürfen nicht auf später abgeschlossene
Untersuchungen übertragen werden. Rollen, Historie und heutige fachliche Gültigkeit
sind getrennt zu beurteilen.
Für englische Originale keine deutsche Quelle erfinden. `original_path` bezeichnet
dann die englische Datei selbst; `original_language=en`, keine künstliche Paarung.

## Verbindliche Abläufe

1. Neues deutsches Dokument: Rolle, Klasse und Historie prüfen, Original als
   `original_language=de` registrieren; geplante fehlende Übersetzung als `MISSING`,
   Synchronität `UNKNOWN`, Prüfung `NOT_VERIFIED` erfassen. Nur autorisierte Dateien schreiben.
2. Erstmalige englische Übersetzung: Quellhash erfassen, geschützte Inhalte erhalten,
   Entwurf als `DRAFT` führen. Nach Fertigstellung `PRESENT`; Quell-/Zielhashpaar
   prüfen und Reviews dokumentieren. `CURRENT` und `VERIFIED` nur nach ihren Regeln.
3. Geändertes deutsches Original: aktuellen Hash getrennt aktualisieren, alte Reviews
   erhalten, Synchronität zunächst `UNKNOWN`; durch Bedeutungsabgleich erforderliche
   Anpassungen feststellen. Nachgewiesene Überholung als `STALE` dokumentieren.
4. Veraltete englische Übersetzung: Unterschiede gegen den belegten alten und neuen
   Quellstand prüfen; autorisierte gezielte Überarbeitung unter Erhalt der Nutzerarbeit.
   Neue Hashes und neue Reviews; ohne ausreichende Prüfung kein `CURRENT`/`VERIFIED`.
5. Geändertes englisches Ziel: `TARGET_MODIFIED`, Nutzeränderungen erhalten und
   Konflikte dokumentieren. Nach autorisierter Zusammenführung neue Hashpaarprüfung;
   keine automatische vollständige Neuerstellung.
6. Historischer Snapshot: konkreten Quellstand samt Hash und Herkunft dokumentieren,
   `snapshot` und `historical=true`; Synchronität gegen die gebundene Quelle beurteilen,
   nicht gegen das später geänderte lebende Dokument. Fehlende Quellbindung offenlassen.
7. Englisches Originaldokument: englische Datei als eigenes Original registrieren,
   keine deutsche Quelle; Übersetzungsprüfung `NOT_APPLICABLE`. Eine spätere deutsche
   Übersetzung wird nach denselben Hash-/Reviewregeln dem englischen Original untergeordnet.

## Publikationsgrenze

Übersetzungs- und Prüfstatus erteilen keinerlei automatische Freigabe für
Publikation, Commit, Push, Build, Hardwaretest oder SD-Karten-Schreibzugriff.
Repositoryregeln und der konkrete Auftrag bestimmen den erlaubten Umfang.

## Initiale Evidenzgrenze

Die fünf bestehenden Paare werden nach Auftrag als `PRESENT`, `UNKNOWN` und
`PARTIALLY_VERIFIED` aufgenommen. Der Reviewverweis `block-4.10o-1n-8q` und das Datum
2026-10-01 stammen aus dem Auftrag. Ein dauerhaft archivierter 8Q-Prüfbericht fehlt;
`evidence_ref=null`. Konkrete historische technische Prüfschritte sind hier nicht
rekonstruierbar und werden nicht erfunden; `semantic_review=NOT_VERIFIED`.
Die in 8R durchgeführten Datei-, Hash- und JSON-Prüfungen ersetzen keine semantische
Prüfung und sind getrennt vom 8Q-Review dokumentiert.

## PRE-WRITE-Checkpoint Block 4.10O-1N-8R

Der folgende vor der ersten Schreiboperation erhobene Checkpoint dokumentiert
Ausgangszustand und Schutz-Hashes; bestehende Änderungen sind keine Änderungen durch 8R.

```json
{
  "pwd": "/home/loomit/nixos-novena-image",
  "head": "6dca698a37b27283e92fc823a4b6869d441beac0",
  "status": " A boot/u-boot/0004-novena-uboot-diag-d0-d36.patch\n A boot/u-boot/0005-novena-diag-d21a-d21d.patch\n A boot/u-boot/0006-novena-diag-brk-p0-p1.patch\n A boot/u-boot/0007-novena-diag-brk-q1-q2.patch\n A boot/u-boot/0008-novena-diag-control-c0-c2.patch\n M boot/u-boot/default.nix\n M docs/DECISIONS.md\n M docs/PROJECT-STATE.md\n M docs/TEST-LOG.md\n?? AGENTS.md\n?? docs/QUELLENVERZEICHNIS.md\n?? docs/en/README.md\n?? docs/en/docs/BUILD-HOST.md\n?? docs/en/docs/BUILD.md\n?? docs/en/docs/GOLDEN-BUILD.md\n?? docs/en/docs/IMAGE-LAYOUT.md\n?? docs/en/docs/QUELLENVERZEICHNIS.md\n?? test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log\n?? test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log\n?? test-logs/2026-10-01/block-4.10o-1n-8e-decisions.entwurf.md\n?? test-logs/2026-10-01/block-4.10o-1n-8e-project-state.entwurf.md\n?? test-logs/2026-10-01/block-4.10o-1n-8e-test-log.entwurf.md\n?? test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log\n?? test-logs/2026-10-01/decisions.vor-8e.md\n?? test-logs/2026-10-01/project-state.vor-8e.md\n?? test-logs/2026-10-01/test-log.vor-8e.md\n?? test-logs/p-ext-2026-09-22/p-ext-boot-01.log\n?? test-logs/p-ext-2026-09-22/p-ext-diag-boot-01.log\n?? test-logs/p-ext-2026-09-23/block-9h-build.log\n?? test-logs/p-ext-2026-09-23/p-ext-d0-d36-boot-01.log\n?? test-logs/p-ext-2026-09-23/p-ext-d21a-d21d-boot-01.log\n?? test-logs/p-ext-2026-09-23/p-ext-p0-p1-boot-01.log\n?? test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log\n",
  "target_existence": {
    "docs/translation-manifest.json": false,
    "docs/translation-glossary.json": false,
    "docs/TRANSLATION-WORKFLOW.md": false
  },
  "protected_sha256": {
    "docs/BUILD-HOST.md": "177912b15172f9e4aad593eea44b58b981a4aec437aa58ffb56fcd24dedbff1f",
    "docs/en/docs/BUILD-HOST.md": "562ad5907f7d426c1282469ea571f05f8abc86d641fb75dc66dda0ed84816586",
    "docs/BUILD.md": "14553d1330b03fbcdd159ef3cd49b86bf8edfaba0974151393631f2f8499e0f5",
    "docs/en/docs/BUILD.md": "d374c37d803de8557f7a465d55c642c485921472a49b3af5bc740602b0f842ee",
    "docs/GOLDEN-BUILD.md": "09987307399c28a48a27f7d0a8284838ef5c2339d6cebb30d7da1303e6670669",
    "docs/en/docs/GOLDEN-BUILD.md": "ae94738ec880752a9ee0978a6906bb2e58c9680a9221257169bf187799648ab0",
    "docs/IMAGE-LAYOUT.md": "89aad123a437c43bd81678e5b067c8d6877514ed18ef6b3ef123b088cc3d839c",
    "docs/en/docs/IMAGE-LAYOUT.md": "a797b064daa3a98aa914f5b32ad1928e58afd5bcda926b971631e59c7f66152b",
    "docs/QUELLENVERZEICHNIS.md": "b0ef32b35336fbb65b0587c6b24d9925362e2716d19d35237b3957bb2307e8b8",
    "docs/en/docs/QUELLENVERZEICHNIS.md": "1b4bffb99ddc42c85fd1eb7d2987e885535b4b0a108b18255dbf62e1ef04fb52",
    "docs/en/README.md": "0b6b1e9c90925974174351454fa792357bd3535b37ed78e461142bc2e3e31e71",
    "research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md": "64d647ced6e6f9cc54073b7df61414b8ee0b26c20458c6264f6696fbb3bdc0d1"
  }
}
```
