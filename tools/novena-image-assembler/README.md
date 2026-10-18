# Novena Debian Image Assembler

Stand 2026-10-18, NOVENA-DEBIAN-RUST-06; ursprünglicher Quellauftrag
NOVENA-DEBIAN-RUST-02. Laut vorgegebener Ausgangslage wurden Rust 1.95.0
über Nix, cargo fmt, sechs Tests mit `--locked` und clippy mit `-D warnings`
bereits erfolgreich qualifiziert. Diese Angaben wurden hier nicht erneut
praktisch geprüft; zugehörige Rohlogs wurden unter `test-logs/` und `evidence/` nicht gefunden.
Die Nix-Integration ist ausschließlich statisch geprüft. Kein Imagebuild,
Medienzugriff oder Hardwaretest in diesem Auftrag.

Rust übernimmt gemäß NOVENA-DEBIAN-RUST-01 die Byte-Assemblierung; Nix bleibt
für die Buildsteuerung zuständig. Der Wortlaut dieser Entscheidung
wird hier aus dem Auftrag übernommen. Die Anbindung liegt ausschließlich im separaten
`image/debian-reference-image.nix`; der NixOS-Builder bleibt unverändert. Dieses Werkzeug baut kein Debian-System,
extrahiert nichts aus dem USB-Master und erzeugt keine Bootparameter.

## Architektur und Aufruf

Die CLI lautet `novena-image-assembler MANIFEST OUTPUT_DIR`.
Genau zwei Argumente, Fehlermeldung auf stderr und Exitstatus 1 bei Fehlern.
`OUTPUT_DIR` darf noch nicht existieren; sein Elternverzeichnis muss existieren.
Keine automatische Verzeichnisbereinigung und kein Überschreiben bestehender
Dateien. Bei Fehlern nach dem Anlegen bleibt eine unvollständige Ausgabe zur
Untersuchung erhalten. Nur Exitstatus 0 bestätigt die vollständige Durchführung;
eine vorhandene `manifest.json` allein ist kein Erfolgssignal (z. B. Syncfehler).

Die Bibliothek besteht aus `manifest.rs` (Parser und Metadaten), `layout.rs`
(geprüfte Bytebereiche), `mbr.rs` (explizite MBR-Bytes) und `assemble.rs`
(Preflight, Kopieren, Rücklesen und Ergebnismanifest).

Zielhost ist Linux x86_64 wie foobox. Die `O_NOFOLLOW`-/`O_PATH`-Konstanten
sind dafür explizit angegeben; andere Hosts sind durch `compile_error!` gesperrt.
Rust-Abhängigkeiten: `serde` 1 mit derive, `serde_json` 1 und `sha2` 0.10.
Das sind Versionsanforderungen, keine aufgelösten oder geprüften Versionen.
Laufzeitwerkzeuge: `blkid` aus util-linux und `mcopy` aus mtools sowie Linux
`/proc/self/fd`. Kein Shellaufruf, kein Mount, kein mkfs, fsck oder resize.

**Cargo.lock fehlt ausdrücklich.** Cargo, rustc und rustfmt wurden auf PATH
nicht gefunden; auch die üblichen lokalen Cargo-Registry-Index-/Cachepfade
wurden nicht gefunden. Eine belastbare Offline-Auflösung ist damit nicht
belegt. Es wurde kein Cargo-Aufruf, Build oder Netzwerkzugriff versucht.
Versionen transitiver Pakete und Registry-Prüfsummen werden nicht erfunden.
Ein später freigegebener Lockfile-/Buildschritt muss die Auflösung und
Nix-Pins qualifizieren, bevor reproduzierbare Builds behauptet werden können.

## Eingabeschema v1

`manifest-v1.schema.json` beschreibt die Datenform (JSON Schema 2020-12).
Die Rust-Validierung ist maßgeblich und prüft zusätzlich Duplikate,
Dateien, bereichsübergreifende Bedingungen und Hashes. Sie lädt das Schema
nicht zur Laufzeit und benötigt dafür weder Netz noch weiteren Crate.
Das Manifest darf höchstens 1 MiB umfassen; der JSON-Parser begrenzt auch
die Verschachtelung. Jede Objektebene weist unbekannte und doppelte Schlüssel
zurück, auch nach JSON-Unescaping. Ganzzahlfelder akzeptieren keine Strings,
Booleans, negativen Werte oder Fließkommaschreibweisen wie `1.0`/`1e0`.
JSON Schema betrachtet mathematisch ganzzahlige Fließkommazahlen dagegen
als Integer; die CLI ist hier absichtlich strenger. Nichtleere Texte werden
in Rust mit Unicode-Whitespace-Prüfung verlangt.

Alle nachstehenden Felder sind Pflichtfelder, ohne Defaults:

| Feld | Inhalt |
| --- | --- |
| `schemaVersion` | Integer `1` |
| `profile` | `novena-debian-2015` |
| `sectorSize` | Integer `512` |
| `sourceRevision` | `bd5c7d3290bb94bf8af17760c321df67d2641343` |
| `inputs` | Genau `spl`, `proper`, `firmware`, `swap`, `rootfs`, `kernel`, `dtb` |
| `layout` | Genau `diskId`, `firmwareStart`, `swapStart`, `rootStart`, `firmwareType` |
| `bootContract` | Genau die acht unten erläuterten Nachweisreferenzen |

Jedes Eingabeartefakt ist ein Objekt mit genau `path` (absoluter Pfad ohne
NUL), `origin` (nichtleere Provenienzreferenz), `size` (positive Byteanzahl,
u64) und `sha256` (64 kleine Hexziffern). Relative Pfade sind nicht vom
Arbeitsverzeichnis abhängig interpretierbar, sondern werden zurückgewiesen.
Die Eingaben sind bereits separat bereitgestellte reguläre Dateien.
Finale Symlinks, Verzeichnisse, FIFOs und Geräte werden abgewiesen.
Nix muss später direkte Store-Dateipfade liefern, keine Result-Symlinks.

`diskId` ist `0x` plus acht Hexziffern, ungleich null, ohne automatische
NovM-/NovS-Wahl. Die drei Starts sind positive Sektornummern im u32-Bereich.
`firmwareType` ist explizit `06`, `0b` oder `0c`; die Eignung für die konkrete
FAT-Variante gehört in den Layoutnachweis. Die Größen werden exakt aus den
Partitionsartefakten übernommen und müssen Vielfache von 512 sein.
Kein Aufrunden, keine Erweiterung und kein Nachlauf hinter p3.

Die acht nichtleeren Strings in `bootContract` sind konkrete, an Eingabehashes
und Layout gebundene Nachweisreferenzen:

| Schlüssel | Nachweisgegenstand |
| --- | --- |
| `pairProvenance` | Zusammengehöriges konkretes SPL-/Proper-Paar der 2015-Linie |
| `romSplProperContract` | Header, Formate, Lade-/Entryadressen, Speicherbereiche, Übergänge |
| `externalMmcEnvironment` | Proper-MMC-Nummerierung, Environment, Unabhängigkeit von interner MMC |
| `eepromAndHooks` | EEPROM/SATA, persistente Werte, uEnv, Hooks und Recovery |
| `bootFilesAndArguments` | Effektiver Scan-/Fallback-Pfad und vollständig belegte Bootargumente |
| `kernelDtbInitrdContract` | Kernel/DTB, initrd-Bedarf bzw. belegte Entbehrlichkeit, Übergabe |
| `externalRootAndFstab` | Externes Rootfs auf demselben Medium, Treiber und historische fstab |
| `partitionIdentityAndCollisions` | Geometrie, FAT-Typ, Disk-ID/PARTUUIDs, Dateisystemidentitäten |

Die Software prüft die Existenz nichtleerer Referenztexte, nicht deren
fachliche Wahrheit oder Erreichbarkeit. Sie löst keine Referenzen auf.
Platzhalter wie „offen“ dürfen nicht zur fachlichen Freischaltung verwendet
werden; bei offenen Voraussetzungen muss der Schlüssel fehlen, was sperrt.
Der Committext allein beweist keine Binärprovenienz. Deshalb gibt es kein
mit erfundenen historischen Werten ausgefülltes Ausführungsmanifest.
Synthetische Werte in Tests bezeichnen ausschließlich künstliche Fixtures.

## Prüfungen und Abbruchbedingungen

Vor jeder Ausgabe werden Manifest, Layout und alle sieben Eingaben geprüft:
regulärer Dateityp vor und nach read-only Open, exakte Größe und vollständiger
SHA256. Zuerst wird der Inode mittels `O_PATH | O_NOFOLLOW` ohne Geräte-I/O
fixiert und per fstat geprüft. Erst der reguläre Inode wird über seinen
`/proc/self/fd`-Pfad lesend geöffnet; ein Austausch des ursprünglichen
Dateinamens kann dadurch kein Blockgerät zum lesenden Open unterschieben.
Die offenen Handles bleiben erhalten. SPL liegt ausschließlich bei
Byteoffset 1024 und muss vor FAT enden. FAT, Swap und ext4 stehen geordnet,
ohne Überschneidungen als Partitionen 1–3. Alle variablen Bereichssummen,
Produkte und Zähler sind checked; MBR-Indexarithmetik ist durch drei feste
Einträge begrenzt. MBR-LBA und Längen sind u32; konservativ darf auch die
Gesamtsektorzahl nicht größer als `0xffffffff` sein.

`blkid -p -s TYPE -o value` muss `vfat`, `swap`, `ext4` liefern. Dies ist
Dateisystemerkennung, keine Konsistenzprüfung oder Reparatur. `mcopy` liest
`::/u-boot.img`, `::/zImage` und `::/novena.dtb` nach stdout; Größe und SHA256
müssen Proper, Kernel und DTB entsprechen. FAT-Namensvergleich folgt der
mtools-/FAT-Semantik, nicht einer behaupteten Groß-/Kleinschreibungsänderung.
Zusätzliche FAT-Inhalte sind durch den gesamten FAT-Hash gebunden; ihre
Semantik gehört in den Bootvertrag. Die Helfer bekommen das schon geöffnete
read-only Artefakt über stdin und `/proc/self/fd/0`. Fehler, fehlende Programme,
fehlende Pflichtdateien und zu große Helferausgaben führen zum Abbruch.
Die Helfer werden beim Streamingfehler beendet und eingesammelt.
Nach den Helfern werden alle Eingaben erneut gehasht.

Die Ausgabe enthält `debian-reference.img` und `manifest.json`. MBR und SPL
werden explizit geschrieben; die Partitionen werden bytegetreu übertragen.
Der MBR hat drei Einträge mit Typ FAT/82/83, Null-Bootflags, saturierten
CHS-Feldern, leerem vierten Eintrag, expliziter Little-Endian-Disk-ID und
55aa-Signatur. BIOS-Code wird nicht hinzugefügt. Lücken sind logisch null
(sparse Dateibereiche sind möglich). Kopier- und Hashpuffer sind auf 64 KiB
begrenzt, Manifest und kleine blkid-Ausgabe separat begrenzt.

Beim Kopieren werden die Quellbytes erneut gegen den Eingabehash geprüft.
Danach: sync, exakte Gesamtgröße, vollständige MBR-Bytes, alle vier Zielbereiche
und vollständiger Imagehash gegen den unabhängig aus MBR, Quellbytes und
Nullbereichen gebildeten Erwartungshash. Alle sieben Quellen werden vor dem
Ergebnismanifest nochmals geprüft. I/O-, Hash- und Syncfehler werden propagiert.

Das Ergebnismanifest hat `resultSchemaVersion: 1`, `assemblerVersion`, das
normalisierte Eingabemanifest und `output: {size, sha256}`. Feste Struct-
Feldreihenfolge, UTF-8, Pretty-JSON plus ein abschließendes LF; keine automatisch
erzeugten Zeitstempel, Hostnamen oder Ausgabe-/Arbeitsverzeichnispfade.
Gleiche Eingaben und Metadaten ergeben bei gleicher Werkzeugversion dieselben
Imagebytes und Ergebnismanifestbytes. Geänderte Eingabepfade/Provenienztexte
ändern das Ergebnismanifest bewusst. Der Rust-Code allein pinnt noch keine
Compiler-, Crate- oder Hostwerkzeugversionen.

## Synthetische Tests und statischer Prüfstand

`src/assemble/tests.rs` enthält Quelltests für vollständige MBR-Bytes,
SPL-/DOS-/u64-Grenzen, Überschneidungen, Ausrichtung, ungültige und doppelte
JSON-Schlüssel, fehlenden Bootvertrag, Hash-/Größenfehler, reguläre Dateien,
Byteerhaltung, Nullbereiche, Korruption beim Rücklesen und Determinismus.
Sie verwenden ausschließlich selbst erzeugte temporäre Bytefixtures.
Die Tests für den Assemblierungskern umgehen gezielt die externen
Dateisystemhelfer; die Fixtures sind keine echten FAT-/Swap-/ext4-Images.
Im ursprünglichen RUST-02-Auftrag lagen die Tests nur als Quellcode vor.
Der oben genannte spätere PASS ist vorgegebene Ausgangslage von RUST-06;
hier wurden keine Tests ausgeführt.

Im ursprünglichen RUST-02-Auftrag wurden TOML-/JSON-Lesbarkeit, lokale
Schema-Referenzen, Dateiumfang, Whitespace und der Quellcode hinsichtlich
Grenzen, Fehlerpfaden und Schreibzielen statisch geprüft. Die damalige
Syntax-/Typprüfung und Ausführung waren offen; der spätere Rust-Prüfstand
ist am Dateianfang eingeordnet. RUST-06 fügt ausschließlich statische
Integrationsprüfungen hinzu, keinen erneuten Build- oder Test-PASS.

## Offene Punkte und Risiken

- Cargo.lock ist vorhanden und wird von Nix verbindlich importiert. Die
  frühere Rust-Qualifikation ist oben eingeordnet; die Reproduzierbarkeit
  des über diese neue Derivation gebauten Binaries ist noch offen.
- Echte synthetische FAT-/Swap-/ext4-Fixtures und Integrationstests der
  `blkid`-/`mcopy`-Aufrufe fehlen. Insbesondere stdout- und FD-Übergabe sind
  implementiert, aber nicht auf dem Zielhost ausgeführt worden.
- Nix-Anbindung und gepinnte Hostwerkzeuge sind in RUST-06 implementiert;
  die Python-Assemblerlogik im Debian-Builder ist entfernt. Evaluierung und
  praktischer Integrationsbuild stehen aus.
- Vertrauenswürdiges PATH sowie unveränderliche Quellen und vertrauenswürdige
  Eltern-/Ausgabeverzeichnisse sind vorausgesetzt. Es gibt keine Sandbox für
  externe Programme, kein openat-Verzeichnis-Handle-Modell, keine Dateisperren
  und keinen Snapshot gegen gleichzeitig mutierende Dateien. Finale Symlinks
  werden gesperrt, aber Zwischenverzeichnisse nicht gegen Austausch abgesichert.
  Wiederholtes Hashen reduziert das Mutationsrisiko, beseitigt es nicht.
- Keine Ressourcenquote oder Prozess-Timeouts für externe Parser; fehlerhafte
  Dateisysteme können Helferfehler oder lange Laufzeiten verursachen. Große
  Images werden mehrfach gestreamt; RAM bleibt begrenzt, I/O-Aufwand ist hoch.
- Keine Mounts, Reparaturen, Masterextraktion, Dateienerzeugung auf FAT oder
  in Rootfs, Firmwareänderung, Hardwareboots oder Blockgeräteschreibfunktion.
- Ein Bootvertrag wird strukturell verlangt, aber seine Fachqualifikation
  bleibt separat. Header-, Speicher-, Environment-, initrd-, Root-/fstab-
  und P_EXT-Gesamtbootnachweise werden nicht durch dieses Werkzeug erbracht.

Quellen: die Dateien `image/debian-reference-image.nix`,
`docs/DEBIAN-REFERENCE-IMAGE.md`, `docs/HISTORICAL-MEDIA.md` (besonders Nachtrag
vom 17.10.2026), `docs/PROJECT-STATE.md`, `docs/DECISIONS.md` und
`docs/TEST-LOG.md`. Der historische USB-Hybrid, rc12/rc14-Dateien, das
2015-Paketpaar und das nicht wiedergefundene Factory-R1-Image bleiben
getrennte Gegenstände. Es wird keine Bootfähigkeit zugesagt.

## 2026-10-18 – Nix-Anbindung (NOVENA-DEBIAN-RUST-06)

Der separate Debian-Builder baut das Hostbinary mit
`pkgs.buildPackages.rustPlatform.buildRustPackage`. `cargoLock.lockFile`
verwendet diese Cargo.lock; Registry-Crates werden von Nix mit ihren
Lockfile-Checksummen bereitgestellt. Cargo läuft mit `--frozen`: keine
Lockänderung, keine Online-Auflösung. Nix-Beschaffung fehlender gehashter
Abhängigkeiten bei einem später genehmigten Build bleibt davon getrennt.
Automatische Tests sind mit `doCheck = false` deaktiviert.

Der Aufrufer liefert `pkgs` aus dem vorhandenen Flake-Lock-Pin; es gibt
keinen neuen Input und keine technische Prüfung dieser Revision durch die
Funktion. Das Binary und der Image-RunCommand werden nativ für den
x86_64-Linux-Buildhost abgeleitet. Die Quellmenge enthält nur Cargo.toml,
Cargo.lock und src/, niemals das vorhandene target/ oder andere lokale
Arbeitsartefakte. Sie werden dabei weder gelöscht noch verändert.

Nix serialisiert das strikte Manifest v1 (einschließlich expliziter
Store-Dateipfade, Größen, SHA256, Layout und Bootvertragsreferenzen) mit
`builtins.toJSON` und übergibt es mittels `passAsFile`. Der absolute
Binarypfad erhält die Manifestdatei und das noch nicht vorhandene `$out`.
`PATH` wird ausschließlich aus den Nix-Paketen util-linux und mtools gesetzt;
kein Hostwerkzeug- oder Python-Fallback. Die Linux-Sandbox benötigt `/proc`.
Ausgabe: `$out/debian-reference.img` und `$out/manifest.json` mit
`resultSchemaVersion: 1`, `assemblerVersion`, `manifest` und
`output: {size, sha256}`. Die Eingabefelder stehen gegenüber dem früheren
Python-Ergebnis nun unter `manifest`.

Schnittstellenprüfung, Pin-/Lock-Bindung, Reproduktionsbedingungen,
Quellfiltergrenzen bei unversionierten Dateien und offene Build-/Bootnachweise
sind in [DEBIAN-REFERENCE-IMAGE.md](../../docs/DEBIAN-REFERENCE-IMAGE.md)
beschrieben. Keine Nix-Evaluierung, Builds oder Tests wurden in RUST-06
ausgeführt. Die Nix-Integration und echte FAT-/Swap-/ext4-Helferaufrufe bleiben
praktisch unqualifiziert; vorhandene Kern-Tests ersetzen das nicht.
