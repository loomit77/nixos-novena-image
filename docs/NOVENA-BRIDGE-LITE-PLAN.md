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
