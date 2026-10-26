# Novena-Backup: gemeinsamer Einstieg

## Produktiver Ablauf und Fehlerbehandlung – 23.10.2026

Dieser Abschnitt ist der aktuelle Betriebsstand. Frühere
`NOT_RELEASED`- und `nur Mocks`-Aussagen weiter unten beschreiben
historische Zwischenstände vor der produktiven Qualifikation.

### Verbindlicher Startweg

Auf foobox ausschließlich den bestehenden gemeinsamen Launcher
verwenden, bei Novena-Projektarbeiten über den GitHub-Runner
`~/novena-backups/github-transport/run-and-upload.bash`.

| Modus | Zweck |
| --- | --- |
| `--usb` | Nur USB-Borg einschließlich Tagesabschluss |
| `--hdd` | Nur VM103-Borg |
| `--all` | USB und danach VM103 |
| ohne Argument | Wie `--all` |
| `--check` | Nur lokale Dateien und Syntax; kein Backup |

Ein `--usb`-Lauf darf nach bestätigtem Erfolg nicht
vorsorglich wiederholt werden, wenn nur VM103 fehlschlug.
Ein `--check`-PASS ist kein Archivnachweis.

### USB-Mountpoint

Das ext4-Dateisystem wird anhand der UUID
`b9161f3a-5903-4761-936f-42360d13d84e` identifiziert.
Der feste Mountpoint lautet
`/run/media/loomit/NOVENA_BORG`.
Der vorhandene fstab-Eintrag verwendet
`rw,noauto,user,nosuid,nodev,noexec`.

Am 23.10.2026 scheiterte der USB-Schritt mit
`USB_MOUNT_STATUS=MOUNT_FAILED`, obwohl UUID und fstab
stimmten. Die beiden Verzeichnisse
`/run/media/loomit` und
`/run/media/loomit/NOVENA_BORG` fehlten.
Nach ihrer administrativen Wiederherstellung war der
unveränderte USB-Backup-Lauf erfolgreich.

Da `/run` flüchtig ist, muss die dauerhafte,
sichere Bereitstellung des Mountpoints noch geklärt
werden. Bis dahin vor einem USB-Lauf Existenz,
Eigentümer, Rechte, Symlinkfreiheit, UUID und
fstab-Eintrag prüfen. Keine wechselnden `/dev/sdX`-
Namen verwenden, keine beliebigen Mounts erzwingen
und keine fremden Mounts aushängen.

### VM103 und Borg-Fehler 105

Am 23.10. scheiterte VM103 zunächst mit
`Backup permission issue` (Borg-Exitcode 105).
Unter `/home/loomit/chatcmd-build` wurden
nicht lesbare RootFS-Verzeichnisse und Root-eigene
Build-Dateien nachgewiesen.

Mit Benutzerfreigabe wurde der gesamte Pfad
`/home/loomit/chatcmd-build` in der lokalen
Borgmatic-Konfiguration
`~/.config/novena-borg/vm103-borgmatic.yaml`
unter `exclude_patterns` ausgeschlossen.
Danach waren Konfigurationsvalidierung und echter
VM103-Archivlauf erfolgreich.

Keine pauschalen `chmod`-/`chown`-Änderungen am
Debian-RootFS durchführen. Den Ausschluss bei
einer Konfigurationswiederherstellung erhalten.
Weitere Sicherungsquellen nicht ohne Freigabe
ausschließen.

### Nachweis und Wiederanlauf

- Runner-Upload `PASS` und fachlicher Blockstatus
  sind getrennt auszuwerten.
- Nach jedem Backup die tatsächlichen Felder
  `USB_STATUS`, `VM103_STATUS`, `OVERALL_STATUS`
  sowie Archivnachweise prüfen.
- Bei VM103 das neue Archiv anhand der Archivliste
  und Archiv-ID bestätigen.
- Nach einem Fehler zunächst das vollständige
  Originalprotokoll aus GitHub lesen; nicht blind
  denselben Backup-Lauf wiederholen.
- Git-HEAD, Index und Worktree vor und nach
  dokumentierten Projektaktionen kontrollieren.
- Keine Passphrasen, privaten Schlüssel oder
  vollständigen Geheimniskonfigurationen in
  Runner-Protokolle oder Git übernehmen.
- Erfolgreiche Archivierung ersetzt keinen
  vollständigen Restore-Test.

### Erfolgsbelege vom 23.10.2026

USB: Archivzahl 14 → 15, `BACKUP_CREATE=PASS`,
`NEW_ARCHIVE_PROOF=PASS`, `USB_UNMOUNT_STATUS=PASS`.

VM103: `NOVENA-VM103-BACKUP-48`,
Archiv `foobox-2026-10-23T21:28:31`,
`VM103_STATUS=PASS`, `OVERALL_STATUS=PASS`,
`GIT_HEAD_STABLE=PASS`, `GIT_INDEX_STABLE=PASS`.

Stand 2026-10-22. Implementierung im Projekt, keine Installation außerhalb
von Git. Ausschließlich statisch und mit Mocks qualifiziert.

## Betriebsarten – 2026-10-22, INDEX-1K-79

Dieser Nachtrag erweitert den Stand INDEX-1K-78. Die nachfolgenden älteren
Angaben zum kombinierten Ablauf gelten für `--all` und den Aufruf ohne Argument.

| Fish-Funktion | Launcher-Option | Ausführung |
| --- | --- | --- |
| `backup-usb` | `--usb` | USB vorbereiten, vorhandenen USB-Launcher starten, eigenen Mount aushängen |
| `backup-hdd` | `--hdd` | ausschließlich vorhandener VM103-Launcher; kein USB-Dateicheck oder Mountwerkzeug |
| `backup-all` | `--all` | USB einschließlich Bereinigung, anschließend VM103, auch bei USB-Fehler |
| — | kein Argument | identisch mit `--all`, bisheriger Ablauf |
| — | `--check` | ausschließlich lokale Dateien und Syntax beider Ziele prüfen |

Genau eine Option ist zulässig; Kombinationen und weitere Argumente enden mit
Exit 2. Alle drei Betriebsarten verwenden dieselbe gemeinsame Sperre.
Lokale Voraussetzungen werden nur für ausgewählte Ziele verlangt. Nicht
gewählte Ziele bleiben `NOT_RUN` mit Exitstatus `NA`. Gesamt-PASS erfordert
alle ausgewählten Launcher erfolgreich sowie erfolgreiche nötige Bereinigung.
Ziel-Exitcodes bleiben unverändert in `*_EXIT_STATUS`; der Prozess verwendet
weiterhin 0/1 für Gesamterfolg/-fehler, 2 für Argumentfehler, 40 für belegte
Sperre und 130/143 für INT/TERM. Keine Wiederholungen.

Die Fish-Dateien liegen unter `config/fish/functions/`, sind **nicht installiert**
und können nach Betriebsfreigabe einzeln mit Fish `source` geladen werden.
Sie erwarten das Projekt unter `$HOME/nixos-novena-image`, rufen Bash mit
festem Modus auf und reichen dessen Exitcode unverändert weiter. Zusätzliche
Argumente werden weitergereicht und vom Launcher als Aufruffehler abgewiesen.
Die Funktionen starten bei Aufruf echte Sicherungen; ihre Anlage ist keine
Betriebsfreigabe. Der USB-Schritt enthält weiterhin den bisherigen Tagesabschluss.

Die fstab-Vorbereitung im Abschnitt „Berechtigungen und Freigabegrenze“ bleibt
maßgeblich: exakt ein UUID-/Zieleintrag mit `rw,noauto,user,nosuid,nodev,noexec`,
keine `users`-Option, keine sudoers-Regel. Ein Administrator muss Eintrag und
Mountpfad erst separat prüfen und einrichten; `/etc/fstab` wurde nicht geändert.
`mount` erhält nur den festen Zielpfad, keine Quelle oder Optionen, damit die
unprivilegierte fstab-user-Prüfung greift. Die tatsächlich wirksamen lokalen
Berechtigungen sind hier nicht qualifiziert. Vorhandene korrekte Mounts bleiben
unangetastet; eigene Mounts werden nur bei übereinstimmender Mount-ID ausgehängt.
Nach erfolgreichem Unmount wird der Zielpfad erneut geprüft; `STILL_MOUNTED`
führt zum Gesamtfehler ohne weiteren Unmountversuch. Die dokumentierten
Grenzen bei konkurrierenden Mountänderungen bleiben bestehen.

Prüfung dieses Nachtrags: `bash -n`, `fish -n`, isolierte Python-Mocks und
`git diff --check`. Die Mocks decken alle drei Modi, Zielkombinationen,
Originalexitcodes, fehlende nicht gewählte Voraussetzungen, gemeinsame Sperre,
fstab-Ablehnung, Mount-Identität und -Besitz, Mount-/Unmountfehler, weiterhin
belegten Zielpfad, TERM-Bereinigung sowie Fish-Aufruf und Exitcode ab.
Keine echten Backups, Geräteaktionen oder externen Schreibzugriffe; kein
externes Runner-Rohprotokoll, Werkzeugausgaben nur im Chat.

Lokale Voraussetzungen ohne Backup, Borg, SSH, Mountzugriff oder Schreibzugriff
prüfen (auch keine Sperrdatei wird angelegt):

```bash
bash ~/nixos-novena-image/scripts/novena-backup.bash --check
```

`CHECK_PASS` bedeutet ausschließlich: lokale Dateien lesbar, Launcher-Syntax
prüfbar und direkt aufgerufene VM103-Helfer ausführbar. Es belegt weder freie
Kapazität, gültige Zugangsdaten, Repositoryzustand, Mount noch Erreichbarkeit.
Kein Aufruf des vorhandenen `--preflight`: dieser führt SSH und Imagesync aus.

Nach gesonderter Betriebsfreigabe starten, als separaten Bash-Prozess:

```bash
bash ~/nixos-novena-image/scripts/novena-backup.bash
printf 'BACKUP_PROCESS_EXIT_STATUS=%s\n' "$?"
```

Dieser Start führt echte Backups aus. Für autorisierte Rohprotokollierung den
bestehenden Runner verwenden; kein neuer systemd-Startparameter wird erfunden.
stdout enthält die Statusfelder, stderr die vorhandenen Launcher-Ausgaben;
für ein vollständiges Betriebsprotokoll beide erfassen. Keine Geheimnisse
oder komplette Konfigurationen ausgeben oder ungeprüft hochladen.

Die Reihenfolge innerhalb der gemeinsamen Sperre ist fest:
USB anhand UUID prüfen und gegebenenfalls einhängen, USB-Launcher ausführen,
eigenen Mount aushängen, danach VM103-Launcher ausführen:

1. `bash ~/novena-backups/workflow-dev/daily-close.bash` ohne Argumente.
   Vorhandene Git-, Mount-UUID-, Repository-ID-, Secret- und Archivvergleichs-
   Guards bleiben erhalten. Dieser Schritt enthält bereits den Tagesabschluss.
2. `bash ~/.config/novena-borg/execute-vm103-backup.bash` ohne Argumente.
   Bestehender Preflight, Mount-Guard, Imagesync, Create und Archivliste bleiben
   erhalten. Laufprotokolle: `~/novena-backups/vm103-backup-state/first-backup-*.log`.

Nach einem USB-Fehler wird VM103 trotzdem versucht; es gibt keine Wiederholung.
Fehlen lokale Voraussetzungen, startet keines der Ziele. Bei einem Abbruch
kann `RUNNING` stehenbleiben: Ergebnis unbekannt, vor einem Neustart vorhandene
Prozesse und Protokolle prüfen. Ein Launcherfehler kann auch nach erfolgreicher
Archivierung entstehen; daher vor jeder Wiederholung zuerst Evidenz auswerten.

Die gemeinsame nicht wartende flock-Sperre liegt unter
`~/novena-backups/borg-private/novena-backup.lock`; das bestehende private
Verzeichnis muss vorhanden sein. Sie bleibt über beide Launcher gehalten,
auch von ihren Kindprozessen geerbt. Sperrdatei niemals zum Entsperren löschen.
Die vorhandene `daily-close.lock` bleibt Aufgabe des USB-Launchers.
Direkte Altaufrufe nutzen die neue gemeinsame Sperre nicht; insbesondere hält
VM103 seine bisherige Sperre nur während des Preflights. Alle künftigen
kombinierten Starts deshalb über diesen Einstieg; keine parallelen Altstarts.

Statusfelder aus dem Abschlussprotokoll anzeigen, ohne Borg oder SSH:

```bash
rg '^(USB_STATUS|USB_EXIT_STATUS|VM103_STATUS|VM103_EXIT_STATUS|OVERALL_STATUS)=' /pfad/zum/freigegebenen-abschlussprotokoll.txt
```

| Feld / Wert | Bedeutung |
| --- | --- |
| `USB_STATUS`, `VM103_STATUS`: `PASS` / `FAIL` | Launcher Exit 0 / ungleich 0 |
| `*_EXIT_STATUS` | Originalexitcode oder `NA`, wenn nicht ausgeführt |
| `NOT_RUN` / `RUNNING` | nicht gestartet / ohne belegten Abschluss |
| `LOCAL_READY` / `LOCAL_MISSING_OR_INVALID` | ausschließlich lokaler Check |
| `OVERALL_STATUS=PASS` / `FAIL` | beide Launcher erfolgreich / mindestens einer fehlgeschlagen |
| `CHECK_PASS` / `CHECK_INCOMPLETE` | lokaler Check vollständig / unvollständig |
| `LOCKED` / `LOCK_UNAVAILABLE` | Sperre belegt / nicht verwendbar |
| `PRECONDITION_FAILED` / `INVALID_ARGUMENT` / `INTERRUPTED` | Vorbedingung fehlt / Aufruf ungültig / Signalabbruch |

Exitcodes: 0 Erfolg oder lokaler Check erfolgreich, 1 Fehler, 2 Aufruffehler,
40 Sperre belegt, 130/143 INT/TERM. Unbehandelbare Prozessabbrüche wie SIGKILL
können keine Abschlussausgabe garantieren. Es wird keine neue Statusdatei geführt.
PASS ist ein Prozessnachweis, kein Restore-Nachweis.

## Tagesabschluss vorbereiten

Den bisherigen Bedienaufruf nach Betriebsfreigabe durch den gemeinsamen
Einstieg ersetzen. Nicht zusätzlich davor oder danach `daily-close.bash`
starten: dies würde USB doppelt sichern. Das externe Skript bleibt unverändert;
den neuen Einstieg dort nicht rekursiv einbauen. Keine Timer oder externen
Konfigurationen wurden verändert. Der bestehende Tagesabschluss bleibt im
USB-Schritt erhalten, das Gesamtergebnis muss zusätzlich VM103 einschließen.

## Evidenz und Prüfung

Die Startwege sind durch die gelesenen lokalen Skripte und Erfolgsprotokolle
248/255 belegt; Identitäten und Grenzen siehe
[aktueller Betriebsstand](BACKUP-AND-SSH-ACCESS.md#aktueller-betriebsstand--nachtrag-2026-10-22).
08B bleibt historische Referenz mit damaligem HEAD-/Clean-Guard und wird
nicht als heutiger Launcher verwendet.

Statische und isolierte gemockte Prüfung im Projekt:

```bash
bash -n scripts/novena-backup.bash
python3 -B tests/novena-backup-mock.py
git --no-optional-locks diff --check
```

Die Mocks verwenden ein temporäres HOME innerhalb des Projekts, ersetzen
beide Launcher und blockieren Borg/Borgmatic/SSH mit Sentinel-Mocks.
Geprüft: Check ohne Ausführung/Dateianlage, Argumentfehler, Reihenfolge,
alle vier Zielergebniskombinationen, gemeinsame Sperre während beider Kinder,
Konkurrenzabweisung und fehlender Launcher. Keine echten Backups ausgeführt.

Arbeitsblock: INDEX-1K-77 (lokale Implementierung, keine externe Runner-Ausführung).
Der letzte lokal auffindbare numerische INDEX-Block ist 76. Vorbereitung und
Werkzeugausgaben sind im Chat erhalten; es gibt kein vollständiges externes
Runner-Rohprotokoll dieses Auftrags, weil externe Schreibzugriffe untersagt sind.


## 2026-10-22 – INDEX-1K-78: USB-Mount-Management

Grundlage des neuen Auftrags: Nutzerbestätigung zu NOVENA-BORG-USB-04/05
vom 22.10.: Datenträger erkannt, nicht eingehängt, USB Exit 26, VM103
 erfolgreich. Die zugehörigen vollständigen Rohlogs sind im Projekt nicht
nachgewiesen; dies ist eine Nutzerangabe, keine hier rekonstruierte Logprüfung.

Identifikation ausschließlich über Dateisystem-UUID
`b9161f3a-5903-4761-936f-42360d13d84e`, Typ ext4. Label `NOVENA_BORG`
ist beschreibend; `/dev/sda1` ist keine dauerhafte Identität.
Ziel: `/run/media/loomit/NOVENA_BORG`, Repository darunter `repository`.

### Berechtigungen und Freigabegrenze

**MOUNT_PERMISSION_STATUS=NOT_RELEASED**: Die sichere Einrichtung auf foobox
ist nicht nachgewiesen. Es wurden keine Systemkonfigurationen verändert.
Der produktive automatische Mount ist noch nicht freigegeben. Die folgenden
Angaben beschreiben eine administrativ vorzubereitende Lösung, keinen im
Auftrag ausgeführten Installationsschritt.

Der Launcher läuft als normaler Benutzer und weist Root-Ausführung zurück.
Er verwendet weder sudo noch Passwortabfragen, sondern ausschließlich einen
festen, administrativ verwalteten fstab-Benutzermount. Er akzeptiert genau
folgenden Eintrag (Optionen auch in dieser Reihenfolge):

```text
UUID=b9161f3a-5903-4761-936f-42360d13d84e /run/media/loomit/NOVENA_BORG ext4 rw,noauto,user,nosuid,nodev,noexec 0 0
```

Vor Betriebsfreigabe muss ein Administrator nachweisen: vertrauenswürdige,
root-verwaltete fstab und Mountprogramme, funktionierende unprivilegierte
`mount -- /run/media/loomit/NOVENA_BORG`- und entsprechende `umount`-Semantik,
Berechtigung nur im beabsichtigten lokalen Benutzerkreis, sowie sicherer
Mountpfad ohne Symlink und ohne durch andere Benutzer austauschbare
Pfadkomponenten. `user` gestattet lokalen Benutzern dieses einzelne Dateisystem
zu mounten; nur der mountende Benutzer darf es wieder aushängen. Das ist keine
auf loomit beschränkte Policy. Ist diese lokale Berechtigungsbreite nicht
akzeptabel, bleibt diese Lösung gesperrt und benötigt eine separat geprüfte
engere Policy. Keine beliebige sudo-mount-Freigabe einrichten.

Die ext4-Rechte des vorhandenen Repositorys müssen loomit bereits Zugriff
gewähren; der Launcher verändert keine Eigentümer oder Dateirechte.
UUID-Duplikate sind administrativ auszuschließen. Andere Automounter und
manuelle Mount-/Unmount-Aktionen am Ziel müssen während des Laufs
organisatorisch ausgeschlossen sein: Die gemeinsame flock-Sperre koordiniert
nur kooperierende Launcher, nicht den Kernel-Mount-Namensraum. Die Prüfungen
sind keine atomare Sperre gegen konkurrierende privilegierte Mountänderungen.
Diese Voraussetzungen sind durch Mocktests ausdrücklich nicht bewiesen.

### Ablauf und Fehler

Unter der gemeinsamen Sperre wird ein vorhandener exakter Mount anhand
Mount-ID, UUID, ext4 und Ziel geprüft. Ein korrekter vorhandener Mount wird
verwendet und bleibt eingehängt. Ein fremder Mount sperrt den USB-Schritt.
Ohne Mount wird die UUID über `/dev/disk/by-uuid/` und lsblk geprüft, danach
der feste fstab-Eintrag. Nach erneuter Prüfung auf einen belegten Zielpfad
folgt genau ein unprivilegierter Mountversuch. Anschließend werden Mount-
Identität und lesender/schreibender Repository-Verzeichniszugriff geprüft.
Die bestehenden Repository-ID- und Borg-Guards bleiben beim alten Launcher.

Nur nach erfolgreichem Mountaufruf und bestätigter Identität merkt sich der
Launcher dessen Mount-ID. Diesen eigenen Mount hängt er nach USB, auch bei
USB-Fehler, genau einmal normal aus. EXIT/INT/TERM versuchen noch ausstehende
Bereinigung; SIGKILL/Stromausfall können sie verhindern. Keine erzwungenen
oder Lazy-Unmounts, keine Wiederholung, kein Power-Off. Ein Unmountfehler
führt zum Gesamtfehler; VM103 wird dennoch gestartet. Bei Signalabbruch
wird VM103 nicht zusätzlich gestartet.

Wechselt die Mount-ID, wird nichts ausgehängt und `IDENTITY_CHANGED`
gemeldet. Kann ein erfolgreicher Mountaufruf anschließend nicht eindeutig
zugeordnet werden, stehen `POSTCHECK_FAILED` und `OWNERSHIP_UNKNOWN` im
Ergebnis: möglicherweise eingehängt, manuelle Inventur erforderlich, kein
blindes Unmount. Auch nach Unmountfehler zuerst lesend inventarisieren.

Zusätzliche Statusfelder: `USB_MOUNT_STATUS` (unter anderem
`ALREADY_MOUNTED`, `MOUNTED`, `FOREIGN_MOUNT`, `DEVICE_MISMATCH`,
`FSTAB_NOT_APPROVED`, `MOUNT_FAILED`, `REPOSITORY_INACCESSIBLE`) und
`USB_UNMOUNT_STATUS` (`NOT_NEEDED`, `PASS`, `FAIL`, `IDENTITY_CHANGED`,
`OWNERSHIP_UNKNOWN`). Bei USB-Vorbereitungsfehler steht `USB_STATUS=MOUNT_FAILED`
und `USB_EXIT_STATUS=NA`; VM103 behält seinen eigenen Exitcode. Ein erfolgreiches
USB-Backup mit fehlgeschlagenem Unmount bleibt `USB_STATUS=PASS`, aber das
Gesamtergebnis ist FAIL. `--check` prüft weiterhin ausschließlich lokale
Dateien und Syntax, ohne Geräteabfrage, Mount, Unmount, Borg, SSH oder Sperrdatei;
es bestätigt weder Mountrechte noch diese administrativen Voraussetzungen.

Qualifikation INDEX-1K-78: statische Syntaxprüfung und isolierte Mocks PASS.
Zusätzlich geprüft: korrekt vorhandener Mount, fehlender Mount, falsche UUID,
fremder Mount/Dateisystemtyp, Mountfehler, USB-/VM103-Fehler, Unmountfehler,
Mount-ID-Wechsel und fehlender Repositoryzugriff. Mount/Unmount/findmnt/lsblk
sind Mocks; sudo, SSH, Borg und Borgmatic werden durch Sentinels blockiert.
Nur der feste Mountpfad wird für die Tests in das temporäre Projekt-HOME
verlegt. Keine realen Geräteaktionen oder Backups. Keine externe
Runner-Protokollierung; Werkzeugausgaben im Chat, kein vollständiges
Runner-Rohprotokoll. Vorhandene Vorarbeiten bleiben erhalten.

## Produktive USB-Qualifikation – 22.10.2026

Die UUID-basierte Mount-Konfiguration auf foobox wurde
produktiveingerichtet und erfolgreich geprüft.

Belege im privaten GitHub-Protokollrepository:

- NOVENA-BACKUP-UNIFY-07:
  Drei Fish-Funktionen installiert und geprüft.
- NOVENA-BACKUP-UNIFY-10:
  fstab-Eintrag installiert; Benutzer-Mount,
  Repository-Zugriff und Benutzer-Unmount PASS.
- NOVENA-BACKUP-UNIFY-11:
  Echter USB-Borg-Lauf erfolgreich.

USB-11:
- USB_MOUNT_STATUS=MOUNTED
- BACKUP_CREATE=PASS
- ARCHIVES_BEFORE=12
- ARCHIVES_AFTER=13
- NEW_ARCHIVES=1
- MISSING_OLD_ARCHIVES=0
- USB_UNMOUNT_STATUS=PASS
- USB_STILL_MOUNTED=NO
- VM103_STATUS=NOT_RUN
- OVERALL_STATUS=PASS

Der USB-Datenträger bleibt physisch angeschlossen,
ist außerhalb des Sicherungslaufs aber ausgehängt.

Die drei Fish-Funktionen sind unter
~/.config/fish/functions/ installiert.

backup-usb ist produktiv qualifiziert.
backup-hdd und backup-all sind implementiert und
isoliert getestet. Der vorhandene VM103-Backup-Weg
war am 22.10.2026 separat erfolgreich; der neue
kombinierte Ablauf ist noch nicht produktiv geprüft.

Der fstab-Eintrag verwendet die Dateisystem-UUID
b9161f3a-5903-4761-936f-42360d13d84e.

Die ursprüngliche fstab wurde gesichert unter:
/etc/fstab.novena-20261022-073137

findmnt --verify meldete null Fehler und drei
Warnungen: zwei zum vorhandenen /mnt/novena-boot-
Eintrag (/dev/sdb1 nicht vorhanden) und eine zur
ausstehenden systemd-daemon-reload-Aktualisierung.

Keine Repository-Initialisierung, kein Prune/Delete.
Ein vollständiger Restore-Test bleibt offen.


<!-- NOVENA-BACKUP-E2E-20261026 -->
## Produktiver Gesamtstand – 26.10.2026 (Block 180)

**Maßgeblicher neuerer Nachweis; historische Einschränkungen oben bleiben als Historie erhalten.**

- Verbindlicher kombinierter Einstieg auf foobox:
  `bash ~/nixos-novena-image/scripts/novena-backup.bash --all`.
  Die Fish-Funktion `backup-all` ist ein Alias dafür.
- Nicht zusätzlich `daily-close.bash` ausführen: Der gemeinsame Launcher ruft es bereits für den USB-Schritt auf. Das Skript prüft Git, Dokumente, Repositoryidentität und Archivbestand, sichert USB und gibt einen Handover-Text aus; es schreibt keine neuen Projektdokumente und pusht keine Commits.
- Der USB-Borgmatic-Pfad ist `~/.config/borgmatic/novena.yaml`; VM103 verwendet `~/.config/novena-borg/vm103-borgmatic.yaml` mit dem bestehenden SSH-/Hostkey-Setup. Keine YAML-Suche und keine manuelle Pfadverschiebung.
- USB: UUID `b9161f3a-5903-4761-936f-42360d13d84e`, fstab-Benutzermount `/run/media/loomit/NOVENA_BORG`; `/etc/tmpfiles.d/novena-borg-mount.conf` wurde in Block 179 installiert. Die Regel erzeugt den Pfad unter `/run` erneut, mountet aber kein Dateisystem. Ein tatsächlicher weiterer Neustart wurde noch nicht qualifiziert.
- Block 180 (26.10., 06:03:25–06:09:22): gemeinsamer Lauf **PASS**. USB-Mount `MOUNTED`, tägliche Sicherung `PASS`, Archivzahl **16 → 17**, ein neues Archiv, kein altes verloren, `USB_UNMOUNT_STATUS=PASS`. VM103 `PASS`, konkretes neues Archiv `foobox-2026-10-26T06:07:20`, Archiv-ID `8bce99b6517c4fd176dc393b62652bdb2fd26d38335ed53337d5b22c2c8828d2`; Imagesync 26 Quellen, zwei neue Kopien, keine Fehler.
- Gesamtstatus: `USB_STATUS=PASS`, `VM103_STATUS=PASS`, `OVERALL_STATUS=PASS`; kein automatischer Retry; HEAD/Index/Worktree unverändert.
- Original: privates `loomit77/project-block-logs` unter `novena/2026/10/NOVENA-BACKUP-E2E-180-20261026-060325-TkHPQ2Ms.txt`; Sidecar-SHA256 `f8155ef6336b9c00201ae100af3fb38e3978daa7fba4007a17eece8cf14c1ac5`.
- Qualifikationsgrenze: erfolgreiche Archiverstellung und Archivlistennachweis, kein vollständiger Datenintegritäts- oder Restore-Test.

**Betriebsregel:** Ein Aufruf für beide Ziele; bei Teilausfall erst Originalprotokoll lesen und nur das fehlgeschlagene Ziel nach eindeutiger Klärung gezielt nachholen (`--usb` beziehungsweise `--hdd`). Vorher keinen weiteren allgemeinen Diagnose- oder doppelten Backup-Lauf starten. Die integrierten Archivkontrollen sind maßgeblich.
