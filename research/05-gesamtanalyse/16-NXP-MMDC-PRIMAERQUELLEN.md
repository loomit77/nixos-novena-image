# NXP-MMDC: geprüfte Primärquellen und Registergrenzen

Stand: 2026-10-03. BLOCK 9C-46C. Externe Recherche und lesender Abgleich vorhandener Quellen; keine Implementierung oder Hardwarequalifikation.

## Leseregel und Ergebnisumfang

Quellenkennungen und vollständige Metadaten stehen in [20-EXTERNE-QUELLENMATRIX.md](20-EXTERNE-QUELLENMATRIX.md). BELEGT bedeutet stets „innerhalb der angegebenen Quelle und Aussagegrenze“. Herstellerangabe, Implementierung und Hardwarebeobachtung sind getrennte Evidenzebenen. Ein zugänglicher Katalogeintrag ersetzt kein gelesenes Referenzmanual.

Die verlangte vollständige Herstellerprüfung der MMDC-Felder ist **nicht abgeschlossen**: IMX6DQRM, AN4467 und IMX6DQ6SDLHDG waren als Dokumentinhalt nicht erreichbar. Der Originaltext von ERR050070, das Consumer-Datenblatt, EB817 und ein historischer DDR-Stress-Tester-Leitfaden konnten gelesen werden. Es gibt daher keine belastbare Grundlage, sämtliche unten genannten Bits als direkt herstellerverifiziert zu markieren.

## Direkt gelesene Herstellerangaben

**N02 – BELEGT, Datenblatt Rev. 6, 11/2018, §4.10, S. 64, Tabelle 43:** DDR3/DDR3L: 532 MHz, 16/32/64 Bit, ein Kanal, zwei Chip-Selects. §4.10.1 nennt JESD79-3D und verlangt die Layoutvorgaben des Hardware Development Guide. §1.2, S. 6 nennt 256 KiB OCRAM und 96 KiB Boot-ROM. Die Teilenummernabbildung auf S. 5 ordnet die Suffixe C/D/E den Revisionen 1.2/1.3/1.6 zu. Das sind weder die gemessene Novena-Taktfrequenz noch eine Identifikation des aktuell verwendeten SoC oder DIMM. [Originaldatenblatt](https://www.nxp.com/docs/en/data-sheet/IMX6DQCEC.pdf).

**N08 – BELEGT, EB817 Rev. 0, 08/2016, §§1–4, S. 1–5:** NXP behandelt nicht optimale SDCLK-Tastverhältnisse; dabei wurden keine Datenintegritätsfehler beobachtet. Tabelle 3 empfiehlt für genutzte SDCLK-Ausgänge CK_FT0_DCC=010b und CK_FT1_DCC=001b: 0x24921492 statt Resetwert 0x24922492 an 0x021b08c0 beziehungsweise 0x021b48c0. Empfohlene Position: nach Kalibrierungsvorgaben, vor Forced Measurement. Nach Initialisierung sind solche Änderungen nur über Self-Refresh/LPMD/DVFS vorgesehen. Layout und Speicherwahl begrenzen die Übertragbarkeit. Das Bulletin schreibt Registerbezeichnungen uneinheitlich als MPDDCR/MPDCCR; eine Vereinheitlichung wird hier nicht als Originalbefund ausgegeben. [EB817](https://www.nxp.com/docs/en/engineering-bulletin/EB817.pdf).

**N09 – BELEGT, DDR Stress Tester User’s Guide, Dokumentkopf Rev. V2.0.0, Datumszeichenfolge 6/10/2015, Kap. 1, 3, 4:** Der Leitfaden trennt Kalibrierung und anschließenden Stresstest, beschreibt DDR3-Reihenfolge WL → DQS-Gating → Read Delay → Write Delay und nennt IRAM als Ausführungsort. S. 3-9 empfiehlt bei WL-Delay über 0x2f WALAT=1; S. 3-11 verlangt Prüfung bei der vorgesehenen Betriebsfrequenz. Der Dateiname V2.1.0 widerspricht der Versionsangabe im Inhalt. Diese Quelle belegt keine aktuellen MMDC-Werte, keine Novena-Qualifikation und nicht das Verhalten von Tool 3.0.0. [Hersteller-PDF im öffentlichen NXP-Forum](https://community.nxp.com/pwmxy87654/attachments/pwmxy87654/imx-processors/77920/1/i.MX_DDR_Stress_Tester_V2.1.0_User_Guide.pdf).

## Dokumentrevisionen und nicht gelesene Inhalte

| Quelle | Tatsächlich geprüft | Aussagegrenze |
|---|---|---|
| N03, offizieller i.MX6Q-Dokumentkatalog | Eintrag IMX6DQRM Rev. 6, Veröffentlichung 22.05.2020; Zugang mit Konto | Kein RM-Originalinhalt gelesen |
| N04, PCN16354 vom 01.08.2014, wirksam 02.08.2014 | Änderungsbereiche einschließlich MMDC, System Boot, Memory Maps | Kein Feld-für-Feld-Revisionsvergleich |
| N06a/b, NXP-Community-Katalog zu RM Rev. 2 | Beschreibung: Juni 2014; Teil 1 Kap. 1–43, Teil 2 Kap. 44–71/Anhänge | Anhang/PDF im Werkzeug nicht verfügbar; Kommentare sind nicht das Manual |
| U03, U-Boot-Commit cec2f200… | Autor nennt AN4467 Rev. 2, 03/2015 und IMX6DQRM Rev. 6, 05/2020 §44.11.3.1.2 | Herstellerstellen nur vom U-Boot-Autor berichtet |
| N05, NXP-Supportantwort 30.04.2025 | Hinweis auf angehängte AN4467 | Anhang selbst nicht gelesen |

N03: [Produktkatalog](https://www.nxp.com/products/i.MX6Q). N04: [PCN16354](https://www.nxp.com/docs/en/product-change-notice/PCN16354.htm). N06a: [RM-R2-Teil-1-Seite](https://community.nxp.com/t5/i-MX-Processors-Knowledge-Base/i-MX-6DQ-Reference-Manual-IMX6DQRM-R2-Part-1/ta-p/1115983). N06b: [Teil-2-Seite](https://community.nxp.com/t5/i-MX-Processors-Knowledge-Base/i-MX-6DQ-Reference-Manual-IMX6DQRM-R2-Part-2/ta-p/1118510).

## Die fünf ausdrücklich verlangten Register

Die folgende Tabelle ist bewusst **keine Ersatz-Registerbeschreibung**. U02 ist der tatsächlich gelesene lokale REF-Quellstand; N01 ist das Herstellererratum. Bitpositionen aus C-Masken sind Implementierungsbefunde, solange das Manual fehlt.

| Register | Direkt belegter Inhalt | Feldpositionen aus REF-Code | Zugriffssemantik und Bedingungen |
|---|---|---|---|
| MPWLGCR | N01 nennt WL_HW_ERRn als unzuverlässige Fehleranzeige; Einzelheiten in 17 | U02 ddr.c:164–179 startet mit 1, wartet Bit 0 auf 0 und prüft Maske 0x00000f00, also Bits 11:8; Codekommentar nennt Startbit HW_WL_EN | Hersteller-R/W-, W1C-, Reset-, Lane-Zuordnung nicht vollständig geprüft. Die von N01 beschriebene automatische Fehlerlöschung ist keine Software-W1C-Aussage |
| MPWLHWERR | N01 benennt alternatives Erfolgs-/Fehlerkriterium je aktiver Byte-Lane | In U02 ddr.c kein Zugriff; die Softwaremasken des Ersatzkriteriums sind somit nicht ableitbar | Bitbreite, Lane-Zuordnung, RO/RW, Clear-on-read, Reset und Zeitpunkt der Gültigkeit fehlen im gelesenen Herstelleroriginal |
| MPWLDECTRL0 | U02 ddr.c:127 ff., 183 ff., 93 ff.: Werte sichern, Soft-Fail-Muster prüfen, ggf. zurückschreiben/korrigieren | Prüfung 0x001f001f; Korrektur jedes Halbworts über 0x17f, Grenze 0x148 | Implementierung liest/schreibt, einschließlich explizitem Readback. Kommentar über „stick“ belegt keinen Herstellervertrag und keine allgemeine Speicherbarriere |
| MPWLDECTRL1 | Wie MPWLDECTRL0; bei dsize=2 beide PHYs verwendet | Gleiche Masken; genaue Hersteller-Feldnamen nicht direkt geprüft | Wie oben; nicht jede Lane-Ausprägung wird vom All-registers-equal-Soft-Fail-Test erfasst |
| MPDGHWST0–3 | U02 modify_dg_result und Diagnoseausgaben | Extraktion Bits 26:16 über 0x07ff0000, Rechtsverschiebung 16; dann Abzug 0xc0, Aufteilung 0x7f und 0x780 | Code nennt oberen/unteren Grenzwert. U03 berichtet Herstellerfeld HW_DG_UPn. Vollständige Feldbeschreibung einschließlich unterer Grenze, Lesen/Löschen/Validität NICHT_GEPRUEFT |

**WIDERSPRUCH:** U02-Kommentar behauptet Korrektur auf oberen Grenzwert minus 0x80, ausgeführt wird minus 0xc0. U03 ändert genau diese zwei Subtraktionen. Der Softwarewiderspruch ist direkt belegt; die unterschiedlichen Herstellerrevisionen sind mangels Originaltext noch QUELLENLUECKE. Siehe 18.

## Wait- und Timeout-Semantik: Softwarevertrag

U02 include/wait_bit.h wurde gelesen. Die Funktion pollt bis alle maskierten Bits gesetzt beziehungsweise gelöscht sind. Sie prüft zuerst den Registerwert, danach get_timer(start) > timeout_ms, verzögert 1 µs und ruft schedule(). Rückgabe: 0 bei Erfüllung, -ETIMEDOUT nach Ablauf, bei aktivierter Unterbrechbarkeit -EINTR. Das ist kein herstellerseitig festgelegter MMDC-Maximalwert.

In ddr.c stehen elf syntaktische Aufrufstellen mit 100 ms und breakable=0: Zeilen 25, 28, 43, 48, 170, 343, 377, 424, 488, 542, 584. Hilfsfunktionen werden mehrfach aufgerufen; elf Stellen sind nicht elf ausgeführte Polls. Alle Rückgaben bleiben unbeachtet. Der Boardaufruf ignoriert zusätzlich beide Kalibrierungsrückgaben. Timerfortschritt und tatsächlicher Ablauf auf Hardware wurden nicht geprüft. Ein Rückgabewert der Kalibrierung von 0 wäre daher kein eigenständiger Beweis für ausgebliebene Timeouts.

| Softwareoperation in U02 | Ausgewertetes Bit | Herstellerprüfung |
|---|---|---|
| FIFO-Reset zweimal | MPDGCTRL0 Bit 31 auf 0 | Manual fehlt |
| Precharge/Refresh-Acknowledge | MDSCR Bit 14 auf 1; am Schluss auf 0 | CON_ACK-Bezeichnung aus Code; genaue Zusage fehlt |
| Dummy Write | MPSWDAR0 Bit 0 auf 0 | Code nennt SW_DUMMY_WR; genaue Zusage fehlt |
| DQS-Gating | MPDGCTRL0 Bit 28 auf 0; Fehlerprüfung Bit 12 | Codebefund; kein vollständig geprüfter Herstellervertrag |
| Read-/Write-Delay | entsprechendes HWCTL Bit 4 auf 0; Fehlerprüfung Bits 3:0 | Wie oben |

## DDR-Timing, Modusregister und Reihenfolge

U02 mx6_ddr3_cfg enthält mem_speed=1600, density=4, width=64, banks=8, rowaddr=16, coladdr=10, pagesz=2 sowie trcd=1375, trcmin=4875 und trasmin=3500. **width=64 ist der tatsächlich gelesene Strukturwert**, keine erfundene Chipbreite des DIMMs.

ddr.c:1300–1324 begrenzt für MX6D/Q die Rate auf 1066 MT/s und den zur Timingberechnung benutzten clock-Wert auf 528 MHz. Dieser Rechenwert ist keine Messung und kein Nachweis einer programmierten physikalischen Taktfrequenz. N02 nennt 532 MHz; der Codekommentar nennt 528 MHz als Grenze. Unterschiedliche Datenblatt-/Codebezüge sind ungeklärt; daraus folgt kein aktueller Overclockingnachweis.

Der Boardpfad ruft IO-Konfiguration, mx6_dram_cfg, 100 µs Pause, WL und DQS auf. Die automatische DDR3-Modusregisterfolge und alle JEDEC-Mindestzeiten wurden nicht vollständig gegen Herstelleroriginal qualifiziert. WL verwendet in U02 MDSCR=0x00808231 für Eintritt, ddr_mr1=0x4 beim Verlassen. Boardwerte rtt_wr=0 und rtt_nom=1 widersprechen den daneben stehenden Widerstandskommentaren; ddr.c:1472 ff. kommentiert rtt_nom=1 anders. Tatsächliche MR-Bitwerte sind über Code belegbar, eine vollständige JEDEC-/DIMM-Validierung fehlt.

Die Software erhält Refresh-/ZQ-Werte, verwendet FIFO-Resets, Precharge, Dummy-Write und Forced Measurement. Herstellerpflichten zu zusätzlichem tRP, CON_ACK-Reihenfolge und Lesen bestimmter Register bleiben offen. U04 belegt historische Reviewfragen, keine verbindliche NXP-Sequenz.

## OCRAM und ROM-Boot

Die Größenangaben aus N02 ersetzen keine Prüfung reservierter OCRAM-Adressen, ROM-Arbeitsbereiche, IVT/DCD-Limits, maximaler SPL-Ladestrecke, Resetübergaben oder Cache-/MMU-Ausgangszustände. Die historischen Kosagi-Tuningadressen und der dokumentierte SPL-Medienoffset sind Boardquellen, keine NXP-ROM-Verträge. Diese Detailprüfung bleibt QUELLENLUECKE; abgeschlossene lokale Layout-/Maschinencodeuntersuchungen werden nicht erneut als neue externe Belege ausgegeben.

## Interpretation für den gegenwärtigen Fehler

Lokales 8B belegt Store/Load-Nullwerte und später -ENOMEM, aber keine echten WL-/DQS-Masken, Wait-Ergebnisse oder MPWLHWERR-Werte. Herstellerquellen stützen die Notwendigkeit, Statusprüfung und Speicherfunktion auseinanderzuhalten. Sie beweisen weder fehlende DDR-Initialisierung noch einen pauschalen RAM-Defekt. H01–H03 bleiben Hypothesen; H05/H06 gewinnen keine direkt diskriminierende neue Herstellergrundlage.

ROOT_CAUSE=UNRESOLVED  
IMPLEMENTATION_ALLOWED=NO  
BUILD_AUTHORIZED=NO  
HARDWARE_TEST_AUTHORIZED=NO  
PATCH_0008_SECOND_BOOT_ALLOWED=NO  
PUSH_AUTHORIZED=NO
