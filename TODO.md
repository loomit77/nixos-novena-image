# TODO – Novena: universelles NixOS-SD-Image

Stand: 2026-10-12 (foobox-Dokumentationscheckpoint DOC-GIT-GESAMT-19S)
Grundlage: Projektgespräche, `AGENTS.md`, `NOVENA-ARBEITSREGELN.txt`, `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md` sowie die GitHub-Repositories `loomit77/Test` und `loomit77/project-block-logs`.
Legende: `[x]` dokumentiert erledigt · `[ ]` offen · **Prüfen** = noch nicht vollständig belegt.
Prioritäten: **P1** vorrangig · **P2** anschließend · **P3** langfristig.

> **Arbeitsregeln:** Vor technischen Änderungen Git-/Index-Zustand, Rohlogs und dokumentierte Checkpoints prüfen. Auf der foobox jeden freigegebenen technischen Block über den Wrapper `~/novena-backups/github-transport/run-and-upload.bash` ausführen. Dieser ruft den unveränderten Runner `~/novena-backups/block-logs/run-block.bash` auf, erzeugt vollständige TXT plus SHA256-Sidecar und überträgt sie synchron ins private GitHub-Protokollrepository. Die Freigabe- und Sicherheitsgrenzen der Regeldateien bleiben bis zu ihrer ausdrücklich autorisierten Aktualisierung maßgeblich. Keine ungefragten Builds, Hardwaretests, Medien-Schreibzugriffe, Git-Resets oder Änderungen am Runner. Intent-to-add-Einträge und vorhandene Nutzeränderungen erhalten. Read-only-Codex-Aufträge schreiben keine Dateien. `printf` statt `echo`. GitHub-Änderungen und lokale foobox-Zustände getrennt bewerten.

## P1-A – Indexierung

- [x] Fortlaufende INDEX-Kennungen und Protokollpflicht als Arbeitsregel dokumentiert.
- [ ] Letzten vollständig geprüften INDEX-Block und nächste freie Kennung aus Originalprotokollen feststellen.
- [ ] Jeden Block mit Auftrag, Zeit, TXT-Datei, SHA256, Git-HEAD, Index-Checkpoint und Ergebnis verknüpfen.
- [ ] Abgebrochene, fehlgeschlagene, vorbereitete und fachlich abgeschlossene Blöcke unterscheiden.
- [ ] Durchsuchbaren, belegbasierten Index aufbauen; Originalprotokolle unverändert lassen.
- [ ] Nach jedem Block Protokollvollständigkeit, Integrität und fachliche Bewertung getrennt prüfen.

## P1-B – Dateiinventur / catalog-v2

- [x] Eigenständigen Inventur-Arbeitsstrang mit INDEX-Protokollen begonnen.
- [ ] Den letzten geprüften catalog-v2-/INDEX-Checkpoint feststellen, bevor die Inventur fortgesetzt wird.
- [ ] Dateien, Pfade, Dateigrößen, SHA256, Quellmedien, Sicherungskopien und Provenienz vollständig erfassen.
- [ ] Dubletten, fehlende Dateien, widersprüchliche Identitäten und unvollständige Manifeste untersuchen.
- [ ] Historische Medien, lokale Backups, Git-Repositories und Nix-Artefakte sauber auseinanderhalten.
- [ ] Inventurdaten mit Blockindex und Quellenverzeichnis verknüpfen.
- [ ] Inventur als eigenen Arbeitsstrang führen; keine unbeauftragten Bootdiagnosen darin beginnen.

## P1-C – GitHub-Upload für Protokolle / weniger manuelle ChatGPT-Uploads

- [x] Privates Zielrepository `loomit77/project-block-logs` eingerichtet.
- [x] Entwicklungsbranch `feature/block-log-upload` in `loomit77/Test` vorhanden; Review unter `docs/UPLOAD-REVIEW.md` und Testplan mit 22 Fällen unter `docs/UPLOAD-V2-TESTPLAN.md` dokumentiert.
- [x] Bestehenden Runner unverändert belassen; automatischen Uploader und Wrapper separat eingerichtet.
- [x] Synchronen Direktupload von TXT und SHA256 nach einem erfolgreichen Block nachgewiesen (GITHUB-UPLOAD-14).
- [x] Archivierung auch bei absichtlich fehlgeschlagenem technischem Block nachgewiesen (GITHUB-UPLOAD-15, Exitcode 17).
- [x] Transportidentität, Archivzustand und GitHub-Lesezugriff anhand von GITHUB-UPLOAD-16 sowie Folgeblöcken überprüft.
- [x] GitHub-Upload von GITHUB-UPLOAD-17A und GITHUB-UPLOAD-18 einschließlich TXT/SHA256 nachgewiesen.
- [x] Lokale `NOVENA-ARBEITSREGELN.txt` kontrolliert um Abschnitt 11 zum automatischen Direktupload ergänzt, Original gesichert, HEAD und Index unverändert (GITHUB-UPLOAD-19). Noch **nicht** als Git-Checkpoint synchronisiert.
- [x] Runner, Wrapper und automatischen Uploader anhand vollständiger Quelltexte und Bash-Syntax lesend auf Regelkonsistenz untersucht (GITHUB-UPLOAD-20). Dies ist eine statische Prüfung, keine vollständige Sicherheitsfreigabe.
- [ ] Widerspruch zwischen früherer manueller Inhaltsprüfung und automatischer Übertragung technisch und organisatorisch abschließend auflösen. Aktuell: Direktupload nur für vorab auf mögliche sensible Ausgaben geprüfte Blöcke; **keine** automatische Geheimniserkennung im Wrapper/Uploader.
- [ ] Prüfen, ob `AGENTS.md` einen ergänzenden Verweis benötigt, ohne Codex-Read-only- oder Freigabegrenzen zu lockern.
- [x] `~/novena-backups/github-transport/DIREKTUPLOAD-ARBEITSREGELN.md` auf foobox angelegt und gegen bekannte Skript-/Regel-SHA256 geprüft (GITHUB-UPLOAD-21; Dokument-SHA256 `5549fc9a534b615972aae44e73f30c2d4cf1a91f55832bad07a861e5a01c60f0`). Dokument lokal; GitHub enthält das vollständige Blockprotokoll, nicht die Markdown-Datei selbst.
- [ ] Wiederaufnahme und gezielte Nacharchivierung nach Upload-/Netzwerkfehler kontrolliert prüfen; kein unkontrollierter Massen-Backfill. Die Wiederaufnahme eines ausstehenden Einzelcommits ist implementiert, aber nicht umfassend funktional qualifiziert (GITHUB-UPLOAD-20).
- [ ] Verbleibende Fälle des 22-teiligen Uploader-Testplans gezielt qualifizieren; bisherige erfolgreiche Funktionstests nicht als vollständige Testplan-Abnahme ausgeben.
- [ ] Ursprünglichen v2-Testplan (`loomit77/Test`, `feature/block-log-upload`, `docs/UPLOAD-V2-TESTPLAN.md`) gegen die aktuell eingesetzte Automatik-Schnittstelle abgleichen; Testfall 08 zur Inhaltsfreigabe ist nicht unverändert übertragbar. Vor Fehlersimulation isoliertes Testkonzept für Fälle 12, 13, 14, 16, 17 und 20 erstellen; produktives Archiv nicht als Testziel verwenden.
- [ ] Idempotenz, Race-Conditions, Push-Fehler, Konflikte und unabhängige Remote-Hashprüfung entsprechend dem noch offenen Testumfang belegen. Statische Schutzmechanismen (`flock`, Git-Zustandsprüfung, SHA256-Vergleich, Remote-Fetch) sind vorhanden, ersetzen aber keine Testnachweise.
- [ ] Veraltete Uploader-Hilfe (`--approve-reviewed-sha256` wird noch angezeigt, im Automatikmodus aber zurückgewiesen) bei einer gesondert freigegebenen Skriptänderung korrigieren.
- [ ] Wrapper-Fehlercodes und getrennte technische/Transport-Ergebnisse dokumentieren: bei Uploadfehler liefert der Wrapper `1`, auch wenn der technische Block einen anderen Fehlercode hatte.
- [ ] Projektübergreifende Zuordnung für Novena, L14, Homelab und weitere Projekte eindeutig regeln.
- [ ] Grenzen dokumentieren: Private GitHub-Repositories sind keine Geheimnisspeicher; GitHub-Lesezugriff kann manuelle ChatGPT-Dateiuploads reduzieren, hebt Produktlimits aber nicht auf.

**Betriebsstand 2026-10-11 (GITHUB-UPLOAD-19/20):** Automatischer Direktupload in den geprüften Erfolgs- und Fehlerfällen funktional nachgewiesen. Lokale Regelergänzung (Abschnitt 11) und statische Konsistenzprüfung abgeschlossen; keine automatische Geheimniserkennung. Vollständige Uploader-Qualifikation, Ausfall-Wiederanlauf, Sicherheitsfreigabe und Git-Synchronisation der lokalen Regeln bleiben **offen**. Die Transportdokumentation wurde lokal in GITHUB-UPLOAD-21 erstellt. Keine rückwirkende Freigabe anderer Aktionen.

## P1-D – Universelles NixOS-SD-Image (technisches Hauptziel)

### U-Boot / SPL / DDR
- [x] Reproduzierbaren projektlokalen U-Boot-/SPL-Baustein und externen P_EXT-Bootpfad entwickelt.
- [x] DDR-A-B-B-A-Experiment vom 2026-10-07 dokumentiert; Patch0012 allein ist kein belegter Fix.
- [ ] Maßgebliche serielle Rohlogs, DDR-Registerbefunde und Provenienz vor jeder neuen Hypothese erneut auswerten.
- [ ] Aktuelle SO-DIMM-/SPD-Identität und reale Rank-Anzahl nur durch gezielte, freigegebene Diagnose klären.
- [ ] DDR-/SPL-Ursache weiter eingrenzen; keinen Root-Cause-Nachweis vorwegnehmen.
- [ ] Eigenständigen Bootpfad `SPL → U-Boot proper → Kernel/Initrd/DTB → Root` schrittweise belegen.
- [ ] Eigenes Bootmenü und Bootauswahl erst auf Basis eines qualifizierten U-Boot-Stands entwerfen.
- [ ] Keine Hardwaretests, Builds oder SD-Schreibvorgänge ohne separate Freigabe.

### Image / NixOS
- [x] Cross-Build und byteidentischen isolierten Image-Build als historischen Referenzstand dokumentiert.
- [x] Funktionale I2C3-/ES8328-Root-Cause abgeschlossen; `regulator-always-on` als dauerhafte Boardanforderung beibehalten.
- [ ] Aktuellen Build- und Image-Stand gegen historische Golden-Referenzen und neuere Diagnosen abgleichen.
- [ ] Partitionierung, Bootdateien, Root-Dateisystem, Kernel, initrd und Device Tree für universellen Betrieb qualifizieren.
- [ ] Nach gelöstem SPL-/DDR-Blocker vollständigen externen SD-Boot mit serieller Evidenz nachweisen.
- [ ] Reproduzierbarkeit, Installationsanleitung, Wiederherstellung und Release-Artefakte abschließend dokumentieren.

## P2 – Historische Quellen und Sicherungen

- [x] Historische Medien-/Provenienzuntersuchungen und Factory-R1-Grenzen dokumentiert.
- [ ] Offene Factory-R1-Quellenlücken und fehlende Binäridentitäten als solche erhalten.
- [ ] Historische SPL-/DDR-Konfigurationen nur anhand belastbarer Primärquellen vergleichen.
- [ ] Backups und SHA256-Manifeste gegen Inventur und Git-Checkpoints prüfen.
- [ ] Golden Build, historische Raw-Master und aktuelle Diagnoseartefakte getrennt aufbewahren.

## P2 – Git, Dokumentation und Arbeitsorganisation

- [ ] Lokalen foobox-HEAD, Branch, Worktree, Index-SHA256 und Intent-to-add-Einträge vor Synchronisation prüfen.
- [x] GitHub-`TODO.md` und lokalen foobox-Stand verglichen (GITHUB-UPLOAD-18): Datei fehlt lokal und im lokalen HEAD.
- [ ] Die aktualisierte GitHub-`TODO.md` später kontrolliert mit dem lokalen Stand abgleichen; den neuen GitHub-Commit nicht blind in einen geänderten Worktree übernehmen.
- [ ] Codeberg und GitHub nach ausdrücklicher Freigabe auf denselben gewünschten Commitstand bringen.
- [ ] `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`, `docs/DECISIONS.md` und Quellenverzeichnis nach belegten Checkpoints aktualisieren.
- [ ] TODO nach jedem bestätigten Meilenstein fortschreiben; offene Punkte nur mit Evidenz abhaken.

## P3 – Veröffentlichung und Weiterentwicklung

- [x] Artikelserie über Novena begonnen; Teil 1 veröffentlicht.
- [ ] Teil 2 und weitere Teile anhand belegter Projektstände fertigstellen.
- [ ] Dokumentation für Nachbau, Installation, Fehlersuche und Wiederherstellung erstellen.
- [ ] Später die Möglichkeit eines modernen, Novena-inspirierten Open-Hardware-Systems gesondert untersuchen.

## Aktueller Freigabestand

- GitHub-Uploader: **Direktupload in geprüften Novena-Fällen im Einsatz; Regelergänzung lokal und statischer Regel-/Skriptabgleich erledigt; Geheimnisschutz, vollständige Qualifikation, Fehler-Wiederanlauf und Dokumentations-Synchronisation offen**.
- DDR-/SPL-Root Cause: **offen**; kein neuer Hardwaretest freigegeben.
- I2C3/ES8328: **abgeschlossen**, ohne neue diskriminierende Evidenz nicht erneut öffnen.
- Universelles externes SD-Image: **noch kein vollständig nachgewiesener eigenständiger Bootpfad**.
- GitHub-Änderungen ersetzen **keinen** lokalen foobox-Checkpoint oder Codeberg-Abgleich.

## Checkpoint 2026-10-12 – Projektindex und Git-Synchronisierung

### Abgeschlossene Meilensteine

- [x] **Projektindex:** Der definierte endliche Gesamtumfang wurde
  mit INDEX-1K-65 am 2026-10-09 formal qualifiziert abgenommen.
  Die anerkannten Evidenz- und Provenienzgrenzen gelten weiter.
  Historische Quellenlücken und eine spätere Snapshot-/Resolverstrategie
  bleiben davon unberührt.
- [x] **TODO auf foobox:** Die bestehende `TODO.md` wurde mit
  DOC-GIT-12 lokal übernommen. DOC-GIT-12R bestätigte die
  Worktree-Integrität; DOC-GIT-13 synchronisierte den Stand
  zu Codeberg und GitHub.
- [x] **Dokumentationsintegration:** Mit DOC-GIT-GESAMT-19O wurden
  52 geprüfte Dateien in drei Commits integriert:
  11 Projektindex-Dateien, 10 Übersetzungs-/Workflow-Dateien
  und 31 Forschungsdokumente.
- [x] **Remote-Synchronisierung:** DOC-GIT-GESAMT-19Q bestätigte
  Codeberg und GitHub auf
  `ae7baffbb60ad6f408825cbeeb0f274eeef8f8ea`.
  Der Git-Index und `docs/TEST-LOG.md` blieben unverändert.

### Fortgeltende Aufgaben

- [ ] **Projektindex:** Den laufenden Blockindex und die
  Evidenzzuordnung fortführen. Dies ist von der bereits erfolgten
  qualifizierten Gesamtabnahme des definierten Umfangs zu trennen.
- [ ] **Dateiinventur:** `catalog-v2` im eigenständigen
  Inventur-Arbeitsstrang fortsetzen.
- [ ] **Protokolltransport:** Ausstehende Sicherheits-, Fehlerfall-
  und Wiederanlaufprüfungen gezielt qualifizieren.
- [ ] **DDR/SPL:** Vor weiteren Eingriffen vorhandene Rohlogs,
  historische Referenzen, Registerbefunde und Quellen systematisch
  zusammenführen. Root Cause bleibt offen; Patch0012 ist
  kein nachgewiesener Fix.
- [ ] **SO-DIMM/SPD:** Modulidentität, Rank-Anzahl und Timings
  anhand vorhandener Evidenz klären; Boarddiagnosen nur mit
  gesonderter Freigabe.
- [ ] **Bootkette:** Den Pfad SPL, U-Boot proper, Kernel,
  initrd, Device Tree und Root-Dateisystem schrittweise belegen.
- [ ] **NixOS-Image:** Universellen SD-Boot, Reproduzierbarkeit,
  Installations- und Recoveryverfahren weiter qualifizieren.

### Einordnung älterer TODO-Einträge

Die ursprünglichen P1-/P2-Abschnitte bleiben als historischer
Arbeitsstand erhalten. Soweit sie die lokale TODO-Übernahme,
den Remote-Abgleich oder die formale Projektindex-Abnahme noch
als offen darstellen, sind diese Aussagen durch die oben
genannten, späteren Checkpoints überholt.

Die Projektindex-Abnahme ist keine DDR-/SPL- oder
Hardwarequalifikation. Der I2C3-/ES8328-Root-Cause bleibt
abgeschlossen. Neue Builds, Patches, Hardwaretests und
Medien-Schreibzugriffe benötigen weiterhin eigene Freigaben.

## Checkpoint 2026-10-12 – Qualifizierte externe HDD-Sicherung R6

### Abgeschlossene Sicherung und Verifikation

- [x] Vollständigen externen HDD-Backup-Checkpoint R6 auf der foobox erstellt.
  Sicherungsziel:
  `/mnt/novena-backup-hdd/novena-archive/checkpoints/2026-10-12-complete-R6`.
- [x] Die R6-Sicherung mit 100.895 Dateien, 20 symbolischen Links
  und 71 PDF-Dateien einschließlich vollständiger SHA256-Rückleseprüfung
  dokumentiert.
- [x] Projektdateien, `novena-system`, historische Recherche,
  Protokollbestände, Runner und zugehörige Manifeste unabhängig
  nachgeprüft (NOVENA-BACKUP-R6-VERIFY-01 und VERIFY-02).
- [x] Unterschiede im laufend veränderten GitHub-Protokolltransport
  untersucht (NOVENA-BACKUP-R6-TRANSPORT-01 bis TRANSPORT-04).
  Acht veränderte Einträge lagen innerhalb der Git-Metadaten des
  tatsächlichen Repositories
  `github-transport/project-block-logs/.git/`.
  Zusätzlich wurden nach dem Backup neue Protokolldateien erzeugt.
  Eine im Backup vorhandene, später aus der Quelle verschwundene
  temporär wirkende Runner-Ausgabedatei wurde identifiziert;
  ihr vollständiger Lebenszyklus ist nicht nachgewiesen.
- [x] Die Git-Objektdatenbank des gesicherten
  `project-block-logs`-Repositories mit
  NOVENA-BACKUP-R6-GIT-FINAL-01 geprüft:
  `git fsck --full` PASS, HEAD und Tree gültig,
  `main` und `origin/main` identisch.
  Gesicherter HEAD:
  `7ab9e6befcd10ab9910774ee83fe64c36802474b`.
- [x] Externe HDD nach den Prüfungen ordnungsgemäß ausgehängt.
  Die nachfolgenden Verifikationen erfolgten ohne Schreibzugriff
  auf die HDD.

### Erkenntnisse für künftige Sicherungen

- [x] Das tatsächlich verwendete Protokoll-Git-Repository liegt
  unter `~/novena-backups/github-transport/project-block-logs`.
  Das übergeordnete Verzeichnis `github-transport` ist selbst
  kein Git-Repository.
- [x] Frühere R6-Prüfergebnisse mit unzureichend geprüften
  Vergleichsreport-Ausgaben nicht als unabhängigen
  Gleichheitsnachweis verwenden. Maßgeblich sind die
  nachfolgenden unabhängigen Verifikationsblöcke.
- [ ] Vor einem neuen Sicherungsblock vorhandene Fehlerprotokolle,
  Zielverzeichnisrechte, Mountzustand und Geräteidentität
  anhand der R1-bis-R6-Erfahrungen prüfen.
- [ ] Laufend veränderte Transport- und Protokollverzeichnisse
  bei künftigen Sicherungen mit definiertem Snapshot-Zeitpunkt
  oder nachvollziehbarer Änderungsabgrenzung behandeln.

### Noch offene Qualifikation

- [ ] Eine vollständige Wiederherstellungsprobe aus dem
  qualifizierten R6-Checkpoint gesondert planen und durchführen.
  Dafür ist eine eigene Freigabe erforderlich.
- [ ] R6-Dokumentationsänderungen nach Prüfung des bestehenden
  Git-Indexzustands gezielt versionieren und anschließend
  kontrolliert zu Codeberg und GitHub synchronisieren.

**Bewertung:** R6 ist als Backup-Checkpoint qualifiziert.
Die erfolgreiche Sicherung und die Integritätsprüfungen sind
keine bereits durchgeführte vollständige Wiederherstellungsprobe.

## Checkpoint 2026-10-12 – Dateiinventur catalog-v2 / INDEX-1K-31G

### Bestätigter Stand

- [x] Bestehenden v1-Inventurbestand mit 611.930 Dateieinträgen
  und 47 Referenzen als Ausgangsbasis dokumentiert.
- [x] Synthetischen v2-Basistest erfolgreich ausgeführt.
- [x] Acht isolierte synthetische Testfälle mit Harness v3
  erfolgreich abgeschlossen:
  BASELINE, OUTPUT_EXISTS, MISSING_SOURCE, HEADER_MISMATCH,
  TSV_COLUMN_MISMATCH, V1_ENTRY_MISMATCH,
  V1_REFERENCE_MISMATCH und HASH_MISMATCH.
- [x] Für die geprüften Negativfälle erwartete Fehlererkennung,
  Schutz vorhandener Ausgabedateien und ausbleibende temporäre
  SQLite-Reste nachgewiesen.
- [x] Vollständiges Testprotokoll
  `NOVENA-CATALOG-31G-V3-TEST-01-20261012-135311-vuziG13d.txt`
  über Runner und privaten GitHub-Protokolltransport archiviert.
- [x] Git-HEAD während des synthetischen Testlaufs unverändert;
  kein produktiver v2-Import durchgeführt.

### Offene nächste Schritte

- [ ] Produktive Quellen, Inventurdateien und v1-Katalog vor
  einem v2-Import erneut auf Integrität und Vollständigkeit prüfen.
- [ ] Produktiven catalog-v2-Import nur nach gesonderter
  ausdrücklicher Freigabe ausführen.
- [ ] Anschließend v2-Datenbank, Tabellen, Datensatzanzahlen,
  Referenzen, Provenienz und logische Fingerprints unabhängig
  gegen den bestätigten Ausgangsbestand abgleichen.
- [ ] Ergebnisse und verbleibende Inventurlücken dokumentieren;
  keinen synthetischen Test als produktive Abnahme ausgeben.

**Freigabegrenze:** Der erfolgreiche 8/8-Testlauf qualifiziert
die geprüften synthetischen Fälle, nicht den produktiven Import.
Keine Freigabe für Builds, Hardwaretests, Medien-Schreibzugriffe
oder sonstige Änderungen am Novena-Bootpfad.

## P1-E – Projektkontinuität: Starter, Übergabe und Tagesabschluss

### Projekt-Starter

- [ ] Einheitlichen Projekt-Starter für neue ChatGPT-Gespräche definieren.
- [ ] `docs/CHAT-STARTER.md` als wiederverwendbare Einstiegsanleitung erstellen.
- [ ] `docs/HANDOVER-LATEST.md` als aktuellen, belegten Übergabestand einführen.
- [ ] Datierte Übergaben unter `docs/handovers/` archivieren.
- [ ] Git-HEAD, Indexzustand, offene Nutzeränderungen, letzte geprüfte Protokolle, Projektstatus und nächsten Arbeitsschritt in die Übergabe aufnehmen.
- [ ] Veraltete und widersprüchliche Übergabedaten erkennen und vor einer Übernahme prüfen.
- [ ] Den Starter in einem neuen Projektchat praktisch testen.

### Tagesabschluss

- [ ] Einheitlichen Tagesabschluss auf der foobox entwerfen.
- [ ] Abgeschlossene, fehlgeschlagene und noch offene Blöcke anhand der Originalprotokolle bewerten.
- [ ] Bestehende TODO, PROJECT-STATE, TEST-LOG und gegebenenfalls DECISIONS kontrolliert aktualisieren.
- [ ] Projektindex und Dateiinventur auf erforderliche Fortschreibung prüfen.
- [ ] Git-HEAD, Index, Worktree und beide Remotes prüfen.
- [ ] Dokumentationsänderungen nur nach ausdrücklicher Freigabe committen und synchronisieren.
- [ ] Externe HDD-Sicherung, Integritätsnachweise und Wiederherstellungsstatus berücksichtigen.
- [ ] Keine automatischen Medien-Schreibzugriffe oder Hardwareaktionen ohne eigene Freigabe.
- [ ] Aktuelle Übergabe mit überprüften Checkpoints und offenen Aufgaben erzeugen.
- [ ] Tagesabschluss auf Vollständigkeit und Wiederaufnehmbarkeit testen.

### Werkzeuge und Integration

- [ ] Die Ergebnisse von `NOVENA-TOOLS-AUDIT-01` anhand der Originalprotokolle verifizieren.
- [ ] Vorhandene Werkzeuge SQLite, ripgrep, fd, jq, plocate, rsync und SHA256-Prüfung berücksichtigen.
- [ ] Recoll für die projektübergreifende Volltextsuche bewerten.
- [ ] Restic und Borg als mögliche Ergänzungen zum vorhandenen HDD-Backup-Konzept vergleichen.
- [ ] Bestehenden Runner und GitHub-Protokolltransport wiederverwenden.
- [ ] Keine unnötige zusätzliche Automatisierungsplattform entwickeln.
- [ ] Bestehende Freigabe- und Sicherheitsgrenzen ausdrücklich erhalten.

### Abnahmekriterien

- [ ] Ein neuer Chat kann den letzten qualifizierten Projektstand aus Übergabe, Git und Originalprotokollen rekonstruieren.
- [ ] Der Tagesabschluss erzeugt vollständige TXT-Protokolle und SHA256-Nachweise.
- [ ] Fehlgeschlagene oder unvollständige Arbeiten werden nicht als abgeschlossen dargestellt.
- [ ] Historische Evidenz, Nutzeränderungen und Git-Indexzustand bleiben erhalten.

## Checkpoint 2026-10-12 – catalog-v2 nach produktivem Import

### Erfolgreich abgeschlossen

- [x] Produktive Quellenprüfung mit neun bestätigten Eingabequellen abgeschlossen (31H).
- [x] 611.930 Dateieinträge und 47 Referenzen vollständig mit dem v1-Katalog abgeglichen.
- [x] Importplanung einschließlich Ausgabeort und Speicherreserve abgeschlossen (31I).
- [x] Produktiven catalog-v2-Import nach ausdrücklicher Freigabe ausgeführt (31J).
- [x] Neue Datenbank unter `~/novena-dateiinventur/katalog/novena-katalog-v2.sqlite` veröffentlicht.
- [x] SQLite-Integrität, Fremdschlüssel und grundlegende Datensatzanzahlen nach Import geprüft.
- [x] Ursprünglichen v1-Katalog, Importer und Git-Checkpoint unverändert erhalten.

### Noch offene Endabnahme

- [ ] Unabhängigen Audit 31K nach Korrektur des Prüfskripts abschließen.
- [ ] Die tatsächlichen SQLite-Spaltennamen vor Wiederholung der Klassifikations- und Referenzauflösungsprüfung kontrollieren.
- [ ] Ergebnisse der erfolgreichen 31K-Teilprüfungen erhalten: SQLite-Integrität, neun Tabellen, zehn Indizes, sieben Fingerprints, 611.930 Einträge und 47 Referenzen.
- [ ] Klassifikationen und Referenzauflösungen unabhängig gegen v1 prüfen.
- [ ] Projektquellen und Provenienzbeziehungen unabhängig abschließend kontrollieren.
- [ ] Erst nach erfolgreicher vollständiger Endprüfung catalog-v2 als technisch abgenommen dokumentieren.

**Aktueller Status:** Der produktive catalog-v2-Import ist erfolgreich.
Die unabhängige Endabnahme ist noch offen. 31K brach bei der
Klassifikationsprüfung mit `sqlite3.OperationalError: no such column: path`
ab. Dies ist ein Fehler im Prüfskript; aus dem Abbruch allein folgt
kein Datenbankdefekt.

**Freigabegrenzen:** Keine neue Freigabe für Builds, Patches,
Hardwaretests, Medien-Schreibzugriffe oder sonstige Änderungen
am Novena-Bootpfad.
