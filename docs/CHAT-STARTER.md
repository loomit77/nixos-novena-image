# Novena – Starttext für einen neuen Chat

> Aktueller Workflowstand: siehe Nachtrag vom 2026-10-17 am Dateiende (Zwei-Skripte-Workflow, E2E PASS; Konkurrenztest offen). Fruehere Statusangaben sind datierte historische Staende.

Wir setzen das Projekt „Novena – universelles NixOS SD-Image“
auf Basis der dokumentierten Projektübergabe fort.

Arbeits- und Repositoryhost ist die foobox.
Zielhardware ist Kosagi Novena.

Lies beziehungsweise prüfe vor neuen technischen Vorschlägen
in dieser Reihenfolge vollständig:

1. `AGENTS.md`
2. `NOVENA-ARBEITSREGELN.txt`
3. `README.md`
4. `docs/HANDOVER-LATEST.md`
5. `TODO.md`
6. Relevante datierte Nachträge in
   `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`,
   `docs/DECISIONS.md` und
   `docs/project-index/README.md`.

Prüfe HEAD, Branch, Index und Worktree getrennt.
Berücksichtige die abgeschlossene Git-Indexbereinigung vom 2026-10-12
und den datierten Nachtrag in `docs/HANDOVER-LATEST.md`:

- Aktueller HEAD: `4e024f8e9da1af2fe91990b0beee301a50ac4ea2`.
- Aktueller Index-SHA256:
  `665d9b5b0f30fb573027c1697cd2d7731108982f62ab121755727e5fcc8ae800`.
- Cleanup-04/06/10 normalisierten 10/47/7, insgesamt 64 Indexeinträge.
- Cleanup-11 war die abschließende read-only Prüfung.
- HEAD: 407 Einträge; Index: 416 Einträge.
- Neun Intent-to-add-Patches 0004–0012; keine gestagten Inhaltsänderungen.
- Sieben modifizierte Dateien, eine fehlende Forschungsdatei
  und 23 unversionierte Dateien.

Die frühere Beschreibung zahlreicher Löschvormerkungen ist historisch.
Bewahre sämtliche Nutzeränderungen, Intent-to-add-Einträge und
historischen Projektbefunde einschließlich des historischen Index-SHA256
`90f75b5f8785128730b3f75741b1063299ac90452b8c2e8cf09234e7477bc0c2`.
Diese Angaben sind dokumentierte Ergebnisse, keine neue Git-Prüfung.
Eine neue Remote-Synchronisierung ist damit nicht belegt.

Nutze `git --no-optional-locks` für lesende Git-Prüfungen.
Keine pauschalen Git-Reparaturen.

Beachte die historischen Hashbindungen des Projektindex,
die begrenzten Abnahmeumfänge von INDEX65 und catalog-v2,
die offene R6-Wiederherstellung und die belegte
P_EXT-Bootgrenze.

DDR/SPL bleibt offen; Patch0012 ist kein bestätigter
isolierter Fix. I2C3/ES8328 ist abgeschlossen.

Neuere und besser belegte Evidenz hat Vorrang.
Bei relevanten ungeklärten Widersprüchen anhalten.

Jeder technische Auftrag benötigt klar definierte
Freigaben und einen Nachweiszweck.

Read-only umfasst auch temporäre Dateien und Runnerlogs.
Die äußere Protokollierung erweitert keine Codex-Rechte.

Protokollierung und GitHub-Upload nur innerhalb
ausdrücklich erteilter Schreib- und Netzwerkfreigaben.
Die konkreten Werkzeugpfade sind:

- Runner: `~/novena-backups/block-logs/run-block.bash`.
- GitHub-Transportwrapper:
  `~/novena-backups/github-transport/run-and-upload.bash`.

Technische Abnahme, Hashintegrität und Transporterfolg
sind getrennt zu bewerten.

Build, Medienzugriff, Schreiben, Readback und
Hardwareboots erfordern getrennte Freigaben.
Hardwareversuche sind zahlenmäßig zu begrenzen
und seriell vollständig aufzuzeichnen.

L14, historische Quellprojekte und die separate
Dateiinventur nicht mit dem SD-Image-Auftrag vermischen.

Mein aktueller Arbeitsauftrag lautet:

[Hier den konkreten nächsten Auftrag einfügen.]

Zunächst nur den bestehenden Projektstand und
die Quellenlage auswerten.

Keine Builds, Patches, Hardwaretests, Git-Änderungen
oder weiteren Schreibvorgänge ohne meine Freigabe.

## Aktueller verifizierter Übergabecheckpoint

Prüfung vom 2026-10-12:
NOVENA-HANDOVER-VERIFY-03

- Branch: main
- HEAD: dab134ec9969221d15983f73d1e0a894b930e292
- Index-SHA256:
  6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937
- Git-HEAD und Index entsprechen dem geprüften Übergabestand.
- Fünf zentrale Projektdateien: SHA256 PASS.
- Drei historische Übergabeprotokolle: Integrität PASS.
- Git-Zustand während der Prüfung unverändert.
- Die ITA-Teilprüfung im VERIFY-03-Skript war fehlerhaft
  beziehungsweise nicht aussagekräftig.
  Keine Indexbeschädigung daraus ableiten.
- Bereits qualifizierte frühere ITA-Prüfungen berücksichtigen.
  Keine Wiederholungsprüfung ohne neuen Erkenntnisgrund.

Die älteren Checkpoints in diesem Dokument bleiben
historische Angaben und dürfen nicht überschrieben werden.

## Automatischer Protokollabruf

Bei UPLOAD_STATUS=PASS zuerst die verbundene
GitHub-Anbindung verwenden.

Privates Protokollrepository:
loomit77/project-block-logs

Neueste Commits nach der Blockkennung durchsuchen.
TXT und SHA256 unter novena/YYYY/MM/ abrufen.
Vollständige Ausgabe und Integrität prüfen.

Bewährter Aufruf auf der foobox:

bash "$WRAPPER" "$BLOCK_ID" "$COMMAND_FILE"

Wrapper:
~/novena-backups/github-transport/run-and-upload.bash

Nicht erneut um einen manuellen Upload bitten,
solange die Dateien direkt zugänglich sind.

Keine unnötigen Wiederholungsprüfungen oder
neuen Metaprojekte beginnen.

Ein erfolgreicher Upload ist keine fachliche Abnahme.

## Nachtrag: erste praktische Chat-Übergabe

Die erste praktische Chat-Übergabe wurde am 2026-10-12
durchgeführt und anschließend dokumentiert.

Letzter bestätigter synchronisierter Dokumentationscommit:
`3ac6b4fffd8322032d7c9e5098fd51133484bc84`.

Dieser neuere Checkpoint ersetzt nicht die historischen
Prüfwerte im Haupttext.

Erkenntnisse:

- Vorhandene Übergaben und Originalprotokolle zuerst auswerten.
- Bei erfolgreichem GitHub-Upload TXT und SHA256 selbstständig
  über die berechtigte GitHub-Anbindung abrufen.
- Keine erneuten Uploads vom Nutzer verlangen, wenn die
  Originaldateien bereits erreichbar sind.
- Fehlgeschlagene Prüfskripte nicht automatisch als
  Beschädigung bereits qualifizierter Zustände interpretieren.
- Keine abgeschlossenen Prüfungen ohne neuen Erkenntnisgrund
  wiederholen.
- Keine SSH-Schlüsselpfade aus anderen Projekten übernehmen.
- Auf der foobox wurde `~/.ssh/id_ed25519` erfolgreich
  für die Synchronisierung verwendet.

Aktuelle organisatorische Priorität:

1. Projekt-Starter einschließlich Übergabeverfahren abschließen.
2. Einheitlichen Tagesabschluss entwickeln.
3. Vorhandene Werkzeuge und den Tools-Audit auswerten.

DDR/SPL, neue Builds und Hardwaretests bleiben bis zu einer
gesonderten Priorisierung beziehungsweise Freigabe zurückgestellt.

## Nachtrag 2026-10-12 – Wiederaufnahme nach NOVENA-DAY-CLOSE-10/11

Für die aktuelle Wiederaufnahme gelten folgende neueren Prüfwerte;
die älteren Angaben oben bleiben historische Checkpoints:

- Projekt auf foobox: `~/nixos-novena-image`, Branch `main`.
- Projekt-HEAD: `5e2f7afbad802b49b8305dd2a5437c8550bae2be`.
- Index-SHA256:
  `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
- Neun ITA-Patches 0004–0012 und sämtliche Nutzeränderungen erhalten.
  Die Index-Löschvormerkung bei zugleich vorhandener `docs/DAILY-CLOSE.md`
  ist kein Auftrag zur Indexreparatur.

Vor jedem neuen Schritt zuerst die vorhandenen vollständigen Originalprotokolle
und Projektdokumentation einschließlich neuester datierter Nachträge prüfen.
Für diesen Stand sind DAY-CLOSE-08/09/10 unter `~/novena-backups/block-logs/`
maßgeblich; genaue Dateinamen und SHA256 stehen im neuesten Nachtrag zu
`docs/DAILY-CLOSE.md`. Weitere Evidenzquelle ist das private GitHub-Archiv
`loomit77/project-block-logs` unter `novena/YYYY/MM/`, jeweils TXT plus
SHA256-Sidecar. Zugriff nur bei berechtigter Anbindung und konkreter
Netzwerkfreigabe; lokal zugängliche Originale zuerst verwenden.

DAY-CLOSE-08 belegt Commit und damalige Pushes zu beiden Projektremotes.
DAY-CLOSE-09 erfasste ab 19:56:19 CEST 173 abgeschlossene Logs mit
135 PASS/38 FAIL, DAY-CLOSE-10 ab 19:59:33 CEST 174 mit 136 PASS/38 FAIL.
Diese Bestände gelten nur zum jeweiligen Erhebungszeitpunkt vor dem
laufenden Block. Historische FAILs nicht umdeuten. DAY-CLOSE-10 war erfolgreich;
seine belegte Codex-Analyse ersetzt keine endgültige Gesamtqualifikation.
Vollständige Tagesabdeckung, Übergabeabnahme und praktische Wiederaufnahme
bleiben offen; die Dokumentationsaktualisierung allein erfüllt sie nicht.

R6-Sicherungscheckpoint qualifiziert, vollständiger Restore offen.
catalog-v2 nur im geprüften Umfang qualifiziert. DDR/SPL, vollständige
Bootprovenienz und vollständiger externer Boot offen. I2C3/ES8328 geschlossen.

Runner `~/novena-backups/block-logs/run-block.bash` und GitHub-Wrapper
`~/novena-backups/github-transport/run-and-upload.bash` bleiben verbindlich
und unverändert. Ihre Nutzung setzt die konkreten Schreib-/Uploadfreigaben
voraus und erweitert keinen Read-only-Auftrag.

Nächste organisatorische Aufgabe: Tages-Sollumfang und Wiederaufnahmekriterium
festlegen, aktualisierte Übergabe abnehmen und praktische Wiederaufnahme
prüfen. Keine neue technische Freigabe aus diesen Hinweisen ableiten.

## 2026-10-13 – Startpunkt für den nächsten Projektchat

Wir setzen das Projekt „Novena – universelles NixOS
SD-Image“ nach dem Tagesabschluss vom 2026-10-12 fort.

Arbeitsrechner: foobox.
Repository: ~/nixos-novena-image.
Branch: main.

Letzter bestätigter HEAD:
17524d75222ee8301efac3ce97e84c6a31640877

Geschützter Index-SHA256:
6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937

DAY-CLOSE-12 und TOOLS-CLOSE-02 sind abgeschlossen
und auf Codeberg und GitHub synchronisiert.

Bitte lies zuerst AGENTS.md,
NOVENA-ARBEITSREGELN.txt, die aktuelle Übergabe,
TODO.md und die relevanten Entscheidungsnachträge.

Prüfe die Wiederaufnahme anhand der dokumentierten
Originalquellen und fasse die drei nächsten
Projektaufgaben zusammen.

Erhalte die neun ITA-Patches 0004–0012,
sämtliche Nutzeränderungen und den Originalindex.

Keine neue allgemeine Auditserie.
Keine Builds, Patches, Hardwaretests,
Commits oder Pushes ohne Freigabe.

## NOVENA-DAY-CLOSE-13 – 2026-10-13

Beim nächsten Projektchat zuerst den neuesten
Nachtrag in docs/HANDOVER-LATEST.md und den
Tagesabschluss NOVENA-DAY-CLOSE-13 lesen.

Aktueller Git-HEAD:
82cdbf880ab4b0993b4d4fd82415b41e85afde9e

Index-SHA256:
6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937

DDR-07H bis DDR-07K erfolgreich:
vollständiges Image gebaut, geprüft und auf
Novena-Test-SD geschrieben; SHA256-Readback PASS.

Patch 0013 noch nicht auf der Novena getestet.
Nächster Schritt ist der freizugebende serielle
Kaltstarttest, nicht ein erneuter Image-Build.

Jede weitere Testvariante als vollständiges
NixOS-SD-Image bauen. Einzelne SPL-/U-Boot-Builds
sind kein regulärer Ersatz für den Gesamtbuild.

Keine neuen Änderungen oder Hardwaretests ohne
konkrete Freigabe.


## Nachtrag 2026-10-14 – Verbindlicher Einstieg nach Arbeitsregeln 13 und 14

Bei einem neuen Chat oder nach längerer Unterbrechung ist die Wiederaufnahmeprüfung nach Arbeitsregel 14 Voraussetzung für neue technische Empfehlungen und Arbeitsblöcke. Lies zuerst AGENTS.md und NOVENA-ARBEITSREGELN.txt einschließlich der Regeln 13 und 14 vom 2026-10-14, dann die neueste Übergabe, den letzten Tagesabschluss, TODO und die relevanten Projektdateien samt datierten Nachträgen. Werte die letzten verfügbaren vollständigen Originalprotokolle aus; eine Zusammenfassung ersetzt diese Nachweise nicht. Benenne fehlende Zugriffe, widersprüchliche Angaben und unvollständige Prüfungen ausdrücklich, bevor darauf aufgebaut wird.

Arbeitsregel 13 verlangt eine projektweite Quellenanalyse im sachlich relevanten und erlaubten Umfang. Berücksichtige Hardware, Firmware, SPL, U-Boot, Kernel und NixOS, ihre Schnittstellen, historische Implementierungen, bestätigte Ursachen, ausgeschlossene Hypothesen sowie Reproduzierbarkeit und Wartbarkeit. Lokale und bereits bereitgestellte Quellen werden zuerst ausgewertet; erforderliche externe Primärquellen folgen nur innerhalb der konkreten Freigabe. Trenne gesicherte Erkenntnisse, ungeprüfte Hypothesen, Risiken, Abhängigkeiten und offene Fragen. Begründe Änderungen der Aufgabenreihenfolge und bewerte Folgen einer vorgeschlagenen Lösung für spätere Projektschritte.

Bei bestehender berechtigter GitHub-Anbindung und freigegebenem Netzwerkzugriff suche benötigte Originalprotokolle selbstständig in `loomit77/project-block-logs` anhand der Blockkennung und der neuesten einschlägigen Commits. Rufe unter `novena/YYYY/MM/` die vollständige TXT-Datei und den zugehörigen `.txt.sha256`-Sidecar ab und werte beide gemeinsam aus. Fordere keinen erneuten manuellen Upload an, solange die benötigten Dateien direkt zugänglich sind. Fehlender Zugriff bleibt eine Nachweislücke; nicht abgerufene Inhalte gelten nicht als geprüft. Uploadstatus, Runnerstatus, Protokollintegrität und fachliches Resultat werden jeweils gesondert angegeben.

Für die Wiederaufnahme ist folgender im Auftrag übergebener Checkpoint vom 2026-10-14 zu berücksichtigen:

- HEAD: `7d98296db282aed0cadbf1d6cf9545ccb591f8a6`.
- Index-SHA256: `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
- ARCHIVE-08: Ausführung beendet, Runner-FAIL, zwei Downloadfehler; kein fachlicher Archivabschluss.
- DDR-08H: Q1/C1 ZERO, D32 `ret=-12`; kein bestätigter vollständiger Boot und kein bestätigter Patchfix. DDR/SPL bleibt offen.
- I2C3/ES8328 bleibt geschlossen; `regulator-always-on` bleibt dauerhafte Boardanforderung.

Diese Angaben wurden hier aus dem Auftrag übernommen, nicht durch Git-Abfragen oder Originalprotokollabrufe neu verifiziert. Ältere Checkpoints bleiben historische Nachweise. Erhalte Index, Intent-to-add-Einträge und sämtliche Nutzeränderungen. Unterscheide abgeschlossene, fehlgeschlagene, vorbereitete und laufende Arbeiten; kläre vor Änderungen Überschneidungen paralleler Arbeiten an Dateien, Datenbanken und Hardware. Vor Skriptänderungen sind vollständiger relevanter Quellcode, Schnittstellen, Annahmen und Abhängigkeiten zu prüfen. Unprotokollierte Vorbereitung bleibt als Lücke sichtbar. Diese Einstiegspflichten erteilen keine Schreib-, Netzwerk-, Git-, Build- oder Testfreigabe.

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

### Konkreter Einstieg im nächsten Chat

„Wir setzen Novena – universelles NixOS SD-Image nach dem Abschluss
vom 15.10.2026 fort. Lies zuerst AGENTS.md, NOVENA-ARBEITSREGELN.txt,
README.md, die neuesten Nachträge in HANDOVER-LATEST, DAILY-CLOSE,
TODO, PROJECT-STATE, TEST-LOG und DECISIONS. Prüfe den tatsächlichen
Git-Zustand lesend mit `git --no-optional-locks` und erhalte alle vorhandenen
Änderungen sowie den Index. Werte die vorhandenen vollständigen
Boot-/Testnachweise aus. Fasse den SD-Image-Stand, Evidenzlücken und den
minimalen nächsten Projektblock zusammen. Zunächst nur lesen;
keine Builds, Tests, Borg-Ausführungen oder Änderungen.“

Die älteren Anweisungen zu einem unmittelbar nächsten Commit/Push,
Hardwaretest oder vorrangigen Starter-/Werkzeugprojekt sind historische
Planungsstände und keine aktuelle Aktionsfreigabe.


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
[TEST-LOG](TEST-LOG.md#2026-10-17--zwei-skripte-workflow-realer-e2e-test-pass).

### Einstieg im nächsten Chat

Nach freigegebenem Projektstart über den Wrapper die Ausgabe von
`chat-start.bash` zusammen mit Regeln, README, TODO, aktueller Übergabe,
DAILY-CLOSE und den jüngsten Nachträgen in PROJECT-STATE, TEST-LOG und
DECISIONS auswerten. Historische HEAD-/Indexwerte oben sind keine aktuellen
Sollwerte. Tatsächlichen Git-Zustand lesend prüfen, Nutzeränderungen erhalten.
Der erfolgreiche Starttest belegt die genannten Startfunktionen; der offene
flock-Konkurrenztest betrifft den Tagesabschluss. DDR/SPL und externer Boot
bleiben offen; I2C3/ES8328 bleibt abgeschlossen.

## 2026-10-18 – Historischer Debian-Bootpfad BOOT-05 bis BOOT-08

Die gezielte historische Untersuchung NOVENA-DEBIAN-BOOT-05
bis NOVENA-DEBIAN-BOOT-08 vom 17.10.2026 ist abgeschlossen.

BOOT-08 verglich die historischen U-Boot-Revisionen
c5efeadb913c8246be372054adcd6106ad4fe067 und
3d26413e65f985131a9573d99a51fd47e7253188
hinsichtlich SPL-Konstanten, Headerbehandlung,
Ladeadressen und relevanter Quellcodeunterschiede.

Die statische Adressberechnung ergab
ADDRESS_CALCULATION_RESULT=PASS.
Der Untersuchungsblock endete mit
NOVENA_DEBIAN_BOOT_08_RESULT=PASS.

Originalprotokoll:
NOVENA-DEBIAN-BOOT-08-20261017-220809-F8puRUnB.txt

GitHub-Protokollarchiv:
loomit77/project-block-logs/novena/2026/10/

Der erfolgreiche historische Vergleich belegt weder
einen vollständigen Boot des aktuellen NixOS-Images
noch eine Behebung der weiterhin offenen DDR/SPL-Ursache.

Dieser historische Untersuchungsabschnitt ist geschlossen.
Keine erneute gleichartige Untersuchung ohne neue Evidenz.

## Tagesabschluss 2026-10-19 – Fortsetzung

Aktueller technischer Stand:
Das historische NixOS-Image mit ALSA und Grow ist gebaut
und statisch erfolgreich geprüft.

Image:
`/nix/store/wzkx9cn7la3qpwqxp3lvjhlpjgsam24h-nixos-novena-sd-image.img`

SHA256:
`1c854d6a148bd212aca5ebe7e3705c84bc4f6b0e5ff62c1f4a211cf5b0fd3406`

Maßgebliche Protokolle:
- NOVENA-NIXOS-HISTORICAL-ALSA-GROW-03
- NOVENA-NIXOS-HISTORICAL-ALSA-GROW-04
- NOVENA-NIXOS-HISTORICAL-ALSA-GROW-05
- NOVENA-NIXOS-HISTORICAL-ALSA-GROW-05R

Nächster Schritt:
Test-SD auf foobox eindeutig identifizieren.
Danach nur mit gesonderter Freigabe schreiben,
zurücklesen und auf der Novena booten.

Den aktuellen Git-Checkpoint und Backupstatus
aus dem jüngsten Tagesabschlussprotokoll übernehmen.
Historische Checkpoints bleiben erhalten.

## Fortsetzung 2026-10-20 – Hardwaretest vorbereiten

Auf foobox sind drei vollständige NixOS-Images bestätigt:

1. Historisches U-Boot 2015.04, Hardwareboot PASS:
   `/nix/store/hs4ygzgcd53hpy5hnxrrgvryw95fk5c8-nixos-novena-sd-image.img`
   SHA256 `4748322366a17dcfcafa75d4591f60ebc34629d9e4973807ab81a7794117170c`

2. Modernes U-Boot 2026.07, vollständiger Hardwareboot nicht nachgewiesen:
   `/nix/store/p4fwzdzm8j57xji213vcva4wg5a79hiq-nixos-novena-sd-image.img`
   SHA256 `937776dbb17f0a870fca22519c20a04845dd530de7db8c0496b3e5d286cc2934`

3. Historisches U-Boot 2015.04 mit ALSA/Grow, noch nicht hardwaregetestet:
   `/nix/store/wzkx9cn7la3qpwqxp3lvjhlpjgsam24h-nixos-novena-sd-image.img`
   SHA256 `1c854d6a148bd212aca5ebe7e3705c84bc4f6b0e5ff62c1f4a211cf5b0fd3406`

VM102: ARMv7-Image-Build am 20.10. erfolgreich:
`/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img`
SHA256 `b1ae43b3ce973377faa92faa5d466a94f3cda4357dacca26e1390a0e67cc7ad9`

Die sieben geprüften Quelldateien auf foobox und VM102 sind identisch.
Der VM-Build nutzte nach statischer Konfiguration den Default
`current-2026`; das Image ist nicht bitidentisch mit den verglichenen
älteren Images. Ein vollständiger Reproduzierbarkeitsnachweis
liegt nicht vor. Beim nächsten neuen Image auf beiden Hosts
exakt dieselbe Konfiguration verwenden.

NOVENA-VM-REPRO-04 scheiterte auf VM102 an fehlendem `python3`;
das ist kein Image-Fehler.

Nächster Schritt: Novena/serielle Konsole vorbereiten, Test-SD
eindeutig identifizieren und ausschließlich nach Freigabe das
ALSA/Grow-Image vom 19.10. schreiben und testen.

## Aktueller Arbeitsstand 2026-10-20 – Doppelbuild

<!-- NOVENA-DOC-DUAL-BUILD-2026-10-20 -->

Die beiden vollständigen NixOS-SD-Image-Builds auf foobox
und VM102 wurden erfolgreich abgeschlossen.

Beide verwendeten dieselbe Image-Derivation und erzeugten
denselben Store-Ausgabepfad. Die Images sind nicht
byteidentisch. MBR, erste 8 MiB und FAT32-Bootpartition
sind identisch.

Der vollständige Root-Dateivergleich grenzt die
Inhaltsunterschiede auf Coreutils 9.11 und Boost 1.89.0 ein.
Die jeweiligen Derivationen sind identisch; die erzeugten
Store-Ausgaben unterscheiden sich und sind jeweils gültig.
Die eigentliche Ursache ist noch nicht nachgewiesen.

Abschlussbericht:
docs/NOVENA-DUAL-REPRO-2026-10-20.md

Der historische U-Boot 2015 wurde auf der Novena getestet.
Die vollständige Bootqualifikation des neuen NixOS-Images
steht weiterhin aus.

Die Novena war zuletzt ausgeschaltet. Kein neuer Build,
SD-Schreibzugriff oder Hardwaretest ohne gesonderte Freigabe.

Vor Fortsetzung aktuelle TODO, Handover und Git-Status lesen.

## Nachtrag 2026-10-21: Audio, Borg VM103 und Tagesabschluss
### Audio / Novena

- NixOS 26.05, Linux 6.18.49 ARMv7, Patch 0007.
- Kopfhörerausgabe funktioniert.
- Interne Lautsprecher sind angeschlossen, bleiben aber ohne hörbare Ausgabe.
- Während der Wiedergabe waren Speaker, LOUT2, ROUT2 und
  audio-amp im ASoC-DAPM-Pfad aktiv.
- `es8328-power` war enabled, GPIO5_17 stand auf HIGH.
- Das belegt keine physisch gemessene Spannung an den
  Lautsprecherverstärkern U10A/U12A.
- Originalschaltplan: NS4890B/TS4990IST, Versorgung
  `P5.0V_DELAYED`, Steuerung `AUD_PA_SHDN`.
- Kein Multimeter vorhanden. Hardwaremessungen und weitere
  Lautsprecherdiagnose auf unbestimmte Zeit zurückgestellt.
- Keine weitere Kernel-, DTS- oder GPIO-Änderung beauftragt.

### Borg / VM103

- Backupziel: Debian-13-VM103, 3-TB-Datenträger.
- Repository:
  `ssh://borgadmin@192.168.178.20/srv/borg-storage/novena/foobox`
- Das verschlüsselte Borg-Repository ist über die bestehende
  SSH-Konfiguration erreichbar.
- Die erste Sicherung wurde am 2026-10-21 durch Block 97
  als temporärer Dienst gestartet.
- Originalprotokoll:
  `~/novena-backups/vm103-backup-state/first-backup-20261021-073016.log`
- Borgmatic scheiterte bei `borg create --dry-run --list`
  mit Exitcode 105 (Backup permission issue).
- Block 135 bestätigte: Repository erreichbar, aber null Archive.
- Deshalb ist noch keine erfolgreiche VM103-Sicherung nachgewiesen.
- Restore-Test offen.
- Das bisherige USB-Borg-Repository bleibt unverändert.
- Nächster Backupschritt: Berechtigungsursache gezielt untersuchen,
  anschließend erst nach Freigabe Sicherung erneut starten.
- Wichtig: Für Borg die vorhandene Datei
  `~/.config/novena-borg/vm103-known-hosts` verwenden.
  Der direkte Borg-Aufruf in Block 131 nutzte diese nicht.

### Nachweise

- NOVENA-PATCH-0007-SPEAKER-POWER-LIVE-130
- NOVENA-DAY-CLOSE-PRECHECK-131
- NOVENA-BORG-SSH-PRECHECK-132
- NOVENA-BORG-ORIGINAL-RUN-133
- NOVENA-BORG-FIRST-BACKUP-STATUS-134
- NOVENA-BORG-ARCHIVE-VERIFY-135

Die TXT-Protokolle und SHA256-Sidecars liegen im privaten
GitHub-Repository `loomit77/project-block-logs` unter
`novena/2026/10/`. Wrapper-PASS ist kein Beweis
für einen fachlich erfolgreichen Borg-Lauf.

### Fortsetzung im nächsten Chat

Arbeitsstand vom 2026-10-21. Zuerst die verbindlichen
Projektregeln und die jüngsten datierten Nachträge lesen.

Aktueller technischer Stand:
- NixOS SD-Image läuft auf der Novena.
- Kopfhörer funktionieren; Lautsprecherdiagnose pausiert.
- Borg-VM103 ist erreichbar, aber das erste Backup ist
  wegen Exitcode 105 fehlgeschlagen; null Archive.
- Keine neue Sicherung, kein Build, keine GPIO-Manipulation
  und keine Hardwaretests ohne ausdrückliche Freigabe.

Nächster sinnvoller Auftrag:
Die Berechtigungsursache des ersten VM103-Borg-Laufs
anhand des Originalprotokolls gezielt und möglichst
lesend untersuchen. Keine neue SSH-Konfiguration aufbauen.
Vor jedem neuen Block die bisherigen Originalprotokolle
und den aktuellen Git-Zustand prüfen.

Dieser Nachtrag hat Vorrang vor älteren, ausdrücklich
als historisch gekennzeichneten Statusangaben.
