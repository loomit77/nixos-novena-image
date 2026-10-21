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
