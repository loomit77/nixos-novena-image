# TODO – Novena: universelles NixOS-SD-Image

> Aktueller Workflowstand: siehe Nachtrag vom 2026-10-17 am Dateiende (NOVENA-WORKFLOW-FLOCK-02 PASS; konkreter Konkurrenzfall abgeschlossen). Frühere Statusangaben sind datierte historische Stände.

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

**Historischer Zwischenstand vor Abschluss der 31K-Nachprüfungen:** Der produktive catalog-v2-Import war erfolgreich; die unabhängige Endabnahme war zu diesem Zeitpunkt noch offen. Der spätere Abschluss ist im nachfolgenden Abschnitt dokumentiert. 31K brach bei der
Klassifikationsprüfung mit `sqlite3.OperationalError: no such column: path`
ab. Dies ist ein Fehler im Prüfskript; aus dem Abbruch allein folgt
kein Datenbankdefekt.

**Freigabegrenzen:** Keine neue Freigabe für Builds, Patches,
Hardwaretests, Medien-Schreibzugriffe oder sonstige Änderungen
am Novena-Bootpfad.

## 2026-10-12 – catalog-v2: technische Endabnahme

- [x] Produktiven catalog-v2-Import mit 611.930 Einträgen
  und qualifizierter Datenbankidentität abgeschlossen (31J).
- [x] Unabhängige Integritäts-, Schema-, Fingerprint- und
  v1/v2-Vergleichsprüfungen durchgeführt (31K).
- [x] 611.930 übernommene Klassifikationen, 47 übernommene
  Referenzauflösungen und 46.026 abgeleitete Verzeichnisse
  unabhängig geprüft.
- [x] 18 Projektquellen und neun Provenienzquellen geprüft.
- [x] Zwei fehlgeschlagene 31K-Prüfläufe als Prüfskriptfehler
  eingeordnet: der ursprüngliche Audit und FINAL-AUDIT-02.
  SCHEMA-REVIEW-01 und die abschließende gezielte
  PROVENANCE-FINAL-03-Prüfung bestanden.
- [ ] Inventurabdeckung weiterer historischer Medien,
  Dubletten und Quellenlücken getrennt weiter untersuchen.
- [ ] catalog-v2 mit dem Projektindex und späteren
  Snapshot-/Resolver-Erweiterungen verknüpfen.

Produktive Datenbank:
`~/novena-dateiinventur/katalog/novena-katalog-v2.sqlite`

SHA256:
`9ac6fa2eb7e4c0dbf5564e431a1b1f350ec855dfe366eb1400cc1b66792a237d`

Der Abschluss betrifft den geprüften Katalogdatenbestand,
nicht die Vollständigkeit sämtlicher historischer Medien.


### Verifizierte Originalprotokolle

Die folgenden lokalen TXT-Dateien wurden anhand ihrer SHA256-Sidecars geprüft. Die beiden FAIL-Ergebnisse bleiben ausdrücklich erhalten.

- `NOVENA-CATALOG-31K-INDEPENDENT-AUDIT-01-20261012-143702-beSg0zOn.txt` — SHA256 `f6aba93a0ae133bafa500bd862046ed22e2d3d7b2022c4a1fe63c504ae83a16d` — `FAIL`.

- `NOVENA-CATALOG-31K-SCHEMA-REVIEW-01-20261012-144632-9hrzp95G.txt` — SHA256 `6eb2694e88f08700d0594ba8d66e92dc2afb8c9a3773e2f9d1ee63be25d35432` — `PASS`.

- `NOVENA-CATALOG-31K-FINAL-AUDIT-02-20261012-144749-ffQdXkr4.txt` — SHA256 `6b43f4c5166776753063432353d7a98ed0b858c9b35443a16f4afd45b633df66` — `FAIL`.

- `NOVENA-CATALOG-31K-PROVENANCE-FINAL-03-20261012-144852-Q37MqFNZ.txt` — SHA256 `bfb1a8d45676bf5af6aef0449e6c426d0d7cc9f22d03a412d3ba6ae3736c134c` — `PASS`.

## Nachtrag 2026-10-12 – Erste Chat-Übergabe und Prioritäten

### Bestätigt abgeschlossen

- [x] Erste praktische Chat-Übergabe durchgeführt.
- [x] `docs/CHAT-STARTER.md` erstellt und erweitert.
- [x] `docs/HANDOVER-LATEST.md` erstellt und erweitert.
- [x] Automatischen Abruf von GitHub-Blockprotokollen
  in der praktischen Projektarbeit bestätigt.
- [x] Dokumentationscommit `3ac6b4f` auf Codeberg und
  GitHub synchronisiert (`NOVENA-CHAT-CONTINUITY-10`).
- [x] Ursprünglichen Git-Index beim Abschluss erhalten.

### Verbindliche Reihenfolge der nächsten Arbeiten

**P1 – Starter**

- [ ] Erkenntnisse der ersten Übergabe vollständig integrieren.
- [ ] Datierte Übergaben und deren Archivierung festlegen.
- [ ] Übergabeablauf ohne redundante Prüfungen abschließen.

**P1 – Tagesabschluss**

- [ ] Einheitliches Tagesabschlussverfahren entwerfen.
- [ ] Originalprotokolle, TODO, Git-Zustand, offene Aufgaben
  und aktuelle Übergabe zusammenführen.
- [ ] Vorhandenen Runner und GitHub-Transport wiederverwenden.
- [ ] Freigabegrenzen und historische Evidenz erhalten.

**P1 – Werkzeuge**

- [ ] Vorhandenen `NOVENA-TOOLS-AUDIT-01` auswerten.
- [ ] Bereits verfügbare Werkzeuge gezielt verwenden.
- [ ] Recoll, Restic und Borg nur bei konkretem Nutzen bewerten.
- [ ] Keine unnötige neue Automatisierungsplattform entwickeln.

Technische DDR-/SPL- und Image-Arbeiten bleiben offen,
werden aber bis zur gesonderten Priorisierung zurückgestellt.

Die älteren TODO-Einträge bleiben als historische
Planungsstände erhalten. Dieser Nachtrag beschreibt
die neuere Priorisierung.

## Nachtrag 2026-10-12 – NOVENA-DAY-CLOSE-11: belegte Teilabschlüsse

Die älteren offenen Kästchen bleiben historische Planungsstände. Für die
Tagesabschlussaufgaben gilt jetzt die folgende begrenzte Fortschreibung;
DDR/SPL-, catalog-v2- und I2C3-Aufgaben werden hier nicht verändert.

- [x] Einheitliches Tagesabschlussverfahren entworfen und in
  `docs/DAILY-CLOSE.md` dokumentiert; technische Erprobung im geprüften
  Teilumfang erfolgreich (DAY-CLOSE-04/06/07), Gesamtqualifikation offen.
- [x] Verfahrensdokument erfolgreich committed und zu Codeberg/GitHub
  gepusht (DAY-CLOSE-08, Commit `5e2f7afbad802b49b8305dd2a5437c8550bae2be`);
  ursprünglichen Index erhalten. Keine neue Remote-Prüfung damit behauptet.
- [x] Abgeschlossenen lokalen Protokollbestand zeitlich begrenzt erhoben:
  DAY-CLOSE-09 ab 19:56:19 CEST, 173 Logs, 135 PASS/38 FAIL;
  DAY-CLOSE-10 ab 19:59:33 CEST, 174 Logs, 136 PASS/38 FAIL.
  Jeweils laufendes Eigenprotokoll ausgeschlossen; keine volle Tagesabdeckung.
- [x] DAY-CLOSE-10 erfolgreich abgeschlossen und protokollierte Codex-Analyse
  mit Zuordnung der 38 historischen FAILs als Analyse berücksichtigt.
  Historische FAILs bleiben unverändert; keine endgültige Gesamtqualifikation.
- [x] Bestehenden Runner und GitHub-Wrapper im Tagesabschlussverfahren
  verbindlich erhalten; keine neue Automatisierung eingeführt.
- [x] R6 als qualifizierten Sicherungscheckpoint mit offenem vollständigem
  Restore eingeordnet; Cleanup-Restore davon getrennt.
- [x] DAILY-CLOSE, HANDOVER-LATEST, CHAT-STARTER und TODO im freigegebenen
  Dokumentationsumfang datiert ergänzt (DAY-CLOSE-11).
- [ ] Tages-Sollumfang einschließlich Zeitraum, aller relevanten Arbeiten,
  begründeter Ausschlüsse und fehlender Nachweise verbindlich festlegen.
- [ ] Aktualisierte Übergabe fachlich abnehmen; gespeicherter Nachtrag
  allein ist kein vollständiger Übergabenachweis.
- [ ] Wiederaufnahmekriterium festlegen und praktische Wiederaufnahme mit
  aktualisierter Übergabe und Starter prüfen; hier nicht ausgeführt.
- [ ] Vollständigen Tagesabschluss und fachliche Gesamtqualifikation
  nachweisen; bislang begrenzte Analyse mit Bewertung PARTIAL.
- [ ] Vollständigen R6-Restore gesondert freigeben und nachweisen.
- [ ] NOVENA-TOOLS-AUDIT-01 fachlich auswerten und Werkzeugentscheidungen
  begründen; nicht Teil dieser Dokumentationsaktualisierung.

Evidenz: vollständige DAY-CLOSE-08/09/10-Originale samt geprüften Sidecars;
exakte Dateinamen und Hashes im Nachtrag zu `docs/DAILY-CLOSE.md`.
Die dokumentierte Aktualisierung ist ausgeführt; ihre Übergabeabnahme und
praktische Wiederaufnahme bleiben offen. Weitere Projektdateiänderungen,
Commits, Pushes, Builds, Netzwerk-, Medien- und Hardwareaktionen sind
durch diesen Nachtrag nicht freigegeben.

## Nachtrag 2026-10-12 – NOVENA-TOOLS-CLOSE-01: begrenzter Werkzeugabschluss

Dieser Nachtrag schreibt ausschließlich die Werkzeugaufgaben fort. Ältere
Werkzeugkästchen bleiben als historische Planungsstände erhalten; für ihren
aktuellen Status gilt dieser Nachtrag. Fachfremde Aufgaben bleiben unverändert.

- [x] NOVENA-TOOLS-AUDIT-01 und NOVENA-TOOLS-AUDIT-02 vollständig anhand
  der lokalen Originalprotokolle ausgewertet; SHA256 stimmt jeweils mit
  dem vorhandenen Sidecar überein.
- [x] Werkzeugauswahl dokumentiert: vorhandene `sqlite3`, `rg`, `fd`,
  `find`, `jq`, `plocate`, `rsync`, `sha256sum` und `b2sum` weiterverwenden.
  Dies schließt die Auswahlentscheidung ab, keine Funktionsqualifikation.
- [x] Beibehaltung von TSV, JSON und Python für den Projektindex beschlossen.
  Keine SQLite-Integration ohne konkreten Mehrwert und eigene Qualifikation.
- [x] Weiterverwendung des bestehenden Runners und GitHub-Wrappers ohne
  Änderung beschlossen; keine zusätzliche Automatisierungsplattform.
- [x] Kein aktueller Installationsbedarf für rmlint und Czkawka festgelegt.
- [ ] **Zurückgestellt:** Recoll erst bei nachgewiesenem Bedarf für
  dokumentübergreifende Volltextsuche neu bewerten; nicht installieren.
- [ ] **Zurückgestellt:** Restic und Borg erst bei gesonderter Überarbeitung
  des Backup-Konzepts vergleichen; nicht installieren.
- [ ] **Zurückgestellt:** SQLite-Integration des Projektindex nur bei
  konkretem Mehrwert gesondert bewerten und qualifizieren.

Primärnachweise sind ausschließlich die beiden Originalaudits unter
`~/novena-backups/block-logs/`; genaue Dateinamen und Hashes stehen im
[Entscheidungsnachtrag](docs/DECISIONS.md#2026-10-12--novena-tools-close-01-begrenzter-werkzeugabschluss).
Beide Audits sind read-only und belegen Werkzeugverfügbarkeit, keine umfassende
Funktionsqualifikation. Historische FAILs bleiben unverändert. Der vollständige
R6-Restore bleibt offen. Keine Installation, Werkzeugtests oder Änderungen am
Projektindex beziehungsweise catalog-v2 wurden für diesen Abschluss ausgeführt.

## Nachtrag 2026-10-13 – NOVENA-OPENBSD-08: Historische OpenBSD-Spur

- [x] Historische OpenBSD-Novena-Unterstützung von 2015
  anhand des ursprünglichen Commits und Folgebelegen geprüft.
- [x] OpenBSD-U-Boot-Ports-Import von 2016.05,
  `novena_defconfig` und Board-ID-Patch nachgewiesen.
- [x] Patchentfernung beim Update auf U-Boot 2016.09
  anhand der Ports-Historie nachgewiesen.
- [ ] Exakte SPL-/U-Boot-Binäridentität des
  OpenBSD-Novena-Tests von 2015 bleibt unbekannt.
- [ ] Optional prüfen, ob U-Boot 2016.05 als zusätzlicher
  DDR-/SPL-Quellvergleich gegenüber GOOD/r7 und
  aktuellen Diagnosequellen einen Erkenntnisgewinn bietet.
- [ ] RM-05Q erst nach gesonderter Priorisierung fortsetzen.

Der OpenBSD-Betrieb ist kein Nachweis einer eigenen
DDR-Initialisierung oder eines Fixes des aktuellen
SPL-/DDR-Fehlers. Bestehende Projektprioritäten,
Freigabegrenzen und historische TODO-Einträge bleiben erhalten.


## Nachtrag 2026-10-14 – Wiederaufnahme, Quellenanalyse und offene Nachweise

Dieser Nachtrag ergänzt die historischen Aufgabenstände, ohne bestehende Kästchen oder Ergebnisse umzudeuten. Die Anforderungen der Arbeitsregeln 13 und 14 vom 2026-10-14 sind in Starter, Tagesabschluss und Übergabe ausformuliert; ihre praktische Erfüllung und fachliche Gesamtqualifikation sind damit nicht nachgewiesen.

Übergebener Git-Checkpoint: HEAD `7d98296db282aed0cadbf1d6cf9545ccb591f8a6`, Index-SHA256 `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`. Die Werte stammen aus dem Auftrag und wurden hier nicht durch Git-Befehle verifiziert. Index, Intent-to-add-Einträge und vorhandene Nutzeränderungen bleiben geschützt.

- [ ] Vor neuen technischen Empfehlungen oder Arbeitsblöcken die verbindliche Wiederaufnahme nach Arbeitsregel 14 durchführen: Regeln, neueste Übergabe, letzter Tagesabschluss, relevante Projektdateien und letzte verfügbare Originalprotokolle auswerten und Widersprüche beziehungsweise Zugriffslücken benennen.
- [ ] Die projektweite Quellenanalyse nach Arbeitsregel 13 im relevanten erlaubten Umfang durchführen: gesamte Boot- und Systemarchitektur, historische Implementierungen, Schnittstellen, Abhängigkeiten, bestätigte Ursachen und ausgeschlossene Hypothesen berücksichtigen; Risiken, Alternativen und Folgen für spätere Schritte bewerten und Prioritätsänderungen begründen.
- [ ] Bei berechtigter GitHub-Anbindung und konkreter Netzwerkfreigabe benötigte Original-TXT-Dateien und SHA256-Sidecars selbstständig in `loomit77/project-block-logs` unter `novena/YYYY/MM/` anhand der Blockkennung suchen, vollständig abrufen und auswerten. Lokal zugängliche Quellen zuerst verwenden; keinen erneuten Upload verlangen, solange die Dateien erreichbar sind. Nicht abgerufene Inhalte bleiben ungeprüft.
- [ ] Für jeden relevanten Block Uploadstatus, Runnerstatus, Integritätsprüfung und fachliches Resultat getrennt mit dem jeweils belegten Umfang erfassen. Erfolgsmarker, Hashgleichheit und Ausführungsende ersetzen keine fachliche Abnahme.
- [ ] ARCHIVE-08 anhand des vollständigen Originals und Sidecars abschließend auswerten: Ausführung beendet, Runner-FAIL, zwei Downloadfehler. Fehlende Quellen und Teilresultate zuordnen; fachlicher Archivabschluss bleibt offen. Keine automatische Wiederholung oder unkontrollierte Nacharchivierung.
- [ ] DDR-08H mit vollständiger serieller Evidenz und Artefaktprovenienz einordnen: Q1/C1 ZERO, D32 `ret=-12`; vollständiger Boot und Patchfix bleiben unbestätigt. DDR/SPL-Root-Cause und externer Gesamtboot bleiben offen. Daraus keinen weiteren Hardwareversuch ohne gesonderte Freigabe ableiten.
- [ ] Vollständigen Tages-Sollumfang, Zeitraum und Ausschlüsse festlegen; Git-, Datei-, Artefakt-, Backup-, Manifest-, Originalprotokoll- und Sidecaridentitäten mit Prüfzeitpunkt und Herkunft dokumentieren. Fehlende Werte als Lücken erhalten; R6-Sicherung und offenen vollständigen Restore getrennt bewerten.
- [ ] Parallele Arbeiten mit Zuständigkeiten und Ressourcen abgrenzen. Vorbereitete, laufende, fehlgeschlagene und abgeschlossene Blöcke unterscheiden; unvollständige Prüfungen und laufende Eigenprotokolle nicht als abgeschlossene Nachweise verwenden.
- [ ] Vor Änderungen bestehender Skripte vollständigen relevanten Quellcode, Schnittstellen, Annahmen und Abhängigkeiten prüfen. Vorbereitung und Kommandodateierzeugung innerhalb der Freigabe protokollieren; verbleibende unprotokollierte Schritte als Nachweislücken benennen.
- [ ] Aktualisierte Übergabe und Wiederaufnehmbarkeit nach definiertem Kriterium prüfen; vollständigen Tagesabschluss und Verfahrensqualifikation erst nach vorhandenen konsistenten Pflichtnachweisen bewerten.

I2C3/ES8328 bleibt geschlossen; `regulator-always-on` bleibt dauerhafte Boardanforderung. Die fachlichen Angaben zu ARCHIVE-08 und DDR-08H sind hier aus dem Auftrag übernommen; ihre Originalnamen, Hashes, Upload- und Integritätsnachweise wurden nicht neu erhoben. Diese Dokumentation erteilt keine zusätzlichen Schreib-, Netzwerk-, Git-, Build-, Test- oder Hardwarefreigaben.

> Aktueller Abschluss: siehe den Nachtrag „2026-10-15 – Abschluss und Übergabe nach Borg 35P-19“ am Dateiende; ältere Stände sind historisch.

## 2026-10-15 – Abschluss und Übergabe nach Borg 35P-19

Dieser datierte Nachtrag ist für den hier beschriebenen Abschluss maßgeblich.
Ältere Statusangaben bleiben historische Checkpoints. Grundlage sind die
vollständig ausgewerteten lokalen TXT-Originale samt übereinstimmenden
SHA256-Sidecars; das Nachweisregister steht im gleichnamigen Nachtrag in
`docs/TEST-LOG.md` (aus `docs/` heraus: `TEST-LOG.md`).

- **Borg-27:** nachgewiesene reale Sicherung am 15.10.2026,
  13:29:56–13:40:47 CEST. `BORGMATIC_CREATE_EXIT=0`, genau ein neues Archiv
  (Bestand 2 → 3), `ARCHIVE_CHECK=PASS` und
  `ARCHIVE_DATA_VERIFICATION=PASS`. Dies ist keine Restore-Abnahme.
- **35O-R5:** 64 erfolgreiche Offline-Tests in fünf Suiten
  (8 + 22 + 10 + 15 + 9); keine reale Borg-Ausführung in diesem Testblock.
- **35P-12:** R4-Laufzeitnachweis erfolgreich: Downstream betreten,
  äußere Sperre während der Bridge gehalten, konkurrierender R4-Aufruf
  abgewiesen, Bridge abgeschlossen, Sperre nach Rückkehr freigegeben.
  **Der R4-Laufzeitnachweis ist nicht mehr offen.** R4 ist ein
  Offline-Prüfmechanismus; daraus folgt keine produktive Repository-Sperre
  für eine reale Sicherung und keine produktive Gesamtfreigabe.
- **35P-14:** FAIL, Exitstatus 1 wegen eines fehlerhaft formatierten
  SHA256-Prüfblocks. Kein daraus belegter Quelldefekt. **35P-14R1:**
  korrigierte Aufrufpfadanalyse PASS; der Testdateihash wurde erfasst,
  aber ausdrücklich nicht mit einem historischen Sollhash verglichen.
  Der frühere FAIL bleibt erhalten.
- **35P-15/16:** Kontrollfluss-, Syntax- beziehungsweise Strukturanalyse
  des bestehenden Tagesabschlusses erfolgreich; keine reale Sicherung.
- **35P-17:** Offline-Borg-Nachweisprüfer erfolgreich gegen Borg-27,
  einschließlich Protokollhash, Sidecar, Archivanzahl, Archiv-ID,
  Archivprüfung, Datenverifikation und Runnernachweis.
- **35P-18:** Offline-Integrationstest und Abweisung einer nicht
  autorisierten Tagesabschlussausführung erfolgreich. Der Starter
  `~/novena-backups/workflow-improvement-35/borg/novena-dayclose-with-borg-35p18.bash`
  prüft den historischen Borg-27-Nachweis und startet keine Sicherung.
  Die Nachweisauswahl ist noch fest gebunden, nicht dynamisch.
- **35P-19:** integrierter Tagesabschluss erfolgreich ausgeführt
  (16:06:05 CEST; `DAYCLOSE_EXECUTED=YES`,
  `DAYCLOSE_INTEGRATION=PASS`, Runner-PASS, Exitstatus 0).
  Dabei kein Borg-/borgmatic-Aufruf und kein Borg-Repositoryzugriff.

**Korrektur veralteter Statusausgaben:** Der in 35P-19 eingebettete ältere
Tagesabschluss meldet weiterhin `BORG_FULL_LOCK_DURATION_PROOF=OPEN` und
führt den Laufzeitnachweis als offene Aufgabe. Das ist durch 35P-12 überholt,
kein neuer Fehlschlag. Auch die damaligen pauschalen offenen Integrationspunkte
sind anhand 35P-17/18/19 auf den tatsächlich erreichten Offline-Nachweisumfang
zu begrenzen. Die Skriptausgabe bleibt unverändert als Originalevidenz erhalten.
Der belegte Ausführungserfolg von 35P-19 ersetzt weder eine unabhängige
Vollständigkeitsinventur aller Tagesarbeiten noch eine produktive Borg-Abnahme.
Upload beziehungsweise Remote-Archivierung wurden in diesem Dokumentationsauftrag
nicht erneut geprüft; lokale Logintegrität und fachliche Ergebnisse sind davon getrennt.

**Offen:** Restore-Abnahme, Offsite-Schlüsselsicherung einschließlich
Wiederherstellbarkeit und produktive Gesamtfreigabe. Die dynamische Auswahl
eines geeigneten aktuellen Sicherungsnachweises sowie die produktive
Sperr-, Freigabe-, Provenienz- und Nachweiserfassung bleiben gesondert zu
qualifizieren. Der vollständige R6-Restore bleibt eine separate offene Aufgabe.

**Nächste Projektpriorität: Novena – universelles NixOS SD-Image.**
Borg-Arbeiten sind für die Wiederaufnahme zurückgestellt. Vor weiteren Builds,
Hardwaretests oder Änderungen zuerst den aktuellen Projektstand, TODO,
Artefaktprovenienz und vollständige vorhandene Boot-/Testnachweise prüfen.
DDR/SPL und der vollständige externe Bootpfad bleiben offen; der dokumentierte
DDR-08H-Befund ist Q1/C1 ZERO und D32 `ret=-12`, kein bestätigter Patchfix
oder vollständiger Boot. I2C3/ES8328 bleibt abgeschlossen;
`regulator-always-on` bleibt dauerhafte Boardanforderung.

### Aktuelle Aufgabenliste für den nächsten Chat

Die folgenden Einträge ersetzen die abweichende Priorisierung älterer
Nachträge für die Wiederaufnahme; historische Kästchen bleiben erhalten.

- [x] Borg-27 als reale erfolgreiche Sicherung mit Archiv- und Datenprüfung belegt.
- [x] 35O-R5: 64 Offline-Tests erfolgreich.
- [x] R4-Laufzeitnachweis durch 35P-12 abgeschlossen.
- [x] Aufrufpfad und Tagesabschluss analysiert (35P-14R1/15/16);
  fehlerhaften SHA256-Prüfblock 35P-14 als FAIL erhalten.
- [x] Nachweisprüfer, Offline-Integration und integrierten Tagesabschluss
  erfolgreich belegt (35P-17/18/19).
- [x] Dokumentation, TODO und Übergabe für den Abschluss 15.10.2026 ergänzt.
- [ ] **P1 SD-Image:** Projektstand, TODO und vollständige vorhandene
  Boot-/Testnachweise einschließlich DDR-08H und Artefaktprovenienz prüfen;
  danach den minimalen nächsten Projektblock begründen und konkret freigeben lassen.
- [ ] DDR/SPL-Ursache und vollständigen externen SD-Boot weiter qualifizieren.
- [ ] **Borg, zurückgestellt:** dynamische Nachweisauswahl sowie produktive
  Integration einschließlich Sperre, Freigabe und Provenienz qualifizieren.
- [ ] Restore-Abnahme und Offsite-Schlüsselsicherung nachweisen.
- [ ] Produktive Borg-Gesamtfreigabe gesondert entscheiden; aktuell BLOCKED.
- [ ] Vollständigen R6-Restore separat planen und freigeben lassen.
- [ ] Neue Übergabe im nächsten Chat anhand der Originale praktisch prüfen.

Keine neue Build-, Hardware-, Medien-, Borg-, Netzwerk- oder Git-Freigabe
folgt aus der Priorisierung. Keine weitere allgemeine Auditserie als
Voraussetzung ohne konkreten Erkenntniszweck beginnen.


## 2026-10-17 – Zwei-Skripte-Workflow: realer E2E-Test PASS

Dieser Nachtrag ist für den aktuellen Bedienungsworkflow maßgeblich.
Ältere Planungs-, Sperr- und Integrationshinweise bleiben historische Stände.
Die älteren `integration/*`- und V8-Dateien bleiben unverändert erhalten.

Aktuelle Bedienungsskripte auf foobox:

- Projektstart: `~/novena-backups/workflow-dev/chat-start.bash`.
- Tagesabschluss: `~/novena-backups/workflow-dev/daily-close.bash`.

Beide werden über den vorhandenen GitHub-Wrapper
`~/novena-backups/github-transport/run-and-upload.bash` ausgeführt.
Die Dokumentation erteilt keine Ausführungs- oder Netzwerkfreigabe.
Der Tagesabschluss führt eine reale Borg-Sicherung aus; sein Aufruf benötigt
entsprechende Freigabe und ist keine rein lesende Statusabfrage.

- **NOVENA-WORKFLOW-TWO-SCRIPTS-04: PASS** – beide Skripte installiert.
- **NOVENA-CHAT-START-TEST-05: PASS** – Projektstart, Git-HEAD,
  Index-SHA256, Worktree, Dokumentinventur und Übergabe erfolgreich.
- **NOVENA-DAILYCLOSE-HARDEN-06: PASS** – Installation mit flock-Sperre,
  Borg-Identitätsprüfung und Archivvergleich am 17.10., 09:45:53 CEST.
  Ein separater Wiederholungsaufruf um 09:47:46–09:47:47 endete mit
  `BASELINE_INTEGRITY=FAIL`, Exit 22; dieser FAIL bleibt erhalten.
- **NOVENA-DAILYCLOSE-E2E-07: PASS** – realer End-to-End-Lauf am
  17.10.2026, 09:54:18–09:54:55 CEST. Exklusive Sperre erworben,
  Borg-Identitätsprüfung und Borgmatic-Konfiguration erfolgreich,
  Backup erstellt. Archivbestand **5 → 6**, genau **1 neues Archiv**,
  **0 verschwundene Archive**. Git-HEAD und Index unverändert.
  Vollständiges TXT-Protokoll und SHA256 vorhanden; GitHub-Upload laut
  verifiziertem Benutzerergebnis erfolgreich, hier nicht erneut abgefragt.

**Abgrenzung:** Der reale E2E-Nachweis ist erfolgreich; ältere pauschale
`PRODUCTIVE_END_TO_END_PROOF=OPEN`- beziehungsweise daraus abgeleitete
Sperrhinweise beschreiben den damaligen Workflow. Ein separater
Parallelstart-/Konkurrenztest der aktuellen flock-Sperre wurde noch nicht
durchgeführt und bleibt offen. Weder der Sperrerwerb im Einzellauf noch
historische R4-/Offline-Tests belegen diesen Konkurrenzfall. Eine umfassende
Fehlerfallqualifikation oder produktive Gesamtfreigabe folgt daraus nicht.

Evidenz: lokal vollständig gelesene Originalprotokolle und jeweils
übereinstimmende SHA256-Sidecars; das Nachweisregister steht im
[TEST-LOG](docs/TEST-LOG.md#2026-10-17--zwei-skripte-workflow-realer-e2e-test-pass).

### Aktuelle Workflow-Aufgaben

Die älteren offenen Kästchen unter P1-E und die pauschale Aufgabe zur
produktiven Integration sind für den oben belegten Zwei-Skripte-Umfang
überholt; weitergehende Abnahmen bleiben getrennt.

- [x] Zwei Bedienungsskripte installiert und Projektstart getestet (04/05).
- [x] Tagesabschluss gehärtet und realen E2E-Lauf erfolgreich durchgeführt (06/07).
- [x] Separaten Parallelstart-/Konkurrenztest für flock abgeschlossen:
  NOVENA-WORKFLOW-FLOCK-02, 17.10.2026, PASS; zweiter Start bei
  gehaltenem Lock mit Exit 40 abgewiesen, Backup-Phase nicht erreicht.
- [ ] Weitergehende Fehlerfallqualifikation und produktive Gesamtfreigabe
  gesondert bewerten; E2E-PASS nicht als vollständige Abnahme ausgeben.
- [ ] Restore-Abnahme, Offsite-Schlüsselsicherung und vollständigen R6-Restore
  weiterhin getrennt nachweisen.

DDR/SPL und der vollständige externe SD-Boot bleiben offene Projektaufgaben.

## 2026-10-17 – NOVENA-WORKFLOW-FLOCK-02: Konkurrenztest PASS

**NOVENA-WORKFLOW-FLOCK-02: PASS.** Der zuvor offene Konkurrenztest
ist für den konkreten Fall „zweiter Aufruf von `daily-close.bash` bei bereits
gehaltener flock-Sperre“ qualifiziert abgeschlossen: sofortige Abweisung
mit Exit-Code 40; die Backup-Phase wurde nicht erreicht.
Ergebniswerte, Git-Checkpoint, Originalprotokoll und Evidenzherkunft sind
zentral im [TEST-LOG](docs/TEST-LOG.md#2026-10-17--novena-workflow-flock-02-konkurrenztest-pass) dokumentiert.

Dieser Nachtrag ersetzt die frühere Offenbewertung dieses Konkurrenzfalls.
Die historischen E2E- und Fehlerbefunde bleiben erhalten. Eine vollständige
Qualifikation sämtlicher Workflow-Fehlerfälle oder produktive Gesamtfreigabe
folgt daraus nicht; weitergehende Fehlerfälle und Restore-Nachweise bleiben
getrennte offene Aufgaben.


## 2026-10-17 – NOVENA-DEBIAN-DOC-01B: statische Teilbefunde und offene Nachweise

Fortschreibung zu NOVENA-STRATEGY-02; ältere Kästchen bleiben historische
Planungsstände. Gesamtaufgabe SOURCE-18 bleibt offen.
[Technische Belege](docs/HISTORICAL-MEDIA.md#2026-10-17--novena-debian-doc-01b-bootvertrag-der-debian-paketlinie-2015)
und [Identitäten](docs/QUELLENVERZEICHNIS.md#2026-10-17--novena-debian-doc-01b-ref-09-bis-ref-14-quellenregister)
sind getrennt von Hardwarequalifikation zu lesen.

Erledigte statische Untersuchungen / Dokumentationsintegration:

- [x] REF-09/10: 2015-Referenzcommit, externe/interne SPL-Controllerwahl,
  SPL-/Proper-MMC-Nummerierung und Commit-Abstammung eingeordnet.
- [x] REF-11/12: SPL-Dateisystemmodus, FAT-Partition 1 und `u-boot.img`;
  vorhandene Raw-Konstante vom ausgewählten Ladevertrag getrennt.
- [x] REF-13/14: EEPROM-SATA-Feature `0x0100`, MMC-0-Environment,
  doppelte Defaults, uEnv-Import, Hooks, extlinux-/Skriptscan und
  zImage-/DTB-Fallback einschließlich NovM-/NovS-Rootsemantik dokumentiert.
- [x] REF-09 bis REF-14: Originalnamen und SHA256-/Sidecarabgleich registriert;
  Extraktionsbelege REF-08/08R samt FAIL/Folge-PASS und Binäridentitäten benannt.
- [x] 2014-Roh-/FAT-Generationen, 2015-Paketpaar und 2020.07-SPL getrennt;
  USB-Master weiterhin als Hybridzustand eingeordnet.

Noch offene technische Nachweise (hier nicht ausgeführt):

- [ ] Vollständige ursprüngliche Paket-/Build- und Binär-zu-Quellprovenienz
  des 2015-Paars; keine Gleichsetzung mit unverändertem Factory-R1/GOOD.
- [ ] Konkreten externen ROM-/SPL-/Proper-Lade- und Übergabevertrag samt
  Headern, Adressen, Speicherbereichen und Gesamtimage qualifizieren.
- [ ] Proper-Initialisierung und Environment-Fallback ohne erforderliche
  interne MMC nachweisen; tatsächliche persistente Werte bestimmen.
- [ ] Effektive EEPROM-/uEnv-/earlyhook-/finalhook-Wirkung einschließlich
  Gültigkeit des erneut importierten Speicherinhalts klären.
- [ ] Externe Nutzlast-/Root-Auswahl auf derselben SD eindeutig nachweisen;
  NovM-/NovS-PARTUUIDs und Kollisionen mit anderen Medien berücksichtigen.
- [ ] Kernel-/initrd-/DTB-Identitäten, Formate, Größen, Lade-/Entpack- und
  Übergabebereiche sowie Debian-Kompatibilität qualifizieren.
- [ ] Vollständigen eigenständigen P_EXT-Boot bis Debian gesondert nachweisen;
  erst danach gemäß STRATEGY-02 moderner Kernel, NixOS und Image-Builder.

Diese offenen Punkte erteilen keine Aktionsfreigabe. Keine neue Bootloaderwahl;
moderne DDR-Diagnose bleibt zurückgestellt und ihre Root Cause offen.
I2C3/ES8328 bleibt geschlossen; `regulator-always-on` dauerhaft.
