Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Leseanleitung und Ausgangscheckpoint

Diese Forschungsgrundlage verbindet historische Projektentwicklung, erhaltene Rohdaten, aktuelle Arbeitskopie und historische Vergleichsquellen. Sie ist keine Root-Cause-Feststellung und keine Aktionsfreigabe.

## Ausgangszustand

BELEGT durch lokale lesende Git-Abfragen: Arbeitsverzeichnis `/home/loomit/nixos-novena-image`, Branch `main`, HEAD `0649b5523b2991d3283413bba835c7ee2f7be432`. Git-Abfragen verwenden `GIT_NO_LAZY_FETCH=1` und `git --no-optional-locks`. Der Forschungszielpfad existierte bei der ersten Prüfung nicht; `research/` war ein echtes Verzeichnis. Vorhandene Änderungen bleiben Eigentum des Ausgangszustands.

Ausgangsstatus: fünf hinzugefügte U-Boot-Diagnosepatches 0004–0008 und geändertes `boot/u-boot/default.nix`; unversioniert AGENTS.md, Quellenverzeichnis, Übersetzungsworkflow, sechs englische Dokumente sowie Glossar und Manifest. Exakte Porcelain-Ausgabe und SHA256-Dateicheckpoint: `15-AUSGANGSCHECKPOINT.json`. Hinzugefügte Patches erscheinen mit Status ` A`; das wird nicht als eigener Commit oder eigener Patchbeitrag interpretiert.

## Lesereihenfolge

1. 01 rekonstruiert die Geschichte und trennt alten internen Bootpfad von P_EXT.
2. 02 und 03 erklären Quellenidentitäten, Quellengrenzen und den Katalogaudit.
3. 04 verbindet SPL, U-Boot, Kernel, initrd und Rootfs.
4. 05 prüft die Testinterpretation; 06 enthält Selbstkritik und Zukunftsrisiken.
5. 07 sammelt begründete Hypothesen mit zehn Prüfpunkten.
6. 08 bereitet externe Recherche vor; 09 und 10 bewahren Fortsetzung und offene Fragen.
7. 11 enthält den dateigenauen Archiv- und Prüfsummenaudit, 12 das URL-Fundstelleninventar, 13 Rohlog- und Evidenzregister und 14 lokal erhobene historische Git-Belege. 16 dokumentiert die Abschlussprüfung.

## Belegklassen

- BELEGT: direkt am bezeichneten lokalen Text, Bytebestand oder Git-Objekt festgestellt. Bei einer Projektaussage heißt dies zunächst: Die Dokumentation berichtet es; Hardwaredurchführung braucht ihre eigene Evidenz.
- INDIZ: begrenzte Unterstützung durch mehrere Befunde, ohne vollständige Kausalitätskette.
- HYPOTHESE: prüfbare mögliche Erklärung oder zukünftiges Risiko.
- WIDERSPRUCH: tatsächlich unterschiedliche Aussagen mit gleichem behauptetem Geltungsbereich; unterschiedliche Versionen allein sind kein Widerspruch.
- QUELLENLUECKE: nötiger Originalbeleg fehlt oder wurde nicht lokal nachgewiesen.
- NICHT_GEPRUEFT: vorhandener Gegenstand wurde in diesem Block nicht ausreichend inhaltlich geprüft.

Quellenart ist davon unabhängig: ORIGINALQUELLE bezeichnet erhaltene unmittelbare Rohbelege bzw. zeitgenössischen Quellinhalt; LOKALE_KOPIE eine lokal archivierte Darstellung einer externen Originalquelle; REKONSTRUKTION eine abgeleitete Analyse; SEKUNDAERQUELLE eine Zusammenfassung ohne vollständige unmittelbare Belegkette; NICHT_VERIFIZIERT eine nicht unabhängig bestätigte Zuordnung. Ein Hash beweist lokale Identität, keine Herkunft oder technische Richtigkeit.

## Untersuchungstiefe und Grenzen

Das Quellenverzeichnis wurde vollständig gelesen. Alle 209 vorbestehenden Forschungsdateien wurden inventarisiert und byteweise für den Audit gelesen; Textinventar und Prüfsummenvergleich sind keine vollständige semantische Begutachtung jeder PDF-Seite, Gerber-Datei, Grafik oder Git-Historie. Dokumentation wurde übergreifend ausgewertet, die neuesten Checkpoints und relevanten Quellpfade vertieft. Alle erhaltenen repositorylokalen P_EXT-Seriellogs wurden vollständig gelesen; auch vollständige Kernel-Rohtexte der H3-1R-Serie wurden im Register auf die einschlägigen Signaturen geprüft.

Die externen lokalen historischen Git-Bäume sind vorhanden; ihre Identität wurde lesend bestimmt und relevante Dateien/Revisionen verglichen. Eine lückenlose Zeilenprüfung aller U-Boot-/Linux-Dateien und sämtlicher archivierten Binärformate ist weiterhin offen. 09 listet diese Arbeit ausdrücklich; die Existenz dieser Berichte bedeutet keine abgeschlossene Gesamtverifikation.

Neuere datierte Nachträge haben Vorrang, insbesondere `docs/PROJECT-STATE.md:5740`. Der 9C-43 benannte Bericht entstand erst in 9C-44 und ersetzt kein Originalprotokoll früherer Blöcke (`research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md:7`).

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
```
