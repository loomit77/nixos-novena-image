# Novena USB – Abschlussbericht vom 2026-10-21

Auftrag: **NOVENA-USB-DOC-CODEX-215-R1**. Konsolidierung der bereits
ausgewerteten Diagnoseblöcke **208 bis 213**, keine neue Diagnoseausführung.

## Ergebnis und Quellenstatus

Die USB-Untersuchung wird vorläufig abgeschlossen. USB-Geräte werden unter
Linux 6.18.49 erkannt; die Ursache der Over-current-Warnung bleibt offen.
Weder tatsächlicher elektrischer Überstrom noch fehlerhafte Signalauswertung
sind bewiesen. Aus dem historischen Hub-Dienst folgt keine heutige Reset-
oder Stromversorgungsmaßnahme.

Die Ergebnisse der Blöcke 208–213 werden als **im Auftrag bestätigter
Auswertungsstand** dokumentiert. Ihre Originalprotokolle wurden im Repository
nicht gefunden und hier nicht erneut geprüft. Eine belastbare Zuordnung
jedes Einzelbefunds zu einer einzelnen Blocknummer ist damit nicht möglich;
die sechs Blöcke werden gemeinsam konsolidiert. Externe Blockprotokolle im
privaten Archiv `loomit77/project-block-logs`, Bereich `novena/2026/10/`,
sind Archivbelege, keine hier lokal geprüften Dateien. Exakte Dateinamen,
Sidecarprüfungen, Exitcodes und Transportergebnisse werden nicht rekonstruiert.

Lokal geprüfte Quellen und ihre Aussagegrenzen:

| Quelle | Beitrag zum Bericht | Grenze |
|---|---|---|
| [Historische Medien, Abschnitt Betriebssystem und Novena-Pakete](HISTORICAL-MEDIA.md#betriebssystem-und-novena-pakete) | Installierte Pakete `novena-usb-hub` und `linux-firmware-image-novena 3.19-novena-r39` | Bestehende historische Auswertung, kein erneuter Raw-Master-Zugriff und kein Ausführungsnachweis |
| [POR-Serie vom 15.09., vollständiges dmesg von Boot 05](../evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt) | `2184200.usb`, `ci_hdrc.1`, Bus 1, Polaritätsmeldung, Hubpfade, ASIX und `enu1u2`; gesondert CODA-Firmwarefehler | Älterer Diagnoseboot, kein neuer Hardwaretest und kein Rohlog von 208–213 |
| [Historische Nix-Paketdefinition](../research/02-historical-software/extracts/novena-next-nixos-novena-HEAD-novena-usb-hub-default.nix) | `novena-next/novena-usb-hub`, Revision `4b1123a`, libusb1; Zweck USB power/overcurrent control | Paketdefinition beweist weder Debian-Dienstlauf noch Portzuordnung; Archiv-Audit bezeichnet den Extrakt als Rekonstruktion |
| [Lokale Kosagi-Main-Page-Kopie](../research/01-original-novena-docs/sources/kosagi-novena-main-page.html) | Abschnitt Software projects beschreibt Port-Stromverwaltung durch `novena-usb-hub` | Keine konkrete Bootausführung |
| [Lokale historische Issue-Log-Kopie](../research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.html) | Historische USB/Power-Notizen und Geräte `05e3:0614` sowie ASIX | Anderer historischer Entwicklungsstand; damalige manuelle Regulatorbefehle sind keine heutige Handlungsempfehlung |
| [Golden-Build-Dokumentation](GOLDEN-BUILD.md#novena-device-tree) und [Vor-Git-Rekonstruktion](TEST-LOG.md#rekonstruktion-der-vor-git-displayentwicklung) | Unterscheidung Basis-DTB und verschiedener Overlay-DTBs | Dokumentierte historische Identitäten; hier keine erneute Binärprüfung |

Der Paketdefinitions-Extrakt hat lokal SHA256
`0e8c4a46537f399236baebb1317b63fdc5ba4701960db6c18c5c4b17e90aeb26`;
dies stimmt mit seinem Eintrag in `research/02-historical-software/SHA256SUMS`
überein. Hashgleichheit ersetzt keine ursprüngliche Quellprovenienz.

## Nachgewiesene Beobachtungen: historisches Debian

Gemäß bestätigtem Auswertungsstand war `novena-usb-hub` installiert und ein
systemd-Oneshot-Dienst aktiviert. Seine Befehlsfolge war:

```text
novena-usb-hub -d u3
novena-usb-hub -e u3
novena-usb-hub -l
```

Dies beschreibt die historische Konfiguration und ist kein auszuführender
Testplan. **Eine erfolgreiche Ausführung während eines historischen Boots
ist nicht bewiesen.** Installation, Aktivierung und erfolgreiche Ausführung
sind getrennte Nachweisstufen.

Die installierte Manpage beschreibt **u = upstream**, **d = downstream**.
Eine historische README verwendet abweichende Portbezeichnungen. Dieser
Quellenwiderspruch bleibt ausdrücklich erhalten; keine der Benennungen wird
stillschweigend zur sicheren Hardwarezuordnung erklärt. Manpage, README und
Dienstbefund sind hier aus dem bestätigten Auswertungsstand übernommen;
die konkreten Originaldateien liegen für diesen Bericht nicht im Repository
vor. Der genaue README-Wortlaut wird deshalb nicht nachgebildet.

Eine sichere Zuordnung von **u3** zu einem Linux-Gerätepfad ist nicht
nachgewiesen. Insbesondere wird u3 nicht aus seiner Ziffer mit einem Port
oder Gerät im Linux-Baum gleichgesetzt.

## Nachgewiesene Beobachtungen: laufendes NixOS und Topologie

Der bestätigte Laufzeitstand ist **Linux 6.18.49 auf Novena**. Der USB-Host
ist `ci_hdrc.1` an `2184200.usb`, USB-Bus 1. Die ausgewerteten Blöcke enthalten:

```text
No over current polarity defined
usb usb1-port1: over-current condition
```

Die erste Meldung und die Hostidentität sind zusätzlich im genannten lokalen
September-dmesg belegt. Dieser ältere Beleg ersetzt nicht das aktuelle
Originalprotokoll zur Over-current-Bedingung.

| Gerät/Funktion | Beobachtete Identität bzw. Linux-Gerätepfad |
|---|---|
| Genesys Logic Hub | `05e3:0614`, `1-1` |
| ASIX AX88772B USB-Ethernet | `1-1.2` |
| Weiterer Hub | `1-1.4` |
| ASIX-Netzwerkinterface | `enu1u2` |

Die Pfade `1-1`, `1-1.2`, `1-1.4` und der Interfacebezug sind auch im lokalen
September-Log sichtbar. Dessen zusätzliche PHY-Treiberbezeichnung AX88772C
ändert nicht die dortige USB-Ethernet-Registrierung als AX88772B.
Die Enumeration belegt erkannte Geräte, keine vollständige Funktions-,
Last- oder elektrische Sicherheitsqualifikation aller Ports.

## Golden-Referenz und begrenzter Device-Tree-Vergleich

Der bestätigte Vergleich verwendet die Referenz vom **11.09.2026** mit SHA256:

```text
b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522
```

**Identitätspräzisierung:** Die bestehende Golden-Dokumentation führt diesen
Hash als unveränderten **Kernel-Basis-DTB**. Der dort gesondert beschriebene
historische Golden-Overlay-DTB hat dagegen den Hash
`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`.
Die Bezeichnung „Golden-DTB“ für die USB-Vergleichsreferenz hebt diese
Unterscheidung nicht auf. Sie beweist keine Gleichheit aller Golden-Dateien.

Nach dem bestätigten Auswertungsstand stimmen die **untersuchten USB- und
Regulator-Eigenschaften** der Referenz mit dem laufenden Device Tree überein:

| Bereich | Untersuchte Eigenschaften |
|---|---|
| OTG | `disable-over-current` vorhanden |
| Host1 | `dr_mode = host`; `disable-over-current` nicht vorhanden; `vbus-supply` verweist auf PFUZE100 `swbst` |
| `swbst` | 5,0 bis 5,15 Volt; `regulator-boot-on` vorhanden |

Die Spannungsspanne ist ein Device-Tree-Konfigurationsbefund, keine physische
Spannungsmessung. **Ein vollständiger binärer Vergleich der Device Trees
wurde nicht durchgeführt.** Der Teilvergleich beweist weder vollständige
DTB-Identität noch die Ursache der Kernelwarnung. Die Eigenschaften werden
hier dokumentiert, nicht am System erneut ausgelesen.

## Schlussfolgerungen, Hypothesen und offene Fragen

**Schlussfolgerungen:** Historische Hub-Verwaltung war installiert und als
Dienst aktiviert. Das beweist keinen erfolgreichen historischen Dienstlauf
und keine Notwendigkeit derselben Befehle unter NixOS. Erkannte USB-Geräte
und Over-current-Meldung stehen im bestätigten Beobachtungsstand nebeneinander.
Der begrenzte DT-Vergleich liefert keine Grundlage für eine USB-DT-Korrektur.

**Hypothesen, nicht nachgewiesen:** Tatsächlicher elektrischer Überstrom und
fehlerhafte Signalauswertung bleiben mögliche Erklärungen ohne gesicherten
Nachweis. Weder fehlende Polaritätsangabe noch fehlendes
`disable-over-current` auf Host1 werden als bewiesene Ursache gewertet.

**Offene Fragen:** Ursache und elektrische Bedeutung der Over-current-Warnung;
sichere Abbildung des historischen u3 auf die Linux-Topologie; erfolgreicher
historischer Dienstlauf; Auflösung der abweichenden Portbenennungen anhand
konkreter Quellstände. Keine dieser Lücken wird durch Raten geschlossen.

**Getrennter Firmwarebefund:** Die historischen Debian-Pakete
`novena-usb-hub` und `linux-firmware-image-novena` sind nachgewiesen.
Das beweist weder das Vorhandensein noch die erfolgreiche Nutzung der unter
NixOS benötigten CODA/VPU-Firmware. Das lokale September-dmesg meldet bereits
fehlgeschlagene Firmwareanforderungen und `coda 2040000.vpu: firmware request failed`.
Die fehlende CODA/VPU-Firmware bleibt ein eigenständiger offener Befund;
eine ursächliche Verbindung zur USB-Warnung ist nicht nachgewiesen.

## Entscheidung und Abschlussgrenze

Die USB-Untersuchung ist **vorläufig abgeschlossen bei offener Ursache**.
Keine USB-Port-Resets, keine Änderung der USB-Stromversorgung und kein
unbegründetes Setzen von `disable-over-current`. Wiederaufnahme der
Ursachenanalyse nur bei neuer, unterscheidungskräftiger Evidenz und innerhalb
eines entsprechend freigegebenen Auftrags.

Dieser Dokumentationsauftrag ergänzt ausschließlich die sechs erlaubten
Markdown-Dateien. Keine Builds, Nix-Evaluierungen, Hardwaretests,
Netzwerkzugriffe, Medienzugriffe, Commits oder Pushes; kein Runner gestartet,
keine neuen Rohlogs oder Sidecars erzeugt. Historische Bootbefunde, moderne
DDR-/SPL-Qualifikation, Audio und Borg bleiben eigenständige Arbeitsstränge.
