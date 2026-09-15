# Novena U-Boot-Dokumentation – Auswertung

## Zweck

Diese Notiz dokumentiert die Auswertung der offiziellen
Novena-U-Boot-Dokumentation im Rahmen der Untersuchung des aktuellen
I2C3-Kaltstartfehlers der Novena.

Untersucht wurden insbesondere:

- die Bootloader-Angaben auf der offiziellen Novena Main Page,
- die offizielle Wiki-Seite „U-boot PVT Notes“,
- die offizielle Wiki-Seite „U-boot-novena“.

Ziel dieser Auswertung ist noch nicht, das tatsächliche Verhalten des
historischen oder des aktuell verwendeten U-Boot zu beweisen.

Stattdessen wird zunächst sauber dokumentiert:

1. was die offiziellen Novena-Dokumente ausdrücklich aussagen,
2. welche Aussagen sie nicht enthalten,
3. welche historischen Softwarequellen sie referenzieren,
4. welche Hypothesen daraus für die spätere Quellcodeanalyse entstehen.

Die Untersuchung erfolgt auf Basis des dokumentierten Kernel-0006-
Checkpoints.

Es wird aus dieser Dokumentationsanalyse kein Kernel-Patch 0007
abgeleitet.

Die Novena bleibt während dieser Phase ausgeschaltet.

---

## 1. Quellen

### 1.1 Novena Main Page

Archivierte Datei:

`research/01-original-novena-docs/sources/kosagi-novena-main-page.html`

SHA-256:

`42e922769ef4d3b1c10a3d3b39081a0e636bedb0d529ecae562e8c1fe2f03640`

Die Main Page ist die übergeordnete offizielle Novena-Dokumentation und
verweist sowohl auf „U-boot PVT Notes“ als auch auf „U-boot-novena“.

---

### 1.2 U-boot PVT Notes

Archivierte HTML-Datei:

`research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.html`

SHA-256:

`354daa562880594e38348b4652c582af0719d92e86b662485524faa040e32f0a`

Archivierte HTTP-Header:

`research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.http-headers.txt`

SHA-256:

`073164d84bd65b76a7bc7777f0feddfd80c257100bcd99ad9584b27a38686d29`

Separat gesicherter eigentlicher Seiteninhalt:

`research/01-original-novena-docs/extracts/u-boot-pvt-notes-content.html`

SHA-256:

`71bb3d7aa9f5c13de3245f5c5137fbbb759b2148c6445ddcf35d55a3d589be65`

MedienWiki-Identität:

- Seitentitel: `U-boot PVT Notes`
- Seitenname: `U-boot_PVT_Notes`
- Revision: `1755`
- Article ID: `371`

Beim Abruf lieferte der Server HTTP 200.

---

### 1.3 U-boot-novena

Archivierte HTML-Datei:

`research/01-original-novena-docs/sources/kosagi-u-boot-novena.html`

SHA-256:

`88757655b9c1ea920941712099f3dd0b072ced6bd2d7b42adc4068111ec7804e`

Archivierte HTTP-Header:

`research/01-original-novena-docs/sources/kosagi-u-boot-novena.http-headers.txt`

SHA-256:

`359310bb527e77cb1117820e5430186f5d68c57a2e523bb25979b710a9ebb40c`

Separat gesicherter eigentlicher Seiteninhalt:

`research/01-original-novena-docs/extracts/u-boot-novena-content.html`

SHA-256:

`b1d20027d5fc8f7a04a874825c6012866613c32c693ce4137608e975e0166ffc`

MedienWiki-Identität:

- Seitentitel: `U-boot-novena`
- Seitenname: `U-boot-novena`
- Revision: `1520`
- Article ID: `416`

Beim Abruf lieferte der Server HTTP 200.

---

## 2. Dokumentierte zweistufige U-Boot-Architektur

Die offiziellen U-Boot-Dokumente beschreiben Novena als System mit
einem zweistufigen U-Boot.

Die erste Stufe ist der Secondary Program Loader, SPL.

Laut „U-boot PVT Notes“ ist der SPL dafür zuständig:

- DDR3-Speicher einzurichten,
- DDR3 zu kalibrieren beziehungsweise Leitungslängen zu bestimmen,
- das Haupt-U-Boot von einem Datenträger zu laden,
- anschließend in das Haupt-U-Boot zu springen.

Die restlichen Aufgaben werden laut dieser Quelle vom Haupt-U-Boot
übernommen.

Die Seite „U-boot-novena“ beschreibt dieselbe grundsätzliche
Zweiteilung.

Der SPL befindet sich außerhalb der normalen Partitionen im frühen
Datenträgerbereich.

Das Haupt-U-Boot liegt als `u-boot.img` auf der ersten Partition.

---

## 3. Aussagen der Novena Main Page

Die Novena Main Page enthält zusätzliche Informationen, die in den
beiden speziellen U-Boot-Wikiseiten nicht beschrieben werden.

Sie bezeichnet U-Boot als damaligen factory-default Bootloader.

Die Main Page beschreibt das Haupt-U-Boot als zweite Bootstufe.

Diese zweite Stufe:

1. lädt die passende Device-Tree-Datei,
2. wertet das Novena-EEPROM aus,
3. passt beziehungsweise beschneidet den Device Tree entsprechend,
4. lädt Linux,
5. springt anschließend in den Kernel.

Die Main Page beschreibt außerdem eine konkrete Verwendung eines
EEPROM-Flags.

Ist `rootfs_ssd` im EEPROM gesetzt, wird der Root-Parameter laut dieser
Dokumentation auf

`PARTUUID=4e6f7653-03`

gesetzt.

Ist `rootfs_ssd` nicht gesetzt oder wird der Recovery-Pfad verwendet,
wird laut Dokumentation

`PARTUUID=4e6f764d-03`

verwendet.

Damit ist auf Dokumentationsebene ausdrücklich belegt, dass das
Haupt-U-Boot Informationen aus dem Novena-EEPROM verwendet.

---

## 4. Bezug zum PVT2-Schaltplan

Die zuvor durchgeführte PVT2-Schaltplananalyse hat U10S als
EEPROM auf I2C3 identifiziert.

Für die dokumentierte PVT2-Hardware gilt:

- Bauteil: U10S
- Typ: FT24C512A-UTR-T
- Bus: I2C3
- Versorgung: P3.3V_DELAYED
- A0: GND
- A1: GND
- A2: GND
- im Schaltplan verwendete 8-Bit-Adresse: `0xAC`
- entsprechende 7-Bit-I2C-Adresse: `0x56`

Damit bestehen zwei voneinander unabhängige Dokumentationsbefunde:

1. Die Novena Main Page sagt, dass das Haupt-U-Boot das
   Novena-EEPROM auswertet.
2. Der PVT2-Schaltplan ordnet U10S dem I2C3-Bus zu.

Diese Kombination macht einen historischen U-Boot-Zugriff auf I2C3
sehr plausibel.

Sie ist jedoch noch kein vollständiger softwareseitiger Beweis für die
konkrete Implementierung des Zugriffs.

Dieser Beweis muss aus dem historischen U-Boot-Quellcode gewonnen
werden.

---

## 5. Historisches U-Boot-Repository

Sowohl „U-boot PVT Notes“ als auch „U-boot-novena“ verweisen auf das
historische Repository:

`xobs/u-boot-novena`

Die PVT Notes nennen dieses Repository direkt in der Build-Anleitung.

Die Seite „U-boot-novena“ beschreibt die damalige Novena-Version als
relativ leicht gegenüber Mainline angepasst.

Als Anpassungen werden insbesondere genannt:

- Unterstützung für die Konfiguration des DDR3-Speichers,
- Debian-Build-Unterstützung.

Diese Beschreibung ist eine historische Dokumentationsaussage.

Sie ersetzt keine Untersuchung der tatsächlichen Quellcodeänderungen.

---

## 6. Dokumentierter historischer Versionsfixpunkt

Die Seite „U-boot-novena“ enthält einen konkreten Build-Befehl:

`git-buildpackage -us -uc --git-upstream-tag=v2014.10-novena-rc5`

Damit ist

`v2014.10-novena-rc5`

ein dokumentierter historischer Versionsfixpunkt für die weitere
Untersuchung.

Die Seite sagt gleichzeitig ausdrücklich, dass mehrere U-Boot-Versionen
für Novena existierten.

Daher darf nicht ohne weitere Prüfung angenommen werden, dass ein
beliebiger Branch oder der aktuelle Stand des historischen
Repositories exakt dem damals eingesetzten Produktions-U-Boot
entspricht.

Für die folgende historische Quellcodeanalyse soll deshalb zunächst
der dokumentierte Tag

`v2014.10-novena-rc5`

untersucht werden.

---

## 7. Negativbefunde der U-Boot-Wikiseiten

Weder „U-boot PVT Notes“ noch „U-boot-novena“ enthalten in ihren
gesicherten Revisionen konkrete Angaben zu:

- I2C,
- I2C3,
- I2C-Busnummern,
- U10S,
- EEPROM-Adresse `0x56`,
- IT6251,
- eDP,
- LCD-I2C,
- AUX_I2C,
- FPGA-I2C,
- ES8328-I2C,
- I2C-Pullups,
- SDA,
- SCL,
- Arbitration Lost,
- IAL,
- I2C-Bus-Recovery,
- IT6251-Reset,
- IT6251-Versorgungssequenz,
- POR-spezifischem I2C-Verhalten.

Die fehlenden Treffer sind für die jeweils archivierten Revisionen
belegt.

Sie beweisen nicht, dass entsprechende Informationen niemals an
anderer Stelle dokumentiert wurden.

---

## 8. Bedeutung für den aktuellen Kernel-0006-Befund

Der aktuelle reproduzierte Kaltstartfehler zeigt:

`Cold FAIL: A=81/80 -> M0=93/80`

Beim erfolgreichen Same-Boot-LDB-Rebind zeigt sich dagegen:

`LDB-Rebind PASS: A=81/80 -> M0=81/a0`

Damit ist die sichtbare I2SR/I2CR-Ausgangslage unmittelbar vor dem
Setzen von MSTA gleich, während sich der erste beobachtbare Zustand
danach unterscheidet.

Die U-Boot-Dokumentation beweist keine Ursache für dieses Verhalten.

Sie erweitert jedoch die Menge der Zustände, die bei der
Root-Cause-Untersuchung berücksichtigt werden müssen.

Insbesondere darf nicht ohne Prüfung angenommen werden, dass Linux
I2C3 nach einem echten POR in einem vollständig unberührten
Controller- oder Buszustand übernimmt.

Falls das historische U-Boot das Novena-EEPROM tatsächlich über den
i.MX6-I2C3-Controller liest, existiert mindestens historisch ein
I2C3-Zugriff vor dem Kernelstart.

Ob dies auch für das aktuell auf der Test-SD eingesetzte U-Boot 2020.07
gilt, ist damit ausdrücklich noch nicht bewiesen.

---

## 9. Drei getrennte Beweisebenen

Für die weitere Untersuchung müssen drei Ebenen strikt getrennt
bleiben.

### Ebene A – historische Dokumentation

Bereits dokumentiert:

- Novena verwendet ein zweistufiges U-Boot.
- Das Haupt-U-Boot wertet laut Main Page das Novena-EEPROM aus.
- U10S befindet sich laut PVT2-Schaltplan auf I2C3.
- `v2014.10-novena-rc5` ist ein dokumentierter historischer
  U-Boot-Versionsfixpunkt.

### Ebene B – historischer U-Boot-Quellcode

Noch zu untersuchen:

- welcher i.MX6-I2C-Controller benutzt wird,
- welche Busnummer verwendet wird,
- wie EIM_D17/EIM_D18 gemuxt werden,
- wann I2C3 initialisiert wird,
- wann U10S gelesen wird,
- ob der Bus anschließend zurückgesetzt oder verändert wird,
- ob U-Boot weitere I2C3-Teilnehmer anspricht,
- ob IT6251 oder Display-Hardware initialisiert wird,
- welche GPIO-, Reset-, Clock- oder Power-Sequenzen ausgeführt werden,
- in welchem Zustand I2C3 beim Sprung in Linux verbleibt.

Der erste Untersuchungsstand soll der dokumentierte Tag

`v2014.10-novena-rc5`

sein.

### Ebene C – tatsächlich verwendetes U-Boot 2020.07

Später separat zu untersuchen:

- exakte Herkunft und Konfiguration,
- tatsächlicher I2C3-Code,
- tatsächlicher EEPROM-Zugriff,
- tatsächlicher Display-/IT6251-Code,
- tatsächliche Pinmux- und GPIO-Sequenzen,
- tatsächlicher Zustand beim Übergang zu Linux.

Nur diese Ebene ist unmittelbar auf den aktuellen 0006-Testaufbau
übertragbar.

---

## 10. Neue Arbeitshypothese

Die bisherige Dokumentation erlaubt folgende Arbeitshypothese:

Das Haupt-U-Boot könnte vor Linux denselben physischen I2C3-Bus
verwenden, auf dem später der IT6251 angesprochen wird.

Diese Hypothese ist hardware- und dokumentationsseitig plausibel,
aber noch nicht softwareseitig vollständig bewiesen.

Insbesondere ist noch offen:

- ob der historische EEPROM-Zugriff wirklich über den i.MX6-I2C3-
  Controller erfolgt,
- welche Controllerzustände danach verbleiben,
- ob weitere I2C3-Teilnehmer angesprochen werden,
- ob das Display beziehungsweise der IT6251 bereits von U-Boot
  initialisiert wird,
- ob dieses Verhalten in U-Boot 2020.07 noch vorhanden ist.

Die Hypothese darf daher nicht als Root Cause des aktuellen
Arbitration-Lost-Fehlers behandelt werden.

---

## 11. Nächster Forschungsschritt

Die Originaldokumentation hat einen konkreten historischen
Software-Fixpunkt geliefert:

`xobs/u-boot-novena`

mit dem dokumentierten Tag:

`v2014.10-novena-rc5`

Der nächste Forschungsblock soll diese historische Softwarequelle
reproduzierbar archivieren und analysieren.

Dabei sind insbesondere zu untersuchen:

- Novena-Boardinitialisierung,
- I2C-Initialisierung,
- `setup_i2c()`,
- Busnummern,
- EIM_D17/EIM_D18,
- EEPROM-Adresse `0x56`,
- EEPROM-Leseweg,
- `board_init`,
- `misc_init_r`,
- Display-/Video-Code,
- IT6251,
- FPGA,
- GPIO- und Reset-Sequenzen,
- Reihenfolge der Initialisierung,
- Zustand unmittelbar vor dem Kernelstart.

Erst anschließend erfolgt der Vergleich mit dem tatsächlich
eingesetzten U-Boot 2020.07.

---

## 12. Zwischenfazit

Die offiziellen U-Boot-Wikiseiten liefern keinen direkten Hinweis auf
den aktuellen I2C3-Arbitration-Lost-Fehler.

Sie liefern jedoch zwei wichtige Bausteine für die weitere
Root-Cause-Untersuchung:

1. Das Haupt-U-Boot wertet laut offizieller Novena Main Page das
   Novena-EEPROM aus.
2. `v2014.10-novena-rc5` ist ein dokumentierter historischer
   U-Boot-Versionsfixpunkt.

Zusammen mit dem PVT2-Schaltplan, der U10S auf I2C3 bei Adresse `0x56`
zeigt, ergibt sich damit eine konkrete und überprüfbare Frage für die
historische Quellcodeanalyse.

Es wird aus diesen Befunden kein Workaround und kein Kernel-Patch 0007
abgeleitet.

Die Novena bleibt ausgeschaltet.

Die externe Test-SD und der dokumentierte Kernel-0006-Zustand bleiben
unverändert.
