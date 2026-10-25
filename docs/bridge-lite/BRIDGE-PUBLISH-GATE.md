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

## Block 160 – synthetisches GraphQL-Transportmodell (2026-10-25)

`synthetic_graphql_publication()` ergänzt das bestehende Gate ohne Netzwerkadapter
oder CLI-Aktivierung. `payload()` verwendet weiterhin Publisher-Schema, Claims
und kanonische Bytes. `production_publish()` lehnt bedingungslos mit
`PRODUCTION_PUBLICATION_DISABLED` ab; `PRODUCTION_ENABLED=False` bleibt bestehen.
Die neue Funktion ist ausschließlich ein Modell für injizierte Testclients.
Es gibt keinen mitgelieferten echten Client, keine Authentifizierung und keine
Objektanlage oder Übertragung von Commitobjekten.

Vor dem simulierten Aufruf werden der externe Commit und der aktuelle HEAD
geprüft: genau ein Elterncommit gleich EXPECTED_OLD; genau eine Hinzufügung des
festen Zielpfads; regulärer Blob mit Mode 100644; vollständige Bytes gleich dem
kanonisierten Kandidaten. Der explizite Repository-Node-ID-Parameter und
`refs/heads/main` werden in `UpdateRefsInput` eingesetzt; genau ein RefUpdate
enthält `beforeOid=EXPECTED_OLD`, `afterOid=NEW_COMMIT` und `force=false`.
Andere Branches sind im aktuellen Modell nicht zugelassen.

Die einzige abgefragte Payload-Eigenschaft ist `clientMutationId`; ohne Input-ID
wird null erwartet. Die Antwort muss exakt
`{"data":{"updateRefs":{"clientMutationId":null}}}` entsprechen.
GraphQL-errors (auch zusammen mit data), fehlende/null-Strukturen und zusätzliche
Felder auf jeder Ebene scheitern geschlossen. Diese konkrete Antwortform ist
noch nicht gegen eine reale Mutation qualifiziert. Eine erfolgreiche Antwort
beweist keinen resultierenden HEAD: der unabhängige Reader prüft anschließend
HEAD, unveränderlichen Commit und Blobbytes sowie nochmals HEAD. Kein Retry,
Force, Rollback oder automatischer Konfliktausgleich; auch Transportausnahmen
werden weitergegeben. Nach Aufrufbeginn kann ein Fehler bereits erfolgte
Veröffentlichung bedeuten. Dann ist ausschließlich separate Untersuchung zulässig.

### Reader-Vertrag und unabhängiger API-Prüfentwurf

Der injizierte Reader liefert normalisierte Datensätze, **keine unveränderten
GitHub-Antworten**. `head(repositoryId, ref)` liefert den OID;
`commit(repositoryId, oid)` liefert exakt `oid`, vollständige `parents` und
vollständige `changes`. Jeder Änderungseintrag enthält `path`, `status`, `mode`,
`type` und rohe `bytes`. Ein späterer Adapter muss Vollständigkeit selbst
nachweisen; das Modell kann ausgelassene Änderungen eines unehrlichen oder
fehlerhaften Readers nicht erkennen. Der Fake besitzt alle Daten im Speicher.

Ein späterer unabhängiger Nur-Lese-Prüfer muss ohne Publisher-Checkout und ohne
Vertrauen auf die Mutationsantwort arbeiten:

1. Repository-Identität über REST GET auflösen und die `node_id` mit der separat
   freigegebenen repositoryId vergleichen; Branch-Ref exakt lesen.
2. Über `git/commits/NEW_COMMIT` den Commit-SHA und die vollständige Elternliste
   gegen NEW_COMMIT und genau EXPECTED_OLD prüfen.
3. Die Bäume des Eltern- und Zielcommits über Git-Trees-GETs vollständig auflösen.
   Bei `truncated=true`, fehlenden Teilbäumen oder unvollständigen Antworten
   abbrechen; bei nichtrekursiver Abfrage Unterbäume vollständig verfolgen.
   Gesamten Blatt-Diff prüfen: genau der neue Zielpfad, zuvor abwesend,
   Mode 100644, Typ blob; keine weitere Änderung. Ein gefilterter Contents-GET
   allein oder die erste Seite einer Commit-Dateiliste reicht nicht.
4. Den am unveränderlichen Zielbaum referenzierten Blob über Git-Blobs-GET lesen,
   Base64 strikt dekodieren, Größe und Blobidentität prüfen; vollständige Bytes
   und SHA256 mit dem vorab freigegebenen kanonischen Payload vergleichen.
5. Branch-Ref erneut lesen und exakt NEW_COMMIT verlangen. API-Fehler,
   Berechtigungslücken, Weiterleitungen auf andere Identitäten und unerwartete
   oder unvollständige Formate dürfen keine Verifikation ergeben.

Dies ist ein Entwurf, kein implementierter REST-Adapter und kein ausgeführter
API-Test. Der ältere Publisher-Remotecheck ersetzt diese Prüfung nicht.

### Externe Provenienz und offene Produktionslücken

Der neue Commit muss künftig separat vorbereitet, geprüft und ausdrücklich
freigegeben werden. Seine Anlage/Übertragung ist hier nicht implementiert.
Außerhalb des JSON sind Repository-Identität, Ref, alter und neuer Commit,
Eltern-/Baum-/Blobidentitäten, Kandidaten-Quelldateihash, kanonischer Payloadhash,
Prüfzeitpunkte und vollständige API-Belege zu erhalten. Das Rückgabeobjekt des
Modells ist lediglich ein synthetischer Beleg und wird nicht automatisch
persistiert. Der Dateihash des eingerückten Kandidaten und der Hash seiner
kanonischen Publikationsbytes sind unterschiedliche Größen.
`publication.commit=null`, `last_verified=null` und die Bedeutung von
`latest_block=146R` nur für die ausgewählten neun Evidenzpaare bleiben erhalten.
Kandidat 149 und seine historische Qualifikation wurden nicht verändert.

Die im Auftrag berichtete Read-only-Introspektion aus Block 159 belegt die
Schemafähigkeit von updateRefs und den genannten Eingabefeldern. Ihre
Primärausgabe liegt in dieser Kopie nicht als zugehöriger Beleg vor; sie wurde
hier nicht neu beschafft. Nicht belegt sind insbesondere atomare exakte
beforeOid-Prüfung einschließlich Rewind, Fehler-/Teiltransaktionsverhalten,
Antwortform, notwendige Rechte, Branchschutz/Rulesets und Sichtbarkeit extern
bereitgestellter Commitobjekte. Das Fake-CAS bildet die benötigte Semantik ab;
es beweist deren reale Umsetzung nicht. Auch ein später funktionierendes CAS
würde ABA nicht erkennen. Eine Readbackprüfung belegt nur ihren Beobachtungsumfang.

### Lokale Qualifikation und Grenzen

Ausgeführt (ausschließlich lokal):

```sh
PYTHONDONTWRITEBYTECODE=1 python scripts/novena-bridge-publication-test.py GraphQLTests
PYTHONDONTWRITEBYTECODE=1 python scripts/novena-bridge-publisher.py --self-test
```

Elf GraphQL-Testmethoden einschließlich negativer Unterfälle prüfen Request,
Before-/After-OID, force=false, stale HEAD, konkurrierenden Advance/Rewind,
GraphQL-Fehler, fehlende/malformed/zusätzliche Antwortfelder, falschen und später
veränderten HEAD, Eltern/Diff/Pfad/Dateityp/Mode/Bytes, Readbackabweichung,
Transportausnahme ohne Retry, Claims und bedingungslose Produktionssperre.
Alle Mutationsantworten sind speicherbasierte Fakes. Die beiden Race-Fälle
verwenden unterschiedliche OIDs für simulierten Advance und Rewind; ein realer
Commitgraph oder GitHub-Server wird dabei nicht ausgewertet.

Der Publisher-Selftest ist schreibfrei und wurde ausgeführt. Die vorhandenen
sechs Git-Publikationstests und der ältere Gate-Selftest wurden **nicht**
ausgeführt: sie erzeugen temporäre Git-Commits und führen lokale Git-Mutationen
aus; das widerspricht dem ausdrücklichen Commitverbot und der Fake-only-Grenze
dieses Blocks. Ihre bisherige Qualifikation wird nicht als neu bestätigt.
Python-Syntaxprüfung im Speicher und `git diff --check` ergänzen die Prüfungen.
Kein Netzwerk, keine echten Mutationen, kein Runner/Upload, keine Commits und
kein Staging. Ergebnis ausschließlich synthetische Testqualifikation;
Produktionsqualifikation und Veröffentlichung bleiben offen und deaktiviert.
