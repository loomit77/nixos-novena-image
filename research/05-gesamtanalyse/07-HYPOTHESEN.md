Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.

# Forschungshypothesen und Unterscheidungsmöglichkeiten

Die Hypothesen sind eine thematische Sammlung aus den vorhandenen lokalen Belegen. Sie beanspruchen keine logisch vollständige Menge aller denkbaren physikalischen Ursachen. Exakte Primärfundstellen zu den Kurzverweisen stehen in 04, Rohbelege in 13 und Versionsbelege in 14. H09–H11 betreffen überwiegend zukünftige oder noch unerreichte Phasen. Bereits abgeschlossene ES8328-Tests und statisch ausgeschlossene einfache Erklärungen werden nicht als neue Pflichttests wiederholt.

## H01: Kalibrierungsfehler oder Timeout vor BSS

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** WL/DQS oder eine Wartephase scheitert, SPL setzt dennoch fort.
2. **Ausgangsbeobachtung:** 8B frühe Store/Load-Nullbeobachtung; keine Kalibrierungswerte im Log.
3. **Unterstützende Belege:** Board und Helfer ignorieren Rückgaben; 04/REF novena_spl.c:583 und ddr.c:25 ff.
4. **Widersprechende oder begrenzende Belege:** Kein gemessener Fehler; fortlaufender SPL und teilweise plausible start/end-Werte begrenzen einfache Totalausfallmodelle.
5. **Fehlende Informationen:** Reale Masken, Wait-Ergebnisse, Register und Resetbedingungen.
6. **Möglichkeiten zur Quellenprüfung:** Gepinnte Helfer, Wait-Zeitbasis und Herstellersequenz getrennt auswerten.
7. **Denkbare spätere Tests:** Später Minimalaufzeichnung vorhandener Rückgaben und Wait-Ergebnisse, ohne Retry/Fix.
8. **Voraussetzungen solcher Tests:** Neue konkrete Freigabe; BSS-freie Erhaltung, Layout-/Maschinencode-/Timingqualifikation.
9. **Risiken:** Instrumentierung verändert Stack, Code und Zeit; zusätzliche Registerlesungen nicht automatisch passiv.
10. **Erkenntnisgewinn:** Unterscheidet gemeldete Fehler, Timeouts und normale Kalibrierungsfortsetzung; noch kein Kausalitätsbeweis.

## H02: Geometrie/Timing passt nicht zum konkreten DIMM

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Feste Elpida-Konfiguration entspricht nicht vollständig der bestückten Modulvariante.
2. **Ausgangsbeobachtung:** Aktueller Pfad fest; historischer Pfad SPD.
3. **Unterstützende Belege:** Tabelle in 04; historische rank/row/col-Auswertung in rc5 ddr3.c.
4. **Widersprechende oder begrenzende Belege:** Historische Bootsoftware kann dasselbe Board betreiben; aktuelle DIMM-Daten fehlen. Kein belegter Parameterfehler.
5. **Fehlende Informationen:** SPD-Dump, Modulmarkierung, Spezifikation und echte MMDC-Werte.
6. **Möglichkeiten zur Quellenprüfung:** Vorhandene lokale Backups/Evidenz nach Modulidentität durchsuchen; feste Werte gegen konkrete Herstellerquelle prüfen.
7. **Denkbare spätere Tests:** Später rein lesende SPD-Erfassung oder separat qualifizierter Vergleichstest.
8. **Voraussetzungen solcher Tests:** Explizite Hardware-/I2C-Freigabe; identische POR-Bedingungen und Artefaktprovenienz.
9. **Risiken:** SPD-Fehler/Fallbackannahmen; falsches Update könnte Bootfähigkeit verlieren.
10. **Erkenntnisgewinn:** Grenzt Universalitätsbereich und mögliche Konfigurationsdifferenz ein.

## H03: Kalibrierungsalgorithmus oder Erratum

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Ein bedingter MMDC-Kalibrierungsfall beeinflusst Ergebnis.
2. **Ausgangsbeobachtung:** Code/Kommentar 0xc0 vs. 0x80; ERR050070 als offener Kandidat.
3. **Unterstützende Belege:** 04/modify_dg_result, 9C-44-Checkpoint.
4. **Widersprechende oder begrenzende Belege:** Kein Herstelleroriginal und keine reale Fehler-/Registerbeobachtung; reine Plausibilität.
5. **Fehlende Informationen:** Erratumrevision, Siliconanwendbarkeit, MPDGHWST-Felder und Lesesemantik.
6. **Möglichkeiten zur Quellenprüfung:** Zuerst lokale Herstellerbelege prüfen; dann externe Originalrevision nach gesondertem Auftrag.
7. **Denkbare spätere Tests:** Erst nach Quellenklärung Register-/Sequenzdiagnose, vorhandene gelesene Werte bevorzugen.
8. **Voraussetzungen solcher Tests:** Neue Freigabe; belegte Lesesemantik, Siliconidentität, statische Qualifikation.
9. **Risiken:** Read-to-clear/Timingeffekte unbekannt; Workaround könnte falsche Revision betreffen.
10. **Erkenntnisgewinn:** Trennt tatsächlichen Algorithmusfehler von Kommentarfehler und nicht anwendbarem Erratum.

## H04: Früher BSS-Clear beeinflusst späteren Zugriff

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** BSS-Clear oder vorausgehende DDR-Stores verändern einen noch unbekannten Speicherzustand.
2. **Ausgangsbeobachtung:** 8B erfolgt nach BSS-Clear; Q1 schon vor Heap-Clear Null.
3. **Unterstützende Belege:** REF crt0 und Checkpoint SRAM/BSS; physischer Zustand nicht gemessen.
4. **Widersprechende oder begrenzende Belege:** Logischer Clear enthält korrekt Target/Control; Heap-Clear überlappt nicht. BSS-Clear hat keine nachgewiesene Störung.
5. **Fehlende Informationen:** Phasenvergleich vor/nach Clear, CPU-/MMDC-Zustand.
6. **Möglichkeiten zur Quellenprüfung:** Genauen memset-Pfad und Adressen aus erhaltenem ELF prüfen, frühere 4.10N-Ausschlüsse beachten.
7. **Denkbare spätere Tests:** Später präregistrierter phasenbezogener Vergleich mit erhaltener SRAM-Diagnose.
8. **Voraussetzungen solcher Tests:** Konkrete Freigabe, Minimalintervention, keine funktionale Clear-Änderung als Diagnoseabkürzung.
9. **Risiken:** Neue Teststores können Zustand selbst verändern.
10. **Erkenntnisgewinn:** Kausalitätsfenster zwischen Kalibrierung und BSS/Allocator eingrenzen.

## H05: Physische Adressaliasierung oder eng begrenztes Speicherverhalten

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Logisch getrennte Adressen bilden wegen Geometrie/Interleaving ein unerwartetes Muster.
2. **Ausgangsbeobachtung:** Control und Target 12 Byte auseinander, beide Null.
3. **Unterstützende Belege:** 04 Adress-/Geometrietabelle; 8B.
4. **Widersprechende oder begrenzende Belege:** Zwei nahe Wörter liefern keine Bank/Row-Diversität; start/end-Werte sind nicht durchgängig Null.
5. **Fehlende Informationen:** Exakte physische Dekodierung aus MDCTL/MDASP/MDMISC und DIMM.
6. **Möglichkeiten zur Quellenprüfung:** Bitzuordnung zuerst mit qualifizierter Registerdokumentation herleiten.
7. **Denkbare spätere Tests:** Später definierte Adresse-/Musterprobe auf unabhängigen Bereichen.
8. **Voraussetzungen solcher Tests:** Hardwarefreigabe, geschützte Bereiche, präregistrierte Interpretation.
9. **Risiken:** Speichertests überschreiben Bootdaten; naive Adresswahl kann Belege zerstören.
10. **Erkenntnisgewinn:** Unterscheidet lokalen und breiteren Fehlerumfang ohne pauschalen DDR-Defekt.

## H06: CPU-/Store-Visibility-Mechanismus

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** CPU-/Memory-Systemzustand beeinflusst die unmittelbar zurückgelesenen Wörter.
2. **Ausgangsbeobachtung:** Q1/C1 lesen Null trotz statisch belegter Stores.
3. **Unterstützende Belege:** 7C/8B und Referenzmaschinenanalyse im 9C-44-Bericht.
4. **Widersprechende oder begrenzende Belege:** Frühere 4.10N-Untersuchung schließt einfache Cache-/Storebuffer-Erklärungen statisch ein; keine konkrete Barrierursache belegt.
5. **Fehlende Informationen:** Realer CP15-/Resetzustand am kritischen Punkt und Architekturvertrag.
6. **Möglichkeiten zur Quellenprüfung:** Vorhandene 4.10N-Ergebnisse vollständig gegen exakten REF abgleichen; kein generischer Barrierfix.
7. **Denkbare spätere Tests:** Nur bei neuer diskriminierender Evidenz präregistrierte Minimaldiagnose.
8. **Voraussetzungen solcher Tests:** Neue Freigabe; CPU-Quellen und Maschinenfolge geklärt.
9. **Risiken:** Barrier-/Cacheänderung wäre funktionale Intervention und könnte Symptom verdecken.
10. **Erkenntnisgewinn:** Prüft verbleibende Mechanismen ohne bereits statisch geschlossene Erklärungen zu wiederholen.

## H07: Instrumentierungs-/Layout-/Provenienzeffekt

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Beobachtung hängt von Diagnosecode oder falsch übertragener Artefaktqualifikation ab.
2. **Ausgangsbeobachtung:** Mehrere Diagnoseversionen haben gleiches Banner; Controladresse hartcodiert.
3. **Unterstützende Belege:** default.nix Localversion; Patch 0008; 02.
4. **Widersprechende oder begrenzende Belege:** REF-Hashes und bestehende Maschinenqualifikation stützen korrekte Zuordnung des 8B-Artefakts.
5. **Fehlende Informationen:** Fehlende ursprüngliche Gateausgaben, genaue Versionszuordnung aller frühen Logs.
6. **Möglichkeiten zur Quellenprüfung:** Patch-/Quellhashes und Byteketten prüfen; Druckreihenfolge vs. Ausführungsreihenfolge trennen.
7. **Denkbare spätere Tests:** Später kontrolliertes Diagnosevergleichspaar, wenn fachlich nötig.
8. **Voraussetzungen solcher Tests:** Konkrete Freigabe, identische Toolchain und je Variante eigene Layoutqualifikation.
9. **Risiken:** Unqualifizierte Globals können Control-Symbol verschieben; Timingänderung.
10. **Erkenntnisgewinn:** Stärkt Belegkette oder identifiziert Diagnoseartefakt statt Hardwaremechanismus.

## H08: Zustands-/POR-/Versorgungsabhängigkeit

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Unterschiedlicher elektrischer oder Reset-Ausgangszustand beeinflusst frühe Speicherfunktion.
2. **Ausgangsbeobachtung:** Historischer Pfad funktioniert, aktueller P_EXT-Pfad scheitert.
3. **Unterstützende Belege:** 05 Bootphasen und unterschiedliche Quellenstände.
4. **Widersprechende oder begrenzende Belege:** Kein kontrolliertes Paar; historische Clockberichte anderer Installer sind kein aktueller Nachweis.
5. **Fehlende Informationen:** Versorgung, Resetart, Batterie-/AC-Zustand, Temperatur und Module.
6. **Möglichkeiten zur Quellenprüfung:** Bestehende trial-/Testbeschreibungen nach vergleichbaren Bedingungen prüfen.
7. **Denkbare spätere Tests:** Später einzelner diskriminierender POR/Warmstart-Vergleich, keine automatische Bootserie.
8. **Voraussetzungen solcher Tests:** Explizite Anzahl und Freigabe, serielle Aufzeichnung vor Start, Artefakt-/Medienqualifikation.
9. **Risiken:** Warmstart übernimmt DDR-/Peripheriezustände; vermischte Bedingungen uninterpretierbar.
10. **Erkenntnisgewinn:** Trennt algorithmischen Fehler und Ausgangszustandsabhängigkeit.

## H09: Nachgelagerte SD-/FAT-/Payloadkompatibilität

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Nach Lösung der frühen Allokation kann MMC/FAT/proper-Laden zusätzlich scheitern.
2. **Ausgangsbeobachtung:** Aktueller Pfad hat diese Phase noch nicht erreicht.
3. **Unterstützende Belege:** 04 .config FS/FAT/Payload und eigene mmc-Enumeration.
4. **Widersprechende oder begrenzende Belege:** Aktuelles -ENOMEM liegt vorher; diese Hypothese erklärt C1/Q1 nicht.
5. **Fehlende Informationen:** Reale SD-Kommandos, FAT-Parser, Payloadheader/Größen.
6. **Möglichkeiten zur Quellenprüfung:** Lokalen Loaderpfad, Partition und Imageheader vollständig statisch verbinden.
7. **Denkbare spätere Tests:** Später genau freigegebener Boot nach vorgelagertem Gate.
8. **Voraussetzungen solcher Tests:** Frühe Ursache/Beobachtung getrennt; genaue Artefakte und neue Hardwarefreigabe.
9. **Risiken:** Mehrere Fehler könnten überlagert werden.
10. **Erkenntnisgewinn:** Qualifiziert noch offenen Rest des V1-Pfads.

## H10: Kernel-/initrd-/Rootauswahlproblem bei Varianten

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Aktuelle Stage1 findet bei bestimmter Hardware/mehreren Klonen falsches oder kein Root.
2. **Ausgangsbeobachtung:** Historischer unknown-block-Fehler; aktuelle feste Labels.
3. **Unterstützende Belege:** hardware/novena.nix; boot.cmd; 04.
4. **Widersprechende oder begrenzende Belege:** Externer Rootboot mit historischem U-Boot war erfolgreich; kein aktueller Stage1-FAIL im 8B-Lauf.
5. **Fehlende Informationen:** Tatsächliche initrd-Module, Mountvertrag, Geräteprioritäten.
6. **Möglichkeiten zur Quellenprüfung:** Gepinnte NixOS-Stage1 und erhaltene initrd-Inventur lesend prüfen.
7. **Denkbare spätere Tests:** Später definierter Varianten-/Mehrmedientest nach vollständigem P_EXT-Boot.
8. **Voraussetzungen solcher Tests:** Neue Freigabe und sichere Medienidentität; keine Datenträgerveränderung ohne Auftrag.
9. **Risiken:** Falscher Root-Mount, Labelduplikate, Versionsdrift.
10. **Erkenntnisgewinn:** Grenzt Universalität und vermeidet Wiederholung alter nicht vergleichbarer Rootfehler.

## H11: Wachstum/Update/Reproduzierbarkeit jenseits des Referenzstands

Status: HYPOTHESE; keine Ursacheneinstufung.

1. **Beschreibung:** Neue Closures oder andere Hosts überschreiten feste Platz-/Zeit-/Bootverträge.
2. **Ausgangsbeobachtung:** P1 fest 128 MiB, Root shrink+16 MiB, same-host Reproduzierbarkeit.
3. **Unterstützende Belege:** image.nix/ext4-Builder/Bashoverlay, 06.
4. **Widersprechende oder begrenzende Belege:** Aktueller G2X-Stand byteidentisch und dokumentiert passend.
5. **Fehlende Informationen:** Worst-case Initrd/Closure, Updateverfahren und weitere Hostvergleichsdaten.
6. **Möglichkeiten zur Quellenprüfung:** Budgets und gepinnte Build-/Updateverträge statisch rechnen.
7. **Denkbare spätere Tests:** Später getrennte Buildqualifikation und erst danach autorisierte Boot-/Updatequalifikation.
8. **Voraussetzungen solcher Tests:** Neue Build-/Testfreigaben, getrennte Provenienz pro Host.
9. **Risiken:** Platzmangel, alte FAT-Dateien, irreführende gleiche Storepfade.
10. **Erkenntnisgewinn:** Definiert verifizierbare Releasegrenzen statt unbelegter Universalitätsbehauptung.

## Architektur A/B als mögliche Diagnoseplanung

A bezeichnet laut überliefertem Chatbericht eine initialisierte lokale Struktur auf dem SRAM-Stack mit Weitergabe an WL/DQS-Helfer. B bezeichnet initialisierten globalen Zustand in SRAM-.data ohne Signaturänderung. Das sind NICHT_VERIFIZIERTE historische Chatzuordnungen, keine lokal belegten früheren Implementierungen. Optionaler Registermessumfang ist davon unabhängig. Quelldatum/Korrektur: docs/QUELLENVERZEICHNIS.md, SPL-007; 9C-44-Checkpoint Abschnitt Diagnosekonzepte. Beide Varianten benötigen neue Autorisierung und Qualifikation; keine Variante wird hier gebaut oder empfohlen, bevor Lesesemantik und Stack-/Linkreserve geklärt sind.
