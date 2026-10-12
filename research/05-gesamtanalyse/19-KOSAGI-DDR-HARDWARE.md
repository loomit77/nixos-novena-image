# Kosagi-Originalquellen: DDR-Hardware, Boardrevisionen und Kaltstart

Stand: 2026-10-03. BLOCK 9C-46C. Originalquellen sind historische Hersteller-/Entwicklerberichte, keine neue Vermessung der heute eingesetzten Novena.

## Quellenauswahl und Datum

K01–K08 werden in 20 mit Original-URLs, Lesebereichen und Grenzen geführt. Der migrierte Studio-Kousagi-Wikibestand liefert über „Retrieved from“ historische oldid-Kennungen, aber nicht bei jeder Seite ein belastbares Änderungsdatum. Suchmaschinen-Crawldatum ist kein Veröffentlichungsdatum. Zeichnungsdatum, ECO-Ausgabe und Quellabruf werden getrennt.

## DDR-Grundaufbau K01

[Novena Main Page](https://www.kosagi.com/w/index.php%3Ftitle=Novena_Main_Page.html), Studio Kousagi Wiki, oldid=1868, Abschnitt Mainboard/Boot: 64-Bit-DDR3-1066-SO-DIMM bis 4 GB und SPL-basiertes Bootverfahren mit DDR-Initialisierung vor U-Boot proper. Die Seite beschreibt außerdem unterschiedliche i.MX6-Varianten. Ihre Angaben sind nicht ohne Prüfung zwischen Quad und DualLite übertragbar.

## Originalschaltplan K08

Original-URL aus der PVT-Quellseite: [SCH-novena_pvt2.PDF](https://bunniefoo.com/novena/pvt2_release/SCH-novena_pvt2.PDF). Externer Neuabruf scheiterte mit Timeout. Verwendet wurde die bereits vorhandene lokale Datei research/01-original-novena-docs/sources/SCH-novena_pvt2.PDF, Blatt 3, Textauszug über pdftotext nach stdout; kein neuer PDF-Download oder Bildexport.

Titelblock: **Novena PVT2-A**, Copyright 2014 Andrew „bunnie“ Huang, Datumszeichenfolge **7/8/2014**. Die Datumsnotation wird nicht ungeprüft umgedeutet.

**BELEGT im Blatttext:** DDR3-SO-DIMM-Sockel mit Hinweis „reverse pinout“, DDR3_D0 bis D63, DQS0–7, CS0/CS1, CKE0/CKE1, ODT0/ODT1, A0–A15, BA0–BA2, SMB-Signale sowie P1.5V_DDR_SW3, P0.75V_REFDDR, P0.75V_DDR3_VTT und RT9045GSP.

**Aussagegrenze:** Die Textextraktion enthält Altium-Netz-/Pinannotation und überlagerten Text. Einzelne Signallabels beweisen keine vollständig visuell geprüfte Pinzuordnung, Leitungslänge, Impedanz, DQ-Bitpermutation oder Bestückung des heutigen Boards. Eine vollständige Layoutprüfung wurde nicht durchgeführt. Das lokale ältere novena_pvt2.PDF ist ein getrenntes Quellenartefakt; sein Hash wird nicht für SCH-novena_pvt2.PDF übernommen.

## Historische DDR-Bringup-Notizen K02

[Novena ddr3 notes](https://www.kosagi.com/w/index.php%3Ftitle=Novena_ddr3_notes.html), oldid=621; Abschnitte Bringup, Tuning, SPD-Dumps und eingebettetes Bootlog tatsächlich gelesen.

**BELEGT als historischer Bericht:** Module gleicher Kapazität/Ranks werden unterschiedlich als Kalibrierung OK/NG eingeordnet. Die Quelle zeigt WL-Soft-Fail trotz errcount=0 und einen Summenvergleich, der vor Tuning abweicht und danach übereinstimmt. Sie berichtet einen DQ0-/Mapping-bezogenen Micron-WL-Sonderfall. Die breite Herstelleraussage ist nicht gegen das konkrete aktuelle DIMM-Datenblatt geprüft.

Historische Modulbeispiele der Quelle:

| Modell | Historische Zuordnung |
|---|---|
| Kingston 99U5428-055 | 2 Rank, 4 GB, Kalibrierung OK |
| Centon MICU58PIA | 2 Rank, 2 GB, Kalibrierung OK |
| Crucial CT51264BC1067.M16FKD | 2 Rank, 4 GB, Kalibrierung NG |
| Crucial CT25664BC1067.M8FMR | 1 Rank, 2 GB, Kalibrierung NG |
| Corsair CM3X4GSD1066 | 2 Rank, 4 GB, Kalibrierung NG |
| Samsung M473B5273DH0-YK0 | 2 Rank, 4 GB, Kalibrierung NG |

Das sind Beispiele früher Versuche, keine aktuelle Kompatibilitätsliste. Sie widerlegen die Gleichsetzung „gleiche Kapazität = gleiche Kalibrierungseigenschaften“. Sie identifizieren kein heute verbautes Modul; eine aktuelle pauschale Micron-Unverträglichkeit folgt nicht.

## DVT-/PVT-Revisionen K03–K06

[K03: Novena Issue Log](https://www.kosagi.com/w/index.php%3Ftitle=Novena_Issue_Log.html), Bringup und Known Issues: historische DDR3-1066-Tests verschiedener Rank-/Kapazitätsfälle und langlaufende Berichte; außerdem Einschalt-Rampen und zusätzliche Resetüberwachung. Solche Angaben sind historische Eigenberichte mit damaligen Bedingungen, keine vollständig erhaltene aktuelle Rohqualifikation.

[K04: Novena DVT Issue Log](https://www.kosagi.com/w/index.php%3Ftitle=Novena_DVT_Issue_Log.html), Hardware Bringup: geänderte DDR-Timings nach PCB-Herstellerwechsel; Neukalibrierung fest kodierter Micron-Timings wird als wahrscheinlich erforderlich bezeichnet. Der Wortlaut ist eine damalige Einschätzung. Die Quelle beschreibt ferner Versorgungsänderungen. Nicht jede dortige FPGA-Speicherkomponente ist der CPU-SO-DIMM.

[K05: Novena PVT Design Source](https://www.kosagi.com/w/index.php%3Ftitle=Novena_PVT_Design_Source.html), oldid=1878: PVT1/PVT2 werden getrennt angeboten; Altium-Originale und KiCad-Übertragungen haben unterschiedliche Provenienz. Übersetzte Designs sind nicht automatisch elektrisch gegen das Original qualifiziert.

[K06: Novena PVT Issue Log](https://www.kosagi.com/w/index.php%3Ftitle=Novena_PVT_Issue_Log.html): gelesene kurze Seite enthält den HPD-Hinweis, keinen belastbaren DDR-Fehlernachweis. Abwesenheit auf dieser Seite beweist nicht Abwesenheit aller PVT-DDR-Probleme.

## Historischer Kaltstart-ECO K07

[Novena PVT2 ECO List](https://www.kosagi.com/w/index.php%3Ftitle=Novena_PVT2_ECO_List.html), oldid=1372, ECO8, ECO13 und ECO23:

**BELEGT als Originalbericht:** ECO13 wechselt die Eingangsversorgungsinduktivität. ECO23 beschreibt darauf zurückgeführte 5-V-Störungen beim kalten Einstecken und Reglerabschaltung. Zusätzlich bestücktes C23N mit 22 µF wird als erfolgreiche Stabilisierung über mehrere Kaltstarts berichtet. Ausgabe: 15.10.2014, ECO-0002 gegen PVT2 Rev. A. ECO8 nennt die geplante CPU-Teilenummer MCIMX6Q5EYM12AC.

**INDIZ für erforderliche Zustandsabgrenzung:** Boardrevision, ECO-Bestückung, Einsteck-/Startablauf und tatsächlicher Versorgungsverlauf gehören zur Anwendbarkeitsprüfung. **HYPOTHESE:** Ein Bezug zur aktuellen frühen DDR-Beobachtung ist nicht nachgewiesen. Keine Bestückungskontrolle, Spannungsmessung oder neuer Boot erfolgte.

Eine Zeichnungs-/BOM-Teilenummer ist kein Hardwareidentitätsnachweis. Die Rev.-Zuordnung aus N02 kann erst nach bestätigter realer Teilenummer übertragen werden. Der alte interne Bootbericht rev1.2 bleibt auf seinen eigenen Lauf begrenzt.

## Board-Bootnotizen K09

[U-boot PVT Notes](https://www.kosagi.com/w/index.php%3Ftitle=U-boot_PVT_Notes.html), oldid=1755: Boardentwickler erläutern SPL-Kalibrierung und Trace-Längenabhängigkeit sowie den historischen Medienoffset. Der Beitrag bestätigt die Boardmotivation, ersetzt jedoch weder ROM-Bootmanual noch die Versionsqualifikation des konkreten historischen SPL.

## Auswirkungen und offene Hardwareidentität

H02 wird auf den konkreten SPD-/Bestückungsvergleich eingegrenzt. H03 darf historische Kalibrierungsbesonderheiten nicht mit aktueller Auslösung gleichsetzen. H08 erhält eine originale historische ECO-Fundstelle, aber keine heutige Versorgungsevidenz. Andere Hypothesen werden durch diese Berichte nicht entschieden.

Offen bleiben aktuelle Boardrevision, tatsächliche ECO-Bestückung, SoC-Siliconidentität, DIMM-Aufdruck/SPD, DRAM-Chips, Rank-/Chipgeometrie und echte Taktrate. Keine dieser Identitäten wird durch Modellwissen ergänzt. Die geschlossene I2C3-/ES8328-Untersuchung und die dauerhafte es8328-power-Anforderung werden nicht verändert.

ROOT_CAUSE=UNRESOLVED  
IMPLEMENTATION_ALLOWED=NO  
BUILD_AUTHORIZED=NO  
HARDWARE_TEST_AUTHORIZED=NO  
PATCH_0008_SECOND_BOOT_ALLOWED=NO  
PUSH_AUTHORIZED=NO
