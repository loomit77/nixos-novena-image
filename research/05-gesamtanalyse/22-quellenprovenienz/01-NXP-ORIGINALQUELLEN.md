Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# NXP-Originalquellen und tatsächlicher Zugriff

Diese Untersuchung baut auf den vorhandenen Forschungsdateien 00, 07–10, 12, 14 und 16–21 sowie dem erhaltenen Bericht aus BLOCK 9C-46D auf. Quellenkennungen und vollständige Provenienz stehen in [05](05-PROVENIENZMATRIX.md). Gedruckte PDF-Seiten und nullbasierte PDF-Indizes werden ausdrücklich unterschieden.

## Inhaltlich geprüfte Herstelleroriginale

**N01 – IMX6DQCE:** [Original](https://www.nxp.com/docs/en/errata/IMX6DQCE.pdf), Rev. 7, 02/2019, 229 Seiten. Revisionstabelle S. 2: ERR050070 wurde mit Rev. 7 aufgenommen. S. 143 (PDF-Index 142), Beschreibung und Workaround: Bei automatischem Write-Leveling werden Fehlerindikatoren WL_HW_ERRn in MPWLGCR vor der Softwareauswertung wieder gelöscht. Stattdessen soll MPWLHWERR verwendet werden; jede genutzte Byte-Lane muss einen Wert ungleich null besitzen. Der Eintrag nennt keinen geplanten Fix und keine BSP-Anwendung. Eine eigene Tabelle betroffener Siliconrevisionen steht dort nicht. Das belegt eine Grenze der Fehleranzeige, keinen auf der Novena beobachteten Kalibrierungsfehler.

**N02 – IMX6DQCEC:** [Original](https://www.nxp.com/docs/en/data-sheet/IMX6DQCEC.pdf), Rev. 6, 11/2018, 171 Seiten. Fig. 1 auf S. 5 ordnet die Silicon-Suffixe C/D/E den Revisionen 1.2/1.3/1.6 zu; Temperaturgrade sind davon getrennt. §4.10, Tabelle 43, S. 64 beschreibt DDR3/DDR3L mit 16/32/64 Bit, einem Kanal und zwei Chip-Selects bis 532 MHz. §1.2, S. 6 nennt 256 KiB OCRAM und 96 KiB ROM. Das ist die Produktbeschreibung, keine Messung von Bestückung, Frequenz oder Chiprevision der aktuellen Novena.

**N07 – EB817:** [Original](https://www.nxp.com/docs/en/engineering-bulletin/EB817.pdf), Rev. 0, 08/2016. §3/Tabelle 3, S. 3: SDCLK-Duty-Cycle-Einstellung 0x24921492 statt Resetwert 0x24922492, Adressen 0x021b08c0/0x021b48c0. Einordnung nach Kalibrierungseinstellungen, vor erzwungener Messung; spätere Änderung über geeignete Low-Power-/Self-Refresh-Sequenz. Das Bulletin meldet keine beobachteten Datenintegritätsprobleme. Es ist kein ERR050070-Workaround.

**N08 – Stress-Tester-Handbuch:** [Original auf NXP Community](https://community.nxp.com/pwmxy87654/attachments/pwmxy87654/imx-processors/77920/1/i.MX_DDR_Stress_Tester_V2.1.0_User_Guide.pdf). Interne Titelseite: V2.0.0, Datum „6/10/2015“; Revisionstabelle: 06/2015. Der URL-Dateiname sagt V2.1.0: **WIDERSPRUCH in der Versionsbezeichnung**, nicht stillschweigend vereinheitlicht. §3-9, PDF-Seite 10: WL, DQS-Gating, Read-Delay und Write-Delay; §3-11: anschließender Stresstest bei Zielfrequenz. Kein Originalquellcode des Tools 3.0.0.

**N09 – EB803:** [Original](https://www.nxp.com/docs/en/engineering-bulletin/EB803.pdf), Rev. 0, 04/2014. Titel: Vergleich Silicon 1.2/1.3; Tabelle 1, S. 2 unterscheidet Silicon- und ROM-Revision sowie Identifikationswerte. Die extern angebotene Textüberschrift „EB802“ passt nicht zur internen Dokumentkennung EB803; maßgeblich ist hier die interne Kennung. Keine Zuordnung der heutigen Novena, kein neuer ROM-Ursachenbefund.

## Nicht als Dokumentoriginal geprüft

**N04 – IMX6DQRM:** Der [NXP-Produktkatalog](https://www.nxp.com/products/i.MX6Q) (N03) führt Rev. 6, 22.05.2020. Direkter PDF-Aufruf lieferte einen internen Abruffehler; der veröffentlichte Downloadlink führte zum NXP-Login und wurde vom Recherchewerkzeug abgebrochen. Keine Anmeldung oder Zugangsumgehung. Der Community-Beitrag über Rev. 2, Part 2 (N11) beschreibt Kapitel 44, stellt aber in der zugänglichen Ausgabe keinen prüfbaren Anhang bereit. Die MMDC-Registerbeschreibung wurde daher **nicht unabhängig verifiziert**.

**N05 – AN4467:** Der NXP-PDF-Pfad lieferte im Recherchewerkzeug 404; auch der historische Freescale-Pfad war nicht abrufbar. Ein NXP-Supportbeitrag vom 30.04.2025 (N10) kündigt einen Anhang an. Dessen Inhalt/Download waren nicht zugänglich. Rev. 2, 03/2015 und §12.3.2 bleiben Angaben aus U03, keine eigene Herstellerprüfung.

**N06 – IMX6DQ6SDLHDG:** Der direkte Pfad lieferte 404; der Katalog-Download endete mit „400 Timeout“. N03 nennt Rev. 4, 20.02.2019. Inhalt und Seitenfundstellen bleiben ungeprüft.

## Zugriffs- und Archivgrenze

Alle erfolgreichen externen Prüfungen erfolgten am 03.10.2026 über den normalen Web-Recherchezugang, anhand dessen PDF-/HTML-Inhaltsausgabe. Das ist mehr als eine URL oder ein Suchtreffer. Es ist aber keine bytegenaue lokale Originalarchivierung und keine PDF-Signaturprüfung. Die Umgebung zeigte bei einem direkten HTTPS-Abruf per Python bereits bei DNS einen Fehler „Temporary failure in name resolution“. Es wurde keine alternative Netzroute eingerichtet.

Für N01/N02/N07/N08/N09 gibt es hier deshalb **keine lokale Originaldatei und keinen SHA256 des PDFs**. Die sieben Forschungsdokumente sichern Befunde, URLs, Revisionen und Fundstellen, ersetzen aber keine Originaldateien. Die Grenze zwischen inhaltlicher Originalprüfung und dauerhafter Byte-Reproduzierbarkeit bleibt offen.
