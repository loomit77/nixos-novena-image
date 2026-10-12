# Recherchelücken, Hypothesenbewertung und Sicherheitscheckpoint

Stand: 2026-10-03. BLOCK 9C-46C. Ergebnis: umfassende Recherche im erreichten Quellenraum, **keine vollständige Herstellerqualifikation**. Qualitätsschranke ist die gelesene Primärevidenz, nicht eine vorgegebene Anzahl von Quellen.

## Ausgangszustand und lokale Grundlage

AGENTS.md wurde vor den Untersuchungen vollständig gelesen. Einstieg über README.md; vollständig gelesen wurden die ausdrücklich verlangten vorhandenen Dateien 00, 03, 04, 06, 07, 08, 09 und 10 in research/05-gesamtanalyse. Hinzu kamen Quellenverzeichnis, die vorhandenen Abschluss-/Prüfgrenzenberichte, historische Git-Belege, relevante aktuelle Nachträge in PROJECT-STATE, TEST-LOG und DECISIONS sowie Rohlog-/Quellstellen. Nicht behauptet wird, sämtliche historischen Projektdateien oder alle Langzeitlogs neu vollständig ausgewertet zu haben.

Die kurze maßgebliche Fall-8B-Rohaufzeichnung wurde vollständig gelesen; die umfangreichen historischen Fallback-/Linux-Logs wurden nicht als neu vollständig extern qualifiziert ausgegeben. Bestehende lokale Abschlussaussagen wurden berücksichtigt, keine neue Hardwarebeobachtung erfunden.

Die vorhandenen Berichte sind überwiegend als BLOCK 9C-46A beschriftet. Eine gesonderte, eindeutig als **BLOCK 9C-46B** bezeichnete Primärdatei wurde bei der exakten Textsuche nicht gefunden. Die vorhandenen neueren Ergänzungen 16-ABSCHLUSSPRUEFUNG.md und 17-VERTIEFENDE-PRUEFGRENZEN.md wurden berücksichtigt. Die fehlende eindeutige B-Zuordnung bleibt eine Provenienzlücke; sie wurde nicht stillschweigend als gelesener separater B-Bericht ausgegeben.

Ausgangsbranch: main. HEAD: 0649b5523b2991d3283413bba835c7ee2f7be432.

Vorhandene Nutzeränderungen: zusätzliche Diagnosepatches 0004–0008, Änderung boot/u-boot/default.nix sowie untracked AGENTS.md, Quellen-/Übersetzungsdokumentation und vorhandene Gesamtanalyse. Alle wurden erhalten. Keine Git-Mutation. Bei allen Git-Abfragen: GIT_NO_LAZY_FETCH=1 und git --no-optional-locks.

## Was tatsächlich erreicht wurde

Die Primärquellen und Lesetiefen sind in 20 verzeichnet, die fachlichen Befunde in 16–19. Direkt zugänglich waren Herstellererrata, Datenblatt, EB817, historischer Stress-Tester-Leitfaden, offizielle Dokument-/Revisionsmetadaten, NXP-Supportkommunikation, eine originale U-Boot-Reviewmail und Kosagi-Entwicklerseiten. Vorhandene lokale Git-Objekte ergänzen den tatsächlichen Versionsvergleich.

Keine Quelle wurde allein wegen eines Suchtreffers zum geprüften Original erklärt. Unzugängliche Hauptdokumente wurden nicht aus Zitaten, fremden SoCs oder allgemeinem Modellwissen nachgebildet.

Die Dokumente enthalten bewusst keine erfundene vollständige MMDC-Registertabelle. Keine Ursache, kein Patch und keine nächste Hardwareaktion ist freigegeben.

## Erreichbarkeits- und Quellenlücken

| Gegenstand / tatsächlicher Abrufweg | Ergebnis / Grund | Nicht geprüfter Umfang |
|---|---|---|
| https://www.nxp.com/docs/en/reference-manual/IMX6DQRM.pdf | Interner Web-Werkzeugfehler, kein Dokumenttext | Vollständiges RM Rev. 6 |
| https://www.nxp.com/webapp/Download?colCode=IMX6DQRM | Fehler, kein Inhalt | Offizieller Downloadpfad |
| https://www.freescale.com/files/32bit/doc/ref_manual/IMX6DQRM.pdf und https://cache.freescale.com/files/32bit/doc/ref_manual/IMX6DQRM.pdf | Cache miss/nicht erreichbar | Historischer Originalpfad |
| NXP-Produktkatalog | Inhalt erreichbar, RM/HDG als Account Required geführt | Keine Anmeldung oder Umgehung; Account-Hinweis allein erklärt nicht jeden Toolfehler |
| NXP-Community RM-R2 Teil 1/2 | Beschreibungsartikel erreichbar, kein brauchbarer PDF-Anhanglink im extrahierten Inhalt | Rev.-2-Registertexte/ROM/OCRAM-Kapitel |
| https://www.nxp.com/docs/en/application-note/AN4467.pdf | 404 | Kalibrierungsschritte und Originalrevision |
| cache.nxp.com / historische Freescale-/assets-Anwendungsnotizpfade zu AN4467 | Fehler oder nicht zugänglich | Keine alternative Herstellerkopie gelesen |
| NXP-Supportantwort vom 30.04.2025 | Anhang angekündigt, PDF selbst nicht erschlossen | Kein gelesener AN4467-Originaltext |
| https://www.nxp.com/docs/en/user-guide/IMX6DQ6SDLHDG.pdf und entsprechende assets-/cache-Pfade | Standardpfad 404, weitere ohne Inhalt | Layout-/Versorgungs-/Bootvorgaben |
| https://www.nxp.com/docs/en/user-guide/IMX_PORTING_GUIDE.pdf, https://www.nxp.com/docs/en/user-guide/IMX6FG.pdf | Kein Inhalt erreichbar | Firmware-/Porting-Anforderungen |
| source.denx.de/GitHub Commit 89d485…; GitHub .patch; Raw-GitHub-Dateipfad | Fehler | Öffentlicher vollständiger Commit-Diff |
| Lokaler Mainline-Partial-Clone | Commitheader/Graph erreichbar, mehrere Blobs fehlen; lazy fetching ausdrücklich deaktiviert | Diffs von 89d485…, d339f169…, b10d93ee…, 14eeb683…, 09dbac817…, aa428944…, 6b5ecb829…, 7d29acd9… |
| https://lists.denx.de/pipermail/u-boot/2015-December/237993.html und Monatsindex | Nicht zugänglich/Cache miss | Direkte Veröffentlichung des Enable-Patches |
| mail-archive.com msg196451, msg197150, msg579500 | Suchindex erreichbar, direkte Öffnung fehlgeschlagen | Vollständige Originalmails / Testbedingungen |
| https://bunniefoo.com/novena/pvt2_release/SCH-novena_pvt2.PDF | Neuabruf Timeout; vorhandene lokale Kopie lesbar | Keine neue externe Byteidentitätsprüfung |
| NXP-Forumbeitrag zu DQS Gating | Suchtreffer, direkte Öffnung gescheitert | Keine daraus übernommenen Registergarantien |
| Direkte Shell-HTTPS-Abfragen zu öffentlichen Quellen | DNS-Auflösung mit Errno -3 gescheitert | Keine Netzwerksandbox-Umgehung; keine Downloaddatei geschrieben |

Die ursprünglichen Hersteller-URLs und die erreichbaren Begleitartikel sind zusätzlich in 20 klickbar. Unterschiedliche fehlgeschlagene Abrufvarianten sind keine unterschiedlichen geprüften technischen Quellen. Nicht alle denkbaren Archive/Mirrors wurden untersucht; unbekannte Kopien sind nicht automatisch Herstelleroriginale.

### Ungeklärte technische Herstellerinformationen

1. MPWLHWERR: Bitfelder, aktive Lane-Zuordnung, RO/RW, Reset, Clear-on-read und Gültigkeitszeitpunkt.
2. Vollständige MPWLGCR-/MPWLDECTRL0/1-/MPDGHWST-Felder samt Bedingungen und Versionsdelta.
3. Direkter Vergleich AN4467 Rev. 2 §12.3.2 mit IMX6DQRM Rev. 6 §44.11.3.1.2. Der U-Boot-Autorbericht ersetzt das nicht.
4. Herstellervorgaben zu CON_ACK, zusätzlichem tRP, Forced Measurement, MR-Wartezeiten und Timeoutgrenzen.
5. Vollständiger DDR3-Modusregister-/ODT-Vertrag und DIMM-spezifische Anforderungen.
6. OCRAM-Reservierungen und ROM-Bootgrenzen für genau i.MX6Quad, nicht nur Gesamtgröße.
7. Qualifizierte betroffene Siliconrevisionen von ERR050070 für die tatsächlich verwendete Hardware.
8. Aktuelles DIMM-Datenblatt/SPD und reale Board-/ECO-/Versorgungs-/Taktidentität.

## Offene Widersprüche

| Widerspruch / Abweichung | Was geklärt ist | Was offen bleibt |
|---|---|---|
| DQS-Kommentar 0x80 vs REF-Code 0xc0 | Direkter Softwarebefund, späterer Commit gelesen | Herstellerrevisionen direkt prüfen; aktueller Hardwareeffekt |
| Bericht AN4467 vs neueres RM | U-Boot-Commit nennt beide Stellen | Keine direkte doppelte Herstellerverifikation |
| REF-Board-RTT-Kommentare vs Zahlen/common-Code | Unterschied tatsächlich gelesen | Vollständige MR-/JEDEC-/Modulqualifikation |
| Datenblatt 532 MHz vs U-Boot-Rechengrenze 528 MHz | Quellenebenen getrennt | Codehistorie/Segmentbezug und realer Takt |
| Stress-Tester-Dateiname V2.1.0 vs Inhalt V2.0.0 | Inhaltsrevision maßgeblich | Herkunft des Dateinamens; Tool 3.0.0 nicht geprüft |
| Historische SPD-Variabilität vs heutige feste Geometrie | Implementierungen unterscheiden sich | Ob der konkrete heutige DIMM tatsächlich abweicht |
| Altes rev1.2-Log vs heutige Hardwarezuordnung | Alter Lauf auf seine Provenienz begrenzt | Identität jeder heute bewerteten Konfiguration |
| Bezug BLOCK 9C-46B | Neuere lokale Ergänzungen berücksichtigt | Eindeutige separate B-Primärdatei nicht gefunden |

## H01 bis H11: nur neue Quellenbelege

Alle H-Nummern entsprechen 07-HYPOTHESEN.md. Keine Bewertung basiert allein auf Ähnlichkeit von Symptomen.

| Hypothese | Neue Evidenz | Einschränkung / unveränderte offene Frage | Ergebnis |
|---|---|---|---|
| H01 Kalibrierungsfehler/Timeout vor BSS | E01 und U04 schärfen Status-/Wait-Abgrenzung; historische Varianten unterscheiden sich | Keine tatsächlichen Wait-Ergebnisse oder Kalibrierungsregister im 8B. Ein fehlerfreier Softwaremaskenwert wäre nicht ausreichend | HYPOTHESE bleibt; unbedingte Erfolgsannahme eingeschränkt |
| H02 Geometrie/Timing des DIMMs | Historische RC-/bd5-Vergleiche und K02; Ersatz-SPD-Pfad tatsächlich gelesen | Gleiche Kapazität genügt nicht. Feste cfg.width=64 ist Code, keine ermittelte DRAM-Chipbreite. Aktuelle SPD-/Chipidentität fehlt | HYPOTHESE bleibt; universelle Festkonfigurationsannahme eingeschränkt |
| H03 Kalibrierungsalgorithmus/Erratum | E01 direkt, U03b vollständig; REF liegt vor späterem DQS-Commit | Statuserratum und DQS-Rechenänderung getrennt. RM-/AN-Originale und reale Auslösung fehlen | HYPOTHESE bleibt; Priorität der Quellenklärung begründet |
| H04 Früher BSS-Clear | Keine neue diskriminierende externe Evidenz | Kein Hersteller-/Hardwarebeleg für störenden Clear | Unverändert; keine Wiederholung abgeschlossener lokaler Ausschlüsse |
| H05 Aliasierung/lokales Speicherverhalten | Geometrie-/Layoutquellen liefern Kontext | Kein qualifizierter Adressdecoder des realen DIMMs, keine neue Adressprobe | Unverändert, HYPOTHESE |
| H06 CPU/Store-Visibility | U04 belegt Readback-Diskussion | MMDC-Registerreadback ist kein Beweis für DDR-Store-Visibility oder erforderlichen Barrierfix; Architekturvertrag ungeprüft | Unverändert; allgemeine Barrierfolgerung eingeschränkt |
| H07 Instrumentierung/Layout/Provenienz | Git-Quellidentitäten und fehlende Diffobjekte besser getrennt | Keine neue Diagnosebinär-/Hardwareevidenz. Ein Commit beweist kein historisches Medium | Unverändert; Provenienzgrenze präzisiert |
| H08 POR/Versorgung | K07 liefert originalen historischen ECO; K03/K04 Revisionskontext | Kein aktueller Versorgungslauf, keine ECO-Bestückung; ähnliche Kaltstartbezeichnung beweist keine Ursache | HYPOTHESE bleibt; Übertragung nur bedingt |
| H09 SD/FAT/Payload | Keine neue diskriminierende DDR-Quelle für nachgelagerte Phase | Früher Speicherbefund dadurch nicht erklärt | Unverändert / aktuell nicht erreicht |
| H10 Kernel/initrd/Root | Keine neue diskriminierende Evidenz | Kein neuer Stage1-Lauf | Unverändert / nachgelagert |
| H11 Wachstum/Update/Reproduzierbarkeit | Späterer Mainline-Commit ist nachvollziehbar getrennt | Kein Build-/Updatevergleich oder Größenproblem geprüft; Pinänderung nicht erlaubt | Unverändert; spätere Änderung nicht als REF-Bestand ausgeben |

**Nicht widerlegt wurde eine vollständige H01–H11-Ursachenhypothese.** Eingeschränkt wurden konkrete pauschale Annahmen: Nullstatus garantiert Erfolg; historisches RAM-PASS qualifiziert den heutigen Diagnosebereich; gleiche Modulkapazität garantiert identische Initialisierung; alle späteren Mainline-Korrekturen stecken schon im REF; historischer ECO/SoC-Aufdruck gilt automatisch heute.

Der in 18 jetzt für bd5 belegte 256-KiB-Summenvergleich schließt eine bisherige Quelllücke im historischen C-Testumfang. Die konkrete damalige Maschinenfolge und Ausführung bleiben getrennt.

## Erforderliche weitere Recherche

### Zuerst Primärtexte und Provenienz

- IMX6DQRM Rev. 6 und AN4467 Rev. 2 über reguläre erreichbare Herstellerwege beschaffen; Dokumentkopf, vollständige Kalibrierungsabschnitte und Registerseiten sichern, bevor technische Umsetzung bewertet wird.
- Die öffentlich angekündigte NXP-Anhangkopie regulär erschließen; Konto-/Zugriffsgrenzen nicht umgehen.
- HDG-Revisionsinhalt und ROM-/OCRAM-Kapitel ergänzen.
- Vollständigen 89d485-Diff und direkte originale Enable-Mail erreichen; fehlende Blobs nur in einem passend autorisierten Beschaffungsauftrag ergänzen.
- DQS-Fixgeschichte einschließlich vorangehender Reviews und genauem Herstellerbezug weiter verfolgen. Nicht gelesene Zwischencommits nicht als implementierte Sequenz ausgeben.
- Aktuelle DIMM-/Boardidentität zuerst aus vorhandenen gesicherten Belegen klären; historische Beispiel-SPDs nicht als aktuelles Modul verwenden.
- Schaltplan/Boardlayout visuell und revisionsgenau prüfen, einschließlich DQ-Permutation, Fly-by und ECO-Provenienz. Der bisherige Textauszug ist keine vollständige Netzlistenqualifikation.

### Erst danach mögliche neue Diagnosen, hier nicht autorisiert

Herstellerqualifizierte minimale Aufzeichnung von Kalibrierung, Waits und aktiven Lane-Statuswerten wäre eine denkbare spätere diskriminierende Richtung. Registerlesungen sind nicht automatisch passiv. Dafür sind neue konkrete Freigaben, belegte Lesesemantik sowie eigene statische Layout-/Timing-/Artefaktqualifikation erforderlich. Kein Retry, Barrierfix, SPD-Umbau, Pinupdate oder weiterer Patch-0008-Boot wurde durchgeführt oder freigegeben.

## Datei- und Sicherheitscheckpoint

Vor dem ersten Schreiben wurden alle **398 vorhandenen regulären Repositorydateien beziehungsweise Symlinkidentitäten** außerhalb .git im Arbeitsspeicher erfasst: SHA-256 für Dateien, Linkziel für Symlinks. Es wurde dafür keine Manifest-/Temporärdatei angelegt.

Nach exklusiver Neuanlage von 16–20 ergab die erneute Inventur:

- 398 Ausgangseinträge unverändert; keine fehlenden oder inhaltlich geänderten Ausgangsdateien.
- Genau fünf neue erlaubte Markdown-Dateien; zusammen 403 Einträge.
- Branch main und HEAD 0649b5523b2991d3283413bba835c7ee2f7be432 unverändert.
- Git-Status einschließlich aller untracked Pfade gelesen; Nutzeränderungen erhalten.
- Alle sechs exakten Zielnamen waren vor dem Schreiben frei. Die vorhandenen anders benannten 16-/17-Berichte wurden nicht überschrieben.

Diese Datei 21 wird anschließend ebenfalls ausschließlich mit exklusiver Neuanlage geschrieben. Die finale Inventur nach allen sechs Dateien wird im Abschlussbericht ausgegeben; diese Datei wird dafür nicht nachträglich geändert. Erwartung: 404 Einträge, genau die sechs autorisierten neuen Markdown-Dateien, keine veränderte Ausgangsdatei.

Sämtliche Inhalte wurden vor dem Schreiben im Arbeitsspeicher entworfen. Keine neuen Roh-PDFs, Logs, Bilder, Backups, Patches oder temporären Dateien. Keine Builds, Tests, Installationen, Hardware-/SSH-/Blockgerätezugriffe, Git-Mutationen oder Netzwerkumgehung. Die Checks sind lesende Inhalts-/Gitprüfungen, keine funktionalen Tests.

**Prüfgrenze:** Der Hashcheckpoint erfasst Repositoryinhalte außerhalb .git, nicht jede Metadatenschreibspur des ganzen Hosts oder interne Metadaten des entfernten Webdienstes. Lesebefehle wurden ohne optionale Git-Schreiblocks und ohne Lazy Fetch ausgeführt. Eine globale forensische Garantie zu Dateisystemmetadaten wird daraus nicht abgeleitet.

ROOT_CAUSE=UNRESOLVED  
IMPLEMENTATION_ALLOWED=NO  
BUILD_AUTHORIZED=NO  
HARDWARE_TEST_AUTHORIZED=NO  
PATCH_0008_SECOND_BOOT_ALLOWED=NO  
PUSH_AUTHORIZED=NO
