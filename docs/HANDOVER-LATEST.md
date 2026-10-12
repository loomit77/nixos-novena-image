# Novena – verifizierte Projektübergabe

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
