# TODO – Novena: universelles NixOS-SD-Image

Stand: 2026-10-11 (GitHub-Direktupload-Checkpoint; lokaler foobox-Stand separat)
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
- [ ] `NOVENA-ARBEITSREGELN.txt` und bei Bedarf `AGENTS.md` hinsichtlich Wrapper, automatischer Übertragung und Sicherheitsgrenzen kontrolliert aktualisieren. Die bisherige Pflicht zur manuellen Inhaltsprüfung vor externer Weitergabe steht noch im Widerspruch zum automatischen Direktupload; vor einer dauerhaften Regeländerung explizit klären, wie sensible Inhalte vor Übertragung ausgeschlossen werden.
- [ ] Dokumentation des Direktuploads im Transportverzeichnis anlegen und gegen den tatsächlich eingesetzten Wrapper prüfen (GITHUB-UPLOAD-18: noch nicht vorhanden).
- [ ] Wiederaufnahme und gezielte Nacharchivierung nach Upload-/Netzwerkfehler kontrolliert prüfen; kein unkontrollierter Massen-Backfill.
- [ ] Verbleibende Fälle des 22-teiligen Uploader-Testplans gezielt qualifizieren; bisherige erfolgreiche Funktionstests nicht als vollständige Testplan-Abnahme ausgeben.
- [ ] Idempotenz, Race-Conditions, Push-Fehler, Konflikte und unabhängige Remote-Hashprüfung entsprechend dem noch offenen Testumfang belegen.
- [ ] Projektübergreifende Zuordnung für Novena, L14, Homelab und weitere Projekte eindeutig regeln.
- [ ] Grenzen dokumentieren: Private GitHub-Repositories sind keine Geheimnisspeicher; GitHub-Lesezugriff kann manuelle ChatGPT-Dateiuploads reduzieren, hebt Produktlimits aber nicht auf.

**Betriebsstand 2026-10-11:** Der automatische Direktupload wird für die aktuellen Novena-Blöcke praktisch verwendet und funktioniert in den geprüften Erfolgs- und Fehlerfällen. Die vollständige Uploader-Qualifikation, der Ausfall-Wiederanlauf und die widerspruchsfreie Sicherheitsregelung sind **noch offen**. Diese Feststellung ist keine rückwirkende Freigabe für andere Aktionen.

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

- GitHub-Uploader: **Automatischer Direktupload in den geprüften Novena-Fällen im Einsatz; vollständige Qualifikation, Fehler-Wiederanlauf und Regelabgleich noch offen**.
- DDR-/SPL-Root Cause: **offen**; kein neuer Hardwaretest freigegeben.
- I2C3/ES8328: **abgeschlossen**, ohne neue diskriminierende Evidenz nicht erneut öffnen.
- Universelles externes SD-Image: **noch kein vollständig nachgewiesener eigenständiger Bootpfad**.
- GitHub-Änderungen ersetzen **keinen** lokalen foobox-Checkpoint oder Codeberg-Abgleich.
