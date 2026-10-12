Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Offene Fragen und Entscheidungsbedarf

| Frage | Status | Fundstelle | Benötigte Evidenz / Priorität |
|---|---|---|---|
| Warum lesen C1/Q1 Null? | HYPOTHESE offen, ROOT_CAUSE=UNRESOLVED | 8B-Rohlog in 13, 04/05 | diskriminierende Phasen-/Konfigurationsbelege; P1 |
| Welche WL/DQS-Masken und Wait-Ergebnisse traten real auf? | QUELLENLUECKE | REF novena_spl.c:583–584, DDR-Waits | vorhandene Messwerte fehlen; neuer Test hier verboten |
| Welches DIMM mit welcher SPD/Rank-/Timingstruktur ist verbaut? | QUELLENLUECKE dieser Gesamtanalyse | 04 feste Elpida vs. historischer SPD | bestehende Modul-/SPD-Belege suchen |
| Ist ERR050070 anwendbar? | NICHT_VERIFIZIERT | Quellenkatalog SPL-006 | Herstelleroriginal, Revision, Silicon, Bedingungen |
| Sind MPDGHWST-Zusatzlesungen nebenwirkungsfrei? | NICHT_VERIFIZIERT | 04 modify_dg_result; 9C-44 | verbindliche Registersemantik |
| Warum Code -0xc0 bei Kommentar -0x80? | WIDERSPRUCH Text/Code | REF ddr.c:62 ff. | Commit-/Herstellersequenz, keine automatische Korrektur |
| Stimmen RTT-Kommentare mit Kodierung? | WIDERSPRUCH | REF novena_spl.c sysinfo / ddr.c Termination/MR1/MR2 | Zahlen/Kommentare/Herstellerdekodierung trennen |
| Wo sind Originalberichte 9C-33–42 und Gateausgaben? | QUELLENLUECKE | 9C-44, 03 | Originale lokal auffinden oder Lücke bewahren |
| Sind A/B historisch originalbelegt? | NICHT_VERIFIZIERT | SPL-007 und 07 | Chatoriginal/Repositorybericht fehlt |
| Welche physische CS/Bank/Row/Column-Zuordnung haben Testadressen? | QUELLENLUECKE | 04, 05 | reale Register/Geometrie + Manual |
| Welchen Einfluss hat C0-Vorablesung? | HYPOTHESE | 05 | präregistriertes diskriminierendes Design |
| Welcher Fehler bleibt nach früher Allokationsphase im MMC-Lader? | NICHT_GEPRUEFT hardwareseitig | REF spl_mmc.c; 04 | statischer Restpfad, später autorisierter Boot |
| Reicht aktuelles bootz/Legacy-initrd-Verhalten? | NICHT_GEPRUEFT unter aktuellem P_EXT | image.nix bootCmd/initrdUimg | Parser-/Headervertrag und reale proper-Beobachtung |
| Ist aktuelle initrd-Modulliste vollständig? | NICHT_GEPRUEFT | 04 Rootpfad | genau zugeordneter Stage1-/Modulbestand |
| Für welche RAM-/Boardvarianten gilt V1? | offener Supportumfang | PROJECT-STATE Definition universell, 06 | begrenzte Matrix mit eigener Evidenz je Variante |
| Wie erfolgen Updates/Rollbacks der FAT-Dateien? | QUELLENLUECKE | hardware.nix noauto, eigener image.nix | atomarer Generations-/Recoveryvertrag |
| Wie viel Root-/FAT-Freiraum wird benötigt? | HYPOTHESE Risiko | ext4 +16 MiB, FAT 128 MiB | Größenbudget und Updatebedarf |
| Wie werden geklonte Labels/UUIDs eindeutig gewählt? | HYPOTHESE Risiko | feste Labels/UUID in hardware/image | Stage1-/Recoveryauswahl |
| Ist Byteidentität auf anderem Host belegt? | NICHT_GEPRUEFT | G2X Same-Host-Bericht | später gesonderter Hostvergleich, keine L14-Vermischung |
| Woher stammt der historische Hybridzustand? | QUELLENLUECKE | HISTORICAL-MEDIA Hybridzustand | Originalimagevarianten/Transformationsevidenz |
| Ist r1 bytegenau lokal vorhanden? | NICHT_VERIFIZIERT | HISTORICAL-MEDIA r1-Retrievability | Originalhash, Größe, qualifizierte Kopie |
| Wie belastbar ist der Barebox-Vergleich? | QUELLENLUECKE lokaler Vergleich | 08 Fragen 31–34 | konkreter Port/Commit/Supportumfang |

## Bewusst geschlossene Fragen

Die dauerhafte ES8328-Always-on-Anforderung ist entschieden. Einzelne historische analoge Details sind kein Grund, H3-1R oder single-master erneut zu testen. Frühere statisch geschlossene einfache Erklärungen (fehlender Store, falscher Targetzeiger, gewöhnliche Heap-BSS-Überlappung, DDR-Heapstack im REF) werden nur bei tatsächlich neuen abweichenden Artefakten wieder geprüft.

## Fortgeltende Grenzen

Offene Frage ist keine Aktionsfreigabe. Fall 8B bleibt einmaliger historischer POR/P_EXT-Lauf. Ein zweiter Patch-0008-Boot ist nicht erlaubt. Keine Fehlerbehebung wurde implementiert, keine Root Cause festgelegt und kein vollständiger aktueller P_EXT-Gesamtboot behauptet.
