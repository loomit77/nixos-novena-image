Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# MMDC: Herstellerbeleg, Softwarebeleg und offene Semantik

Klassen gelten pro Aussage: **A** im geprüften Original nachgewiesen; **B** nur in lokaler Software nachgewiesen; **C** aus Sekundärquelle übernommen; **D** plausibel, unbewiesen; **E** widersprüchlich; **F** nicht verifizierbar. Ergebnisstatus bezeichnet den erreichten Nachweisumfang, nicht den Ursachenstatus. Quellen siehe [05](05-PROVENIENZMATRIX.md).

## Registeradressen sind derzeit Softwarebefunde

U01 umfasst die erhaltenen REF-Dateien. Aus `struct mmdc_p_regs`, MX6Q-Basisadressen und 32-Bit-Feldanordnung folgen:

| Register | PHY0 | PHY1 | Klasse |
|---|---|---|---|
| MPWLGCR | 0x021b0808 | 0x021b4808 | B |
| MPWLDECTRL0 | 0x021b080c | 0x021b480c | B |
| MPWLDECTRL1 | 0x021b0810 | 0x021b4810 | B |
| MPWLHWERR | 0x021b0878 | 0x021b4878 | B |
| MPDGHWST0 | 0x021b087c | 0x021b487c | B |
| MPDGHWST1 | 0x021b0880 | 0x021b4880 | B |
| MPDGHWST2 | 0x021b0884 | 0x021b4884 | B |
| MPDGHWST3, zusätzlicher Softwarekontext | 0x021b0888 | 0x021b4888 | B |

Fundstelle: U01, `mx6-ddr.h`:38–122, insbesondere 75–77 und 103–107. Weder ein Herstellername im Header noch die Wiederholung in mehreren Softwareständen macht diese Adressen zu unabhängigen Herstellerbestätigungen. N01 bestätigt für zwei Register einen begrenzten funktionalen Zusammenhang, keine Adresstabelle.

## Kalibrierungsablauf und Aussagegrenzen

| Gegenstand | Nachweis | Klasse / Ergebnis |
|---|---|---|
| MPWLGCR als Fehleranzeige eingeschränkt; alternative Bewertung | N01, S. 143; genaue Herstellerbedingung in 01 | A / BESTAETIGT für die dokumentierte Bedingung |
| Fehleranzeige impliziert aktuellen DDR-Fehler | Kein passender Registerbefund in E01 | D / OFFEN |
| REF-WL-Start und Wartefolge | U01 `ddr.c`:106–253, Start um 164, Wait um 170 | B / BESTAETIGT als Code |
| REF-Fehlerauswertung | Maske 0xf00; Softfail prüft ganze Register auf 0x001f001f, nicht maskierte Felder | B / BESTAETIGT als Code, keine Herstellersemantik |
| REF verwendet MPWLHWERR für WL-Auswertung | Im geprüften Helfer keine entsprechende Nutzung | B / BESTAETIGT als begrenzter Negativbefund |
| MPWLDECTRL0/1: Verzögerungsfelder, Reset, Zugriff und Lane-Zuordnung | Code existiert; N04 nicht zugänglich | B für Code; F / NICHT_VERIFIZIERT für Herstellerbeschreibung |
| MPDGHWST0–2: Ergebnisfelder und Gültigkeitszeitpunkt | Software extrahiert Bits; Herstellertext fehlt | B bzw. F / TEILWEISE_BESTAETIGT |
| Read-to-clear, W1C, RO/RW, Seiteneffekte zusätzlicher Lesezugriffe | Keine ausreichend geprüfte Originalbeschreibung | F / NICHT_VERIFIZIERT |
| Vollständige verbindliche DDR-Initialisierungsreihenfolge | Software und N08 beschreiben unterschiedliche Ebenen; N04/N05/N06 fehlen | F / OFFEN |

U01 `modify_dg_result`:62–86 extrahiert aus Statuswerten Bits 26:16, subtrahiert 0xc0 und setzt die Gate-Felder aus Masken 0x7f/0x780 zusammen. Der Kommentar nennt 0x80. **E / WIDERSPRUCH** zwischen Code und Kommentar ist belegt. U03 ändert später zwei Subtraktionen auf 0x80. Seine zitierten Herstellerpassagen sind ohne N04/N05 weiterhin **C**, kein durch diese Untersuchung bestätigter Algorithmusvertrag.

U01 enthält elf syntaktische Wait-Aufrufe mit 100-ms-Argument; deren Rückgaben werden im DDR-Helfer nicht ausgewertet. Der konkrete Helperpfad ist in U01 als `include/wait_bit.h` erfasst: Wertprüfung, danach Timervergleich und Timeout-Rückgabe. Das beweist weder die reale Zeitbasis noch einen eingetretenen Timeout. Der Boardpfad ruft Konfiguration, 100 µs Verzögerung, WL und DQS-Kalibrierung auf und ignoriert deren Rückgaben (`novena_spl.c`:578–584).

N08 ist eine eigene Stress-Tester-Anleitung; eine Übertragung seiner Sequenz auf das vollständige Bootprotokoll wäre eine eigene Schlussfolgerung. N07 behandelt eine zusätzliche Duty-Cycle-Einstellung. Keine dieser Quellen ersetzt die fehlende Registerbeschreibung.

## Siliconrevision und ERR050070

N02/N09 ermöglichen die Einordnung geeigneter Identitätsbelege. K03 dokumentiert eine geplante historische CPU-Bestückung; K02 enthält historische Bootbanner. Beides identifiziert **nicht** den Chip von Fall 8B. Dokumentfamilie, konkreter Chip, aktiver Kalibrierungsmodus, belegter Fehlerfall und Symptomkette müssen getrennt geprüft werden.

Für H01/H03 ist der Herstellerabgleich **TEILWEISE_BESTAETIGT**: ein realer Anzeigevorbehalt ist belegt. Für die Behauptung „ERR050070 verursacht den Novena-Fehler“ fehlt die entscheidende Evidenz. E01 enthält keine WL-/DQS-Ergebnisse. Es wird kein Workaround vorgeschlagen, kein Registerzugriff ausgeführt und kein zweiter Boot freigegeben.

## Hypothesenbezug

H01: mögliche nicht ausgewertete Fehler/Timeouts bleiben offen. H02: Softwaregeometrie ohne aktuelle Modulidentität bleibt offen. H03: Anzeigevorbehalt bestätigt, konkreter Algorithmusvertrag und Kausalität offen. H04–H07: keine neue diskriminierende Hardwareevidenz. H08: nur historische Versorgungsberichte in 03. H09/H10 liegen nach der in E01 beobachteten Fehlerphase; H11 bleibt eine getrennte Reproduzierbarkeitsfrage. Keine Hypothese wurde hier als Root Cause bestätigt.
