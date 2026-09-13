# Golden Build – funktionierendes Novena-Display

Stand: 2026-09-13

## Zweck

Dieses Dokument beschreibt den gesicherten und verifizierten Golden Build
des reproduzierbaren NixOS-Images für das Kosagi Novena.

Der Golden Build dient als bekannte funktionierende Referenz, auf die bei
späteren Kernel-, Device-Tree-, DRM-, Display-, I2C- oder
Bootloader-Experimenten zurückgegriffen werden kann.

Es ist wichtig, mehrere Ebenen voneinander zu unterscheiden:

1. die am 2026-09-11 angelegte Golden-Build-Sicherung
2. den darin enthaltenen historischen Display-DTB
3. den späteren Git-Stand des Projekts
4. die am 2026-09-12 erfolgte erfolgreiche Hardware-Validierung des
   aktuellen Images auf echter Novena-Hardware

Die Sicherung vom 2026-09-11 wird nicht verändert oder überschrieben.

## Zugehöriger Git-Stand der ursprünglichen Golden-Build-Sicherung

Repository:

```text
~/nixos-novena-image
```

In den Metadaten der ursprünglichen Golden-Build-Sicherung ist folgender
Git-Commit dokumentiert:

```text
f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7
```

Kurzform:

```text
f26464f
```

Commit:

```text
Add reproducible Novena NixOS image project
```

Dieser Commit ist der Root-Commit der vorhandenen Git-Historie.

Wichtig:

Der im Backup dokumentierte Git-Commit darf nicht als Beweis dafür
verstanden werden, dass sämtliche darin gesicherten Binärartefakte aus
exakt diesem committed Quellzustand gebaut wurden.

Insbesondere wurde der gesicherte Novena-DTB mit SHA-256

```text
e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f
```

nachträglich als Artefakt eines früheren Vor-Git-Overlay-Zustands
identifiziert.

Der exakte Repository-Inhalt von Commit `f26464f` wurde zusätzlich als
unabhängiges tar.gz-Archiv gesichert.

## Zusätzlicher Git-Sicherungspunkt vom 2026-09-12

Nach der erfolgreichen Hardware-Validierung wurde ein zusätzlicher
annotierter Git-Tag angelegt:

```text
novena-display-working-2026-09-12
```

Der Tag zeigt auf Commit:

```text
1d43425
```

Commit-Beschreibung:

```text
Document Nix closure backup for golden build
```

Dieser Tag markiert einen bestätigten funktionierenden
Display-Zwischenstand vor weiteren Änderungen an der
I2C-Diagnoseinstrumentierung.

## Kernel

Kernel-Version:

```text
Linux 6.18.49
```

Gesicherter Kernel:

```text
kernel/zImage
```

SHA-256:

```text
30a63036889af44301f4821f76ca66a7b8f428c18e725ca5ec29775b6d5fbf5c
```

Der komplette Kernel-Ausgabebaum wurde im Golden Backup übernommen.

Der zugehörige Golden-Kernel liegt im Nix Store unter:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Die später untersuchte Derivation zeigt für diesen Golden-Kernel die
Display-Patches `0001` und `0002`.

Die später entwickelten Diagnose-Patches `0003` und `0004` gehören nicht
zum historischen Golden-Kernel.

Späterer `0003`-Debug-Kernel:

```text
/nix/store/2d1h5iihfq560zm0sg4kh4n4ydvk2a2m-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Noch späterer `0004`-Diagnosekernel:

```text
/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Diese Debug-Builds sind Diagnoseentwicklungen nach dem Golden Build und
dürfen nicht als Bestandteil der ursprünglichen Sicherung interpretiert
werden.

## Novena Device Tree

Separat gesicherter Novena-DTB:

```text
dtb/imx6q-novena.dtb
```

SHA-256:

```text
e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f
```

Dieser DTB verwendet Linux 6.18.49 als Kernelbasis, stammt aber aus
einem früheren Device-Tree-Overlay-Zustand als der später versionierte
Projektstand in `hardware/novena.nix`.

Der zugehörige unveränderte Kernel-Basis-DTB besitzt den SHA-256-Wert:

```text
b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522
```

Der historische Golden-DTB wurde über erhaltene Nix-Derivationen auf
folgenden Overlay-Output zurückgeführt:

```text
/nix/store/pgyag1yxfcrwf1lz4zqpjhgag2ah564b-device-tree-overlays/imx6q-novena.dtb
```

Der später aktuell gebootete Display-DTB besitzt dagegen den SHA-256-Wert:

```text
31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f
```

Beide DTB-Zustände basieren auf demselben Kernel-Basis-DTB.

## Kernel-Konfiguration

Gesicherte Kernel-Konfiguration:

```text
config/linux-6.18.49.config
```

SHA-256:

```text
cef1365fe594704ec72394e8abbe2b96cc8ccad37090fcff42aa8d221aaa5ef3
```

Damit ist neben dem fertigen Kernel auch die Konfiguration des bekannten
Buildstands separat erhalten.

## Fertiges SD-Image

Gesichertes Image:

```text
image/nixos-novena-sd-image.img
```

SHA-256:

```text
5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b
```

Dieses Image entspricht dem am 2026-09-07 erfolgreich fertiggestellten
NixOS-Novena-Build.

Nix-Store-Ursprung:

```text
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

Dieses Image wurde am 2026-09-12 auf echter Novena-Hardware erfolgreich
mit funktionierender Linux-seitiger Displayinitialisierung getestet.

## Hardware-Validierung am 2026-09-12

Der aktuelle Build wurde am 2026-09-12 auf echter Novena-Hardware kalt
gestartet.

Der Test erreichte:

* SPL/U-Boot
* Linux 6.18.49
* NixOS Stage 1
* Root-Dateisystem
* regulären Login
* erfolgreiche Linux-seitige IT6251-Stromversorgung
* erfolgreiche IT6251-Product-ID-Erkennung
* erfolgreiche IT6251-Initialisierung
* erfolgreiches DisplayPort-Linktraining
* stabile interne Displayausgabe mit 1920x1080

Die Product-ID wurde bereits beim ersten Versuch korrekt gelesen.

Gelesene Register:

```text
0x00 = 0x15
0x01 = 0xca
0x02 = 0x51
0x03 = 0x62
```

Kernelmeldung:

```text
IT6251 detected: vendor ca15 device 6251
```

## Display-Link

Die IT6251-Initialisierung war bereits im ersten Versuch erfolgreich.

Das Linktraining endete nach zehn Iterationen.

Systemstatus:

```text
0x3e
```

Gemessene aktive Auflösung:

```text
hactive: 1920
vactive: 1080
```

Erfolgsindikatoren:

```text
is_stable: stable 1920x1080
display link stable
bridge_enable: exit success
```

Damit ist bestätigt, dass der aktuelle Linux-6.18.49-Stand den internen
Novena-Displaypfad grundsätzlich vollständig initialisieren kann.

## Regulatorstatus nach erfolgreicher Initialisierung

Nach dem erfolgreichen Displaystart wurden die GPIO-Zustände geprüft.

Ergebnis:

```text
gpio-15 (regulator-lvds-lcd) out hi
gpio-28 (regulator-display) out hi
```

Regulator-Zusammenfassung:

```text
lcd-lvds-power
```

* aktiv
* 3300 mV
* Backlight-Consumer aktiv

und:

```text
lcd-display-power
```

* aktiv
* 3300 mV
* Consumer `2-005c-power` aktiv

Damit waren sowohl Display- als auch Backlight-Versorgung aktiv.

## Relevante Device-Tree-Korrekturen

Der aktuell funktionierende Displaystand enthält unter anderem:

* explizite LDB-Clock-Zuweisungen
* `single-master;` auf dem Display-I2C-Bus
* Novena-spezifisches IT6251-Pinmux
* Novena-spezifisches Backlight-Pinmux
* `startup-delay-us = <2000000>` für `reg_display`
* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`

Mit diesem Gesamtzustand wurde eine stabile interne Displayausgabe mit
1920x1080 bestätigt.

Die exakte isolierte Wirkung der Clock-Zuweisungen wurde in den
erhaltenen Vor-Git-Aufzeichnungen nicht separat nachgewiesen.

Historisch verschwanden nach Einführung von `single-master;` die zuvor
sichtbaren `-EAGAIN`-Fehler auf dem Display-I2C-Bus.

Eine spätere Quellcodeanalyse zeigte jedoch, dass `single-master;`
intern `multi_master = false` setzt und dadurch die normale IAL-
Behandlung im Treiber übersprungen wird.

Die Eigenschaft darf deshalb nicht als Beweis dafür verstanden werden,
dass Hardware-Arbitration-Lost vollständig verhindert wurde.

## Historischer Display-Overlay-Zustand

Der historische Golden-DTB `e36cd0c8...` unterscheidet sich vom später
aktuell gebooteten DTB `31f2b35e...`.

Der frühere Zustand enthielt unter anderem:

* `regulator-always-on;` auf `reg_display`
* `regulator-always-on;` auf `reg_lvds_lcd`
* den bereits im Kernel-Basis-DTB vorhandenen
  `startup-delay-us = <200000>` auf `reg_display`
* kein `single-master;` auf dem Display-I2C-Bus
* keine späteren expliziten Clock-Zuweisungen

Der spätere funktionierende Zustand enthält dagegen:

* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`
* `startup-delay-us = <2000000>`
* `single-master;`
* explizite Clock-Zuweisungen

Die beiden Zustände dürfen deshalb nicht als identische Device-Tree-
Generation behandelt werden.

## Erwartete IT6251-Reset-NACKs

Während der IT6251-Reset-Sequenzen wurden temporär folgende Meldungen
beobachtet:

```text
error -6 writing to eDP addr 0x5
error -6 writing to LVDS addr 0x5
```

Diese Meldungen entsprechen dem bekannten Verhalten während der
IT6251-Resetsequenz und verhinderten den erfolgreichen Displaystart
nicht.

Die Initialisierung lief anschließend vollständig weiter und endete mit
einem stabilen 1920x1080-Link.

## Aktuelle Einschränkung

Der erfolgreiche Test vom 2026-09-12 bestätigte, dass der Linux-6.18.49-Displaypfad grundsätzlich vollständig funktionieren kann.

Spätere Tests zeigten jedoch, dass dieser Erfolg nicht bei jedem echten Kaltstart reproduzierbar ist. Mit einem isoliert STMPE811-deaktivierten Test-DTB wurden fünf echte Kaltstarts durchgeführt: 2/5 direkte Display-Erfolge, 3/5 direkte Display-Fehler und 3/3 erfolgreiche DRM-Rebind-Recoveries.

Die anschließende `0004`-/`0005`-Diagnose lokalisierte die entscheidende Cold-Boot-Divergenz weiter auf den Übergang in den Master-/START-Zustand:

`FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

gegen:

`Same-Boot Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Auch das Entfernen von `single-master;` verhindert das vom i.MX6-I2C-Controller gemeldete IAL-Ereignis nicht.

Diese spätere Diagnose ändert den historischen Golden Build nicht. Der Golden Build bleibt eine bekannte funktionierende Referenz, ist aber kein Beweis für deterministische Cold-Boot-Stabilität. Die tiefere Root Cause des START-/IAL-Verhaltens ist weiterhin offen.

## Separates i2c-0-Diagnoseproblem

Der aktuelle Diagnose-Patch protokolliert Arbitration-Lost-Ereignisse
auch auf anderen i.MX-I2C-Controllern.

Auf `i2c-0` wurden nach dem Boot große Mengen folgender Meldung erzeugt:

```text
NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
```

Zusätzlich wurde beobachtet:

```text
<i2c_imx_write> write timedout
```

Dieses Problem ist getrennt vom erfolgreichen IT6251-Betrieb auf
`i2c-2` zu betrachten.

Vor weiteren Reproduzierbarkeitstests muss der Diagnose-Patch
entschärft oder auf den relevanten Bus begrenzt werden.

## STMPE811 / Touchscreen

Der STMPE811 auf I2C-Adresse `0x44` ist sowohl im historischen
Golden-DTB `e36cd0c8...` als auch im später funktionierenden Baseline-DTB
`31f2b35e...` aktiv.

Später wurde der STMPE811 in einem kontrollierten Test durch ein separates
Overlay deaktiviert.

Baseline-DTB:

```text
31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f
```

STMPE-deaktivierter Test-DTB:

```text
d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8
```

Der semantische DTB-Vergleich zeigte als einzige inhaltliche Änderung
`status = "disabled"` am STMPE811-Knoten.

Im Hardwaretest verschwanden damit die STMPE-/`0-0044`-Fehler
vollständig, während das intermittierende IT6251-Cold-Boot-Problem
bestehen blieb.

Damit ist experimentell bestätigt, dass die beobachteten
STMPE811-/Touchscreen-Fehler nicht die Ursache des intermittierenden
IT6251-Cold-Boot-Problems sind.

## Projektarchiv

Archiv des in der ursprünglichen Golden-Build-Sicherung dokumentierten
Git-Commits:

```text
metadata/nixos-novena-image-f26464f.tar.gz
```

SHA-256:

```text
75779166a971aba60003c0599294e3f69389bce789375e1e47d3baf8d295f6b3
```

Das Archiv wurde mit `git archive` direkt aus Commit `f26464f` erzeugt.

Dadurch existiert zusätzlich zu den Git-Remotes eine unabhängige Kopie
der exakten Projektquellen dieses Commits.

Wichtig:

Dieses Git-Archiv dokumentiert den Quellstand von `f26464f`.

Es ist kein Provenienznachweis dafür, dass der historische
Golden-DTB `e36cd0c8...` aus genau diesem committed Quellstand gebaut
wurde.

## Golden-Backup-Pfad auf foobox

Lokaler Backup-Pfad:

```text
~/novena-backups/nixos-display-working-2026-09-11/
```

Gesamtgröße nach Abschluss der Nix-Closure-Sicherung:

```text
6,2G
```

Aufbau:

```text
nixos-display-working-2026-09-11/
├── SHA256SUMS
├── config/
│   └── linux-6.18.49.config
├── dtb/
│   └── imx6q-novena.dtb
├── image/
│   └── nixos-novena-sd-image.img
├── kernel/
│   ├── System.map
│   ├── zImage
│   └── dtbs/
├── metadata/
│   ├── build-info.txt
│   ├── image-references.txt
│   ├── kernel-references.txt
│   ├── nix-closure-info.txt
│   └── nixos-novena-image-f26464f.tar.gz
└── nix/
    ├── image-closure.nar
    └── kernel-closure.nar
```

## Integritätsprüfung

Für sämtliche normalen Dateien des Golden Backups wurde eine gemeinsame
SHA-256-Prüfsummenliste erzeugt:

```text
SHA256SUMS
```

Nach Abschluss aller Sicherungsschritte wurde das komplette Backup erneut
mit folgendem Befehl geprüft:

```text
sha256sum -c SHA256SUMS
```

Finales Ergebnis:

```text
Fehlerhafte Einträge: 0
Geprüfte Dateien: 1398
```

Damit waren zum Zeitpunkt der Prüfung alle 1398 gesicherten Dateien
bitgenau konsistent mit der Prüfsummenliste.

## Nix-Store-Ursprung

Zum Zeitpunkt der Sicherung zeigten die Projekt-Symlinks auf folgende
Nix-Store-Objekte.

Image:

```text
/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img
```

Kernel:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Novena-DTB:

```text
/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb
```

Kernel-Konfiguration:

```text
/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49
```

Die unmittelbaren Nix-Store-Referenzen von Image und Kernel wurden
zusätzlich gespeichert:

```text
metadata/image-references.txt
metadata/kernel-references.txt
```

Die gespeicherten Metadaten müssen im Zusammenhang mit der oben
beschriebenen Vor-Git-Provenienz des historischen Device Trees gelesen
werden.

## Nix-Closure-Sicherung

Zusätzlich zu den fertigen Binärartefakten wurden die vorhandenen
Nix-Store-Closures exportiert.

### Image-Closure

Datei:

```text
nix/image-closure.nar
```

Größe:

```text
3833159224 bytes
```

Enthaltene Store-Pfade zum Zeitpunkt des Exports:

```text
605
```

SHA-256:

```text
8228f00e3f9f85046a55621727e82f70395125e75f00ee0fc497402879464b02
```

### Kernel-Closure

Der Kernel-Store-Pfad war nicht Bestandteil der zuvor ermittelten
Image-Closure und wurde deshalb separat exportiert.

Datei:

```text
nix/kernel-closure.nar
```

Größe:

```text
96724392 bytes
```

SHA-256:

```text
5f44cfedcdea2d98cb3b8ea4b56fae35e5c0894a8ca5beffb8f78e080e8b069f
```

Die Details der Closure-Sicherung sind zusätzlich dokumentiert in:

```text
metadata/nix-closure-info.txt
```

Die Archive können später in einen kompatiblen Nix Store importiert
werden mit:

```text
nix-store --import < archive.nar
```

Die Closure-Archive ergänzen die binären Golden-Build-Artefakte und die
Projektquellen.

Sie ersetzen weder das fertige SD-Image noch das Git-Repository.


## Spätere Diagnoseentwicklung nach dem Golden Build

Nach der Golden-Sicherung wurde die I2C-Diagnose weitergeführt.

Golden-Kernel:

```text
/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Patchbasis: `0001`, `0002`.

Späterer `0003`-Debug-Kernel:

```text
/nix/store/2d1h5iihfq560zm0sg4kh4n4ydvk2a2m-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Deriver:

```text
/nix/store/422lrj9vi0ydb6cczl0ib8wjx8ll7sqc-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv
```

Patchbasis: `0001`, `0002`, `0003`.

Neuer `0004`-Diagnosekernel:

```text
/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49
```

Deriver:

```text
/nix/store/qk21vdydrywnkqlh3rrvg7n9zn4pwapk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv
```

Quelle:

```text
/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz
```

Patchbasis: `0001`, `0002`, `0003`, `0004`.

Dieser Build wurde erfolgreich erstellt und provenienzgeprüft. Die
anschließend durchgeführten Hardwaretests sind im folgenden Abschnitt
dokumentiert.

## Diagnose-Meilenstein nach dem Golden Build: 0004/0005

Die historische Golden-Sicherung bleibt unverändert. Die folgenden
Ergebnisse gehören ausschließlich zur späteren Diagnoseentwicklung und
dürfen nicht als Eigenschaften des historischen Golden-DTB beziehungsweise
des ursprünglichen Golden-Build-Zustands gelesen werden.

Mit `0004` zeigte ein Cold-Boot-FAIL mit aktivem `single-master;`:

`A=81/80 B=93/80 C=93/80 D=93/d8 E=93/d8 IRQ=93/d8`

Nach isoliertem Entfernen von `single-master;` blieb der Cold-Boot-Fehler bestehen. Der dabei verwendete STMPE-deaktivierte DTB besitzt SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Ohne `single-master;` erkannte der Treiber `IAL=0x93` und brach mit `-EAGAIN` ab. Damit ist bestätigt, dass `single-master;` das vom i.MX6-I2C-Controller gemeldete IAL-Ereignis weder verursacht noch verhindert.

Mit `0005` wurde der frühe Fehlerpfad direkt instrumentiert. Patch-SHA-256:

`81c2de96d7200a6e7e2684518c711bc5686b8d5295a52d5e7aaa54efed94e556`

Kernel:

`/nix/store/97pmqlp5xvsh1l0i877lfc4r4sq6i7np-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

SHA-256 des `zImage`:

`197ce0ef325307a877eb5a889519387be91fdf536da3bee03883f0c40264fc7c`

Der erste echte Cold Boot mit diesem Stand schlug fehl. Boot-ID:

`48462ba4-4972-4d23-9261-e69c822ed584`

Direkte START-Diagnose:

`A=81/80 B=93/80 C=83/80 ret=-11`

Im gleichen Boot war ein DRM-Rebind erfolgreich:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

mit `IRQ=a2/f8` beziehungsweise `IRQ=a6/f8`. Der Rebind erreichte einen stabilen 1920x1080-Link.

Damit ist die Cold-Boot-Divergenz unmittelbar beim Eintritt in den Master-/START-Zustand lokalisiert. Die tiefere Root Cause bleibt offen.

Das vollständige `0005`-Testarchiv besitzt SHA-256:

`0c38c954a7fac033b5fd619653efd8eb0d5fffa3c4024329cb6d85d247e1c3db`

und wurde auf Novena und foobox identisch verifiziert.

## Display-relevante Projektdateien

Der aktuelle Projektstand enthält insbesondere:

```text
kernel/0001-drm-bridge-it6251.patch
kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch
kernel/0003-i2c-imx-debug-arbitration-lost.patch
kernel/0004-i2c-imx-debug-start-state.patch
kernel/it6251.c
kernel/it6251.c.before-drm-lifecycle
```

Diese Dateien müssen zusammen mit

```text
flake.nix
flake.lock
hardware/novena.nix
image/novena-image.nix
```

und der verwendeten Kernel-Konfiguration betrachtet werden, wenn der
funktionierende Displaystand reproduziert oder weiterentwickelt wird.

Für die Device-Tree-Provenienz ist zusätzlich zu beachten, dass der
historische Golden-DTB `e36cd0c8...` aus einem früheren Vor-Git-
Overlay-Zustand stammt und nicht allein aus dem Git-Commit `f26464f`
rekonstruiert werden darf.

## Wiederherstellungsstrategie

Bei späteren Änderungen am Display-, DRM-, I2C- oder Bootpfad soll der
Golden Build als Rückfallreferenz erhalten bleiben.

Vor riskanten Änderungen sollen mindestens gesichert beziehungsweise
geprüft werden:

1. aktueller Git-Stand
2. aktuelle DTB-Hashes
3. aktueller Image-Hash
4. funktionierender Rückfall-DTB
5. externes SD-Testmedium
6. serieller Konsolenzugriff

Das ursprüngliche Golden Backup vom 2026-09-11 bleibt unverändert.

Neue Versuchsstände sollen in separaten Builds, Git-Commits oder
zusätzlichen Backups abgelegt werden.

## Aktuell bestätigter Referenzzustand


Für die weitere Arbeit müssen der historische Golden-Zustand und die
späteren Diagnosezustände getrennt betrachtet werden.

Der historische Golden Build vom 2026-09-11 bleibt unverändert und ist
weiterhin die gesicherte funktionierende Referenz.

Für den später reproduzierten Display-Baseline-Zustand gilt:

* Kernel: Linux 6.18.49
* NixOS: 26.05.20260903.a5cc6f2
* reproduzierter Display-Baseline-DTB:
  `31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`
* Kernel-Basis-DTB:
  `b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`
* historischer Golden-DTB:
  `e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`
* IT6251-Product-ID kann erfolgreich gelesen werden
* Display-Link kann stabil 1920x1080 erreichen
* Linux kann die Displayversorgung selbst einschalten
* `single-master;` ist auf dem Display-I2C-Bus aktiv
* `startup-delay-us = <2000000>` ist auf `reg_display` aktiv

Der anschließend isoliert getestete STMPE811-deaktivierte DTB besitzt:

`d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8`

Der semantische Unterschied gegenüber dem reproduzierten
Display-Baseline-DTB besteht ausschließlich aus:

```dts
status = "disabled";
```

am STMPE811-Knoten.

Der Hardwaretest bestätigte:

* die STMPE811-/`0-0044`-Fehler verschwinden vollständig
* das intermittierende IT6251-Cold-Boot-Problem bleibt bestehen

Damit ist experimentell bestätigt, dass die beobachteten
STMPE811-/Touchscreen-Fehler nicht die Ursache des intermittierenden
IT6251-Cold-Boot-Problems sind.

Die anschließende Fünf-Cold-Boot-Serie ergab:

* 2/5 direkte Display-Erfolge
* 3/5 direkte Display-Fehler
* 3/3 erfolgreiche DRM-Rebind-Recoveries nach einem Fehler

Der aktuelle Diagnosezustand ist deshalb noch kein reproduzierbarer
Produktionsstand.

Insbesondere ist der umfangreiche I2C-Diagnosepfad noch nicht für einen
finalen Produktionskernel geeignet.

Der neue `0004`-Diagnosekernel wurde erfolgreich gebaut und
provenienzgeprüft, aber zum Stand dieses Dokuments noch nicht auf echter
Novena-Hardware getestet.

## Nächste Schritte

Der historische Golden Build bleibt unverändert als bekannte funktionierende Referenz erhalten.

Die spätere Diagnoseentwicklung hat den Cold-Boot-Fehler wesentlich genauer lokalisiert. Der nächste Schritt ist eine getrennte Root-Cause-Analyse des i.MX6-I2C-Master-/START-Übergangs.

Ausgangspunkt:

`Cold Boot FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

gegen:

`Same-Boot Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Vor einer Einstufung eines späteren Entwicklungsstands als Produktionsreferenz sind weiterhin erforderlich:

1. Root Cause des unterschiedlichen START-/IAL-Verhaltens ausreichend nachweisen
2. daraus eine gezielte funktionale Korrektur ableiten
3. Rollback-Artefakte und Backups vor jedem Test prüfen
4. mehrere identische echte Kaltstarts ohne manuelle Recovery bestehen
5. Diagnoseinstrumentierung entfernen oder auf das notwendige Minimum reduzieren
6. erst danach einen neuen finalen Golden- beziehungsweise Produktionsreferenzstand festlegen

Bis dahin bleibt das Backup vom 2026-09-11 die unveränderte historische Golden-Sicherung.
