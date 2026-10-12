Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# Historische Novena-Originale: Bestückung, Revision und ECO

Die folgenden Aussagen stammen aus Kosagi-Entwicklerseiten oder erhaltenen Hardwareoriginalen, nicht aus allgemeinen Modellannahmen. Vollständige Quellenmetadaten stehen in [05](05-PROVENIENZMATRIX.md). Historische Boardakten sind Primärquellen für damalige Entwicklerbeobachtungen; ihre physikalische Erklärung ist damit nicht automatisch unabhängig bewiesen.

## Historisch dokumentierte DIMMs

K01 beschreibt die Plattform als 64-Bit-DDR3-1066-SO-DIMM-System bis 4 GB. Das ist keine Garantie für jeden SO-DIMM.

K02, Original „Novena ddr3 notes“, oldid 621, Abschnitt „SPD values“, dokumentiert:

| Modulbezeichnung im Original | Rank / Kapazität | damaliges Urteil |
|---|---|---|
| Kingston HP594907 | 1 / 1 GB | OK |
| Crucial CT51264BC1067.M16FKD | 2 / 4 GB | NG |
| Crucial CT25664BC1067.M8FMR | 1 / 2 GB | NG |
| Corsair CM3X4GSD1066 | 2 / 4 GB | NG |
| PNY Generic 4 GB | 2 / 4 GB | NG |
| Samsung M473B5273DH0-YK0 | 2 / 4 GB, custom layout | NG |
| Centon MICU58PIA | 2 / 2 GB | OK |
| Kingston 99U5428-055 | 2 / 4 GB | OK |

„OK/NG“ sind lokale historische Urteile ohne hier vollständig rekonstruierte Testmatrix. Weitere Bring-up-Abschnitte enthalten Tuning- und Bootausgaben. Die pauschale Micron-WL-Erklärung ist eine Entwicklerinterpretation, keine separat geprüfte Speicherherstellerquelle. Ein dortiger Fehlerzähler von null allein ist kein PASS-Vertrag.

K04, Original „Novena DVT Issue Log“, oldid 944, „Distribution Log“: SN003 mit 2-GB-Single-Rank-Crucial, SN004/SN006 mit 4-GB-Dual-Rank-Crucial, SN005 mit 4-GB-Dual-Rank-PNY; SN007 mit 2-GB-Single-Rank-Crucial und „200 hours burn-in ok“. Das ist ein realer historischer Testbericht; vollständige Rohlogs, Frequenzen und Bedingungen fehlen. Hardware-Bring-up erwähnt DDR-Timingänderungen nach PCB-Herstellerwechsel; eine weitere Kalibrierung wird als vermuteter Bedarf formuliert.

Unterschiedliche NG-/Burn-in-Angaben zu Crucial sind kein beweisbarer Widerspruch für dasselbe Modul unter identischen Bedingungen: Boardphase, Zeitpunkt und Testverfahren unterscheiden sich bzw. sind unvollständig bestimmt. **OFFEN** bleibt die Vergleichbarkeit. Die aktuell bestückte Novena wird keinem dieser Module zugeordnet.

## PVT2-Hardwareoriginale

K05 ist das erhaltene `SCH-novena_pvt2.PDF`; K06 ist das getrennte, anders gehashte `novena_pvt2.PDF`. Keine der Dateien wurde ersetzt. K05, Blatt 3 „cpu_sodimm“, Novena PVT2-A, Datum wörtlich „7/8/2014“: 64 DQ, acht DQS-Gruppen, zwei CS/CKE/ODT, 1,5-V-DDR-Versorgung und 0,75-V-Referenz/VTT. Das bestätigt die gezeichnete Topologie, nicht Fertigungszustand, Signalqualität oder tatsächliche Nutzung beider Ranks.

K07 ist das historische Altium-Archiv mit unter anderem `03cpu_sodimm.SchDoc` und `novena_pvt2_a.PcbDoc`; K08 die Gerberlieferung. Geprüft wurden Identität und Archivinhalt, **kein vollständiger Layout-/Netzlisten-/Längenabgleich**. PDF-Erzeugungsdatum, ZIP-Zeitstempel und HTTP-Last-Modified sind verschiedene Metadaten. Aus ihnen folgt keine nachgewiesene Boardfertigung.

K09 trennt PVT1/PVT2-Designlieferungen. Eine KiCad-Konvertierung wäre ohne zusätzlichen Abgleich kein Ersatz für die Altium-Originale.

## ECOs und Versorgungsbeobachtungen

K03, ECO 8, nennt die geplante CPU MCIMX6Q5EYM12AC. Das ist eine historische Bestückungsanweisung, keine aktuelle Chipidentifikation. ECO 13 betrifft die Eingangsdrossel.

K03, ECO 23: C23N, 22 µF, DNP → bestückt; dokumentiert einen Versorgungs-/Anlaufbefund bei kaltem Einstecken und einen Stabilitätsbericht nach Änderung. Geltung: PVT2 Rev. A, ECO-0002. **Kein Beweis für H08 auf der aktuellen Novena.** K04s C22N-Änderung und FPGA-Speicher U12F sind davon getrennt; U12F ist nicht das CPU-SO-DIMM.

**WIDERSPRUCH / E, ungeklärte Chronologie:** K03 nennt eine ECO-Ausgabe am 15.10.2014, der HTML-Footer eine letzte Änderung am 14.10.2014 19:18. Zeitbasis, Migration oder Datumsfehler sind nicht belegt. Keine Vermutung wird als Auflösung eingetragen. Die technische ECO-Aussage bleibt als damaliger Bericht lesbar; die genaue Ablaufdatierung bleibt offen.

K10/K11 dokumentieren frühere EVT-/DVT-Arbeit. Ihre Geltung wird nicht automatisch auf PVT2 übertragen. Die bestehende abgeschlossene I2C3-/ES8328-Untersuchung und die permanente Boardanforderung `regulator-always-on` bleiben unberührt.

## Ergebnis

**BESTAETIGT:** Existenz und Inhalt der genannten historischen Bestückungs-/ECO-Angaben sowie lokale Originalidentitäten. **TEILWEISE_BESTAETIGT:** historische Funktion anhand Entwicklerlogs, ohne vollständige Reproduktionsbedingungen. **NICHT_VERIFIZIERT:** aktuelle Modul-/Chip-/Boardidentität und tatsächliche ECO-Umsetzung. **OFFEN:** H02/H08 und Vergleichbarkeit der historischen DDR-Tests. Keine historische Angabe erklärt E01 unmittelbar.
