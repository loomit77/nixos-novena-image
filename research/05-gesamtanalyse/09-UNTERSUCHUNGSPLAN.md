Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Untersuchungsplan und Bearbeitungsstand

## In diesem Block durchgeführt

AGENTS.md vollständig gelesen; Ausgangs-Git und Arbeitskopie lokal geprüft, vorhandene Änderungen erhalten. Forschungsziel vorher nicht vorhanden. Quellenverzeichnis vollständig gelesen. Recherche-/Evidenzdateien inventarisiert, manifestierte Hashes geprüft. Aktueller erhaltenener REF-Quellpfad und relevante historische SPD-/DDR-/Bootpfade verglichen. Vorhandene P_EXT-Seriellogs vollständig ausgewertet; weitere Kernel-/Buildlogs im Register erfasst. Neue deutsche Berichte und Register angelegt. Keine funktionalen Änderungen und keine neuen Testergebnisse.

## Noch nicht abgeschlossene fachliche Arbeiten

1. Vollständige zeilenweise Begutachtung aller historischen DDR-Helfer, Versionsdeltas und Herstellerregistersemantik. 04 ist ein gezielter Pfadvergleich, kein vollständiger Formalbeweis aller MMDC-Zweige.
2. Vollständige Zuordnung der Mainline-2015-Mail zu konkretem lokalem Commit und der festen Elpida-Konfiguration zu ihrer Einführungshistorie. 14 liefert xobs-Vergleichsobjekte, aber keinen lückenlosen Mergegraph.
3. Semantische Prüfung aller PDF-Seiten, Gerber-/Altium-Dateien, archivierten Bundles und HTML-Navigationslinks. 11 inventarisiert/hasht sie; das ist keine vollständige technische Inhaltsverifikation.
4. Aktuelle initrd-/Stage1-/Kernelconfig- und Device-Tree-Memoryfixup-Inventur am genau zugeordneten fertigen Image-/Storebestand. Keine neue Extraktion außerhalb des erlaubten Schreibbereichs und kein Mount/Build erfolgt.
5. Vollständiges RAMbudget einschließlich proper-Relocation, framebuffer/LMB, kernel expansion und aller vorgesehenen DIMMs.
6. Vollständige Fehlervertragsanalyse für Hush/fatload/bootz, PMIC-APIs und ext4-Builder-Shellumgebung.
7. Exakte Provenienzkette aller älteren Tests zu Rohlogs/Medienreadbacks. Fehlende Originale nicht durch narrative Berichte ersetzen.
8. Redaktionelle Normalisierung des großen URLinventars: Originalquelle, Mirror, Navigation und fachliche Relevanz einzeln bestimmen.
9. Unabhängige Identitätsprüfung weiterer lokaler v2020.07-/v2026.07-Bäume. Bisher nur relevanter REF und ausgewählte Git-Bäume qualifiziert.

Diese offenen Arbeiten sind keine neuen Hardwaretestaufträge. Der Umfang großer Quellbäume und fehlende Hersteller-/Originalbelege verhindern einen ehrlichen Anspruch „alles technisch abschließend geprüft“. Die vorliegende Forschungsgrundlage bewahrt deshalb Befunde und verbleibende Prüfschritte.

## Priorisierte Fortsetzung ohne Wiederholung

**P1: Quellen schließen.** Fehlende Originalberichte/Gateausgaben lokal suchen, genaue Fundorte dokumentieren; Hersteller-MMDC-/Erratumbelege zuerst lokal suchen, dann nur nach gesonderter Netzwerkfreigabe recherchieren. Kein erneuter Hashlauf als Ersatz für neue Erkenntnis.

**P2: DDR-Versionsmatrix vervollständigen.** Aktueller REF, xobs rc5/rc12/bd5c7d3 und passender Mainline-Kalibrierungscommit mit effective values statt nur Initialwerten vergleichen. Konkretes DIMM/DDR-Clock/Rank dokumentieren. Bewertung ist statisch; SPD-Auslesen vom Board benötigt eigene Freigabe.

**P3: Diagnoseentwurf nur auf geklärter Grundlage.** Falls Rückgabe-/Timeoutbeobachtung neue diskriminierende Informationen liefern kann, schriftlich Messpunkte, Speichererhaltung, Seiteneffekte, Fallmatrix und Stopbedingungen entwerfen. A/B und optionaler Registerumfang getrennt entscheiden. Keine Implementierung aus diesem Plan ableiten.

**P4: Gesamten Restbootvertrag statisch qualifizieren.** MMC/FAT/Payload, proper-Environment, Bootscript, DTB, initrd, Rootlabel und Adress-/Platzbudgets verbinden. Dies darf parallel im fachlichen Sinn untersucht werden; es ersetzt nicht die frühe Root-Cause-Frage.

**P5: Release-/Recovery-Vertrag.** Unterstützte RAM-/Board-/Kartenvarianten und Mindestkapazität begrenzen; Update-/Rollback, Labelduplikate und Freiraum vor Veröffentlichung spezifizieren.

## Gates einer späteren autorisierten Implementierung

1. Quellenfrage und diskriminierende Hypothese benannt.
2. Gesonderte Implementierungsfreigabe und patchbasierte Provenienz.
3. Gesonderte Buildfreigabe; exakte Toolchain/Pin/Patchliste.
4. Statische Quellen-/ELF-/Symbol-/Instruktions-/Größenqualifikation; jede Layoutänderung neu prüfen.
5. Separate Medienfreigabe, stabile Identität, Sicherung, exakter Bereich, vollständiger Readback.
6. Separate Hardwarefreigabe mit exakter Versuchszahl, POR/Bootquelle, serieller Aufzeichnung vor Start.
7. Interpretation anhand vorab festgelegter Fälle; Fehlschlag ist keine Wiederholungsfreigabe.

Bis zu diesen gesonderten Aufträgen bleiben IMPLEMENTATION_ALLOWED, BUILD_AUTHORIZED, HARDWARE_TEST_AUTHORIZED und PATCH_0008_SECOND_BOOT_ALLOWED jeweils NO. Die funktionale ES8328/I2C3-Untersuchung bleibt geschlossen.
