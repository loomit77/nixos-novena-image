# Vertiefende Prüfgrenzen: historischer RAM-Test und produktiver Displaypatch

Stand: 2026-10-03. BLOCK 9C-46A. Ergänzung nach gezielter Quellnachprüfung; keine neuen Tests oder Implementierungen. Die Angaben ergänzen 04–07 und begründen keine Wiedereröffnung der abgeschlossenen ES8328/I2C3-Untersuchung.

## Historischer CPU-RAM-Test ist begrenzt

BELEGT im archivierten rc5-Quellstand `9402489e9a3e52a9df5464b4ce3c792cee2ba3a5`:

- `research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/ddr3.c:141`: write_random_data beginnt bei 0x10000000 für bank=0.
- Zeilen 156–170: count=0x20000, size=2, jeweils ein ushort-Store, Adressschritt 2.
- Zeilen 174–207: read_random_data generiert dasselbe Muster und vergleicht nur die aufsummierten erwarteten und gelesenen Werte.
- Zeilen 938–940: novena_memory_test ruft nur bank=0 auf.

Daraus folgt durch statische Intervallrechnung: 0x20000 × 2 = 0x40000 Byte, geprüft wird logisch [0x10000000,0x10040000), also 256 KiB. Control/Target des aktuellen Falls 8B bei 0x18200030/0x1820003c liegen nicht darin. Das Debug-Format berechnet hingegen count × sizeof(addr) × size; für das beabsichtigte 32-Bit-ARM-ulong entspräche das 1 MiB. Debugtext und tatsächlicher Schleifenadressschritt dürfen nicht verwechselt werden.

BELEGT: Der Vergleich erfolgt als Summenvergleich, nicht als Abbruch bei jeder einzelnen abweichenden Speicherzelle. Gegensinnig kompensierende Fehler können eine gleiche Summe ergeben. Die Zugriffsausdrücke sind im historischen C kein volatile; welche Instruktionen tatsächlich entstanden, muss für den jeweiligen historischen Binärstand getrennt geprüft werden. Es wird hier keine Compilerfehlübersetzung behauptet.

Im rc5 `board/kosagi/novena/novena_spl.c`, Abschnitt unmittelbar nach do_dqs_calibration, führt ein negatives Testergebnis lediglich zur Ausgabe „Fail!“; danach folgt weiterhin BSS-Clear und board_init_r. Damit ist fortlaufender historischer Boot ebenfalls kein Beweis, dass sein Test erfolgreich war. Die tatsächliche historische Logausgabe „Pass“ ist ein Beleg des dort gemeldeten Ergebnisses, aber kein vollständiger RAM-, Rank- oder Adressvergleich mit aktuellem P_EXT.

Aussagegrenze: Die genannten Größen und Fehlerreaktionen wurden am rc5-Quellstand geprüft. Eine Übertragung auf das im historischen 2015-Hardwarelog ausgeführte bd5c7d3-Binary verlangt dessen konkrete Testfunktion und Maschinenqualifikation. Die in 14 gesicherten historischen Ausschnitte ersetzen das nicht. Auch ein tatsächlich bestandener begrenzter historischer Test beweist keinen allgemeinen aktuellen DDR-PASS.

## Produktiver IT6251-Patch: lokaler Quellenbezug

Die hinzugefügte Treiberdatei aus `kernel/0001-drm-bridge-it6251.patch` wurde rein im Arbeitsspeicher aus den Pluszeilen rekonstruiert und bytegleich mit `kernel/it6251.c` verglichen. Ergebnis: 29710 Byte, SHA256 `e9c6a7d62101f29e9c69cae0f7d1fc43a370d8591951cd8b0bb9eb20ab06d9d7`, Texte gleich. Dies qualifiziert die lokale Beziehung zwischen Patchinhalt und Entwicklungsdatei; es ist kein Kernelbuild oder Binär-/Hardwarevergleich. `hardware/novena.nix` bindet diesen Patch in der produktiven Patchliste ein.

BELEGT: Die Low-Level-Schreibhelfer in `kernel/it6251.c:82` und `:101` geben I2C-Fehler und kurze Transfers zurück. Im Initialisierungspfad werden viele dieser Rückgaben nicht einzeln ausgewertet (`kernel/it6251.c:300` ff.; Patch etwa Zeilen 305–490). Nicht nur die dokumentiert erwarteten Reset-NACKs, sondern auch Konfigurationsschreibzugriffe etwa LVDS-PLL 0x3b werden unbedingte Funktionsaufrufe ohne Returnprüfung. Später prüft der Treiber Link-/Stabilitätszustand und liefert bei Fehlschlag -EIO. Dies ist eine aggregierte Erfolgsprüfung, kein Beweis, dass jeder einzelne Konfigurationsschreibzugriff erfolgreich war.

INDIZ für eine spätere Robustheits-/Diagnosefrage: Ein echter Konfigurationsschreibfehler könnte erst beim späteren Linkcheck sichtbar werden und mehrere Ursachen ununterscheidbar machen. Widersprechende Grenze: Der bestehende Displaypfad ist mit dauerhaftem ES8328-Always-on funktional qualifiziert; es liegt aus diesem Block kein neuer Displayausfall vor. Erwartete temporäre Reset-NACKs dürfen nicht pauschal fatal gemacht werden.

Die vorhandenen Pre-enable-/Enable-/Post-disable- und Power-Up-Pfade behandeln Regulatorfehler sowie Wiederanläufe. Für vollständige Update-/Suspend-/DRM-API-Kompatibilität fehlen in dieser Gesamtanalyse noch eine versionsgenaue Untersuchung der Kernel-Bridgeverträge und passende erhaltene Laufzeitbelege. Kein Retry, Delay, Powerzyklus oder Treiberfix wird neu vorgeschlagen oder implementiert. Diese Frage ist nachrangig gegenüber dem ungelösten frühen P_EXT-Speicherbefund und keine Fortsetzung der abgeschlossenen I2C-Root-Cause-Serie.

## Ergänzende Fortsetzung

1. Bei historischen RAM-PASS-Behauptungen Testumfang und tatsächliches Binary benennen; nicht auf den gesamten 3,8-GiB-Speicher extrapolieren.
2. RAMbudget/DDR-Versionsmatrix um historische Testadressen und Prüfverfahren ergänzen.
3. Nur bei einer eigenständigen späteren Robustheitsfrage den produktiven Displaypatch gegen den exakten Kernel-DRM-Vertrag prüfen. Keine abgeschlossenen Hardwaretests ohne neue diskriminierende Evidenz wiederholen.

ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
