# H3-1R – POR-Cold-Boot-Versuchsserie mit dauerhaft aktivem ES8328-Power-Regulator

Dieses Verzeichnis enthält die vollständige Evidence der kontrollierten
H3-1R-Versuchsserie vom 15. September 2026.

## Fragestellung

H3-1R untersuchte, ob das automatische Abschalten des Novena-Regulators
`es8328-power` während des Cold-Boots zur bekannten I2C3-/IT6251-
Fehlerklasse beiträgt.

Im gesicherten Baseline-Cold-FAIL wurde `es8328-power` während des
Bootvorgangs abgeschaltet. Beim späteren ersten relevanten IT6251-START
trat die instrumentierte Fehlersignatur

`A=81/80 -> M0=93/80`

mit `arbitration lost` auf.

## Kontrollierte Intervention

Für H3-1R wurde am bestehenden Device-Tree-Knoten `reg_audio_codec`
ausschließlich

`regulator-always-on`

ergänzt.

Der vor dem Hardwaretest durchgeführte semantische DTB-Vergleich bestätigte,
dass dies die einzige funktionale Device-Tree-Änderung gegenüber der
Baseline war.

Es wurde kein zusätzlicher Kernel-Patch `0007` eingeführt.

## Vorregistriertes Testprotokoll

Die Versuchsserie bestand aus fünf echten POR-Cold-Boots.

Für jeden Lauf galt:

* Novena vollständig ausschalten,
* mindestens 30 Sekunden ohne Versorgung warten,
* externe H3-1R-Test-SD verwenden,
* kein Reboot anstelle eines POR,
* kein LDB-Unbind/Rebind,
* keine manuelle I2C-Aktion vor der Beweissicherung,
* erster IT6251-START nach POR als primäre Messung,
* vollständige Evidence vor dem nächsten Lauf sichern.

Die fünf Läufe befinden sich in:

* `boot-01/`
* `boot-02/`
* `boot-03/`
* `boot-04/`
* `boot-05/`

## Ergebnis

Alle fünf vorregistrierten POR-Cold-Boots waren erfolgreich.

Für jeden Lauf gilt:

* `POR_COLD_BOOT=YES`,
* `DISPLAY=ON`,
* `regulator-always-on` im Live-Device-Tree vorhanden,
* keine protokollierte Abschaltung von `es8328-power`,
* erster IT6251-START mit `M0=81/a0`,
* keine `M0=93/80`-Fehlersignatur im vollständigen Kernel-Log,
* kein `arbitration lost` im vollständigen Kernel-Log.

Zusammenfassung:

| Lauf | POR | Display | erster M0 | ES8328-Abschaltung | 93/80 | Arbitration lost |
| --- | --- | --- | --- | --- | --- | --- |
| 01 | ja | an | `81/a0` | nein | nein | nein |
| 02 | ja | an | `81/a0` | nein | nein | nein |
| 03 | ja | an | `81/a0` | nein | nein | nein |
| 04 | ja | an | `81/a0` | nein | nein | nein |
| 05 | ja | an | `81/a0` | nein | nein | nein |

## Inhalt eines Boot-Verzeichnisses

Jeder der fünf Läufe enthält 15 Dateien einschließlich `SHA256SUMS`.

Gesichert wurden unter anderem:

* vollständiges Kernel-Log,
* IT6251-START-Auszug,
* kritisches Boot-Zeitfenster,
* ES8328-Power-Auszug,
* Regulatorzustände,
* GPIO-Zustand,
* I2C3-Geräte,
* Live-Device-Tree-Zustand,
* DRM-/Displaystatus,
* Testkennung.

Die konkreten Dateinamen und ihre Integritätswerte sind jeweils in
`SHA256SUMS` des betreffenden Boot-Verzeichnisses dokumentiert.

## Integritätsprüfung

Alle fünf Repository-Kopien wurden nach dem Kopieren einzeln im jeweiligen
tatsächlichen Zielverzeichnis mit

`sha256sum -c SHA256SUMS`

verifiziert.

Das Ergebnis war für alle fünf Läufe:

`SHA256_EXIT=0`

Zusätzlich wurden die ursprünglichen Evidence-Verzeichnisse und die
Repository-Kopien für jeden Lauf byteweise verglichen.

Für `boot-01` bis `boot-05` ergab sich jeweils `IDENTISCH`.

Die unabhängigen Ausgangskopien bleiben zusätzlich unter

`~/novena-backups/h3-1r-trials-2026-09-15/`

erhalten.

## Bewertung

Die Versuchsserie liefert starke experimentelle Evidenz dafür, dass das
automatische Abschalten des ES8328-Power-Domains einen kausalen Beitrag
zur untersuchten Novena-I2C3-Cold-Boot-Fehlerklasse leistet.

Diese Bewertung wird zusätzlich durch einen unabhängigen historischen
Novena-Befund gestützt: Commit

`e48619edadbde342d79655e73654f0b21fc5e20b`

dokumentiert ebenfalls eine Beeinträchtigung von I2C3 durch das Abschalten
von `es8328-power` und verwendet `regulator-always-on` als Lösung.

Auf Grundlage der aktuellen Versuchsserie und dieses historischen Befunds
wurde `regulator-always-on` anschließend als dauerhafte boardspezifische
Novena-Konfiguration übernommen.

Die formale Entscheidung ist in `docs/DECISIONS.md` dokumentiert.

Der ausführliche Versuchs- und Entscheidungsverlauf befindet sich in
`docs/PROJECT-STATE.md`, insbesondere in Block 3.8 bis Block 3.11.

## Aussagegrenze

Die Evidence bestimmt nicht den exakten elektrischen Mechanismus der
ES8328-/I2C3-Wechselwirkung.

Mögliche Clamp-, Rückspeisungs-, Pull-up- oder andere transiente
elektrische Effekte bleiben ohne zusätzliche elektrische Messungen offen.

Auch fünf erfolgreiche POR-Cold-Boots stellen keine universelle Garantie
für sämtliche denkbaren Startbedingungen dar.

Die dauerhafte Board-Entscheidung ist deshalb von der weiterhin offenen
Frage nach dem vollständigen elektrischen Mechanismus zu unterscheiden.
