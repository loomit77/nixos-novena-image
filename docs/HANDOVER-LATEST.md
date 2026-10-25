# Novena – verifizierte Projektübergabe

> Aktuelle Entwicklungspriorität: **NOVENA-ENTSCHEIDUNG-2026-10-18-NIXOS-FIRST**, siehe Nachtrag am Dateiende. Frühere Priorisierungen bleiben historische Stände.

> Aktueller NixOS-Stand: siehe Nachtrag „2026-10-18 – SD-Wiederherstellung und Systemstatus“ am Dateiende. Für die Wiederaufnahme ist dieser Folgestand zu beachten.

> Aktueller Workflowstand: siehe Nachtrag vom 2026-10-17 am Dateiende (NOVENA-WORKFLOW-FLOCK-02 PASS; konkreter Konkurrenzfall abgeschlossen). Frühere Statusangaben sind datierte historische Stände.

Stand: 2026-10-12, Europe/Berlin  
Prüfhost: foobox  
Repository: `~/nixos-novena-image`  
Geltung: dokumentierter Prüfstand, keine Aktionsfreigabe.

## 1. Einstieg und verbindliche Regeln

Vor jeder Projektfortsetzung vollständig lesen:

1. `AGENTS.md`
2. `NOVENA-ARBEITSREGELN.txt`
3. `README.md`
4. `docs/HANDOVER-LATEST.md`
5. `TODO.md`
6. Einschlägige datierte Nachträge in `docs/PROJECT-STATE.md`,
   `docs/TEST-LOG.md`, `docs/DECISIONS.md` und
   `docs/project-index/README.md`.

`AGENTS.md` hat Vorrang vor dem operativen Regelwerk.
Der konkrete Auftrag kann engere Grenzen setzen.
Keine dieser Dateien erteilt selbst eine Aktionsfreigabe.

Eine rein lesende Freigabe umfasst weder temporäre Schreibvorgänge
noch Runnerlogs oder Netzwerkübertragung. Protokollierung und Upload
benötigen die dafür erforderlichen Freigaben.

## 2. Git-Checkpoint und Schutz des Arbeitszustands

Historischer Prüfstand vor der Indexbereinigung vom 2026-10-12:

- Branch: `main`
- HEAD: `4e024f8e9da1af2fe91990b0beee301a50ac4ea2`
- Historischer `.git/index` SHA256:
  `90f75b5f8785128730b3f75741b1063299ac90452b8c2e8cf09234e7477bc0c2`

HEAD, Index und Worktree sind getrennte Zustände.

Im historischen Zustand enthielt der Index zahlreiche vorgemerkte Löschungen, während
gleichnamige Dateien im Worktree untracked vorhanden waren.
Daneben bestanden `MM`-Zustände und Intent-to-add-Einträge.
Diese Beschreibung der vielen Löschvormerkungen ist historisch
und beschreibt nicht den bereinigten Indexstand des folgenden Nachtrags.

Insbesondere die U-Boot-Patches 0004–0012 besitzen dokumentierte
Intent-to-add-Flags `20004000`.

Keine pauschalen `git add -A`, `git reset`, `git clean`,
Restores oder sonstigen Indexreparaturen.

Vor Änderungen den tatsächlichen Zustand erneut lesen und
mit diesem datierten Checkpoint vergleichen.

### Nachtrag 2026-10-12: Git-Indexbereinigung abgeschlossen

Der nach der Bereinigung dokumentierte Zustand lautet:

- Aktueller HEAD: `4e024f8e9da1af2fe91990b0beee301a50ac4ea2`.
- Aktueller `.git/index` SHA256:
  `665d9b5b0f30fb573027c1697cd2d7731108982f62ab121755727e5fcc8ae800`.
- Cleanup-04: 10 normalisierte Indexeinträge.
- Cleanup-06: 47 normalisierte Indexeinträge.
- Cleanup-10: 7 normalisierte Indexeinträge.
- Cleanup-11: abschließende read-only Prüfung.
- Insgesamt 64 normalisierte Indexeinträge.
- HEAD: 407 Einträge; Index: 416 Einträge.
- Neun Intent-to-add-Patches 0004–0012 bleiben erhalten.
- Keine gestagten Inhaltsänderungen.
- Sieben modifizierte Dateien, eine fehlende Forschungsdatei
  und 23 unversionierte Dateien.

Dieser Nachtrag übernimmt die dokumentierten Bereinigungsergebnisse;
er ist keine erneute Git-Prüfung. Der historische Index-SHA256
und sämtliche historischen Projektbefunde bleiben erhalten.
Die Bereinigung schließt DDR/SPL nicht ab und bestätigt Patch0012
nicht als isolierten Fix. I2C3/ES8328 bleibt abgeschlossen.
Die Regeln gegen pauschale Git-Reparaturen und für gesonderte
Freigaben vor Builds oder Hardwaretests gelten weiterhin.

Die letzte dokumentierte Remote-Synchronisierung und der
aktuelle Remote-Zustand sind getrennt zu bewerten.
Dieser Übergabestand bestätigt keine neue Remote-Abfrage.

## 3. Technischer Bootstand

Das Projekt verfolgt ein reproduzierbares, eigenständig
bootfähiges externes NixOS-SD-Image für Kosagi Novena.

Für den P_EXT-Pfad ist belegt:

`P_EXT → ROM → externe SD/USDHC2 → SPL v2026.07`

Der anschließende vollständige externe Boot über U-Boot proper
bis NixOS ist damit noch nicht nachgewiesen.

DDR/SPL bleibt eine offene technische Hauptfrage.

Im kontrollierten A-B-B-A-Experiment galt:

- A: BI0 bis Patch0011.
- B: A zuzüglich Patch0012.
- Alle vier Läufe: Q1/C1 ZERO.
- Anschließend D32 `ret=-12`.

Patch0012 wird unter diesen Bedingungen nicht als
isolierter Fix unterstützt. Die DDR/SPL-Root-Cause bleibt offen.
Der D32-Fehler erklärt die frühere Zero-Signatur nicht.

Maßgeblich sind die datierten DDR-Abschnitte in
`docs/PROJECT-STATE.md` und `docs/TEST-LOG.md`.

## 4. Abgeschlossene Untersuchungen

Die I2C3/ES8328-Powertransient-Untersuchung ist abgeschlossen.
Die Boardanforderung `regulator-always-on` bleibt erhalten.

Ohne neue diskriminierende Evidenz darf die geschlossene
Root-Cause nicht erneut als offene Ausgangshypothese behandelt werden.

## 5. Projektindex und historische Evidenz

INDEX65 wurde innerhalb seines festgelegten Umfangs formal
als `QUALIFIED_ACCEPTED` abgeschlossen.

K2-A umfasst S01–S09/S13/S16:
acht PASS und drei QUALIFIED_PASS.

Dies bedeutet weder globale Testunabhängigkeit noch vollständige
Original- oder Hardwarequalifikation.

Besonders wichtig ist die historische FND-000001-Bindung:

Historischer TEST-LOG-SHA256:
`b165636a7e5146903d9675639d8d435a474c88f7231341922781bbf027b00719`

Aktuelle dokumentierte TEST-LOG-Fassung:
`6c9a935eb5498f6e08f1f99be199d91ddc769a62776b90fe0de91430c0035a12`

Die historische Ganzdateibindung ist für die aktuelle
Dateifassung nicht erfüllt.

Historische Registerwerte nicht stillschweigend nachziehen.
Eine Snapshot-/Resolverstrategie ist noch nicht implementiert.

## 6. Dateiinventur catalog-v2

Der geprüfte Produktionsbestand wurde technisch qualifiziert:

- 611.930 Einträge beziehungsweise Klassifikationen.
- 47 Referenzauflösungen.
- 46.026 abgeleitete Verzeichnisse.

Klassifikationen und Auflösungen wurden aus v1 übernommen.

Die Abnahme ist keine vollständige historische Medieninventur.
Zwei fehlgeschlagene Auditläufe behalten ihren FAIL-Status.

Datenbankidentität, Umfang und Grenzen stehen in den
aktuellen datierten catalog-v2-Nachträgen der Projektdokumentation.

Die gesonderte Dateiinventur bleibt ein eigener Arbeitsstrang.

## 7. Backup und Protokolltransport

R6 ist als externer HDD-Sicherungscheckpoint qualifiziert.
Eine vollständige Wiederherstellungsprobe ist weiterhin offen
und benötigt eine eigene Freigabe.

Die HDD wurde durch diese Übergabe nicht erneut geprüft.

Der bestehende Runner befindet sich unter:

`~/novena-backups/block-logs/run-block.bash`

Der GitHub-Transportwrapper befindet sich unter:

`~/novena-backups/github-transport/run-and-upload.bash`

Technisches Blockergebnis, TXT-Vollständigkeit, SHA256-Integrität
und Transportergebnis sind getrennt auszuwerten.

Erfolgreiche Uploads beweisen keine vollständige
Uploader-Sicherheitsqualifikation.

Eine automatische Geheimniserkennung ist nicht nachgewiesen.
Vor freigegebenen Uploads müssen sensible Inhalte ausgeschlossen sein.
Das private Transportrepository ist kein Geheimnisspeicher.

## 8. Weitere offene Fragen

Zu den offenen beziehungsweise begrenzt qualifizierten Themen gehören:

- SO-DIMM, SPD, Rank und DDR-Provenienz.
- Factory-R1-Quellenlücken.
- NXP-Quellenaufnahme und eigenständige Inhaltsqualifikation.
- SRAM-Snapshot-Machbarkeit.
- Vollständiger externer Bootpfad.
- Historische Indexbindung und fehlende Resolverstrategie.
- R6-Wiederherstellungsprobe.
- Noch offene Transport- und Sicherheitsqualifikationen.

Keine dieser Fragen rechtfertigt ohne Quellenabgleich
automatisch einen neuen Hardwareversuch oder Build.

## 9. Aktuelle Aufgaben und nächster Schritt

`TODO.md` ist die laufende Aufgabenquelle.
Neuere datierte Nachträge haben Vorrang vor älteren,
teilweise überholten offenen Kästchen.

Zum Zeitpunkt dieser Übergabe ist der nächste
organisatorische Schritt:

Die Übergabedokumente fachlich prüfen, anschließend nur
mit entsprechender Freigabe gezielt versionieren und
die beiden Remotes unabhängig überprüfen.

Danach einen technischen Arbeitsauftrag aus der
aktuellen TODO und den jüngsten Projektnachträgen auswählen.

Vor jedem neuen Arbeitsblock:

- Vorherigen Block vollständig auswerten.
- Tatsächlichen Git-Zustand und maßgebliche Quellen prüfen.
- Freigabegrenzen und Nachweiszweck festlegen.
- Bei relevantem Widerspruch anhalten.
- Die vorgeschriebene Protokollierung nur innerhalb
  der tatsächlich erteilten Schreib-/Netzwerkfreigabe ausführen.

## 10. Maßgebliche Nachweise

Primäre lokale Quellen:

- `AGENTS.md`
- `NOVENA-ARBEITSREGELN.txt`
- `README.md`
- `TODO.md`
- `docs/PROJECT-STATE.md`
- `docs/TEST-LOG.md`
- `docs/DECISIONS.md`
- `docs/project-index/README.md`
- `test-logs/`
- `evidence/`
- `research/`

Letzte Übergabeprüfungen:

- `NOVENA-HANDOVER-01-20261012-150559-w9niTCaA.txt`
- `NOVENA-HANDOVER-02-20261012-151816-ha2nh6So.txt`
- `NOVENA-HANDOVER-02A-20261012-152256-tkQBwdT4.txt`

Die vollständigen Protokolle liegen im freigegebenen
Blocklog-/Transportverfahren.

Die Übergabe ersetzt weder Rohlogs noch historische Quellen.
Dokumentierte Ergebnisse und aktuell neu geprüfte Tatsachen
müssen ausdrücklich unterschieden werden.

## 11. Projektgrenzen

foobox ist der primäre Analyse-, Repository- und Buildhost.
Novena ist die Zielhardware.

Das L14-Projekt, `novena-system`, historische Quellrepositories
und das separate Protokolltransportrepository sind
eigenständige Arbeitsbereiche.

Keine projektübergreifenden Änderungen ohne konkrete Freigabe.

## Nachtrag 2026-10-12: Chat-Kontinuität und Protokollabruf

Der spätere verifizierte Projektcheckpoint lautet:

- Branch: main
- HEAD: dab134ec9969221d15983f73d1e0a894b930e292
- Index-SHA256:
  6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937

Nachweis:
NOVENA-HANDOVER-VERIFY-03-20261012-175805-65E7YuW7.txt

Git-HEAD, Index und fünf zentrale Projektquellen
stimmten mit dem Übergabestand überein.
Drei frühere Übergabeprotokolle bestanden die
lokale Integritätsprüfung.

Die ITA-Auswertung dieses einzelnen Prüfskripts
war nicht aussagekräftig. Sie begründet keine
Reparatur oder Wiederholung bereits qualifizierter
Indexprüfungen.

Das private GitHub-Protokollrepository lautet:

loomit77/project-block-logs

ChatGPT kann bei vorhandener berechtigter
GitHub-Anbindung die hochgeladenen Protokolle
selbstständig anhand der Blockkennung suchen.

Archivpfad:
novena/YYYY/MM/

Bewährter Wrapper-Aufruf:

bash "$WRAPPER" "$BLOCK_ID" "$COMMAND_FILE"

Wrapperpfad:
~/novena-backups/github-transport/run-and-upload.bash

Die vollständige TXT-Datei und ihr SHA256-Sidecar
sind getrennt abzurufen und auszuwerten.

Manuelle erneute Uploads und redundante Prüfblöcke
sind zu vermeiden, solange vorhandene Evidenz
zugänglich und ausreichend ist.

Historische Checkpoints bleiben unverändert.
Die Dokumentation erteilt keine neuen Freigaben.

## Nachtrag 2026-10-12: erste Chat-Übergabe abgeschlossen

Die erste praktische Chat-Übergabe und ihre anschließende
Dokumentationsintegration wurden abgeschlossen.

Verifizierter synchronisierter Commit:
`3ac6b4fffd8322032d7c9e5098fd51133484bc84`.

Nachweis:
`NOVENA-CHAT-CONTINUITY-10`.

Ergebnis:

- Codeberg-Push: PASS.
- GitHub-Push: PASS.
- Beide Remote-HEADs identisch: PASS.
- Lokaler HEAD erhalten: PASS.
- Ursprünglicher Git-Index erhalten: PASS.
- Keine neuen Builds oder Hardwaretests.

Die vorangegangenen Blöcke dokumentieren außerdem:

- `NOVENA-CHAT-CONTINUITY-08`: FAIL aufgrund eines
  unzutreffend angenommenen SSH-Schlüsselpfads.
- `NOVENA-CHAT-CONTINUITY-09`: tatsächliche SSH-Konfiguration
  und vorhandenen Agent-Schlüssel ermittelt; PASS.
- `NOVENA-CHAT-CONTINUITY-10`: beide Remotes erfolgreich
  synchronisiert; PASS.

Die Fehlschläge bleiben historische Ergebnisse.
Sie werden nicht nachträglich zu PASS umgedeutet.

Die Übergabe hat bestätigt, dass ChatGPT bei vorhandener
GitHub-Berechtigung die Originalprotokolle selbst abrufen kann.

Nächste organisatorische Prioritäten:

1. Starter und Übergabeverfahren abschließen.
2. Tagesabschluss entwerfen und qualifizieren.
3. Werkzeuge anhand des vorhandenen Tools-Audits auswerten.

Keine erneute allgemeine Projektinventur beginnen,
solange die vorhandenen Ergebnisse den konkreten
nächsten Schritt ausreichend belegen.

Dieser Nachtrag erteilt keine neue Freigabe.

## Nachtrag 2026-10-12 – NOVENA-DAY-CLOSE-11: aktuelle Wiederaufnahmebasis

Geltung: gezielte Dokumentationsaktualisierung auf foobox anhand der
Originalprotokolle DAY-CLOSE-08/09/10; keine vollständige Tagesabnahme.
Der folgende Stand ist neuer als die historischen Übergabecheckpoints:

- Branch: `main`.
- Projekt-HEAD: `5e2f7afbad802b49b8305dd2a5437c8550bae2be`.
- Index-SHA256:
  `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
- Neun ITA-Einträge der U-Boot-Patches 0004–0012 mit Flags `20004000`
  sind erhalten; HEAD und Index wurden vor dieser Aktualisierung lesend geprüft.

DAY-CLOSE-08 belegt den erfolgreichen Verfahrenscommit dieses HEADs und
die damaligen erfolgreichen Pushes zu Codeberg und GitHub. Es erfolgte
hier keine neue Remote-Abfrage. Vorhandene Nutzeränderungen bleiben erhalten:
MM-Zustände bei Regeln, TODO, Starter und Übergabe, Änderungen an U-Boot,
englischer Dokumentation, Forschungsquellen und Rohlogs. Die vorgemerkte
Löschung bei zugleich vorhandener untracked `docs/DAILY-CLOSE.md` wird
nicht repariert; Index und Worktree sind getrennt zu betrachten.

DAY-CLOSE-09 erfasste am 2026-10-12 ab 19:56:19 CEST 173 abgeschlossene
Protokolle: 135 Runner-PASS, 38 Runner-FAIL. DAY-CLOSE-10 erfasste ab
19:59:33 CEST 174: 136 Runner-PASS, 38 Runner-FAIL. Beide Bestände schließen
das jeweils laufende Eigenprotokoll aus und gelten nur zum Erhebungszeitpunkt.
Historische FAILs bleiben FAIL. DAY-CLOSE-10 endete erfolgreich um
20:03:05 CEST; seine Codex-Auswertung ist belegte Analyse, keine endgültige
Gesamtqualifikation. Vollständige Tagesabdeckung und vollständiger
Tagesabschluss bleiben offen.

Fortgeltende fachliche Grenzen:

- R6 ist ein qualifizierter Sicherungscheckpoint; vollständiger Restore offen.
  Ein erfolgreicher Cleanup-Restore betrifft einen anderen Sicherungsumfang.
- catalog-v2 ist ausschließlich für den geprüften Produktionsbestand
  qualifiziert: 611.930 Einträge/Klassifikationen, 47 Referenzauflösungen,
  46.026 abgeleitete Verzeichnisse. Vollständige historische Inventur offen;
  beide fehlgeschlagenen 31K-Audits bleiben FAIL.
- DDR/SPL-Root-Cause, vollständige historische Bootausführungsprovenienz und
  vollständiger eigenständiger externer Bootpfad bleiben offen. P_EXT bis SPL
  ist belegt; Patch0012 ist kein belegter isolierter Fix.
- I2C3/ES8328 bleibt geschlossen; `regulator-always-on` bleibt Boardanforderung.

Maßgebliche Originale sind die drei in
[DAILY-CLOSE.md](DAILY-CLOSE.md#originalnachweise) exakt benannten TXT-Dateien
mit SHA256-Sidecars unter `~/novena-backups/block-logs/`. Das private
GitHub-Protokollarchiv `loomit77/project-block-logs`, Pfad `novena/2026/10/`,
ist eine weitere Evidenzquelle; es wurde in diesem Auftrag nicht abgefragt.

Nächster organisatorischer Schritt: Tages-Sollumfang mit Zeitraum und
begründeten Ausschlüssen sowie das Wiederaufnahmekriterium festlegen;
anschließend diese aktualisierte Übergabe fachlich abnehmen und die
Wiederaufnahme praktisch prüfen. Die Dokumente sind jetzt ergänzt, diese
Abnahme und praktische Prüfung sind noch nicht ausgeführt. Vor neuen
technischen Schritten zuerst vorhandene Originalprotokolle und maßgebliche
Projektdokumentation auswerten. Runner und GitHub-Wrapper bleiben verbindlich
und unverändert, jeweils innerhalb konkreter Protokollierungs-/Uploadfreigaben.
Dieser Nachtrag erteilt keine neue technische oder sonstige Aktionsfreigabe.

## 2026-10-12 – Übergabe nach TOOLS-CLOSE-02

Letzter bestätigter Projektcheckpoint:
`17524d75222ee8301efac3ce97e84c6a31640877`
auf `main`, foobox.

Geschützter Originalindex:
`6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.

Neun ITA-Patches 0004–0012 und sonstige
Nutzeränderungen bleiben erhalten.

DAY-CLOSE-12 und TOOLS-CLOSE-02 wurden erfolgreich
zu Codeberg und GitHub synchronisiert.
TOOLS-CLOSE-01 dokumentierte die Werkzeugentscheidungen;
TODO und DECISIONS enthalten den maßgeblichen Nachtrag.

Werkzeugauswahl abgeschlossen, bedarfsabhängige
Ergänzungen zurückgestellt. Keine neue
Automatisierungsplattform vorgesehen.

Für die nächste Wiederaufnahme:
1. Regeln und diese Übergabe lesen.
2. Aktuellen Git-Checkpoint kontrollieren.
3. TODO und DECISIONS berücksichtigen.
4. Praktischen Starter-Test durchführen.
5. Anschließend technische Aufgaben priorisieren.

Offen bleiben vollständige Tagesqualifikation,
praktische Wiederaufnahme, R6-Restore,
DDR/SPL- und Bootpfadfragen.
I2C3/ES8328 bleibt geschlossen.

Keine Builds, Hardwaretests oder Git-Änderungen
ohne ausdrückliche Freigabe.

## NOVENA-DAY-CLOSE-13 – 2026-10-13

Aktueller technischer Übergabestand:

- Projekt: Novena – universelles NixOS SD-Image.
- Host: foobox.
- Branch: main.
- HEAD:
  82cdbf880ab4b0993b4d4fd82415b41e85afde9e
- Geschützter Index-SHA256:
  6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937
- Intent-to-add-Einträge und vorhandene
  Worktree-Änderungen sind zu erhalten.
- U-Boot 2026.07 mit Patch 0013.
- Patch 0013 isoliert rtt_nom=1 zu rtt_nom=2.
- Vollständiges NixOS-SD-Image erfolgreich gebaut.
- Imagepfad:
  /nix/store/p4fwzdzm8j57xji213vcva4wg5a79hiq-nixos-novena-sd-image.img
- Imagegröße: 2581291008 Bytes.
- SHA256:
  937776dbb17f0a870fca22519c20a04845dd530de7db8c0496b3e5d286cc2934
- Vollständiges Image auf Novena-Test-SD geschrieben.
- Vollständiger SD-Readback: SHA256 PASS.
- DDR-07H, DDR-07I, DDR-07J und DDR-07K: PASS.
- Bootfähigkeit von Patch 0013 noch nicht getestet.
- DDR/SPL-Root-Cause weiterhin offen.
- I2C3/ES8328 weiterhin abgeschlossen.

Maßgebliche Originalprotokolle:
NOVENA-DDR-07H, DDR-07I, DDR-07J und DDR-07K
vom 2026-10-13, jeweils TXT und SHA256-Sidecar.

Arbeitsregel:
Jede neue Testvariante als vollständiges
NixOS-SD-Image bauen und qualifizieren.
Keine regulären isolierten SPL-Builds als Testweg.

Nächster technischer Schritt:
Novena-Kaltstart mit vorbereiteter Test-SD,
serielle Konsole vollständig aufzeichnen.
Nur nach ausdrücklicher Hardwaretestfreigabe.

Keine neuen Builds, Medien-Schreibzugriffe,
Patches, Git-Commits oder Pushes ohne Freigabe.

Die älteren datierten Übergaben bleiben historische
Checkpoints und werden nicht überschrieben.


## Nachtrag 2026-10-14 – Übergabestand und verpflichtende Wiederaufnahme

Dieser Nachtrag übernimmt die im Auftrag angegebenen neueren Checkpoints und Befunde. Die älteren datierten Übergaben bleiben unverändert als historische Zustände erhalten. In dieser isolierten Dokumentationskopie wurden weder Git-Zustand noch Remotes, Hardware oder Originalprotokolle neu geprüft.

Übergebener Projektcheckpoint:

- HEAD: `7d98296db282aed0cadbf1d6cf9545ccb591f8a6`.
- Geschützter Index-SHA256: `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.

Diese Werte sind keine neue Bestätigung des tatsächlichen Zustands dieser Kopie oder der foobox. Branch, Worktree, parallele Änderungen und Remote-Stände sind daraus nicht abzuleiten. Vor späteren autorisierten Änderungen ist der tatsächliche Zustand abzugleichen; vorhandene Nutzeränderungen, Intent-to-add-Einträge und Index bleiben geschützt.

ARCHIVE-08 ist in der Ausführung beendet. Das Runner-Ergebnis lautet FAIL; zwei Downloadfehler bleiben erhalten. Daraus folgt kein fachlicher Archivabschluss und keine vollständige Quellenaufnahme. Erfolgreiche Teilabrufe dürfen nur mit ihrer eigenen Evidenz bewertet werden. Fehlende Inhalte, Downloadursachen und etwaige Folgearbeiten sind anhand der vollständigen Originale zu klären; eine automatische Wiederholung ist nicht freigegeben.

DDR-08H zeigt Q1/C1 ZERO und D32 `ret=-12`. Ein vollständiger Boot und ein Patchfix sind nicht bestätigt; DDR/SPL und der vollständige eigenständige externe Bootpfad bleiben offen. Der ältere Stand vom 2026-10-13 beschreibt den damaligen Build-, Schreib- und Readbacknachweis. Er ersetzt die Auswertung des neueren DDR-08H-Befunds nicht. Aus `ret=-12` allein werden weder Root Cause noch Erklärung der vorherigen Zero-Signatur abgeleitet. I2C3/ES8328 bleibt geschlossen; `regulator-always-on` bleibt dauerhafte Novena-Boardanforderung.

Die hier übernommenen Blockangaben sind keine erneute Originalauswertung. Exakte Originaldateinamen, Hashwerte, Sidecars sowie Upload- und Integritätsstatus von ARCHIVE-08 und DDR-08H sind in diesem Auftrag nicht erhoben worden. Diese Nachweisidentitäten bleiben für die Wiederaufnahmeprüfung zuzuordnen; sie werden nicht aus Blockkennungen konstruiert.

Nach Arbeitsregel 14 vom 2026-10-14 sind vor neuen technischen Empfehlungen und Arbeitsblöcken zuerst Regeln, neueste Übergabe, letzter Tagesabschluss, TODO, relevante Projektdateien und letzte verfügbare Originalprotokolle auszuwerten. Lokal oder bereits bereitgestellte Quellen haben Vorrang. Bei berechtigter GitHub-Anbindung und konkreter Netzwerkfreigabe sind benötigte TXT-Originale und `.txt.sha256`-Sidecars selbstständig im bekannten Repository `loomit77/project-block-logs` unter `novena/YYYY/MM/` anhand der Blockkennung und einschlägiger Commits abzurufen. Zugängliche Dateien werden nicht unnötig erneut angefordert; fehlender Zugriff wird benannt, nicht als Prüfung behandelt.

Upload, Runner, Protokollintegrität und Fachresultat werden getrennt bewertet. Die Quellenanalyse nach Arbeitsregel 13 berücksichtigt die relevante gesamte Boot- und Systemarchitektur, historische Implementierungen, Schnittstellen, Abhängigkeiten, ausgeschlossene Hypothesen und spätere Projektfolgen. Gesicherte Erkenntnisse, Hypothesen, Risiken und offene Fragen bleiben getrennt; eine Änderung der Prioritäten braucht eine Begründung. Vor Skriptänderungen ist der vollständige relevante Quellcode zu prüfen.

Für parallele Arbeiten sind Zuständigkeiten, Ressourcen und Abschlusszustände festzustellen. Vorbereitete und laufende Blöcke sowie unvollständige Prüfungen bleiben als solche sichtbar; keine gegenseitigen Änderungen an denselben Dateien, Datenbanken oder Hardware-Ressourcen ohne abgestimmten Umfang. Unprotokollierte Vorbereitung wird als Nachweislücke geführt. Der vollständige Tagesabschluss muss Git-, Datei-, Backup- und Nachweisidentitäten samt Herkunft, Zeit und Grenzen dokumentieren; seine Gesamtqualifikation und die praktische Wiederaufnahme bleiben hier unbestätigt. R6-Restore bleibt offen.

Nächster empfohlener Schritt ist die autorisierte Wiederaufnahmeprüfung einschließlich der vollständigen Originalauswertung von ARCHIVE-08 und DDR-08H und der Zuordnung ihrer Nachweisidentitäten. Erst daraus sind ein minimaler Folgeblock und erforderliche Freigaben abzuleiten. Dieser Nachtrag erteilt keine Netzwerk-, Git-, Build-, Test-, Medien- oder Hardwarefreigabe.


---

## Übergabe 2026-10-14 – NOVENA-DAY-CLOSE-15D

Kennung: NOVENA-DAY-CLOSE-15D / 2026-10-14

### Verifizierter Tagesstand

- DAY-CLOSE-15A: PASS, Inventur der Tagesprotokolle.
- DAY-CLOSE-15B: PASS, Git- und Indexanalyse.
- DAY-CLOSE-15C: PASS, isolierte Commitvorschau.
- 75 erfasste Tagesprotokolle, davon 68 PASS und
  7 FAIL einschließlich des abgeschlossenen 15A.
- ARCHIVE-08 bleibt FAIL: Threads 488 und 561 fehlen.
- ARCHIVE-09A ist PASS: 343 HTML-Dateien vorhanden.
- DDR-08H: Q1/C1 ZERO; D32 ret=-12.
- Kein bestätigter vollständiger Boot mit dem
  zuletzt untersuchten Stand.
- Die vorhandenen U-Boot-/DDR-Änderungen dürfen
  nicht ungeprüft gestaged oder committet werden.

### Git-Schutzregeln

Ausgangs-HEAD:
7d98296db282aed0cadbf1d6cf9545ccb591f8a6

Ausgangs-Index-SHA256:
6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937

Der ursprüngliche Index ist geschützt. Für den
Dokumentationscommit einen isolierten Index
verwenden und nur die fünf in 15C geprüften
Dokumentationsdateien übernehmen.

### Unmittelbarer nächster Schritt

1. Ergebnis von 15D einschließlich Datei-Hashes prüfen.
2. Isolierten Commitumfang nach 15D erneut verifizieren.
3. Freigegebenen Dokumentationscommit erstellen.
4. Zu Codeberg und GitHub pushen.
5. Beide Remote-HEADs und Dokumentationsinhalte
   gegen den Commit prüfen.
6. Erst dann den Tagesabschluss als vollständig melden.

### Weitere offene Arbeiten

- DDR/SPL-Ursachenanalyse auf Grundlage der
  vorhandenen Originalprotokolle fortsetzen.
- Archiv-Threads 488 und 561 gezielt nachrecherchieren.
- Historische Quellen und vorhandene Testlogs
  systematisch auswerten.
- Keine neuen Builds, Patches, Hardwaretests oder
  SD-Schreibvorgänge ohne ausdrückliche Freigabe.

### Arbeits- und Protokollregeln

Alle technischen Blöcke über den bestehenden
Runner und GitHub-Transport protokollieren.
Vollständige TXT-Ausgabe und SHA256-Begleitdatei
sind Pflicht. Upload-Erfolg allein beweist keinen
technischen Blockerfolg.

Der nächste Chat beginnt mit der Prüfung der
gepushten Projektdateien und der Originalprotokolle
im GitHub-Protokollrepository.

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

### Tatsächlich lokal erhobener Git-Zustand

Prüfung am 2026-10-15T16:13:14+02:00,
Repository `/home/loomit/nixos-novena-image`, Branch `main`:
HEAD `ee99ba0c296626e63c214ec91f5b7d771e35d1f7`, Index-SHA256 `6fb7f10e615598a4d24878426cc58c7566abd1e582673c31d9cc9643afd94937`.
Dies ist eine lokale lesende Erhebung, keine Remote-Synchronisationsprüfung.
Der Status vor den Dokumentationsergänzungen lautet:

```text
MM NOVENA-ARBEITSREGELN.txt
MM TODO.md
 A boot/u-boot/0004-novena-uboot-diag-d0-d36.patch
 A boot/u-boot/0005-novena-diag-d21a-d21d.patch
 A boot/u-boot/0006-novena-diag-brk-p0-p1.patch
 A boot/u-boot/0007-novena-diag-brk-q1-q2.patch
 A boot/u-boot/0008-novena-diag-control-c0-c2.patch
 A boot/u-boot/0009-novena-diag-write-level-status.patch
 A boot/u-boot/0010-novena-diag-dqs-status.patch
 A boot/u-boot/0011-novena-spl-disable-bank-interleaving.patch
 A boot/u-boot/0012-novena-ddr-read-specific-dummy-write.patch
 M boot/u-boot/default.nix
MM docs/CHAT-STARTER.md
D  docs/DAILY-CLOSE.md
MM docs/DECISIONS.md
MM docs/HANDOVER-LATEST.md
MM docs/PROJECT-STATE.md
MM docs/QUELLENVERZEICHNIS.md
MM docs/TEST-LOG.md
 M docs/en/docs/BUILD-HOST.md
 M docs/en/docs/BUILD.md
 M docs/en/docs/GOLDEN-BUILD.md
 M docs/en/docs/IMAGE-LAYOUT.md
 M docs/en/docs/QUELLENVERZEICHNIS.md
 D docs/en/research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md
?? boot/u-boot/0013-novena-spl-rtt-nom-rzq2.patch
?? docs/DAILY-CLOSE.md
?? research/nxp-reference-manuals/
?? test-logs/2026-10-05/
?? test-logs/2026-10-07/
```

Die neun Patches 0004–0012 sind vorhandene Intent-to-add-Einträge.
Der Index enthält bereits Dokumentationsänderungen gegenüber HEAD;
`MM` bezeichnet zusätzlich abweichende Worktree-Inhalte.
`docs/DAILY-CLOSE.md` ist im Index zur Löschung vorgemerkt und liegt zugleich
als unversionierte Worktree-Datei vor. Keine Indexreparatur ableiten.
Vorhandene U-Boot-, Übersetzungs-, Forschungs- und Logänderungen gehören
nicht zum Schreibumfang dieses Auftrags. Die historischen HEAD-/Indexwerte
weiter oben sind keine aktuellen Wiederaufnahmevorgaben.

Dieser Auftrag ergänzt ausschließlich sieben bestehende Dokumentationsdateien.
HEAD und Index bleiben erhalten; keine Git-Mutationen, Commits, Pushes,
Builds, Tests, Borg-Ausführungen, Netzwerk-, Blockgeräte- oder Hardwarezugriffe.
Die neue Dokumentation ist lokal uncommittet; eine Veröffentlichung oder
aktuelle Übereinstimmung mit Codeberg/GitHub wird nicht behauptet.

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

<!-- BORG-INTEGRATION-24J -->

## Borg-Integration: Uebergabestand vom 2026-10-15

Pruefserie: BORG-INTEGRATION-24A bis 24I abgeschlossen.

Ergebnisse:

- Realer Borg-Lauf: PASS (Block 16).
- Archivintegritaet: PASS (Block 22).
- Historische Tagesabschlussintegration: PASS (35P-19).
- Status-30: 15/15 Offline-Tests PASS.
- Sperrmechanismus: 9/9 Offline-Tests PASS.
- Gesamtpruefung 24I: PASS.
- Git-HEAD und Originalindex bei den Pruefungen erhalten.

Offen bleiben der durchgaengige Sperrnachweis fuer
einen realen Borg-Lauf und der produktive End-to-End-Nachweis
einschliesslich Fehlerweitergabe.

Naechster technischer Arbeitsschritt:
Ein einziger gezielter Integrationstest fuer diese
beiden verbleibenden Anforderungen, erst nach
ausdruecklicher Freigabe.

Keine erneute Serie redundanter Offline-Tests.

BORG_FULL_LOCK_DURATION_PROOF=OPEN
PRODUCTIVE_END_TO_END_PROOF=OPEN
PRODUCTIVE_DAYCLOSE_RELEASE=BLOCKED


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

### Wiederaufnahme

Für die Bedienung die beiden aktuellen Skripte verwenden, jeweils nur im
konkret freigegebenen Umfang. Den erfolgreichen E2E-Lauf nicht wiederholen,
nur um ältere OPEN-Ausgaben zu beseitigen. Als getrennte Workflow-Folgearbeit
bleibt der Konkurrenztest in TODO. Restore-Abnahme, Offsite-Schlüsselsicherung
und R6-Restore bleiben offen. DDR/SPL und vollständiger externer Boot bleiben
unverändert offen; I2C3/ES8328 bleibt abgeschlossen.


## 2026-10-17 – NOVENA-WORKFLOW-FLOCK-02: Konkurrenztest PASS

**NOVENA-WORKFLOW-FLOCK-02: PASS.** Der zuvor offene Konkurrenztest
ist für den konkreten Fall „zweiter Aufruf von `daily-close.bash` bei bereits
gehaltener flock-Sperre“ qualifiziert abgeschlossen: sofortige Abweisung
mit Exit-Code 40; die Backup-Phase wurde nicht erreicht.
Ergebniswerte, Git-Checkpoint, Originalprotokoll und Evidenzherkunft sind
zentral im [TEST-LOG](TEST-LOG.md#2026-10-17--novena-workflow-flock-02-konkurrenztest-pass) dokumentiert.

Dieser Nachtrag ersetzt die frühere Offenbewertung dieses Konkurrenzfalls.
Die historischen E2E- und Fehlerbefunde bleiben erhalten. Eine vollständige
Qualifikation sämtlicher Workflow-Fehlerfälle oder produktive Gesamtfreigabe
folgt daraus nicht; weitergehende Fehlerfälle und Restore-Nachweise bleiben
getrennte offene Aufgaben.

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

## Nachtrag 2026-10-18 – Aktuelle Entwicklungspriorität

Kennung: NOVENA-ENTSCHEIDUNG-2026-10-18-DEBIAN-FIRST

Die weitere Entwicklung konzentriert sich zunächst ausschließlich
auf ein bootfähiges historisches Debian Jessie 8.2 auf der Kosagi Novena.

Der vorhandene Rust-Assembler unter
tools/novena-image-assembler/ ist bereits in
image/debian-reference-image.nix integriert.

Git-Checkpoint nach der Rust-Integration:
e491651f574d20c1905194d12b37d759c500ccb7

GitHub und Codeberg wurden auf diesen Stand synchronisiert.
NOVENA-BORG-BACKUP-08B meldete BORGMATIC_CREATE=PASS.
Ein vollständiger Restore-Test ist damit nicht nachgewiesen.

Nächster technischer Meilenstein:
Ein mit dem Rust-Assembler erzeugtes historisches Debian-Jessie-8.2-SD-Image
soll auf der Novena nachweislich booten.

Vor einem Build sind die noch offenen konkreten Bootvoraussetzungen
des historischen Debian-Referenzprofils zu klären.
Bereits abgeschlossene SPL-Untersuchungen werden ohne neue Evidenz
nicht wiederholt.

Das aktuelle NixOS-Image bleibt unverändert.
Insbesondere wird image/novena-image.nix nicht auf Rust umgestellt.
Die Erweiterung um ein NixOS-Profil ist bis zum erfolgreichen
Debian-Meilenstein zurückgestellt.

NOVENA-UNIVERSAL-RUST-01 ist ein vorbereiteter, aber nicht
freigegebener Implementierungsauftrag.

Keine automatische Freigabe für Builds, Hardwaretests,
SD-Schreibzugriffe oder weitere Codeänderungen.

## Jessie-Erfolg vom 18.10.2026

Historischer Jessie-SD-Boot mit grafischer Anmeldung
und Abmeldung erfolgreich. Xorg funktioniert mit
deaktivierter GPU-Beschleunigung.

Details:
docs/NOVENA-JESSIE-BOOT-20261018.md


## NOVENA-NIXOS-82: Hardware-Boot 2026-10-18

**Ergebnis:** Erster vollständiger NixOS-Boot mit dem
experimentellen historischen U-Boot 2015.04 auf echter
Kosagi-Novena-Hardware nachgewiesen.

**Nachweise:**
- NOVENA-NIXOS-76: historisches SD-Image gebaut.
- NOVENA-NIXOS-77: Image-Struktur geprüft.
- NOVENA-NIXOS-78: Bootvorbereitung.
- NOVENA-NIXOS-79: Datenträgerinventur.
- NOVENA-NIXOS-80: Image auf 64-GB-Test-microSD geschrieben;
  bytegenauer SHA256-Rücklesevergleich PASS.
- NOVENA-NIXOS-81: vollständiges serielles Bootprotokoll,
  Login und laufendes NixOS nachgewiesen.

**Technischer Ist-Zustand:**
- SPL und U-Boot proper: historischer Stand 2015.04,
  Commitkennung `bd5c7d3`.
- SPL-Speichertest: PASS.
- U-Boot erkennt 3,8 GiB DRAM und das Chimei-FHD-Panel.
- Kernel, initrd und Device Tree werden von der Test-SD geladen.
- Linux 6.18.49, ARMv7l, vier CPU-Kerne.
- NixOS 26.05 (Yarara).
- Root-Dateisystem `/dev/mmcblk1p2`, ext4, `rw`.
- Login über die serielle Konsole erfolgreich.
- Login auf dem Novena-Display sichtbar.
- `systemctl is-system-running`: `running`.

**Abgrenzungen:**
- Der vollständige Boot mit dem aktuellen U-Boot 2026.07
  ist durch diesen Test nicht nachgewiesen.
- Die beiden Kernel-Patches waren beim erfolgreichen Boot aktiv;
  ein ungepatchtes Vergleichsimage ist derzeit nicht erforderlich.
- Der sichtbare Display-Login beweist noch keine vollständig
  erfolgreiche IT6251-Initialisierung unter Linux.
- Im Bootprotokoll stehen IT6251-I²C-Schreibfehler (`-6`)
  und `coda 2040000.vpu: firmware request failed`.
  Diese Beobachtungen bleiben für spätere Hardwarequalifikation
  offen; sie widerlegen den erfolgreichen Systemboot nicht.
- Keine vollständige Qualifikation aller Hardwarefunktionen.
- Die Test-microSD ist der aktuelle funktionierende
  historische Boot-Referenzstand.
- Die ursprüngliche Debian-Jessie-SD und die SSD
  bleiben als getrennte Systeme geschützt.

**Originalprotokoll:**
`project-block-logs/novena/2026/10/NOVENA-NIXOS-81-20261018-141155-1rgZqQi3.txt`

**Image-SHA256:**
`4748322366a17dcfcafa75d4591f60ebc34629d9e4973807ab81a7794117170c`

**Nächste Schritte:** Boot-Referenzstand erhalten,
Dokumentations-Checkpoint synchronisieren und anschließend
die Hardwarequalifikation gezielt planen.
Keine Wiederholung des erfolgreichen Boots ohne Anlass.


## NOVENA-NIXOS-86: Displaynachweis 2026-10-18

**Status:** Interner Displaypfad unter NixOS 26.05
mit Linux 6.18.49 funktional bestätigt.

**Hardware- und Treiberbefunde:**
- Historischer U-Boot-Bootpfad 2015.04.
- Linux-Displaytreiber: `imx-drm`.
- DRM-Connector `card1-LVDS-1`: `connected`, `enabled`.
- Erkannter Displaymodus: 1920x1080.
- IT6251 auf I2C-2, Adresse 0x5c, Treiber gebunden.
- IT6251-Gerätekennung: Vendor `ca15`, Device `6251`.
- DRM-Bridge erfolgreich registriert und eingebunden.
- Link-Training nach vier Iterationen beendet.
- Kernelmeldung: `display link stable`.
- Kernelmeldung: `bridge_enable: exit success`.
- Framebuffer: `imx-drmdrmfb`, 1920x1080,
  16 Bit pro Pixel.
- Login auf dem internen Display sichtbar.

**Bekannte Initialisierungswarnungen:**
- `error -6 writing to eDP addr 0x5`
- `error -6 writing to LVDS addr 0x5`

Trotz dieser Meldungen setzt der IT6251-Treiber
seine Initialisierung fort und meldet einen
stabilen Display-Link bei 1920x1080.

**Bewertung:**
Die Linux-seitige Displayinitialisierung ist
durch DRM-Status und Kernelmeldungen belegt.
Der Displaypfad gilt für diesen getesteten
Boot als funktional bestätigt.

Die beiden I2C-Fehler bleiben dokumentiert.
Sie rechtfertigen ohne neue Fehlerbeobachtung
keine Wiederaufnahme der bereits abgeschlossenen
I2C3/ES8328-Root-Cause-Untersuchung.

Die Etnaviv-GPU wurde erkannt, einschließlich
`/dev/dri/renderD128`. Eine tatsächliche
GPU-Beschleunigung wurde nicht getestet.

Nicht nachgewiesen sind langfristige
Displayzuverlässigkeit und sämtliche
Grafikfunktionen.

**Originalprotokoll:**
`project-block-logs/novena/2026/10/NOVENA-NIXOS-85-20261018-143540-KIhZVEHW.txt`

**Fortsetzung:** Displaydiagnose abgeschlossen.
Weitere Hardwarefunktionen nach Bedarf einzeln
qualifizieren; funktionierenden SD-Referenzstand
unverändert erhalten.


## 2026-10-18 – SD-Wiederherstellung und Systemstatus

Maßgeblicher NixOS-Folgestand zu NOVENA-NIXOS-82/86, nach den vom Nutzer
bestätigten Angaben vom 18.10.2026. [TEST-LOG mit Imageidentität, öffentlichen
Fingerabdrücken und Evidenzgrenzen](TEST-LOG.md#2026-10-18--sd-wiederherstellung-und-systemstatus).
Dieser Dokumentationsauftrag hat keine erneuten Hardware- oder SSH-Prüfungen ausgeführt.

- **Zwischenfall:** 64-GB-microSD im laufenden Betrieb versehentlich mechanisch
  ausgeworfen; Kartenentfernung, I/O-Fehler, Journal-Abbruch und beschädigtes
  Root-Dateisystem gemeldet. Lesender `e2fsck` auf foobox zeigte Inkonsistenzen.
  Kein nachgewiesener MMC-Treiberfehler. Nutzerentscheidung: Wiederherstellung
  aus dem Referenzimage statt Reparatur.
- **NOVENA-NIXOS-99C: PASS.** SD vollständig mit dem unveränderten Referenzimage
  überschrieben und gesamten geschriebenen Imagebereich per Rücklesen/SHA256
  verifiziert. Referenz: Symlink
  `/home/loomit/novena-backups/build-results/novena-nixos-26.05-historical-2015.img`,
  **2581291008 Bytes**, SHA256
  `4748322366a17dcfcafa75d4591f60ebc34629d9e4973807ab81a7794117170c`.
  `/dev/sdb` war die damalige Identifikation auf foobox, keine dauerhafte Gerätekennung.
- **Boot und Zugang:** Anschließender Boot und serieller Login erfolgreich;
  `end0`: `192.168.11.12/24`. ED25519-Hostfingerabdruck unabhängig seriell
  verifiziert, danach `known_hosts` auf foobox aktualisiert. Fehlende
  `/root/.ssh/authorized_keys` durch erneute Autorisierung des öffentlichen
  foobox-Schlüssels ergänzt; deklarative Dauerhaftigkeit bleibt offen.
- **NOVENA-NIXOS-102: PASS.** SSH erfolgreich, NixOS 26.05 Yarara,
  Linux 6.18.49 armv7l, Systemstatus `running`, Root `/dev/mmcblk1p2` ext4 rw.
  Keine fehlgeschlagenen systemd-Dienste, keine Treffer der gezielten
  Dateisystem-/I/O-Fehlersuche seit Neustart.
  Root: **2.2G** gesamt, **1.6G** belegt, **472M** verfügbar, **78 %** Belegung.

**Nächste offene Arbeiten:** SSH-Autorisierung deklarativ in der NixOS-Konfiguration
sicherstellen; Root-Partition für größere SD-Karten beziehungsweise dynamische
Erweiterung prüfen. Laufenden ALSA-Paketbuild NOVENA-NIXOS-91B später separat
auswerten; Abschluss und Ergebnis bleiben hier offen. Kein Audiotest durchgeführt.

Bestätigt ist der historische U-Boot-2015.04-Bootpfad; modernes U-Boot 2026.07
bleibt ohne bestätigten erfolgreichen Boot. Abgeschlossene Display- und
I2C3-/ES8328-Untersuchungen bleiben geschlossen; `regulator-always-on` bleibt
Boardanforderung. Dieser Stand erteilt keine Freigabe für weitere Builds,
SSH-Verbindungen, Medienzugriffe, Hardwaretests oder Git-Mutationen.


## 2026-10-18 – NIXOS-FIRST: maßgebliche Wiederaufnahmepriorität

Kennung: **NOVENA-ENTSCHEIDUNG-2026-10-18-NIXOS-FIRST**;
[Beschluss und Evidenzgrenzen](DECISIONS.md#2026-10-18--universelles-nixos-sd-image-zuerst).
Das universelle NixOS-SD-Image steht wieder an erster Stelle.
Die früheren DEBIAN-FIRST-/JESSIE-FIRST-Vorgaben sind als Reihenfolge abgelöst.

Jessie 8.2 bootete erfolgreich von SD (NOVENA-JESSIE-48), einschließlich
dokumentierter grafischer Anmeldung und Abmeldung im
[Jessie-Abschluss](NOVENA-JESSIE-BOOT-20261018.md). NixOS 26.05 mit Kernel
6.18.49 bootete erfolgreich von externer SD mit historischem SPL/U-Boot
2015.04 (NOVENA-NIXOS-81/82). Die neueren System- und Displaynachweise in
dieser Übergabe bleiben erhalten. Jessie bleibt funktionierende historische
Referenz; Rust-Assembler und sämtliche Jessie-Arbeiten bleiben erhalten.

Für die Fortsetzung zuerst die priorisierten Aufgaben im
[TODO-Nachtrag](../TODO.md#2026-10-18--nixos-first-maßgebliche-aufgabenabgrenzung)
und die vorhandenen Artefakt-/Bootnachweise auswerten. Nächster Planungsgegenstand
ist die weitere Qualifikation des universellen NixOS-Images auf Basis des
funktionierenden historischen Bootpfads. Ein neuer Build oder eine
Rust-Umstellung ist damit nicht beauftragt. Ein neu erzeugtes Rust-Image
benötigt eigene Qualifikation; der bestehende Hardwareboot ersetzt sie nicht.

Moderne DDR-/SPL-Fragen und U-Boot 2026.07 bleiben separat offen.
Keine erneuten abgeschlossenen Jessie-/BOOT-05–08-, Display- oder
I2C3-/ES8328-Untersuchungen ohne neue diskriminierende Evidenz.
`regulator-always-on` bleibt Boardanforderung. Weitere Builds,
Nix-Evaluierungen, SSH-/Netzwerkzugriffe, SD-Schreibzugriffe, Hardwaretests
und Git-Mutationen benötigen eigene konkrete Freigaben.

NOVENA-DOC-NIXOS-FIRST-01 ändert ausschließlich vier Dokumente.
Die Aussagen beruhen auf bestätigten Auftragsfakten und vorhandener
Dokumentation; keine erneute Prüfung der extern referenzierten Originallogs.

## Tagesabschluss 2026-10-19 – historisches NixOS-Image

### Bestätigtes Image

Pfad:
`/nix/store/wzkx9cn7la3qpwqxp3lvjhlpjgsam24h-nixos-novena-sd-image.img`

SHA256:
`1c854d6a148bd212aca5ebe7e3705c84bc4f6b0e5ff62c1f4a211cf5b0fd3406`

Größe: 2.588.422.144 Byte.

Konfiguration:
- NixOS 26.05
- Kernel 6.18.49
- Historischer U-Boot 2015
- alsa-utils 1.2.14
- boot.growPartition = true
- Root autoResize = true

### Verifizierte Ergebnisse

- HISTORICAL-ALSA-GROW-03: PASS, Image gebaut.
- HISTORICAL-ALSA-GROW-04: PASS, Bootdateien verglichen.
- HISTORICAL-ALSA-GROW-05: FAIL, Locale-Fehler bei comm.
- HISTORICAL-ALSA-GROW-05R: PASS, korrigierter Vergleich.

SPL, U-Boot proper, zImage und novena.dtb sind
byteidentisch mit dem am 18.10. erfolgreich gestarteten Image.

boot.cmd und boot.scr unterscheiden sich nach Normalisierung
ausschließlich im NixOS-Systempfad.

Beide initrd-Archive enthalten jeweils 1.998 Dateieinträge.
Geänderte Store-Objekte betreffen insbesondere initrd-units,
initrd-fstab, initrd-system.conf und initrd-parse-etc.service.
Die vollständige inhaltliche Gleichwertigkeit der neu
adressierten Store-Objekte ist nicht abschließend nachgewiesen.

### Offene Hardwarequalifikation

Das neue Image wurde noch nicht auf SD geschrieben.
Ein Hardwareboot mit diesem Image steht aus.
ALSA und Grow sind noch nicht auf der Novena geprüft.

Der nächste Schritt ist die eindeutige Identifikation
der Test-SD auf foobox.

Keine weiteren Builds oder Hardwaretests ohne gesonderte Freigabe.

## 2026-10-20 – VM102-Build und Reproduzierbarkeitsstatus

### Drei nachgewiesene Images auf foobox

| Stand | U-Boot | SHA256-Anfang | Hardwarestatus |
| --- | --- | --- | --- |
| Referenz 18.10. | 2015.04 | `47483223` | Boot PASS |
| Älteres Testimage | 2026.07 | `937776db` | Vollständiger Boot nicht nachgewiesen |
| ALSA/Grow 19.10. | 2015.04 | `1c854d6a` | Noch offen |

Die drei Image-Dateien wurden in NOVENA-VM-REPRO-03
auf foobox als vorhanden bestätigt.

### VM102

Der unabhängige ARMv7-NixOS-Image-Build endete am
20.10.2026 um 03:22:49 MESZ mit BUILD_EXIT_STATUS=0.

Image:
`/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img`

Größe: 2.588.422.144 Byte

SHA256:
`b1ae43b3ce973377faa92faa5d466a94f3cda4357dacca26e1390a0e67cc7ad9`

Buildquelle: `/home/loomit/novena-cross` (kein Git-Repository).
Sieben geprüfte Quelldateien sind zwischen VM102 und
foobox SHA256-identisch, einschließlich `flake.lock`,
Image-Modul und beider U-Boot-Definitionen.

Die statische Modulkonfiguration setzt den Bootloader-Default
auf `current-2026`. Der VM-Build enthielt ALSA/Grow.
Die vollständige Identität aller Build-Eingaben und eine
bitidentische Reproduktion sind nicht nachgewiesen.

Das VM-Image unterscheidet sich in Größe und SHA256 vom
älteren modernen Testimage; der Entwicklungsstand ist
nicht gleichgesetzt. Es unterscheidet sich auch vom
historischen ALSA/Grow-Image.

### Prüfprotokolle

- NOVENA-VM-REPRO-01: PASS, Build- und Imageidentität.
- NOVENA-VM-REPRO-02: PASS, tatsächliches VM-Quellverzeichnis.
- NOVENA-VM-REPRO-03: PASS, sieben Quelldateien identisch;
  drei Images auf foobox bestätigt.
- NOVENA-VM-REPRO-04: FAIL, `python3` auf VM102 nicht
  verfügbar. Lokales Image analysiert, VM-Teil nicht ausgeführt.
  Kein Hinweis auf einen Build- oder Imagefehler.

### Entscheidung und Fortsetzung

Kein weiterer Build zur Reproduktion alter Entwicklungsstände.
Beim nächsten neuen Image werden foobox und VM102 mit
exakt derselben Konfiguration gebaut; danach Vergleich der
vollständigen Image-Prüfsummen.

Nächster Hardwaretest: ALSA/Grow-Image vom 19.10. mit
historischem U-Boot 2015.04. Vor jedem SD-Schreibzugriff
Datenträger eindeutig identifizieren und gesonderte
Freigabe einholen. Serielle Bootaufzeichnung und ALSA/Grow-
Qualifikation stehen aus.

## Nachtrag 2026-10-20 – Abschluss des unabhängigen Doppelbuilds

<!-- NOVENA-DOC-DUAL-BUILD-2026-10-20 -->

Die zuvor dokumentierten laufenden Builds sind abgeschlossen.
Beide vollständigen ARMv7-NixOS-SD-Image-Builds waren erfolgreich.

Buildhosts:
- foobox: Nix 2.35.2
- VM102: Nix 2.31.2

Identischer Store-Ausgabepfad:
`/nix/store/cd8kj8nr539kkv7xl5zmvz0xss4n95gf-nixos-novena-sd-image.img`

Imagegröße jeweils: 2588422144 Bytes.

Image-SHA256:
- foobox: 5e4413cba9d284041fe3b1ca42e403e46d1cc53abdadc6c9346c62214a8d961c
- VM102: 67f7899000584867467c8740e39df46ef67d3320fd06ca69d0607b01c8af528c

MBR, erste 8 MiB und vollständige FAT32-Bootpartition
sind byteidentisch.

Root-Dateivergleich:
- foobox: 155047 Einträge
- VM102: 155045 Einträge
- Zwei zusätzliche Boost-Shared-Library-Pfade auf foobox
- Unterschiedliche gemeinsame Dateien: Coreutils-ELF
  und `/nix-path-registration`

Coreutils- und Boost-Derivationen sind auf beiden Hosts
byteidentisch. Die jeweiligen Store-Ausgaben besitzen
unterschiedliche NAR-Hashes, sind jedoch jeweils gültig.
Die technische Ursache der Nichtreproduzierbarkeit bleibt
offen und wird nicht ohne neuen Auftrag untersucht.

Verbindlicher Abschlussbericht:
docs/NOVENA-DUAL-REPRO-2026-10-20.md

Kernelpatch 0007 und seine Einbindung in hardware/novena.nix
gehören zum untersuchten Image-Stand. Der historische
U-Boot 2015 wurde bereits auf der echten Novena getestet.
Die vollständige Bootqualifikation des neuen NixOS-Images
steht noch aus.

Keine neuen Builds, SD-Schreibzugriffe oder Hardwaretests
ohne ausdrückliche Freigabe.

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

## 2026-10-21 – USB-Übergabe nach Abschluss 208–213

NOVENA-USB-DOC-CODEX-215-R1: USB-Untersuchung vorläufig abgeschlossen,
Ursache offen. Geräteenumeration unter Linux 6.18.49 trotz Over-current-
Meldung; weder elektrischer Überstrom noch fehlerhafte Signalauswertung
bewiesen. Historische Dienstaktivierung ist kein erfolgreicher Bootlauf;
u3 nicht sicher einem Linux-Pfad zugeordnet. Manpage-/README-Widerspruch
und begrenzten DT-Vergleich beachten.

[Maßgeblicher USB-Abschlussbericht](NOVENA-USB-2026-10-21.md) trennt lokale
Quellen von auftragsbestätigten Ergebnissen und externen Archivbelegen.
Keine Port-Resets, keine USB-Stromversorgungsänderung, kein unbegründetes
`disable-over-current`. Wiederaufnahme nur mit neuer unterscheidungskräftiger
Evidenz; keine neue Aktionsfreigabe. Fehlende CODA/VPU-Firmware separat offen.
Die übrigen datierten Projektstände und Arbeitsstränge bleiben maßgeblich.

<!-- NOVENA-DAYCLOSE-20261021-239 -->
## 2026-10-21 – Image 2 und Übergabe für den 22.10.

- Image 1: Kernelpatch 0007 und U-Boot 2026.07.
  Realer Novena-Test durchgeführt.
  Manueller Start von `boot.scr` auf `mmc 1:1` erfolgreich;
  Linux und NixOS erreichten die serielle Login-Anzeige.
  Dies ist kein Nachweis für automatischen Boot.
- U-Boot-Diagnose Block 190: Die automatische
  Partitionssuche berücksichtigt MBR-Bootmarkierungen.
- Block 193: Korrektur im Image-Builder:
  FAT32/FIRMWARE (p1) bootable, ext4/NIXOS_SD (p2)
  nicht bootable.
- Image 2: vollständiger foobox-Build Block 195 PASS,
  Git-Quellstand `6fce37e5498fcbc1c3bf78685dd5baf7d900f346`.
- Image 2:
  `/nix/store/n7bm1lbqbhxk62h1n45k0xwis9nzk091-nixos-novena-sd-image.img`
- Imagegröße: `2588422144` Bytes.
- SHA256:
  `144d71c37536d09e5e47d41eb270fa131bdd154244fdd2d21a37a4b0d958777d`
- Block 237: Image vorhanden, Größe und SHA256
  stimmen, MBR-Signatur und Partitionseinträge PASS.
  p1 Bootflag 0x80, p2 Bootflag 0x00.
- Block 238: Tagesabschluss-Vorprüfung PASS;
  Git-Worktree sauber, Workflow-Skripte vorhanden.
- Mesa-Commit `1cefe299` liegt nach dem Image-2-
  Quellstand. Mesa ist damit nicht als Bestandteil
  dieses Images nachgewiesen.
- Automatischer Hardwareboot von Image 2:
  NICHT GETESTET.
- Hardwaretest auf 2026-10-22 verschoben.
  Vorher SD-Zielgerät eindeutig bestimmen.
  Schreiben nur nach gesonderter Freigabe.
- Kein neuer Build und keine weiteren Patches
  vor diesem Hardwaretest.
- Borg: reguläres Daily-Close-Backup und separates
  VM103-Backup sind getrennt nachzuweisen.
  VM103 war zuvor mit Exitcode 105 fehlgeschlagen;
  kein erfolgreicher neuer Lauf wird hier behauptet.
- Der Dokumentationsabschluss ist nicht selbst
  der Nachweis einer Borg-Sicherung.

## Aktuelle Fortsetzung – 2026-10-22

Wir setzen das universelle Novena-NixOS-SD-Image-
Projekt fort.

Der letzte geprüfte Image-Kandidat ist Image 2 vom
21.10.2026 mit korrigierter MBR-Bootmarkierung.
Build 195 und Integritätsprüfung 237: PASS.

Image:
`/nix/store/n7bm1lbqbhxk62h1n45k0xwis9nzk091-nixos-novena-sd-image.img`

SHA256:
`144d71c37536d09e5e47d41eb270fa131bdd154244fdd2d21a37a4b0d958777d`

Image 1 wurde bereits real getestet. Manueller
Boot von `boot.scr` auf `mmc 1:1` erreichte NixOS.
Image 2 wurde noch nicht hardwareseitig getestet.

Nächster Auftrag:
1. Tagesabschluss- und Borg-Ergebnisse vom
   21.10. anhand der Originalprotokolle prüfen.
2. Aktuellen Git- und Dokumentationsstand prüfen.
3. SD-Zielgerät eindeutig identifizieren.
4. Nach ausdrücklicher Freigabe Image 2 schreiben.
5. Automatischen Boot über serielle Konsole testen.

Keine Builds, Patches, Git-Änderungen oder
Hardwarezugriffe ohne Freigabe.

<!-- NOVENA-DUAL-BUILD-20261022-CURRENT -->
## Nachtrag 2026-10-22 – VM102/VM104

Die vollständigen Novena-NixOS-SD-Image-Builds wurden
am 22.10.2026 gegen 19:40 MESZ auf VM102 und VM104
mit identischem Quellsnapshot gestartet.

Der ursprüngliche Symlink-Fehler wurde korrigiert.
Die Fehlversuche sind archiviert.

Letzter bestätigter Status um 19:42 MESZ:
Beide Builds liefen. Ein fertiges Vergleichsimage
war zu diesem Zeitpunkt nicht nachgewiesen.

Offen: Abschlussstatus, Exitcodes, Imagegrößen,
SHA256, Vergleich mit foobox und Provenienz.

Die Builds dürfen nicht erneut gestartet oder
ohne gesonderte Freigabe unterbrochen werden.

Backup: scripts/novena-backup.bash --all.
Der USB-Teil enthält bereits den Tagesabschluss.

<!-- NOVENA-HANDOVER-20261024-BOOT02 -->
## 2026-10-24 – Aktueller Novena-Kaltstart und Übergabe

Der externe SD-Bootpfad wurde erneut real getestet:
SPL 2026.07, U-Boot, Linux 6.18.49 und
NixOS 26.05 erreichten automatisch die serielle
Login-Anzeige.

Serielle Originalaufzeichnung auf foobox:
`~/novena-backups/serial-logs/NOVENA-SD-BOOT-20261024-RETRY.log`

SHA256:
`dea76b5a932efff2413ce5f7f6b8cacc7414d57824644944c8ced51570a18c70`

Der nachfolgende Block NOVENA-BOOT-VERIFY-02 bestätigte:
- SSH auf 192.168.11.11 erfolgreich
- Root-Dateisystem auf /dev/mmcblk1p2
- systemd running, null fehlgeschlagene Dienste
- Ethernet mit bekannter Geräte-MAC
- SATA-SSD und interne MMC erkannt
- DRM-Geräte und LVDS-Anschluss vorhanden
- ES8328-Audiogerät registriert
- rtc0/SNVS lesbar
- rtc1/PCF8523 weiterhin Invalid argument

GPU-Beschleunigung wurde durch diese Prüfung nicht
funktional nachgewiesen.

Bekannte IT6251-, Audio-, PHY- und USB-Meldungen
sind kein neuer Nachweis einer Root Cause.
Abgeschlossene I2C3-/ES8328-Untersuchungen
nicht ohne neue Evidenz wiederholen.

Die NXP-Lizenzfrage zur proprietären VPU-Firmware
ist offen. Firmware nicht öffentlich verteilen.
Keine detaillierten neuen Firmware-Nutzungsberichte
öffentlich veröffentlichen, solange die Rechte
ungeklärt sind.

Wichtig: Das erfolgreich gestartete Bestands-SD-System
ist von neueren, separat gebauten Image-Artefakten
zu unterscheiden. Der Kaltstart qualifiziert nicht
automatisch die VM102-/VM104-Vergleichsimages.

Naechste Prioritaeten:
1. PCF8523-RTC anhand vorhandener Logs, Device Tree
   und Kernelkonfiguration gezielt untersuchen.
2. Firmware-Persistenz und optionalen CODA-Ladepfad
   ohne Firmwareweitergabe statisch einordnen.
3. Grafikbeschleunigung später getrennt qualifizieren.
4. Sichere Ersteinrichtung des öffentlichen Images
   planen (Root-Passwort und SSH).
5. Bildreproduzierbarkeit und Hostvergleiche getrennt
   anhand der vorhandenen Belege weiterführen.

Keine neuen Builds, Reboots, Installationen oder
Hardwaretests ohne gesonderte Freigabe.

<!-- NOVENA-20261024-RTC-SD-STATUS -->
## Nachtrag 2026-10-24 – NixOS-SD-Boot und RTC

Autonomer externer NixOS-SD-Boot erfolgreich.
NOVENA-BOOT-VERIFY-02: PASS.
Kernel 6.18.49; Root /dev/mmcblk1p2.

PCF8523 rtc1:
RTC_VL_DATA_INVALID und RTC_VL_BACKUP_LOW.

Das originale PVT2-Layout bestaetigt
BT11P (BR1225A/FAN) und U10P.
Der elektrische Netzpfad bleibt
aufgrund unvollstaendiger Parserdaten offen.

Die statische RTC-Untersuchung
ist vorlaeufig abgeschlossen.

CODA960:
Proprietaere Firmware nicht fuer die
oeffentliche Imageverteilung freigegeben.
NXP-Lizenzklaerung steht aus.

RTC-Originalprotokolle:
NOVENA-RTC-VL-02,
NOVENA-RTC-PCB-01/02,
NOVENA-RTC-NET-01/02/03.

Die Novena wird aktuell nicht benoetigt.


## Nachtrag 2026-10-24 – VM102/VM104-Image-Reproduzierbarkeit

Auf VM102 und VM104 wurde ein vollständiges
Novena-NixOS-SD-Image byteidentisch nachgewiesen.

Image:
`/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img`

SHA256:
`40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a`

Die Image-Derivation und ihr SHA256 stimmen ebenfalls überein.

Primärbelege: NOVENA-IMAGE-COHORT-54,
NOVENA-IMAGE-THREEWAY-56 und
NOVENA-VM-IMAGE-INVENTORY-66.

Weitere Builds sind für die erneute Bestätigung dieses
Referenzimages nicht erforderlich.

Die vollständige Unabhängigkeit aller Abhängigkeits-Builds
bleibt gesondert zu qualifizieren.

foobox-Abweichungen werden getrennt behandelt.

Ausführliche Dokumentation:
`docs/VM102-VM104-REPRODUCIBILITY-2026-10-24.md`

## Nachtrag 2026-10-24 – Display-Untersuchung abgeschlossen

Statische Offline-Analyse der historischen
Backlight-/IT6251-/U-Boot-Übergabe qualifiziert
abgeschlossen. Die Ursache der Dunkelphase
ist weiterhin nicht bewiesen.

Maßgeblicher Ergebnisstand:
`docs/DISPLAY-HANDOVER-2026-10-24.md`.

Nächster Schwerpunkt: NixOS-SD-Image,
bestehende Qualifikation und Release-Anforderungen.
Keine neuen Builds oder Hardwaretests freigegeben.

## Nachtrag 2026-10-25 – U-Boot 2026 / Display

NOVENA-DOC-CONSOLIDATE-90

Neuer maßgeblicher Ergebnisbericht:
docs/UBOOT26-DISPLAY-QUALIFICATION-2026-10-25.md

Basis-HEAD: 262c2d52dbb91cef8f77dacb309d5abd2862751c

VM102 und VM104: vollständige U-Boot-2026-Images
mit identischer SHA256 c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750.

foobox: getrennte Displayvariante mit SHA256
99c34a52484e3d43598fe6487c97d76fb5bdafc58efb934769b6923f7863d5a9.

Der kompilierte DTB enthält genau die beiden
vorgesehenen neuen Regulator-Always-on-Eigenschaften.

SPL-Bereich, U-Boot proper und zImage sind
zwischen Baseline und Displayvariante identisch.

Die acht zusätzlichen Bytes der boot.scr-Payload
sind als regulärer Skriptgrößenpräfix erklärt.

Prüfblöcke 88/89 bleiben aufgrund ihrer
Python-Ausgabefehler formal FAIL.
Ihre dokumentierten Teilbefunde bleiben gültig.

Noch kein aktueller Hardwareboot und keine
funktionale Bestätigung des Display-Fixes.

Dokumentationsänderungen sind zunächst nur lokal.
Commit/Push sind nicht Bestandteil dieses Blocks.

## Nachtrag 2026-10-25 – SD-Hardwareboot und Boot-Check

Blöcke 104C–109 belegen den vollständigen SD-Boot
mit U-Boot 2026.07 und NixOS 26.05 (Kernel 6.18.49).
Block 108 bestätigt Root-Grow auf etwa 59 GiB und
den stabilen IT6251-Displaylink mit 1920x1080.
Die U-Boot-Bildausgabe war im neuesten Versuch
nicht sichtbar, obwohl sie zuvor beobachtet wurde.
Die Ursache bleibt offen; kein Display-Fix bestätigt.
Die Novena wurde nach Abschluss ausgeschaltet.

Neuer Entwicklungsstand:
- scripts/novena-boot-check.bash: angelegt, noch nicht auf Hardware getestet.
- modules/novena-development-access.nix: vorbereitet, nicht aktiviert.
- scripts/novena-release-check.bash: konservativer Release-Preflight.
- docs/NOVENA-BOOT-CHECK.md: Konzept und Sicherheitsgrenzen.
Die aktuelle Root-Passwortkonfiguration blockiert eine Releasefreigabe.
Noch offen: dedizierten foobox-Public-Key festlegen, Modul qualifizieren,
maschinenlesbare Historie spezifizieren und Gesamtimage prüfen.

Diese Umsetzung enthält keinen Build und keinen neuen Hardwaretest.
