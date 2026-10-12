Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Testlogs: Chronologie, Belegstärke und Aussagegrenzen

## Verfahren

Alle 16 vorbestehenden Dateien unter test-logs sind in 13 mit Größe, Hash, Textzeilen und Herkunftsgrenze erfasst. Die sechs P_EXT-Seriellogs vom 22./23. September und das Fall-8B-Log wurden vollständig gelesen. Ein Dateidatum oder Verzeichnisdatum ist kein Nachweis des realen Startzeitpunkts. Erhaltene Entwürfe und vor-8e-Dokumentkopien sind keine weiteren Bootversuche.

Die Evidenzserie vom 15. September enthält vollständige dmesg-Dateien, trial-Angaben und ergänzende Zustände. Alle 79 manifestierten Evidence-Dateien wurden lokal hashgeprüft, ohne neue Tests. Die Signaturauswertung in 13 ist Suche in bestehenden vollständigen Logs; sie ersetzt keine elektrische Messung.

## Frühe Fehler und erfolgreiche Displayzustände

Der ältere 5.7.0-rc2-Rootfehler ist nur im Projektbericht erhalten (`docs/TEST-LOG.md:5`). Es fehlt hier die bytequalifizierte vollständige damalige Boot-/Artefaktkette. Deshalb keine rückwirkende genaue Root-Cause-Zuordnung.

Der erfolgreiche 12.-September-Lauf wird im Testbericht mit Linux-seitigem Display-Power-up, Product-ID und Link beschrieben (`docs/TEST-LOG.md:190`). Spätere Cold-FAILs zeigen, dass ein einzelner Erfolg kein deterministischer POR-Nachweis ist. Same-Boot-Rebind und echter POR dürfen nicht als gleiche Ausgangsbedingungen gezählt werden.

Historische Instrumentierung verändert Timing und Fehlerpfade: `single-master` überspringt IAL-Prüfungen, löst aber nicht nachweislich das Hardwareereignis. Erwartete Reset-NACKs sind vom untersuchten initialen START-Fehler zu trennen. Chronologie und Versionszuordnung stehen in `docs/TEST-LOG.md:642` und dem 0006-Abschnitt.

## H3-1 und H3-1R: abgeschlossene Untersuchung

BELEGT in archivierten Logs: Baseline zeigt Audio-Regulatorabschaltung und spätere START-FAIL-Signatur, erfolgreicher Rebind einen anderen Post-MSTA-Zustand. H3-1R änderte nach dokumentierter DT-Qualifikation nur Always-on am Audio-Regulator; fünf präregistrierte POR-Läufe bestanden (`docs/PROJECT-STATE.md:2164`; `evidence/block-3.10-h3-1r-por-series-2026-09-15/README.md:1`).

Die lokale Prüfung bestätigt die erhaltenen manifestierten Bytes und erwarteten IT6251-/I2C-Signaturen; sie zählt keine neuen Boots. GPIO-/Regulatorsnapshots nach dem Boot sind keine Analogmessungen im START-Fenster. Historische Leakagewerte 10 mA/1 V werden nicht als heutige PVT2-Messung behandelt. Funktionaler Abschluss bleibt maßgeblich; keine Wiederholung dieser Serie wird empfohlen.

## Reproduzierbarkeit

G2X nach Bashfix wird als erfolgreicher Same-Host-/Separate-Store-Bytevergleich dokumentiert (`docs/TEST-LOG.md:1424`). Die heute erhaltenen Full-Image-Buildlogs vom 30. September gehören zu einem späteren Diagnoseimage. Sie erweitern den früheren Reproduzierbarkeitsnachweis nicht automatisch. Erfolgsmarker in einem Buildlog beweisen Buildabschluss, keinen Hardwareboot; Logvollständigkeit/Ergebnisdateiidentität braucht unabhängige Zuordnung.

## P_EXT-Chronologie

| Reihenfolge | Primärdatei | Beobachtung | Getragene Eingrenzung |
|---|---|---|---|
| 1 | test-logs/p-ext-2026-09-22/p-ext-boot-01.log | SPL v2026.07, Trying to boot from MMC1; später historischer 2015-Boot und Linux im selben Text | Segmente/Recovery trennen; nicht als aktueller proper-Erfolg lesen |
| 2 | test-logs/p-ext-2026-09-22/p-ext-diag-boot-01.log | Diagnose erreicht D0 | Ende am Such-/Initialisierungseinstieg; keine MMC-Root-Cause |
| 3 | test-logs/p-ext-2026-09-23/p-ext-d0-d36-boot-01.log | D21 erreicht, D32/D33 -12 | Allokationskorridor vor D22, noch nicht priv/plat unterschieden |
| 4 | test-logs/p-ext-2026-09-23/p-ext-d21a-d21d-boot-01.log | D21A brk=0, D21B, kein D21C | In Verbindung mit Code erste priv-Allokation gescheitert |
| 5 | test-logs/p-ext-2026-09-23/p-ext-p0-p1-boot-01.log | P0/P1=0 | brk spätestens nach mem_malloc_init inkonsistent |
| 6 | test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log | Q1/Q2/P0=0 | 7C, erste Nullbeobachtung schon vor Heap-Clear |
| 7 | test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log | C0/C1/C2/Q1/Q2/P0/P1=0 | 8B, zwei nahe Kontroll-/Targetbeobachtungen |

Die Reihenfolge wird durch `docs/TEST-LOG.md`/Präregistrierungen gestützt; identische Banner können unterschiedliche Patchstände verbergen. Das erste längere Log enthält ausdrücklich SPL/proper 2015, POR-Meldung, NixOS-Kernel und Login-Kontext. Diese Fortsetzung gehört zum historischen internen/Recovery-Pfad, nicht zum erfolgreichen Übergang aus dem eingangs gezeigten aktuellen SPL. Der genaue Ablauf stammt aus der Testdokumentation (`docs/TEST-LOG.md:1478`).

## Artefakte und Medienbedingungen

Patch-0008-Referenz: ELF 69bb3a4…; SPL 4611111…; vollständiges Diagnoseimage laut Bericht 2581291008 Byte und SHA256 `90405b094423b8e515d670f3a72cb3146c68d0ad5f65d5d71d88a965966f0fe3`. Das ältere G2X-Image hat dagegen 8a2b8ac… . Gleiche Größe beweist keine Identität (`docs/PROJECT-STATE.md:5660`; 02).

Vor 8B dokumentiert: vollständiger Imagewrite und exakter Full-Range-Readback, ungemountetes Medium, P_EXT, echter POR, seriell 115200/8N1. Die Aussage Full-Range gilt nur für 2581291008 Byte, nicht den übrigen Bereich der größeren SD. Die heutige Analyse greift weder auf das Medium zu noch behauptet sie eine neue Readbackprüfung.

Dokumentierte frühere Verfahrensprobleme: `bs=1`/direct-Readback scheiterte am Verfahren; nach Sektorausrichtung gelöst, kein zweiter Write. CRLF führte zu einem falschen Stringvergleich; nur Vergleichsvariablen wurden normalisiert. Daraus folgt kein HardwareFAIL (`docs/PROJECT-STATE.md:4609`).

## Fall 8B im Detail

Rohdatei: 830 Byte, SHA256 `9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61`, vollständig in 13 erhalten. BELEGT: C0=C2=0 erfüllt die ursprüngliche Gültigkeitsbedingung. C1=Q1=0 führt nach Präregistrierung zu CONTROL_FAIL_TARGET_FAIL (`docs/DECISIONS.md:801`).

Die textuelle Q1/Q2-Ausgabe erscheint vor C0/C1/C2, obwohl die Kontrollsequenz zuvor ausgeführt wird: Ausgabeplatzierung nach mem_malloc_init erklärt diese Reihenfolge. Print-Reihenfolge darf nicht als Reihenfolge sämtlicher Speicheroperationen gelesen werden (`boot/u-boot/0008-novena-diag-control-c0-c2.patch:1`).

D21A meldet start=18300000/end=18400000/brk=0. D21B `priv=?` ist kein numerisch qualifizierter Pointer. Der Code-/Markerpfad stützt NULL/ENOMEM; das Fragezeichen selbst beweist NULL nicht. Nach D33 endet das erhaltene Log; keine belegte CPU-Haltinstruktion, kein gemessener Reset und kein vollständiger Langzeitverlauf.

Ausschlussgrenze: Eine ausschließlich auf das Targetwort beschränkte einfache Erklärung wird geschwächt. Zwei 12 Byte entfernte Adressen sind keine unabhängigen Tests über DRAM-Bänke, Ranks oder Byte-Lanes. C0 kann vorkonditionieren. Keine SPD-Dumps, tatsächlichen Kalibrierungsmasken, Wait-Ergebnisse, Registerstände oder analogen DDR-Wellenformen sind in diesen 830 Bytes enthalten. Damit bleiben DDR, MMDC, Cache, BSS-Clear und weitere Mechanismen Hypothesen.

## Fehlende Messwerte und offene Vergleichbarkeit

Keine kontrollierte Paarserie aktueller vs. historischer SPL mit gleichem DIMM, gleichen POR-Bedingungen und gleicher Minimalinstrumentierung. Kein unabhängiger Registersnapshot an den kritischen Phasen. Keine vorliegende genaue CS/Bank/Row/Column-Dekodierung. Diagnosepatches verändern Codegröße, Layout und Zeit. Die Full-Image-Qualifikation vor 8B verbessert Artefaktkonsistenz, macht aber frühere Diagnosevarianten nicht identisch.
