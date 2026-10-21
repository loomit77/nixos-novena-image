# Novena – SSH-Zugänge und Borg-Wiederherstellungswege

Stand: 2026-10-21. Diese Datei dokumentiert **nachgewiesene Verbindungsparameter und Fundstellen**, keine Passwörter oder privaten Schlüssel. Sie ist keine Freigabe zum Verbindungsaufbau, Backup oder Restore.

## Grundregeln

- `AGENTS.md` und `NOVENA-ARBEITSREGELN.txt` haben Vorrang. Vor Zugriffen aktuellen HEAD, Index, Worktree und Primärprotokolle prüfen.
- Niemals private SSH-Schlüssel, Borg-Passphrasen, Tokens oder Recovery-Secrets in Git einchecken.
- Hostkeys nicht ungeprüft übernehmen, `StrictHostKeyChecking` nicht abschalten.
- Bei Ersatzrechnern zunächst Schlüssel, Passphrasen, Konfiguration und Fingerabdrücke aus unabhängiger, geschützter Sicherung wiederherstellen; Identitäten unabhängig prüfen.
- GitHub-Archiv der Originalprotokolle: privates Repository `loomit77/project-block-logs`, `novena/YYYY/MM/`, jeweils TXT und SHA256. Lokale Originale: `~/novena-backups/block-logs/`. GitHub-Upload-PASS ist kein fachlicher Backup-PASS.

## SSH-Zugänge – dokumentierter Stand

| Zweck | Ziel / Benutzer | Authentisierung und Hostkey | Status / Grenze |
| --- | --- | --- | --- |
| Borg VM103 | `borgadmin@192.168.178.20` | foobox: `/home/loomit/.ssh/id_ed25519`; dedizierte Hostkeydatei `/home/loomit/.config/novena-borg/vm103-known-hosts` | Repository lesend erreicht am 21.10.2026 |
| Novena NixOS | `192.168.11.12`; Benutzer anhand vorhandener SSH-Konfiguration/TEST-LOG verifizieren | ED25519-Hostkey seriell geprüft; foobox-Schlüssel autorisiert | SSH PASS in NOVENA-NIXOS-102 (18.10.2026); deklarative Dauerhaftigkeit der Autorisierung offen |
| GitHub Projekt | `git@github.com:loomit77/nixos-novena-image.git` | foobox SSH, vorhandener Schlüssel | Push 21.10.2026 laut DAY-CLOSE-136 PASS |
| Codeberg Projekt | `ssh://git@codeberg.org/loomit/nixos-novena-image.git` | foobox SSH, vorhandener Schlüssel | Push 21.10.2026 laut DAY-CLOSE-136 PASS |

VM103 ED25519-Hostfingerabdruck laut Originalprotokoll NOVENA-BORG-SSH-PRECHECK-132:
`SHA256:D3vVrOIEkyr8d97jk5Taoxxv9q0sP3NDxZu69wqaTSY`.
Dies ist ein historisch dokumentierter Fingerabdruck, kein Ersatz für eine unabhängige Verifikation bei Schlüsselwechsel.

Für VM103 ist die bestehende Borgmatic-Konfiguration maßgeblich:
`/home/loomit/.config/novena-borg/vm103-borgmatic.yaml`.
Dokumentierte SSH-Optionen: `-i /home/loomit/.ssh/id_ed25519 -o IdentitiesOnly=yes -o BatchMode=yes -o StrictHostKeyChecking=yes -o UserKnownHostsFile=/home/loomit/.config/novena-borg/vm103-known-hosts -o GlobalKnownHostsFile=/dev/null`.
Der direkte Borg-Aufruf in Block 131 verwendete diese Hostkeydatei nicht; ihn nicht als Vorlage wiederverwenden.

## Sicherungsweg A: bestehendes USB-Borg-Repository

- Externer USB-Datenträger; verschlüsseltes Borg-Repository (repokey-blake2), bestehender borgmatic-Weg auf foobox.
- Erfolgreicher dokumentierter Lauf: `NOVENA-BORG-BACKUP-08B` am 18.10.2026: `BORGMATIC_CREATE=PASS`; kein Prune/Delete, Git HEAD/Index unverändert.
- Das exakte Repository-Verzeichnis und der aktuell gültige USB-Mount sind **vorhandener Konfiguration und Originalprotokollen zu entnehmen**, hier nicht erraten. Wechselnde `/dev/sdX`-Namen sind keine Geräteidentität.
- Vorhandenes USB-Repository unverändert erhalten. Echter Restore-Test weiterhin offen. Passphrase interaktiv bzw. aus geschützter unabhängiger Verwahrung, nicht aus Git.
- Original: `novena/2026/10/NOVENA-BORG-BACKUP-08B-20261018-084729-bcth4gtw.txt` im privaten Protokollrepository.

## Sicherungsweg B: VM103-Borg über SSH

- Ziel: Debian-13-VM103, 3-TB-Datenträger; Zielpfad `/srv/borg-storage/novena/foobox`.
- Repository: `ssh://borgadmin@192.168.178.20/srv/borg-storage/novena/foobox`.
- Verschlüsselung: repokey BLAKE2b; Repository-ID laut Block 135: `ed3cd754387db5d8681d7a00a33419c67c66c1134c8d9dc7f86b2632fcaf9e19`.
- Konfiguration: `~/.config/novena-borg/vm103-borgmatic.yaml`; Passphrase-**Dateipfad** `~/.config/novena-borg/vm103-passphrase` (Dateiinhalt geheim); Hostkeydatei siehe oben.
- Launcher laut Block 134: `~/.config/novena-borg/execute-vm103-backup.bash`; Laufprotokoll `~/novena-backups/vm103-backup-state/first-backup-20261021-073016.log`.
- Sicherungsstart Block 97 am 21.10.2026; Preflight und Mount-Guard PASS, Borgmatic `borg create --dry-run --list` brach mit Exitcode **105 (Backup permission issue)** ab. Die genaue betroffene Quelldatei/Berechtigung ist noch nicht nachgewiesen.
- Block 135: Repository erreichbar, **null Archive**; keine erfolgreiche VM103-Sicherung und kein Restore nachgewiesen.
- Nächster Schritt: vorhandenes vollständiges Laufprotokoll und Quellrechte gezielt lesend auswerten. Kein erneuter Lauf, kein chmod/chown und keine Konfigurationsänderung ohne gesonderte Freigabe.
- Belege: `NOVENA-BORG-VM103-FIRST-BACKUP-97`, `NOVENA-BORG-ORIGINAL-RUN-133`, `NOVENA-BORG-FIRST-BACKUP-STATUS-134`, `NOVENA-BORG-ARCHIVE-VERIFY-135` (alle vom 21.10.2026 im privaten Protokollrepository).

## Wiederherstellung / Notfall

1. Vertrauenswürdige Ersatzumgebung und Datenträger identifizieren, niemals unbesehen formatieren oder überschreiben.
2. Originalkonfigurationen, SSH-Schlüssel, Hostkey-Fingerabdrücke und Borg-Passphrasen aus **separat geschützter** Sicherung beschaffen; Git allein enthält diese Geheimnisse bewusst nicht.
3. USB-Weg bevorzugt anhand aktueller Repository-Konfiguration und identifiziertem Mount prüfen; VM103 erst nach nachgewiesener erfolgreicher Archivierung als Restorequelle verwenden.
4. Vor produktivem Restore Archivliste, Repository-Identität, Umfang und Zielpfad lesend prüfen. Restore nur in freigegebenes isoliertes Ziel; Prüfsummen und Dateirechte nachprüfen.
5. Wiederherstellbarkeit beider Wege ist derzeit **nicht durch einen vollständigen Restore-Test qualifiziert**.

## Offene Dokumentationspunkte

- Exakten USB-Repositorypfad, Geräteidentität, Mount-Guard und zugehörige Borgmatic-Datei aus Originalkonfiguration übernehmen.
- Vollständige SSH-Inventur weiterer Hosts (z. B. Proxmox/VM102/L14) separat anhand aktueller Konfiguration und geprüfter Logs ergänzen, nicht aus Erinnerungen erraten.
- Unabhängige Aufbewahrung und Wiederherstellung der SSH-Schlüssel, Passphrasen und Borg-Recovery-Informationen qualifizieren.
- VM103 Exitcode 105 bis zur konkreten Ursache untersuchen; danach separat autorisierter Backup- und Restore-Test.


## Ergänzung 21.10.2026 – belegte SSH-Ziele, systemd und Backup-Status

Diese Ergänzung hat Vorrang vor den älteren, oben stehenden Statusaussagen, soweit sie denselben Sachverhalt betreffen. Keine geheimen Schlüssel oder Passphrasen enthalten.

### Proxmox / VM102 / VM103: überprüfte Verbindungen

- **NixOS-Build-VM102:** `loomit@192.168.178.18`, Hostname `nixos-base`. SSH-Verbindung am 20.10.2026 in `NOVENA-DUAL-SSH-VERIFY-28` nach Hostkeyprüfung PASS; am 20.10. in `NOVENA-VM102-BUILD-RESULT-44` erneut erreicht. ED25519-Hostfingerabdruck `SHA256:T2Gh++PlrP3SWgO+pYqQkcJlRuv+lKHCUGDLxPNdMDY`. Ein vorheriger Aufruf scheiterte wegen veraltetem RSA-Eintrag in `~/.ssh/known_hosts` (`NOVENA-DUAL-SSH-CONNECT-27`); niemals `StrictHostKeyChecking` abschalten. SSH-Schlüsseldatei und aktuell verwendete Hostkey-Datei für diesen Zugang anhand des erfolgreichen Befehls vor erneuter Verwendung prüfen.
- **Borg-VM103:** `borgadmin@192.168.178.20`, Hostname `borg-backup`, Debian 13. Erfolgreicher SSH-Login in `PROXMOX-BORG-CLOUD-SSH-39` (20.10.2026). SSH-Identität und dedizierte known-hosts-Datei stehen im obigen VM103-Abschnitt; Fingerabdruck dort ebenfalls dokumentiert.
- **Proxmox-Host:** SSH-Hostkeyprüfung in `PROXMOX-BORG-CLOUD-SSH-39` PASS. Der exakte, in diesem Protokoll nicht ausgegebene SSH-Benutzer, Host/IP, Hostkey-Fingerprint und verwendete Identity-Datei sind **noch nicht als verifizierter Direktbefehl erfasst**. Diese Werte aus dem erfolgreichen Originalkommando oder der lokalen SSH-Konfiguration ergänzen, nicht aus VM-IP oder Erinnerung ableiten.
- **Schlüssel auf foobox:** `/home/loomit/.ssh/id_ed25519.pub` hat laut `PROXMOX-BORG-SSH-PRECHECK-15` den Fingerabdruck `SHA256:gucuSswYjvyAIqrHwQLZ7wyEWkGhzXp0hr9WPx6ZeQs`. Das ist der **Client-Public-Key**, kein Server-Hostkey.

### Verifizierter früherer VM103-systemd-Auftrag

Der vorherige Lauf verwendete **eine transiente systemd-Service-Unit** mit dem exakten Namen `novena-borg-vm103-first-backup.service`. Primärbeleg: `NOVENA-BORG-VM103-FIRST-BACKUP-97-20261021-072751-nWuuEppd.txt`, darin `Running as unit: novena-borg-vm103-first-backup.service` und anschließend `active`. Der genaue ursprüngliche `systemd-run`-Befehl ist aus diesem Log **nicht** rekonstruierbar. Nicht mit den distributionsseitigen `borgmatic.service` oder `borgmatic-user.service` verwechseln.

Inventur `NOVENA-BORG-VM103-SYSTEMD-INVENTORY-141` und `NOVENA-BORG-VM103-SYSTEMD-HISTORY-142`: Distributions-Borgmatic-Units inaktiv, Timer deaktiviert, keine aktuell aktive transiente Backup-Unit. Eine abgeschlossene transiente Unit kann aus der Liste verschwinden. Für künftige lange Backups die zuvor geprüfte systemd-Startweise rekonstruieren und über den Runner starten; **keinen zweiten Lauf parallel starten**.

### VM103-Berechtigungsreparatur und aktueller Sicherungslauf

- `NOVENA-BORG-VM103-PERMISSIONS-137` und `NOVENA-BORG-VM103-POLICY-138`: Borgmatic sichert `/home/loomit`, keine Ausschlussmuster. Zwei nicht lesbare Objekte festgestellt.
- `NOVENA-BORG-VM103-PERMISSION-FIX-139`: gezielte Änderung der Dateimodi bestätigt: `~/.cache/yay/arm-linux-gnueabihf-gcc-stage1/pkg` **111 → 511**, `~/novena-arch/ArchLinuxARM-armv7-latest/usr/lib/dbus-daemon-launch-helper` **110 → 510**. Eigentümer/Gruppen unverändert. Für eventuelle Rücknahme ursprüngliche Modi dokumentiert; nicht unbesehen zurücksetzen.
- `NOVENA-BORG-VM103-BACKUP-140`: am 21.10.2026 09:21:29 auf foobox über interaktiven Runner gestartet, **nicht über systemd**. Prozessprüfung in `SYSTEMD-INVENTORY-141` zeigt echte `borg create`-Ausführung **ohne `--dry-run`** und SSH zu VM103. `SYSTEMD-HISTORY-142` bestätigt Borg noch aktiv um 09:31. **Abschluss, erfolgreiches Archiv und Restore weiterhin unbestätigt**, bis Abschlussprotokoll und Archivliste geprüft wurden. Den laufenden Auftrag nicht parallel erneut starten.

### Backup-Weg A – USB: ergänzende Identität

`NOVENA-BORG-BACKUP-08B` bestätigt am 18.10.2026 `BORGMATIC_CREATE=PASS`, den Mount-Check und die aus der Konfiguration gelesene Repository-Location, gibt den genauen Pfad jedoch nicht aus. In `SYSTEMD-INVENTORY-141` ist ein eingesteckter SanDisk-Datenträger mit Label `NOVENA_BORG`, Partitions-UUID `851d09dc-4e9b-4d3f-b99b-df4fd1bde0fbc` und Dateisystem-UUID `b9161f3a-5903-4761-936f-42360d13d84e` dokumentiert. Diese Kennungen sind Identifikationshilfen, kein Ersatz für eine Mount-/Repository-Prüfung. **Exakten USB-Repositorypfad und Borgmatic-Konfigurationsdatei noch aus foobox-Konfiguration belegen.**

### Unmittelbar offene Inventur

1. Proxmox: tatsächlichen erfolgreichen SSH-Aufruf einschließlich Benutzer, Ziel und Hostkeydatei **ohne Geheimnisse** verifizieren.
2. VM102: exakte SSH-Optionen aus erfolgreichem Originalkommando/SSH-Konfiguration prüfen.
3. USB-Borg: Konfigurationsdatei, tatsächlichen Repositorypfad, Mount und Launcher/Startweise belegen.
4. VM103: ursprünglichen systemd-run-Aufruf aus der erhaltenen Befehlsdatei/Originalhistorie rekonstruieren; Abschluss und Archiv von Block 140 separat prüfen.
5. Nach Dokumentationsänderung GitHub/Codeberg-Stand vergleichen und Codeberg synchronisieren; nicht annehmen, dass ein GitHub-Commit automatisch bei Codeberg ankommt.


## Inventur-Nachtrag 21.10.2026, 09:56 – ACCESS-INVENTORY-143

Primärbeleg: privates GitHub-Protokoll `novena/2026/10/NOVENA-BORG-ACCESS-INVENTORY-143-20261021-095610-dbJrrT5l.txt`; `BLOCK_RESULT=PASS`, `GIT_CHANGED=NO`.

- `ssh -G 192.168.178.18`: `user loomit`, `hostname 192.168.178.18`, Port 22, Standard-`identityfile`-Kandidaten einschließlich `~/.ssh/id_ed25519`, `StrictHostKeyChecking ask`, Standard-`known_hosts`. Das zeigt **Konfigurationsdefaults**, nicht den bei früheren erfolgreichen SSH-Tests exakt verwendeten Befehl.
- `ssh -G 192.168.178.20`: ebenfalls Defaultbenutzer `loomit`; **für VM103 nicht als Login-Vorlage verwenden**. Der belegte funktionierende Benutzer ist `borgadmin` mit explizitem Schlüssel und dedizierter Hostkeydatei, siehe oben.
- `~/.ssh/config` war nicht lesbar/vorhanden; keine Proxmox-Host-Aliase ermittelt. **Proxmox-IP/Benutzer noch offen.**
- Standard-`known_hosts`: Eintrag für `192.168.178.18` vorhanden, für `192.168.178.20` nicht vorhanden. VM103 verwendet eine **separate** bekannte Hostkeydatei; das Fehlen im Standardfile ist daher kein Verbindungsfehler.
- USB-Mount-Abfrage lieferte keinen Treffer mit Label `NOVENA_BORG` zum Inventurzeitpunkt. **Kein Beweis für fehlenden Datenträger oder Repositoryverlust.**
- Unter `~/.config` wurde nur `/home/loomit/.config/novena-borg/vm103-borgmatic.yaml` als Borgmatic-YAML gefunden. Das schließt eine USB-Konfiguration an anderer Stelle oder in anderem Format nicht aus. **USB-Repositorypfad bleibt offen.**
- VM103-Skripte vorhanden: `/home/loomit/.config/novena-borg/run-vm103-backup.pre96.bash`, `run-vm103-backup.bash` und `execute-vm103-backup.bash`. Der Inhalt wurde **nicht** geprüft; keine Variante ohne Prüfung als endgültigen Startweg deklarieren.
- Um 09:56:10 liefen `borgmatic` (PID 20558) und `borg` (PID 20582) aus Block 140 noch, ca. 32 Minuten nach Start. **Kein bestätigter Archivabschluss**.

**Sicherer Folgeweg:** Bestehende originale `*.command.bash`-Dateien des erfolgreichen USB-Backups und des systemd-Starts gezielt *lesend* auf Startmechanismus und Konfigurationspfad prüfen, mit Geheimnis-Redaktion; Proxmox-Verbindungsparameter nur aus verifizierten Befehlen übernehmen. Während Block 140 läuft, keinen zweiten Borg-Lauf starten. GitHub-Änderung bedeutet keine automatische Codeberg-Synchronisation.
