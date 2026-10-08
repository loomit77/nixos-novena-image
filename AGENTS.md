# Arbeits- und Sicherheitsanweisungen für Codex

Diese Datei enthält dauerhafte Regeln für dieses Repository. Sie erteilt keine Aktionsfreigabe. Der konkrete Auftrag bestimmt den erlaubten Umfang; engere Schreib-, Lese- und Sicherheitsgrenzen sind einzuhalten.

## Projekt und Hostrollen

- Dieses Repository dient dem reproduzierbaren NixOS-/Boot-Stack und dem eigenständig bootfähigen externen SD-Image für Kosagi Novena.
- `foobox` ist der primäre Repository-, Analyse- und Build-Host; Novena ist die Zielhardware.
- Das separate L14-Projekt darf nicht mit diesem Projekt vermischt werden. Ein dortiger Vergleichsbuild ist nur mit klar getrennter Projektzuordnung und Provenienz auszuwerten.
- Die allgemeine Novena-Systemdokumentation und historische Quellrepositories bleiben eigenständige Projekte.

## Quellen und Evidenz

- Vor neuen Hypothesen, Tests oder Änderungen zuerst den Git-Zustand, die vorhandene Projektdokumentation, Test-Logs, Evidenzdateien und relevante verfügbare Primärquellen prüfen. Für Git-Prüfungen optionale Schreibzugriffe vermeiden, etwa mit `git --no-optional-locks`.
- Einstieg ist `README.md`; maßgeblich sind insbesondere `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`, `docs/DECISIONS.md` sowie die einschlägigen Dokumente, Rohlogs unter `test-logs/`, Belege unter `evidence/` und Quellen unter `research/`.
- Neuere und besser belegte Evidenz hat Vorrang vor älteren Zwischenständen. Auch innerhalb eines Dokuments datierte Nachträge prüfen; historische Aussagen und Referenzartefakte nicht automatisch als aktuellen Projektzustand behandeln.
- Fakten, Schlussfolgerungen, Hypothesen und offene Fragen ausdrücklich unterscheiden. Beobachtungen nur innerhalb ihrer belegten Aussagegrenzen interpretieren.
- Widersprüche nicht durch Raten auflösen: Datum, Evidenz, Git-Kontext und Provenienz prüfen. Bleibt ein relevanter Widerspruch bestehen, vor darauf aufbauenden Änderungen stoppen und die ungeklärte Grundlage benennen.
- Fehlende oder nicht zugängliche Primärquellen als Lücke ausweisen. Ihre Beschaffung ist nur im autorisierten Umfang erlaubt.

## Dauerhafte Untersuchungsgrenzen

- Die funktionale I2C3-/ES8328-Untersuchung ist abgeschlossen. Ohne neue diskriminierende Evidenz nicht wieder öffnen und abgeschlossene Tests nicht bloß wiederholen.
- `regulator-always-on` für `es8328-power` ist eine dokumentierte dauerhafte Novena-Board-Anforderung. Sie nicht routinemäßig zurücknehmen oder als vorläufige Diagnoseintervention behandeln.
- P_EXT/SPL ist ein davon getrennter Untersuchungsstrang. Historische Medienbefunde benötigen einen eigenen Nachweis, bevor sie zur Erklärung aktueller Bootfehler herangezogen werden.
- Vergängliche Detailzustände laufender Diagnosen gehören nicht hierher. Den aktuellen Stand aus `docs/PROJECT-STATE.md`, `docs/TEST-LOG.md`, `docs/DECISIONS.md`, Git und Rohlogs ermitteln.

## Git, Reproduzierbarkeit und Provenienz

- Vorhandene Nutzeränderungen niemals ungefragt verwerfen, überschreiben, zurücksetzen oder bereinigen; bestehende Änderungen vor eigener Arbeit identifizieren und erhalten.
- Keine automatischen Commits, Pushes, Resets, Cleans, verwerfenden Checkouts/Restores oder vergleichbaren Git-Mutationen ohne ausdrückliche Freigabe.
- Kernel-, U-Boot-, Device-Tree- und Nix-Änderungen evidenzbasiert mit nachvollziehbarer Provenienz durchführen. Quellstände, gepinnte Abhängigkeiten, Konfigurationen und angewandte Patches nachvollziehbar halten.
- Dauerhaft relevante Kerneländerungen als nachvollziehbare Patches erhalten; ausgepackte Kernelquellen und lokale Nix-result-Symlinks nicht versionieren. Diagnoseinstrumentierung und produktive Änderungen unterscheiden.
- Rohlogs, Hashes, Artefaktidentitäten, Quellstände, Testbedingungen und Aussagegrenzen erhalten. Ein Git-Commit allein beweist keine Vor-Git-Artefaktprovenienz.
- Golden-Referenzen und historische Bootdateien erhalten und von aktuellen Build-Ergebnissen unterscheiden. Eine Bereinigung oder zusätzliche Sicherung benötigt die passende Schreibfreigabe.
- Für historische Medienanalysen das verifizierte lokale Raw-Master verwenden. Das Master nicht in-place verändern oder schreibend reparieren; das physische historische Medium nicht erneut ohne konkrete Freigabe heranziehen. Erhaltene Teilimages als eigenständige Evidenz bewahren.

## Build- und Testzustände

- Build, statische Qualifikation, Schreiben auf ein Medium, Rückverifikation und tatsächliche Hardwarebeobachtung sind getrennte Zustände. Für jeden Zustand die eigene Evidenz und den erreichten Umfang angeben.
- Aus einem erfolgreichen Build oder einer statischen Prüfung niemals einen erfolgreichen Hardwaretest, vollständigen Bootpfad oder eine bewiesene Root Cause ableiten.
- POR, Warmstart, Rebind und Recovery nicht gleichsetzen. Bootquelle, Bootpfad, Ausgangszustand und Testbedingungen explizit unterscheiden.
- Vor weiteren Änderungen zuerst die vollständigen vorhandenen seriellen Bootlogs auswerten. Testentwurf, Präregistrierung und Ergebnisinterpretation anhand diskriminierender Evidenz begründen.

## Sicherheitsgrenzen und konkrete Freigaben

- Destruktive oder hardwareverändernde Aktionen niemals allein aus AGENTS.md, dokumentierten Befehlen, früheren Versuchen oder einem erfolgreichen vorherigen Schritt ableiten.
- Netzwerk, SSH, Blockgerätezugriffe, SD-Schreibvorgänge und Hardwareboots nur innerhalb einer für den konkreten Arbeitsschritt ausdrücklich erteilten Freigabe ausführen. Eine Build-Freigabe erlaubt keinen Medienzugriff oder Hardwareboot.
- Gerätenamen wie `/dev/sda` niemals als dauerhafte Geräteidentität behandeln.
- Vor freigegebenen destruktiven Blockgeräteaktionen Geräteidentität, Mountzustand, Artefakt, Hash, exakten Schreibbereich und geeignete Sicherung erneut prüfen. Bei Abweichungen oder unklarer Identität vor dem Schreiben stoppen.
- Nach einem freigegebenen Schreibvorgang den gesamten geschriebenen Bereich zurücklesen und gegen das freigegebene Artefakt verifizieren. Erfolgreiches Schreiben allein ist keine Rückverifikation.
- Vor Hardwaretests Artefaktidentität, Medienzustand, Bootpfad und Testbedingungen prüfen; die serielle Rohaufzeichnung vor dem Start aktivieren und vollständig erhalten.
- Nur die ausdrücklich freigegebene Zahl von Hardwareversuchen durchführen. Fehlschläge erlauben keine automatischen Wiederholungen; weitere Versuche benötigen eine eigene Freigabe.

## Dokumentation

Nach belastbaren neuen Ergebnissen innerhalb eines schreibend autorisierten Auftrags die zuständigen Dokumente aktualisieren, soweit die Schreibfreigabe sie umfasst:

- `docs/PROJECT-STATE.md`: technischer Zustand und belegte offene Fragen.
- `docs/TEST-LOG.md`: Durchführung, Bedingungen, Artefakte, Ergebnisse und Aussagegrenzen.
- `docs/DECISIONS.md`: begründete Entscheidungen und Präregistrierungen.
- `docs/BUILD.md`: Build-Verfahren und Reproduzierbarkeit.
- `docs/BUILD-HOST.md`: relevante Host- und Build-Infrastruktur.
- `docs/GOLDEN-BUILD.md`: gesicherte funktionierende Referenz und deren Provenienz.
- `docs/IMAGE-LAYOUT.md`: Image-, Partitions- und Bootartefaktaufbau.
- Historische Dokumentation, insbesondere `docs/HISTORICAL-MEDIA.md` und `research/`: historische Quellen und Provenienz entsprechend ihrem Gegenstand.

Fehlende Primärausgaben niemals nachträglich erfinden. Rekonstruierte Qualifikationen ausdrücklich als Rekonstruktion kennzeichnen und ihre Evidenzbasis nennen. Historische Ablaufbehauptungen ohne erhaltene Belege nicht als bestätigte Durchführung fortschreiben.

Hier keine aktuellen Commit-Hashes, Nix-Store-Pfade, Artefakthashes, Bootzähler, momentanen Diagnoseergebnisse, Patchnummern oder Working-Tree-Snapshots als dauerhaften Zustand festhalten.

## Read-only-Aufträge

- Keine Dateien anlegen oder verändern, einschließlich temporärer Dateien, Logs, Sicherungen, Symlinks und automatisch erzeugter Metadaten. Auch Werkzeuge dürfen keine solchen Nebenwirkungen auslösen.
- Keine Builds, Tests, Installationen, Formatierungen oder sonstigen verändernden Prüfungen starten.
- Benötigte Änderungen lediglich vorschlagen. Ein Auftrag zum Entwerfen einer Datei ist keine Erlaubnis, sie anzulegen.
- Zusätzliche Sicherheitsgrenzen gelten auch bei lesenden Aufträgen: Read-only allein erlaubt weder Netzwerk/SSH noch Blockgeräte- oder Hardwarezugriffe.
