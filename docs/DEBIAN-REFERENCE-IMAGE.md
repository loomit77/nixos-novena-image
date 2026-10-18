# Separates Debian-Referenzimage

Stand: 2026-10-18, fortgeschrieben mit NOVENA-DEBIAN-RUST-06.
Ursprünglicher Implementierungsauftrag: NOVENA-DEBIAN-IMPLEMENT-01.

`image/debian-reference-image.nix` ist ein separater Assembler im Sinne der
im Auftrag gewählten Variante C. Er importiert weder den NixOS-Builder noch
erzeugt er ein Debian-System. Der Wortlaut von NOVENA-DEBIAN-ARCHITEKTUR-01
liegt im Repository nicht vor; Grundlage sind die verbindlichen Anforderungen
dieses Implementierungsauftrags. Es gibt keine Flake-Integration und keine
automatische Auswahl historischer Dateien.

## Schnittstelle

Import als Funktion mit folgenden Argumenten; dies ist eine Beschreibung,
keine Aufforderung zur Evaluierung oder zum Build:

| Argument | Inhalt |
| --- | --- |
| `pkgs` | Explizit gepinnter Nixpkgs-Paketsatz des Aufrufers; Hostwerkzeuge aus `buildPackages` |
| `rootfs` | Separates historisches ext4-Partitionsimage, unveränderte Bytes |
| `firmware` | Vollständiges, separat vorbereitetes FAT-Partitionsimage inklusive qualifizierter Bootkonfiguration |
| `swap` | Separates Swap-Partitionsimage inklusive seiner explizit gewählten Identität |
| `bootloader2015` | `{ sourceRevision; spl; proper; }`, zusammengehöriges Paar der 2015-Paketlinie |
| `kernel`, `dtb` | Explizite historische Binärdateien; keine Ableitung aus aktuellem NixOS |
| `layout` | `{ diskId; firmwareStart; swapStart; rootStart; firmwareType; }` ohne Defaults |
| `bootContract` | Nachweisreferenzen für alle unten genannten Voraussetzungen; Default `null` sperrt den Builder |

Jedes Artefakt (`rootfs`, `firmware`, `swap`, `spl`, `proper`, `kernel`, `dtb`)
hat die Form `{ path; origin; size; sha256; }`: Store-Dateipfad, nichtleere
Herkunftsangabe, positive Byteanzahl und 64-stelliger kleingeschriebener
hexadezimaler SHA256. `origin` soll Extraktionsprotokoll, Quellartefakt mit Hash
und gegebenenfalls Bytebereich nennen. Die Eingaben müssen bereits separat
bereitgestellt sein; der Builder extrahiert nichts aus dem USB-Master und
lädt nichts herunter. Storepfad und deklarierter Hash ersetzen keine
ursprüngliche Paket-/Buildprovenienz. Der Aufrufer muss auch den Nixpkgs-Pin
und diese Builderrevision für spätere Reproduktion festhalten.

`sourceRevision` muss
`bd5c7d3290bb94bf8af17760c321df67d2641343` sein. Diese Bindung bezeichnet
die dokumentierte 2015-Referenzlinie, beweist aber keine Binärabstammung.
`pairProvenance` muss den gemeinsamen Paket-/Extraktionskontext belegen;
ein lediglich eingetragener Commit ist kein solcher Beleg.

`layout` verwendet 512-Byte-Sektoren, eine DOS-Disk-ID in der Schreibweise
`0x` plus acht Hexziffern und FAT-MBR-Typ `06`, `0b` oder `0c`.
Partition 1 ist FAT, 2 Swap (82), 3 ext4 (83). Die Partitionen übernehmen
exakt die Größen ihrer Eingaben. Die Ausgabedatei endet hinter p3;
historischer unpartitionierter Nachlauf wird nicht kopiert. Es werden weder
aktive Partition noch BIOS-Bootcode ergänzt. Die Geometrie des USB-Hybrids
und die G2X-NixOS-Geometrie sind keine automatischen Defaults. FAT-Typ und
Filesystemvariante müssen im Layoutnachweis zusammen qualifiziert sein.

Der SPL wird bei Byteoffset `0x400` vor p1 eingebettet. Proper wird als
`u-boot.img` auf FAT erwartet; Kernel und DTB als `zImage` und `novena.dtb`.
Diese Dateien werden nur lesend extrahiert und mit den expliziten Eingaben
verglichen. Es werden keine Bootargumente, Ladeadressen, Skripte, UUIDs,
fstab-Einträge, initrds oder Recovery-Dateien erzeugt oder umgeschrieben.
Zusätzliche FAT-Dateien sind durch den Gesamt-FAT-Hash gebunden und müssen
im Bootvertrag berücksichtigt werden. Die drei Pflichtdateien allein
beweisen nicht, welcher Scan-/Hook-/Fallback-Pfad tatsächlich wirkt.

## Kontrollierte Abbrüche und Nachweise

Vor Erstellung der Derivation sperren Assertions ein fehlendes/falsches
2015-Paar, unvollständige Artefaktmetadaten und fehlende Nachweisreferenzen.
Nix selbst weist fehlende Pflichtargumente zurück. `bootContract` verlangt
für jeden dieser Schlüssel eine nichtleere Zeichenkette mit konkretem
Nachweisverweis, gebunden an die Eingabehashes und das gewählte Layout:

| Schlüssel | Zu klärende Voraussetzung |
| --- | --- |
| `pairProvenance` | Zusammengehörigkeit und Herkunft der konkreten SPL-/Proper-Dateien |
| `romSplProperContract` | IVT/Header, Formate, Lade-/Entryadressen, Speicherbereiche und Übergänge des konkreten Paars |
| `externalMmcEnvironment` | Proper-Gerätenummerierung und Verhalten ohne interne MMC; persistentes Environment auf MMC 0 |
| `eepromAndHooks` | EEPROM/SATA-Einfluss, Environment, uEnv, earlyhook/finalhook und Recovery-Zustand |
| `bootFilesAndArguments` | Wirksamer FAT-Scan-/Fallback-Pfad und belegte vollständige Bootargumente |
| `kernelDtbInitrdContract` | Historischer Kernel/DTB, erforderliche oder ausdrücklich entbehrliche initrd, Formate und Speicherbelegung |
| `externalRootAndFstab` | Root auf derselben externen SD, Treiberverfügbarkeit, unveränderte historische fstab für Root/Boot/Swap |
| `partitionIdentityAndCollisions` | Gewählte Geometrie, FAT-Typ, Disk-ID/PARTUUIDs und Dateisystemidentitäten ohne relevante Kollisionen |

Die Schnittstelle prüft Vorhandensein der Referenzen, nicht deren fachliche
Wahrheit. Platzhalter wie „offen“ oder bloße Zusicherungen sind keine gültige
Qualifikation und dürfen nicht zur Freischaltung verwendet werden. Solange
eine Voraussetzung ungeklärt ist, bleibt `bootContract = null` beziehungsweise
der entsprechende Schlüssel fehlt: kontrollierter Abbruch statt Annahme.

Im später gesondert freizugebenden Build führen fehlende reguläre Dateien,
abweichende Größen/Hashes, falsche Dateisystemtypen, fehlende/abweichende
FAT-Pflichtdateien, ungültiges Layout, fehlende Sektorausrichtung,
Überlappungen einschließlich SPL/FAT und Überschreitung der DOS-Sektorgrenze
zum Abbruch. Werkzeugfehler werden nicht ignoriert. Diese Vorprüfungen
erfolgen vor Anlegen der Ausgabe. Fehler bei der anschließenden Kopie oder
Hash-Rückprüfung lassen die Derivation fehlschlagen.

Ausgabe ist ein Verzeichnis mit `debian-reference.img` und `manifest.json`.
Das Ergebnismanifest v1 enthält `resultSchemaVersion: 1`, `assemblerVersion`,
`manifest` (das normalisierte Eingabemanifest v1 einschließlich Metadaten,
Layout und Nachweisreferenzen) und `output: { size, sha256 }`. Gegenüber dem
früheren Python-Ergebnis liegen die Eingabefelder jetzt unter `manifest`;
Verbraucher müssen diese Strukturänderung berücksichtigen.
Die eingebetteten Bereiche werden gegen ihre Eingabehashes zurückgeprüft.
Das ext4 wird nicht gemountet, repariert, verkleinert oder erweitert; auch
Swap und FAT werden bytegetreu kopiert. Kein Blockgerät wird geöffnet.
Die Byteerhaltung betrifft die Konstruktion; ein später laufendes Debian
kann sein Rootfs selbstverständlich verändern.

## Evidenz und offene Voraussetzungen

Maßgeblich sind [HISTORICAL-MEDIA.md](HISTORICAL-MEDIA.md), insbesondere
der Nachtrag vom 17.10.2026 zur 2015-Linie, sowie das dort verlinkte
[Quellenregister](QUELLENVERZEICHNIS.md). Der rohe rc12-SPL, die rc14-FAT-Dateien
und das 2015-Rootfs-Paketpaar sind getrennte Generationen. Der Master ist
ein Hybrid mit NovS-Disk-ID und MMC-fstab; er wird nicht als fertige externe
SD-Vorlage behandelt. [IMAGE-LAYOUT.md](IMAGE-LAYOUT.md) beschreibt daneben
historische NixOS-G2X-Artefakte, deren Bootparameter hier nicht übernommen werden.

Der jüngere [Projektstand](PROJECT-STATE.md) vom 18.10.2026 schließt den
historischen BOOT-05–08-Vergleich ab. Dessen statisches PASS der
2014-Adressberechnung qualifiziert weder das hier geforderte 2015-Paar noch
den vollständigen externen Boot. Diese Untersuchung wird nicht wiederholt.

Für diesen Builder sind noch kein vollständiger eingabegebundener Bootvertrag,
keine geeignete FAT-/Swap-Eingabekombination und keine externe Root-/fstab-
Qualifikation hinterlegt. Insbesondere bleiben interne MMC-Abhängigkeit,
Environment/EEPROM/Hooks, effektives `root=`, initrd-Bedarf und konkrete
Kernel-/DTB-Übergabe offen. Deshalb gibt es keine freigeschaltete
Beispielkonfiguration mit erfundenen Werten und keine Bootfähigkeitsbehauptung.

## Historischer Prüfstand NOVENA-DEBIAN-IMPLEMENT-01

Die folgenden Angaben beschreiben ausschließlich den ursprünglichen Auftrag;
der aktuelle Integrationsstand folgt im datierten Nachtrag.


Im Implementierungsauftrag sind ausschließlich diese Dokumentation und der
separate Builder angelegt. Kein NixOS-Builder, Masterimage oder Git-Index
wird geändert. Keine Nix-Evaluierung, kein Build, Funktionstest,
Hardwareversuch oder SD-Schreiben gehört zu diesem Auftrag.
Zusätzliche Runner-/Sidecar-Dateien sind von der Zwei-Dateien-Freigabe nicht
umfasst; die Prüfausgaben werden in der Auftragsantwort berichtet.
Eine statische Prüfung kann Python-/Shellsyntax und Diff prüfen; ohne
verfügbaren separaten Nix-Parser bleibt die Nix-Syntax manuell geprüft.
Sie belegt weder einen erfolgreichen Build noch Bootfähigkeit.

## 2026-10-18 – NOVENA-DEBIAN-RUST-06: Rust-/Nix-Integration

Die bisher eingebettete Python-Assemblierung ist vollständig durch den
unveränderten Rust-Assembler aus `tools/novena-image-assembler/` ersetzt.
Die Schnittstellenprüfung anhand sämtlicher Rust-Quelldateien, des Schemas
und des bisherigen Builders zeigt denselben Layout- und Eingabevertrag:
MBR, SPL bei Byte 1024, FAT/Swap/ext4 als p1–p3 und `u-boot.img` auf FAT.
Die Argumente der Nix-Funktion und die acht Nachweissperren bleiben erhalten.
Es gibt keinen Python-Fallback. Der Rust-Parser weist unbekannte oder doppelte
JSON-Felder zurück; auch optionale Zusatzfelder in Artefakten, Layout oder
Bootvertrag sind daher nicht zulässig. Eingabepfade werden explizit als
Strings unter Erhaltung ihres Nix-Abhängigkeitskontexts serialisiert.
Finale Symlinks werden von Rust abgewiesen: direkte reguläre Store-Dateien
oder reguläre Dateien innerhalb einer Store-Ausgabe verwenden.

### Buildarchitektur und Pin

Der Aufrufer muss `pkgs` aus dem bereits in `flake.lock` festgelegten
Nixpkgs-Input verwenden (Revision
`a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4`). Die Funktion behält bewusst
`pkgs` als explizites Argument und lädt selbst keinen Paketsatz nach;
sie erzwingt dessen Revision nicht technisch. Ein abweichender Paketsatz
liegt außerhalb dieser Pin-/Reproduktionsbeschreibung. Keine neuen Inputs,
keine Änderung an `flake.nix` oder `flake.lock`, kein neues Flake-Buildziel.
Auch die bereits vorhandene lokale Änderung an `flake.nix` bleibt erhalten.

`pkgs.buildPackages.rustPlatform.buildRustPackage` baut ein natives
Hostprogramm; der Image-RunCommand stammt ebenfalls aus `buildPackages`.
Eine Assertion begrenzt den Host auf `x86_64-linux`, entsprechend den
Linux-UAPI-Konstanten im Rust-Code. Compiler, Cargo, stdenv, `blkid`
(util-linux) und `mcopy` (mtools) stammen aus demselben Paketsatz. Es wird
kein ARM-Assemblerbinary und kein Debian-System gebaut.

Der Quellbaum wird mit `lib.fileset.toSource` ausschließlich aus
`Cargo.toml`, `Cargo.lock` und `src/` gebildet. Vorhandene unversionierte
Dateien, insbesondere `target/`, bleiben liegen und fließen nicht ein.
`cargoLock.lockFile` bindet die vorhandene Lockdatei; Nix importiert die dort
festgelegten Crates mit ihren Checksummen als Vendor-Abhängigkeiten.
Alle derzeitigen externen Einträge sind Registry-Crates mit SHA256;
Git-Abhängigkeiten oder ungehashte Nachauflösung werden nicht freigeschaltet.
`cargoBuildFlags = [ "--frozen" ]` verbietet Lockänderungen und Cargo-Netzwerk;
fehlende Vendor-Abhängigkeiten oder Lock-Inkonsistenzen müssen scheitern.
Nix darf bei einem später ausdrücklich freigegebenen Build fehlende,
hashgebundene Crates oder gepinnte Werkzeuge beschaffen. Das ist von einer
Cargo-Netzwerkauflösung zu unterscheiden; hier erfolgte keine Beschaffung.

`doCheck = false` deaktiviert die automatische Cargo-Testphase dieses
Bausteins. Vorhandene Rust-Tests werden weder geändert noch wiederholt;
die Nix-Integration und Dateisystemhelfer benötigen eigene Qualifikation.

### Manifest, Werkzeuge und Ausgabe

`builtins.toJSON` erzeugt das Eingabemanifest mit den festen Feldern
`schemaVersion = 1`, `profile = "novena-debian-2015"`, `sectorSize = 512`
sowie `sourceRevision`, den sieben Artefakten, `layout` und `bootContract`.
`passAsFile` stellt es als reguläre Builddatei bereit. Der einzige
Assembleraufruf ist der absolute Store-Binarypfad mit den Argumenten
`"$manifestPath" "$out"`. Der Assembler erwartet ein noch nicht vorhandenes
Ausgabeverzeichnis und erzeugt dort ausschließlich Image und Ergebnismanifest.
Die feste Ausgabepfadübergabe vermeidet Pfadinterpolation aus historischen
Metadaten; der Ausgabepfad wird nicht in das Ergebnismanifest aufgenommen.

Vor dem Aufruf wird `PATH` vollständig auf die Nix-Store-bin-Verzeichnisse
von util-linux und mtools gesetzt. Dadurch finden die namensbasierten
Rust-Helferaufrufe keine beliebigen Hostprogramme. Der Rust-Code bereinigt
deren Umgebung zusätzlich und setzt `LC_ALL=C` sowie `MTOOLSRC=/dev/null`.
`/proc/self/fd` muss in der späteren Linux-Buildumgebung verfügbar sein.
Kein Mount, kein sfdisk/dd und keine Dateisystem-Neuerstellung sind nötig.

Rust prüft Größen und SHA256 aller Eingaben, Dateisystemtypen und die drei
FAT-Pflichtdateien, bevor `$out` entsteht. Er kopiert mit begrenzten Puffern,
prüft Eingaben erneut, liest MBR und eingebettete Bereiche zurück und vergleicht
den gesamten Imagehash einschließlich Nullbereichen mit dem Erwartungshash.
Fehler propagieren als Derivationsfehler; eine Teilausgabe gilt nicht als
Erfolg. Diese Rückprüfung ist eine Prüfung einer regulären Builddatei und
kein Medienreadback oder Hardwaretest.

Gleiche Quellstände, Lockdatei, Pin, Eingabebytes, Metadaten und Layout sollen
bei gleicher Werkzeugkette dieselben Image- und Ergebnismanifestbytes ergeben.
Das Ergebnis enthält keine automatisch erzeugten Zeitstempel oder Hostnamen;
geänderte Store-Eingabepfade oder Herkunftstexte ändern bewusst das Manifest.
Die tatsächliche Reproduzierbarkeit des Nix-Binaries und des integrierten
Images ist damit entworfen, aber noch nicht praktisch nachgewiesen.

### Prüfung und offene Grenzen dieses Auftrags

Ausschließlich statische Durchsicht und lesende Struktur-/Syntaxprüfungen;
keine Nix-Evaluierung, Nix-/Cargo-/Image-Builds oder Testausführung.
Die frühere Rust-Qualifikation (Rust 1.95.0 über Nix, fmt PASS,
`cargo test --locked` 6/6 PASS, clippy mit `-D warnings` PASS) ist die vom
Auftraggeber vorgegebene Ausgangslage, kein in RUST-06 erneut erhobener Befund.
Zugehörige Rohlogs wurden unter `test-logs/` und `evidence/` nicht gefunden.
Die früheren README-Angaben „noch nicht ausgeführt“ sind insoweit historisch.

Offen bleiben eine spätere Evaluierung mit dem tatsächlichen Pin, der
Nix-Hostbinary-Build, die echte `blkid`-/`mcopy`-FD-/stdout-Integration in der
Nix-Sandbox und ein vollständiger Image-/Reproduktionsbuild. Die vorhandenen
sechs Quelltests umgehen die Dateisystemhelfer; ihr PASS ersetzt diese
Integrationsnachweise nicht. Nix-Syntax bleibt ohne ausgeführten separaten
Nix-Parser manuell geprüft. Die unversionierten Builder-/Assemblerdateien
sind für Git-basierte Flake-Quellfilter noch nicht automatisch verfügbar;
dieser Auftrag verändert den Index nicht. Ein späterer Aufrufer muss einen
vollständigen, eindeutig identifizierten Quellbaum bereitstellen.

Die fachlichen Bootvertragslücken aus dem vorigen Abschnitt bestehen weiter.
Keine Aussage über erfolgreiche Image-Erstellung oder Bootfähigkeit.
Geändert werden nur dieser Text, der separate Debian-Builder und dessen
Assembler-README. NixOS-Builder, historische Originale und Git-Index bleiben
unberührt. Runner-/Sidecar-Dateien außerhalb dieser Freigabe werden nicht
angelegt; es erfolgt kein Upload.
