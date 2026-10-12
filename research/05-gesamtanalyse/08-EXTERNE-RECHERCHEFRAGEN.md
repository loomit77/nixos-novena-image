Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Katalog für spätere externe Recherche

Kein Link wurde in diesem Block abgerufen. 12 enthält bekannte URLs mit lokalen Zeilenfundstellen; Erreichbarkeit, Inhalt und heutige Version sind NICHT_GEPRUEFT. Vor späterer Recherche zuerst den bereits vorhandenen lokalen Beleg nutzen. Gesucht werden Originalrevisionen, keine allgemeine Plausibilitätsbestätigung.

## NXP/i.MX6/MMDC

1. Welche Referenzmanualrevision gilt für das konkret dokumentierte i.MX6Q-Silicon? Welche DDR-, MMDC- und OCRAM-Kapitel sind maßgeblich?
2. Welche vollständige Originalbeschreibung hat ERR050070: betroffene Produkte/Masksets, Auslösebedingungen, Register, Workaround und Revision? Ist das reale Board betroffen?
3. Welche Lesesemantik haben MPDGHWST0–3: read-only, read-to-clear, Latchzustand oder zeitabhängige Daten? Sind wiederholte Reads in/außerhalb Kalibrierung zulässig?
4. Welche Felder bilden HW_DG_LOW/UP ab? Warum nennen vorhandene U-Boot-Kommentare -0x80, während Code -0xc0 benutzt? Herstellersequenz und Erratumrevision vergleichen.
5. Welche Wartebedingungen sind nach FIFO reset, CON_REQ/ACK, Dummy Write, WL und Gating zwingend? Welche Folgen hat Timeoutfortsetzung?
6. Wie werden CS/Bank/Row/Column/Byte-Lane aus MDCTL, MDASP, MDMISC, Busbreite und DIMMgeometrie bestimmt? Ist Interleaving relevant?
7. Welche konkreten RTT_NOM/RTT_WR-Registerkodierungen gelten; stimmen U-Boot-MR1/MR2-Rechnung und Kommentare überein?
8. Welche Initialisierungsdauer zwischen mx6_dram_cfg und Kalibrierung ist für Spannung, Takt und DIMM vorgeschrieben?
9. Welche DDR-Frequenzbegrenzung und PLL/MMDC-Clockbeziehung gilt für Quad vs. DualLite/Solo? Bausteinrating von tatsächlichem Takt trennen.
10. Welche ROM-IVT/Boot-Data/DCD/SD-Ladegrenzen gelten exakt; wie unterscheiden sich erste 4 KiB und gesamte SPL-Ladung?
11. Welche Reset-/Cache-/Storebuffer-Verträge gelten vor und nach ROM→SPL? Nur noch offene Fragen aus 4.10N aufnehmen, keine generische Barrierempfehlung.

## Kosagi/Novena/DDR3

12. Welche ausgelieferten SO-DIMMs, Ranks, Kapazitäten und Hersteller wurden je Bare Board/Desktop/Laptop/Heirloom unterstützt?
13. Existieren Original-SPD-Dumps und verbindliche Modul-Datenblätter des konkret bestückten DIMMs?
14. Warum wurde im Mainline-Novena-SPL feste Elpida-Geometrie statt SPD gewählt? Supportscope und bekannte Grenzen ermitteln.
15. Welche genauen Änderungen brachte Marek Vasuts Novena-Kalibrierungspatch 2015 und welcher Mainlinecommit entspricht der archivierten Mail?
16. Wie unterscheiden sich xobs rc5/rc7/rc12/rc14/bd5c7d3 und Mainline bezüglich Geometrie, RTT, page size, Rankhandling, CPU-RAM-Test und Fehlerreaktion?
17. Welche Gründe und Grenzen hatte canned SPD im xobs-Commit e08b7460f1? Wird auch der Ersatz selbst validiert und welche Module setzt er voraus?
18. Welche Hardware-ECOs betreffen DDR, Spannungsversorgung, POR, MMC und externe Bootwahl? EVT/DVT/PVT getrennt betrachten.
19. Welche Novena-PMIC-/Senoko-/EEPROM-Einstellungen sind für externen ROM-Boot nötig und welche gehören nur zum Display/Linux?
20. Gibt es öffentlich belegte ähnliche frühe SPL-Store/Load- oder DDR-Kalibrierungsfälle mit vollständigen Artefakt-/Registerdaten?

## U-Boot/SPL/MMC/SD

21. Welche Mainlinehistorie erklärt die aktuellen sysinfo/elpida-Werte und ihr Kommentarmismatch? Änderungen nicht nur nach Betreff prüfen.
22. Wurden ignorierte mmdc_do_*-/wait_for_bit-Rückgaben upstream diskutiert oder korrigiert? Versionsgenaue Fehlerverträge feststellen.
23. Welche Timer-/Watchdogannahmen gelten während 100-ms-Kalibrierungswaits im SPL?
24. Wie entstehen SPL dev0, Bootquelle MMC1, proper mmc1 und Linux mmcblk*-Nummern im gepinnten Stand?
25. Welche SD-/FAT32-/MBR-/Legacy-Imagegrenzen gelten bei den vorgesehenen Karten und Payloadgrößen?
26. Welche RAM-Adressen und Reservierungen benutzt proper vor bootz, inklusive LMB, Relocation, framebuffer und initrd_high?
27. Wie reagiert der konkrete Hush-Scriptpfad auf fehlgeschlagenes fatload und nachfolgendes bootz? Gibt es geschützte Standardmuster für Fehlerbehandlung?
28. Welche Auswirkungen haben fehlende PMIC-Read/Write-Rückgabeprüfungen auf Novena und wie behandelt upstream solche Fehler?
29. Wie robust ist ENV_IS_NOWHERE für das gewünschte Update-/Recoverykonzept; welche persistente Konfiguration wird damit bewusst aufgegeben?
30. Welche Grenzen müssen wachsende SPL-Wrapper, RAW-Sektoroptionen und FAT-Payloads gemeinsam einhalten?

## Barebox als Vergleich, nicht als beschlossener Wechsel

31. Gibt es einen aktuellen oder historischen Novena-Boardport, welche Revision und Hardwarevarianten unterstützt er?
32. Nutzt dessen DDR-Pfad feste Daten, SPD oder eigene Kalibrierung? Sind MMDC-Timeouts und Fehler dokumentiert?
33. Wie behandelt Barebox OCRAM/BSS, MMC-Laden und Device-Tree-Memoryfixups? Vergleich dient unabhängiger Quellenprüfung, nicht automatischem Bootloaderwechsel.
34. Gibt es bootfähige Originalartefakte mit genauer Provenienz und vollständigen POR-Logs? Fehlende lokale Vergleichsstände ausdrücklich bewahren.

## Linux/NixOS/Cross-Build/Image

35. Welche Treiber und firmware braucht die konkrete Stage1 für USDHC2/ext4/Labelmount? Aktuellen Pin von alten 5.7-Anleitungen trennen.
36. Wie wird Memorygröße im Novena-DTB von U-Boot aktualisiert und welche ARMv7-Highmem-/CMA-Grenzen gelten je RAMvariante?
37. Welche Bindings/Clock-IDs/Graphanforderungen haben IT6251, LDB und Panel im gepinnten Kernel und bei späteren Upgrades?
38. Welche NixOS-Module unterstützen initiales Rootwachstum und welche Bedingungen benötigen sie? Ist diese Funktion im aktuellen eigenen Image überhaupt aktiviert?
39. Welche dokumentierte Methode aktualisiert FAT-Bootdateien atomar mit NixOS-Generation/Closure und erhält Rollbackfähigkeit?
40. Welche bekannten Cross-Reproduzierbarkeitslücken verbleiben neben Bash pipesize, FAT-Metadaten und ext4-hash_seed?
41. Welche toolchain-/mkimage-/e2fsprogs-/mtools-Versionswechsel verändern Bytes oder Bootverhalten?
42. Welche Freiraum-/Mindestkartengrößen und Sicherheits-/Zugangsvorgaben sollte ein universelles veröffentlichbares Image besitzen?
43. Existiert das ursprüngliche novena-mmc-disk-r1.img mit verifizierbarer Hash-/Größenprovenienz? Webarchivmetadaten allein genügen nicht.
44. Wie lässt sich die historische NovM/NovS-/fstab-Divergenz anhand originaler Imagevarianten erklären, ohne Benutzerhandlungen zu erfinden?

## Rechercheprotokoll für einen späteren freigegebenen Auftrag

Je Antwort: Hersteller/Projekt, Original-URL, genaue Revision/Commit, Dokumentdatum, Abrufdatum, lokale Archivdatei, SHA256, zitierte Seite/Zeile, betroffene Hardware und Übertragungsgrenze festhalten. Spiegel dienen als Kopie mit Provenienzlücke. Negative Suchergebnisse sind keine Nichtexistenzbeweise. Neue Erkenntnis nur dann als Ursache einstufen, wenn sie die beobachtete konkrete Sequenz trägt und konkurrierende Erklärungen diskriminiert.
