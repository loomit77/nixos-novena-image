Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# U-Boot: Originalcode, Review und fehlende Übergangsstände

Quellenregister: [05](05-PROVENIENZMATRIX.md). Vorhandene lokale Gitobjekte wurden ausschließlich mit `GIT_NO_LAZY_FETCH=1 git --no-optional-locks` gelesen. Keine Beschaffung fehlender Objekte, kein Checkout, kein Build.

## Lokaler REF als Softwareoriginal

U01 ist ein tatsächlich vorhandener Build-Quellbaum auf Basis v2026.07/ece349ade2973e220f524ce59e59711cc919263f mit lokalen Diagnosepatches. Sein DDR-Code belegt die in 02 genannten Registerzugriffe und Rückgabepfade, nicht ihre physische Wirkung.

Die feste Boardkonfiguration ist ein Codebefund: `novena_spl.c`:493–524, dsize=2, ncs=1, rtt_wr=0, rtt_nom=1, WALAT=0, RALAT=5, BI=1; Speicherbeschreibung mit mem_speed=1600, density=4, width=64, banks=8, rowaddr=16, coladdr=10, pagesz=2. Kommentare zu Terminierungswiderständen werden nicht als Herstellerbeleg verwendet. Die Modulgeometrie muss anhand der tatsächlich bestückten Speicherquelle geprüft werden. Eine Beschreibung mit width=64 ist keine unabhängige Aussage über die Breite einzelner SDRAM-Chips.

E01 ist die vollständige erhaltene kurze serielle Beobachtung von Fall 8B. Sie enthält frühe Nullrücklesungen und -12, aber weder SPD noch Chiprevision, Kalibrierungsregister, Wait-Rückgaben oder eine ausgeführte Herstellerqualifikation. Ein positiver Softwarebefund wird nicht zum positiven Hardwarebefund.

## Xobs-Historie und SPD

U02: lokales `u-boot-novena`, HEAD da65dfe4eb70bda1fac5238718c7c8ef763af3bd. Die frühere in 14 dokumentierte Gruppierung rc5/rc7 bzw. rc12/rc14 wird übernommen; gleiche DDR-Dateien bedeuten nicht gleiche vollständige SPLs. Keine erneute Testserie.

Am Stand bd5c7d3290bb94bf8af17760c321df67d2641343:

- Die Datei heißt **`common/ddr_spd.c`**, nicht `lib/ddr_spd.c`. `ddr3_spd_check` ab Zeile 89 berechnet CRC16 mit Polynom 0x1021, Startwert 0, Länge 117/126 gemäß SPD-Byte 0, Vergleich mit Bytes 126/127; Rückgabe 0/1. `common/Makefile`:242 und `include/configs/novena.h`:32 verbinden CONFIG_DDR_SPD mit dem Objekt.
- `ddr3.c`:807–830 liest SPD über 0x50 und ersetzt es bei I2C-/CRC-Fehler durch ein eingebautes Muster; dieses wird nicht nochmals CRC-geprüft. Die Muster-Modulkennung S9905469-145.A00LF identifiziert kein physisches DIMM.
- `ddr3.c`:142–210 enthält einen deterministischen Test von 256 KiB ab 0x10000000; Ergebnis ist ein Boolean aus Summenvergleich, **kein negativer Fehlercode**. Die ältere negative Ergebnisdarstellung ist durch den Code eingeschränkt (46D/K7). Der Bereich entspricht weder den aktuellen Testwörtern noch dem Heap.
- Der Boardpfad um 690–706 meldet Fehler, setzt aber fort. WL-/DQS-Rückgaben werden nicht ausgewertet.

U05/U06, originale archivierte Kosagi-Bootseiten, sind Entwicklungs-/Versionshinweise; sie sind kein Beweis, dass eine bestimmte Binärdatei mit diesen Parametern erzeugt oder auf der heutigen Novena betrieben wurde.

## Hauptlinien-Übergang 2015/2016

U07: 89d485940106c095ec69ecb12a78a42e096dece7, „arm: imx6: Enable DDR calibration on Novena“. Commitobjekt unabhängig lokal gelesen: Autor Marek Vasut, 16.12.2015 15:40:07 +0100; Committer Stefano Babic, 02.02.2016 20:59:05 +0100. Committext kündigt Kalibrierung und geänderte Startparameter an.

**Nicht geprüft:** der vollständige Diff und vollständiger v2016.03-Board-/DDR-Stand. Das lokale Teilrepository fehlt am betreffenden Blob eb46265a32d5c8731bda79b286b8c62cbf83ea65; Lazy Fetch blieb deaktiviert. Der in 46D/14 geprüfte Tag-Graph ordnet den Commit v2016.03-rc2/stabil zu, nicht v2016.01/rc1. Das ersetzt den fehlenden Parameterdiff nicht.

U04: [originale Reviewmail vom 22.12.2015](https://lists.denx.de/pipermail/u-boot/2015-December/238954.html), Marek Vasut mit Eric-Nelson-Zitaten. Enthält Kalibrierungsreview, MR1-/Handshake-Fragen, Timeout-Diskussion und bereits Kommentar 0x80 gegen Code 0xc0. Ein Reviewentwurf ist nicht automatisch der gemergte Code; der dortige Polling-/Hang-Pfad unterscheidet sich von U01. NXP-Verweise in der Mail sind indirekte Angaben, keine gelesenen Herstelleroriginale.

U08 bezeichnet die weiterhin nicht abrufbaren Enable-/Folgemails. URLs und Werkzeugfehler stehen in 05. Das erreichbare Review wird nicht als Ersatz für diese anderen Originale ausgegeben.

## Änderung 2026: belegter Diff, eingeschränkte Interpretation

U03: cec2f200b4bfcc466e4a83196ed5fdd54678c1c7, parent 573b03317aab3529bd813ef5259cd84c98edb52d, Autor 25.06.2026, Committer 27.07.2026. Vollständiger lokaler Diff gelesen: zwei Subtraktionen 0xc0 → 0x80 in `modify_dg_result`, Masken unverändert.

Commitbegründung nennt AN4467 Rev. 2 §12.3.2, RM Rev. 6 §44.11.3.1.2 und eine Winbond-Beobachtung. Das ist **B** für Änderung, **C** für die nicht unabhängig zugänglichen Herstellerpassagen und **D** für ihre Übertragung auf Fall 8B. Tested-by ist Metadatum, kein hier vorhandenes Testrohlog. Eine vermutete Übereinstimmung mit Stress Tool 3.0.0 ist kein geprüfter Toolquellcode.

46D belegt die Tagzuordnung: nicht v2026.07 oder v2026.10-rc1; enthalten in rc2/rc3/rc4. Keine hier neu geprüfte Aussage zu einem späteren stabilen Release. Weder Änderung noch Begründung sind eine Patchfreigabe.

## Geschlossene und verbleibende Lücken aus 46D

K2 (SPD-Pfad/CRC/Integration) und K6/K7 (Softfail/Boolean) sind als Softwarebefunde geklärt. K1 bleibt eine Archivierungs-/Byte-Reproduzierbarkeitslücke für neue externe Originale. K3 (89d485/v2016.03) bleibt offen. K4 (Wiki-Footer) wird in 05 nachgetragen, soweit Originalfooter vorhanden. K5 (ECO-Datierung) bleibt ungelöst. K8 (früher nur flüchtiger Ausgangsbestand) lässt sich für 46C nicht rückwirkend rekonstruieren; für diesen Auftrag wird der vor eigenen Schreibvorgängen erfasste Bestand dauerhaft in 07 abgelegt.
