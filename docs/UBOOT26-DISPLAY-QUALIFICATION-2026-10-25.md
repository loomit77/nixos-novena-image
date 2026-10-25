# Novena – U-Boot 2026 und Displayvariante

Datum: 2026-10-25

Block: NOVENA-DOC-CONSOLIDATE-90

Status: Offline-Build- und Artefaktqualifikation;
kein erfolgreicher Hardwareboot nachgewiesen.

## 1. Quellprovenienz

Baseline-Commit:

262c2d52dbb91cef8f77dacb309d5abd2862751c

U-Boot: 2026.07

Aktive U-Boot-Patches:
0001–0008, 0014 und 0015.

Die Patches 0009–0013 sind keine aktiven Bestandteile
dieses Buildstands.

Die Displayvariante stammt aus einem separaten
Git-Archiv-Snapshot desselben Commits.

Quellverzeichnis auf foobox:

~/novena-display-variant-84/source

Anwendung der Displayänderung nur in diesem Snapshot.
Das ursprüngliche Repository blieb unverändert.

## 2. Baseline-Builds

VM102: nixos-base

Build abgeschlossen:
2026-10-24 15:05:51 +02:00

VM104: novena-clean-builder

Build abgeschlossen:
2026-10-24 14:16:06 +00:00
= 2026-10-24 16:16:06 +02:00

Beide:
BUILD_STATUS=PASS
BUILD_EXIT_CODE=0

Imagegröße:
3240607744 Bytes

Image-SHA256:
c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750

Die vollständigen Imagebytes sind aufgrund
gleicher verifizierter SHA256-Werte byteidentisch
mit kryptografisch sehr hoher Sicherheit.

VM102 verwendete ein bereits vorhandenes
Nix-Store-Ergebnis. VM104 führte im protokollierten
Build zahlreiche Derivationen aus.

Daraus folgt keine vollständige Unabhängigkeit
jedes einzelnen Abhängigkeits-Builds.

## 3. Displayvariante auf foobox

BUILD_STATUS=PASS
BUILD_EXIT_CODE=0

Abschluss:
2026-10-24 15:32:30 +02:00

Imagegröße:
3240607744 Bytes

Image-SHA256:
99c34a52484e3d43598fe6487c97d76fb5bdafc58efb934769b6923f7863d5a9

Im separaten Quellsnapshot wurden ausschließlich
zwei Device-Tree-Eigenschaften ergänzt:

reg_display:
regulator-always-on;

reg_lvds_lcd:
regulator-always-on;

Die bestehende startup-delay-us-Einstellung
von 2000000 blieb unverändert.

## 4. Vergleich der Artefakte

Die ersten 8 MiB einschließlich des
SPL-Bereichs sind identisch.

SHA256 der ersten 8 MiB:

5206f64ee67b7efb8138b1b2bc24966e671ebc76483bba45e8c1078df48cb0f1

Die folgenden Firmwaredateien sind identisch:

u-boot-dtb.img:
b45648b6ef6d243d084b6580870613fe43c4d4886c2644b60f5c0b7c15fb97c4

zImage:
20a10cebceb35e44440f626a80d1b69245b21d917023dc31998841569af21bc6

Die Device-Tree-Dateien unterscheiden sich:

Baseline-novena.dtb:
d9eaa356fab80e5d205972683c83d21f72c23ee8b968163934f0f2ef97179cc1

Displayvariante-novena.dtb:
69f6b4f3cccbe3c6204dace560c43816e8b88082e2710463bfec4dcf74ea9232

Der strukturelle DTB-Vergleich aus Block 87 ergab:

DTB_ADDED_ALWAYS_ON_COUNT=2
DTB_REMOVED_ALWAYS_ON_COUNT=0
DTB_OTHER_PROPERTY_CHANGES=0
DTB_TARGET_REGULATOR_CHECK=PASS

Die beiden Ergänzungen befinden sich an:

reg_display: regulator-display
regulator-name = "lcd-display-power"

reg_lvds_lcd: regulator-lvds-lcd
regulator-name = "lcd-lvds-power"

## 5. Bootskripte

boot.cmd unterscheidet sich in den beiden
Imagevarianten byteweise.

Nach Normalisierung der NixOS-Systempfade sind
die Skripttexte gleich.

Die eingebetteten boot.scr-Skripte sind nach
derselben Normalisierung ebenfalls gleich.

Der Legacy-U-Boot-Skriptheader beträgt 64 Bytes.

Das Skript enthält einen 420-Byte-Payload:

- 4 Bytes Skriptlänge: 412
- 4 Bytes Nullwort
- 412 Bytes eigentlicher Skripttext

Das acht Byte lange Präfix lautet:

0000019c00000000

Block 88 bestätigte für die Baseline:

BASELINE_EMBEDDED_EQUALS_BOOT_CMD=True

Block 89 bestätigte für die Displayvariante:

EMBEDDED_EQUALS_BOOT_CMD=True

Damit ist die zuvor beobachtete Differenz
zwischen der Größe von boot.cmd und der
boot.scr-Payloadlänge technisch erklärt.

Die automatische Abschlussausgabe der Blöcke
88 und 89 schlug jedoch aufgrund eines
NameError in den Prüfscripten fehl.

Diese beiden historischen Blockergebnisse
bleiben FAIL. Die vor dem Fehler ausgegebenen
Messwerte sind separat ausgewertet.

## 6. Qualifikationsgrenzen

Erfolgreich belegt:

- Zwei gleiche U-Boot-2026-Baseline-Images
- Erfolgreicher foobox-Displayvarianten-Build
- Gültige Image-SHA256-Nachweise
- Identischer roher SPL-Bereich
- Identisches U-Boot proper
- Identischer Kernel
- Genau zwei beabsichtigte DTB-Ergänzungen
- Bootskript-Präfix technisch erklärt

Nicht belegt:

- Erfolgreicher externer Hardwareboot mit
  dem aktuellen U-Boot-2026-Image
- Funktionale DDR-/SPL-Qualifikation auf
  der realen Novena für diesen Stand
- Funktionale Beseitigung der Display-Dunkelphase
- Vollständige Unabhängigkeit aller Buildabhängigkeiten
- Releasefreigabe für den Hardwareeinsatz

Der Displaypatch ist ein offline qualifizierter
Hardwaretest-Kandidat, kein bestätigter
Display-Root-Cause-Fix.

## 7. Nächste Schritte

1. Keine erneute Untersuchung des geklärten
   Acht-Byte-Skriptpräfixes ohne neue Evidenz.

2. SPL-/DDR-/U-Boot-2026-Qualifikation mit
   den vorhandenen Artefakten weiterführen.

3. Einen späteren echten Novena-Boot und einen
   Display-A/B-Test ausschließlich nach
   gesonderter Freigabe durchführen.

4. Dokumentationsänderungen vor einem Commit
   getrennt prüfen. Kein automatischer Push.

## 8. Originalprotokolle

Privates GitHub-Archiv:

loomit77/project-block-logs

Pfad:

novena/2026/10/

Relevante Blöcke:

- NOVENA-UBOOT26-DUAL-BUILD-78
- NOVENA-UBOOT26-BUILD-STATUS-79
- NOVENA-UBOOT26-BUILD-START-80 – FAIL
- NOVENA-UBOOT26-START-DIAG-81
- NOVENA-UBOOT26-BUILD-START-82
- NOVENA-UBOOT26-BUILD-STATUS-83
- NOVENA-DISPLAY-VARIANT-84 – FAIL
- NOVENA-DISPLAY-VARIANT-84A
- NOVENA-THREE-BUILD-STATUS-85
- NOVENA-DISPLAY-QUALIFY-86
- NOVENA-DISPLAY-QUALIFY-87
- NOVENA-STATUS-AND-BOOTSCRIPT-88 – FAIL
- NOVENA-BOOTSCRIPT-FINAL-89 – FAIL

Original-TXT und SHA256-Sidecars bleiben
unverändert. Diese Zusammenfassung ersetzt
keinen Originalnachweis.

NOVENA-DOC-CONSOLIDATE-90
