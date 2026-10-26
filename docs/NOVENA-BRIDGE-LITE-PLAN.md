# NOVENA-BRIDGE-LITE – Dokumentations- und Statuskonzept

Stand: 2026-10-25
Status: Planung beschlossen; Umsetzung nicht begonnen.

## 1. Ziel

Die Novena-Projektdokumentation soll leichter zugänglich werden,
ohne historische Erkenntnisse, Originalprotokolle oder
reproduzierbare Nachweise zu verlieren.

NOVENA-BRIDGE-LITE aus dem Homelab-Kontext soll bei der
künftigen Bereitstellung eines geprüften aktuellen
Projektzustands berücksichtigt werden.

Die Bridge ist keine zweite maßgebliche Wissensbasis.
Die verifizierten Originalquellen bleiben führend.

## 2. Reihenfolge

1. Aktuellen Hardwaretest der foobox-Displayvariante abschließen.
2. Bestehende Novena- und Homelab-Dokumentation auswerten.
3. Tatsächlichen Stand und Schnittstellen von
   NOVENA-BRIDGE-LITE anhand ihrer Originalquellen feststellen.
4. Dokumentationsstruktur und Datenmodell vereinbaren.
5. Kleinen lesenden Pilot umsetzen und qualifizieren.
6. Erst nach bestandener Prüfung in Chat-Starter und
   Tagesabschluss integrieren.

Die Schritte 2 bis 6 dürfen den Displaytest nicht
unnötig verzögern.

## 3. Vorgesehene Dokumentationsstruktur

- PROJECT-STATUS.md: kurzer, aktueller und verifizierter Status.
- TODO.md: aktive und geplante Aufgaben.
- HANDOVER-LATEST.md: kompakte Übergabe mit Quellenverweisen.
- CHAT-STARTER.md: kurzer Einstieg in den aktuellen Stand.
- IMAGE-INVENTORY.md und image-inventory.json:
  vorhandene Image-Artefakte und nachgewiesene Identitäten.
- TEST-LOG.md und GitHub-Blockarchiv:
  vollständige historische und technische Evidenz.

Die endgültigen Pfade und die Konsolidierung bestehender
Dokumente sind noch zu entscheiden.

Historische Dokumentation wird nicht gelöscht.
Ältere Nachträge bleiben erreichbar und datiert.

## 4. Rolle von NOVENA-BRIDGE-LITE

Vorgesehener erster Pilot:

- Bestehende Git- und Inventurdaten lesend auswerten.
- Runner-Ergebnis, Archiv-Upload und fachliches
  Testergebnis getrennt erfassen.
- Hardwareboots nur eindeutig identifizierten Artefakten
  zuordnen.
- Widersprüche und fehlende Informationen als
  UNKNOWN beziehungsweise REVIEW_REQUIRED kennzeichnen.
- Einen überprüfbaren Status-Snapshot mit Quellenverweisen
  bereitstellen.

Eine technische Architektur oder vorhandene Bridge-API
wird durch diesen Plan nicht als bereits bestätigt behauptet.

Vor der Implementierung muss das bestehende
Homelab-Konzept abgeglichen werden.

## 5. Sicherheits- und Betriebsgrenzen

- Keine automatische Ausführung von Builds oder Hardwaretests.
- Keine SD-Schreibzugriffe durch die Bridge.
- Keine Änderungen an Runner oder Originalprotokollen.
- Keine erweiterten SSH- oder Git-Schreibrechte.
- Keine eigenständige Veröffentlichung ungeprüfter Ergebnisse.
- Kein neuer Dauerbetrieb, keine zusätzliche VM und
  keine KI-Abhängigkeit ohne begründete Notwendigkeit.
- Git-Index und bestehende Intent-to-add-Einträge schützen.

Der Pilot arbeitet zunächst ausschließlich lesend.
Spätere Schreib- oder Veröffentlichungsfunktionen
benötigen einen eigenen Auftrag und eigene Freigaben.

## 6. Qualitätskriterien

Ein generierter Projektstatus muss:

- seinen Erstellungszeitpunkt nennen;
- Git-HEAD, Quellen und Prüfstand zuordnen;
- bestätigte Fakten von Vermutungen unterscheiden;
- vorhandene PASS- und FAIL-Ergebnisse korrekt wiedergeben;
- bekannte Hostabweichungen berücksichtigen;
- ungesicherte Image-/Hardware-Zuordnungen offenlassen;
- bei widersprüchlichen Daten keine Sicherheit vortäuschen.

## 7. Aktueller Ausgangspunkt

Die Novena-Image-Inventur wurde am 2026-10-25
mit 209 Dateivorkommen und 116 unterschiedlichen
SHA256-Dateiinhalten erstellt.

Referenz:
- docs/IMAGE-INVENTORY.md
- docs/image-inventory.json

Der aktuelle Displaytest betrifft die getrennte
foobox-Displayvariante. Ihre funktionale Wirkung
auf echter Hardware ist noch nicht bestätigt.

## 8. Nächster Schritt nach dem Displaytest

Zunächst die bestehende NOVENA-BRIDGE-LITE-Dokumentation
aus dem Homelab vollständig prüfen.

Danach eine kleine Architektur- und Datenflussentscheidung
dokumentieren, bevor Implementierungen beginnen.

<!-- NOVENA-BRIDGE-DOCS-124 -->

## Nachtrag 2026-10-25 – Lokaler Pilot implementiert

Der ursprüngliche Plan bleibt als historischer Planungsstand
erhalten. Die nachstehenden Ergebnisse aktualisieren ihn.

### Erreichte Meilensteine

- Block 121: lokale Quellensuche abgeschlossen.
  Eine eigenständige Homelab-Bridge wurde dabei nicht
  nachgewiesen. Die Suche war begrenzt.
- Block 122: erster lokaler Python-Pilot erstellt.
  JSON und Markdown wurden erfolgreich erzeugt.
- Block 123: Härtungsversuch fehlgeschlagen.
  Ursache war ein nicht passender Text-Patch-Kontext.
  Die Wiederherstellung wurde ausgelöst.
- Block 123R: kontrollierte Überarbeitung erfolgreich.
  Fünf synthetische Prüfungen bestanden.
  Der eigene laufende Runner-Block wurde als vorläufig
  erkannt und nicht als abgeschlossen gewertet.

Maßgeblicher technischer Ausgangspunkt:
NOVENA-BRIDGE-LITE-HARDENING-123R.

Die erfolgreiche technische Prüfung ist noch keine
vollständige fachliche Produktivabnahme.

### Architekturentscheidung für den Pilot

Die Implementierung läuft lokal auf foobox.

Bestandteile:

- scripts/novena-bridge-lite.py
- docs/PROJECT-STATUS.md
- docs/bridge-lite/current-state.json

Datenquellen:

- lokales Git-Repository
- lokale Runner-TXT-Protokolle und SHA256-Sidecars
- docs/image-inventory.json

Die Bridge ist keine zweite maßgebliche Wissensbasis.
Originalprotokolle und Primärdokumente bleiben führend.

Es gibt zunächst keine eigene VM, keinen Dienst,
keine Datenbank und keine Netzwerk-API.

### Datenverarbeitung

Die Bridge verarbeitet aktuell bis zu zwölf
abgeschlossene beziehungsweise prüfbedürftige
lokale TXT-Protokolle.

Laufende oder unvollständige Protokolle können
gesondert als vorläufig erfasst werden.

Die fachliche Abnahme bleibt UNKNOWN.
Auch der unabhängig bestätigte GitHub-Uploadstatus
wird noch nicht automatisch ermittelt.

Die Image-/Hardware-Zuordnung wird nicht aus
gleichen Dateinamen oder bloßen Build-Erfolgen abgeleitet.

### Veröffentlichung und Sicherheit

Die beiden Snapshot-Ausgabedateien erhalten
dieselbe Snapshot-ID.

Ihre Ersetzung erfolgt einzeln atomar.
Eine gemeinsame transaktionale Veröffentlichung
ist damit noch nicht nachgewiesen.

Der Pilot darf keine Builds, Hardwaretests,
SD-Schreibzugriffe oder Git-Mutationen auslösen.

### Offene Qualifikation

- Parser-Semantik historischer Protokolle prüfen.
- Verschiedene Block-Ergebnisfelder korrekt zuordnen.
- SHA256- und Quellvertrauen getrennt bewerten.
- Snapshot-Konsistenz bei unterbrochener Veröffentlichung
  nachweisbar absichern.
- Veröffentlichungsumfang und Datenschutz prüfen.
- Unabhängige Gesamtqualifikation durchführen.
- Erst danach mögliche Integration in den
  Tagesabschluss gesondert entscheiden.

Bedienung und Grenzen:
docs/NOVENA-BRIDGE-LITE.md

<!-- NOVENA-BRIDGE-ARCHITECTURE-127R -->

## Architekturentscheidung 2026-10-25 – Homelab-Abgleich

Grundlage ist das ursprüngliche Homelab-Konzept
NOVENA-BRIDGE-LITE vom 24.10.2026.

Die Bridge ist eine kontrollierte Informationsverbindung
zwischen foobox, dem vorhandenen Runner, dem privaten
GitHub-Protokollarchiv und ChatGPT.

Sie ersetzt keine Originalquellen und stellt keinen
allgemeinen Shell-Zugriff bereit.

### Baustein A – Bridge Core

Der bereits implementierte lokale Pilot verarbeitet:

- Git-HEAD und lokalen Worktree-Status;
- lokale Runner-TXT-Protokolle;
- lokale SHA256-Sidecars;
- die bestehende Novena-Image-Inventur.

Er erzeugt einen JSON-Snapshot und eine
menschenlesbare Markdown-Übersicht.

Die vorhandene technische Teilqualifikation basiert
auf Block 122 und Block 123R. Block 123 bleibt FAIL.

Ein Runner-PASS ist keine fachliche Abnahme.
Ein gültiger lokaler SHA256-Abgleich ist kein
unabhängiger Beweis für einen GitHub-Upload.

### Baustein B – Bridge Exchange

Dieser Baustein ist noch nicht implementiert.

Er soll einen geprüften Statusindex unter dem Namen
NOVENA-CURRENT-STATE.json im bestehenden privaten
GitHub-Archiv project-block-logs bereitstellen.

ChatGPT soll zunächst diesen Statusindex und danach
die referenzierten Originalprotokolle lesen können.

Das Ziel ist ein schneller, überprüfbarer Projektstart
ohne wiederholte manuelle Übertragung von Logs.

Keine zusätzliche VM, Datenbank, Netzwerk-API,
öffentliche MCP-Schnittstelle oder Tailscale Funnel.

### Zustandsmodell

LAST_VERIFIED:
Letzter anhand seiner Quellen bestätigter Checkpoint.

CURRENT_UNKNOWN:
Aktueller Live-Zustand ist nicht belegt.

BLOCK_FAILED:
Ein fehlgeschlagener Block wird dokumentiert, ersetzt
aber nicht den letzten gültigen Checkpoint.

IN_PROGRESS:
Ein Vorgang läuft oder sein Abschluss ist unbekannt.

UNKNOWN:
Ein einzelnes Datenfeld ist nicht nachgewiesen.

Diese Begriffe sind Architekturvorgaben und noch
nicht vollständig im Pilot implementiert.

### Historisches Homelab-Statusschema

Der Entwurf vom 24.10.2026 enthielt:

- schema_version
- project
- block_id
- timestamp_utc
- runner
- upload
- technical_result
- log_file
- sha256_file
- git_head
- build_state
- verified

Der ursprüngliche JSON-Datensatz war ein Beispiel,
kein tatsächlich qualifizierter Projektcheckpoint.

Das spätere Statusschema muss mindestens trennen:

- Runner- und Wrapper-Erfolg;
- fachliches Ergebnis;
- Integrität des lokalen Protokolls;
- unabhängigen Uploadnachweis;
- Git-Checkpoint;
- Build- und Hardwarezustand.

Das pauschale Feld verified wird nicht unkritisch
als allgemeine Freigabe übernommen.

Nicht belegte Angaben bleiben UNKNOWN.

### Checkpoint- und Veröffentlichungsregel

Ein fehlgeschlagener Block darf LAST_VERIFIED
nicht überschreiben.

Fehler müssen separat und mit Quellenverweisen
erhalten bleiben.

Die Veröffentlichung erfolgt erst nach Prüfung
des fertigen Statusindex und seiner Referenzen.

Die bisherige TXT-/SHA256-Protokollierung,
der Runner und der Git-Workflow bleiben bestehen.

### Nächste Schritte

1. Verbindliches Bridge-Exchange-Datenmodell.
2. Zuordnung echter Originalprotokolle.
3. Prüfung des unabhängigen GitHub-Lesewegs.
4. Implementierung von LAST_VERIFIED.
5. Negativtests für Block-Fail und Upload-Fail.
6. Sichere Veröffentlichung im privaten Archiv.
7. Nachweis des Nutzens bei einem neuen Chat.
8. Spätere Automatisierung nur mit Freigabe.

Keine automatische Tagesabschlussintegration.

<!-- NOVENA-BRIDGE-RUST-DECISION-20261026 -->

## Nachtrag 2026-10-26 – Fertigstellungsplan und Sprachwechsel

Der historische Plan und die Python-Pilotimplementierung
bleiben als Referenz erhalten.

Neue Priorität:

1. Vorhandene Rust-Werkzeuge und Cargo-Abhängigkeiten
   inventarisieren.
2. Bridge-Funktionen und Testvektoren konsolidieren.
3. Rust-Komponenten schrittweise und mit Vergleichstests
   gegen die Python-Referenz entwickeln.
4. Sichere Commitbereitstellung und Veröffentlichungs-
   semantik getrennt qualifizieren.
5. Einen überprüfbaren Statusindex bereitstellen.
6. Statusindex und Originalbelege im neuen Chat lesen
   und ihren praktischen Nutzen nachweisen.
7. Danach zum universellen NixOS-SD-Image zurückkehren.

Keine zusätzliche VM, kein Hintergrunddienst und
keine allgemeine Shell-API sind Teil dieses Beschlusses.

Bestehende Runner und Protokolle werden nicht ersetzt.
Automatisierung darf nur in gesondert freigegebenem
Umfang aktiviert werden.

Das produktive Publish-Gate bleibt gesperrt.

## Nachtrag 2026-10-26 – Lokal getesteter Rust-Bridge-Workflow

Die Rust-Bridge unterstützt die explizite lokale Quellenauswahl für
Runnerprotokoll, lokale Git-Beobachtung und Image-Inventur. Der lokale
Workflow erzeugt eine private Statusvorschau samt Provenienz. Ein optionaler
Shadow-Adapter und ein privater Status-Viewer sind vorhanden.
Die historischen Planungsabschnitte bleiben als Referenz erhalten.

Nachgewiesene Teilergebnisse:

- Block `NOVENA-BRIDGE-RUST-250`: fünf gezielte Shadow-Adapter-Tests `PASS`.
- Block `NOVENA-BRIDGE-RUST-256`: vier Viewer-Laufzeittests `PASS`.
- Block `NOVENA-BRIDGE-RUST-260`: erster erfolgreicher End-to-End-Aufruf
  der lokalen Statuserstellung und anschließenden Anzeige. Der Preparer
  lieferte Exitstatus `3`; der Viewer las die private Ausgabe erfolgreich
  zurück. Der Reviewstatus bleibt `REVIEW_REQUIRED`.

Diese Ergebnisse belegen den lokalen Vorschauworkflow. Technische und
fachliche Abnahme bleiben `UNKNOWN`. Es gibt keine Checkpoint-Promotion
und keine Veröffentlichung. Inventurkonflikte und die unabhängige
Gesamtqualifikation bleiben offen. Es gibt keine automatische
Aktualisierung und keine Runner-Integration.

Die manuelle Bedienung mit `prepare-status.sh` und dem separaten
`bridge-status.sh`-Aufruf ist im
[Rust-Bridge-README](../tools/novena-bridge/README.md#manuelle-statuserstellung-und-anzeige)
dokumentiert. Die Ausgabe bleibt eine historische lokale Vorschau und
erteilt keine Freigabe.

## Freigabematrix 2026-10-26 – Rust-Bridge

Kennung: NOVENA-BRIDGE-RUST-265.

Diese Matrix bewertet ausschließlich den vorhandenen lokalen
Bridge-Piloten. Sie ist keine Produktivfreigabe und ersetzt
keine Originalprotokolle.

| Prüfkriterium | Status | Evidenz / Grenze |
|---|---|---|
| Lokale Rust-Verarbeitung | PASS | Explizite Originalquellen; Block 252 |
| Private Statusvorbereitung | PASS | Block 253 und End-to-End-Block 260 |
| Manueller Status-Viewer | PASS | Vier Laufzeitfälle, Block 256 |
| Optionaler Shadow-Adapter | PASS | Fünf Laufzeitfälle, Block 250 |
| Statussemantik Exchange v1 | PASS | Block 264 |
| Originalbytes / SHA256 | LOKAL GEPRÜFT | Keine unabhängige Quellenauthentifizierung |
| Provenienz | REVIEW_REQUIRED | Lokale Auswahl und Git-Beobachtung |
| Image-Inventur | REVIEW_REQUIRED | Vier bekannte Pfadkonflikte und eine Scanwarnung |
| Technische Gesamtqualifikation | UNKNOWN | Keine unabhängige Gesamtprüfung |
| Fachliche Gesamtqualifikation | UNKNOWN | Keine Freigabe erteilt |
| Aktueller Live-Projektstatus | CURRENT_UNKNOWN | Historische lokale Vorschau |
| Hardware-/Image-Zuordnung | UNKNOWN | Keine gesicherte Identität aus Bridge-Quellen |
| Remote-/Upload-Nachweis | UNKNOWN | Keine unabhängige Attestation |
| last_verified-Promotion | GESPERRT | Keine qualifizierte Freigabe |
| Produktive Veröffentlichung | GESPERRT | Eigenständige Freigabe erforderlich |
| Automatische Aktualisierung | NICHT AKTIV | Kein Dienst und keine Runner-Integration |

### Voraussetzungen für spätere Freigaben

Eine spätere technische oder fachliche Gesamtfreigabe setzt
eine gesonderte unabhängige Prüfung der dafür vorgesehenen
Originalquellen und Statuszuordnungen voraus.

Insbesondere müssen Quellenherkunft, Authentizität,
Prüfumfang, Git-Beobachtung und die beabsichtigte
Image-/Hardware-Zuordnung hinreichend belegt werden.
Nicht belegbare Aussagen bleiben UNKNOWN.

Eine Promotion von last_verified benötigt einen ausdrücklich
qualifizierten Checkpoint mit referenzierter Originalevidenz.
Runner-PASS, Quell-SHA256-PASS oder ein erfolgreiches
Erzeugen einer Statusvorschau genügen hierfür nicht.

Produktive Veröffentlichung, automatische Aktualisierung,
Runner-Integration und etwaige Git-Schreiboperationen
benötigen jeweils einen gesonderten Auftrag und eine
ausdrückliche Freigabe. Bis dahin bleiben diese Gates gesperrt.

### Abgeschlossene Untersuchungen

Der Reproduzierbarkeitsvergleich vom 20.10.2026 bleibt gemäß
docs/NOVENA-DUAL-REPRO-2026-10-20.md abgeschlossen.
Die Inventurabweichungen sind bekannt und dokumentiert.
Daraus wird kein neuer Image-, Build- oder Hardwaretest
abgeleitet. Eine erneute Ursachenuntersuchung erfolgt nur
bei neuer Evidenz oder gesondertem Auftrag.

### Nachweisgrenze

Block NOVENA-BRIDGE-RUST-264 bestätigte die lokale
Statussemantik. Die verwendete private Vorschau ist
historisch und ausdrücklich kein Live-Checkpoint.

Der Projekt- und Veröffentlichungsstatus darf aus dieser
Matrix nicht automatisch verändert werden.

<!-- NOVENA-BRIDGE-RUST-LANGUAGE-2026-10-26 -->

## Architekturentscheidung 26.10.2026 – Rust als Bridge-Sprache

Kennung: NOVENA-BRIDGE-RUST-LANGUAGE-2026-10-26.

### Verbindliche Entscheidung

**Rust ist die maßgebliche Implementierungssprache
für die neue Novena-Bridge.**

Die bestehende Rust-Codebasis unter
`tools/novena-bridge/` wird weiterentwickelt.

Insbesondere sollen folgende Komponenten in Rust
implementiert beziehungsweise integriert werden:

- Verarbeitung und Validierung von Originalquellen;
- Exchange-Protokoll und Statusmodell;
- Git-Objekt- und Commit-Verifikation;
- GitHub-Transport und API-Anbindung;
- Compare-and-Swap-/CAS-Veröffentlichungslogik;
- unabhängige Rücklesung und Ergebnisprüfung;
- Checkpoint-Schutz und Freigabesperren.

### Abgrenzung zu Python und Bash

Die älteren Python-Bridge-Skripte bleiben als
historische Referenzen und bestehende Quellen
erhalten. Sie werden nicht zum produktiven
Implementierungspfad der neuen Rust-Bridge.

Es wird kein zweiter Python-Publisher entwickelt.
Vorhandene Python-Funktionen dürfen zur Prüfung
historischer Verträge herangezogen werden, ersetzen
aber keine Rust-Implementierung.

Bash bleibt als dünne Aufruf-, Bedien- und
Runner-Schicht zulässig. Fachliche Bridge-Logik
wird nicht neu in Bash implementiert.

### Entwicklungsregeln

- Bestehende Rust-Module wiederverwenden.
- Keine Neuentwicklung bereits implementierter Funktionen.
- Vor Änderungen konkrete Rust-Schnittstellen prüfen.
- Isolierte und reproduzierbare Rust-Tests bevorzugen.
- Keine automatische GitHub- oder Git-Mutation.
- Keine Produktivveröffentlichung ohne eigene Freigabe.
- Kein automatisches Fortschreiben von last_verified.
- UNKNOWN und REVIEW_REQUIRED bleiben erhalten,
  solange die erforderliche Evidenz fehlt.
- Keine Builds oder Hardwaretests ohne Freigabe.

### Bekannter Projektstand

Der lokale Rust-Workflow für private Statusvorschauen
und den manuellen Viewer wurde erfolgreich geprüft.

Die dokumentierte Freigabematrix bleibt maßgeblich.
Eine vollständige produktive GitHub-Anbindung in Rust
ist noch nicht qualifiziert.

Die Python-zentrierten Inventuren der Blöcke 268 und
269 waren lesend und haben den Rust-Code nicht ersetzt.

Die nächste Implementierungsarbeit erfolgt ausschließlich
anhand der vorhandenen Rust-Schnittstellen.
