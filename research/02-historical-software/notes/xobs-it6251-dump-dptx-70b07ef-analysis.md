# xobs/it6251-dump-dptx – Analyse der Revision 70b07ef

## Status

Historische Quellenanalyse für die Untersuchung des Cold-POR-I2C3/
IT6251-Fehlers auf der Kosagi Novena.

Untersuchte Revision:

    70b07efbf2621cba7f791a8a44abc80d0e447012

Diese Revision wird vom historischen Repository:

    novena-next/nixos-novena

für das Paket:

    it6251-dump-dptx

verwendet.

Die Untersuchung dient der historischen Einordnung. Aus dieser Quelle
allein wird kein Kernel-Patch und insbesondere kein Patch 0007 abgeleitet.

## Herkunft aus novena-next/nixos-novena

Die Datei:

    it6251-dump-dptx/default.nix

im historischen Repository novena-next/nixos-novena verwendet:

    owner = "xobs";
    repo = "it6251-dump-dptx";
    rev = "70b07ef";

Die dort angegebene Beschreibung lautet sinngemäß, dass das Werkzeug die
EDID des über den IT6251 angeschlossenen LCD-Panels ausliest.

Die Kurzrevision:

    70b07ef

wurde im Originalrepository eindeutig aufgelöst zu:

    70b07efbf2621cba7f791a8a44abc80d0e447012

Zum Zeitpunkt der Untersuchung war diese Revision zugleich HEAD von
master.

## Repository-Historie

Das Repository enthält fünf Commits:

    70b07ef Merge pull request #2 from akil/hotfix/libi2c
    f8f8fbf fix: libi2c
    7820969 it6251-dump-dptx: Add binary
    d44d53b it6251-dump-dptx: Fix build on base system
    078f804 Initial commit

Der untersuchte Research-Clone war sauber.

## Reproduzierbares Quellenarchiv

Das vollständige Git-Repository einschließlich Historie wurde als
Git-Bundle archiviert:

    research/02-historical-software/sources/xobs-it6251-dump-dptx/xobs-it6251-dump-dptx.bundle

SHA-256:

    1c2e391127d62d3ae6951ad94774d8b092312759ae25c334de15b125d70971ec

`git bundle verify` bestätigt, dass das Bundle eine vollständige Historie
enthält.

Ein Testklon aus dem archivierten Bundle ergab als HEAD:

    70b07efbf2621cba7f791a8a44abc80d0e447012

Damit kann das untersuchte Repository unabhängig von der zukünftigen
Verfügbarkeit des ursprünglichen GitHub-Repositories rekonstruiert werden.

## Archivierte untersuchte C-Quelle

Die exakte Datei:

    it6251-dump-dptx.c

der Revision 70b07ef wurde separat archiviert als:

    research/02-historical-software/extracts/xobs-it6251-dump-dptx-70b07ef-it6251-dump-dptx.c

SHA-256:

    faa8697a47185ad214c460e0eef19c6d34909ab823ff3d473fa44a1900806e13

Ein Bytevergleich gegen das Git-Objekt der Revision ergab:

    SOURCE_COMPARE=IDENTICAL

## Archivierte Provenienz

Die Commit-, Tree-, Ref- und Historieninformationen wurden archiviert als:

    research/02-historical-software/extracts/xobs-it6251-dump-dptx-70b07ef-provenance.txt

SHA-256:

    99326868d7ac1b6947173f544c54d519bcff893428f279329eecaabc27e9cc26

## Verwendeter Linux-I2C-Bus und IT6251-Adresse

Die Quelle definiert:

    I2C_FILENAME "/dev/i2c-2"
    DP_I2C      0x5c

Damit greift das Programm auf Linux-I2C-Bus 2 und die 7-Bit-Adresse:

    0x5c

zu.

Für die Novena entspricht Linux i2c-2 dem bereits in unserer aktuellen
Untersuchung relevanten Hardware-I2C3-Controller.

Die Adresse 0x5c stimmt mit der späteren historischen U-Boot-Erkennung des
IT6251 überein.

## Low-Level-I2C-Zugriffe

### Schreibzugriff

`i2c_write_byte()` öffnet:

    /dev/i2c-2

mit:

    O_RDWR

und führt über:

    ioctl(..., I2C_RDWR, ...)

eine einzelne I2C-Message aus.

Die Nutzdaten bestehen aus:

    Registeradresse
    Registerwert

Nach der Transaktion wird der Dateideskriptor wieder geschlossen.

### Lesezugriff

`i2c_read_byte()` öffnet ebenfalls für jeden Aufruf erneut:

    /dev/i2c-2

Der eigentliche Zugriff erfolgt mit:

    I2C_RDWR

und zwei Messages:

1. Schreiben der Registeradresse.
2. Lesen eines Bytes mit I2C_M_RD.

Damit handelt es sich um eine kombinierte I2C-Transaktion.

Auch nach diesem Zugriff wird der Dateideskriptor wieder geschlossen.

## Keine spezielle Behandlung von I2C-Arbitration-Loss

Die Low-Level-Funktionen enthalten keine spezielle Behandlung von:

- Arbitration Loss;
- I2C-Bus-Recovery;
- SDA/SCL-GPIO-Recovery;
- Controller-Reset;
- Controller-Reinitialisierung;
- Retry nach fehlgeschlagener Transaktion.

Wenn `I2C_RDWR` fehlschlägt, geben die Funktionen lediglich:

    -1

zurück.

Das Programm implementiert daher selbst keine der historischen
U-Boot-Recovery-Maßnahmen, die wir zuvor in xobs/u-boot-novena gefunden
haben.

## Erster I2C-Zugriff bei normalem Programmstart

Bei einem Programmstart ohne Argumente führt `main()` unmittelbar aus:

    dptx_get_edid(edid, sizeof(edid));

Innerhalb von `dptx_get_edid()` beginnt die Schleife ohne vorherige
Power-, Reset- oder Delay-Sequenz.

Der erste Hardwarezugriff ist:

    i2c_write_byte(DP_I2C, 0x23, EDID_SEG_WRITE | 0x02);

Damit ist der erste relevante Zugriff bereits ein Schreibzugriff auf:

    IT6251 0x5c
    Register 0x23

Vor diesem Zugriff enthält der Programmpfad keine:

- sleep()-Verzögerung;
- usleep()-Verzögerung;
- IT6251-Reset-Sequenz;
- Display-Power-Sequenz;
- I2C-Bus-Recovery;
- explizite I2C-Controller-Initialisierung;
- Retry-Schleife für den ersten I2C-Zugriff.

## Bedeutung der vorhandenen usleep()-Aufrufe

Die Quelle enthält zwei funktional relevante Stellen mit:

    usleep(1000)

Keine davon liegt vor dem ersten IT6251-I2C-Zugriff.

### AUX-Wartefunktion

`dptx_auxwait()` liest wiederholt IT6251-Register:

    0x2b

und wartet jeweils:

    1000 µs

wenn das AUX-Busy-Bit weiterhin gesetzt ist.

Die Schleife läuft maximal 200-mal.

Diese Verzögerung findet erst statt, nachdem zuvor eine AUX-Operation über
IT6251-Register gestartet wurde.

Sie kann daher keine Voraussetzung für das Zustandekommen des ersten
I2C-STARTs zum IT6251 sein.

### Frequenzmessung

In `dptx_show_vid_info()` wird Register:

    0x12

gelesen und verändert.

Zwischen zwei Schreibzugriffen auf dieses Register liegt:

    usleep(1000)

Auch diese Verzögerung erfolgt erst nach zahlreichen bereits
durchgeführten IT6251-I2C-Transaktionen.

Sie ist für die Initialisierung des I2C-Busses oder den ersten Zugriff
nicht relevant.

## AUX- und EDID-Zugriffe

Das Werkzeug benutzt die IT6251-Registerschnittstelle, um den
DisplayPort-AUX-Kanal anzusteuern.

Für das EDID-Lesen werden unter anderem die Register:

    0x23
    0x24
    0x25
    0x26
    0x2b
    0x2c

verwendet.

Die Quelle startet AUX-Operationen und wartet anschließend über
`dptx_auxwait()` auf deren Abschluss.

Das Werkzeug geht damit davon aus, dass der IT6251 zum Zeitpunkt des
Programmstarts bereits ausreichend initialisiert und über I2C erreichbar
ist.

## Aussage für die aktuelle Cold-POR-Untersuchung

Die Quelle zeigt eindeutig:

    Das historische Userspace-Werkzeug implementiert vor seinem ersten
    IT6251-I2C-Zugriff keine eigene Power-, Reset-, Delay-, Retry- oder
    Bus-Recovery-Sequenz.

Das ist ein relevanter negativer Befund.

Er beweist jedoch NICHT:

    dass der IT6251 unmittelbar nach einem echten Power-On Reset ohne
    vorherige Initialisierung zuverlässig über I2C erreichbar ist.

Der entscheidende Grund ist der Ausführungskontext.

`it6251-dump-dptx` ist ein Linux-Userspace-Diagnoseprogramm. Zum Zeitpunkt
seines Starts hat das System bereits:

- Boot-ROM/SPL durchlaufen;
- U-Boot durchlaufen;
- den Linux-Kernel gestartet;
- die relevanten Kernel-Treiber initialisiert;
- möglicherweise bereits frühere Zugriffe auf I2C3 und den IT6251
  durchgeführt;
- Display-Power- und Reset-Zustände möglicherweise bereits verändert.

Deshalb darf ein erfolgreicher Zugriff dieses Programms nicht mit dem
ersten I2C3-START nach einem echten Cold-POR gleichgesetzt werden.

## Vergleich mit dem aktuellen Linux-6.18.49-Fehler

Unser dokumentierter Cold-POR-Fehler mit Patch 0006 zeigt:

    vor MSTA:
        A = 81/80

    erster post-MSTA-Snapshot:
        M0 = 93/80

Damit ist beim ersten beobachtbaren Zustand nach dem Versuch, Master zu
werden:

- IAL bereits gesetzt;
- MSTA bereits wieder gelöscht.

Der Fehler entsteht somit wesentlich früher als die AUX-Operationen, für
die `it6251-dump-dptx` seine Wartezeiten verwendet.

Die `usleep(1000)`-Aufrufe dieses Userspace-Werkzeugs liefern daher keine
direkte Erklärung für den aktuellen START-/Arbitration-Loss-Fehler.

## Beziehung zum historischen U-Boot

Die zuvor untersuchten xobs/u-boot-novena-Stände sind in diesem Punkt
interessanter als dieses Userspace-Werkzeug.

Historisches U-Boot enthält vor I2C-Transfers eine physische
Bus-Idle-/Recovery-Behandlung über SDA/SCL-GPIOs und besitzt
Retry-Verhalten bei Arbitration Loss.

Spätere untersuchte U-Boot-Stände führen außerdem einen echten
IT6251-Erkennungstransfer auf:

    Bus 2
    Adresse 0x5c

durch.

`it6251-dump-dptx` enthält diese Bus-Recovery dagegen nicht.

Die beiden Softwarepfade dürfen daher nicht gleichgesetzt werden.

## Schlussfolgerung

Revision 70b07ef von xobs/it6251-dump-dptx ist eine nützliche historische
Primärquelle für:

- die verwendete IT6251-I2C-Adresse 0x5c;
- die Verwendung von Linux /dev/i2c-2;
- die IT6251-AUX-/EDID-Registerzugriffe;
- das Verhalten eines historischen Linux-Userspace-Diagnosewerkzeugs.

Sie enthält jedoch keinen eigenen Mechanismus, der unseren
Cold-POR-I2C3-Arbitration-Loss erklären würde.

Insbesondere existiert vor dem ersten IT6251-I2C-Zugriff keine
Power-, Reset-, Delay-, Retry- oder Bus-Recovery-Sequenz.

Der nächste sinnvollere historische Primärquellenzweig ist deshalb der
von novena-next/nixos-novena tatsächlich verwendete Kernel:

    novena-next/linux
    nvn_v5.7-rc2

Dort sollen insbesondere untersucht werden:

- der Novena Device Tree;
- der i.MX-I2C-Treiber;
- IT6251-/Display-Unterstützung;
- historische Novena-spezifische Patches;
- Unterschiede zur aktuellen Linux-6.18.49-Basis.

Aus der vorliegenden Quelle ergibt sich weiterhin keine Rechtfertigung für
Patch 0007.
