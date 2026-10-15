# Borg-Integration – Qualifizierter Zwischenabschluss 01–22

Stand: 2026-10-15  
Projekt: Novena – universelles NixOS SD-Image  
Ausführungsrechner: foobox

## 1. Zusammenfassung

Die Integrationsblöcke BORG-INTEGRATION-01 bis 22 haben die
Vorbereitung und den ersten kontrollierten echten Borgmatic-Sicherungslauf
für den Novena-Projektbestand dokumentiert.

Der erste echte Sicherungslauf wurde mit Exitcode 0 abgeschlossen.
Das neu erstellte Archiv wurde anhand des lokalen Create-Zeitfensters
eindeutig zugeordnet und anschließend mit Borg 1.4.5 unter Verwendung
von `borg check --archives-only --verify-data` geprüft.

Der Integritätscheck wurde ebenfalls mit Exitcode 0 abgeschlossen.

**Ergebnis: Der erste echte Sicherungslauf ist auf Archiv- und
Datenintegrität erfolgreich geprüft.**

Diese Feststellung ist ausdrücklich keine Freigabe der automatischen
Borg-Integration in den Novena-Tagesabschluss.

## 2. Verifizierte Umgebung

- Host: foobox
- Borg: 1.4.5
- Borgmatic: 2.1.10
- Borgmatic-Konfiguration:
  `/home/loomit/.config/borgmatic/novena.yaml`
- Repository:
  `/mnt/novena-borg/repository`
- Dateisystem des Sicherungsmediums: ext4
- Erwartete UUID:
  `b9161f3a-5903-4761-936f-42360d13d84e`
- Konfigurierte Quellverzeichnisse: 7
- Fehlende Quellverzeichnisse bei der Vorprüfung: 0

Die vollständigen Quellpfade und ursprünglichen Borg-Ausgaben werden
nicht in öffentliche oder transportierte Projektprotokolle übernommen.

## 3. Entwicklung und Qualifikation

### BORG-INTEGRATION-01 bis 06

Analyse des historischen Borg-27-Ablaufs und Entwicklung der
isolierten Kandidaten für:

- gemeinsamen Sperrmechanismus,
- dauerhafte Erfassung ursprünglicher Prozessausgaben,
- Nachweisführung mit SHA256,
- Auswertung des Archivbestands,
- konservative Behandlung unklarer Create-Ergebnisse,
- Status-30-Anbindung.

Die Ergebnisse waren zunächst ausschließlich offline beziehungsweise
historisch qualifiziert. Ein produktiver Borg-Lauf war damit noch
nicht nachgewiesen.

### BORG-INTEGRATION-07 bis 15

Bestandsaufnahme der realen Borg-/Borgmatic-Umgebung,
Entwicklung der Prozess- und Git-Adapter und Prüfung der
tatsächlichen Sicherungsvoraussetzungen.

Wesentliche Ergebnisse:

- Borg und Borgmatic sind installiert.
- Das vorgesehene Sicherungsmedium ist eingebunden.
- Die Repository-Zuordnung stimmt.
- Alle sieben konfigurierten Quellverzeichnisse existieren.
- Die Git-Adapter wurden mit realen lesenden Aufrufen geprüft.
- Der absichtlich veränderte Git-Index blieb erhalten.

Die automatische Tagesabschlussintegration wurde nicht produktiv
freigegeben.

### BORG-INTEGRATION-16

Erster kontrollierter echter Borgmatic-Create-Aufruf.

Ergebnisse:

- Vorprüfung: PASS
- Mount-Identität: PASS
- Repository-Zuordnung: PASS
- Freier Speicher vor Create: 83,90 GiB
- Create-Exitcode: 0
- Create-Laufzeit: ungefähr 45,7 Sekunden
- Automatische Wiederholung: deaktiviert
- Originalausgaben: lokal dauerhaft erfasst
- Neues Archiv zu diesem Zeitpunkt: noch nicht verifiziert

### BORG-INTEGRATION-17 bis 21

Lesende Archivbestandsprüfung und Zuordnung des neuen Archivs.

Der Archivbestand enthielt vier Archive.

Die anfängliche Zuordnung scheiterte an der Interpretation der
Borg-Zeitstempel ohne explizite Zeitzone.

Die Auswertung mit der tatsächlichen lokalen Zeitzone der foobox
ergab genau ein Archiv im Create-Zeitfenster.

Das zugeordnete Archiv begann ungefähr 20,395 Sekunden nach dem
Create-Start.

Die vollständige Archiv-ID wurde ausschließlich lokal gespeichert.

### BORG-INTEGRATION-22

Gezielte Integritätsprüfung des zugeordneten Archivs mit Borg 1.4.5.

Verwendete Prüfoptionen:

`borg check --verify-data --archives-only`

Ergebnisse:

- Archivzuordnung: PASS
- Mount-Identität: PASS
- Repository-Zuordnung: PASS
- Check-Exitcode: 0
- Check-Laufzeit: ungefähr 3 Minuten 19 Sekunden
- Originalausgaben: lokal dauerhaft erfasst
- Zweiter Create-Aufruf: nicht ausgeführt

**Bewertung: Archiv- und Datenintegritätsprüfung PASS.**

## 4. Lokale Belege

Die wesentlichen Originalbelege befinden sich auf der foobox:

- `~/novena-backups/borg-integration-16-private/`
- `~/novena-backups/borg-integration-17-private/`
- `~/novena-backups/borg-integration-21-private/`
- `~/novena-backups/borg-integration-22-private/`

Dazu gehören die Create-Ergebnisse, die Archivliste,
die lokale Archivzuordnung sowie die Originalausgaben und
Prüfsummen des Integritätschecks.

Die technischen Arbeitsblöcke wurden zusätzlich über den
bestehenden Runner als vollständige TXT-Protokolle mit
SHA256-Sidecar protokolliert.

## 5. GitHub-Nachweise

Transport-Repository:

`loomit77/project-block-logs`

Wichtige Commit-Checkpoints:

| Block | GitHub-Commit |
| --- | --- |
| BORG-INTEGRATION-15 | `43e3a994afc76f0293b3cbfba3dbc72f8317ac84` |
| BORG-INTEGRATION-16 | `5c67b558e8c958ad076945ba35fdaa87d3cd7084` |
| BORG-INTEGRATION-17 | `24c952ac62e1322772f5ade9551d390ef9cf36cf` |
| BORG-INTEGRATION-18 | `62552cdd711c1154c0893f1e8b6880a658ff31a2` |
| BORG-INTEGRATION-19 | `9689c7dcc9715a4fe9cdfdd75f6f67565a59d987` |
| BORG-INTEGRATION-20 | `048ee597f2ec7b58173171b8d5ffd88b2a72a9a4` |
| BORG-INTEGRATION-21 | `12dadf906b483c42534c460424bcae1c4d9bb2b8` |
| BORG-INTEGRATION-22 | `26544004a6d397b0be15e56ee7181891159012e8` |

Diese Checkpoints belegen die transportierten Prüfprotokolle.
Die vollständigen ursprünglichen Borg-Ausgaben verbleiben lokal.

## 6. Sicherheitsgrenzen

Während des kontrollierten ersten Sicherungslaufs:

- wurde genau ein Create-Aufruf durchgeführt;
- erfolgte keine automatische Create-Wiederholung;
- wurden keine Prune- oder Delete-Operationen ausgeführt;
- wurde kein Borg-Repair ausgeführt;
- blieben die ursprünglichen Borg-Ausgaben lokal;
- blieb die automatische Tagesabschlussfunktion gesperrt.

Die verwendeten lokalen Sperren sind noch nicht als gemeinsamer
Sperrmechanismus mit dem vollständigen Tagesabschluss qualifiziert.

## 7. Noch offene Aufgaben

Vor einer produktiven Freigabe sind insbesondere erforderlich:

1. Die bestehende Tagesabschlussfunktion mit der qualifizierten
   Borg-Ausführung verbinden.
2. Gemeinsame Sperre für Tagesabschluss und Borg-Operationen
   tatsächlich anbinden und prüfen.
3. Originalausgaben, Exitcodes, SHA256 und Runner-Protokolle
   dauerhaft und konsistent verknüpfen.
4. Status-30 als maßgebliche abschließende Entscheidungsinstanz
   vollständig integrieren.
5. Verhalten bei Fehlern, Unterbrechungen und unklarem
   Create-Ergebnis absichern.
6. Einen kontrollierten Wiederherstellungstest durchführen.
7. Dokumentation und TODO nach der abschließenden Integration
   aktualisieren.
8. Die produktive Freigabe ausdrücklich und separat erteilen.

## 8. Abschlussstatus

| Gegenstand | Status |
| --- | --- |
| Realer Borgmatic-Create-Aufruf | PASS |
| Zeitliche Archivzuordnung | PASS |
| Archiv- und Datenintegritätscheck | PASS |
| Dauerhafte lokale Originalausgaben | PASS für den manuellen Lauf |
| Automatischer Tagesabschluss | BLOCKED |
| Vollständige Runner-/Status-30-Integration | OFFEN |
| Praktischer Restore-Test | OFFEN |
| Produktive Freigabe | NICHT ERTEILT |

Der erste echte Sicherungslauf ist qualifiziert abgeschlossen.

Die automatische Tagesabschlussintegration bleibt ein eigener
offener Arbeitsschritt.
