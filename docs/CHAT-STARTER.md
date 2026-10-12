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
