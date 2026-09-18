# Block 3.14A – Power-/I2C3-Zustandsrekonstruktion

## 1. Ziel

Block 3.14A rekonstruiert die zeitliche und logische Ereigniskette zwischen
der automatischen Abschaltung von `es8328-power` und der später
log-sichtbaren I2C3-Fehlersignatur.

Die Untersuchung ist eine reine Quellen- und Logrekonstruktion.

Es wurden keine neuen Kernel-Patches, Builds, Bootversuche oder
Hardwaremessungen durchgeführt.

## 2. Ausgangspunkt

Block 3.13F hatte den kausalen Bereich bereits stark auf die
ES8328-/Audio-Power-Domain eingegrenzt.

Insbesondere war bereits belegt:

- Linux schaltet im Baseline-Cold-FAIL `es8328-power` ab.
- Später tritt die instrumentierte I2C3-Signatur
  `A=81/80 -> M0=93/80` mit `arbitration lost` auf.
- Mit `regulator-always-on` bestanden fünf von fünf vorregistrierten
  echten POR-Kaltstarts.
- Die Codec-seitigen I2C-Pull-ups können aus `P3.3V_DELAYED` versorgt
  bleiben, während die ES8328-/Audio-Power-Domain abgeschaltet ist.
- Der genaue elektrische Mechanismus dieses Grenzzustands ist nicht
  bewiesen.

Block 3.14A sollte deshalb nicht erneut die Beteiligung des ES8328
testen, sondern den Software-, Power- und Buszustand zeitlich exakt
rekonstruieren.

## 3. Exakter Linux-6.18.49-Regulatorpfad

Als exakte Kernelquelle des untersuchten Linux-6.18.49-Stands wurde das
bereits in der Projektdokumentation referenzierte Nix-Store-Archiv
verwendet:

`/nix/store/z4dyijrrjydyb7avcwm7vp5vadrmwqkv-linux-6.18.49.tar.xz`

Das enthaltene Kernel-Makefile bestätigt:

- `VERSION = 6`
- `PATCHLEVEL = 18`
- `SUBLEVEL = 49`

Die direkte Analyse von `drivers/regulator/core.c` zeigt für diesen
exakten Kernelstand folgenden Ablauf.

`regulator_init_complete()` setzt bei vorhandenem Device Tree
`has_full_constraints = true` und plant anschließend:

`schedule_delayed_work(&regulator_init_complete_work, msecs_to_jiffies(30000));`

Der Cleanup wird damit 30000 ms nach diesem Late-Initcall eingeplant.

`regulator_init_complete_work_function()` ruft anschließend für die
Regulatoren `regulator_late_cleanup()` auf.

Dort wird ein Regulator nicht automatisch abgeschaltet, wenn unter
anderem:

- `always_on` gesetzt ist,
- sein Status nicht verändert werden darf,
- `use_count` ungleich null ist,
- oder er bereits nicht aktiviert ist.

Für einen aktivierten, unbenutzten und abschaltbaren Regulator unter
vollen Constraints wird dagegen unmittelbar vor der Abschaltung
ausgegeben:

`rdev_info(rdev, "disabling
");`

Danach folgt:

`_regulator_do_disable(rdev);`

Damit kann die Baseline-Meldung:

`[   33.761742] es8328-power: disabling`

direkt dem Late-Cleanup des Regulator-Core von Linux 6.18.49
zugeordnet werden.

Die Meldung stammt nicht aus dem ES8328-Codec-Treiber.

## 4. Bedeutung von `regulator-always-on`

Die erfolgreiche H3-1R-Konfiguration ergänzt am bestehenden
`reg_audio_codec`:

`regulator-always-on;`

Der exakte Linux-6.18.49-Regulator-Core prüft bereits am Anfang von
`regulator_late_cleanup()`:

`if (c && c->always_on)`

und beendet für diesen Regulator den Cleanup.

Damit ist direkt aus dem verwendeten Kernelquellcode erklärt, warum die
H3-1R-Konfiguration genau den automatischen Abschaltpfad verhindert,
der im Baseline-Boot die Meldung `es8328-power: disabling` erzeugt.

Dies ergänzt die experimentelle H3-1R-Evidenz, ersetzt sie aber nicht.

## 5. Modellierte Versorgung des ES8328

Der historische Novena-Device-Tree ordnet dem ES8328 alle vier
modellierten Codec-Versorgungen demselben Regulator zu:

- `DVDD-supply = <&reg_audio_codec>;`
- `AVDD-supply = <&reg_audio_codec>;`
- `PVDD-supply = <&reg_audio_codec>;`
- `HPVDD-supply = <&reg_audio_codec>;`

Auch `audio-amp-supply` verweist auf `reg_audio_codec`.

Der Regulator trägt den Namen:

`es8328-power`

und wird über den historischen Board-Pfad mit GPIO5_17 gesteuert.

Die im Device Tree modellierten `5000000` Mikrovolt dürfen nicht als
reale 5-V-Versorgung der Codec-Pins interpretiert werden.

Die Schaltung zeigt als reale geschaltete Audio-Versorgung
`AUD_P3.3V`.

Für die weitere Rekonstruktion werden deshalb getrennt betrachtet:

- `reg_audio_codec` als Linux-Regulator-Abstraktion,
- `es8328-power` als Regulatorname,
- GPIO5_17 als historischer Enable-/Steuerpfad,
- `AUD_P3.3V` als reale geschaltete Audio-Versorgung.

## 6. Reale I2C3-/Power-Topologie

Die Schaltungsanalyse zeigt für den ES8328-Zweig:

`I2C3_SCL -> R26A 330R -> AUD_I2C3_SCL -> ES8328E`

`I2C3_SDA -> R27A 330R -> AUD_I2C3_SDA -> ES8328E`

Auf der Codec-Seite liegen:

- R10B = 1 kOhm Pull-up an `P3.3V_DELAYED`
- R11B = 1 kOhm Pull-up an `P3.3V_DELAYED`

Der ES8328 selbst liegt dagegen auf der geschalteten
Audio-Versorgung `AUD_P3.3V`.

Die Audio-Versorgung wird über den dokumentierten Schaltpfad mit Q11A,
Q10A und Q12A kontrolliert.

Damit existiert ein realer Power-Domain-Grenzzustand, in dem:

- die ES8328-/Audio-Versorgung abgeschaltet ist,
- die Codec-seitigen I2C-Leitungen aber weiterhin über
  `P3.3V_DELAYED` hochgezogen werden können.

Diese Aussage beschreibt die dokumentierte Topologie.

Sie beweist noch keinen bestimmten internen Clamp-, Backfeed- oder
Transientenmechanismus.

## 7. Chronologische Rekonstruktion des Baseline-Boots

Die relevanten monotonic Kernel-Zeitstempel des gesicherten
Baseline-Cold-FAIL sind:

| Zeit | Ereignis | Aussage |
| --- | --- | --- |
| 33.761742 s | `es8328-power: disabling` | Linux-6.18.49-Regulator-Late-Cleanup schaltet den unbenutzten Audio-Regulator ab |
| 42.097171 s | IT6251 `bridge_attach: enter` | späterer Display-Bridge-Lifecycle beginnt |
| 42.097215 s | downstream attach 0 | Displaypfad wird weiter aufgebaut |
| 42.097263 s | imx-drm bindet LDB | DRM-Komponenten werden verbunden |
| 42.114171 s | DRM initialisiert | Display-Subsystem erreicht nächste Initialisierungsstufe |
| 42.395240 s | `bridge_pre_enable: enter` | IT6251-Pre-Enable beginnt |
| 42.395306 s | `power_up: enter powered=0` | IT6251-Power-Up-Pfad beginnt |
| 42.395351 s | `power_up: regulator_enable` | IT6251 fordert seine Versorgung an |
| 43.172998 s | `imx-es8328 sound: Unable to register: -517` | Sound-Gerät meldet `EPROBE_DEFER`; kein Kausalitätsbeweis für I2C3 |
| 44.471549 s | IT6251-Regulator aktiviert | IT6251-Power-Up erreicht aktivierte Versorgung |
| 44.471609 s | `product ID attempt 1/5` | erster log-sichtbarer Registerzugriffsversuch des späteren Fehlerpfads |
| 44.478167 s | `arbitration lost`, `I2SR=0x93` | erste log-sichtbare problematische I2C3-Transaktion |
| 44.478235 s | `START failure A=81/80 B=93/80 C=83/80 ret=-11` | instrumentierte START-Fehlersignatur |

Der Abstand zwischen:

`es8328-power: disabling`

und der ersten log-sichtbaren Arbitration-Loss-Meldung beträgt:

`10.716425 s`

Dieser Wert ist keine nachgewiesene elektrische Fehlerlatenz.

Er beschreibt ausschließlich den Abstand zwischen zwei
log-sichtbaren Ereignissen.

## 8. Zustand vor der Regulator-Abschaltung

Vor `33.761742 s` ist der Audio-Regulator im für den Cleanup relevanten
Sinn aktiviert.

Aus dem exakten Linux-6.18.49-Code und der tatsächlich ausgegebenen
`disabling`-Meldung folgt für den Cleanup-Zeitpunkt:

- `always_on` war nicht gesetzt,
- der Regulator durfte seinen Status ändern,
- `use_count` war null,
- der Regulator wurde als aktiviert erkannt,
- volle Constraints waren aktiv.

Der vorhandene Log zeigt vor der Abschaltung keine bereits bestehende
instrumentierte `93/80`-Fehlersignatur.

## 9. Zustand bei der Regulator-Abschaltung

Bei `33.761742 s` erreicht `es8328-power` den
Linux-6.18.49-Late-Cleanup.

Der Kernel protokolliert `disabling` und führt danach den
Regulator-Disable-Pfad aus.

Auf Board-Ebene ist dieser Software-Regulator der
ES8328-/Audio-Power-Domain zugeordnet.

Damit wechselt die untersuchte Konfiguration in den dokumentierten
Power-Domain-Grenzzustand:

- Audio-/Codec-Versorgung aus,
- externe Codec-seitige I2C-Pull-ups potenziell weiterhin aus
  `P3.3V_DELAYED` versorgt.

## 10. Zustand nach der Regulator-Abschaltung

Zwischen `33.761742 s` und dem späteren IT6251-Zugriff ist im
vorhandenen Log kein weiterer I2C3-Transfer nachweisbar.

Diese Formulierung ist wesentlich.

Aus dem Fehlen einer Logmeldung darf nicht geschlossen werden, dass
physikalisch oder softwareseitig garantiert kein I2C3-Transfer
stattgefunden hat.

Der nächste für die Rekonstruktion klar sichtbare problematische
I2C3-Zugriff entsteht im späteren IT6251-Power-Up-/Product-ID-Pfad.

Zwischen:

`product ID attempt 1/5` bei `44.471609 s`

und:

`arbitration lost` bei `44.478167 s`

liegen ungefähr:

`0.006558 s`

Damit liegt die erste log-sichtbare problematische Busreaktion sehr nah
am späteren Registerzugriffsversuch und nicht unmittelbar an der
33.761742-s-Regulator-Meldung.

## 11. Bedeutung von `-517`

Die Meldung:

`imx-es8328 sound: Unable to register: -517`

entspricht `EPROBE_DEFER`.

Sie liegt zeitlich zwischen der Audio-Regulator-Abschaltung und dem
späteren IT6251-Fehler.

Block 3.14A liefert jedoch keinen Beleg dafür, dass diese Meldung die
I2C3-Störung verursacht.

Sie bleibt deshalb ein dokumentierter Zwischenzustand und kein
kausaler Root-Cause-Beleg.

## 12. Zustandstabelle

| Phase | `es8328-power` | ES8328-/Audio-Domain | Codec-seitige I2C-Pull-ups | log-sichtbarer I2C3-Zustand |
| --- | --- | --- | --- | --- |
| vor 33.761742 s | aktiviert | versorgt | aus `P3.3V_DELAYED` versorgbar | keine hier nachgewiesene `93/80`-Signatur |
| bei 33.761742 s | Late-Cleanup führt Disable aus | Übergang zur abgeschalteten Domain | können weiterhin aus `P3.3V_DELAYED` versorgt sein | kein unmittelbar gleichzeitig protokollierter I2C3-Fehler |
| nach 33.761742 s | abgeschaltet, sofern nicht später erneut angefordert | dokumentierter Power-Domain-Grenzzustand | können weiterhin versorgt sein | zunächst kein weiterer Transfer im vorhandenen Log nachweisbar |
| 44.471609 s | Baseline-Zustand weiterhin Teil der rekonstruierten Fehlerkette | Grenzzustand ist für die Kausalkette relevant | externe Pull-up-Domain getrennt von Codec-Power | IT6251 `product ID attempt 1/5` |
| 44.478167 s | — | — | — | erste log-sichtbare problematische Transaktion: `arbitration lost`, `I2SR=0x93` |
| 44.478235 s | — | — | — | `START failure A=81/80 B=93/80 C=83/80 ret=-11` |

## 13. Direkt nachgewiesen

Direkt nachgewiesen beziehungsweise unmittelbar aus den gesicherten
Quellen ableitbar ist:

1. Der untersuchte Kernel ist Linux 6.18.49.

2. Der exakte Linux-6.18.49-Regulator-Core führt einen verzögerten
   Late-Cleanup unbenutzter Regulatoren aus.

3. `regulator_late_cleanup()` gibt unmittelbar vor dem Disable
   `disabling` aus.

4. Die Baseline-Meldung `es8328-power: disabling` entspricht diesem
   Regulator-Core-Pfad.

5. `regulator-always-on` verhindert diesen Cleanup-Pfad.

6. Im Baseline-Boot erfolgt die Abschaltmeldung bei `33.761742 s`.

7. Die erste log-sichtbare problematische I2C3-Transaktion erfolgt bei
   `44.478167 s`.

8. Der ES8328 ist über die Audio-Power-Domain versorgt, während seine
   Codec-seitigen I2C-Pull-ups aus `P3.3V_DELAYED` versorgt werden
   können.

9. H3-1R mit `regulator-always-on` bestand fünf von fünf
   vorregistrierten echten POR-Kaltstarts ohne die bekannte
   `93/80`-/Arbitration-Loss-Signatur.

## 14. Stark gestützt

Stark gestützt ist:

**Das automatische Abschalten der ES8328-/Audio-Power-Domain bringt die
Novena in einen Power-Domain-Grenzzustand, der kausal an der
untersuchten I2C3-Kaltstartstörung beteiligt ist.**

Diese Aussage stützt sich gemeinsam auf:

- den exakten Linux-6.18.49-Regulatorpfad,
- die Novena-Schaltung,
- das lokale Supply-Modell,
- die Baseline-Ereigniskette,
- den historischen Novena-Fix,
- und die unabhängige H3-1R-POR-Serie.

## 15. Elektrisch ungeklärt

Nicht bewiesen bleibt, welcher mikroskopische elektrische Mechanismus
innerhalb des Power-Domain-Grenzzustands die konkrete I2C3-Signatur
erzeugt.

Weiterhin offen sind insbesondere:

- ein interner ES8328-Clamp-Pfad,
- ein konkreter Backfeed-Pfad,
- eine mögliche Teilversorgung,
- resultierende Spannungen und Ströme,
- ein bestimmter Abschalttransient,
- der genaue zeitliche Eintritt eines elektrischen Fehlzustands nach
  der Regulator-Abschaltung.

Die `10.716425 s` zwischen Abschaltmeldung und erster log-sichtbarer
problematischer I2C3-Transaktion dürfen ausdrücklich nicht als
elektrische Fehlerlatenz interpretiert werden.

## 16. Konsequenz

Block 3.14A ändert die bereits getroffene Board-Entscheidung nicht.

Für Novena bleibt:

**`es8328-power` dauerhaft mit `regulator-always-on` eingeschaltet.**

Es besteht auf Basis von Block 3.14A kein Anlass für:

- Patch 0007,
- eine Wiederholung der fünf H3-1R-POR-Tests,
- eine erneute STMPE811-Isolation,
- eine Wiederholung der Single-Master-Isolation,
- zusätzliche IT6251-Delay-/Retry-Experimente als Root-Cause-Test.

## 17. Abschluss

Block 3.14A schließt die zeitliche und logische
Power-/I2C3-Zustandsrekonstruktion ab.

Die Ereigniskette kann auf Software- und Board-Ebene konsistent
rekonstruiert werden:

`Linux-6.18.49-Late-Cleanup`

`-> es8328-power: disabling`

`-> ES8328-/Audio-Power-Domain aus`

`-> I2C-Pull-up-Domain bleibt getrennt versorgbar`

`-> späterer IT6251-I2C3-Zugriff`

`-> erste log-sichtbare problematische Transaktion`

`-> arbitration lost / 93/80`

Die kausale Beteiligung der ES8328-/Audio-Power-Domain ist stark
gestützt.

Der genaue elektrische Mechanismus bleibt ohne zusätzliche
Herstellerinformation oder Hardwaremessung unbestimmt.

Damit ist Block 3.14A inhaltlich abgeschlossen.
