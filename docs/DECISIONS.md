# Decisions

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
