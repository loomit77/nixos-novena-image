Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Quellenverzeichnis-Audit

Grundlage: vollständiges `docs/QUELLENVERZEICHNIS.md` (71 Zeilen im Ausgangsstand). Die folgenden Statusangaben ersetzen keine Onlineprüfung. Alle bekannten URL-Fundstellen aus dem untersuchten Textbestand stehen in 12; jeder archivierte Forschungspfad mit SHA256 und Manifeststatus in 11.

## Einzelprüfung der Katalog-IDs

| ID | Herkunft/Original | Lokaler Beleg und bisherige Verwendung | Bewertung und offene Verifikation |
|---|---|---|---|
| UB-001 | Offizielles U-Boot-GitHub | Gepinnte Revision in default.nix und REF; aktuelle SPL-Basis | LOKALE_KOPIE/BELEGT für REF, URL heute NICHT_GEPRUEFT; Zuordnung einzelner historischer Änderungen braucht Git-Objekte |
| UB-002 | U-Boot-Memory-Dokumentation latest | Katalog und vorhandene SPL-Rekonstruktion | SEKUNDAERQUELLE für allgemeine Einordnung; lokal archivierter Volltext nicht nachgewiesen; latest nicht automatisch v2026.07 |
| UB-003 | Kconfig main | REF/.config und lokales Kconfig als besserer Versionsbeleg | LOKALE_KOPIE für REF; main-URL kein Nachweis des konkreten Builddefaults |
| UB-004 | ARM-software-Spiegel malloc.h | REF/include/malloc.h, common/malloc_simple.c, common/dlmalloc.c | Spiegelzuordnung NICHT_VERIFIZIERT; gepinnte lokale Header maßgeblich |
| UB-005 | offizieller Git-Endpunkt | URL bekannt | NICHT_VERIFIZIERT hinsichtlich heutiger Erreichbarkeit; kein Fetch |
| NV-001 | Marek Vasut DDR-Patch 2015 | Katalog nennt Onlineeinsicht 2026-10-01; lokale historische DDR-Git-Historie ergänzend in 14 | Original-Mailvolltext nicht neu lokal nachgewiesen; Mailpatch/Git-Commit-Zuordnung offen; keine Fall-8B-Ursache |
| NV-002 | Android/Google-Spiegel, benannter Branch | Katalog mit früherem Prüfdatum | Versionsfixierung und unabhängiger lokaler Volltext offen; nicht als gepinnter aktueller Code verwenden |
| NV-003 | Nest-Google-Spiegel | Katalog mit früherem Prüfdatum | Gleichnamige Boarddatei ist kein identischer Stand; Quellrevision und Archivierung offen |
| NV-004 | Fedora-Spiegel, Commit c1a6f371… | Katalog | Konkrete URLrevision bekannt, lokale Objektexistenz/Bytegleichheit noch zu qualifizieren |
| SPL-001 | Erhaltenes REF-ELF | 9C-44 und erneuter SHA256 in 02 | ORIGINALQUELLE/BELEGT für Byteidentität; kein neuer Maschinen-/Boot-PASS |
| SPL-002 | REF Board, DDR, wait_bit, crt0, config/map | Quellpfade in 04, aktuelle Hashes in 02 | ORIGINALQUELLE für erhaltenen Baum; reale Kalibrierungsregister bleiben QUELLENLUECKE |
| SPL-003 | 830-Byte-8B-Seriellog | Volltext/Hash in 13, Interpretation 05 | ORIGINALQUELLE/BELEGT; POR/Mediumzuordnung kommt aus Projektbericht |
| SPL-004 | Patch 0008 und Präregistrierung | Hartcodierte Adresse und Rekonstruktion | BELEGT für den Referenzstand; neue Linklayouts NICHT_GEPRUEFT |
| SPL-005 | 9C-33–42 und Original 9C-43 | Im Repository nicht vorhanden gemäß Checkpoint | QUELLENLUECKE; 9C-44-Rekonstruktion ist kein Ersatzoriginal |
| SPL-006 | ERR050070 und Registerlesesemantik | Kein qualifizierter lokaler Herstellerbeleg | NICHT_VERIFIZIERT/QUELLENLUECKE; keine Inhalts- oder Workaroundbehauptung |
| SPL-007 | A/B-Angaben aus Chat/Auftrag | 9C-49-Korrektur im Katalog und Checkpoint | NICHT_VERIFIZIERT als historische Originalquelle; A/B sauber von optionalem Messumfang trennen |

## Hardwarearchiv

Die Kosagi-PVT2-/eDP-Unterlagen, ECO- und Issue-Logs, U-Boot-Wikiseiten sowie AMD/Xilinx-Handbücher sind LOKALE_KOPIEN von Originalquellen. Notes und Extrakte sind Ableitungen. Herkunft, URLs, Revisionen und Abrufmetadaten sind in Notes/HTTP-Headers erhalten (11/12). Heutige Prüfsummengleichheit bestätigt diese lokalen Bytes, nicht damalige Serverauthentizität.

PVT2-PDF-Titelblockdatum 2014, HTTP-Last-Modified 2026 und Abrufdatum September 2026 beschreiben unterschiedliche Ereignisse. Das ist kein belegter Widerspruch. Ebenso unterscheiden sich EVT-, DVT- und PVT-Widerstandswerte aufgrund dokumentierter Entwicklung (`research/01-original-novena-docs/notes/novena-pvt2-source-metadata.md:15`; `research/03-root-cause-synthesis/block-3.15w-es8328-i2c3-historische-root-cause.md:3`).

Die Schaltplan-PDF-Textextraktion hatte R11A falsch zugeordnet; die spätere visuelle Prüfung korrigiert die Topologie. Ein unveränderter SHA256 macht eine Extraktionsinterpretation nicht richtig (`research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md:68`). Neue Grafik-/Gerber-Detailprüfung ist NICHT_GEPRUEFT.

## Softwarearchiv

rc5 ist ein dokumentiertes Commitobjekt, keine bloße Tagzeichenfolge. Die historischen Auszüge rc5/rc7/rc12, Linux 5.7/v6.18, Novena-next-Nix und IT6251-Historie sind einzeln manifestiert (11). Enthaltene Git-Bundles werden heute nur gehasht, nicht als ausgeführte Bundle-Integritätsprüfung ausgegeben. Linux v6.18-Auszug und exakte 6.18.49-Buildquelle dürfen nicht ohne Patchlevelvergleich gleichgesetzt werden.

Das Katalogetikett GEPRUEFT 2026-10-01 wird ausschließlich als überlieferte frühere Onlineprüfung geführt. Es erteilt keine gegenwärtige Erreichbarkeits-, Vollständigkeits- oder Versionsgarantie. Die im Katalog selbst angekündigte umfassendere URLinventur ist mit 12 als neues getrenntes Forschungsartefakt vorbereitet; das bestehende Quellenverzeichnis bleibt unverändert.

## Offene Auditaufgaben

Für jede nicht lokal archivierte Katalogseite künftig Originalvolltext, Abrufdatum, genaue Revision und Hash sichern. Alle URLfunde in 12 müssen nach fachlicher Relevanz und tatsächlicher Originalherkunft redaktionell normalisiert werden; automatische Sammlung erfasst auch Navigation, Mirrors und generische Lizenzverweise. Fehlende Provenienz darf nicht aus ähnlich benannten Dateien ergänzt werden.
