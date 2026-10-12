# Novena-Projektindex v0.3 – INDEX-1K-15 (Teilimplementierung)

Der Index ist eine Verweisschicht und ersetzt keine Primärquelle. Er enthält
18 Quellen, neun Artefakte und neun Quellen-Artefakt-Beziehungen. Die ersten
fünf Quellen sowie `source-relations.tsv`, `findings.tsv` und
`evidence-links.tsv` sind unverändert aus v0.2 erhalten.

`sources.tsv` trennt die Identität (`namespace`, `original_id`) von der
Registerprovenienz (`record_locator`, `record_sha256`). Registerhashes gelten
für die ganze Datei. `sha256` bezeichnet ausschließlich lokale Originalbytes;
leer bedeutet unbekannt, nicht Null-Hash. Externe URLs werden niemals abgerufen.

SPL-001/002 sind `artifact_reference` beziehungsweise `artifact_collection`:
Ihr Quellenlocator verweist auf den Registereintrag, ihre Originalbytes stehen
in `artifacts.tsv`. `source-artifacts.tsv` ordnet mit `PRIMARY` das ELF und mit
`COMPONENT` die acht weiteren Dateien zu. SPL-004 ist das lokale Patchoriginal
und wird zusätzlich zu seinem Registereintrag per SHA256 geprüft.
SPL-005 (`missing_original`), SPL-006 (`primary_source_gap`) und SPL-007
(`historical_chat`) haben keine hier registrierten Originalartefakte. Die
fehlenden Originalberichte und lokale Originalausgabe bleiben Lücken.
SPL-006 beschreibt den historischen Registerumfang; spätere datierte Nachträge
im Quellenverzeichnis präzisieren inzwischen dokumentierte Herstellerbelege.

Artefaktlocator haben ausschließlich die Form `REF_LOCKED_DEBUG/relativer/pfad`.
Der Alias wird nur durch `--ref-locked-debug-root` aufgelöst; es gibt keinen
voreingestellten Nix-Store-Pfad. Absolute Pfade, Traversal und Symlink-Ausbrüche
werden abgewiesen. Die Originalprüfung prüft Größe, SHA256 und die lokale
Provenienzdatei einschließlich referenzierter Zeilen. Strukturprüfung benötigt
keine verfügbaren Originaldateien oder Referenzwurzel.

Quellenstatus bleiben die v0.2-Werte `HISTORICAL_CATALOG_CHECK`,
`LOCAL_HASH_VERIFIED`, `CONTENT_UNAVAILABLE`, `DOCUMENTED_READ_SCOPE`.
Artefaktstatus sind `HISTORICAL_HASH_VERIFIED` und `UNVERIFIED`. Statuswerte
beschreiben dokumentierte historische Qualifikation; ein Validatorlauf schreibt
sie nicht um. `verified_at` bleibt bei unbekanntem Verifikationsdatum leer.
Alle Artefakte benötigen einen Identitäts- und Provenienzhash, auch bei
`UNVERIFIED`; der Status allein behauptet keine aktuelle Prüfung.

`SRC-000005` verweist als Registerprovenienz auf `05-PROVENIENZMATRIX.md#L19`.
Die EB817-Identifizierung beruht zusätzlich auf `01-NXP-ORIGINALQUELLEN.md#L13`.
Die beiden N07 verschiedener Namensräume sind unterschiedliche Dokumente;
`DIFFERENT_FROM` ist kein Alias. `PROVEN` gilt nur im genannten Finding-Scope.

## Verbindlicher Projektvertrag

`baseline-contract.json` ist ein unabhängig festgelegter Sollbestand mit Version
`novena-project-index-contract-1`. Die Werte wurden einmalig anhand der expliziten
INDEX-1K-12-Bindungen, der vorhandenen v0.3-Evidenz, des Quellenregisters und des
hashverifizierten historischen TAR festgelegt. Beim Prüfen werden keine Sollwerte
aus den gerade geprüften TSV-Tabellen abgeleitet. Der Validator bindet zusätzlich
den gesamten Vertrag über `CONTRACT_SHA256`; fehlende, beschädigte oder unbekannte
Verträge führen zu FAIL. Es gibt keine automatische Regeneration oder Reparatur.
Eine bewusste Vertragsänderung erfordert Prüfung und Änderung des Vertrags sowie
seines Pins im Validator.

Der Vertrag verlangt ART-000001 bis ART-000009 mit exakten Typen und Locatoren
sowie SAL-000001 bis SAL-000009 mit exakten IDs, Quellen, Artefakten und Rollen.
SAL-000001 bindet SRC-000009 an ART-000001 als PRIMARY; die weiteren acht binden
SRC-000010 an die entsprechenden Artefakte als COMPONENT. Fehlende Bestände
werden auch erkannt, wenn zusammen mit einem Artefakt alle darauf verweisenden
Zuordnungen fehlen. Leere Tabellen mit gültigem Header bestehen diese Prüfung
nicht. Gültige zusätzliche Quellen, Artefakte und Zuordnungen bleiben zulässig.
Der Vertrag pinnt hier Typen und Locatoren; aktuelle Originalbytes werden nur
bei der gesonderten Originalprüfung gegen Tabellenhashes und Größen geprüft.

`structure()` bleibt die allgemeine Schema-/Strukturprüfung, damit isolierte
Negativtests ihre ursprünglichen Fehlerklassen erreichen. `project_contract()`
prüft danach die Projektpflichtbestände und `register_guard()` die Berechtigung
zum Auslassen von Originalbytes. CLI-Läufe führen beide Prüfungen aus. Auch
`originals()` prüft die Registerberechtigung vor jedem möglichen Auslassen.
`structure()` allein qualifiziert keinen vollständigen Projektindex.

## Registerausnahmen

Nur folgende Identitäten dürfen REGISTER_TYPES-Ausnahmen verwenden:

| Source-ID | Namespace | Original-ID | Typ | Registerlocator |
|---|---|---|---|---|
| SRC-000009 | sources-md | SPL-001 | artifact_reference | docs/QUELLENVERZEICHNIS.md#L60 |
| SRC-000010 | sources-md | SPL-002 | artifact_collection | docs/QUELLENVERZEICHNIS.md#L61 |
| SRC-000012 | sources-md | SPL-005 | missing_original | docs/QUELLENVERZEICHNIS.md#L64 |
| SRC-000013 | sources-md | SPL-006 | primary_source_gap | docs/QUELLENVERZEICHNIS.md#L65 |
| SRC-000014 | sources-md | SPL-007 | historical_chat | docs/QUELLENVERZEICHNIS.md#L66 |

Source-ID, Namespace, Original-ID, Typ, Locator, Registerlocator und Status sind
zusammen gebunden. Reservierte IDs dürfen die Bindung auch durch Änderung des
Typs nicht umgehen. Registerlocator und Quellenlocator müssen übereinstimmen;
ein Originalhash ist nicht erlaubt. Die letzten drei Identitäten verlangen
CONTENT_UNAVAILABLE, die ersten beiden HISTORICAL_CATALOG_CHECK. Historische
SRC-000001 bis SRC-000005 erhalten durch Umklassifizierung keine Ausnahme.
Ein Registerhash verifiziert ausschließlich Registerbytes und niemals ein
fehlendes Original. Zusätzliche Registerausnahmen verlangen eine ausdrückliche
Vertragsänderung; gewöhnliche gültige zusätzliche Datensätze benötigen sie nicht.

`register_guard()` verlangt zusätzlich nach erfolgreicher Identitätsprüfung
`sha256 == ''`. Ein nichtleerer oder anderweitig nicht tatsächlich leerer Wert
führt zu `register source misclassified as original`. Identitätsverletzungen
bleiben `register exception identity unauthorized`. Diese Reihenfolge gilt auch
beim direkten Aufruf von `originals()` ohne vorherige Strukturprüfung.

## Unabhängige historische Regression

`regress_v02.py` liest ausschließlich die Bytes des folgenden lokalen Backups:

`/home/loomit/novena-backups/project-index-v02-before-v03-60785fc2/project-index-v02.tar`

Der erwartete Archivhash ist unabhängig im Vertrag festgelegt:
`18afce33e3e7a0f6cc4851b6c3e3fe7366031c806575ea671454b0c9ad4d4844`.
Die bei der Implementierung gelesene Sidecar-Datei stimmt damit überein; sie
wird zur Laufzeit nicht als Sollwertquelle verwendet. Der Vertrag enthält die
vollständige Mitgliederliste samt Typ, Größe und SHA256. Doppelte Mitglieder,
Links, Sonderdateien, unerwartete Pfade und fehlende Mitglieder werden abgewiesen.
Es erfolgt keine Extraktion. Archiv und Sidecar werden niemals geschrieben.

Oracle ist das unveränderte historische Mitglied
`docs/project-index/validate.py`, 6860 Byte, SHA256
`de1728d058d9e7df94129baba66d969c37ba0e7ce50a00f1b1a18530f6c84af4`.
Erst nach Authentifizierung wird es aus dem Speicher kompiliert. Sein `main()`,
`require()`, seine historischen Regeln, Hashlogik und Pfadregeln bleiben
unverändert. Der Adapter ersetzt lediglich CLI-Eingaben, HERE und Path durch
eine kontrollierte Speicherumgebung und erfasst die Ausgabe. Seine Path-Objekte
stellen Dateiinhalt, Zeilenzahl, Existenz und Pfadauflösung bereit. Die historische
Strukturprüfung wird ausdrücklich nicht durch die aktuelle Implementierung ersetzt.

Die historischen TSV-Mitglieder werden im Speicher geparst. Alle Felder der
SRC-000001–000005, REL-000001, FND-000001–000003 und EVL-000001–000004 müssen im
aktuellen Index erhalten bleiben. Zusätzliche Zeilen sind erlaubt. Insbesondere
führen fehlende SRC-000001 oder SRC-000002 unabhängig von Fremdschlüsseln zu FAIL.

`corpus()` definiert 24 explizite Fälle: Die bisherigen 23 Fälle mit ihren
erwarteten Fehlerklassen bleiben erhalten; hinzu kommt ein isoliertes
Finding-ID-Duplikat (`finding_id: duplicate ID`). Abgedeckt sind historische Struktur, gültige kontrollierte
Originale, ID-Duplikat, Status-, Provenienz-, Hash-, Relations-, Referenz- und
Locatorfehler sowie lokale Originalhashfälle. Der Treiber vergleicht tatsächlich
ausgeführte historische und aktuelle Ergebnisse mit dem jeweils erwarteten
Ergebnis beziehungsweise der erwarteten Fehlerklasse. Die frühere Erkennung eines
fehlenden lokalen Hashes in v0.3 ist eine ausgewiesene Verschärfung. Eine zusätzliche
Kontrolle verlangt historischen FAIL und aktuellen PASS für die ausdrücklich
zugelassene artifact_reference-Registerausnahme. Diese Ausnahme ist eine
Regelerweiterung: die historische Originalprüfung verlangt unverändert einen
Originalhash für jeden lokalen Quellenlocator.

Die Ergebnisadapter fangen ausschließlich die jeweilige `Invalid`-Klasse.
`OSError`, Parser-/Unicodefehler und unerwartete Ausnahmen propagieren und gelten
nicht als erwartete Invalid-Regelfehler. Fehlerklassen werden als vollständige
Meldung oder Meldung mit anschließendem `: `-Identitäts-/Locatorzusatz verglichen;
beliebige Teilstringtreffer reichen nicht aus.

`direct_controls()` ergänzt 33 benannte historische/aktuelle Vergleiche:
je vier Roh-TSV-Parserfälle für die vier historischen Tabellen (gültig, falscher
Header, zusätzliche und fehlende Spalte), zehn direkte Pfadfälle sowie sieben
direkte Originalprüfungen. Gültige Parserresultate werden zusätzlich gegen den
historischen Feldbestand verglichen. Direkte Pfad-/Originalkontrollen führen
keine Strukturprüfung vorab aus, damit deren Verschärfungen entfernte Regeln
nicht verdecken. `run()` verlangt auch diese Kontrollen vor seinem finalen PASS.

BLOCKED: Der vollständige INDEX-1K-14-Plan und seine H01–H35-Matrix lagen im
Implementierungsauftrag nicht als Text oder auffindbare Projektdatei vor.
Die benannten Zusatzkontrollen behaupten daher weder eine Zuordnung zu H01–H35
noch die vollständige Umsetzung dieser Matrix. Die unabhängige Abnahme steht
aus; in INDEX-1K-15 wurden keine Tests ausgeführt.

Dieser endliche Korpus belegt die geprüften Ergebnisvergleiche und keine
vollständige semantische Äquivalenz aller denkbaren Eingaben. Die Speicherumgebung
belegt keine vollständige Nachbildung aller Betriebssystem-/Symlink-Eigenschaften.
Die eigenständige historische Originalprüfung am realen Repository wird hier
nicht als ausgeführt behauptet.

## CLI und Ergebnisstufen

Aus dem Repositoryverzeichnis, ohne Bytecode-Dateien:

```sh
python3 -B docs/project-index/validate.py
python3 -B docs/project-index/regress_v02.py
python3 -B docs/project-index/validate.py --v02-regression
python3 -B docs/project-index/test_validate.py
```

`--index-root` liest einen anderen TSV-Bestand; der unabhängige Vertrag bleibt
projektlokal. `--v02-regression` delegiert an denselben unabhängigen Treiber und
filtert keine Quellen mehr stillschweigend heraus. Die frühere Kombination mit
`--check-originals` wird mit FAIL abgewiesen. Originalprüfung ist ein eigener Lauf:

```sh
python3 -B docs/project-index/validate.py --check-originals --repo-root /home/loomit/nixos-novena-image --ref-locked-debug-root /pfad/zum/verifizierten/build-tree
```

Die Originalprüfung verlangt eine ausdrücklich angegebene Referenzwurzel; sie
führt keine Builds, Downloads, Statusänderungen oder Reparaturen aus. Der
Beispielpfad ist zu ersetzen. Dieser Implementierungsauftrag hat keine reale
Originalprüfung am Referenzbaum ausgeführt.

- `STRUCTURE_CHECK=PASS PROJECT_CONTRACT_CHECK=PASS`: gültige Struktur und vollständige Projektbindungen.
- `ORIGINALS_CHECK=PASS`: gesondert gelesene lokale Register-/Originalbytes stimmen mit Tabellenangaben überein.
- `ARCHIVE_IDENTITY=PASS`: archivweite und vollständige Mitgliederidentitäten stimmen.
- `HISTORICAL_DATA_IDENTITY=PASS`: historischer Feldbestand ist erhalten.
- `HISTORICAL_VALIDATOR_IDENTITY=PASS`: unveränderte Oraclebytes sind authentifiziert.
- `HISTORICAL_RULE_CORPUS=PASS`: explizite historische Ergebnisvergleiche bestehen.
- `REGISTER_EXTENSION=PASS`: ausgewiesene v0.3-Erweiterung hat das erwartete unterschiedliche Ergebnis.
- `V02_REGRESSION=PASS`: alle Regressionsteilstufen und aktuelle Projektprüfung bestanden.

PASS endet mit Exitcode 0, FAIL mit Exitcode 1. Der historische Modus benötigt
lokalen Lesezugriff auf das Backup. Ein fehlendes Archiv führt zu FAIL.

## Tests und Sicherheitsgrenzen

N01–N18, N19–N24 und die sieben bisherigen v0.2-Negativtests sind erhalten.
Die früheren temporären Dateien wurden durch Speicherfixtures ersetzt. Jeder
Test beginnt mit einem gültigen Ausgangszustand und prüft die beabsichtigte
Fehlerklasse. Ein Audit-Hook verbietet bei direktem Testlauf Dateischreibzugriffe,
Dateimutationen, Subprozesse und Socketverbindungen. `-B` verhindert Bytecode.
Keine Tests benötigen temporäre Dateien; keine Tests sind deshalb BLOCKED.
Reale Filesystemeigenschaften werden durch die Speicherfixtures nur kontrolliert
modelliert. N14 prüft weiterhin, dass die unveränderte Pfadregel einen modellierten
Symlink-Ausbruch abweist.

| Tests | Fehlerklassen |
|---|---|
| K1-01–03 | Header-only; jedes Pflichtartefakt ohne verbleibende Referenzen; jede Pflichtzuordnung |
| K1-04–05 | falsche Artefakt-/Zuordnungs-ID |
| K1-06–08 | falsche Source-/Artifact-Bindung und Relationsrolle |
| K1-09–10 | falscher Artefakttyp und Locator |
| K2-01 | Archivhash |
| K2-02–08 | doppelte Mitglieder, Links, Sonderdateien, Pfade, fehlende Mitglieder, Größe und Hash |
| K2-09 | Oraclehash vor Ausführung |
| K2-10–11 | fehlende SRC-000001 beziehungsweise SRC-000002 |
| K2-12 | jede historische Zeile und jedes historische Feld |

Zusatzkontrollen prüfen fehlenden/beschädigten/unbekannten Vertrag, Bindungen
aller fünf Registerausnahmen, unzulässige historische Umklassifizierung,
gültige zusätzliche Datensätze, unveränderte historische Felder und den
historischen Ergebnisvergleich einschließlich ausgewiesener Regelerweiterung.
Für innere TAR-Negativtests wird nur im Test eine geänderte äußere Hashvorgabe
verwendet, damit die jeweils beabsichtigte innere Fehlerklasse erreicht wird;
der produktive Vertrag bleibt unverändert gepinnt.

INDEX-1K-15 ergänzt explizite Erwartungen für die erhaltenen 23 Korpusfälle,
Finding-Duplikate, falsche Fehlerklassen und die Weitergabe unerwarteter Ausnahmen.
Sensitivitätskontrollen entfernen ausschließlich im Speicher die aktuelle
Finding-ID-Eindeutigkeitsprüfung, den Symlink-Ausbruchschutz, die direkte
Originalhashpflicht oder die neue Registerhashbedingung. Historische Oraclebytes
werden dabei nicht verändert. Der Finding-Mutant muss `compare_rules()` und
`run()` vor dem finalen `V02_REGRESSION=PASS` scheitern lassen. Eine zusätzliche
Parserkontrolle ersetzt im Speicher den aktuellen Parser durch einen ungeprüften
Parser und verlangt die Erkennung des fehlenden Headerschutzes.
Alle fünf Registerausnahmen werden einzeln direkt positiv sowie mit manipuliertem
Hash und jedem Identitäts-/Vertragsfeld negativ geprüft; auch unberechtigte
REGISTER_TYPES und Typwechsel reservierter IDs sind direkt abgedeckt.
N01–N24, K1-01–K1-10, K2-01–K2-12 und die bisherigen v0.2-Tests bleiben erhalten.
Diese Angaben beschreiben implementierte Tests, keine bestandenen Testläufe.

Die lokal gesuchten Originalberichte INDEX-1K-9, INDEX-1K-10 und INDEX-1K-11
wurden nicht gefunden. Die obige K1-/K2-Nummerierung konkretisiert daher die
expliziten Anforderungen von INDEX-1K-12; eine exakte Übernahme eines nicht
vorliegenden INDEX-1K-11-Testkatalogs wird nicht behauptet.

INDEX-1K-15 ändert ausschließlich `validate.py`, `regress_v02.py`,
`test_validate.py` und dieses README.
TSV-Daten, TAR und Sidecar bleiben unverändert. Keine Netzwerkzugriffe, Builds,
Nix-Evaluierungen, Medienzugriffe, Hardwaretests oder Git-Mutationen. Hashgleichheit
belegt Byteidentität, keine Root Cause, Bootfähigkeit, vollständige historische
Abläufe oder Authentizität über die dokumentierte lokale Archivprovenienz hinaus.
Offen bleiben fehlende Originalberichte, lokale Chatoriginalausgaben und die
im Quellenregister ausgewiesenen Primärquellenlücken.

---

### 2026-10-08 – INDEX-1K-28/29/30: geprüfter und übernommener Dokumentationscheckpoint

**Status: README_INTEGRATED; INDEPENDENT_ACCEPTANCE=PENDING.** Dieser Abschnitt wurde in INDEX28 entworfen, in INDEX29 innerhalb seines begrenzten Geltungsbereichs positiv geprüft und in INDEX30 wortgetreu in dieses README übernommen. Er konsolidiert INDEX22/25, die Ausführungen INDEX23/26 und die Bewertungen INDEX24/27. Die Übernahme begründet keine globale unabhängige Gesamtabnahme; K2 bleibt PENDING.

#### Quellen und Nachweisidentitäten

Die Primärprotokolle liegen unter:

`/home/loomit/novena-backups/block-logs/`

Maßgebliche Dateien sind:

- `INDEX-1K-21-20261008-110555-IcljGUIa.txt`
- `INDEX-1K-22-20261008-111329-GAWNQ7VD.txt`
- `INDEX-1K-23-20261008-112218-7sosWfnZ.txt`
- `INDEX-1K-24-20261008-112433-RKNnk2bS.txt`
- `INDEX-1K-25-20261008-113500-iePtDAAs.txt`
- `INDEX-1K-26-20261008-114733-kY2DVghx.txt`
- `INDEX-1K-27-20261008-114907-mvFmpmV7.txt`

Ihre vorhandenen SHA256-Sidecars wurden in INDEX28 lokal geprüft und stimmen überein. Die zentrale Ausführungsevidenz INDEX26 hat SHA256:

`4ca11629cee412dea478bc14e9b80d28d639d2712a7ae41a70cec81a9e056778`

Die zentrale unabhängige Bewertung INDEX27 hat SHA256:

`cce5e359e28267cdce0895f56a6c7dacb5463fd6144c399eb0b249e2585cc1ed`

Der dokumentierte Git-Kontext lautet:

- HEAD: `60785fc2ea36f0a12d03411c9812a616f06a810e`
- Git-Index SHA256: `a4c8b39a15619befe96fe41f9813b8e710550e82379a587baa713c27c98b9d7e`

Da `docs/project-index/` unversioniert ist, identifiziert HEAD allein die geprüften Dateien nicht. Die folgenden SHA256 identifizieren den **vor diesem Dokumentationsnachtrag** geprüften Stand:

```text
124f6b8b672a3ece7f099d0a2389fcfc95877f9a7201c9bf1038cb783485dcd6  validate.py
450b82cc3a01224920ffd4a4a4390aa0a2c346beec7ade3a3c61a0439e1e1032  regress_v02.py
2e84c3165f1bd3923c7040e910aa5d8817259000104f35715055fe9f41ef9aa7  test_validate.py
c41797a207a16723e08eb311db64ab630cadb4268dfaf2e42cb60c02fb801ce7  baseline-contract.json
ebbdd9cd23d0e73f4ab275d561ffaac77012373e7e6da39534c707b44a56925e  README.md
8a66cc14f5f873f545a0138bb4772af72897455b0a99533fba62795fcf5c787d  artifacts.tsv
0b4c841c8da8c084191cb00a0bb66656e34abbe7ba9275d7960ae7cd25589844  source-artifacts.tsv
6966ddbe3d0b858196bd40c0b1084c5638ffa9d3d42fd5feb4385e6caae732da  sources.tsv
92fe1a1a1ed270287363d5f17543e83943f494c28e4e3b1258cf34a915faf6ab  source-relations.tsv
60438b59f149c2024fcfde80b478e87f52a51add92b5516726462ca9b0a93315  findings.tsv
2f0454acfff7693bffcebe595b368c3789278a3d47a36e7fb7de715018f506cb  evidence-links.tsv
```

Historische Grundlage:

`/home/loomit/novena-backups/project-index-v02-before-v03-60785fc2/project-index-v02.tar`

Archiv-SHA256:

`18afce33e3e7a0f6cc4851b6c3e3fe7366031c806575ea671454b0c9ad4d4844`

Historisches Validator-Mitglied: `docs/project-index/validate.py`, 6860 Byte, gepinnter SHA256:

`de1728d058d9e7df94129baba66d969c37ba0e7ce50a00f1b1a18530f6c84af4`

INDEX28 bestätigt den Gesamtarchivhash gegen den Sidecar. Die historische Mitgliedsauthentifizierung und Ausführung werden aus den erhaltenen Regressionsergebnissen übernommen; INDEX28 führte keine neue Mitgliedsprüfung oder Python-Ausführung durch. Hashgleichheit belegt lokale Byteidentität innerhalb dieser Provenienz.

#### Viergliedrige Abnahmematrix

| Bereich | Status | Geltungsbereich |
|---|---|---|
| K1 | PASS | Gepinnte Pflichtbestände ART-000001–000009 und SAL-000001–000009 samt geprüften Bestands- und Bindungsfehlern. |
| K2 | PENDING | Gesamtanspruch offen; M04/M05 sind für den nachstehend definierten 18/130-Bestand begrenzt qualifiziert. |
| REGISTER_TYPES | QUALIFIED_PASS | Fünf autorisierte Identitäten, jeweils 26 Varianten; direkte Originalprüfungsvergleiche und fünf zusätzliche vollständige Positivpfade. |
| TEST_INDEPENDENCE | QUALIFIED_PASS | Unabhängig definierte Eingaben/Sollresultate und beobachtete Vergleichsinventur ausschließlich für die 18 isolierten und 130 Registerfälle. |

Diese Matrix übernimmt die abschließende Bewertung aus INDEX27. QUALIFIED_PASS erfüllt keinen globalen Unabhängigkeitsanspruch. Die dokumentarische Anerkennung dieses begrenzten Umfangs bleibt reviewpflichtig.

#### Exakter M04/M05-Geltungsbereich

M04 prüft getrennt den Sollbestand, die erzeugten Fälle, abgeschlossene alte/neue Ergebnisadapter und erfolgreich abgeschlossene Vergleichsprädikate. Treiberrückgaben allein gelten nicht als Ausführungsinventur.

M05 legt Eingabedeltas und erwartete alte/neue Ergebnisse separat fest. Die Sollwerte werden nicht aus den geprüften Generatorerwartungen oder beobachteten Ergebnissen übernommen.

Der isolierte Bestand enthält:

```text
empty-source_id
empty-finding_id
empty-relation_id
empty-link_id
duplicate-relation_id
duplicate-link_id
namespaced-duplicate
source-original-id
source-record-locator
source-record-hash
basis-hash-syntax
finding-hash-syntax
relation-source-a
relation-self
basis-empty-hash
finding-empty-hash
evidence-source
repository-absent
```

Die ersten 17 Fälle sind Strukturvergleiche ohne Originalprüfung. `repository-absent` ist ein separater direkter Originalprüfungsvergleich mit leeren Tabellen, leeren Fixturedateien und der Wurzel `/absent`.

Der Registerbestand umfasst exakt:

| Source-ID | Original-ID | Typ | Registerzeile | Status |
|---|---|---|---:|---|
| SRC-000009 | SPL-001 | artifact_reference | 60 | HISTORICAL_CATALOG_CHECK |
| SRC-000010 | SPL-002 | artifact_collection | 61 | HISTORICAL_CATALOG_CHECK |
| SRC-000012 | SPL-005 | missing_original | 64 | CONTENT_UNAVAILABLE |
| SRC-000013 | SPL-006 | primary_source_gap | 65 | CONTENT_UNAVAILABLE |
| SRC-000014 | SPL-007 | historical_chat | 66 | CONTENT_UNAVAILABLE |

Namespace ist jeweils `sources-md`; beide Locator binden die entsprechende Zeile von `docs/QUELLENVERZEICHNIS.md`.

Je Identität sind 26 Varianten festgelegt:

- eine autorisierte Kombination;
- sieben Einzeländerungen der Vertragsfelder;
- vier Wechsel zu anderen Registertypen;
- ein gewöhnlicher Typ mit HTTP-Locator unter reservierter ID;
- eine fremde ID und fünf historische IDs SRC-000001–000005;
- sieben Hashvarianten: Nullhash, `bad`, `None`, `False`, `0`, Leerzeichen und passender Registerdateihash.

`None`, `False` und `0` betreffen ausschließlich direkte Python-Funktionseingaben. Sie sind keine unmittelbar darstellbaren TSV-Zellen.

Die aktuelle Regel verlangt zuerst die sieben Identitätsbindungen und danach exakt `sha256 == ''`. Historische Ablehnung wegen fehlenden Originalhashs und aktuelle Annahme autorisierter Registerausnahmen sind eine ausdrücklich zugelassene Regelerweiterung.

| Bestand | Erzeugte Fälle | Vergleichsprüfungen | Abgeschlossene Adapteraufrufe |
|---|---:|---:|---:|
| Isoliert | 17 | 18 | 36 |
| Register | 130 | 135 | 270 |

Bei den Registerfällen entstehen 130 direkte Vergleiche und fünf zusätzliche vollständige Vergleiche über Parser, Struktur und Originalprüfung. Die vollständigen autorisierten Pfade erwarten historisch Ablehnung und aktuell PASS.

Die Inventur verlangt eindeutige und vollständige Identitäten, passende Eingaben, Fixturebytes, Modi und exakte Ergebnisse sowie die Bindung von Fall, Phase und alter/neuer Seite an das erfolgreich geprüfte Vergleichsprädikat.

#### Dynamische Nachweise aus INDEX26

INDEX26 protokolliert auf foobox:

- 96 bestandene Unit-Testmethoden, darunter beide M04/M05-Inventurmethoden und fünf INDEX25-Sensitivitätsmethoden;
- 24 bestandene historische Korpusfälle;
- 33 bestandene historische Direktkontrollen;
- erfolgreiche Archiv-, historische Daten- und Validatoridentitätsstufen;
- `REGISTER_EXTENSION=PASS` und `V02_REGRESSION=PASS`;
- Exitstatus 0 für Unit-Tests und Regression;
- erfolgreiche Vorher-/Nachher-Dateiidentitätskontrollen und unveränderten Git-Checkpoint.

Die bestandenen Inventurmethoden verlangen die oben genannten 18/135 Vergleiche und 36/270 Adapteraufrufe. Dies ist ein protokollierter Assertionsnachweis. Eine separat erhaltene Ereignisliste jedes Vergleichs liegt im INDEX26-Protokoll nicht vor.

Die Sensitivitätsmethoden erkennen die benannten vollständigen Fallausfälle, fehlende oder doppelte erzeugte Fälle, manipulierte Generatorerwartungen, eine verkürzte übergebene Sollmenge und die vollständige Wiederholung eines Vergleichstreibers. Erwartungsmanipulationen werden bereits vor der Adapterausführung erkannt.

`ORIGINALS_CHECK=PASS checked=3` stammt aus einem kontrollierten Unitfixture. Es qualifiziert keine vollständige reale Originalprüfung des Index oder Referenzbaums.

#### Unabhängige Bewertung aus INDEX27

INDEX27 prüft statisch die aktuellen Originaldateien und die vorhandene INDEX26-Ausführungsevidenz. Die Bewertung bestätigt die begrenzte Abnahmefähigkeit von M04/M05 für 18 isolierte und 130 Registerfälle.

Sie unterscheidet die positiven Inventurkontrollen von den Sensitivitätskontrollen. Vollständige Mutationsabdeckung wird nicht behauptet. Insbesondere bleibt die gezielte dynamische Sensitivität gegen folgende Änderungen unbelegt:

- Auslassen nur des Vergleichsprädikats bei weiterhin ausgeführten Adaptern;
- selektives Duplizieren eines Vergleichs oder einer Phase;
- Auslassen nur des vollständigen autorisierten Registerpfads;
- Vertauschen von Fallidentitäten bei gleichen Fehlerklassen;
- Inhaltsänderung einer übergebenen Sollspezifikation bei gleicher Identitätsmenge.

Schutzmechanismen hierfür sind statisch sichtbar. Ein gemeinsamer Bypass durch konsistente Veränderung von Audit und kanonischer Definition ist ein denkbares Risiko, kein beobachteter Fehler.

INDEX27 endet mit `INDEPENDENT_ACCEPTANCE=REQUIRES_REVIEW`.

#### Offene K2-Nachweise und Unabhängigkeitsgrenzen

Die neue Ausführungsinventur umfasst nicht:

- den separaten 24-Fall-Korpus einschließlich seines zusätzlichen Registererweiterungsvergleichs;
- die 33 Direktkontrollen: 16 Parser-, zehn Pfad- und sieben Originalkontrollen;
- sämtliche Feldmutationen aus `test_K2_12_all_historical_fields`.

Für diese Bereiche bestehen implementierte und protokolliert bestandene Kontrollen. Eine entsprechende unabhängige Inventur der tatsächlich ausgeführten Einzelidentitäten ist durch M04/M05 nicht nachgewiesen.

Gemeinsam bleiben historische Ausgangsdaten, `MemoryPath.files`/`links`, Serialisierung, Ergebnisadapter und Teile der Vergleichsinfrastruktur. Die historische Datenaufbereitung verwendet aktuelle `v.SCHEMAS`. INDEX25 ergänzt unabhängige exakte Ergebniserwartungen; es schafft keine unabhängige Dateisystemimplementierung.

Die kanonische INDEX25-Spezifikation bleibt die letzte Sollinstanz. Ihre konsistente eigene Veränderung ist nicht durch eine zweite unveränderliche Fallquelle abgesichert. Erläuternde Regelreferenzen im Manifest werden nicht automatisch gegen historische Quellstellen verifiziert.

Der verbindliche K2-Gesamtumfang bleibt zu klären. Weitergehende Sensitivitätskontrollen sind nur bei einem entsprechend erweiterten Abnahmeanspruch zu planen.

#### Historische R01–R66/H01–H35-Grenze

Eine authentifizierte historische Originalmatrix R01–R66 beziehungsweise H01–H35 ist im geprüften Quellenbestand nicht nachgewiesen. Rekonstruierte Arbeitsmatrizen und der neue INDEX25-Bestand ersetzen diese Originale nicht.

Eine autoritative Zuordnung oder vollständige historische Matrixerfüllung wird nicht behauptet. Eine solche Abnahme erfordert die Originalmatrizen mit nachvollziehbarer Provenienz. Alternativ muss ein neuer endlicher Sollumfang ausdrücklich ohne Anspruch auf Erfüllung dieser historischen Matrizen festgelegt werden.

#### CLI-, Dateisystem- und Audit-Hook-Grenzen

Der historische Adapter ersetzt die Argumentauswertung. Die aktuelle Vergleichsseite verwendet direkte Funktionsaufrufe. Der eigenständige Regressionstreiber wurde ausgeführt; vollständige Argument-, stderr- und Exitcodesemantik sämtlicher CLI-Pfade ist dadurch nicht qualifiziert.

`MemoryPath` modelliert Verzeichnisse und Symlinks vereinfacht. Reale Symlinkketten, Berechtigungen, IO-Zustände und eine vollständige Originalprüfung am identifizierten Referenzbaum bleiben außerhalb dieser Abnahme.

INDEX26 startet die Unit-Tests über `unittest discover`. Der Audit-Hook in `test_validate.py` wird ausschließlich im `__main__`-Block installiert und ist bei diesem Importpfad nicht aktiv. Der Lauf belegt deshalb keine aktive Audit-Hook-Sperre. Die erfolgreichen Identitätskontrollen bleiben davon getrennte Evidenz für die benannten Dateien; sie sind keine globale Nebenwirkungsinventur.

Aus diesen Grenzen folgt kein nachgewiesener Schreibvorfall oder fachlicher Validatorfehler.

#### Ergebnis und nächste dokumentarische Schritte

**Kein produktiver Validatorfehler ist nachgewiesen.**

Die Quellen begründen eine begrenzte dokumentarische Anerkennung des 18/130-Bestands. Sie begründen keine globale historische Semantikäquivalenz, vollständige CLI-/Dateisystemqualifikation oder Hardwareaussage.

Der Dokumentationsentwurf wurde in INDEX29 gegen die Originalquellen geprüft und in INDEX30 wortgetreu in dieses README übernommen. Die dort empfohlene Schreibfreigabe und ihre anschließende Ausführung sind abgeschlossen; die unabhängige Gesamtabnahme bleibt offen.

Als verbleibender nächster Schritt ist der verbindliche K2-Gesamtumfang dokumentarisch festzulegen. Zusätzliche technische Nachweise sind nur bei einer konkreten neuen Fehlerhypothese oder einem ausdrücklich erweiterten Abnahmeumfang zu entwerfen und separat zu autorisieren. Die historischen INDEX15-Aussagen bleiben als damaliger Implementierungsstand erhalten.

Eine Wiederholung der unveränderten 96 Tests ist für diesen dokumentarischen Schritt nicht erforderlich. Der Checkpoint erteilt keine Implementierungs-, Test-, Build-, Netzwerk-, Medien- oder Hardwarefreigabe.

---

### 2026-10-09 – INDEX-1K-58-R1: K2-Dokumentationscheckpoint

Der in INDEX56 genehmigte K2-Umfang ist nun als additiver Nachtrag im
[Umfangsbeschluss](../DECISIONS.md#2026-10-09--index-1k-58-r1-k2-umfang-und-drei-dateien-dokumentationscheckpoint)
dokumentiert; der
[Projektstatus](../PROJECT-STATE.md#2026-10-09--index-1k-58-r1-k2-dokumentationscheckpoint)
führt die offenen Abnahmefragen. Die historischen INDEX15- und
INDEX28/29/30-Aussagen bleiben unverändert. Der damals noch offene
Umfangsbeschluss ist entschieden; eine K2- oder Gesamtabnahme folgt daraus nicht.

| Kategorie | Genehmigte Anforderungen |
|---|---|
| A – verbindlich | K2-S01 bis K2-S09, K2-S13, K2-S16 |
| B – optional | K2-S10, K2-S14 |
| C – außerhalb des Mindestumfangs | K2-S11, K2-S12, K2-S15, K2-S17, K2-S18 |

Der endliche Umfang umfasst Archiv-/Oracleauthentifizierung, vier historische
Header und 16 Parserkontrollen, 13 erhaltene Datensätze/110 Feldwerte,
24 Korpusfälle und 17 isolierte Strukturfälle, zehn Pfad-/sieben Originalkontrollen
samt Repository-Abwesenheit im Speichermodell, benannte Fehlerklassen,
Prüfstufenintegration, die begrenzte 18/130-Inventur, vorhandene Mutanten und
fünf autorisierte Registerausnahmen. Die exakten S01–S09/S13/S16-Definitionen
stehen im verlinkten Beschluss. K2-S-Kennungen sind Anforderungen und nicht
die Testnamen K2-01–K2-12. Registerausnahmen und frühere v0.3-Ablehnung fehlender
lokaler Originalhashes bleiben ausgewiesene zulässige Regelunterschiede.

```text
K2_SCOPE_DECISION=APPROVED_BY_USER
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_ACCEPTANCE=NOT_ESTABLISHED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
```

K1 betrifft die gepinnten ART-/SAL-Pflichtbindungen. REGISTER_TYPES betrifft
fünf Identitäten mit je 26 Varianten und fünf vollständigen Vergleichspfaden.
TEST_INDEPENDENCE betrifft ausschließlich den 18/130-Bestand.

Die Evidenzzuordnung stützt sich auf die im Umfangsbeschluss exakt benannten
lokalen Originalprotokolle INDEX26/27/33/54/55/56/57 unter
`/home/loomit/novena-backups/block-logs/`; alle sieben stimmen beim lesenden
Abgleich in INDEX-1K-58-R1 mit ihren SHA256-Sidecars überein. INDEX26 ist der
historische Lauf mit 96 bestandenen Unit-Testmethoden, 24 Korpusfällen,
33 Direktkontrollen, REGISTER_EXTENSION/V02_REGRESSION PASS und Exitstatus 0.
INDEX27 begrenzt die Bewertung und endet mit REQUIRES_REVIEW. Die 18 isolierten
Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/270
Adapteraufrufe sind Assertionsnachweise ohne separat erhaltene Ereignisliste.
INDEX33/54/55 bleiben damalige Matrix-/Auditstände; der Benutzerbeschluss
aus INDEX56 erweitert deren historische Ergebnisse nicht rückwirkend.

Die unabhängige Einzelinventur für Korpus, Direktkontrollen und alle
Feldmutationen (S10) und zusätzliche Auditmutanten (S14) bleiben optional.
Gemeinsame Ausgangsdaten, Speicherumgebung, Serialisierung und Adapter sowie
die letzte kanonische Sollinstanz begrenzen die Unabhängigkeit. S11/S12/S15/S17/S18
gehören nicht zum Mindestumfang: vollständige CLI-, reale Dateisystem-/Original-,
aktive Audit-Hook-Qualifikation, historische Matrixerfüllung und universelle
Semantikäquivalenz werden nicht behauptet. Authentifizierte R01–R66/H01–H35-
Originalmatrizen, INDEX9–11-Originalberichte und der vollständige INDEX14-Plan
bleiben Quellenlücken. INDEX37/39 sind nur im dokumentierten indirekten bzw.
übergebenen Umfang einzuordnen; fehlende TXT-Dateien allein beweisen keine
fehlende Durchführung. Rekonstruktionen ersetzen keine Originalausgaben.
`ORIGINALS_CHECK=PASS checked=3` bleibt Unitfixture-Evidenz. Der historische
discover-Lauf aktivierte den Audit-Hook nicht; Dateiidentitätskontrollen
sind keine globale Nebenwirkungsinventur.

**FND-000001 bleibt historisch erhalten.** `docs/TEST-LOG.md#L2362` bindet
zusammen mit dem historischen `record_sha256` die gesamte Datei, nicht nur
eine Zeile. Der Locator prüft Zeilenexistenz, keine textliche Finding-Übereinstimmung.
Aktuell entspricht die Datei den gebundenen Bytes. Selbst ein additiver
TEST-LOG-Nachtrag würde einen Hashkonflikt verursachen; Nachziehen historischer
Hashes oder Locatoren widerspräche S04/`preserve()`.

Das gepinnte historische Index-TAR enthält die Registerreferenz, aber keine
Kopie der TEST-LOG-Originaldatei. Der passende historische Snapshot ist lokal
im Git-Objektbestand vorhanden und wurde bytegleich zur aktuellen Datei
bestätigt; seine Commit-/Blob-Identitäten stehen im Umfangsbeschluss.
Dies ist keine vollständige reale Originalqualifikation oder automatisch
verfügbare zusätzliche Provenienzwurzel.

Die genehmigte Drei-Dateien-Übergangslösung dokumentiert Beschluss, Status und
Evidenzzuordnung hier sowie in DECISIONS/PROJECT-STATE; TEST-LOG bleibt
byteidentisch am bisherigen Pfad. Sie ersetzt für diesen Auftrag ausdrücklich
den INDEX56-Vier-Dateien-Plan. Eine vollständige Vier-Dateien-Umsetzung oder
langfristige Snapshot-/Resolverlösung wird nicht behauptet. Historische TSVs,
TAR, Code und Verträge bleiben unverändert; keine zusätzliche Snapshotdatei.

Offen bleiben die eigene fachliche K2-Abnahme gegen A, die anschließende
Projektindex-Gesamtabnahme und der künftige Pfad für fortlaufende
Testdokumentation. Neue Findings allein lösen die bestehende Originalbindung
nicht; ein Ausschluss von S12 entfernt keine Validatorregel. Für diesen
Checkpoint sind keine Tests, Builds, Hardware-, Netzwerkzugriffe oder
Git-Mutationen erfolgt. Die Dokumentation begründet keine Hardwareaussage
und verändert den eigenständigen DDR-/SPL-Untersuchungsstand nicht.

### 2026-10-09 – INDEX-1K-61: K2-Abnahmecheckpoint

INDEX60 gibt das fachliche Votum **QUALIFIED_ACCEPTED ausschließlich für A**
ab. Dieser eigenständige Checkpoint dokumentiert es ergänzend zum
historischen INDEX58-R1-Umfangsbeschluss; dessen damalige offenen Werte
bleiben erhalten. Die Entscheidung steht im
[Abnahmenachtrag](../DECISIONS.md#2026-10-09--index-1k-61-fachliche-k2-abnahme-für-umfang-a),
der aktuelle Status im
[Projektstatus](../PROJECT-STATE.md#2026-10-09--index-1k-61-projektindexstatus-nach-fachlicher-k2-abnahme).

```text
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
```

| Kategorie | Anforderungen und Stand |
|---|---|
| A – genehmigter Mindestumfang | Elf Pflichtanforderungen: S01–S06/S08/S13 PASS (acht), S07/S09/S16 QUALIFIED_PASS (drei). |
| B – optional | S10 zusätzliche Einzelinventur und S14 zusätzliche Auditmutanten bleiben offen; keine Voraussetzung für A. |
| C – ausgeschlossen | S11 CLI, S12 reale Dateisystem-/Originalqualifikation, S15 aktiver Audit-Hook, S17 historische Matrixerfüllung, S18 universelle Semantikäquivalenz. Keine Erfüllungsbehauptung. |

S07 ist auf die benannten Fehlerklassen und geprüften Ausnahmezweige begrenzt.
S09 belegt unabhängig definierte Eingaben/Sollresultate und die
Assertionsinventur von 18 isolierten Vergleichen/36 Adapteraufrufen sowie
130 Registerfällen/135 Vergleichen/270 Adapteraufrufen. Eine separat erhaltene
Einzelereignisliste fehlt; Ausgangsdaten, Schemas, Serialisierung, Adapter und
`MemoryPath` bleiben gemeinsam. Die letzte kanonische Sollinstanz besitzt
keine zweite unveränderliche Fallquelle. S16 gilt für genau fünf autorisierte
Identitäten, sieben Bindungen, exakt leeren Originalhash, je 26 Varianten und
fünf zusätzliche vollständige Vergleichspfade im Modell. Registerhashes
verifizieren keine fehlenden Originalbytes.

**Die zusätzliche unabhängige Einzelinventur für Korpus, Direktkontrollen
und sämtliche Feldmutationen ist nicht Bestandteil des nachgewiesenen Umfangs.**
Der übrige A-Nachweis bleibt endlich; vorhandene Mutanten belegen keine
vollständige Mutationsabdeckung. `ORIGINALS_CHECK=PASS checked=3` bleibt
Unitfixture-Evidenz. Der historische discover-Lauf aktivierte den Audit-Hook
nicht; Dateiidentitätskontrollen sind keine globale Nebenwirkungsinventur.

Originalquelle des Votums ist
`/home/loomit/novena-backups/block-logs/INDEX-1K-60-20261009-093539-m2uOpPQh.txt`
(SHA256 `3118d0f3f1c6a3f75e65634c2259f4ef4a29328df62cc5f83f3762081660f21e`).
INDEX56 beschließt den Umfang, INDEX57 analysiert den Provenienzkonflikt,
INDEX58-R1 dokumentiert die Drei-Dateien-Lösung und INDEX59 bestätigt die
Abnahmebereitschaft. Die fünf Originalprotokolle sind im Abnahmenachtrag
exakt benannt und stimmen in INDEX61 mit ihren Sidecars überein.
Die technische Laufzeitevidenz bleibt INDEX26 mit dem begrenzten Review
INDEX27; INDEX60 bestätigt die zugehörigen Dateiidentitäten statisch.
Dieser Dokumentationscheckpoint behauptet keinen neuen Testlauf.

Authentifizierte R01–R66/H01–H35-Originalmatrizen, INDEX9–11-Originalberichte
und der vollständige INDEX14-Plan bleiben Quellenlücken; Rekonstruktionen
ersetzen keine Originalausgaben. INDEX37/39 behalten ihre indirekte
beziehungsweise übergebene Provenienz. Der unversionierte Index wird nicht
allein durch HEAD identifiziert. Weder umfassende reale Originalprüfung
noch universelle historische Semantikäquivalenz oder Hardwareaussagen folgen
aus der K2-Abnahme.

TEST-LOG bleibt byteidentisch am bisherigen Pfad. FND-000001,
`record_locator` und `record_sha256` bleiben unverändert; der Hash bindet
die gesamte Datei. Das historische Index-TAR enthält keine TEST-LOG-
Originalkopie. Der vorhandene lokale Git-Snapshot ist keine implementierte
Snapshotstrategie oder Resolverlösung. Deren Entscheidung bleibt offen.
**Fachliche K2-Abnahme und Projektindex-Gesamtabnahme bleiben getrennt:**
PROJECT_INDEX_ACCEPTANCE ist weiterhin NOT_ESTABLISHED. Der genehmigte
Mindestumfang wird durch diesen Checkpoint nicht erweitert.

### 2026-10-09 – INDEX-1K-63: Verbindlicher Projektindex-Gesamtabnahmeumfang

Der ausdrückliche Benutzerbeschluss zu INDEX-1K-63 genehmigt die belegten
Teilqualifikationen gemeinsam als vollständigen verbindlichen
Projektindex-Gesamtabnahmeumfang, einschließlich der dokumentierten
qualifizierten Einschränkungen. Der
[Umfangsbeschluss](../DECISIONS.md#2026-10-09--index-1k-63-verbindlicher-projektindex-gesamtabnahmeumfang)
enthält die Quellenzuordnung; der
[Projektstatus](../PROJECT-STATE.md#2026-10-09--index-1k-63-verbindlicher-gesamtumfang-und-offene-gesamtabnahme)
führt Beschluss und offene Gesamtabnahme getrennt.

```text
PROJECT_INDEX_SCOPE_DECISION=APPROVED_BY_USER
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
FINAL_ACCEPTANCE_REVIEW=PENDING
```

INDEX58-R1 bestimmt K2-A mit S01–S09/S13/S16; INDEX61 dokumentiert
acht PASS (S01–S06/S08/S13) und drei QUALIFIED_PASS (S07/S09/S16).
Die fachliche Bewertung INDEX62 bestätigt diese begrenzten Teilnachweise
und benennt den damals fehlenden Gesamtumfang als Entscheidungshindernis.
Der jetzige Benutzerbeschluss beantwortet dessen Umfangsfrage, ohne das
historische NOT_ESTABLISHED-Votum rückwirkend zu ändern. Die lokale
Originalquelle INDEX62 und ihr mit dem Sidecar übereinstimmender Hash sind
im verlinkten Umfangsbeschluss benannt.

K1 bleibt auf die gepinnten ART-/SAL-Pflichtbindungen begrenzt.
REGISTER_TYPES gilt für genau fünf autorisierte Identitäten, sieben Bindungen,
exakt leeren Originalhash, je 26 Varianten und fünf zusätzliche vollständige
Modellpfade. Registerhashes bestätigen keine fehlenden Originalbytes.
TEST_INDEPENDENCE gilt ausschließlich für den 18/130-Bestand: 18 isolierte
Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/270
Adapteraufrufe als Assertionsnachweis ohne separate Einzelereignisliste.
Gemeinsame Daten, Schemas, Serialisierung, Adapter und `MemoryPath` sowie
die kanonische Sollinstanz ohne zweite unveränderliche Fallquelle bleiben
anerkannte Grenzen. S07 umfasst benannte Fehlerklassen und geprüfte
Ausnahmezweige; vorhandene Mutanten belegen keine vollständige Abdeckung.

Historische Originalqualifikation und Quellenlücken bleiben innerhalb der
dokumentierten Aussagegrenzen; fehlende Originalmatrizen/-berichte und der
vollständige INDEX14-Plan werden nicht durch Rekonstruktionen ersetzt.
INDEX37/39 behalten ihre indirekte beziehungsweise übergebene Provenienz.
`ORIGINALS_CHECK=PASS checked=3` bleibt Unitfixture-Evidenz; der historische
discover-Lauf aktivierte den Audit-Hook nicht. Globale Testunabhängigkeit,
umfassende reale Originalqualifikation und universelle Äquivalenz sind unbelegt.

B (S10/S14) bleibt optional; C (S11/S12/S15/S17/S18) bleibt außerhalb des
Pflichtumfangs. Diese Bereiche, historische Originalqualifikation und eine
zusätzliche Snapshot-/Resolverstrategie sind keine neuen Pflichtkriterien
der Gesamtabnahme. Bestehende Validatorregeln, Vertragspins und historische
Felder bleiben unberührt. TEST-LOG und FND-000001 samt Locator und
Ganzdateihash bleiben unverändert. Der lokale Git-Snapshot ist keine
implementierte Resolverstrategie; eine konsistente Lösung für künftige
TEST-LOG-Änderungen bleibt separate langfristige Folgearbeit.

**Umfangsgenehmigung bedeutet keine automatische Gesamtabnahme.** Die
abschließende fachliche Gesamtabnahmeprüfung gegen diesen nun verbindlichen
Umfang einschließlich seiner Einschränkungen steht aus. Alle historischen
Statuswerte, Aussagen und Abschnitte bleiben erhalten. INDEX63 ergänzt nur
DECISIONS, PROJECT-STATE und dieses README; keine weiteren Dateien, neuen
Testläufe, Builds, Hardwaretests, Netzwerkaktionen oder Git-Mutationen.

### 2026-10-09 – INDEX-1K-65: Formaler qualifizierter Projektindex-Abschluss

Der Benutzer hat nach der fachlichen Bewertung INDEX-1K-64 ausdrücklich
die formale qualifizierte Projektindex-Gesamtabnahme freigegeben. Dieser
Nachtrag dokumentiert die neue Benutzerentscheidung vom 2026-10-09 nach
INDEX64. INDEX61 dokumentierte ausschließlich die fachliche K2-Abnahme,
INDEX63 den verbindlichen Gesamtumfang; INDEX64 gab die abschließende
fachliche Empfehlung ab und ließ die formale Benutzerabnahme noch offen.

Der verbindliche, nun formal abgenommene Umfang bleibt:

```text
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
```

Die fachliche Bewertung aus INDEX-1K-64 wird als Abnahmegrundlage übernommen:

```text
CHECKPOINT_REVIEW=PASS
DOCUMENTATION_REVIEW=PASS
EVIDENCE_REVIEW=QUALIFIED_PASS
MANDATORY_BLOCKERS=NONE_IDENTIFIED
PROJECT_INDEX_ACCEPTANCE_RECOMMENDATION=QUALIFIED_ACCEPTED
```

Die jetzt ausdrücklich erteilte formale Benutzerfreigabe lautet:

```text
PROJECT_INDEX_SCOPE_DECISION=APPROVED_BY_USER
PROJECT_INDEX_ACCEPTANCE=QUALIFIED_ACCEPTED
FINAL_USER_ACCEPTANCE=APPROVED
ACCEPTANCE_BASIS=INDEX-1K-63,INDEX-1K-64
```

Damit ist die formale qualifizierte Projektindex-Gesamtabnahme für den
in INDEX63 genehmigten endlichen Umfang abgeschlossen. Das fachliche
Votum INDEX64 und die anschließende Benutzerentscheidung bleiben als
unterschiedliche Schritte nachvollziehbar. Die Freigabe ist die ausdrückliche
Entscheidung im Auftrag zu INDEX65, keine aus Teilwerten oder Runner-PASS
abgeleitete automatische Gesamtabnahme.

K1 gilt für die gepinnten ART-/SAL-Pflichtbindungen. REGISTER_TYPES gilt
für genau fünf autorisierte Identitäten, sieben Bindungen, exakt leeren
Originalhash, je 26 Varianten und fünf zusätzliche vollständige Modellpfade;
Registerhashes verifizieren keine fehlenden Originalbytes.
TEST_INDEPENDENCE bleibt ausschließlich auf den 18/130-Bestand begrenzt:
18 isolierte Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/
270 Adapteraufrufe als Assertionsnachweis ohne separate Einzelereignisliste.
Gemeinsame Daten, Schemas, Serialisierung, Adapter und `MemoryPath` sowie
die kanonische Sollinstanz ohne zweite unveränderliche Fallquelle bleiben
anerkannte Einschränkungen. K2-A umfasst S01–S09/S13/S16: acht PASS
(S01–S06/S08/S13), drei QUALIFIED_PASS (S07/S09/S16). S07 bleibt auf benannte
Fehlerklassen und geprüfte Ausnahmezweige begrenzt. Teilweise gemeinsame
Evidenz ist keine zusätzliche unabhängige Bestätigung.

Die Abnahme umfasst ausdrücklich nicht:

- vollständige historische Originalqualifikation;
- universelle Semantikäquivalenz;
- globale Testunabhängigkeit;
- vollständige reale Dateisystem-/Originalprüfung;
- neue Aussagen über Hardware, DDR, SPL oder Bootfähigkeit;
- optionale K2-Bereiche B (S10/S14) und ausgeschlossene Bereiche C
  (S11/S12/S15/S17/S18);
- eine implementierte Snapshot-/Resolverstrategie.

Die dokumentierten Quellenlücken und Evidenzgrenzen bleiben bestehen.
Rekonstruktionen ersetzen keine fehlenden Originalmatrizen oder -berichte;
`ORIGINALS_CHECK=PASS checked=3` bleibt Unitfixture-Evidenz. Der historische
discover-Lauf aktivierte den Audit-Hook nicht. Vorhandene Mutanten belegen
keine vollständige Mutationsabdeckung. Die Snapshot-/Resolverlösung für
künftige TEST-LOG-Änderungen bleibt separate langfristige Folgearbeit.

Alle früheren Aussagen und Statuswerte, insbesondere NOT_ESTABLISHED,
PENDING und das damalige FINAL_USER_ACCEPTANCE=PENDING aus INDEX64, bleiben
als historische Kenntnisstände unverändert. Die neue Entscheidung gilt ab
diesem Nachtrag und ausschließlich für den benannten Projektindexumfang.
Bestehende Validatorregeln, Vertragspins und historische Felder bleiben
unberührt. FND-000001 behält sämtliche historischen Felder einschließlich
`record_locator=docs/TEST-LOG.md#L2362` und
`record_sha256=b165636a7e5146903d9675639d8d435a474c88f7231341922781bbf027b00719`.
Der Hash bindet weiterhin die gesamte unveränderte TEST-LOG-Datei am
bisherigen Pfad. Der vorhandene lokale Git-Snapshot ist keine implementierte
Snapshot-/Resolverstrategie.

Die lokal geprüften Ergebnisprotokolle liegen unter
`/home/loomit/novena-backups/block-logs/`:

| Block | Originalprotokoll | SHA256 |
|---|---|---|
| INDEX-1K-61 | `INDEX-1K-61-20261009-094329-9VNEICD3.txt` | `1bdd98498923c6bd15b9dc2485acae2c5af2372c1b7c46524dbfcef8ded390ef` |
| INDEX-1K-63 | `INDEX-1K-63-20261009-095443-7J4YhpM9.txt` | `41fc4a4c89d67db9cc88380411d3a218bd4b5b9746357dc2786cdadb3406fb92` |
| INDEX-1K-64 | `INDEX-1K-64-20261009-100341-rQPTvBmV.txt` | `24b46e161efe2f111ffd5123c774a60861db54af89382eeaeca16b7d3bd9519a` |

Alle drei Hashes stimmen in INDEX65 beim lesenden Abgleich mit den vorhandenen
Sidecars überein. Ihre Abschlüsse weisen COMMAND_EXIT_STATUS=0,
BLOCK_RESULT=PASS und LOG_COMPLETE=YES aus; die fachlichen Ergebnisse wurden
getrennt geprüft. INDEX64 ist eine lesende fachliche Bewertung, kein neuer
Testlauf. Die historische Laufzeitevidenz bleibt INDEX26 mit den dokumentierten
Reviewgrenzen.

INDEX65 ergänzt ausschließlich DECISIONS, PROJECT-STATE und Index-README.
Die bisherigen Dateiinhalte bleiben bytegenau erhalten. Keine weiteren
Dateien, temporären Dateien, Runnerlogs oder Sicherungen werden erzeugt.
Keine Git-Mutationen, Commits, Pushes, Builds, Tests, Hardwarezugriffe oder
Netzwerkaktionen werden durchgeführt.

Der [formale Beschluss](../DECISIONS.md#2026-10-09--index-1k-65-formaler-qualifizierter-projektindex-abschluss)
und der [Projektstatus](../PROJECT-STATE.md#2026-10-09--index-1k-65-formaler-qualifizierter-projektindex-abschluss)
dokumentieren denselben qualifizierten Abschluss.
