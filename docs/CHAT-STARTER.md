# Novena – Starttext für einen neuen Chat

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
