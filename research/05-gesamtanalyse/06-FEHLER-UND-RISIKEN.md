Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Kritische Selbstprüfung und künftige Risiken

## Bereits dokumentierte methodische Fehler

1. BELEGT: PDF-Textextraktion ordnete R11A als I2C-Pull-up ein; visuelle Primärprüfung korrigierte dies (`research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md`, Absatz „R11A is not an I2C pull-up“). Konsequenz: Schaltplantext ist Suchhilfe, Netzverbindungen brauchen visuelle/strukturierte Bestätigung. Keine Wiedereröffnung des abgeschlossenen I2C-Themas.
2. BELEGT: sichtbares Verschwinden von EAGAIN bei single-master war kein Nachweis verschwundenen Hardware-IAL (`docs/TEST-LOG.md:107`). Fehlerunterdrückung darf nicht mit funktionaler Behebung gleichgesetzt werden.
3. BELEGT: ursprüngliche Gate-Abfolge FAIL→Korrektur ohne erhaltene Primärausgabe wurde später nicht als bestätigte Durchführung fortgeführt (`docs/DECISIONS.md:867`). Rekonstruierter technischer PASS darf keine verlorene historische Befehlsfolge erfinden.
4. BELEGT: nicht übereinstimmender Debug-Toolchainkontext war kein Identitätsnachweis; erst Locked-Debug-SPL war byteidentisch (`docs/PROJECT-STATE.md:4454`). Gleichnamige Funktionen oder Quellversion genügen nicht zur Maschinenzuordnung.
5. BELEGT: Readback-Alignment und CRLF-Vergleich erzeugten Verfahrensfehler (`docs/PROJECT-STATE.md:4625`). Rohlogs müssen erhalten bleiben; Parser-Normalisierung ist ausdrücklich getrennt zu führen.
6. BELEGT als bekannte Grenze: Git-Sicherungscommit beweist keine Vor-Git-DTB-Herstellung und keine isolierte Wirkung einzelner Clockkorrekturen (`docs/GOLDEN-BUILD.md:71`; `docs/TEST-LOG.md:102`).

## Aktuelle Selbstkorrektur und Missverständnisrisiken

- MMC-Eintritt/-ENOMEM ist nachgelagerter Beobachtungspunkt. Die spätere Q1-Beobachtung liegt vor Heap-Clear und MMC-Allokation. Einen MMC-Fix ohne weitere Evidenz daraus abzuleiten wäre unzulässig (05; REF fsl_esdhc_imx.c:1322).
- Aktuelle feste DDR-Werte weichen von historischem SPD-Pfad ab (04). Daraus folgt weder, dass SPD das Problem sicher löst, noch, dass jeder Parameterunterschied eine Regression ist.
- WIDERSPRUCH: DDR-Kommentare stimmen bei RTT nicht mit Helferkodierung überein; WIDERSPRUCH: modify_dg_result-Kommentar 0x80 gegenüber Code 0xc0. Beides ist quellenbasiert dokumentiert, ohne eigenmächtige „Korrektur“ (04).
- Kalibrierungsfortgang ist kein PASS; selbst Rückgabemaske 0 erfasst ignorierte Timeouts nicht (research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md, Abschnitt Kalibrierungsbefunde).
- „DQS-Aufruf fehlt“ wäre bei Tail-Branch falsch; „Stack im DDR-Heap“ ist für den qualifizierten REF-Stand statisch widerlegt. Andere zukünftige Builds sind nicht damit qualifiziert (research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md, SRAM-Stack-Abschnitt).
- Konstanter Versionssuffix kann mehrere Patchstände bezeichnen (`boot/u-boot/default.nix`, extraConfig). Hardwarelogbanner muss durch SPL-Hash/Mediumzustand ergänzt werden.
- Katalog GEPRUEFT ist ein altes Prüfdatum, kein heutiger Onlinebeleg. Archivhash ist kein technischer Richtigkeitsbeweis (03/11).

## Zukunftsrisiken für das universelle Image

| Risiko | Status und Quelle | Auswirkung/Voraussetzung | Nächste diskriminierende Prüfung |
|---|---|---|---|
| Vollständiger P_EXT-Pfad offen | BELEGT als Nachweislücke, PROJECT-STATE Wiedereinstieg | V1-Eigenständigkeit noch nicht erreicht | zuerst frühe offene Speicherbeobachtung erklären |
| Abweichende SO-DIMMs/Ranks | HYPOTHESE, REF feste Geometrie vs. historische SPD | falsche Geometrie/Timing bei nicht qualifizierten Varianten | lokale Modul-/SPD-/Herstellervergleichsmatrix |
| Kleine RAM-Bestückung | HYPOTHESE, Script initrd bei 0x20000000 | Zieladresse außerhalb nutzbaren RAMs | vollständige Intervallrechnung je unterstützter Kapazität |
| Initrd-/Kernelwachstum | HYPOTHESE, feste Ladeadressen/P1 128 MiB | FAT-Platzmangel/Überlappung/Relocation | exakte Byte-/RAM-Budgetrechnung |
| Ladefehler im Script | HYPOTHESE, drei unbedingte fatload in image.nix | Boot mit ungeeigneten alten/teilgeladenen Bytes | Hush- und bootz-Fehlervertrag lokal prüfen |
| Mehrere geklonte Medien | HYPOTHESE, konstante Labels/UUID/Disk-ID | mehrdeutiger Root-Mount/Recovery | Stage1-Auswahl und persistente Pfade analysieren |
| Größere SDs | BELEGT initiale feste Partition; Nutzung weiterer Kapazität offen | freie physische Kapazität bleibt ungenutzt | Wachstumspfad in NixOS-Modulen prüfen |
| Kleinere SDs | HYPOTHESE aus aktueller Imagegröße | Image passt nicht; nominelle GB genügen nicht | Mindestkapazität in Bytes definieren |
| Größere Imagegenerationen | HYPOTHESE aus DOS/512-Sektorlayout | MBR-Feldgrenzen und Werkzeugannahmen | formale Größenlimits vor Veröffentlichung bestimmen |
| Kernel/DTB drift | HYPOTHESE, eigene Overlays mit numerischen IDs | Display, MMC, Versorgung fehlerhaft | Bindings und DTB-Quelle je Pin prüfen |
| Fehlende initrd-MMC/ext4-Treiber | QUELLENLUECKE aktueller Stage1-Inventur | Root-Mountfehler | tatsächliche config/Module im erhaltenen Store lesen |
| Wenig Rootfs-Reserve | INDIZ, ext4 shrink +16 MiB | Nix-Updates/Logs füllen Rootfs | Closure-/Freiraumbudget ohne neues fsck bestimmen |
| Nicht aktualisierte FAT-Bootdateien | HYPOTHESE, manuelles Script, noauto/nofail | Systemgeneration und Bootartefakte divergieren | Update-/Rollbackvertrag spezifizieren |
| PMIC-API-Fehler | HYPOTHESE, proper ignoriert einige Returns | undefinierter Registerwert/USB-Versorgung | API-/Codepfad, noch keine Hardwareintervention |
| Fehlerpfad ext4-Builder | INDIZ, errorlog/return ohne hier gesicherte Umgebung | schlechter Buildabbruch/fehlende Diagnose | gepinnten Builder-Shellkontext lesen |
| Reproduzierbarkeit über andere Hosts | NICHT_GEPRUEFT, G2X war Same-Host | neue hostabhängige Bytes möglich | spätere unabhängige Hostqualifikation planen |
| Recovery nach beschädigtem P1 | QUELLENLUECKE Releasevertrag | SPL kann proper nicht laden | lokales Recoverykonzept mit geschützten Referenzen |
| Standardzugang | BELEGT Konfiguration, configuration.nix initialPassword/openssh | veröffentlichte Images tragen bekannten Anfangszugang | vor Release Zugangskonzept festlegen |

Die letzte Zeile betrifft zukünftige Veröffentlichung und ist kein Nachweis eines erfolgten Zugriffs. Hardwarevarianten wie Bare Board/Desktop/Laptop/Heirloom, Senoko, Panel und PMIC sind getrennt zu qualifizieren; ein erfolgreicher Laptopdisplay-Test beweist keine Vollfunktion sämtlicher Varianten. `CONFIG_MX6Q` im REF ist ebenfalls kein Nachweis eines getesteten Dual/Solo-Builds.

## Dokumentationskonsistenz

README und BUILD enthalten ausdrücklich historische G2X-Referenzen. Alte „aktuell“-Absätze im PROJECT-STATE und TEST-LOG, frühe NOT_RUN-Stände und spätere 8B-Ergebnisse sind phasenbezogen. GOLDEN-BUILD enthält weiterhin historische offene I2C-Formulierungen; die Einordnung am Anfang und die spätere DECISIONS vom 20. September sind vorrangig. Nur Aussagen über denselben Versions-/Zeitbereich sind als echte Widersprüche zu behandeln.

Keine Risiken dieser Tabelle werden als beobachtete Ausfallursache des heutigen Falls 8B ausgegeben.
