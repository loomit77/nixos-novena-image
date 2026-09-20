# Historische Kosagi-Hardwarequellen zu ES8328/I2C3

## Zweck

Dieses Verzeichnis archiviert historische Kosagi-Primärquellen zur elektrischen Wechselwirkung zwischen der abschaltbaren Audio-Versorgung der Novena und dem I2C-Bus.

Die Quellen wurden im Rahmen von Block 3.15W der I2C3/Display-Root-Cause-Untersuchung am 20. September 2026 erneut direkt von der Kosagi-Webseite abgerufen und anschließend unverändert archiviert.

Die archivierten HTML-Dateien sind Primärquellen. Die Aussagen in dieser README sind eine Einordnung im Rahmen des aktuellen Projekts und ersetzen nicht den Originaltext.

## Quelle 1: Novena EVT to DVT changes

- Titel: `Novena EVT to DVT changes`
- Historische Kosagi-Wiki-Seite: `Novena_EVT_to_DVT_changes`
- Abrufdatum: 2026-09-20
- Datei: `Novena_EVT_to_DVT_changes.html`
- HTTP-Header: `Novena_EVT_to_DVT_changes.http-headers.txt`
- SHA-256 HTML: `98626b8de199d68449c86225994802ed302ea03f7335153199bb12e5954bcb42`

### Relevanter Inhalt

Unter ECO8 (`Audio chip sucks (power)`) dokumentiert Kosagi, dass der Audiochip beim Abschalten Leistung über den I2C-Bus zurückleckt. Der Pulldown müsse verstärkt werden, um den Chip vollständig zurückzusetzen und gegen die I2C-Pull-ups anzukämpfen.

Die zugehörige ECO-Tabelle dokumentiert:

- EVT: R21A = 100 Ohm
- DVT: R21A = 20 Ohm
- auf EVT1A wurde 10 Ohm eingesetzt;
- 20 Ohm wurde als voraussichtlich ausreichend für DVT angegeben.

ECO20 verweist erneut auf die Erfahrung mit dem Audio-Codec und beschreibt einen vorhandenen 330-Ohm-Widerstand bei einer anderen Abschaltfunktion als wahrscheinlich zu schwach gegen erhebliche Leckströme.

## Quelle 2: Novena Issue Log

- Titel: `Novena Issue Log`
- Historische Kosagi-Wiki-Seite: `Novena_Issue_Log`
- Abrufdatum: 2026-09-20
- Datei: `Novena_Issue_Log.html`
- HTTP-Header: `Novena_Issue_Log.http-headers.txt`
- SHA-256 HTML: `02a2314f7b3548d625b75719815e9d05c5f04012cb5173702abb8a253920d6d4`

### Relevanter Inhalt

Der Issue Log dokumentiert für den Audio-Power-off-Pulldown einen Ausgangswert von R21A = 100 Ohm. Dabei wurden ungefähr 10 mA Strom über Leckpfade beobachtet, was zu ungefähr 1 V auf der eigentlich abgeschalteten Versorgung führte.

Als unmittelbare Hardwareänderung wurde R21A auf 10 Ohm reduziert. Der Eintrag vermerkt, dass diese Änderung zu diesem Zeitpunkt nur auf einem Board durchgeführt worden war.

Der Issue Log verweist später selbst auf `Novena EVT to DVT changes` als Zusammenfassung. Damit gehören die beiden archivierten Seiten unmittelbar zur selben historischen Entwicklungskette.

## Einordnung der R21A-Werte

Die Angaben 100 Ohm, 10 Ohm und 20 Ohm sind kein belegter Widerspruch, sondern dokumentieren unterschiedliche Entwicklungszustände:

1. 100 Ohm war der problematische Ausgangszustand.
2. 10 Ohm wurde als unmittelbare Änderung auf mindestens einem EVT-Board eingesetzt.
3. Die EVT-zu-DVT-ECO-Dokumentation spezifiziert anschließend 20 Ohm für DVT und erklärt, dass dieser Wert als ausreichend erwartet wurde.
4. Die im Projekt untersuchten PVT2-Boardquellen zeigen R21A ebenfalls mit 20 Ohm.

Daraus darf nicht abgeleitet werden, dass die historischen Messwerte von ungefähr 10 mA und ungefähr 1 V direkt auf der heutigen PVT2-Hardware erneut gemessen wurden. Diese Werte gehören zum dokumentierten historischen 100-Ohm-Zustand.

## Bedeutung für die aktuelle Root-Cause-Untersuchung

Die Quellen belegen historisch direkt, dass die Novena-Audio-Power-Domain beim Abschalten eine relevante elektrische Wechselwirkung mit I2C hatte und dass Leckstrom über den I2C-Pfad als Ursache erkannt wurde.

Sie bestimmen nicht den exakten internen Strompfad innerhalb des ES8328 und beweisen für sich allein nicht, dass jeder später beobachtete I2C3-Fehler denselben analogen Detailmechanismus besitzt.

Die Verbindung zum aktuellen Kaltstartfehler wird deshalb separat mit den späteren Softwarequellen, dem Commit e48619edadbde342d79655e73654f0b21fc5e20b, den PVT2-Schaltplanquellen und den eigenen H3-1R-/3.14B-/3.14C-Ergebnissen bewertet.
