# TODO – Novena: universelles NixOS-SD-Image

Stand: 2026-10-10
Grundlage: Projektgespräche, `AGENTS.md`, `NOVENA-ARBEITSREGELN.txt`, `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md` sowie die GitHub-Repositories `loomit77/Test` und `loomit77/project-block-logs`.
Legende: `[x]` dokumentiert erledigt · `[ ]` offen · **Prüfen** = noch nicht vollständig belegt.
Prioritäten: **P1** vorrangig · **P2** anschließend · **P3** langfristig.

> **Arbeitsregeln:** Vor technischen Änderungen Git-/Index-Zustand, Rohlogs und dokumentierte Checkpoints prüfen. Auf der foobox jeden freigegebenen technischen Block mit dem vorhandenen Runner `~/novena-backups/block-logs/run-block.bash` als vollständige TXT plus SHA256-Sidecar protokollieren. Keine ungefragten Builds, Hardwaretests, Medien-Schreibzugriffe, Git-Resets oder Änderungen am Runner. Intent-to-add-Einträge und vorhandene Nutzeränderungen erhalten. Read-only-Codex-Aufträge schreiben keine Dateien. `printf` statt `echo`. GitHub-Änderungen und lokale foobox-Zustände getrennt bewerten.

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
- [x] Entwicklungsbranch `feature/block-log-upload` in `loomit77/Test` vorhanden.
- [x] Review unter `docs/UPLOAD-REVIEW.md` und Testplan mit 22 Fällen unter `docs/UPLOAD-V2-TESTPLAN.md` dokumentiert.
- [ ] Aktuellen Entwicklungsstand des Uploadskripts gegen GitHub und lokale foobox-Dateien abgleichen.
- [ ] Alle 22 Testfälle kontrolliert ausführen und mit TXT/SHA256 protokollieren; insbesondere Race-Conditions, Push-Fehler, Wiederaufnahme und Konflikte.
- [ ] Vertrauliche Inhalte vor jedem Upload prüfen; private Repositories nicht als Geheimnisspeicher behandeln.
- [ ] Vollständigkeit, Hashgleichheit, Idempotenz und unabhängige Remote-Prüfung nachweisen.
- [ ] Zugriff auf archivierte Logs über die verbundene GitHub-Schnittstelle mit konkreten Dateien praktisch prüfen.
- [ ] Erst nach Freigabe den Uploader produktiv einsetzen; bestehenden Runner nicht ungefragt ändern.
- [ ] Grenzen dokumentieren: GitHub-Zugriff kann manuelle ChatGPT-Dateiuploads reduzieren, hebt Produktlimits aber nicht auf.
- [ ] Projektübergreifende Zuordnung für Novena, L14, Homelab und weitere Projekte eindeutig regeln.

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
- [ ] Diese `TODO.md` mit dem lokalen Stand abgleichen; den neuen GitHub-Commit nicht blind in einen geänderten Worktree übernehmen.
- [ ] Codeberg und GitHub nach ausdrücklicher Freigabe auf denselben gewünschten Commitstand bringen.
- [ ] `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`, `docs/DECISIONS.md` und Quellenverzeichnis nach belegten Checkpoints aktualisieren.
- [ ] TODO nach jedem bestätigten Meilenstein fortschreiben; offene Punkte nur mit Evidenz abhaken.

## P3 – Veröffentlichung und Weiterentwicklung

- [x] Artikelserie über Novena begonnen; Teil 1 veröffentlicht.
- [ ] Teil 2 und weitere Teile anhand belegter Projektstände fertigstellen.
- [ ] Dokumentation für Nachbau, Installation, Fehlersuche und Wiederherstellung erstellen.
- [ ] Später die Möglichkeit eines modernen, Novena-inspirierten Open-Hardware-Systems gesondert untersuchen.

## Aktueller Freigabestand

- GitHub-Uploader: **Entwicklung, nicht produktiv freigegeben**.
- DDR-/SPL-Root Cause: **offen**; kein neuer Hardwaretest freigegeben.
- I2C3/ES8328: **abgeschlossen**, ohne neue diskriminierende Evidenz nicht erneut öffnen.
- Universelles externes SD-Image: **noch kein vollständig nachgewiesener eigenständiger Bootpfad**.
- GitHub-Änderungen ersetzen **keinen** lokalen foobox-Checkpoint oder Codeberg-Abgleich.
