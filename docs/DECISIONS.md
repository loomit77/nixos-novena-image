# Decisions

> Aktuelle Entwicklungspriorität: **NOVENA-ENTSCHEIDUNG-2026-10-18-NIXOS-FIRST**, siehe Nachtrag am Dateiende. Frühere Priorisierungen bleiben historische Stände.

> Aktueller Workflowstand: siehe Nachtrag vom 2026-10-17 am Dateiende (Zwei-Skripte-Workflow, E2E PASS; Konkurrenztest offen). Fruehere Statusangaben sind datierte historische Staende.

## 2026-09-11 – Eigenes Git-Repository

Das universelle NixOS-SD-Image wird als eigenständiges Projekt
versioniert.

Es wird nicht mit dem allgemeinen Novena-Systemrepository vermischt.

Begründung:

Das SD-Image soll als reproduzierbares Softwareprodukt unabhängig von
der individuellen Installation des aktuell verwendeten Novena
betrachtet werden.

## 2026-09-11 – Patches statt kompletter Kernelquellen

Komplette ausgepackte Kernelquellen werden nicht in Git gespeichert.

Dauerhaft relevante Änderungen müssen als nachvollziehbare Patchdateien
gesichert werden.

Aktuelle Patchserie:

- `0001-drm-bridge-it6251.patch`
- `0002-drm-panel-add-innolux-n133hse-ea1.patch`
- `0003-i2c-imx-debug-arbitration-lost.patch`
- `0004-i2c-imx-debug-start-state.patch`
- `0005-i2c-imx-debug-start-error.patch`
- `0006-i2c-imx-debug-start-transition.patch`

Die Patches `0003` bis `0006` dienen der aktuellen I2C-Root-Cause-
Diagnose. Sie sind Diagnoseinstrumentierung und nicht als dauerhafter
Produktionsbestandteil des finalen Images beschlossen.

## 2026-09-11 – Referenz-Bootdateien werden versioniert

Die Dateien unter `boot/reference/` werden im Repository behalten.

Begründung:

Sie dokumentieren einen bekannten Novena-Bootzustand und sind klein
genug, um sinnvoll in Git gesichert zu werden.

Sie sind ausdrücklich nicht als aktuelle Build-Ergebnisse zu verstehen.

## 2026-09-11 – Nix result-Symlinks werden nicht versioniert

Folgende Einträge bleiben außerhalb von Git:

- `result`
- `result-kernel`
- `result-dtb`
- `result-kconfig`

Sie verweisen lediglich auf lokale Nix-Store-Ergebnisse.

## 2026-09-11 – Bootlogs vor Änderungen auswerten

Bei zukünftigen realen Boottests wird zuerst der vollständige serielle
Bootlog gesichert.

Erst nach dessen Auswertung werden Kernel, Device Tree, Bootskript oder
Image verändert.

## 2026-09-15 – ES8328-Versorgung bleibt dauerhaft eingeschaltet

Für Novena wird `es8328-power` dauerhaft mit
`regulator-always-on` konfiguriert.

Diese Einstellung wird als boardspezifische Hardwarekonfiguration
übernommen und nicht mehr nur als H3-1R-Testintervention geführt.

Begründung:

Im gesicherten Baseline-Cold-FAIL schaltet Linux `es8328-power`
während des Bootvorgangs ab. Später folgt die instrumentierte
I2C3-Fehlersignatur `A=81/80 -> M0=93/80` mit `arbitration lost`.

Für H3-1R wurde als einzige semantische Device-Tree-Änderung
`regulator-always-on` am bestehenden `reg_audio_codec` ergänzt.

Unter dieser kontrollierten Intervention bestanden fünf von fünf
vorregistrierten echten POR-Cold-Boots. In keinem der fünf vollständigen
Kernel-Logs traten die Audio-Regulator-Abschaltung, die bekannte
`93/80`-Signatur oder `arbitration lost` auf.

Zusätzlich dokumentiert die historische Novena-Änderung
`e48619edadbde342d79655e73654f0b21fc5e20b` unabhängig davon, dass
das Abschalten von `es8328-power` den I2C3-Bus auf realer
Novena-Hardware beeinträchtigt, und verwendet ebenfalls
`regulator-always-on`.

Die Kombination aus kontrollierter aktueller Versuchsserie und
unabhängigem historischem Hardwarebefund rechtfertigt die dauerhafte
Übernahme als Novena-Board-Konfiguration.

Nicht entschieden ist damit der genaue elektrische Mechanismus der
ES8328-/I2C3-Wechselwirkung. Insbesondere werden mögliche Clamp-,
Rückspeisungs-, Pull-up- oder andere transiente elektrische Effekte
durch diese Entscheidung nicht als bewiesen betrachtet.

Für diese Board-Konfiguration wird kein zusätzlicher Kernel-Patch
`0007` eingeführt. Die Lösung bleibt eine Device-Tree-Eigenschaft des
bestehenden Audio-Power-Regulators.


## 2026-09-20 – Funktionale ES8328/I2C3-Root-Cause-Untersuchung abgeschlossen

Die am 15. September 2026 getroffene Entscheidung, `es8328-power`
dauerhaft mit `regulator-always-on` zu konfigurieren, bleibt
unverändert bestehen.

Block 3.15W erweitert die Begründung dieser bestehenden Entscheidung
durch wiederentdeckte historische Kosagi-Primärquellen.

Der historische `Novena Issue Log` dokumentiert im EVT-Zustand mit
R21A = 100 Ohm ungefähr 10 mA Leakage und ungefähr 1 V Restspannung
auf der Audio-Versorgung.

Die historische EVT-zu-DVT-Dokumentation ordnet die elektrische
Wechselwirkung ausdrücklich einer Rückspeisung über I2C zu und nennt
die I2C-Pull-ups als Gegenlast des Power-off-Pulldowns.

R21A wurde im Entwicklungsverlauf von 100 Ohm über einen
10-Ohm-EVT-Versuch auf 20 Ohm für DVT geändert. PVT2 verwendet
ebenfalls 20 Ohm.

Zusammen mit dem historischen Novena-next-Linux-Commit
`e48619edadbde342d79655e73654f0b21fc5e20b`, der aktuellen
H3-1R-Serie mit 5/5 erfolgreichen POR-Cold-Boots und der
Controlleranalyse aus Block 3.14B/3.14C ist die elektrische
Wechselwirkung der abgeschalteten ES8328-Audio-Power-Domain mit I2C3
als funktionale Root Cause der untersuchten Kaltstartstörung
ausreichend belegt.

Nicht als bestimmt gelten weiterhin der exakte interne Leckstrompfad
im ES8328 und der genaue analoge Spannungs- und Stromverlauf während
des Fehlers auf dem aktuellen PVT2-Board.

Diese Detailfragen sind für die funktionale Board-Konfiguration nicht
erforderlich.

Daher werden für den Abschluss dieser Root-Cause-Untersuchung keine
weiteren elektrischen Messungen, kein Patch 0007 und keine bloßen
Wiederholungen bereits abgeschlossener Boot- oder `single-master`-
Tests verlangt.

Eine spätere analoge Untersuchung bleibt als optionale
Hardwarecharakterisierung möglich, ist aber kein offener Blocker.

Vollständige Synthese:

`research/03-root-cause-synthesis/block-3.15w-es8328-i2c3-historische-root-cause.md`

## 2026-09-21 – V1 muss ohne vorhandenen internen Bootloader starten

Für Version 1 wird „universell“ so definiert, dass das externe
NixOS-SD-Image auf einer frisch beschriebenen geeigneten SD-Karte
unabhängig vom bereits installierten Betriebssystem und dessen Boot-
oder Root-Dateien starten können muss.

Die während Phase 4 erfolgreich durchgeführten drei POR-Cold-Boots
erfüllen diese Anforderung noch nicht vollständig.

Bei allen drei Tests wurde seriell folgende tatsächliche Bootgrenze
beobachtet:

`ROM -> interne microSD -> SPL/U-Boot 2015 -> externe SD boot.scr -> externer Kernel/Initrd/DTB -> externes Root-Dateisystem`

Die Tests bestätigen damit die Funktionsfähigkeit des externen
NixOS-Systems innerhalb dieser Bootgrenze. Sie bestätigen jedoch nicht
den bisher im externen Image eingebetteten U-Boot 2020.07 als
eigenständigen externen Bootpfad.

Die Abhängigkeit von einem bereits vorhandenen internen U-Boot wird
deshalb nicht als endgültiger V1-Zustand akzeptiert.


## 2026-09-21 – P_EXT und externe SD bilden den eigenständigen V1-Bootpfad

Für den eigenständigen externen V1-Bootpfad wird der dafür vorgesehene
Novena-Boot-Select `P_EXT` verwendet.

Der Zielpfad lautet:

`P_EXT -> i.MX6 ROM -> externe SD/USDHC2 -> SPL -> U-Boot proper -> boot.scr -> Kernel/Initrd/DTB -> externes Root-Dateisystem`

Für diesen V1-Pfad muss der SPL deshalb die externe SD-Schnittstelle
USDHC2 verwenden.

U-Boot proper soll anschließend die externe SD als erstes MMC-Bootziel
verwenden. In der bestehenden Novena-Gerätenummerierung ist dies
`mmc1`.

Ein automatischer Rückfall auf `mmc0` wird für diesen MMC-Bootpfad
nicht hinzugefügt, weil dies die Abhängigkeit von der internen
microSD wieder in den definierten V1-Pfad einführen würde.

Generische USB-, SATA-, PXE- und DHCP-Bootziele dürfen als nachgeordnete
U-Boot-Fallbacks bestehen bleiben. Sie stellen keine versteckte
Abhängigkeit von der internen microSD dar.


## 2026-09-21 – U-Boot v2026.07 mit drei Novena-Anpassungen ist die neue Bootloader-Basis

Der historische, im bisherigen Image eingebettete U-Boot 2020.07 wird
für den neuen eigenständigen externen Bootpfad nicht als aktive
Bootloader-Basis weitergeführt.

Als neue Basis wird der stabile Upstream-Stand U-Boot `v2026.07`
verwendet.

Exakter Upstream-Commit:

`ece349ade2973e220f524ce59e59711cc919263f`

Darauf werden genau drei funktionale Novena-Anpassungen angewendet:

1. U1 stellt den SPL für den externen V1-Pfad von USDHC3 auf USDHC2 um.
2. U2 stellt das erste MMC-Bootziel von `mmc0` auf `mmc1` um und entfernt
   die historischen lokalen MMC0-Linux- und SD-Update-Vorgaben aus der
   Novena-Default-Umgebung.
3. U3 entfernt das persistente MMC-Environment. Der Bootloader verwendet
   stattdessen das nichtpersistente Default-Environment.

Die ursprünglichen Implementierungscommits sind:

* U1: `69c43c2cdae6196bdd3d77b4f2c8de8a908193ae`
* U2: `a742f5f1b60d28eea887188a4c39b140eebe0968`
* U3: `f8baca04e22de46111af34159afd54830b3e328a`

Die drei Änderungen werden im Hauptprojekt als Patchserie unter
`boot/u-boot/` versioniert.

Die historischen Dateien unter `boot/reference/` bleiben weiterhin im
Repository erhalten. Sie dienen der Provenienz und dem Vergleich, sind
aber nach abgeschlossener Image-Integration nicht mehr als aktive
Quelle des neuen Bootloader-Pfads vorgesehen.

Der qualifizierte U-Boot-Komponentencheckpoint im Hauptprojekt ist:

`8cd6d449284f20c29b549e837caad61762abf73b`

Zum Zeitpunkt dieser Entscheidung ist dieser neue U-Boot noch nicht in
`image/novena-image.nix` aktiviert und noch nicht per `P_EXT` auf realer
Novena-Hardware getestet.


## 2026-09-21 – Normaler pkgs.buildUBoot-Build ist der produktive U-Boot-Buildvertrag

Der neue U-Boot wird im Projekt mit dem gepinnten Nixpkgs-
`pkgs.buildUBoot`-Baustein gebaut.

Quelle, Upstream-Commit, Patchserie, Source-Hash, Versionsidentität und
`SOURCE_DATE_EPOCH` werden deterministisch festgelegt.

Die feste Versionsidentität lautet:

`2026.07-00003-gf8baca04e22d`

Der verwendete `SOURCE_DATE_EPOCH` ist:

`1789984363`

Der normale `pkgs.buildUBoot`-Baustein setzt für U-Boot
`hardeningDisable = [ "all" ]`. Dieser normale Nix-Buildvertrag wird
für den produktiven Projektpfad beibehalten.

Es wird kein zusätzliches Wrapper-Hardening erzwungen, nur um frühere
manuell erzeugte Binärdateien byte-identisch nachzubilden.

Begründung:

Die zunächst beobachteten Größen- und Hash-Unterschiede zwischen den
manuellen Builds und dem normalen `pkgs.buildUBoot`-Build wurden in
einem kontrollierten A/B-Versuch vollständig auf den Hardening-Zustand
des Nix-GCC-Wrappers zurückgeführt.

Mit wieder aktiviertem Wrapper-Hardening reproduzierte der kontrollierte
Nix-Build die früheren manuellen Artefakte byte-identisch. Damit war
nachgewiesen, dass die Abweichung nicht durch fehlende Quellobjekte,
eine andere U-Boot-Konfiguration, andere Linkbefehle oder eine
unvollständige Cross-Kompilierung verursacht wurde.

Der normale produktive `pkgs.buildUBoot`-Build wurde anschließend mit
`nix-store --realise --check` erneut gebaut und reproduzierte seine
Artefakte byte-identisch.

Die qualifizierten produktiven Artefakte dieses Buildvertrags sind:

* `SPL`: 52224 Byte,
  SHA-256 `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0`
* `u-boot-dtb.img`: 602120 Byte,
  SHA-256 `c5a2d0e7f2b9ebeae6387eee98630a43944f2cd2b52fc8a5a98a2e82ba55f9f7`

Diese Entscheidung betrifft den Buildvertrag des U-Boot-Bausteins.
Die reale Bootfähigkeit über `P_EXT` wird davon getrennt erst durch den
späteren Hardwaretest nach vollständiger Image-Integration bestätigt.


## 2026-09-22 – Bash-Pipegröße im Cross-Build wird deterministisch festgelegt

Ein vollständiger Image-Neubau in einem getrennten Nix-Store zeigte,
dass das ARM-Bash-Binary trotz identischer Derivation unterschiedliche
Bytes enthalten konnte.

Die Abweichung wurde bis auf zwei Instruktionen in
`do_redirection_internal.constprop.0` eingegrenzt.

Dabei wurden unterschiedliche eingebaute Werte beobachtet:

* `65536`
* `8192`

Bash erzeugt `builtins/pipesize.h` normalerweise über das auf dem
Build-Host kompilierte Hilfsprogramm `psize.aux` und `psize.sh`.

Die Bash-Builddatei weist selbst darauf hin, dass dieses Verfahren bei
einem Cross-Build technisch falsch ist, weil damit die Pipegröße des
Build-Hosts statt des Zielsystems ermittelt wird.

Auf der foobox wurde experimentell bestätigt, dass sich die
beobachteten Werte aus dem dynamischen Linux-Pipe-Zustand des
Build-Hosts ergeben können. Beim Erreichen des Pipe-Soft-Limits sank
die Kapazität neu erzeugter Pipes reproduzierbar von `65536` auf
`8192` Byte.

Damit war der Mechanismus der Image-Nichtreproduzierbarkeit bestimmt.

Für den Novena-Cross-Build wird deshalb projektlokal:

`NIX_CROSS_PIPESIZE=4096`

gesetzt.

Der Wert `4096` wird als deterministischer konservativer Zielwert auf
Basis von Linux `PIPE_BUF` verwendet. Er wird nicht als feste
tatsächliche Pipe-Kapazität des laufenden Zielsystems interpretiert.

Die Änderung wird ausschließlich bei Cross-Builds angewendet.
Native Bash-Builds bleiben unverändert.

Sowohl der interaktive als auch der nichtinteraktive ARM-Bash-Build
werden erfasst.

Die tatsächlichen ARM-Binaries wurden nach dem Fix disassembliert.
Beide enthalten im relevanten Code den Wert `4096`, während die
hostseitige Erzeugung über `psize.aux` und `psize.sh` im qualifizierten
Cross-Build nicht mehr stattfindet.

Der Fix ist im Commit

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

versioniert.


## 2026-09-22 – Image-Reproduzierbarkeit wird durch unabhängigen Bytevergleich qualifiziert

Ein identischer Derivation- oder Nix-Store-Pfad allein wird im Projekt
nicht als ausreichender Nachweis für Byte-Reproduzierbarkeit behandelt.

Diese Entscheidung folgt aus dem zuvor beobachteten Bash-Fall, bei dem
derselbe input-adressierte Buildpfad in getrennten Stores
unterschiedliche Binärinhalte erzeugte.

Für den aktuellen Image-Checkpoint wurde deshalb ein neuer isolierter
Nix-Store angelegt und das vollständige Image aus exakt dem
versionierten Commit

`d06de5e30f294144d0be66cc7903fedb1ed6fe0a`

neu gebaut.

Normal-Store- und Clean-Store-Ergebnis besitzen jeweils:

* Größe: `2581291008` Byte
* SHA-256:
  `8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc`

Der direkte Bytevergleich ergab:

`cmp = 0`

Damit gilt die Same-Host-/Separate-Store-Reproduzierbarkeit dieses
Image-Stands als nachgewiesen.

Dieser Nachweis wird ausdrücklich von einem späteren
Cross-Host-Reproduzierbarkeitstest getrennt. Ein solcher Test soll
später unabhängig auf dem L14 erfolgen.

## 2026-09-23 – P_EXT-Diagnose bleibt rein beobachtend

Der erste reale P_EXT-Test bestätigt den Bootpfad vom i.MX6-ROM über
die externe SD bis zum projektlokalen SPL v2026.07.

Der anschließende Übergang zu U-Boot proper v2026.07 ist noch nicht
nachgewiesen.

Der erste Diagnoseboot erreichte D0 unmittelbar vor
`spl_mmc_find_device()`, aber keinen der nachfolgenden Marker D1 bis
D15.

Aus diesem Befund wird bewusst keine funktionale Änderung an
MMC-Nummerierung, USDHC-Auswahl, IOMUX, Clock-Konfiguration,
Timeoutwerten oder Boot-Payload abgeleitet.

Stattdessen wird der bestehende Kontrollfluss mit einer zweiten,
ausschließlich seriellen Diagnoseinstrumentierung D0 bis D36 weiter
beobachtet.

Für deren Hardwaretest gilt eine Ein-Boot-Regel: Nach maximal einem
initialen P_EXT-Boot wird die Ausgabe ausgewertet, bevor ein weiterer
Boot zulässig ist.

Der letzte beobachtete Marker dient ausschließlich zur Lokalisierung
des nächsten Untersuchungskorridors und gilt nicht für sich allein als
Nachweis einer Root Cause.

## 2026-09-23 – D21-D22-Diagnose wird vor weiterem P_EXT-Test präregistriert

Der präregistrierte D0 bis D36-Hardwaretest wurde genau einmal
durchgeführt. Der Mitschnitt erreichte D21 beim Eintritt in
`fsl_esdhc_initialize()`, aber keinen der Marker D22 bis D31.
`fsl_esdhc_initialize()` gab `-12` zurück, `board_mmc_init()`
propagierte denselben Rückgabewert.

Die anschließende statische Analyse grenzte den D21-D22-Korridor
auf zwei bestehende `calloc()`-Aufrufe ein:

* `priv`: 120 Byte
* `plat`: 480 Byte

Mindestens einer dieser beiden Aufrufe liefert auf dem beobachteten
Pfad `NULL`. Der bisherige Hardwaremitschnitt unterscheidet nicht,
welcher der beiden Aufrufe fehlschlägt.

Die statische Prüfung bestätigt zudem, dass der Full-Malloc-Bereich
von `0x18300000` bis `0x18400000` vor D21 initialisiert wird. Eine
Erschöpfung des 8-KiB-`malloc_f`, eine fehlende Full-Malloc-Initialisierung
oder eine allein aufgrund der konfigurierten Größe zu kleine 1-MiB-Region werden daher nicht als Erklärung des beobachteten
Pfads weiterverfolgt.

Aus `-12` beziehungsweise `-ENOMEM` wird insbesondere kein physischer
RAM-Mangel, keine DDR-Korruption und keine dlmalloc-Korruption als
Root Cause abgeleitet.

Der unveränderte D0-bis-D36-Hardwaretest wird daher nicht wiederholt.
Ebenso werden vor der nächsten Beobachtung keine funktionalen
Änderungen an MMC-Nummerierung, USDHC-Auswahl, IOMUX, Clock-Timing,
Timeouts, Allocatorgröße, Heaplayout oder Rückgabesemantik
vorgenommen.

Als nächster zulässiger Hardwaretest wird ausschließlich ein minimaler
D21A-D21D-Diagnosetest präregistriert. Er beobachtet die
Allocator-Grenzen `mem_malloc_start`, `mem_malloc_end` und
`mem_malloc_brk` sowie die Rückgabewerte der beiden bestehenden
`calloc()`-Aufrufe. Der bestehende D22-Marker bleibt erhalten.

Vor Patch, Build und Hardwaretest werden die Ergebnisfälle A bis E
festgelegt:

* A: die erste Allokation `priv` liefert `NULL`.
* B: `priv` liefert einen Wert, die zweite Allokation `plat` liefert `NULL`.
* C: beide Allokationen sind erfolgreich und D22 erscheint; die bisherige
  D21-D22-Zuordnung wird neu bewertet.
* D: die neue Zustandsdiagnose beeinflusst den Pfad; A, B oder C werden
  nicht zugewiesen.
* E: die erwarteten Allocator-Invarianten sind bereits vor dem
  ersten `calloc()` verletzt.

Diese Entscheidung definiert ausschließlich den nächsten
Diagnoseschritt. Sie ist kein Nachweis einer funktionalen oder
Hardware-Root-Cause.

## 2026-09-23 – D21A-D21D-Hardwaretest erst nach SPL-Größenqualifikation

Der präregistrierte D21A-D21D-Diagnosetest bleibt der nächste
zulässige P_EXT-Hardwaretest.

Vor einem Schreibzugriff auf die externe Novena-Test-SD wurde die
gegenüber D0-D36 vergrößerte SPL deshalb unabhängig auf
Buildidentität, Imageaufbau und Größenlimits geprüft.

Der qualifizierte D21A-D21D-SPL besitzt:

* Größe `0xDC00` = `56320` Byte;
* SHA-256
  `6b16b53c6f2346b24ea010a1b6bb3b44beedfe09f61652450cf76e84180133d0`.

Ein im exakt durch `flake.lock` festgelegten Nixpkgs-Kontext
erzeugter Analyse-Buildbaum liefert einen byteidentischen SPL.

Die Raw-SPL besitzt `0xC048` Byte und liegt unter
`CONFIG_SPL_MAX_SIZE=0x10000`. Das finale i.MX-Boot-Image besitzt
`0xDC00` Byte und liegt unter `CONFIG_SPL_SIZE_LIMIT=0x11000`.

Der erzeugte i.MX-Boot-Data-Header enthält eine Größe von `0xE000`.
Der Raw-Payload beginnt innerhalb der finalen SPL-Datei bei Offset
`0xC00`.

Für die Novena-SD-Ablage ab Medienoffset `0x400` gilt:

`[0x400,0xE000)`

als exakt zu sichernder und bei einem später freigegebenen Test zu
beschreibender SPL-Bereich.

Die initiale 4-KiB-Laderegion des i.MX6-ROM-Bootpfads wird nicht als
maximale SPL-Gesamtgröße interpretiert.

Vor dem tatsächlichen Schreibvorgang muss die physische externe
Novena-Test-SD erneut eindeutig identifiziert werden. Der vollständige
Zielbereich ist vor dem Schreiben zu sichern. Anschließend darf nur
die exakt qualifizierte SPL geschrieben und durch einen bytegenauen
Readback verifiziert werden.

Erst danach ist genau ein D21A-D21D-P_EXT-Hardwareboot mit bereits
aktiver serieller Aufzeichnung zulässig.

Aus der Größenqualifikation wird keine Aussage darüber abgeleitet,
welcher der beiden `calloc()`-Aufrufe den zuvor beobachteten
`-ENOMEM`-Pfad verursacht.

## 2026-09-23 – Vor einem Allocator-Fix wird Q1/Q2/P0 präregistriert

Der inzwischen ausgeführte D21A-D21D-Hardwaretest zeigt bereits vor der
ersten `calloc()`-Allokation einen inkonsistenten Full-Malloc-Zustand:

* `mem_malloc_start=0x18300000`;
* `mem_malloc_end=0x18400000`;
* `mem_malloc_brk=0x00000000`.

Die vollständige D21A-D21D-Auswertung lokalisiert den beobachteten
`-ENOMEM`-Pfad auf die erste `calloc()`-Allokation von `priv`.

Der anschließende präregistrierte P0/P1-Test qualifiziert Fall C:
`mem_malloc_brk` ist bereits unmittelbar nach der Rückkehr aus
`mem_malloc_init()` Null und bleibt an P1 sowie D21A Null.

Ein byteidentischer Analyse-Build des auf der Hardware getesteten
P0/P1-SPL bestätigt gleichzeitig, dass der erzeugte Maschinencode in
`mem_malloc_init()` den Wert `0x18300000` nach `mem_malloc_brk` bei
`0x1820003c` speichert. Der anschließend ausgeführte Heap-`memset()`
beschreibt ausschließlich `[0x18300000,0x18400000)` und überlappt
`mem_malloc_brk` damit im normalen Adressintervall nicht.

Aus diesen Befunden wird noch kein funktionaler Allocator-Fix
abgeleitet. Insbesondere wird nicht behauptet, dass der erzeugte Store
auf der realen Hardware unmittelbar anschließend bereits als
`0x18300000` zurückgelesen werden kann.

Als nächster zulässiger Diagnoseschritt wird deshalb ausschließlich ein
minimaler Drei-Punkt-Test Q1/Q2/P0 festgelegt:

* Q1 liest `mem_malloc_brk` per echtem `volatile`-Load unmittelbar nach
  dem bestehenden Store und vor dem Heap-`memset()`;
* Q1 wird ausschließlich in einer lokalen automatischen `ulong`-Variable
  gehalten;
* zwischen Q1 und dem Heap-`memset()` erfolgt keine Diagnoseausgabe;
* Q2 liest `mem_malloc_brk` per echtem `volatile`-Load unmittelbar nach
  Rückkehr aus dem Heap-`memset()`;
* Q1 und Q2 werden erst nach Q2 gemeinsam ausgegeben;
* P0 bleibt als bestehende Beobachtung unmittelbar nach Rückkehr aus
  `mem_malloc_init()` erhalten;
* P1 und D21A bleiben ebenfalls erhalten.

Die öffentliche Signatur von `mem_malloc_init()` wird für diesen Test
nicht verändert. Es erfolgen keine Headeränderung und keine neue globale
oder statische Diagnosevariable.

Die Position von Q2 ist gegenüber dem vorherigen Entwurf ausdrücklich
präzisiert: Q2 liegt innerhalb von `mem_malloc_init()` unmittelbar nach
dem Heap-`memset()`. P0 bleibt davon getrennt der Messpunkt nach Rückkehr
aus `mem_malloc_init()`.

Die vorregistrierte Ergebnismatrix lautet:

* 7A: Q1=`0x18300000`, Q2=`0x18300000`, P0=`0x18300000`;
* 7B: Q1=`0x18300000`, Q2=`0x00000000`, P0=`0x00000000`;
* 7C: Q1=`0x00000000`, Q2=`0x00000000`, P0=`0x00000000`;
* 7E: Q1=`0x18300000`, Q2=`0x18300000`, P0=`0x00000000`;
* 7D: jede andere Kombination.

Vor einer Hardwarefreigabe muss die erzeugte Maschine statisch
qualifiziert werden. Dabei müssen insbesondere echte Q1-/Q2-Loads aus
`0x1820003c`, die unveränderte Heap-`memset()`-Semantik, das Fehlen
neuer persistenter BSS-Diagnoseobjekte sowie die SPL-Größen-, ROM-,
Container- und Mediengrenzen bestätigt werden.

Der derzeit vorhandene Patch
`boot/u-boot/0007-novena-diag-brk-q1-q2.patch` ist nur ein älteres
Source-Experiment und noch nicht die Implementierung dieser Entscheidung.
Er ist nicht in `default.nix` eingebunden und nicht hardwarequalifiziert.

Bis zur Implementierung und vollständigen statischen Qualifikation
dieses Drei-Punkt-Tests gilt:

* kein Schreibzugriff auf die Novena-Test-SD;
* kein weiterer P_EXT-Hardwareboot;
* kein funktionaler Allocator-Fix;
* keine Wiederholung bereits qualifizierter D0-D36-, D21A-D21D- oder
  P0/P1-Hardwaretests.

Die abgeschlossene I2C3-/ES8328-Untersuchung wird dadurch nicht wieder
geöffnet.

## 2026-09-23 – Fall 7C verschiebt die nächste Diagnose vor den Heap-memset

Der präregistrierte Q1/Q2/P0-Test wurde nach vollständiger statischer
Qualifikation genau einmal auf der realen Novena ausgeführt.

Die Messwerte lauten:

```text
Q1 = 0x00000000
Q2 = 0x00000000
P0 = 0x00000000
```

Damit ist der präregistrierte Fall `7C` qualifiziert.

Die statische Maschinencode-Qualifikation bestätigt, dass Q1 ein echter
Load von `mem_malloc_brk` bei `0x1820003c` unmittelbar nach dem
bestehenden Store und vor dem Heap-`memset()` ist. Zwischen dem
bestehenden Store, Q1 und dem anschließenden Heap-`memset()` befindet
sich keine Diagnoseausgabe.

Aus Fall `7C` wird deshalb entschieden:

* der Heap-`memset()` wird nicht als erstmalige Ursache des bereits bei
  Q1 beobachteten Nullwerts weiterverfolgt;
* die Rückkehr aus `mem_malloc_init()`, der P0-zu-P1-Pfad, der
  Loader-Callback und der MMC-/eSDHC-Pfad werden ebenfalls nicht als
  erstmalige Ursache dieses Nullwerts weiterverfolgt;
* Fall `7B` ist durch den Hardwarebefund ausgeschlossen;
* aus Fall `7C` wird weiterhin kein funktionaler Allocator-Fix
  abgeleitet;
* insbesondere wird noch keine konkrete Root Cause für das beobachtete
  Store-/Load-Verhalten behauptet.

Der nächste Diagnoseschritt muss das verbleibende Fenster um den
bestehenden Store weiter zerlegen. Vor einer Implementierung muss ein
neuer falsifizierbarer Test präregistriert werden, der insbesondere den
tatsächlichen Eingangswert des bestehenden Stores von dessen
unmittelbarem Store-/Load-Verhalten unterscheidet.

Bis zu dieser neuen Präregistrierung gilt:

* kein Patch `0008`;
* kein weiterer Schreibzugriff auf die Novena-Test-SD;
* kein weiterer P_EXT-Hardwareboot;
* keine Wiederholung des Q1/Q2/P0-Hardwaretests;
* keine Wiederholung der bereits qualifizierten D0-D36-,
  D21A-D21D- oder P0/P1-Hardwaretests.

Der aktuelle Hardwarestatus ist:

```text
TOTAL_P_EXT_HARDWAREBOOT_COUNT=3
Q1_Q2_P0_HARDWAREBOOT_COUNT=1
SECOND_Q1_Q2_P0_BOOT_ALLOWED=NO
ANOTHER_BOOT_ALLOWED=NO
TOTAL_0007_SD_WRITE_COUNT=1
SECOND_0007_SD_WRITE_ALLOWED=NO
PREREGISTERED_RESULT_CLASS=7C
PATCH_0008_ALLOWED=NO
```

Die historische Untersuchung des P_EXT-Bootpfads, einschließlich der
Zuordnung von externer SD, USDHC2 und USDHC3, bleibt ein davon
getrennter Untersuchungsstrang. Aus ihr wird keine Erklärung für den
bereits vor dem MMC-/eSDHC-Pfad beobachteten Fall `7C` abgeleitet.

Die abgeschlossene I2C3-/ES8328-Untersuchung bleibt geschlossen.

## 2026-09-28 – Historische Medien werden getrennt und beweisorientiert behandelt

Die historische Medien- und Image-Provenienz ist ein eigener
Untersuchungsstrang und wird nicht mit der aktuellen P_EXT-/Allocator-Diagnose
oder der abgeschlossenen I2C3-/ES8328-Root-Cause-Untersuchung vermischt.

Für historische Originalmedien gilt:

- Ein beobachteter historischer Zustand wird nicht ohne direkten Nachweis als
  unverändertes „Factory Image“ bezeichnet.
- Beobachtete Tatsachen, Primärquellenbefunde, starke Korrelationen und nicht
  nachgewiesene Kausalzusammenhänge werden ausdrücklich getrennt.
- Das historische physische USB-Medium wird nach Erstellung und Verifikation
  des vollständigen lokalen Raw-Master-Images nicht mehr als normale
  Analysequelle verwendet.
- Weitere Analysen erfolgen am verifizierten lokalen Raw-Master-Image; dieses
  wird nicht in-place verändert.
- Unnötige weitere Voll-Lesedurchläufe des physischen historischen
  USB-Mediums werden vermieden.
- Eine gefundene Kopie von `novena-mmc-disk-r1.img` darf erst nach
  bytegenauer Prüfung gegen die dokumentierte historische SHA-256-Prüfsumme
  als verifiziertes r1-Image behandelt werden.
- Historische Befunde werden nicht ohne eigenen Nachweis als Erklärung des
  aktuellen P_EXT-/Allocator-Falls `7C` verwendet.

Der aktuelle historische Forschungsstand und die dazugehörigen
Aussagegrenzen sind in `docs/HISTORICAL-MEDIA.md` dokumentiert.

Insbesondere bleiben folgende Grenzen bestehen:

```text
Q1_HISTORICAL_USB_HYBRID_TRANSFORMATION=UNRESOLVED
Q2_NOVM_NOVS=RESOLVED
Q3_R1_EXACT_PARTITION_GEOMETRY=OPEN
CAUSAL_LINK_INSTALLER_TO_HISTORICAL_USB=NOT_PROVEN
```

Neue historische Untersuchungen sollen erst dann eröffnet werden, wenn neue
Primärevidenz eine der offenen Fragen tatsächlich weiter diskriminieren kann.

## 2026-09-30 – Block 4.10N wird als statische Untersuchungsphase abgeschlossen

Nach dem hardwarequalifizierten Q1/Q2/P0-Ergebnis Fall `7C` wurde das
verbleibende Store-/Load-Fenster ohne weiteren Hardwareboot statisch
untersucht.

Für den exakt hardwarezugehörigen Locked-Build ist nachgewiesen:

- `mem_malloc_brk` liegt bei `0x1820003c` innerhalb der SPL-BSS;
- der BSS-Clear gibt vor `board_init_r()` gewöhnliche CPU-Null-Stores über
  diese Adresse aus;
- `mem_malloc_init()` speichert anschließend `0x18300000` nach exakt
  `0x1820003c`;
- Q1 lädt unmittelbar danach aus exakt derselben Adresse;
- der reale Hardwarewert von Q1 ist trotzdem `0x00000000`;
- dieser Nullwert entspricht dem vorher durch den BSS-Clear vorgesehenen
  Wert, ohne dass daraus ein kausaler Mechanismus abgeleitet wird;
- MMU und D-Cache sind auf dem relevanten Pfad deaktiviert;
- der Locked-Build enthält den U-Boot-Workaround für Cortex-A9-Erratum
  743622;
- die MMDC-DDR-Kalibrierung ist kein Nachweis eines gewöhnlichen
  CPU-Store-/Load-Tests an `0x1820003c`.

Die vollständige physische Zerlegung von `0x1820003c` in
CS/Bank/Row/Column wird nicht aus einer angenommenen generischen
DDR3-Bitbelegung konstruiert. Die MMDC-Abbildung hängt von der konkreten
Konfiguration einschließlich `DSIZ`, `ROW`, `COL`, Bankanzahl,
Bank-Interleaving und Chip-Select-Grenzen ab.

Mangels einer für diesen konkreten Fall vollständig belegten Mapping-Regel
wird die exakte CS-/Bank-/Row-/Column-Zuordnung als `UNKNOWN` dokumentiert.

Diese verbleibende Aussagegrenze verhindert nicht den Abschluss der
statischen Phase. Die bisher verfügbaren statischen Projekt- und
Primärquellen wurden für das Store-/Q1-Fenster ausgeschöpft.

Daraus wird entschieden:

```text
BLOCK_4_10N_STATIC_PHASE=CLOSED
ROOT_CAUSE=UNRESOLVED
PHYSICAL_DDR_MMDC_MECHANISM=UNRESOLVED
EXACT_DRAM_CS_BANK_ROW_COLUMN=UNKNOWN
PATCH_0008_ALLOWED=NO
ANOTHER_BOOT_ALLOWED=NO
```

Es wird kein weiterer unspezifischer Diagnosepatch erzeugt und kein
weiterer P_EXT-Hardwareboot durchgeführt.

Ein zukünftiger Patch `0008` darf erst präregistriert werden, wenn ein
einzelner falsifizierbarer Test vorliegt, der mindestens zwei noch
plausible Mechanismen des verbliebenen physischen CPU-/DDR-/MMDC-Pfads
voneinander unterscheiden kann.

Insbesondere werden weder ein DDR-Defekt noch eine bestimmte
Bank-/Row-/Column-Störung, falsches Bank-Interleaving, ein MMDC-Erratum oder
der BSS-Clear selbst ohne zusätzliche Evidenz zur Root Cause erklärt.

Die I2C3-/ES8328-Untersuchung bleibt geschlossen. Die historische Medien-
und Image-Provenienz bleibt ein separater Untersuchungsstrang.

## 2026-09-30 – Block 4.10O präregistriert einen adressvergleichenden Folgetest für Fall 7C

Nach Abschluss von Block 4.10N wurde der verbleibende Evidenzraum in
4.10O-1A bis 4.10O-1D ausschließlich read-only ausgewertet. Dabei wurde kein
bereits vorhandener unabhängiger früher CPU-DDR-Kontrollzugriff gefunden, der
den beobachteten Store-/Q1-Fall an `mem_malloc_brk` bereits diskriminiert.

Als Kontrolladresse wird `max_total_mem` bei `0x18200030` präregistriert. Die
Problemadresse bleibt `mem_malloc_brk` bei `0x1820003c`. Für beide Vergleiche
wird der Wert `0x18300000` verwendet.

Die Kontrolle soll unmittelbar vor dem normalen `mem_malloc_init()`-Aufruf in
`board_init_r()` den Ausgangswert C0 lesen, `0x18300000` speichern, den
unmittelbaren Wert C1 laden, anschließend C0 wiederherstellen und den Restore
als C2 kontrollieren. Der bestehende reale Store mit unmittelbar folgendem Q1
in `mem_malloc_init()` bleibt unverändert der Target-Test.

Präregistrierte Resultatklassen:

```text
8A = CONTROL_PASS_TARGET_FAIL
8B = CONTROL_FAIL_TARGET_FAIL
8C = CONTROL_PASS_TARGET_PASS
8D = CONTROL_FAIL_TARGET_PASS
8E = UNEXPECTED
8X = TEST_INVALID
```

Ein gültiger Test verlangt, dass der gemessene Ausgangszustand mit dem
BSS-Clear vereinbar ist und nach dem Restore wiederhergestellt wurde. Eine
verletzte Kontrollbedingung führt zu `8X`; daraus wird keine
Mechanismusentscheidung abgeleitet.

Die Resultatklassen sind keine Root-Cause-Klassen. Insbesondere beweist `8A`
keine bestimmte DRAM-Bank-/Row-/Column-/Byte-Lane- oder MMDC-Ursache und `8B`
keinen allgemeinen DDR-Defekt.

Vor einem Hardwaretest muss ein zukünftiger Patch `0008` zunächst gebaut und
statisch qualifiziert werden. Die Disassemblierung muss insbesondere den
beabsichtigten unmittelbaren Kontroll-Store/Load, die Zieladressen, den Restore,
die bestehende Target-Sequenz, das BSS-Layout sowie die SPL-Größen- und
ROM-Grenzen bestätigen.

Die Freigaben werden deshalb nicht zusammengezogen. Mit dieser Entscheidung
wird nur der Testentwurf präregistriert. Noch nicht freigegeben sind:

```text
PATCH_0008_ALLOWED=NOT_YET
BUILD_ALLOWED=NOT_YET
SD_WRITE_ALLOWED=NO
HARDWARE_BOOT_ALLOWED=NO
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO
```

Erst nach Dokumentation und Prüfung dieser Präregistrierung darf in einem
separaten Schritt entschieden werden, Patch `0008` zu erstellen und einen rein
statischen Build durchzuführen. Ein SD-Schreibvorgang und ein weiterer
P_EXT-Hardwareboot benötigen danach weiterhin eine eigene Freigabe.

Die I2C3-/ES8328-Untersuchung bleibt geschlossen. Die historische Medien- und
Image-Provenienz bleibt ein separater Untersuchungsstrang.

## 2026-09-30 – Block 4.10O-1L/1M schließt die statische Patch-0008-Qualifikation ab

Die in Block 4.10O präregistrierte Voraussetzung für einen späteren adressvergleichenden Hardwaretest ist erfüllt.

Patch `0008-novena-diag-control-c0-c2.patch` wurde gebaut und anhand eines Locked-Debug-Builds bis auf Maschinencodeebene qualifiziert. Der normale Patch-0008-Build und der Locked-Debug-Build liefern einen bitidentischen installierten SPL.

Die präregistrierte Kontrolladresse ist weiterhin:

```text
max_total_mem = 0x18200030
```

Das bestehende Target ist weiterhin:

```text
mem_malloc_brk = 0x1820003c
```

Der Abstand beträgt 12 Byte. Der Kontrollwert ist `0x18300000`.

Die Maschinencodeprüfung bestätigt C0, Store V, C1, Restore C0 und C2 in der vorgesehenen Reihenfolge. Sie bestätigt außerdem für das Target den Store nach `mem_malloc_brk` und den anschließenden Q1-Load über denselben unveränderten Target-Zeiger.

Die Raw-SPL-Provenienz ist ebenfalls geschlossen: Das Raw-SPL ist 49.480 Byte groß und kommt im 56.320 Byte großen installierten SPL-Wrapper exakt einmal bei Offset `0xc00` vor.

Die statische Qualifikation stützt sich auf die unabhängig erhaltene Artefakt- und Maschinencode-Evidenz. Eine ursprüngliche Primärausgabe der zuvor dokumentierten Blöcke `4.10O-1L-3C` beziehungsweise `4.10O-1L-3C-R1` konnte bei der späteren Provenienzprüfung nicht aufgefunden werden. Die zuvor dokumentierte Abfolge eines `FINAL_TARGET_GATE=FAIL` und einer anschließenden Korrektur dieses Gates wird deshalb nicht als belegte Projekthistorie fortgeführt.

Es gilt auf Basis der unabhängig belegten technischen Evidenz:

```text
PATCH_0008_BUILD_QUALIFIED=YES
MACHINE_CODE_QUALIFIED=YES
STATIC_QUALIFICATION_GATE=PASS
HARDWARE_TEST_RESULT=NOT_RUN
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO
```

Die in Block 4.10O präregistrierten Klassen 8A bis 8X und ihre Interpretationsgrenzen werden nicht nachträglich geändert. Die mögliche Vorkonditionierung durch den C0-Load bleibt eine bekannte Einschränkung.

Diese Entscheidung gibt noch keinen SD-Schreibvorgang und keinen P_EXT-Hardwareboot frei. Die Hardwarefreigabe wird separat auf Basis des dokumentierten Pre-Hardware-Checkpoints entschieden.


## 2026-09-30 – Block 4.10O-1N: Patch 0008 ist verifiziert auf dem Testmedium

Der qualifizierte Patch-0008-SPL wurde nach vollständiger Medium-, Bereichs- und Baseline-Prüfung gezielt ab Offset `0x400` auf das Novena-Testmedium geschrieben.

Der anschließende Readback ist byteidentisch zum qualifizierten SPL und besitzt SHA-256 `4611111bf2412257bf537a0c3b95a79b796d7359a8ff4de54046cbe2fe56a25c`. Der MBR sowie Disk-ID, Partitionen und Dateisystemidentitäten blieben unverändert.

Die spätere Provenienzprüfung der Dokumentation fand keine ursprüngliche Primärausgabe der zuvor dokumentierten 3C/3C-R1-Gate-Abfolge. Diese Abfolge wird deshalb nicht als belegte Projekthistorie verwendet. Die statische Qualifikation beruht weiterhin auf der unabhängig erhaltenen Artefakt- und Maschinencode-Evidenz.

Es gilt:

```text
PATCH_0008_ON_SD=VERIFIED
SD_WRITE_VERIFIED=YES
POST_WRITE_GATE=PASS
HARDWARE_BOOT_ATTEMPTED=NO
HARDWARE_TEST_RESULT=NOT_RUN
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO
```

Die präregistrierten Klassen 8A bis 8X und ihre Interpretationsgrenzen bleiben unverändert. Ein Hardwareergebnis wird erst durch einen gesondert freigegebenen POR-Test erzeugt.

## 2026-09-30 – Block 4.10O-1N-7B bis 7E qualifiziert den vollständigen Pre-POR-Imagezustand

Vor dem präregistrierten Patch-0008-Hardwaretest wird nicht mit der zuvor vorhandenen gemischten SD-Konfiguration weitergearbeitet. Stattdessen wurde das vollständige NixOS-SD-Image mit dem statisch qualifizierten Patch `0008` neu gebaut, read-only qualifiziert, vollständig auf die Test-SD geschrieben und über den gesamten Imagebereich zurückverifiziert.

Das qualifizierte Image besitzt:

```text
IMAGE_SIZE=2581291008
IMAGE_SHA256=90405b094423b8e515d670f3a72cb3146c68d0ad5f65d5d71d88a965966f0fe3
SPL_SHA256=4611111bf2412257bf537a0c3b95a79b796d7359a8ff4de54046cbe2fe56a25c
U_BOOT_DTB_IMG_SHA256=af608b67beb8c250a08d1b5fe9e96ef6096df4df0b9a2833b128396c4c195104
```

Der vollständige Readback der ersten `2581291008` Byte der Test-SD stimmt SHA-256-genau mit diesem Image überein. Damit stammen insbesondere SPL und `u-boot-dtb.img` für den kommenden Test aus dem gemeinsam qualifizierten aktuellen Buildstand.

Diese Entscheidung verändert weder die Präregistrierung noch die Ergebnisinterpretation. Ein späteres Ergebnis wird weiterhin ausschließlich anhand der zuvor festgelegten Fälle `8A` bis `8X` klassifiziert. Aus der Image- oder Datenträgerqualifikation selbst wird keine Aussage über die DDR-Root-Cause abgeleitet.

```text
PRE_POR_MEDIA_STATE_QUALIFIED=YES
HARDWARE_TEST_RESULT=NOT_RUN
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO
```

## 2026-10-01 – Patch-0008-Hardwaretest qualifiziert Fall 8B

Der in Block 4.10O präregistrierte adressvergleichende
Kontrolltest wurde am 2026-10-01 genau einmal auf der
Novena ausgeführt.

Die Gültigkeitswerte C0 und C2 waren jeweils Null.

Der Kontrollwert C1 und der Zielwert Q1 waren ebenfalls
jeweils Null.

Damit wird das Ergebnis entsprechend der unveränderten
Präregistrierung als Fall `8B` klassifiziert:

`CONTROL_FAIL_TARGET_FAIL`

Das Ergebnis zeigt, dass die beobachtete fehlgeschlagene
Rücklesung nicht auf die einzelne Target-Adresse
`mem_malloc_brk` beschränkt ist.

Es beweist keinen allgemeinen DDR-Defekt und keinen
konkreten Hardware- oder Softwaremechanismus.

Entscheidungen:

1. Fall 8B wird als gültiges Hardwareergebnis dokumentiert.
2. Die Präregistrierung der Fälle 8A bis 8X bleibt unverändert.
3. Der Patch-0008-Hardwaretest wird nicht wiederholt.
4. Der Originalmitschnitt bleibt unverändert erhalten.
5. Vor weiteren Tests, Builds oder Patches erfolgt ein
   systematischer Quellen- und Ergebnisabgleich.
6. Die bestehende Diagnoseinstrumentierung wird vorerst
   weder entfernt noch als dauerhafte Lösung übernommen.

HARDWARE_TEST_RESULT=8B
PATCH_0008_SECOND_BOOT_ALLOWED=NO
ROOT_CAUSE=UNRESOLVED
ROOT_CAUSE_CLAIM_ALLOWED=NO

## 2026-10-02 – BLOCK 9C-44: Rekonstruktion erhalten, Umsetzung gesperrt

Die lokal statisch rekonstruierbaren Themen aus dem Auftrag zu 9C-42/9C-43
werden im [Dokumentationscheckpoint](../research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md)
erhalten. Fehlende Repository-Originalberichte 9C-33 bis 9C-42 werden nicht
durch eine erfundene Ablaufhistorie ersetzt.

Ignorierte WL-/DQS- und Warte-Rückgaben begründen eine Diagnoselücke,
keine bewiesene Ursache von Fall 8B. ERR050070 wird ohne zugänglichen
Hersteller-Primärbeleg nicht zur verifizierten Ursache oder Fixgrundlage.
MPDGHWST-Lesesemantik bleibt UNKNOWN; zusätzliche Lesungen werden nicht
als automatisch nebenwirkungsfreie Diagnose vorausgesetzt.

A/B werden im Checkpoint gemäß Auftrag BLOCK 9C-49 als Angaben aus dem
früheren Chatbericht zu BLOCK 9C-42 geführt, nicht als lokal verifizierte
Repository-Originalquelle: A ist eine lokal initialisierte Diagnose-Struktur
auf dem SRAM-Stack mit Parameterweitergabe an WL-/DQS-Helfer; B ist ein
initialisierter globaler Diagnosezustand in SRAM-`.data` bei unveränderten
Funktionssignaturen. Optionaler Messumfang ist davon getrennt.
Keine Variante ist ausgewählt, implementiert, präregistriert oder freigegeben.
Vor einer späteren konkreten Freigabe wären Primärquellenlücken, SRAM-Stack-
Bedarf, Erhaltung vor dem BSS-Clear, geänderter DQS-Tail-Pfad und erneute
Qualifikation der hartcodierten Patch-0008-Control-/Targetadressen zu klären.
Die Fallmatrix 8A–8X und ihre Aussagegrenzen werden nicht geändert.

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_HARDWARE_BOOT_COUNT=1
PATCH_0008_SECOND_BOOT_ALLOWED=NO
```

## 2026-10-04 – Dokumentationscheckpoint nach vorläufigem Abschluss der statischen Fall-8B-Untersuchung

Die statische Untersuchung für Fall 8B gilt gemäß ausdrücklicher Auftragsangabe zum Abschluss von BLOCK 9C-47W-8G vorerst als ausgeschöpft. Dies ist kein Nachweis einer DDR-Root-Cause und kein Ausschluss sämtlicher denkbarer Ursachen. Der fehlende Originalbericht wird nicht durch eine rekonstruierte Ablaufhistorie ersetzt.

Der einmalige Patch-0008-Hardwarelauf bleibt Fall `8B = CONTROL_FAIL_TARGET_FAIL`; Kontrollgültigkeit und bisherige Aussagegrenzen bleiben unverändert. Er wird nicht wiederholt.

Die inzwischen vorhandenen RM-Rev.-2-Originalpassagen erlauben eine präzisere Dokumentation von MPDGHWST-Ergebnisfeldern, Ergebnislesekontext und Halbtaktkorrektur. Sie begründen keine allgemeine Nebenwirkungsfreiheit zusätzlicher Lesungen. Die dokumentierte frühere ERR050070-Inhaltsprüfung wird von fehlenden lokalen Originalbytes, konkreter Siliconanwendbarkeit und unbewiesener Auslösung getrennt.

Historische Codewerte `0xc0`, spätere Änderungen auf `0x80`, Fehlerbitmasken und ignorierte Rückgaben werden als Quellenbefunde erhalten. Daraus folgt keine Implementierungsentscheidung. Keine Diagnosearchitektur A/B wird ausgewählt oder freigegeben.

Dieser Checkpoint erteilt keine Schreib-, Implementierungs-, Build-, Medien-, Hardwaretest- oder Pushfreigabe. Ein zweiter Patch-0008-Boot bleibt verboten.

Die funktionale I2C3-/ES8328-Untersuchung bleibt geschlossen. `regulator-always-on` für `es8328-power` bleibt dauerhafte Boardanforderung.

```text
ROOT_CAUSE=UNRESOLVED
STATIC_EVIDENCE_EXHAUSTED=YES
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
PUSH_AUTHORIZED=NO
```

## 2026-10-09 – INDEX-1K-58-R1: K2-Umfang und Drei-Dateien-Dokumentationscheckpoint

Der in INDEX-1K-56 genehmigte Benutzerbeschluss wird hier additiv
dokumentiert. INDEX-1K-33 definiert die Anforderungskennungen, INDEX-1K-54
auditiert den damaligen Stand und INDEX-1K-55 ordnet den Mindestumfang ein.
INDEX-1K-56 entwirft die Dokumentation; INDEX-1K-57 klärt deren
TEST-LOG-Provenienzkonflikt. Der vorliegende Auftrag autorisiert ausschließlich
Nachträge in `docs/DECISIONS.md`, `docs/PROJECT-STATE.md` und
`docs/project-index/README.md`. Frühere Vorschläge und PENDING-Werte bleiben
als historische Aussagen unverändert erhalten.

### Genehmigter Umfang

| Kategorie | Anforderungen | Bedeutung |
|---|---|---|
| A | K2-S01 bis K2-S09, K2-S13, K2-S16 | Verbindlicher Mindestumfang: elf Anforderungen. |
| B | K2-S10, K2-S14 | Optional; keine Voraussetzung des Mindestumfangs. |
| C | K2-S11, K2-S12, K2-S15, K2-S17, K2-S18 | Außerhalb des beschlossenen Mindestumfangs; keine Erfüllungsbehauptung. |

K2 bezeichnet eine endliche historische Regression und eine begrenzte
v0.3-Regelerweiterung. Die Kennungen K2-S01–S18 sind Anforderungen aus
INDEX33/55 und nicht die Testnamen K2-01–K2-12.

| Anforderung A | Festgelegter Nachweisumfang |
|---|---|
| K2-S01 | Historisches Archiv und vollständigen vertraglichen Mitgliederbestand vor Oracleverwendung authentifizieren. |
| K2-S02 | Unveränderte historische Validatorbytes vor Kompilierung authentifizieren. |
| K2-S03 | Vier historische Tabellenheader und 16 direkte Parserkontrollen: gültiger Inhalt, falscher Header, zusätzliche und fehlende Spalte je Tabelle. |
| K2-S04 | Alle 13 historischen Datensätze mit 110 Feldwerten erhalten; zusätzliche Datensätze zulassen. Identische Serialisierung und Zeilenreihenfolge werden nicht verlangt. |
| K2-S05 | Die 24 benannten Korpusfälle und 17 isolierten Strukturfälle der identifizierten Regression. |
| K2-S06 | Zehn direkte Pfadkontrollen, sieben direkte Originalkontrollen und separater Repository-Abwesenheitsvergleich im Speichermodell. |
| K2-S07 | Benannte Fehlerklassen und Weitergabe unerwarteter Ausnahmen; ausschließlich die jeweils passende Invalid-Klasse als Regelresultat behandeln. |
| K2-S08 | Isolierte und Registerprüfstufen vor finalem Regression-PASS integrieren, einschließlich der vorhandenen gezielten Integrationskontrollen. |
| K2-S09 | Unabhängig definierte Eingaben und Sollresultate sowie Assertionsnachweis für 18 isolierte Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/270 Adapteraufrufe. |
| K2-S13 | Vorhandene, in INDEX26 bestandene Regel-, Teilprädikat-, Integrations- und Inventurmutanten; keine vollständige Mutationsabdeckung. |
| K2-S16 | Genau fünf autorisierte Registeridentitäten, sieben Identitätsbindungen, exakt leerer Originalhash, je 26 Varianten und fünf zusätzliche vollständige autorisierte Vergleichspfade. |

K2-S10 würde die unabhängige Einzelinventur auf Korpus, Direktkontrollen und
historische Feldmutationen erweitern. K2-S14 würde zusätzliche gezielte
Auditmutanten aufnehmen. C umfasst vollständige CLI-Qualifikation (S11),
umfassende reale Dateisystem-/Originalqualifikation (S12), aktive
Audit-Hook-Qualifikation (S15), vollständige historische R01–R66/H01–H35-Erfüllung
(S17) und universelle Semantikäquivalenz (S18). Der Ausschluss aus dem
Mindestumfang entfernt keine bestehenden Validatorregeln.

Die frühere v0.3-Ablehnung fehlender lokaler Originalhashes und die autorisierten
Registerausnahmen sind ausgewiesene zulässige Regelunterschiede. Ein
Registerhash verifiziert Registerbytes und keine fehlenden Originalbytes.

### Evidenz und erhaltene Bewertungen

Die lokal gelesenen Originalprotokolle liegen unter
`/home/loomit/novena-backups/block-logs/`:

- `INDEX-1K-26-20261008-114733-kY2DVghx.txt`: historische Testausführung.
- `INDEX-1K-27-20261008-114907-mvFmpmV7.txt`: begrenzte unabhängige Bewertung.
- `INDEX-1K-33-20261008-121745-O3Xy6bZi.txt`: ursprüngliche S01–S18-Matrix.
- `INDEX-1K-54-20261008-203110-xgBzzHM6.txt`: fachliches Abschlussaudit.
- `INDEX-1K-55-20261008-204340-9DFXYLcG.txt`: Einordnung des Mindestumfangs.
- `INDEX-1K-56-20261008-205143-cxpY6Ist.txt`: Benutzerbeschluss und unvollständiger Vier-Dateien-Plan.
- `INDEX-1K-57-20261009-084923-rvMPAcRU.txt`: Provenienzanalyse und vorgeschlagene Drei-Dateien-Lösung.

Alle sieben Protokollhashes stimmen beim lokalen lesenden Abgleich in
INDEX-1K-58-R1 mit ihren vorhandenen Sidecars überein. Hashgleichheit belegt
Byteübereinstimmung und keine fachliche Gesamtabnahme. INDEX26 ist
Ausführungsevidenz; spätere Bewertungen sind keine erneuten Testläufe.

INDEX26 protokolliert 96 bestandene Unit-Testmethoden, 24 historische
Korpusfälle, 33 Direktkontrollen, `REGISTER_EXTENSION=PASS`,
`V02_REGRESSION=PASS` und jeweils Exitstatus 0 für Unit-Tests und Regression.
Die M04/M05-Inventur ist ein Assertionsnachweis mit den unter S09 genannten
Zahlen; eine separat erhaltene Ereignisliste jedes Vergleichs fehlt.
INDEX27 endet mit `INDEPENDENT_ACCEPTANCE=REQUIRES_REVIEW`.

```text
K2_SCOPE_DECISION=APPROVED_BY_USER
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_ACCEPTANCE=NOT_ESTABLISHED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
```

K1-PASS gilt für die gepinnten ART-000001–000009-/SAL-000001–000009-Bestände
und geprüften Bindungsfehler. REGISTER_TYPES-QUALIFIED_PASS gilt für die fünf
autorisierten Identitäten mit je 26 Varianten und fünf vollständigen Pfaden.
TEST_INDEPENDENCE-QUALIFIED_PASS gilt ausschließlich für den 18/130-Bestand.
Der Umfangsbeschluss ist keine K2-Abnahme und keine Projektindex-Gesamtabnahme.

### Historische Evidenzgrenzen

Authentifizierte historische Originalmatrizen R01–R66/H01–H35, die
Originalberichte INDEX9–11 und der vollständige INDEX14-Plan sind im geprüften
Bestand nicht nachgewiesen. Rekonstruktionen und INDEX25 ersetzen sie nicht.
INDEX37 ist in INDEX54 indirekt historisch eingeordnet; INDEX39 liegt dort
hinsichtlich der Pakete A–C als übergebene Zusammenfassung vor. Fehlende
TXT-Dateien beweisen allein keine fehlende Durchführung; ohne Originalausgaben
wird deren Inhalt aber nicht als direkt verifiziert dargestellt. Für diesen
Dokumentationsbeschluss sind keine Wiederholungen von INDEX37/39 erforderlich.

Die neue unabhängige Einzelinventur umfasst weder den separaten 24-Fall-Korpus
samt Registererweiterungsvergleich noch die 33 Direktkontrollen oder sämtliche
historischen Feldmutationen. Gemeinsame Ausgangsdaten, Serialisierung,
Ergebnisadapter und `MemoryPath` bleiben Unabhängigkeitsgrenzen; die historische
Datenaufbereitung verwendet aktuelle `v.SCHEMAS`. Die kanonische
INDEX25-Spezifikation hat keine zweite unveränderliche Fallquelle.

Gezielte dynamische Sensitivität gegen das alleinige Auslassen des
Vergleichsprädikats, selektive Vergleichs-/Phasenduplikate, Auslassen nur des
vollständigen Registerpfads, Identitätsvertauschung bei gleicher Fehlerklasse
und Sollinhaltsänderung bei gleicher Identitätsmenge bleibt unbelegt.
Statisch sichtbare Schutzmechanismen sind kein solcher Laufzeitnachweis.

`ORIGINALS_CHECK=PASS checked=3` stammt aus einem kontrollierten Unitfixture
und belegt keine vollständige reale Originalprüfung. Das vereinfachte
Speichermodell qualifiziert weder umfassende reale Symlink-/Berechtigungs-/IO-
Semantik noch sämtliche CLI-Pfade. Der INDEX26-Lauf über `unittest discover`
aktivierte den ausschließlich unter `__main__` installierten Audit-Hook nicht.
Dateiidentitätskontrollen sind getrennte Evidenz, keine globale
Nebenwirkungsinventur. Daraus folgt kein nachgewiesener Schreibvorfall.

### FND-000001 und Drei-Dateien-Übergangslösung

FND-000001 bleibt mit allen historischen Feldern erhalten: Status `OPEN`,
Scope `Patch 0008 single documented boot`, Aussage Fall 8B ohne etablierte
Root Cause, `record_locator=docs/TEST-LOG.md#L2362` und
`record_sha256=b165636a7e5146903d9675639d8d435a474c88f7231341922781bbf027b00719`.
Diese Bindung ist identisch im historischen TAR. Die Locatorprüfung verlangt
Zeilenexistenz, keine textliche Übereinstimmung mit der Finding-Aussage;
`originals()` prüft den Hash der gesamten aufgelösten Datei.

**Der Konflikt betrifft eine Ergänzung von TEST-LOG; aktuell besteht für diese
Bindung kein Hashbruch.** Selbst ein additiver Nachtrag würde den Ganzdateihash
ändern. Hash oder Locator nachzuziehen widerspräche dem historischen
Feldwerterhalt unter K2-S04/`preserve()`. Strukturprüfung und historische
Fixture-Regression belegen keinen vollständigen realen Originalprüfungs-PASS.

Das historische Archiv
`/home/loomit/novena-backups/project-index-v02-before-v03-60785fc2/project-index-v02.tar`
hat SHA256 `18afce33e3e7a0f6cc4851b6c3e3fe7366031c806575ea671454b0c9ad4d4844`.
Es enthält ein Verzeichnis und sechs Dateien unter `docs/project-index/`,
aber **keine Kopie der referenzierten TEST-LOG-Originaldatei**. Es bewahrt deren
Hash als Registerwert, nicht deren Originalbytes.

Der exakt passende historische Snapshot ist lokal im Git-Objektbestand
vorhanden: Commit `60785fc2ea36f0a12d03411c9812a616f06a810e`, Pfad
`docs/TEST-LOG.md`, Blob `620460235449764df08daeadf863a53b0fea093d`.
In INDEX-1K-58-R1 wurden diese Git-Bytes lesend direkt mit der aktuellen
Datei verglichen: byteidentisch. Eine zusätzliche Snapshotdatei wurde nicht
angelegt. Dies belegt lokale Byteidentität, keine weitergehende Herkunftskette
und keine vollständige konsistente zusätzliche Provenienzwurzel.

Der INDEX56-Vier-Dateien-Plan wird für diesen Auftrag ausdrücklich durch den
begrenzten Drei-Dateien-Schritt ersetzt: Beschluss hier, aktueller Indexstatus
in PROJECT-STATE und Einstieg/Evidenzzuordnung im Index-README. Die Index-TSVs
binden diese drei Dateien nicht als Originale. `docs/TEST-LOG.md` bleibt am
bisherigen Pfad byteidentisch. Die dort geplante Evidenzzuordnung wird durch
diesen Nachtrag und die beiden verlinkten Nachträge zugänglich gemacht.
Eine vollständige Umsetzung des Vier-Dateien-Plans wird nicht behauptet.

### Offene Abnahmefragen

Eine eigene fachliche Bewertung muss den identifizierten Nachweisen den nun
verbindlichen endlichen A-Umfang gegenüberstellen und die Qualifikationsgrenzen
ausdrücklich anerkennen. Ob daraus eine K2-Abnahme und anschließend eine
Projektindex-Gesamtabnahme folgt, bleibt offen. B erfordert bei späterer
Aufnahme eigene begründete und autorisierte Nachweise; C wird hier nicht erfüllt.

Für fortlaufende Testdokumentation bleibt eine eigene Pfad-/Snapshotentscheidung
offen. Ein zusätzlicher Git-Snapshothinweis oder neue Findings allein ändern
die bestehende Auflösung von FND-000001 nicht. Eine getrennte Provenienzwurzel
müsste sämtliche Bindungen konsistent bedienen; eine versionierte Auflösung
wäre eine zusätzliche, separat zu spezifizierende Validatorsemantik. Historische
Felder, TAR und Ganzdateihashregeln werden nicht stillschweigend geändert.

Dieser Checkpoint führt keine Tests, Builds, Hardware- oder Netzwerkaktionen
und keine Git-Mutationen durch. Ein Runner oder zusätzliche Protokolldateien
werden innerhalb der Drei-Dateien-Freigabe nicht angelegt. Die DDR-/SPL-
Untersuchung bleibt ein eigener Strang; historische Daten bleiben unverändert.
Der [Projektstatus](PROJECT-STATE.md#2026-10-09--index-1k-58-r1-k2-dokumentationscheckpoint)
und der [Indexeinstieg](project-index/README.md#2026-10-09--index-1k-58-r1-k2-dokumentationscheckpoint)
führen diesen Beschluss weiter.

## 2026-10-09 – INDEX-1K-61: Fachliche K2-Abnahme für Umfang A

Dieser eigenständige Nachtrag dokumentiert das fachliche Abnahmevotum aus
INDEX-1K-60. Der in INDEX56 genehmigte und in INDEX58-R1 dokumentierte
Mindestumfang bleibt unverändert: K2-S01 bis K2-S09, K2-S13 und K2-S16.

```text
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
```

| Pflichtanforderungen | Bewertung aus INDEX60 |
|---|---|
| S01, S02, S03, S04, S05, S06, S08, S13 | PASS (acht Anforderungen) |
| S07, S09, S16 | QUALIFIED_PASS (drei Anforderungen) |

Die Entscheidung erkennt die folgenden Einschränkungen ausdrücklich an:

- **S07:** Die passenden Invalid-Klassen, exakten benannten Fehlerklassen
  und die Weitergabe geprüfter unerwarteter Ausnahmen sind belegt.
  INDEX26 protokolliert unter anderem `test_wrong_rule_class_rejected`,
  `test_oserror_not_rule_result`, `test_unexpected_exception_not_rule_result`
  und `test_INDEX18_direct_comparison_exceptions_propagate`. Eine vollständige
  Ausnahmeabdeckung sämtlicher CLI-/I/O-Einstiegspunkte folgt daraus nicht.
- **S09:** Die beiden bestandenen M04/M05-Inventurmethoden in INDEX26 und
  der begrenzte Review INDEX27 tragen die unabhängig definierten Eingaben
  und Sollresultate: 18 isolierte Vergleiche/36 Adapteraufrufe sowie
  130 Registerfälle/135 Vergleiche/270 Adapteraufrufe. Das sind
  Assertionsnachweise ohne separat erhaltene Einzelereignisliste.
  Ausgangsdaten, Schemas, Serialisierung, Ergebnisadapter und `MemoryPath`
  bleiben gemeinsam; die kanonische INDEX25-Spezifikation besitzt keine
  zweite unveränderliche Fallquelle. Eine vollständig unabhängige zweite
  Implementierung wird nicht behauptet.
- **S16:** Die in INDEX26 protokollierten Register-, Bindungs-, Leerhash-
  und Inventurtests tragen genau SRC-000009/000010/000012/000013/000014,
  sieben Identitätsbindungen, je 26 Varianten und fünf zusätzliche
  vollständige autorisierte Vergleichspfade im Modell. Der Originalhash
  muss exakt `''` sein; `None`, `False` und `0` sind Python-Negativwerte,
  keine zusätzlichen TSV-Arten. Registerhashes bestätigen Registerbytes,
  keine fehlenden Originale. Die Ausnahme ist eine begrenzte v0.3-Erweiterung.

INDEX60 bewertet damit alle elf Anforderungen innerhalb des beschlossenen
endlichen A-Umfangs als erfüllt; dort verbleibt keine entscheidende
Evidenzlücke. Das Votum stützt sich auf historische Ausführungsevidenz aus
INDEX26, den begrenzten Review INDEX27 und die in INDEX60 statisch bestätigten
Dateiidentitäten. Die acht PASS gelten ebenfalls nur für ihre festgelegten
Archiv-, Oracle-, Parser-, Feld-, Fall-, Modell-, Integrations- und
Mutantennachweise, nicht für universelle Vollständigkeit.

Die für diesen Nachtrag lokal geprüften Originalprotokolle liegen unter
`/home/loomit/novena-backups/block-logs/`:

- `INDEX-1K-56-20261008-205143-cxpY6Ist.txt`: Umfangsbeschluss; Vier-Dateien-Plan noch unvollständig.
- `INDEX-1K-57-20261009-084923-rvMPAcRU.txt`: Provenienzkonflikt und vorgeschlagene Drei-Dateien-Lösung.
- `INDEX-1K-58-R1-20261009-090422-uNhAI48f.txt`: additive Umfangsdokumentation, noch keine Abnahme.
- `INDEX-1K-59-20261009-092022-hMxY3qDP.txt`: Dokumentationsreview PASS und K2-Abnahmebereitschaft.
- `INDEX-1K-60-20261009-093539-m2uOpPQh.txt`: eigenständige fachliche Evidenzmatrix und Abnahmevotum.

Alle fünf stimmen in INDEX61 beim lesenden Hashabgleich mit ihren vorhandenen
SHA256-Sidecars überein. Die technischen Originalnachweise INDEX26/27 sind
im vorstehenden INDEX58-R1-Nachtrag exakt benannt. INDEX59/60 sind Bewertungen
und keine neuen Testläufe; INDEX61 dokumentiert diese Entscheidung.

B (S10/S14) bleibt optional und offen. Die zusätzliche unabhängige
Einzelinventur für Korpus, Direktkontrollen und sämtliche Feldmutationen
ist nicht Bestandteil des nachgewiesenen Umfangs. C (S11/S12/S15/S17/S18)
bleibt ausgeschlossen: umfassende CLI-, reale Dateisystem-/Original- und
aktive Audit-Hook-Qualifikation sowie universelle Semantikäquivalenz sind
unbelegt; authentifizierte historische R01–R66/H01–H35-Originalmatrizen,
INDEX9–11-Originalberichte und der vollständige INDEX14-Plan bleiben Lücken.
Rekonstruktionen ersetzen diese Originale nicht. Die indirekte beziehungsweise
übergebene Provenienz von INDEX37/39 wird nicht aufgewertet.

**Die fachliche K2-Abnahme ist ausdrücklich keine Projektindex-Gesamtabnahme.**
Frühere PENDING-/NOT_ESTABLISHED-Werte bleiben als damalige Kenntnisstände
erhalten. TEST-LOG bleibt byteidentisch; FND-000001 einschließlich
`record_locator` und `record_sha256` bleibt unverändert. Das historische TAR
enthält keine TEST-LOG-Originalkopie; der vorhandene Git-Snapshot ist keine
implementierte Snapshotstrategie. Snapshot-/Resolverentscheidung und
Projektindex-Gesamtabnahme bleiben offen. Neue Tests oder eine Erweiterung
des Mindestumfangs werden durch diesen Nachtrag nicht verlangt.

## 2026-10-09 – INDEX-1K-63: Verbindlicher Projektindex-Gesamtabnahmeumfang

Der ausdrückliche Benutzerbeschluss im Auftrag zu INDEX-1K-63 legt die
bereits belegten Teilqualifikationen gemeinsam als vollständigen verbindlichen
Projektindex-Gesamtabnahmeumfang fest. Die dokumentierten qualifizierten
Einschränkungen sind ausdrücklich Bestandteil dieses Umfangs.

```text
PROJECT_INDEX_SCOPE_DECISION=APPROVED_BY_USER
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
PROJECT_INDEX_ACCEPTANCE=NOT_ESTABLISHED
FINAL_ACCEPTANCE_REVIEW=PENDING
```

Grundlage des Abgleichs sind INDEX-1K-58-R1 (genehmigter K2-Umfang und
Drei-Dateien-Übergangslösung), INDEX-1K-61 (dokumentiertes fachliches
K2-Votum aus INDEX60) und die fachliche Bewertung INDEX-1K-62. Deren lokale
Originalquelle ist
`/home/loomit/novena-backups/block-logs/INDEX-1K-62-20261009-095001-LFe8QbYs.txt`,
SHA256 `2bc3405d5a038e6e2567df9c223998e7d8cbb290c8722312a1beaae1ba933e1b`;
der lesende Hashabgleich in INDEX63 stimmt mit dem vorhandenen Sidecar überein.
INDEX62 benennt den damals fehlenden verbindlichen Gesamtumfang als
Entscheidungshindernis und stellt keine technischen Pflichtblocker für K2-A
fest. Der jetzige Benutzerbeschluss beantwortet diese Umfangsfrage; er
ändert das historische Votum INDEX62 nicht rückwirkend.

### Anerkannte Teilumfänge und Grenzen

- K1-PASS gilt für die gepinnten ART-000001–000009-/SAL-000001–000009-
  Pflichtbindungen und geprüften Bindungsfehler.
- REGISTER_TYPES-QUALIFIED_PASS gilt für genau fünf autorisierte Identitäten,
  sieben Identitätsbindungen, exakt leeren Originalhash, je 26 Varianten und
  fünf zusätzliche vollständige Vergleichspfade im Modell. Registerhashes
  bestätigen Registerbytes, keine fehlenden Originalbytes.
- TEST_INDEPENDENCE-QUALIFIED_PASS gilt ausschließlich für den 18/130-Bestand:
  18 isolierte Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/
  270 Adapteraufrufe als Assertionsnachweis ohne separate Einzelereignisliste.
  Ausgangsdaten, Schemas, Serialisierung, Adapter und `MemoryPath` bleiben
  gemeinsam; die kanonische Sollinstanz hat keine zweite unveränderliche
  Fallquelle. Ein globaler Unabhängigkeitsnachweis wird nicht behauptet.
- K2-A umfasst genau K2-S01 bis K2-S09, K2-S13 und K2-S16 nach den exakten
  Definitionen in INDEX58-R1. INDEX61 dokumentiert acht PASS
  (S01–S06/S08/S13) und drei QUALIFIED_PASS (S07/S09/S16).
  S07 bleibt auf benannte Fehlerklassen und geprüfte Ausnahmezweige begrenzt;
  S09/S16 behalten die vorstehenden Inventur- und Registergrenzen.
  Die übrigen A-Nachweise sind ebenfalls endlich; vorhandene Mutanten
  belegen keine vollständige Mutationsabdeckung.

Historische Originalqualifikation wird nicht als neue Pflicht aufgenommen.
Die fehlenden authentifizierten R01–R66/H01–H35-Originalmatrizen,
INDEX9–11-Originalberichte und der vollständige INDEX14-Plan bleiben
Quellenlücken; Rekonstruktionen ersetzen sie nicht. INDEX37/39 behalten ihre
indirekte beziehungsweise übergebene Provenienz. Eine vollständige reale
Originalprüfung ist nicht belegt; `ORIGINALS_CHECK=PASS checked=3` bleibt
Unitfixture-Evidenz. Der historische discover-Lauf aktivierte den Audit-Hook
nicht; Dateiidentitätskontrollen sind keine globale Nebenwirkungsinventur.
Diese Grenzen werden anerkannt, ohne fehlende Nachweise als erbracht auszugeben.

B (S10 zusätzliche Einzelinventur für Korpus, Direktkontrollen und sämtliche
Feldmutationen; S14 zusätzliche Auditmutanten) bleibt optional. C
(S11 umfassende CLI-, S12 reale Dateisystem-/Original-, S15 aktive Audit-Hook-
Qualifikation, S17 historische Matrixerfüllung, S18 universelle
Semantikäquivalenz) bleibt außerhalb des Pflichtumfangs. Eine zusätzliche
Snapshot-/Resolverstrategie ist langfristige Folgearbeit und kein neues
Pflichtkriterium der Gesamtabnahme. Bestehende Validatorregeln, Vertragspins
und historische Feldwerterhaltung bleiben davon unberührt.

FND-000001 und seine historische Locator-/Ganzdateihashbindung bleiben
unverändert; TEST-LOG bleibt byteidentisch am bisherigen Pfad. Das historische
TAR enthält keine TEST-LOG-Originalkopie; der vorhandene lokale Git-Snapshot
ist keine implementierte Snapshot-/Resolverstrategie. Vor einer künftigen
TEST-LOG-Änderung bleibt eine konsistente separate Lösung erforderlich.

### Beschluss und Gesamtabnahme bleiben getrennt

**Genehmigt ist der Gesamtumfang, nicht die Projektindex-Gesamtabnahme.**
Die abschließende fachliche Gesamtabnahmeprüfung gegen diesen nun verbindlichen
Umfang einschließlich seiner Einschränkungen steht aus. Aus den Teilwerten
oder diesem Dokumentationscheckpoint folgt keine automatische Gesamtabnahme.
Historische Statuswerte, Aussagen und Abschnitte bleiben unverändert erhalten.

Der [Projektstatus](PROJECT-STATE.md#2026-10-09--index-1k-63-verbindlicher-gesamtumfang-und-offene-gesamtabnahme)
und der [Indexeinstieg](project-index/README.md#2026-10-09--index-1k-63-verbindlicher-projektindex-gesamtabnahmeumfang)
dokumentieren denselben Beschluss. INDEX63 ergänzt ausschließlich diese drei
Dateien; keine weiteren Dateien, Runnerlogs oder Sicherungen werden angelegt.
Keine neuen Testläufe, Builds, Hardwaretests, Netzwerkaktionen oder
Git-Mutationen; keine neue Hardwareaussage oder Änderung des DDR-/SPL-Stands.

## 2026-10-09 – INDEX-1K-65: Formaler qualifizierter Projektindex-Abschluss

Der Benutzer hat nach der fachlichen Bewertung INDEX-1K-64 ausdrücklich
die formale qualifizierte Projektindex-Gesamtabnahme freigegeben. Dieser
Nachtrag dokumentiert die neue Benutzerentscheidung vom 2026-10-09 nach
INDEX64. INDEX61 dokumentierte ausschließlich die fachliche K2-Abnahme,
INDEX63 den verbindlichen Gesamtumfang; INDEX64 gab die abschließende
fachliche Empfehlung ab und ließ die formale Benutzerabnahme noch offen.

Der verbindliche, nun formal abgenommene Umfang bleibt:

```text
K1=PASS
REGISTER_TYPES=QUALIFIED_PASS
TEST_INDEPENDENCE=QUALIFIED_PASS
K2_SCOPE=A_ONLY
K2_MANDATORY_REQUIREMENTS=11
K2_ACCEPTANCE=QUALIFIED_ACCEPTED
```

Die fachliche Bewertung aus INDEX-1K-64 wird als Abnahmegrundlage übernommen:

```text
CHECKPOINT_REVIEW=PASS
DOCUMENTATION_REVIEW=PASS
EVIDENCE_REVIEW=QUALIFIED_PASS
MANDATORY_BLOCKERS=NONE_IDENTIFIED
PROJECT_INDEX_ACCEPTANCE_RECOMMENDATION=QUALIFIED_ACCEPTED
```

Die jetzt ausdrücklich erteilte formale Benutzerfreigabe lautet:

```text
PROJECT_INDEX_SCOPE_DECISION=APPROVED_BY_USER
PROJECT_INDEX_ACCEPTANCE=QUALIFIED_ACCEPTED
FINAL_USER_ACCEPTANCE=APPROVED
ACCEPTANCE_BASIS=INDEX-1K-63,INDEX-1K-64
```

Damit ist die formale qualifizierte Projektindex-Gesamtabnahme für den
in INDEX63 genehmigten endlichen Umfang abgeschlossen. Das fachliche
Votum INDEX64 und die anschließende Benutzerentscheidung bleiben als
unterschiedliche Schritte nachvollziehbar. Die Freigabe ist die ausdrückliche
Entscheidung im Auftrag zu INDEX65, keine aus Teilwerten oder Runner-PASS
abgeleitete automatische Gesamtabnahme.

K1 gilt für die gepinnten ART-/SAL-Pflichtbindungen. REGISTER_TYPES gilt
für genau fünf autorisierte Identitäten, sieben Bindungen, exakt leeren
Originalhash, je 26 Varianten und fünf zusätzliche vollständige Modellpfade;
Registerhashes verifizieren keine fehlenden Originalbytes.
TEST_INDEPENDENCE bleibt ausschließlich auf den 18/130-Bestand begrenzt:
18 isolierte Vergleiche/36 Adapteraufrufe und 130 Registerfälle/135 Vergleiche/
270 Adapteraufrufe als Assertionsnachweis ohne separate Einzelereignisliste.
Gemeinsame Daten, Schemas, Serialisierung, Adapter und `MemoryPath` sowie
die kanonische Sollinstanz ohne zweite unveränderliche Fallquelle bleiben
anerkannte Einschränkungen. K2-A umfasst S01–S09/S13/S16: acht PASS
(S01–S06/S08/S13), drei QUALIFIED_PASS (S07/S09/S16). S07 bleibt auf benannte
Fehlerklassen und geprüfte Ausnahmezweige begrenzt. Teilweise gemeinsame
Evidenz ist keine zusätzliche unabhängige Bestätigung.

Die Abnahme umfasst ausdrücklich nicht:

- vollständige historische Originalqualifikation;
- universelle Semantikäquivalenz;
- globale Testunabhängigkeit;
- vollständige reale Dateisystem-/Originalprüfung;
- neue Aussagen über Hardware, DDR, SPL oder Bootfähigkeit;
- optionale K2-Bereiche B (S10/S14) und ausgeschlossene Bereiche C
  (S11/S12/S15/S17/S18);
- eine implementierte Snapshot-/Resolverstrategie.

Die dokumentierten Quellenlücken und Evidenzgrenzen bleiben bestehen.
Rekonstruktionen ersetzen keine fehlenden Originalmatrizen oder -berichte;
`ORIGINALS_CHECK=PASS checked=3` bleibt Unitfixture-Evidenz. Der historische
discover-Lauf aktivierte den Audit-Hook nicht. Vorhandene Mutanten belegen
keine vollständige Mutationsabdeckung. Die Snapshot-/Resolverlösung für
künftige TEST-LOG-Änderungen bleibt separate langfristige Folgearbeit.

Alle früheren Aussagen und Statuswerte, insbesondere NOT_ESTABLISHED,
PENDING und das damalige FINAL_USER_ACCEPTANCE=PENDING aus INDEX64, bleiben
als historische Kenntnisstände unverändert. Die neue Entscheidung gilt ab
diesem Nachtrag und ausschließlich für den benannten Projektindexumfang.
Bestehende Validatorregeln, Vertragspins und historische Felder bleiben
unberührt. FND-000001 behält sämtliche historischen Felder einschließlich
`record_locator=docs/TEST-LOG.md#L2362` und
`record_sha256=b165636a7e5146903d9675639d8d435a474c88f7231341922781bbf027b00719`.
Der Hash bindet weiterhin die gesamte unveränderte TEST-LOG-Datei am
bisherigen Pfad. Der vorhandene lokale Git-Snapshot ist keine implementierte
Snapshot-/Resolverstrategie.

Die lokal geprüften Ergebnisprotokolle liegen unter
`/home/loomit/novena-backups/block-logs/`:

| Block | Originalprotokoll | SHA256 |
|---|---|---|
| INDEX-1K-61 | `INDEX-1K-61-20261009-094329-9VNEICD3.txt` | `1bdd98498923c6bd15b9dc2485acae2c5af2372c1b7c46524dbfcef8ded390ef` |
| INDEX-1K-63 | `INDEX-1K-63-20261009-095443-7J4YhpM9.txt` | `41fc4a4c89d67db9cc88380411d3a218bd4b5b9746357dc2786cdadb3406fb92` |
| INDEX-1K-64 | `INDEX-1K-64-20261009-100341-rQPTvBmV.txt` | `24b46e161efe2f111ffd5123c774a60861db54af89382eeaeca16b7d3bd9519a` |

Alle drei Hashes stimmen in INDEX65 beim lesenden Abgleich mit den vorhandenen
Sidecars überein. Ihre Abschlüsse weisen COMMAND_EXIT_STATUS=0,
BLOCK_RESULT=PASS und LOG_COMPLETE=YES aus; die fachlichen Ergebnisse wurden
getrennt geprüft. INDEX64 ist eine lesende fachliche Bewertung, kein neuer
Testlauf. Die historische Laufzeitevidenz bleibt INDEX26 mit den dokumentierten
Reviewgrenzen.

INDEX65 ergänzt ausschließlich DECISIONS, PROJECT-STATE und Index-README.
Die bisherigen Dateiinhalte bleiben bytegenau erhalten. Keine weiteren
Dateien, temporären Dateien, Runnerlogs oder Sicherungen werden erzeugt.
Keine Git-Mutationen, Commits, Pushes, Builds, Tests, Hardwarezugriffe oder
Netzwerkaktionen werden durchgeführt.

Der [Abnahmestatus](PROJECT-STATE.md#2026-10-09--index-1k-65-formaler-qualifizierter-projektindex-abschluss)
und der [Indexeinstieg](project-index/README.md#2026-10-09--index-1k-65-formaler-qualifizierter-projektindex-abschluss)
führen denselben neuen Beschluss.

## 2026-10-12 – NOVENA-TOOLS-CLOSE-01: begrenzter Werkzeugabschluss

Dieser datierte Nachtrag dokumentiert die ausdrücklich beauftragten
Werkzeugentscheidungen. Ausschließliche Primärnachweise sind die vollständig
gelesenen lokalen Originale unter `~/novena-backups/block-logs/`:

| Audit | Originalprotokoll | SHA256 |
|---|---|---|
| NOVENA-TOOLS-AUDIT-01 | `NOVENA-TOOLS-AUDIT-01-20261012-122941-mQgkXb6F.txt` | `ed93c2d1287abb073c10e7edce1e12d931dd7d9575c9d87dddf967085935f6db` |
| NOVENA-TOOLS-AUDIT-02 | `NOVENA-TOOLS-AUDIT-02-20261012-123144-1VsvMjZQ.txt` | `215be39817bdc7f8c9feea854dea5d6f9e69bf3254829cec0f6130c0b9d22514` |

Der lesende SHA256-Abgleich stimmt jeweils mit dem vorhandenen Sidecar
überein. Beide Abschlüsse weisen `COMMAND_EXIT_STATUS=0`, `BLOCK_RESULT=PASS`
und `LOG_COMPLETE=YES` aus. Die Audits sind read-only: AUDIT-01 belegt die
Verfügbarkeit der unten genannten Werkzeuge sowie die vorhandenen Runner- und
Wrapperpfade; AUDIT-02 dokumentiert `INDEX_FORMAT=TSV_JSON_PYTHON` und
`SQLITE_IMPORT_POSSIBLE=TO_BE_QUALIFIED`. Das sind begrenzte Struktur- und
Verfügbarkeitsnachweise, keine umfassende Funktionsqualifikation.

### Entscheidungen

1. Vorhandene `sqlite3`, `rg`, `fd`, `find`, `jq`, `plocate`, `rsync`,
   `sha256sum` und `b2sum` weiterverwenden. Ihre dokumentierte Verfügbarkeit
   trägt die Beibehaltung; zusätzliche Werkzeuge sind dafür nicht erforderlich.
2. Recoll nicht installieren. Erst bei nachgewiesenem Bedarf für
   dokumentübergreifende Volltextsuche neu bewerten. Die Audits belegen keinen
   solchen Bedarf und keinen funktionalen Vergleich.
3. Restic und Borg nicht installieren. Erst bei gesonderter Überarbeitung des
   Backup-Konzepts vergleichen. Aus der Werkzeugverfügbarkeit folgt keine
   Backup- oder Restorequalifikation; der vollständige R6-Restore bleibt offen.
4. Für rmlint und Czkawka besteht kein aktueller Installationsbedarf.
   Die Audits liefern keinen Nachweis eines konkreten zusätzlichen Nutzens.
5. TSV, JSON und Python für den Projektindex beibehalten. SQLite-Verfügbarkeit
   allein begründet keine Integration; diese verlangt konkreten Mehrwert und
   eigene Qualifikation. Der Projektindex und catalog-v2 werden nicht verändert.
6. Bestehenden Runner `~/novena-backups/block-logs/run-block.bash` und
   GitHub-Wrapper `~/novena-backups/github-transport/run-and-upload.bash`
   weiterverwenden und nicht verändern. Der Pfadnachweis ist keine vollständige
   Funktions- oder Sicherheitsqualifikation und erteilt keine Ausführungsfreigabe.
7. Keine zusätzliche Automatisierungsplattform einführen.

Der Abschluss betrifft die Audit-Auswertung und diese Auswahlentscheidungen.
Spätere bedarfsabhängige Prüfungen sind in TODO zurückgestellt; vorhandene
fachfremde Aufgaben bleiben erhalten. Historische FAILs werden nicht verändert
oder nachträglich aufgewertet. Ein vollständiger R6-Restore wird nicht behauptet.

NOVENA-TOOLS-CLOSE-01 ergänzt ausschließlich TODO und DECISIONS. Keine
Installation, Werkzeugtests, Builds, Hardwaretests, Netzwerkzugriffe,
Git-Mutationen, Commits oder Pushes; keine neuen Projektdateien und keine
automatischen Reparaturen. Bestehende Inhalte und Nutzeränderungen bleiben
erhalten. Die Audit-Originale und Sidecars bleiben unverändert.

> Aktueller Abschluss: siehe den Nachtrag „2026-10-15 – Abschluss und Übergabe nach Borg 35P-19“ am Dateiende; ältere Stände sind historisch.

## 2026-10-15 – Abschluss und Übergabe nach Borg 35P-19

Dieser datierte Nachtrag ist für den hier beschriebenen Abschluss maßgeblich.
Ältere Statusangaben bleiben historische Checkpoints. Grundlage sind die
vollständig ausgewerteten lokalen TXT-Originale samt übereinstimmenden
SHA256-Sidecars; das Nachweisregister steht im gleichnamigen Nachtrag in
`docs/TEST-LOG.md` (aus `docs/` heraus: `TEST-LOG.md`).

- **Borg-27:** nachgewiesene reale Sicherung am 15.10.2026,
  13:29:56–13:40:47 CEST. `BORGMATIC_CREATE_EXIT=0`, genau ein neues Archiv
  (Bestand 2 → 3), `ARCHIVE_CHECK=PASS` und
  `ARCHIVE_DATA_VERIFICATION=PASS`. Dies ist keine Restore-Abnahme.
- **35O-R5:** 64 erfolgreiche Offline-Tests in fünf Suiten
  (8 + 22 + 10 + 15 + 9); keine reale Borg-Ausführung in diesem Testblock.
- **35P-12:** R4-Laufzeitnachweis erfolgreich: Downstream betreten,
  äußere Sperre während der Bridge gehalten, konkurrierender R4-Aufruf
  abgewiesen, Bridge abgeschlossen, Sperre nach Rückkehr freigegeben.
  **Der R4-Laufzeitnachweis ist nicht mehr offen.** R4 ist ein
  Offline-Prüfmechanismus; daraus folgt keine produktive Repository-Sperre
  für eine reale Sicherung und keine produktive Gesamtfreigabe.
- **35P-14:** FAIL, Exitstatus 1 wegen eines fehlerhaft formatierten
  SHA256-Prüfblocks. Kein daraus belegter Quelldefekt. **35P-14R1:**
  korrigierte Aufrufpfadanalyse PASS; der Testdateihash wurde erfasst,
  aber ausdrücklich nicht mit einem historischen Sollhash verglichen.
  Der frühere FAIL bleibt erhalten.
- **35P-15/16:** Kontrollfluss-, Syntax- beziehungsweise Strukturanalyse
  des bestehenden Tagesabschlusses erfolgreich; keine reale Sicherung.
- **35P-17:** Offline-Borg-Nachweisprüfer erfolgreich gegen Borg-27,
  einschließlich Protokollhash, Sidecar, Archivanzahl, Archiv-ID,
  Archivprüfung, Datenverifikation und Runnernachweis.
- **35P-18:** Offline-Integrationstest und Abweisung einer nicht
  autorisierten Tagesabschlussausführung erfolgreich. Der Starter
  `~/novena-backups/workflow-improvement-35/borg/novena-dayclose-with-borg-35p18.bash`
  prüft den historischen Borg-27-Nachweis und startet keine Sicherung.
  Die Nachweisauswahl ist noch fest gebunden, nicht dynamisch.
- **35P-19:** integrierter Tagesabschluss erfolgreich ausgeführt
  (16:06:05 CEST; `DAYCLOSE_EXECUTED=YES`,
  `DAYCLOSE_INTEGRATION=PASS`, Runner-PASS, Exitstatus 0).
  Dabei kein Borg-/borgmatic-Aufruf und kein Borg-Repositoryzugriff.

**Korrektur veralteter Statusausgaben:** Der in 35P-19 eingebettete ältere
Tagesabschluss meldet weiterhin `BORG_FULL_LOCK_DURATION_PROOF=OPEN` und
führt den Laufzeitnachweis als offene Aufgabe. Das ist durch 35P-12 überholt,
kein neuer Fehlschlag. Auch die damaligen pauschalen offenen Integrationspunkte
sind anhand 35P-17/18/19 auf den tatsächlich erreichten Offline-Nachweisumfang
zu begrenzen. Die Skriptausgabe bleibt unverändert als Originalevidenz erhalten.
Der belegte Ausführungserfolg von 35P-19 ersetzt weder eine unabhängige
Vollständigkeitsinventur aller Tagesarbeiten noch eine produktive Borg-Abnahme.
Upload beziehungsweise Remote-Archivierung wurden in diesem Dokumentationsauftrag
nicht erneut geprüft; lokale Logintegrität und fachliche Ergebnisse sind davon getrennt.

**Offen:** Restore-Abnahme, Offsite-Schlüsselsicherung einschließlich
Wiederherstellbarkeit und produktive Gesamtfreigabe. Die dynamische Auswahl
eines geeigneten aktuellen Sicherungsnachweises sowie die produktive
Sperr-, Freigabe-, Provenienz- und Nachweiserfassung bleiben gesondert zu
qualifizieren. Der vollständige R6-Restore bleibt eine separate offene Aufgabe.

**Nächste Projektpriorität: Novena – universelles NixOS SD-Image.**
Borg-Arbeiten sind für die Wiederaufnahme zurückgestellt. Vor weiteren Builds,
Hardwaretests oder Änderungen zuerst den aktuellen Projektstand, TODO,
Artefaktprovenienz und vollständige vorhandene Boot-/Testnachweise prüfen.
DDR/SPL und der vollständige externe Bootpfad bleiben offen; der dokumentierte
DDR-08H-Befund ist Q1/C1 ZERO und D32 `ret=-12`, kein bestätigter Patchfix
oder vollständiger Boot. I2C3/ES8328 bleibt abgeschlossen;
`regulator-always-on` bleibt dauerhafte Boardanforderung.

Entscheidung für die Übergabe: Offline-Abnahme, reale historische Sicherung
und produktive Freigabe getrennt führen. Keine Wiederholung des abgeschlossenen
R4-Nachweises ohne neue diskriminierende Evidenz. Die Entscheidung vom
12.10., Borg vorerst nicht einzusetzen, ist durch die belegte reale Sicherung
vom 15.10. als damaliger Planungsstand überholt.


## 2026-10-17 – Zwei-Skripte-Workflow: realer E2E-Test PASS

Dieser Nachtrag ist für den aktuellen Bedienungsworkflow maßgeblich.
Ältere Planungs-, Sperr- und Integrationshinweise bleiben historische Stände.
Die älteren `integration/*`- und V8-Dateien bleiben unverändert erhalten.

Aktuelle Bedienungsskripte auf foobox:

- Projektstart: `~/novena-backups/workflow-dev/chat-start.bash`.
- Tagesabschluss: `~/novena-backups/workflow-dev/daily-close.bash`.

Beide werden über den vorhandenen GitHub-Wrapper
`~/novena-backups/github-transport/run-and-upload.bash` ausgeführt.
Die Dokumentation erteilt keine Ausführungs- oder Netzwerkfreigabe.
Der Tagesabschluss führt eine reale Borg-Sicherung aus; sein Aufruf benötigt
entsprechende Freigabe und ist keine rein lesende Statusabfrage.

- **NOVENA-WORKFLOW-TWO-SCRIPTS-04: PASS** – beide Skripte installiert.
- **NOVENA-CHAT-START-TEST-05: PASS** – Projektstart, Git-HEAD,
  Index-SHA256, Worktree, Dokumentinventur und Übergabe erfolgreich.
- **NOVENA-DAILYCLOSE-HARDEN-06: PASS** – Installation mit flock-Sperre,
  Borg-Identitätsprüfung und Archivvergleich am 17.10., 09:45:53 CEST.
  Ein separater Wiederholungsaufruf um 09:47:46–09:47:47 endete mit
  `BASELINE_INTEGRITY=FAIL`, Exit 22; dieser FAIL bleibt erhalten.
- **NOVENA-DAILYCLOSE-E2E-07: PASS** – realer End-to-End-Lauf am
  17.10.2026, 09:54:18–09:54:55 CEST. Exklusive Sperre erworben,
  Borg-Identitätsprüfung und Borgmatic-Konfiguration erfolgreich,
  Backup erstellt. Archivbestand **5 → 6**, genau **1 neues Archiv**,
  **0 verschwundene Archive**. Git-HEAD und Index unverändert.
  Vollständiges TXT-Protokoll und SHA256 vorhanden; GitHub-Upload laut
  verifiziertem Benutzerergebnis erfolgreich, hier nicht erneut abgefragt.

**Abgrenzung:** Der reale E2E-Nachweis ist erfolgreich; ältere pauschale
`PRODUCTIVE_END_TO_END_PROOF=OPEN`- beziehungsweise daraus abgeleitete
Sperrhinweise beschreiben den damaligen Workflow. Ein separater
Parallelstart-/Konkurrenztest der aktuellen flock-Sperre wurde noch nicht
durchgeführt und bleibt offen. Weder der Sperrerwerb im Einzellauf noch
historische R4-/Offline-Tests belegen diesen Konkurrenzfall. Eine umfassende
Fehlerfallqualifikation oder produktive Gesamtfreigabe folgt daraus nicht.

Evidenz: lokal vollständig gelesene Originalprotokolle und jeweils
übereinstimmende SHA256-Sidecars; das Nachweisregister steht im
[TEST-LOG](TEST-LOG.md#2026-10-17--zwei-skripte-workflow-realer-e2e-test-pass).

Entscheidung für die Dokumentation: Die beiden `workflow-dev`-Skripte bilden
den aktuellen Bedienungsweg. Historische Implementierungen und Statuswerte
bleiben erhalten und werden nicht rückwirkend aufgewertet. Der erfolgreiche
reale E2E-Lauf und die noch fehlende Konkurrenzqualifikation werden getrennt
geführt; keine neue allgemeine Offline-Testserie oder automatische
Wiederholung des erfolgreichen Backups wird daraus abgeleitet.


## 2026-10-17 – NOVENA-STRATEGY-01: eigenständiges externes NixOS-SD-Image

Der Benutzer hat diese Projektstrategie ausdrücklich freigegeben. Ziel ist
ein vollständig eigenständig bootfähiges, reproduzierbares NixOS-Image auf
einer einzigen externen SD-Karte, ohne Abhängigkeit von Boot- oder Root-Dateien
eines vorhandenen internen Systems. Vorrang hat der schnellste technisch
belastbare Weg; möglichst moderne Bootloader sind kein Selbstzweck.

Bevorzugte Zielkette und Arbeitshypothese:

`i.MX6-ROM → externe SD / P_EXT → historischer Novena-SPL → dazu passendes historisches U-Boot proper → moderner geeigneter Linux-Kernel → initrd und Device Tree → NixOS-Root auf derselben externen SD`

Diese Kette ist noch kein vollständig qualifizierter Hardwareerfolg. Die
Priorisierung gilt ab diesem Nachtrag. Insbesondere wird die Entscheidung
vom 2026-09-21 zugunsten von U-Boot v2026.07 als bisheriger Basis nicht
rückwirkend umgeschrieben; sie beschreibt den damaligen Entwicklungsweg.
Die Strategie allein ändert keine Implementierung und keinen Imageinhalt.

### Berücksichtigte Ergebnisse und Aussagegrenzen

- SOURCE-14 konsolidiert den DDR-/SPL-Forschungsstand: Root Cause und
 konkreter Novena-WALAT-Verstoß bleiben unbewiesen; unveränderte negative
 Fixversuche und breite Suchen ohne neue Suchkante nicht wiederholen.
- SOURCE-15 bewertet die Registererfassung als grundsätzlich im Quellablauf
 platzierbar, gibt den vorhandenen Snapshotentwurf aber nicht zur Umsetzung
 frei. Zielbasis, SRAM-Platzierung, Lebensdauer, Stackgrenzen,
 Registerlesezeitpunkte und Fehler-/Timeoutbindung bleiben zu klären.
- SOURCE-16 trennt ROM-Boot, SPL-Ladepfad und spätere U-Boot-Bootziele.
 Versionsübergreifende SPL-/Proper-Paare benötigen einen konkreten
 Übergabevertrag; bloßer Dateiaustausch ist keine Qualifikation.
- SOURCE-17 belegt einen gespeicherten historischen bd5-ROM-Wrapper,
 quellseitige Unterstützung für externe SD/USDHC2 und den historischen
 Suchdateinamen `u-boot.img` auf FAT-P1. Der geprüfte moderne Legacy-Header
 passt statisch im untersuchten Umfang. Das bestehende Image enthält nach
 diesem Bericht nicht den erforderlichen FAT-Dateinamen; eine passende
 Headerstruktur allein qualifiziert weder Medium noch Laufzeit.
 Ein Mischboot historischer SPL / modernes Proper ist hardwareseitig
 nicht qualifiziert; vollständige Speicherlayoutkompatibilität bleibt offen.

Lokale Originalprotokolle unter `~/novena-backups/block-logs/`:

| Block | TXT | SHA256 |
|---|---|---|
| SOURCE-14 | `NOVENA-SOURCE-14-20261017-123330-zbgcyzxd.txt` | `10193e95fc8e07d6922f1561a182dc16018c8474a5b3e32e8172f376058e9d83` |
| SOURCE-15 | `NOVENA-SOURCE-15-20261017-124216-Q3YYixSb.txt` | `248a9561f30b83dd6d604bbca40a0140cf817b7f30e8cccdf4202cfbb9b722c0` |
| SOURCE-16 | `NOVENA-SOURCE-16-20261017-125228-PwrPwyN5.txt` | `e124bab54f042cd0e2ded2059756d11d1f8eb04f718954743c12453cf2464e90` |
| SOURCE-17 | `NOVENA-SOURCE-17-20261017-130015-yBhqDcg6.txt` | `82592c0acf5d691b5690592f95c607929103ed71410c1f1b69629216de565f7b` |

Für diesen Dokumentationsauftrag wurden die lokalen Ganzdateihashes gegen
die Sidecars abgeglichen und die relevanten Ergebnisabschnitte gezielt
gelesen; keine erneute vollständige technische Quellenqualifikation.
Die Protokollabschlüsse weisen jeweils Runner-PASS und `LOG_COMPLETE=YES`
aus. Das ist kein Hardware-PASS. Remote-Archivierung wurde hier nicht geprüft.

Nicht nachgewiesen sind der vollständig verifizierte historische SPL-Boot
von externer SD, die vollständige bevorzugte Kette bis NixOS und die Fähigkeit
des historischen Proper, beliebige moderne Kernel und initrds zu laden.
Die früheren NixOS-Erfolge mit internem historischem Bootloader bleiben auf
jene Bootgrenze beschränkt. Die DDR-Root-Cause bleibt offen.

### Alternativen und nächster Schritt

Historischer SPL plus modernes U-Boot proper bleibt eine untersuchte
Ausweichoption. Moderner SPL/U-Boot v2026.07 bleibt langfristige
Modernisierung. Weitere DDR-/WL-/DQS-Diagnose wird vorerst zurückgestellt;
das ist keine fachliche Auflösung der offenen Ursache. Die früheren
Empfehlungen aus SOURCE-14 bis SOURCE-17 bleiben historische Bewertungen;
für den nächsten Schritt gilt jetzt die Benutzerpriorisierung SOURCE-18.

**SOURCE-18 ist als nächster technischer Schritt ausschließlich lesend
vorgesehen:** Bestandsaufnahme der vollständigen historischen Bootloaderkette
und ihrer Eignung für NixOS auf externer SD. Sie soll konkrete SPL-/Proper-
Identitäten und Quellenprovenienz, ROM-/FAT-/USDHC-Ladeverträge,
Environment und Bootskript, Kernel-/initrd-/DTB-Formate, Größen und
Lade-/Entpack-/Übergabeadressen sowie die Root-Auswahl auf derselben externen
SD anhand vorhandener Belege prüfen. Fehlende Nachweise bleiben offen.
SOURCE-18 wird in diesem Dokumentationsauftrag nicht ausgeführt.

Für alle technischen Blöcke bleiben bestehender Runner, vollständiges TXT
und SHA256 sowie Archivierung über den bestehenden GitHub-Upload-Wrapper
verbindlich. Runnerstatus, Integrität, Transport und fachliches Ergebnis
getrennt bewerten. Historische Originale und Golden-Referenzen schützen;
Raw-Master nicht in-place verändern, physische Originalmedien nicht erneut
heranziehen. I2C3/ES8328 bleibt abgeschlossen; `regulator-always-on` bleibt
dauerhafte Boardanforderung.

NOVENA-STRATEGY-01 erlaubt ausschließlich additive Dokumentation in
DECISIONS, PROJECT-STATE und der eindeutig identifizierten Projekt-TODO
`TODO.md`. Keine zusätzlichen Dateien oder externen Schreibzugriffe,
keine Git-Schreiboperationen, Builds, Medienzugriffe oder Hardwaretests.
Wegen des ausdrücklichen Netzwerkverbots und des engen Dateiumfangs werden
hier weder Runnerlogs erzeugt noch Wrapper/Upload ausgeführt; eine
Archivierung dieses Dokumentationsschritts wird nicht behauptet.

## 2026-10-17 – NOVENA-STRATEGY-02: Debian-Referenz und geringstes technisches Risiko

Dieser ausdrücklich bestätigte Beschluss präzisiert NOVENA-STRATEGY-01 und
ist für die weitere Priorisierung maßgeblich. Frühere Einträge bleiben als
historische Kenntnisstände erhalten. Vorrang hat der Weg mit dem geringsten
technischen Risiko; die frühere Formulierung „schnellster technisch
belastbarer Weg“ ist entsprechend präzisiert.

1. Primärziel bleibt ein vollständig eigenständig von externer SD bootfähiges
   NixOS-System, ohne Boot- oder Root-Dateien eines internen Systems.
2. Zunächst historisches Debian als Referenzsystem untersuchen. Ein Kandidat
   darf ohne Provenienzprüfung weder als Original- noch als GOOD-Image
   bezeichnet werden. Herkunft, Integrität und nachgewiesene Bootfähigkeit
   sind getrennt zu bewerten.
3. Das historische USB-Raw-Master ist nicht automatisch ein geeignetes
   externes SD-Bootimage. Sein dokumentierter Hybridzustand und seine
   Erhaltungsprovenienz ersetzen keine Qualifikation für den SD-/P_EXT-Pfad.
4. Zuerst eine vollständig eigenständige externe SD-Bootkette mit
   historischem SPL und dazu passendem historischem U-Boot proper bis zum
   historischen Debian-Referenzsystem nachweisen. Interne Bootloader oder
   lediglich einzelne erreichte Bootstufen erfüllen dieses Ziel nicht.
5. Danach einen vorhandenen geeigneten modernen Linux-Kernel verwenden
   und dessen Eignung in dieser Kette nachweisen; anschließend NixOS auf
   derselben externen SD verwenden und den vollständigen Boot nachweisen.
6. Abschließend den reproduzierbaren NixOS-Image-Builder für diese
   Gesamtkette qualifizieren. Historische Buildnachweise bleiben auf ihre
   jeweiligen Artefakte und Bedingungen beschränkt.
7. Modernes SPL/U-Boot v2026.07 und weitere DDR-/WL-/DQS-Diagnose vorerst
   zurückstellen. Auch der Mischboot aus historischem SPL und modernem
   Proper hat gegenüber dem passenden historischen Paar keinen Vorrang.
   Die offene DDR-Root-Cause wird dadurch nicht aufgelöst.

**SOURCE-18 wird eine rein lesende Provenienz-, Image- und
Bootvertragsprüfung historischer Debian-Referenzkandidaten.** Die frühere
unmittelbare Ausrichtung auf NixOS wird durch diese Referenzprüfung
präzisiert. Zu prüfen sind anhand vorhandener lokaler Belege:

- Kandidatenidentität, Herkunft, Hashbelege, historische Änderungen und
  Aussagegrenzen einer möglichen Original-/GOOD-Einstufung;
- Gesamtimage, Partitions- und Dateisystemlayout sowie die Abgrenzung von
  USB-Master, internem MMC-Image, Factory-Release und externem SD-Bootimage;
- konkrete historische SPL-/Proper-Identitäten und ihr zusammenpassender
  ROM-/USDHC-/FAT-Lade- und Übergabevertrag, Dateinamen und Speicherbereiche;
- Environment, Bootskript, historische Kernel-/initrd-/DTB-Artefakte,
  Formate, Größen, Lade-/Entpack-/Übergabeadressen und Root-Auswahl auf
  derselben externen SD ohne Abhängigkeit vom internen System.

Ergebnis soll eine belegte Kandidatenbewertung mit offenen Nachweisen sein.
Eine statische Eignungsbewertung ist kein Hardware-Bootnachweis. Fehlende
Primärquellen bleiben als Lücken benannt; keine automatische Beschaffung,
Reparatur oder Imageableitung. SOURCE-18 wird hier nur präzisiert, nicht
ausgeführt. Spätere Builds, Medienzugriffe und Hardwareversuche benötigen
jeweils eine konkrete Freigabe. Das lokale Raw-Master und erhaltene
Teilimages bleiben geschützt; kein Zugriff auf das physische Originalmedium.

**Protokollkorrektur zu NOVENA-STRATEGY-01:** Laut ausdrücklich bestätigter
Benutzerangabe wurde der Auftrag über den bestehenden Runner ausgeführt;
ein vollständiges TXT-Protokoll wurde erzeugt und über den GitHub-Wrapper
hochgeladen. Codex selbst hat keine zusätzlichen Runnerlogs angelegt und
keinen eigenständigen Upload gestartet. Die früheren pauschalen Aussagen
zu fehlendem Runner-/Upload-Aufruf beziehungsweise nicht behaupteter
Archivierung beschreiben den tatsächlichen Gesamtworkflow daher nicht
zutreffend und werden hiermit korrigiert. Runner-/Wrapperausführung und
zusätzliche Aktionen von Codex sind getrennte Sachverhalte.

- **NOVENA-STRATEGY-01: FAIL.** Codex-Dokumentation ausgeführt, HEAD
  unverändert, Index-SHA256 gegenüber der Baseline verändert, Runner Exit 41.
  Kein Commit und kein Push. Der FAIL bleibt erhalten.
- **NOVENA-STRATEGY-01-AUDIT: Runner PASS.** Index lesbar; der inhaltliche
  Indexvergleich mit dem ursprünglichen Snapshot ist noch nicht abgeschlossen.
  Daraus folgt keine Aussage, dass der Index unverändert sei, und keine
  nachträgliche Aufwertung des früheren FAIL.

Diese Ergebnisangaben und der Uploadstatus werden mit der bestätigten
Benutzerangabe als Evidenzbasis dokumentiert; keine erneute Remoteprüfung
und kein abgeschlossener Indexvergleich in NOVENA-STRATEGY-02.

Dieser Auftrag ergänzt ausschließlich DECISIONS, PROJECT-STATE und TODO.
Keine Git-Schreiboperationen, Builds, Hardwaretests, Image-Schreibvorgänge
oder Netzwerkaktionen; kein zusätzlicher Runner-/Upload-Aufruf durch Codex.
I2C3/ES8328 bleibt abgeschlossen; `regulator-always-on` bleibt dauerhaft.

## 2026-10-17 – SOURCE-18B: Einordnung des historischen SPL/MMC-Befunds

Dokumentationscheckpoint: `NOVENA-DOC-SOURCE18B-02`.

Die statische Untersuchung SOURCE-18B-11 bis SOURCE-18B-18
präzisiert die technische Evidenz zur historischen
SPL-/MMC-Bootkette.

Für das konkret untersuchte U-Boot-2020.07-SPL ist die
Novena-spezifische USDHC3-Initialisierung nachgewiesen.
Der generische CPU-MMC-Fallback verwendet eine
Controllerbasisadresse von Null.

Ein funktionierender USDHC2-Initialisierungspfad und
der vollständige autonome Boot über externe SD/P_EXT
sind damit nicht nachgewiesen.

Dies ist eine Evidenzeinordnung, keine neue
Bootloader- oder Hardwareentscheidung.

NOVENA-STRATEGY-02 bleibt unverändert maßgeblich:
Zunächst historische Debian-Referenzkandidaten und
eine passende historische SPL-/Proper-Bootkette
qualifizieren, anschließend modernen Kernel,
NixOS und reproduzierbaren Image-Builder.

Vorhandene DDR-/SPL-Forschungsergebnisse, frühere
Fehlerbefunde und die abgeschlossene I2C3/ES8328-
Root-Cause bleiben unverändert.


## 2026-10-17 – NOVENA-DEBIAN-DOC-01B: Evidenzeinordnung REF-09 bis REF-14

Dieser Nachtrag ordnet statische Evidenz ein und trifft **keine neue
Bootloaderentscheidung**. NOVENA-STRATEGY-02 und frühere Entscheidungen
bleiben unverändert. Die [historische Hauptdarstellung](HISTORICAL-MEDIA.md#2026-10-17--novena-debian-doc-01b-bootvertrag-der-debian-paketlinie-2015)
und das [Quellenregister](QUELLENVERZEICHNIS.md#2026-10-17--novena-debian-doc-01b-ref-09-bis-ref-14-quellenregister)
binden die Aussagen an Originalprotokolle und den exakten Referenzcommit
`bd5c7d3290bb94bf8af17760c321df67d2641343`.

Der 2015-Quellstand belegt einen externen USDHC2-SPL-Pfad und den
FAT-p1-/`u-boot.img`-Vertrag. Daraus folgt weder die Eignung des gesamten
USB-Hybrids noch eine vollständige P_EXT-Bootfähigkeit des extrahierten Paars.
Die 2014-Generationen und der 2020.07-SPL aus SOURCE-18B bleiben separat;
Quellunterstützung einer Linie qualifiziert kein anderes Binary.

Controllerwahl im SPL, MMC-Nummerierung in Proper, persistentes Environment
auf MMC 0, EEPROM-/Hook-Einflüsse, Nutzlastquelle und Rootquelle sind getrennte
Qualifikationsgegenstände. Insbesondere ist externe Bootquelle kein Nachweis
der Unabhängigkeit von interner MMC oder SATA. Der vorhandene Default-Fallback
beweist keinen Gesamtboot ohne internes System. Doppelte Defaultstrings
beweisen allein keinen effektiven Laufzeitwert.

REF-09 bis REF-14 dokumentieren statische Teilergebnisse mit Runner-PASS;
Integrität, Extraktionsherkunft, vollständige Buildprovenienz und Hardwarefunktion
bleiben getrennt. REF-08-FAIL bleibt trotz REF-08R-PASS erhalten. Fehlende
Kernel-/initrd-/DTB-, Environment-, Root- und P_EXT-Nachweise bleiben offen;
kein Original-/GOOD-Prädikat und keine neue Freigabe für Builds oder Versuche.
Moderne DDR-Ergebnisse werden nicht umgedeutet. I2C3/ES8328 bleibt geschlossen,
`regulator-always-on` bleibt dauerhafte Novena-Boardanforderung.

## 2026-10-18 – Historisches Debian Jessie 8.2 zuerst

Kennung: NOVENA-ENTSCHEIDUNG-2026-10-18-JESSIE-FIRST

Diese Entscheidung präzisiert und korrigiert den Nachtrag
NOVENA-ENTSCHEIDUNG-2026-10-18-DEBIAN-FIRST.

Die unmittelbare Entwicklungspriorität ist ein reproduzierbares,
bootfähiges historisches Debian-Jessie-8.2-SD-Image für die
Kosagi Novena, erstellt mit dem vorhandenen Rust-Assembler.

Debian 12 ist ein späterer, eigenständiger Entwicklungsschritt.
Die Rust-Integration des aktuellen NixOS-Image-Builders bleibt
ebenfalls zurückgestellt.

Die historische Referenzkonfiguration und ihre konkrete
Bootfähigkeit müssen anhand der vorhandenen Quellen qualifiziert
werden. Eine vollständige Jessie-8.2-Provenienz oder Bootfähigkeit
wird durch diese Prioritätsentscheidung nicht als bereits
nachgewiesen behauptet.

Frühere technische Befunde und abgeschlossene Untersuchungen
bleiben unverändert. Builds, Hardwaretests und SD-Schreibzugriffe
erfordern weiterhin eine gesonderte Freigabe.


## 2026-10-18 – NOVENA-JESSIE-20: experimenteller Assembliermodus

Der Referenzmodus bleibt Standard. Nur explizite Experimentwahl ersetzt die
Bootnachweissperre durch acht getrennte Beschreibungen offener Bedingungen.
Experimentangaben sind niemals Bootnachweise. Eigener Image-Dateiname und
Ergebnisformat v2 kennzeichnen das unqualifizierte Ergebnis. SHA256-,
Dateisystemerkennungs-, Layout-, SPL- und Rückprüfungen bleiben gemeinsam.

[Schnittstelle, Evidenzgrenzen und offene Risiken](DEBIAN-REFERENCE-IMAGE.md#2026-10-18--novena-jessie-20-experimenteller-erstboot-modus).


## 2026-10-18 – Universelles NixOS-SD-Image zuerst

Kennung: **NOVENA-ENTSCHEIDUNG-2026-10-18-NIXOS-FIRST**
Dokumentationsauftrag: NOVENA-DOC-NIXOS-FIRST-01.

**Beschluss:** Das universelle, reproduzierbare und eigenständig von externer
SD bootfähige NixOS-Image hat ab jetzt Entwicklungsvorrang. Dieser additive
Beschluss löst die Priorisierung DEBIAN-FIRST und JESSIE-FIRST ab. Deren
historische Aussagen und die zugrunde liegenden Arbeiten bleiben erhalten.

**Faktenbasis:** Debian Jessie 8.2 bootete erfolgreich von SD
(NOVENA-JESSIE-48); grafische Anmeldung und Abmeldung sind in
[NOVENA-JESSIE-BOOT-20261018](NOVENA-JESSIE-BOOT-20261018.md) dokumentiert.
NixOS 26.05 mit Linux 6.18.49 bootete erfolgreich von externer SD mit
historischem SPL/U-Boot 2015.04 (NOVENA-NIXOS-81/82;
[Bootnachweis](TEST-LOG.md#novena-nixos-82-hardware-boot-2026-10-18)).
Die erfolgreichen Hardwareboots begründen die Rückkehr zum NixOS-Hauptziel.

**Aussagegrenze:** Ein erfolgreicher Hardwareboot qualifiziert ausschließlich
den belegten Artefakt- und Teststand. Er belegt weder die Bootfähigkeit eines
neu erzeugten Rust-Images noch die Reproduzierbarkeit eines neuen Builders.
Jessie-Booterfolg und Qualifikation eines mit Rust erzeugten Jessie-Images
bleiben getrennte Meilensteine; Letztere wird hier nicht als erledigt erklärt.

Jessie bleibt historische, funktionierende Referenz. Rust-Assembler,
Debian-Nix-Integration und Jessie-Arbeiten bleiben erhalten; ihre weitere
Qualifikation ist nachgeordnet. Ein Rust-NixOS-Profil ist keine Voraussetzung
für die Fortsetzung des NixOS-Projekts. Die frühere Sperrfolge „erst Rust-Jessie
qualifizieren, danach NixOS“ gilt als Priorisierung nicht mehr.
NOVENA-UNIVERSAL-RUST-01 erhält dadurch keine Implementierungsfreigabe.

Moderne DDR-/SPL-Fragen und der vollständige Boot mit U-Boot 2026.07 bleiben
separat offen und vorerst zurückgestellt. Der historische Booterfolg löst
deren Root Cause nicht auf. Abgeschlossene BOOT-05–08-, Display- und
I2C3-/ES8328-Untersuchungen werden ohne neue diskriminierende Evidenz nicht
wiederholt; `regulator-always-on` bleibt dauerhafte Boardanforderung.

Evidenzbasis dieses Beschlusses sind die im Auftrag bestätigten Fakten und
die vorhandene Projektdokumentation. Die referenzierten Originalprotokolle
NOVENA-JESSIE-48 und NOVENA-NIXOS-81 wurden in diesem Auftrag nicht erneut
geprüft; sie liegen nicht im Repository vor. Keine neue Hardwarequalifikation.
Der Beschluss erteilt keine Build-, Nix-Evaluierungs-, Medien-, Hardware-,
Netzwerk- oder Git-Schreibfreigabe.


## 2026-10-21 – NOVENA-DDR-IMPLEMENT-168: quellentreue DDR-Reproduktion

Auf ausdrücklichen Implementierungsauftrag wird die zusammenhängende
Novena-DDR-Sequenz aus `bd5c7d3` boardlokal als Patch 0014 integriert.
0001–0008 bleiben aktiv; 0009–0013 werden ausschließlich aus der aktiven
Nix-Liste genommen und als historische Versuche bytegleich erhalten.
Keine Änderung der allgemeinen i.MX6-DDR-Datei, keine zweite DDR-Sequenz.

Bewusst erhalten: PHY1-MDMISC-Restore, fehlender WL-/PHY0-Latenzrestore,
historische MAPSR-Masken, WL-Softfail ohne moderne Lane-Korrektur,
Gatingkorrektur `0xc0`, beide aktiven Dummy Writes, Integer-Timingrechnung,
Chipbreite als IOMUX-Argument und unbegrenztes Polling. Das bestätigt
weder deren Hardwareeignung noch eine Root Cause. Notwendige Anpassungen:
moderne Includes/Typen und CRT-Rückkehr, explizite I2C1-Buswahl samt
Setupfehler-Fallback; bestehender externer USDHC2-/mmc1-Vertrag bleibt.

HISTORY-167R hat gegenüber früheren historischen Zwischeninterpretationen
Vorrang: bewusst begrenzte Mainline-Aufnahme, kein behaupteter SPD-Ausbau
2016. Historische Mainline-Fremderfolge qualifizieren nicht den neuen Port.
[Entscheidungsdetails und Abweichungsliste](NOVENA-DDR-IMPLEMENT-168.md).
Abnahme ausschließlich `PASS_SOURCE_ONLY`; Builds, Medienaktionen und
Hardwaretests bleiben getrennte, hier nicht erteilte Freigaben.
