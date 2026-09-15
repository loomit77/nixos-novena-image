# Test Log

Stand: 2026-09-13

## Frühere NixOS-Tests

Ein älterer NixOS-Ansatz verwendete den Novena-spezifischen Kernel:

`5.7.0-rc2`

Ein früherer Bootversuch erreichte den Kernel, endete jedoch mit:

`VFS: Unable to mount root fs on unknown-block(0,0)`

Damit war nachgewiesen, dass Kernel und Device Tree grundsätzlich
gestartet werden konnten, die Root-/Initrd-Kette aber noch nicht
korrekt war.

## Analyse des bekannten funktionierenden Images

Das bekannte Novena-NixOS-Image wurde untersucht.

Festgestellt wurden:

* Bootloaderbereich vor Partition 1
* FAT-Partition `FIRMWARE`
* ext4-Partition `NIXOS_SD`
* SPL/U-Boot
* Kernel
* Device Tree
* initrd
* U-Boot-Bootskript

Diese Erkenntnisse bilden die Grundlage für das neue universelle Image.

## Linux 6.18.49

Für das aktuelle Image wird Linux 6.18.49 verwendet.

Die Displaybasis besteht aus `0001-drm-bridge-it6251.patch` und
`0002-drm-panel-add-innolux-n133hse-ea1.patch`.

Für die Diagnose kamen später `0003-i2c-imx-debug-arbitration-lost.patch`
und `0004-i2c-imx-debug-start-state.patch` hinzu.

Die Fünf-Cold-Boot-Serie wurde mit `0001` bis `0003` durchgeführt.
`0004` wurde anschließend auf Basis dieser Ergebnisse entwickelt,
erfolgreich gebaut und provenienzgeprüft, aber noch nicht auf der Novena
getestet.

## 2026-09-07 – aktuelles Diagnose-Image gebaut

Das aktuelle NixOS-SD-Image wurde erfolgreich gebaut.

Exit-Code:

`0`

Nix-Store-Ausgabe:

`/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img`

Größe:

`2581291008` Bytes

SHA-256:

`5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b`

Das Image enthält zusätzliche Instrumentierung im IT6251-Treiber und im
i.MX-I2C-Treiber.

## Frühere Displaydiagnose

Bei früheren Tests wurde festgestellt, dass der IT6251 grundsätzlich mit
Linux 6.18.49 angesprochen werden kann.

In einem frühen Diagnosezustand wurden jedoch falsche aktive Bildgrößen
gemessen, beispielsweise:

`hactive: 3504`

`vactive: 671`

Die IT6251-Registerprogrammierung selbst entsprach weitgehend dem
historischen funktionierenden Novena-Treiber.

Im später funktionierenden Device-Tree-Zustand sind für die beiden
LDB-DI-Clock-Selektoren explizite Clock-Zuweisungen vorhanden.

Mit dem späteren Gesamtzustand konnte korrekt gemessen werden:

`hactive: 1920`

`vactive: 1080`

Damit ist belegt, dass der funktionierende spätere Displayzustand die
expliziten Clock-Zuweisungen enthält und eine stabile 1920x1080-Ausgabe
erreicht.

Der genaue einzelne Vor-Git-Test, in dem diese Clock-Zuweisungen
erstmals eingeführt wurden, und ihre isolierte Wirkung gegenüber den
anderen gleichzeitig entwickelten Device-Tree-Korrekturen sind aus den
erhaltenen Aufzeichnungen nicht eindeutig rekonstruierbar.

## I2C-Arbitration-Lost-Diagnose

Historisch wurden auf dem Display-I2C-Bus Meldungen mit `I2SR=0x93` und
Rückgabewert `-11` beobachtet.

Nach Einführung von `single-master;` verschwanden diese sichtbaren
`-11`-/`EAGAIN`-Fehler.

Die spätere Analyse des exakten Linux-6.18.49-Quellcodes zeigte jedoch:

```c
i2c_imx->multi_master =
        !of_property_read_bool(pdev->dev.of_node, "single-master");
```

Damit bedeutet `single-master;` exakt `multi_master = false`.

In diesem Modus werden die normalen IAL-Prüfungen in
`i2c_imx_bus_busy()` und `i2c_imx_trx_complete()` übersprungen. Die
Eigenschaft verhindert daher nicht, dass die Hardware selbst
`I2SR_IAL` setzt.

Die historische Beobachtung wird heute so eingeordnet: `single-master;`
beseitigte die sichtbare softwareseitige `-EAGAIN`-Fehlerbehandlung, ist
aber kein Beweis für das vollständige Verschwinden von
Hardware-Arbitration-Lost.

## Untersuchung von regulator-always-on

Ein vorheriger Teststand enthielt für die IT6251-Versorgung:

`regulator-always-on`

Auch `reg_lvds_lcd` war in einem früheren Overlay-Zustand mit
`regulator-always-on` versehen.

Die Regulator-Diagnose zeigte, dass `lcd-display-power` dadurch auch dann
aktiv blieb, wenn der IT6251-Consumer selbst nicht aktiv war.

Damit bestand die Möglichkeit, dass Linux einen bereits durch U-Boot
initialisierten IT6251-Zustand übernahm, anstatt den Chip selbst aus
einem ausgeschalteten Zustand zu initialisieren.

`regulator-always-on` wurde deshalb für `reg_display` entfernt.

Der Kernel-Basis-DTB enthielt bereits eine Einschaltverzögerung von:

`startup-delay-us = <200000>`

Im späteren Teststand wurde diese auf:

`startup-delay-us = <2000000>`

erhöht.

Ein anschließender Test bestätigte, dass Linux den Regulator tatsächlich
aus- und wieder einschalten konnte und die konfigurierte Verzögerung von
ungefähr zwei Sekunden eingehalten wurde.

## Früherer Test nach echtem Power-Cycle

Bei einem früheren Test nach Entfernen von `regulator-always-on` trat
nach dem Einschalten der IT6251-Versorgung folgendes Verhalten auf:

1. erster I2C-Schreibtransfer: `-110` / `ETIMEDOUT`
2. nachfolgende Zugriffe: `-6` / `ENXIO`
3. IT6251 Product-ID konnte nicht gelesen werden
4. Linux deaktivierte den Regulator nach dem Fehlschlag
5. internes Display blieb aus

Die relevante Sequenz begann mit:

`power_up: regulator_enable`

und ungefähr zwei Sekunden später mit dem ersten Product-ID-Zugriff.

Der erste Zugriff endete mit:

`<i2c_imx_write> write timedout`

Dieser Fehler war der Ausgangspunkt für die erweiterte
I2C-ISR-Instrumentierung des aktuellen Diagnose-Images.

## 2026-09-12 – erfolgreicher Hardwaretest

Am 2026-09-12 wurde das aktuelle Diagnose-Image erneut auf echter
Novena-Hardware kalt gestartet.

Das System erreichte den regulären NixOS-Login.

Der Test zeigte diesmal einen vollständig erfolgreichen Linux-seitigen
Displaystart.

### IT6251 Probe

Der IT6251-Treiber wurde auf:

`i2c-2`

mit Adresse:

`0x5c`

registriert.

Die Probe wurde erfolgreich abgeschlossen:

`IT6251 DRM bridge registered`

`probe: exit success`

### Linux-seitiges Power-Up

Die DRM-Bridge rief anschließend die Power-Up-Sequenz auf:

`bridge_pre_enable: enter`

`power_up: enter powered=0`

`power_up: regulator_enable`

`power_up: regulator enabled`

Danach begann der erste Product-ID-Versuch:

`power_up: product ID attempt 1/5`

Die Kernel-Zeitstempel des erfolgreichen Tests zeigten, dass die
konfigurierte Einschaltverzögerung von ungefähr zwei Sekunden vor dem
ersten IT6251-Zugriff eingehalten wurde.

Anders als beim vorherigen fehlgeschlagenen Test trat diesmal kein
Timeout beim ersten IT6251-Zugriff auf.

### Product-ID erfolgreich

Bereits der erste Product-ID-Versuch war vollständig erfolgreich.

Register `0x00`:

`0x15`

Register `0x01`:

`0xca`

Register `0x02`:

`0x51`

Register `0x03`:

`0x62`

Ergebnis:

`IT6251 detected: vendor ca15 device 6251`

Danach:

`power_up: exit success`

`bridge_pre_enable: exit ret=0 powered=1`

Damit ist nachgewiesen, dass Linux den IT6251 in diesem Test nach dem
Einschalten der Versorgung selbst erfolgreich erreichen konnte.

### Display-Link

Die anschließende IT6251-Initialisierung war erfolgreich.

Das DisplayPort-Linktraining endete nach zehn Iterationen.

Systemstatus:

`0x3e`

Gemessene aktive Auflösung:

`hactive: 1920`

`vactive: 1080`

Erfolgsindikatoren:

`is_stable: stable 1920x1080`

`display link stable`

`bridge_enable: exit success`

Damit wurde auf echter Novena-Hardware ein stabiler interner
1920x1080-Display-Link erreicht.

### Regulatorstatus

Nach erfolgreicher Displayinitialisierung wurden folgende GPIO-Zustände
beobachtet:

`gpio-15 (regulator-lvds-lcd) out hi`

`gpio-28 (regulator-display) out hi`

Die Regulator-Zusammenfassung zeigte sowohl `lcd-lvds-power` als auch
`lcd-display-power` aktiv bei 3300 mV.

Der IT6251 war als Consumer `2-005c-power` eingetragen.

### Erwartete IT6251-Reset-NACKs

Während der Reset-Sequenzen wurden temporär folgende Meldungen
beobachtet:

`error -6 writing to eDP addr 0x5`

`error -6 writing to LVDS addr 0x5`

Die Initialisierung lief anschließend erfolgreich weiter.

Diese Meldungen verhinderten den erfolgreichen Displaystart nicht und
werden für diesen Test als erwartete temporäre Nichtantworten während
der IT6251-Resetsequenz eingeordnet.

### I2C-ISR-Diagnose

Die zusätzliche Instrumentierung zeigte während der erfolgreichen
IT6251-Schreibtransfers wiederholt ISR-Eintritte.

Beispiel:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=0 len=1`

und anschließend:

`NOVENA-I2C: IT6251 ISR enter state=2 I2SR=0xa2 I2CR=0xf8 idx=1 len=1`

Damit wurde für diesen erfolgreichen Test bestätigt, dass die
instrumentierten IT6251-I2C-Transfers bis in den Interruptpfad des
i.MX-I2C-Treibers verfolgt werden konnten.

Die Diagnoseinstrumentierung selbst ist jedoch sehr umfangreich und
kann das Timing beeinflussen.

Sie ist deshalb noch nicht für einen finalen Produktionskernel
geeignet.

## Separates i2c-0-Diagnoseproblem

Nach dem erfolgreichen Displaystart wurden auf `i2c-0` große Mengen von
Meldungen der Form:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

beobachtet.

Später trat dort zusätzlich auf:

`<i2c_imx_write> write timedout`

Dieses Verhalten ist getrennt vom erfolgreichen IT6251-Betrieb auf
`i2c-2` zu betrachten.

Der Diagnose-Patch muss vor abschließenden Reproduzierbarkeitstests
entfernt, entschärft oder auf den tatsächlich benötigten Bus begrenzt
werden.

## STMPE811 / Touchscreen

Die spätere Untersuchung des laufenden Device Trees zeigte, dass der
STMPE811 auf I2C-Adresse `0x44` weiterhin aktiv ist.

Beim Boot wurde er zunächst erfolgreich erkannt:

`stmpe811 detected, chip id: 0x811`

Auch das Touchscreen-Eingabegerät wurde registriert.

Im weiteren Betrieb wurden wiederholt STMPE-I2C-Fehler beobachtet,
darunter:

`stmpe-i2c 0-0044: failed to read regs 0xb: -110`

sowie entsprechende Fehler mit:

* `-6`
* `-11`

Ein Vergleich des historischen Golden-DTB `e36cd0c8...` mit dem später
aktuell gebooteten DTB `31f2b35e...` zeigte, dass der STMPE811-/
Touchscreen-Teilbaum in beiden Zuständen aktiv und inhaltlich
gleich ist.

Damit ist die frühere Annahme widerlegt, dass der funktionierende
Displayzustand durch eine Deaktivierung des STMPE811 entstanden sei.

Displaypfad und STMPE-/Touchscreen-Problem sind getrennt zu behandeln.

Als nächster kontrollierter Test soll nur der STMPE811-/
Touchscreen-Pfad reproduzierbar deaktiviert werden, während der
funktionierende Displaypfad unverändert bleibt.

## Rekonstruktion der Vor-Git-Displayentwicklung

Die Git-Historie des Projekts beginnt mit:

`f26464fac1dc87b6bb07f0c1eac8a0a7f01ed3d7`

Commit:

`Add reproducible Novena NixOS image project`

Der historische Golden-DTB besitzt den SHA-256-Wert:

`e36cd0c8d229c3d34e688e896f468e40761e9240cf99b6e8691a9c5138f4cd6f`

Der später aktuell gebootete DTB besitzt:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

Beide basieren auf demselben Kernel-Basis-DTB:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Die DTB-Unterschiede stammen deshalb aus unterschiedlichen
Overlay-Zuständen.

Der historische `e36cd0c8...`-Zustand enthielt unter anderem:

* `regulator-always-on` auf `reg_display`
* `regulator-always-on` auf `reg_lvds_lcd`
* kein `single-master;`
* keine späteren expliziten Clock-Zuweisungen
* den Basiswert `startup-delay-us = <200000>`

Der später funktionierende `31f2b35e...`-Zustand enthält:

* kein `regulator-always-on` auf `reg_display`
* kein `regulator-always-on` auf `reg_lvds_lcd`
* `single-master;`
* explizite Clock-Zuweisungen
* `startup-delay-us = <2000000>`

Der Commit `f26464f...` enthält bereits den späteren
Device-Tree-Quellzustand mit `single-master`, 2-Sekunden-Delay und den
Clock-Zuweisungen.

Deshalb darf der in der Golden-Build-Sicherung dokumentierte
Git-Commit nicht als Provenienznachweis dafür verwendet werden, dass
der historische DTB `e36cd0c8...` aus exakt diesem committed
Quellzustand gebaut wurde.

Die vorhergehenden Display-Experimente fanden vor der vorhandenen
Git-Historie statt und müssen aus Nix-Artefakten, Device Trees, Backups
und Testaufzeichnungen rekonstruiert werden.


## 2026-09-12 – isolierter STMPE811-Disable-Test

Der STMPE811-/Touchscreen-Pfad wurde mit einem separaten Overlay gezielt
deaktiviert:

```dts
&touch {
        status = "disabled";
};
```

Exakt reproduzierte Baseline:

`31f2b35e9d0f05adbe6b6e07dbe92bf517b4736366ff81aafa7144ea18377e0f`

STMPE-deaktivierter Test-DTB:

`d9e0c8d5554315814143da89a5b661553b6587902add7ca0d2d41139c09544b8`

Der semantische Vergleich zeigte ausschließlich `status = "disabled"`
am STMPE811-Knoten.

Im Hardwaretest verschwanden die zuvor beobachteten STMPE-/`0-0044`-
Fehler vollständig. Das intermittierende IT6251-Displayproblem blieb
jedoch bestehen.

## 2026-09-12 – Fünf-Cold-Boot-Serie

Mit unverändertem STMPE-deaktiviertem Test-DTB wurden fünf echte
Kaltstarts durchgeführt.

* Cold Boot 1: PASS — Boot-ID `79b5de22-7579-4c04-bc5d-26b70eca4ba0`
* Cold Boot 2: PASS — Boot-ID `2d2f940a-d924-46ce-97e991dd82960d65`
* Cold Boot 3: FAIL — Boot-ID `ef1ab7ab-e833-4b19-96da-be150ae1a556`, danach DRM-Rebind PASS
* Cold Boot 4: FAIL — Boot-ID `6d4bc1c8-086e-4261-805a-03e3da1a7e02`, danach DRM-Rebind PASS
* Cold Boot 5: FAIL — Boot-ID `bef4ce6b-f53f-41d6-869b-c12d647214fb`, danach unmittelbarer DRM-Rebind PASS

Gesamt:

* native Cold-Boot-Erfolge: 2/5
* native Cold-Boot-Fehler: 3/5
* erfolgreiche DRM-Rebind-Recoveries nach Fehler: 3/3

Erfolgreiche Starts beziehungsweise Recoveries zeigten typischerweise
`I2SR=0xa2` oder `0xa6` mit `I2CR=0xf8`.

Alle drei fehlgeschlagenen Cold Boots begannen den ersten IT6251-
Transfer mit `I2SR=0x93` und `I2CR=0xd8`; einzelne Timeout-Pfade zeigten
`0x91/0xd8`.

`0x93` enthält unter anderem `ICF`, `IAL`, `IIF` und `RXAK`.
`0xd8` enthält kein `MSTA`.

Die Debugausgabe liest diese Register im ISR-Wrapper vor dem Master-ISR.
Daraus folgt nicht, dass der Transfer ohne MSTA gestartet wurde. Die
Quellcodeanalyse von `i2c_imx_start()` zeigt ausdrücklich, dass MSTA vor
dem Transfer angefordert wird.

Für i.MX6 wird `I2SR_CLR_OPCODE_W0C` verwendet. `i2c_imx_start()` schreibt
vor jedem START `0x00` nach I2SR und löscht damit vorherige Statusbits.
Ein bloß übrig gebliebenes altes IAL-Bit ist deshalb stark entkräftet.

Im ISR-Pfad führt gesetztes RXAK über `i2c_imx_isr_acked()` zu
`-ENXIO`, also `-6`.

Warum das vom i.MX6-I2C-Controller gemeldete IAL-Ereignis in den
fehlgeschlagenen Cold Boots entsteht, ist noch nicht bewiesen.

## 2026-09-13 – Diagnose-Patch 0004

Auf Grundlage des reproduzierbaren `0x93/0xd8`-Fehlermusters wurde
`kernel/0004-i2c-imx-debug-start-state.patch` entwickelt.

Der Patch speichert START-Snapshots A bis E für Adresse `0x5c` ohne an
diesen Punkten selbst zusätzliche Logs auszugeben. Die Ausgabe erfolgt
erst im bereits instrumentierten ISR-Pfad.

Der Patch wurde gegen den exakten Linux-6.18.49-Quellstand mit bereits
angewendetem `0003` geprüft. Dry-Run: `PATCH_DRY_RUN_STATUS=0`.

Erfolgreicher Kernel-Build:

`/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Deriver:

`/nix/store/qk21vdydrywnkqlh3rrvg7n9zn4pwapk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Quelle:

`/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz`

Die Derivation enthält bestätigt `0001`, `0002`, `0003`, `0004`.

Separate Ausgaben:

* `/nix/store/4c7nvncjad0sbgahcngalj8nn0la7njh-linux-armv7l-unknown-linux-gnueabihf-6.18.49-dev`
* `/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49`
* `/nix/store/z1dl1i2pkqzz3p3cg1y9s7knhmbwzczy-linux-armv7l-unknown-linux-gnueabihf-6.18.49-modules`

Das Modules-Output enthält `lib/modules/6.18.49`.

Der anschließend durchgeführte Hardwaretest ist im folgenden Abschnitt
dokumentiert.

## 2026-09-13 – Hardwaretest Diagnose-Patch 0004 mit single-master

Der `0004`-Kernel wurde auf echter Novena-Hardware getestet.

Kernel:

`/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

SHA-256 des `zImage`:

`23156ad9ae38cc8db808cdc337b4a80d9f3e1e65076080ab79a7778cf7b43961`

Beim nativen Cold Boot blieb das Display dunkel. Boot-ID:

`aa5c3bfe-51ae-46f6-9a73-90e7428bccff`

START-Snapshots:

`A=81/80 B=93/80 C=93/80 D=93/d8 E=93/d8 IRQ=93/d8`

A enthält noch kein `IAL`. Unmittelbar nach dem `MSTA`-Versuch zeigt B `IAL`, während `MSTA` bereits nicht mehr gesetzt ist. Die Divergenz liegt damit vor dem ersten Adressbyte.

Der Same-Boot-Rebind war erfolgreich. Erfolgreiche START-Snapshots:

`A=81/80 B=81/a0 C=81/a0 D=81/f8 E=81/f8 IRQ=a2/f8`

beziehungsweise `IRQ=a6/f8`.

## 2026-09-13 – isolierter Test ohne single-master

`single-master;` wurde isoliert aus `i2c3` entfernt; STMPE811 blieb deaktiviert.

DTB-SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Der native Cold Boot schlug weiterhin fehl. Boot-ID:

`f32745ef-b5dc-459a-ad80-f3b268dc950f`

Der Treiber meldete `NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`; der IT6251-Pfad endete mit `SEND failed ret=-11`.

Der Same-Boot-Rebind war erfolgreich:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8 IRQ=a2/f8`

beziehungsweise `IRQ=a6/f8`.

Damit ist bestätigt, dass `single-master;` das vom i.MX6-I2C-Controller gemeldete IAL-Ereignis nicht verhindert und nicht dessen Ursache ist. Die Eigenschaft verändert die Treiberbehandlung.

Archiv `novena-0004-no-single-master-test-2026-09-13.tar.gz`, SHA-256:

`653ca502f83d4a3c6d8afc2f7ab36098eae3c6196b7e4e86215360ce30103766`

## 2026-09-13 – Diagnose-Patch 0005

Für die Ausgabe der START-Snapshots auf dem frühen `-EAGAIN`-Fehlerpfad wurde `kernel/0005-i2c-imx-debug-start-error.patch` ergänzt.

Patch-SHA-256:

`81c2de96d7200a6e7e2684518c711bc5686b8d5295a52d5e7aaa54efed94e556`

Kernel:

`/nix/store/97pmqlp5xvsh1l0i877lfc4r4sq6i7np-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Deriver:

`/nix/store/lakip7c5jxqj4ys7vgwzh1fcspk465vx-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

SHA-256 des `zImage`:

`197ce0ef325307a877eb5a889519387be91fdf536da3bee03883f0c40264fc7c`

## 2026-09-13 – 0005 Cold Boot FAIL und Same-Boot-Rebind PASS

Boot-ID:

`48462ba4-4972-4d23-9261-e69c822ed584`

Der native Cold Boot schlug fehl. STMPE811 war `disabled`; `single-master` war nicht vorhanden.

Reproduzierbarer Fehlerpfad:

`NOVENA-I2C: IT6251 START failure A=81/80 B=93/80 C=83/80 ret=-11`

A liegt vor dem Master-Startversuch, B unmittelbar danach und zeigt frisches `IAL` bei bereits fehlendem `MSTA`. C wurde nach `bus_busy()` erfasst, nachdem dort `IAL` gelöscht wurde. Der Rückgabewert ist `-EAGAIN`.

Der Same-Boot-Rebind war erfolgreich:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8 IRQ=a2/f8`

beziehungsweise `IRQ=a6/f8`.

Der IT6251 erreichte anschließend `System status: 0x3e`, `hactive: 1920`, `vactive: 1080`, `display link stable` und `bridge_enable: exit success`.

Direkter Vergleich:

`Cold Boot FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

`Rebind PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Damit ist die entscheidende Cold-Boot-Divergenz auf den Eintritt in den Master-/START-Zustand lokalisiert. Die tiefere Root Cause bleibt offen.

Gesicherte SHA-256-Werte:

* nativer Kernel-Log: `d3362fcc09c7a896a567ad8e29ce30e9c470e53dbf6903c44b690e50f5438692`
* Boot-ID-Datei: `10335015dc278fe8cae18b8dfc80dd70398e3f76946c010da251ee73a3662ac3`
* Ergebnisdatei: `12e317c3cdb9e7a2aa9138c05d02e5d5e62236c2ebc2b87dbbc49feca2788740`
* Post-Rebind-Kernel-Log: `26cc1df43f0a222c4011e7003911ccff8dfa362dbf694ec2981d565216d282bc`
* Post-Rebind-Ergebnisdatei: `795bd21294c918b72e0187157e75ac4602a05d7b35bd112f8bdaae8ec24ed7e2`

Archiv `novena-0005-no-single-master-test-2026-09-13.tar.gz`, SHA-256:

`0c38c954a7fac033b5fd619653efd8eb0d5fffa3c4024329cb6d85d247e1c3db`

Der Hash wurde auf Novena und foobox identisch verifiziert; `gzip -t` meldete `GZIP: OK`.

## Nächste Testschritte

Der `0004`-/`0005`-Diagnoseblock ist abgeschlossen und extern gesichert.

Die nächste Phase ist eine gezielte Root-Cause-Analyse des Master-/START-Übergangs.

Ausgangspunkt:

`FAIL: A=81/80 B=93/80 C=83/80 ret=-11`

gegen:

`PASS: A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

Zu untersuchen sind insbesondere Buszustand und mögliche SDA-/SCL-Bedingungen beim ersten START, Controller-, Pinmux- und Clock-Zustand vor Cold-Boot-START und Rebind, Initialisierungsreihenfolge und mögliche Interaktion mit anderen I2C-Geräten beziehungsweise Controllern sowie das zusätzlich beobachtete Arbitration-Lost-Verhalten auf `i2c-0`.

Eine funktionale Korrektur soll erst vorgenommen werden, wenn eine konkrete Ursache ausreichend begründet ist. Danach ist erneut eine echte Cold-Boot-Stabilitätsserie erforderlich.

Die Diagnoseinstrumentierung bleibt vorerst ein Entwicklungswerkzeug und ist nicht als Produktionsstand einzustufen.

## 2026-09-15 – Diagnose-Patch 0006

Auf Grundlage des mit `0005` eingegrenzten Fehlerfensters wurde
`kernel/0006-i2c-imx-debug-start-transition.patch` entwickelt.

Ziel des Patches ist es, den Übergang unmittelbar nach dem Setzen von
`MSTA` feiner aufzulösen.

Der Patch erfasst acht direkt aufeinanderfolgende Registerpaare
`M0` bis `M7` aus `I2SR` und `I2CR`.

Innerhalb dieses Messfensters befinden sich absichtlich:

* keine Delays
* keine Kernel-Logs
* keine Timestamp-Abfragen

Die 16 MMIO-Lesezugriffe selbst können das Timing prinzipbedingt
beeinflussen. `0006` ist deshalb weiterhin reine
Diagnoseinstrumentierung und keine funktionale Korrektur.

Patch-SHA-256:

`23a0e0c00860565fca1d8ccb1546b4d8e893d0f423e6c450013e76a47b70903f`

Kernel-Deriver:

`/nix/store/2g78mma5gxjvr9g6mci7c66mkcy2yssk-linux-armv7l-unknown-linux-gnueabihf-6.18.49.drv`

Kernel:

`/nix/store/f03kxwghama2l7if4p1798fvcyrhvwjj-linux-armv7l-unknown-linux-gnueabihf-6.18.49`

Separate Ausgaben:

* `dev`: `/nix/store/ay6ds8zsb4bxm284xjvw4vydp09w3dv0-linux-armv7l-unknown-linux-gnueabihf-6.18.49-dev`
* `modules`: `/nix/store/08ls92b1vpj0v2nqa3ssygdms6zsslbb-linux-armv7l-unknown-linux-gnueabihf-6.18.49-modules`

SHA-256 des `zImage`:

`f5cb8d264c14286e53a0ea5884e167c7d4815c529046a71e1813cb37512302d5`

Kernel-Basis-DTB SHA-256:

`b560d50c186e0953cc1b9042ca991ffed7609db2d00379446e4286c252e47522`

Finaler Overlay-DTB SHA-256:

`0058510ff34cae32185b8d1b27e6514d0e140974f9aff5542ccc2311220ae081`

Der finale DTB ist bytegleich mit dem bereits getesteten
No-`single-master`-DTB. Damit blieb die Device-Tree-Seite für den
`0006`-Vergleich konstant.

STMPE811 blieb deaktiviert und `single-master;` blieb auf `i2c3`
entfernt.

## 2026-09-15 – 0006 echter POR-Cold-Boot FAIL

Der Hardwaretest wurde aus vollständig ausgeschaltetem Zustand mit
eingesetzter externer Test-SD durchgeführt.

U-Boot meldete:

`Reset cause: POR`

Damit ist der Test als echter Power-On-Cold-Boot bestätigt.

Boot-ID:

`e904173c-7b90-4f71-b3da-e8e450528670`

Das interne Display blieb beim nativen Boot dunkel.

Der bekannte frühe Fehlerpfad blieb erhalten:

`NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93`

`NOVENA-I2C: IT6251 START failure A=81/80 B=93/80 C=83/80 ret=-11`

Die neue `0006`-Messung zeigte bei den fehlgeschlagenen
Product-ID-Zugriffen reproduzierbar:

`M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80`

Damit gilt:

* A unmittelbar vor dem `MSTA`-Versuch ist `81/80`.
* Bereits die erste beobachtbare Probe nach dem `MSTA`-Schreibzugriff,
  M0, ist `93/80`.
* `IAL` ist bei M0 bereits gesetzt.
* `MSTA` ist bei M0 bereits wieder gelöscht.
* M0 bis M7 bleiben in diesem Zustand.

Das IAL-Ereignis entsteht damit spätestens zwischen dem
Pre-MSTA-Snapshot A und der ersten beobachtbaren Post-MSTA-Probe M0.

`0006` konnte keinen späteren Übergang innerhalb M0 bis M7 beobachten,
weil der fehlerhafte Zustand bereits bei M0 vollständig vorhanden war.

Der IT6251-Product-ID-Zugriff scheiterte anschließend vollständig und
der Displaypfad wurde nicht aktiviert.

Die tiefere Ursache dafür, warum der i.MX6-I2C-Controller bei diesem
Cold Boot unmittelbar beim Master-/START-Eintritt IAL meldet, ist damit
noch nicht bewiesen.

## 2026-09-15 – 0006 Same-Boot-Recovery

Im exakt gleichen Boot wurde zunächst nur der IT6251-I2C-Treiber
unbind/bind ausgeführt.

Dieser Schritt registrierte die DRM-Bridge erneut, aktivierte jedoch
nicht den vollständigen Displaypfad. Es entstand kein neuer
Product-ID-Transfer und damit auch keine für den A/B-Vergleich
verwertbare neue START-Sequenz.

Anschließend wurde der Plattformtreiber `imx-ldb` im selben Boot
unbind/bind ausgeführt.

Dadurch wurde der vollständige Displaypfad erneut aktiviert.

Die IT6251-Initialisierung war nun erfolgreich.

Die erfolgreichen START-Transition-Messungen unterschieden sich bereits
bei M0 fundamental vom Cold-Boot-Fehler.

Typischer erfolgreicher Beginn:

`M0=81/a0`

Die folgenden Samples blieben zunächst `81/a0`; bei einzelnen
Transaktionen erschien innerhalb der späteren Samples bereits
`a1/a0`.

Die erfolgreichen START-Snapshots entsprachen:

`A=81/80 B=81/a0 C=a1/a0 D=a1/f8 E=a1/f8`

mit erfolgreichen IRQ-Zuständen wie:

`IRQ=a2/f8`

beziehungsweise:

`IRQ=a6/f8`

Der IT6251 wurde erfolgreich angesprochen, das Linktraining
abgeschlossen und der Displaypfad erreichte erneut:

`System status: 0x3e`

`hactive: 1920`

`vactive: 1080`

`display link stable`

`bridge_enable: exit success`

Damit liegt innerhalb derselben Boot-Session ein besonders direkter
A/B-Vergleich vor:

Cold Boot FAIL:

`A=81/80 -> M0=93/80`

Same-Boot LDB-Rebind PASS:

`A=81/80 -> M0=81/a0`

Der Pre-MSTA-Zustand A ist gleich. Die entscheidende Divergenz ist
bereits bei der ersten beobachtbaren Probe nach dem MSTA-Schreibzugriff
vorhanden.

Der Fehler ist damit sehr eng mit dem Master-/START-Erwerb verbunden.
Eine Root Cause ist weiterhin nicht bewiesen.

## 2026-09-15 – Sicherung der 0006-Evidence

Die vollständige Same-Boot-Evidence wurde vor dem Ausschalten der
Novena auf die foobox übertragen.

Die acht in `SHA256SUMS` erfassten Evidence-Dateien wurden auf der
foobox mit:

`sha256sum -c SHA256SUMS`

vollständig verifiziert.

Alle Dateien meldeten:

`OK`

Die gesicherte Boot-ID lautet:

`e904173c-7b90-4f71-b3da-e8e450528670`

Evidence-Verzeichnis auf der foobox:

`/home/loomit/novena-backups/i2c-0006-evidence-e904173c-2026-09-15`

Zusätzlich wurde ein Archiv erstellt:

`/home/loomit/novena-backups/novena-i2c-0006-evidence-e904173c-2026-09-15.tar.gz`

SHA-256 des Archivs:

`2b4b635ecfb693a2d7471efd1c67e9ccc65e240d8a08b207cd3e9805750d0ee8`

Nach erfolgreicher Übertragung und Hash-Verifikation wurde die Novena
sauber heruntergefahren und ausgeschaltet.

### Zeitstempel-Hinweis

Die Novena protokollierte bei diesem Test teilweise den 13. September,
obwohl der physische Test am 2026-09-15 durchgeführt wurde.

Diese Wallclock-Zeit ist deshalb für die Provenienz dieses Tests nicht
maßgeblich.

Für die eindeutige Zuordnung werden insbesondere verwendet:

* Boot-ID `e904173c-7b90-4f71-b3da-e8e450528670`
* monotone Kernel-Zeitstempel innerhalb des Boots
* SHA-256-verifizierte Evidence-Dateien
* das am 2026-09-15 auf der foobox erzeugte Evidence-Archiv

## Checkpoint nach Diagnose-Patch 0006

Der Diagnoseblock `0006` ist abgeschlossen.

Bestätigter Kernbefund:

`Cold FAIL: A=81/80 -> M0=93/80`

gegen:

`Same-Boot LDB-Rebind PASS: A=81/80 -> M0=81/a0`

Beim Cold-Boot-Fehler ist das frische IAL somit bereits bei der ersten
beobachtbaren Probe nach dem MSTA-Schreibzugriff vorhanden und MSTA
bereits wieder gelöscht.

Die Root Cause dieses unterschiedlichen Hardwareverhaltens ist weiterhin
offen.

Insbesondere wurde noch keine funktionale Retry-, Delay- oder
Bus-Recovery-Lösung als eigentliche Fehlerbehebung eingeführt.

Die Diagnoseinstrumentierung `0003` bis `0006` bleibt
Entwicklungsinstrumentierung und ist nicht als Produktionsstand
einzustufen.

Der Diagnoseblock `0006` bildet den technischen Inhalt des
zugehörigen Git-Checkpoints. Der Checkpoint gilt als versionskontrolliert
abgeschlossen, sobald der ihn enthaltende aktuelle Commit auf beide
Projekt-Remotes übertragen, dort identisch verifiziert und der lokale
Worktree als sauber bestätigt wurde.

Erst danach soll die Entwicklung eines weiteren Diagnose-Patches oder
einer funktionalen Korrektur beginnen.

Für einen Wiedereinstieg nach einer Pause gelten Git, die committed
Projektdokumentation und die gesicherten Testartefakte als maßgebliche
Quelle. Gesprächserinnerungen dienen nur als zusätzliche Orientierung
und dürfen verifizierten Repository- oder Messdaten nicht vorgezogen
werden.
