# Historische U-Boot-DDR-Pfade und späterer Mainline-Unterschied

Stand: 2026-10-03. BLOCK 9C-46C. Versionsvergleich nur anhand tatsächlich gelesener Inhalte oder ausdrücklich begrenzter Git-Metadaten.

## Provenienz und Lesetiefe

U01: vorhandenes lokales Repository /home/loomit/novena-historical-research/u-boot-novena, Originalprojekt [xobs/u-boot-novena](https://github.com/xobs/u-boot-novena). Vollständig gelesen wurde bd5c7d3:board/kosagi/novena/ddr3.c; dazu tatsächlich gelesene RC-Dateien aus der lokalen Grundlage, erneute Hash-/Diffvergleiche, SPL-DDR-Aufrufstellen und common-DDR-Deltas. Kein Fetch, Checkout oder Build. Ein Git-Quellstand beweist nicht die vollständige Provenienz eines früher geschriebenen Bootmediums.

U02: REF-Buildbaum im Nix-Store; dessen primäre DDR-Helfer, Boardstruktur und wait_bit.h wurden gelesen. Gepinnte Mainline-Basis ece349ade2973e220f524ce59e59711cc919263f, v2026.07; lokales default.nix und Patches 0001–0008 bleiben eigenständige Provenienzbestandteile. Dies ist kein neu gebautes Artefakt.

U03: vorhandenes lokales Mainline-Repository /home/loomit/novena-research/u-boot-current. Metadaten und erreichbarer Git-Graph wurden gelesen. Mehrere historische Blobs fehlen im Partial Clone. GIT_NO_LAZY_FETCH=1 verhinderte deren Nachladen; Fehlermeldungen werden als Quellenlücke ausgewiesen. Vollständig gelesen wurde der Commit cec2f200… einschließlich Diff; nicht vollständig gelesen wurden die älteren unten genannten Commit-Diffs.

## Der ausdrücklich verlangte Commit 89d485940106c095ec69ecb12a78a42e096dece7

Titel: **arm: imx6: Enable DDR calibration on Novena**.

| Ereignis | Tatsächlich belegter Zeitpunkt/Status |
|---|---|
| Autor | Marek Vasut |
| Autorendatum | 16.12.2015 15:40:07 +0100, aus lokalem Commitobjekt |
| Mailinglistenveröffentlichung des Enable-Patches | Suchindex nennt 16.12.2015 06:41:38 -0800; Original-Mail nicht direkt erreichbar, daher INDIZ |
| Committer | Stefano Babic |
| Commitdatum | 02.02.2016 20:59:05 +0100, aus lokalem Commitobjekt |
| Geprüfte Releasegrenzen | Kein Vorfahr von v2016.01 oder v2016.03-rc1; Vorfahr von v2016.03-rc2 und v2016.03 |
| Vollständiger Commit-Diff | Abruf lokal scheitert an fehlendem Blob eb46265…; öffentliche Commit-/Patchseiten nicht erreichbar |
| Inhalt des Committexts | Aktivierung DDR-Kalibrierung für Novena-SO-DIMM und Anpassung anfänglicher DDR-Parameter |

Originalreferenzen: [Mainline-Commit](https://source.denx.de/u-boot/u-boot/-/commit/89d485940106c095ec69ecb12a78a42e096dece7), [GitHub-Projektkopie](https://github.com/u-boot/u-boot/commit/89d485940106c095ec69ecb12a78a42e096dece7), [ursprünglicher Mailinglistenverweis](https://lists.denx.de/pipermail/u-boot/2015-December/237993.html), [Suchindex des Enable-Patches](https://www.mail-archive.com/u-boot%40lists.denx.de/msg196451.html).

Die Zeitpunkte sind nicht austauschbar. v2016.03 ist der früheste hier durch Graphprüfung bestätigte stabile Release mit dem Commit, rc2 die entsprechende geprüfte RC-Grenze. Aus den Metadaten allein werden keine vollständigen konkreten Initialparameter des damaligen Diffs rekonstruiert.

## Erreichte originale Reviewdiskussion U04

Tatsächlich gelesen: [Marek Vasut, Antwort vom 22.12.2015 02:52:02 CET auf „Add DDR3 calibration code for MX6 Q/D/DL“](https://lists.denx.de/pipermail/u-boot/2015-December/238954.html), einschließlich der zitierten Reviewpassagen, Werkzeugzeilen 10–400.

**BELEGT als Diskussionsinhalt:** Die Teilnehmer diskutieren AN4467 als Referenz, CON_ACK/tRP-Abfolge, festes MR1=0x4, das 0x001f001f-Kriterium, Dummy-Readback sowie Übergang zu gemeinsamem wait_for_bit. Der zitierte Entwurf beendet fehlgeschlagene kurze Polls mit hang(). Bereits dort stehen Kommentar 0x80 und Code 0xc0 nebeneinander. Reviewfragen sind keine bestätigten Hardwarefehler oder Belege, dass alle Vorschläge später integriert wurden.

Diese Mail betrifft den gemeinsamen Helfer, nicht den vollständigen Diff des Enable-Novena-Commits. Das Zitieren früheren Patchcodes qualifiziert nur die darin tatsächlich enthaltenen Abschnitte. Herstellerverweise in einer Entwicklerantwort werden nicht als selbst gelesene Herstellerseiten verbucht.

## Tatsächlich verglichene Novena-Versionen

| Version | Commit | DDR3-Datei SHA-256 / Lesebeziehung |
|---|---|---|
| v2014.10-novena-rc5 | 9402489e9a3e52a9df5464b4ce3c792cee2ba3a5 | f75834972afdb596cf885db4d97c69623cb9dc28738dc9cc1615d45e1bacc945 |
| v2014.10-novena-rc7 | 4330c957155183eec60537071cc595d39d9b5872 | identisch zu rc5; keine separate inhaltliche Neuerung |
| v2014.10-novena-rc12 | c5efeadb913c8246be372054adcd6106ad4fe067 | 142afc7acb12992208fb78f1d058d91c12ad00eec33daef0761b280fa1183160; tatsächlicher Delta-Vergleich zu rc5 |
| v2014.10-novena-rc14 | 3d26413e65f985131a9573d99a51fd47e7253188 | DDR3-Datei identisch zu rc12; SPL-Datei nicht identisch |
| bd5c7d3 / v2014.10.r7-novena | bd5c7d3290bb94bf8af17760c321df67d2641343 | 3453a653e08e2f30a1132dcc8eb5c85a16eb03be678d1426d9e50282bb31c0fc; 986 Zeilen vollständig gelesen |

Die common-DDR-Datei ist in rc5/rc7/rc12/rc14 identisch: dc1ce26de2d1dade756f3d7841d3960fa27f09ed1181176c531560307ea772c0. bd5c7d3: 91288b6bbcf24dfd5ad3064d118af7226639666a6f71a1f5843e01afeaa2742a; gelesener Delta fügt MX6SX-Unterstützung hinzu, kein belegter Wechsel der MX6Q-Taktberechnung.

Die Identität der DDR-Datei erlaubt den Vergleich dieses Inhalts, nicht die Gleichsetzung ganzer Releaseartefakte oder Boards. Nicht gelesene RC-Versionen werden nicht bewertet.

## SPD, Ersatz-SPD und Geometrie

| Aspekt | rc5/rc7 | rc12/rc14 | bd5c7d3 | aktueller REF |
|---|---|---|---|---|
| Konfigurationsgrundlage | SPD | SPD | SPD oder Ersatz-SPD | feste elpida_4gib_1600-Struktur |
| SPD-Leseweg | I2C, 0x50, Startoffset 0, einbyteige Adresse | wie links | ddr3.c:812–820 | kein SPD-Konfigurationsaufruf im gelesenen Boardpfad |
| Fehler bei SPD-Lesen | fataler Pfad | weiterhin fatal | Ersatzarray, Zeilen 821–825 | nicht dieser Mechanismus |
| CRC-/SPD-Prüfung | nicht wie in bd5 vorhanden | nicht wie in bd5 vorhanden | ddr3_spd_check; bei Fehler Ersatzarray, 827–830 | nicht dieser Mechanismus |
| Seitengröße | pagesz=1 | tck_min-/tfaw_min-Auswahl ergänzt | entsprechender Auswahlbaum | pagesz=2 fest |
| Ranks/row/col/chip width | aus SPD | aus SPD | Zeilen 907–936 | ncs=1, row=16, col=10; cfg.width=64 |
| Bank-Interleaving | aus diesem Pfad deaktiviert | ebenso | bi_on=0 | bi_on=1 |
| RTT / Zusatzlatenzen | historischer SPD-Pfad | gleicher Kern | rtt_nom=2, rtt_wr=2, ralat=3, walat=3 | rtt_nom=1, rtt_wr=0, ralat=5, walat=0 |

bd5 liest sizeof(spd), setzt im Funktionskörper keinen I2C-Bus explizit und initialisiert I2C über CONFIG_SYS_I2C_SPEED. Das beweist keinen erfolgreich gelesenen realen SPD im historischen Boot. Buszuordnung wird nicht aus Adresse 0x50 erraten. Der bereits abgeschlossene I2C3-/ES8328-Strang wird nicht erneut geöffnet.

Das gelesene Ersatzarray board/kosagi/novena/spd.h enthält eingebetteten ASCII-Modultext S9905469-145.A00LF. Dieser Text identifiziert das Ersatzarray, **nicht** das aktuell eingesetzte Modul. Nach CRC-Fehler und Ersetzung erfolgt in der gelesenen Funktion keine zweite ddr3_spd_check-Prüfung. Ein allgemeiner Nachweis, dass der Ersatz-SPD zum bestückten DIMM passt, fehlt.

bd5 akzeptiert im Code nur bus_width=0x03, also den dort als 64 Bit ohne ECC beschriebenen Fall, fordert acht Banken und berechnet Rankzahl, Zeilen/Spalten und Kapazität aus SPD-Feldern. Nicht jede vom Bitfeld darstellbare Geometrie ist damit hersteller- oder boardqualifiziert. Insbesondere ist die Festkonfiguration im REF keine automatische Modulerkennung.

## Timingberechnung und historische Änderungen

bd5: MTB-Skalierung ist ganzzahlig (mtb_dividend*100)/mtb_divisor. tRCD, tRC und tRAS werden aus SPD-Zahlen jeweils mit zusätzlichem +1 und anschließendem Skalierungsfaktor abgeleitet. Feinzeitoffsets werden in dieser gelesenen Funktion nicht ausgewertet; eine Division-durch-null-Abwehr ist dort nicht sichtbar. Daraus wird kein tatsächlicher fehlerhafter SPD und kein aktueller Timingfehler abgeleitet.

Der tck_min-/tfaw_min-Auswahlbaum erkennt mehrere Raten. Bei unbekannter Seitengröße wird eine Debugmeldung ausgegeben; bei zu langsamer Rate folgt dram_fatal. Diese Implementierung ist nicht mit einer vollständigen JEDEC-SPD-Decoderqualifikation gleichzusetzen. Das Hersteller-SPD-Dokument des aktuellen Moduls fehlt.

Lokale Commitbelege:

- 2ab1217f8c70cc266f0157bf9baec86e34e56805, Sean Cross, Autor/Commit 17.10.2014 17:56:14 +0800: Seitengrößendekodierung; gelesener Delta zwischen RC-Gruppen.
- e08b7460f140010442c7ec4707cd2c93c6b44b38, Sean Cross, Autor 18.05.2015 19:09:17 +0800, Commit 19:09:42 +0800: Ersatz-SPD bei Lesefehler.
- bd5c7d3290bb94bf8af17760c321df67d2641343, Sean Cross, Autor/Commit 04.09.2015 10:08:38 +0800: Debian-Versionsänderung; kein eigenständiger DDR-Fix in diesem letzten Commit.

## Kalibrierung, Waits und Fehlerfortsetzung

Historisches bd5 board_init_f: SPD → IO-Konfiguration → mx6_dram_cfg → do_write_level_calibration → do_dqs_calibration → Speichertest → BSS-Clear → board_init_r. Zeilen 690–706. Beide Kalibrierungsrückgaben werden ignoriert. Der Speichertestfehler wird gedruckt, danach geht der Code weiter.

Die historischen Pollschleifen sind unbeschränkt, anders als REF mit 100-ms-Aufrufen. Der zitierte Mainline-Entwurf U04 hat wiederum eine begrenzte Schleife mit fatalem Timeout. Diese drei Zustände dürfen nicht gleichgesetzt werden.

WL in bd5 prüft MPWLGCR & 0xf00 in beiden PHYs, verwendet die gemeinsame 0x001f001f-Soft-Fail-Erkennung und stellt ursprüngliche Werte bei diesem Sonderfall wieder her. Der explizite MPWLDECTRL-Readback liegt außerhalb DEBUG. Er belegt eine bewusste Implementierungsentscheidung, keine allgemeine Readback-/Barrierpflicht des Herstellers.

Historisches WL maximiert RALAT/WALAT ohne entsprechende ursprüngliche Rücksicherung in dieser Funktion. DQS sichert MDMISC erst anschließend; in bd5 ddr3.c:739–740 wird die Rückschreibung ausdrücklich an PHY1 ausgeführt. REF sichert/restauriert MDMISC im WL-Pfad und schreibt im DQS-Pfad an PHY0, obwohl der dortige Kommentar weiter MMDCP1 nennt. Das sind konkrete Quellunterschiede und ein Code-/Kommentarwiderspruch; ohne MMDC-Vertrag kein bewiesener Defekt.

Der historische CPU-RAM-Test umfasst durch Zähler und 16-Bit-Zugriff den Bereich [0x10000000,0x10040000), 256 KiB, mit Summenvergleich. Er umfasst nicht den aktuellen Diagnosebereich um 0x18200030/0x18300000 und ist keine flächendeckende DDR-Qualifikation. Diese Quellfeststellung ist jetzt auch für bd5 durch vollständig gelesene ddr3.c belegt; ein tatsächlich ausgeführtes historisches Binärverhalten ist damit nicht automatisch bewiesen.

## Neuer späterer Mainline-Commit U03

[cec2f200b4bfcc466e4a83196ed5fdd54678c1c7](https://source.denx.de/u-boot/u-boot/-/commit/cec2f200b4bfcc466e4a83196ed5fdd54678c1c7)

Titel: **mx6: ddr: Subtract half a cycle instead of three quarters of a cycle after DQS gating calibration**.

- Autor Marek Vasut, 25.06.2026 16:17:29 +0200.
- Committer Fabio Estevam, 27.07.2026 11:27:19 -0300.
- Vollständiger Committext und Diff lokal gelesen.
- Zwei Subtraktionen in modify_dg_result: 0xc0 → 0x80; sonstige Masken/Aufteilung im gelesenen Diff unverändert.
- Kein Vorfahr von v2026.07 oder v2026.10-rc1; enthalten in den lokal vorhandenen Tags v2026.10-rc2, rc3 und rc4.
- Kein Nachweis in diesem Auftrag für einen endgültigen v2026.10-Release. Keine Aktualisierung des Projektpins.

**BELEGT als Autorenaussage:** Der Commit begründet die Änderung mit unterschiedlichen Herstellerdokumentständen und berichtet Instabilität mit Winbond W634GU6RB. **QUELLENLUECKE:** Die genannten AN4467-/RM-Originalstellen und Tool 3.0.0 wurden nicht direkt gelesen. **HYPOTHESE:** Eine Übertragung auf den aktuellen Novena-DIMM oder dessen Fehler ist nicht belegt.

Die Suchindex-Mail eines Testers vom 03.07.2026 ist als direkter Abruf gescheitert. Ihr Bericht darf nicht als in diesem Auftrag geprüfte Novena-Hardwarequalifikation verbucht werden. Der lokale Tested-by-Eintrag ist Commitmetadatum, keine erhaltene eigene Rohmessung.

Der REF enthält noch die 0xc0-Operation. Damit wird die pauschale Annahme eingeschränkt, der Projektpin enthalte bereits jede spätere Mainline-Kalibrierungskorrektur. Gleichzeitig bleibt ein Herstellervergleich nötig, bevor aus der Änderung ein Novena-Fix abgeleitet würde.

## Weitere historische Commitobjekte: begrenzte Aussage

Die Metadaten von d339f169… (Helfereinführung), b10d93ee… (gemeinsames wait_for_bit), 14eeb683… (WL-Korrektur), 09dbac817… (Latenzrücksicherung), aa428944… (Wartephase vor erstem MRS), 6b5ecb829… (ODT) und 7d29acd9… (Novena DCD zu SPL) sind lokal vorhanden. Vollständige Diffs scheiterten an fehlenden Promisor-Blobs. Sie dienen als Recherchepfade, nicht als tatsächlich gelesene Zwischenimplementierungen. Kein stillschweigendes Auffüllen aus Modellwissen.

## Konsequenz

H01/H02/H03 erhalten klarere Quellenbedingungen. Kalibrierungsrückgabe, tatsächlicher Timeout, reales DIMM und Versorgung bleiben unbeobachtet. Historische Softwareunterschiede erklären allein weder den aktuellen Store/Load-Befund noch eine Root Cause. Eine Implementierung, ein Update oder ein weiterer Boot ist nicht Teil dieses Auftrags.

ROOT_CAUSE=UNRESOLVED  
IMPLEMENTATION_ALLOWED=NO  
BUILD_AUTHORIZED=NO  
HARDWARE_TEST_AUTHORIZED=NO  
PATCH_0008_SECOND_BOOT_ALLOWED=NO  
PUSH_AUTHORIZED=NO
