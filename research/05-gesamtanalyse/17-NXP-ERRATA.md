# NXP-Errata: ERR050070 und Anwendbarkeitsgrenzen

Stand: 2026-10-03. BLOCK 9C-46C. Der Abruf und die Auswertung sind Recherche, keine Freigabe eines Workarounds.

## Herstelleroriginal N01

[Chip Errata for the i.MX 6Dual/6Quad and i.MX 6DualPlus/6QuadPlus, IMX6DQCE](https://www.nxp.com/docs/en/errata/IMX6DQCE.pdf), NXP Semiconductors, Rev. 7, 02/2019, 229 PDF-Seiten. Tatsächlich gelesen: Titelseite, Revisionshistorie, MMDC-Übersicht und MMDC-Einträge S. 138–143. Nicht das gesamte Dokument vollständig ausgewertet. PDF-Seitenindex 142 entspricht gedruckter S. 143. Auswertung über PDF-Text; angefragte Screenshots ergaben hier keine sichtbare Bildprüfung.

**E01 – BELEGT, ERR050070, S. 143:** Bei automatischem Hardware-Write-Leveling werden MPWLGCR[WL_HW_ERRn] bei Fehler gesetzt, aber vor Softwareerfassung automatisch gelöscht. Sie sind keine verlässliche Fehleranzeige. Als Workaround nennt NXP MPWLHWERR: Nichtnull je verwendeter Byte-Lane bedeutet erfolgreich; Null in einer aktiven Lane bedeutet Fehler. Die Software soll diese Anzeige verwenden. Keine geplante Siliconkorrektur; BSP-Status laut Eintrag nicht anwendbar auf den BSP. Das ist keine Aussage über beliebige SPL-Software.

**E02 – BELEGT / eingeschränkte Reichweite:** Rev. 7 ergänzt ERR050070. Dokumenttitel umfasst Dual/Quad und DualPlus/QuadPlus. Der Eintrag nennt keine bestimmte betroffene Siliconrevision; eine gesonderte verifizierte Maskenliste liegt nicht vor. Historisches rev1.2 ist kein aktueller Hardwareidentitätsnachweis.

**E03 – BELEGT, weitere MMDC-Einträge:** ERR011421 auf S. 142 schließt DDR3 ausdrücklich aus; ERR010481 auf S. 140–141 schließt DDR3/DDR3L aus. ERR005778 auf S. 138 betrifft Betrieb unter 100 MHz. ERR009596 auf S. 139 betrifft ARCR_GUARD. Keine dieser Bedingungen wurde aktuell gemessen.

## Voraussetzungenprüfung für Novena

| Voraussetzung | Quellenstand | Einstufung |
|---|---|---|
| Produktfamilie des Projekts: i.MX6Quad | Boardunterlagen und lokale Projektdokumentation | BELEGT für Projektziel; keine neu durchgeführte Hardwareidentifikation |
| Identität und Siliconrevision des jeweils betroffenen aktuellen Laufs | Historischer interner Boot meldet rev1.2; aktuelles 8B ohne gleichwertige Identifikation | QUELLENLUECKE für universelle Übertragung |
| Automatisches Hardware-WL wird im REF programmiert | U02 ddr.c:164 ff.; Board novena_spl.c:583 | BELEGT als Implementierung, keine erfasste Abschlussbeobachtung |
| Welche Byte-Lanes tatsächlich aktiv und funktionsfähig waren | REF beabsichtigt 64 Bit; realer Lane-Status fehlt | QUELLENLUECKE |
| Auswertung von MPWLHWERR im REF | Kein Zugriff in der gelesenen ddr.c | BELEGT: dokumentierter Ersatzweg dort nicht implementiert |
| Tatsächlich aufgetretener WL-Fehler | Keine MPWLHWERR-Aufzeichnung, keine echte WL-Maske im 8B-Rohlog | NICHT_GEPRUEFT |
| Zusammenhang mit Store/Load-Nullwerten | Kein kontrollierter Übergangs-/Kausalitätsbeleg | HYPOTHESE |

**Technische Interpretation:** Die in U02 verwendete Prüfung von MPWLGCR mit 0xf00 reicht angesichts E01 nicht als allgemeiner Erfolgsnachweis. Der zusätzliche Soft-Fail-Test auf gleichzeitiges 0x001f001f in allen relevanten MPWLDECTRL-Registern ist ein anderes Softwarekriterium. Es ist nicht als gleichwertiger Ersatz für das laneweise Herstellerkriterium qualifiziert. Auch unveränderte oder plausible Kalibrierungswerte beweisen keine korrekte Speicherfunktion.

**Nicht behauptet wird:** ERR050070 sei auf jeder heute untersuchten Novena-Siliconkonfiguration vollständig qualifiziert oder tatsächlich ausgelöst; ein Fehler des Statusregisters verursache unmittelbar jeden Speicherfehler; der Herstellerworkaround sei bereits sicher implementierbar. Zum letzten Punkt fehlen insbesondere Registerfeldzuordnung, Zugriffseigenschaften und Gültigkeitszeitpunkt aus dem RM.

## Gegenbelege und Abgrenzungen

E03 begrenzt die Übertragung ähnlich klingender LPDDR2-/Power-up-Errata auf DDR3. Ein Kaltstartsymptom genügt nicht zur Auswahl eines Erratums. Für den Unter-100-MHz-Fall fehlt ein realer Taktverlauf; der REF-Rechenwert beantwortet nicht jede Übergangsphase.

EB817 ist ein gesondertes Engineering Bulletin, kein Beleg, dass ERR050070 durch Tastverhältniskorrektur behoben würde. Der spätere U-Boot-DQS-Commit betrifft wiederum eine andere Rechenoperation. Alle drei Sachverhalte bleiben getrennt.

## Auswirkung auf Hypothesen

- H01: Die Annahme „keine WL-Fehlerbits bedeutet fehlerfreie Kalibrierung“ wird eingeschränkt. Kein Nachweis eines tatsächlichen WL-Fehlers.
- H03: Das Erratum ist nun direkt herstellerbelegt; konkrete Anwendbarkeit und Auslösung bleiben offen. Der DQS-Algorithmusfall darf damit nicht gleichgesetzt werden.
- H08: Keine zusätzliche aktuelle Versorgungs- oder POR-Beobachtung. Ähnliche Erratumtitel reichen nicht als Kausalitätsbeleg.
- H04–H07 und H09–H11: Kein neuer diskriminierender Erratabeleg.

## Erforderliche Herstellerklärung

Benötigt werden eine für den konkreten SoC qualifizierte Revisionszuordnung, die MPWLHWERR-Registerbeschreibung samt Lane-Feldern und eine genaue Einordnung, wann die Statuswerte nach WL gültig sind. Die fehlenden Herstellerinformationen bleiben ausdrücklich offen. Ein späteres Lesen neuer MMDC-Register ist eine neue Hardwareaktion und hier nicht erlaubt.

ROOT_CAUSE=UNRESOLVED  
IMPLEMENTATION_ALLOWED=NO  
BUILD_AUTHORIZED=NO  
HARDWARE_TEST_AUTHORIZED=NO  
PATCH_0008_SECOND_BOOT_ALLOWED=NO  
PUSH_AUTHORIZED=NO
