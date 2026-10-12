Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# Konkrete verbleibende Hersteller- und Quellenfragen

Diese Liste ist keine Test- oder Implementierungsfreigabe. Sie benennt die für eine spätere Entscheidung benötigten Originalbelege. Der normale verfügbare Zugriff wurde genutzt; ein Login, eine alternative Netzroute oder eine Wiederholung von Hardwareversuchen wird nicht vorgenommen.

| Priorität / Bezug | Fehlender Nachweis | Konkrete Frage und erforderliche Quelle |
|---|---|---|
| 1 / H01,H03 | N04, vollständige MMDC-Registerbeschreibung | IMX6DQRM Rev. 6 sowie ggf. relevante ältere Revision: Adressen, Resetwerte, RO/RW/W1C/Read-to-clear, Lane-Zuordnung und gültiges Lesefenster für MPWLGCR, MPWLHWERR, MPWLDECTRL0/1, MPDGHWST0–2. Hat späteres Starten einer Kalibrierung oder Lesen Einfluss auf erhaltene Statuswerte? |
| 1 / H01,H03 | N05, AN4467 als Original | Rev. 2, 03/2015, insbesondere §12.3.2 und vollständiger Sequenzkontext: Was bedeuten 0xc0 und 0x80; welche Errata/Revisionen ändern die Vorschrift? Widerspruch zu RM nur anhand beider Originale auflösbar. |
| 1 / H03 | Anwendbarkeit N01 auf aktuellen Chip | Welche Silicon-/Masksetrevisionen betrifft ERR050070 konkret? Ist die Anzeigeeinschränkung für den tatsächlich verwendeten Modus relevant? Herstellerpräzisierung plus getrennt autorisierte reale Chipidentifikation erforderlich. Kein jetziger Zugriff. |
| 1 / H02 | Aktuelles SO-DIMM | Tatsächliche Markierung, SPD, Rank-/Chipgeometrie und zugehöriges Modul-/Chipdatenblatt. Die historischen K02/K04-Muster identifizieren dieses Modul nicht. Beschaffung aus vorhandener Evidenz oder später separat autorisierter Erfassung. |
| 2 / H01,H02,H03 | N06, Hardware Developer’s Guide | Zugängliches Original mit Revision/Fundstellen zu Initialisierung, Terminierung, WALAT/RALAT/BI und Board-/Modulanpassung; nicht aus Kommentaren rekonstruieren. |
| 2 / H02,H03 | U07-Parameterübergang | Vollständiger 89d485-Diff und v2016.03-Originale vor/nach Änderung, samt ursprünglicher Enable-Mail. Lokales Teilrepository und Committext reichen nicht. |
| 2 / H03 | Tool 3.0.0 | Gepinnter Originalquellcode oder nachvollziehbare offizielle Algorithmusdokumentation. N08 gehört zu intern V2.0.0; URL-Bezeichnung V2.1.0 ist ungeklärt. |
| 2 / H08 | PVT2-/ECO-Umsetzung | Boardrevision und tatsächliche Bestückung C23N/ECO-0002 der aktuellen Novena; vollständige ursprüngliche Testbedingungen. Historisches ECO-Dokument ist keine Vollzugsbescheinigung. |
| 2 / Provenienz | Byteoriginale N01/N02/N07/N08/N09/K02/K04/U04 | Originaldateien über normalen erlaubten Zugang sichern, Abrufbeleg und SHA256 der tatsächlichen Bytes. Der aktuelle Web-Inhaltszugriff stellt solche Dateien hier nicht bereit. |
| 3 / Historie | K03-Chronologie | Ursprüngliche Wiki-Revisionshistorie/ECO-Ausgabedokument zum Verhältnis 14.10.-Footer/15.10.-Ausgabe. Keine Zeitverschiebung erfinden. |
| 3 / Historie | Historische Qualifikation | Rohlogs und Einstellungen zu K02 sowie K04/SN007-Burn-in; Vergleichbarkeit von NG- und späteren Funktionsberichten. Vollständiger Layout-/Netzlistenabgleich K05/K07/K08 bleibt separat. |
| 3 / Reproduzierbarkeit | Alter 46C-Ausgangsbestand | Fehlender damaliger dauerhafter Vorher-Manifestnachweis kann durch diesen Auftrag nicht nachträglich ersetzt werden. Das neue Manifest gilt ausschließlich für 46E. |

## Neu erreichter Nachweisumfang

**BESTAETIGT:** N01/N02 und zusätzliche relevante Herstelleroriginale N07/N08/N09 wurden inhaltlich unabhängig gelesen; K02/K04 und U04 sind jetzt als tatsächliche Originalinhalte statt bloßer Fundstellen geprüft. Lokale Software-/Hardwareoriginale sind mit Identität erfasst.

**TEILWEISE_BESTAETIGT:** Kalibrierungsbewertung, historische Modulkompatibilität und Testberichte. Begrenzungen betreffen Revisionsbezug, vollständige Testbedingungen und fehlende Originalbytearchive.

**NICHT_VERIFIZIERT:** N04/N05/N06-Inhalte, Herstellerbestätigung der Registeradressen und Nebenwirkungen, U07-Diff/v2016.03, aktuelles DIMM und aktueller Chip.

**WIDERSPRUCH:** Code-/Kommentarwert in U01/U04; N08-Dateiname gegen interne Revision; K03-Datierungsrelation. Kein ungelöster Widerspruch wird als Grundlage einer Änderung verwendet.

**OFFEN:** Root Cause und H01–H08 im jeweils dokumentierten Umfang; H09–H11 werden durch diese Recherche nicht neu qualifiziert. ERR050070 bleibt eine belegte Einschränkung der Diagnoseanzeige, keine festgestellte Ursache.

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
PUSH_AUTHORIZED=NO
```
