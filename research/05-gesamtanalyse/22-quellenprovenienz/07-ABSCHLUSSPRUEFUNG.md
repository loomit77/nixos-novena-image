Stand: 2026-10-03 · BLOCK 9C-46E · Forschungsauftrag ohne Implementierung.

# Abschlussprüfung und dauerhaftes Ausgangsmanifest

## Erreichter Umfang

**BESTAETIGT:** Inhaltliche unabhängige Prüfung der NXP-PDF-Originale N01 (Errata), N02 (Consumer Data Sheet), N07 (EB817), N08 (Stress-Tester-Anleitung) und N09 (EB803) über den normalen Webzugang. Zusätzlich Originalinhalte K02 (DDR-Notizen), K04 (DVT-Issue-Log), U04 (Reviewmail) und K01; vorhandene Hardware-/Weboriginale K03/K05–K11/U05/U06 lokal, vorhandene Software-/Gitoriginale U01–U03 sowie U07-Metadaten lokal. „Unabhängig“ bedeutet hier direktes Lesen des Originalinhalts gegenüber früheren Forschungszusammenfassungen, nicht ein erneutes physikalisches Experiment.

**TEILWEISE_BESTAETIGT:** Herstellerabgleich der Kalibrierungsdiagnose und historische Funktions-/Bestückungsberichte. Eine Originalinhaltsausgabe ist kein lokales Bytearchiv; Entwicklerlogs sind keine vollständig rekonstruierte Testkampagne.

**NICHT_VERIFIZIERT:** N04/N05/N06 als Dokumentinhalte, Herstellersemantik der Registeradressen/Zugriffsfolgen, vollständiger U07-Diff/v2016.03 und reale heutige DIMM-/Silicon-/Boardidentität. Keine vermeintlich vollständige Schließung dieser Lücken.

**WIDERSPRUCH:** Kommentar/Code 0x80/0xc0, N08-URL-Version/interne Version, K03-ECO-/Footer-Chronologie. Befunde und Grenzen stehen in 01–05; keine Implementierung beruht auf einer geratenen Auflösung.

**OFFEN:** Root Cause. Konkrete verbleibende Nachweise stehen in [06](06-OFFENE-HERSTELLERFRAGEN.md), sämtliche Quellenidentitäten/Fundstellen in [05](05-PROVENIENZMATRIX.md).

## Sicherheits- und Änderungsumfang

Das Zielverzeichnis war bei der Erstprüfung und unmittelbar vor Anlage nicht vorhanden. Angelegt wurde ausschließlich dieses Verzeichnis mit den sieben beauftragten Markdown-Dateien; Dateien wurden exklusiv neu angelegt, keine vorhandene Datei überschrieben. Keine Originalquelle wurde ersetzt. Keine weiteren Repositorydateien, temporären Dateien, Logs oder Symlinks wurden erzeugt.

Git wurde nur lesend mit GIT_NO_LAZY_FETCH=1 und --no-optional-locks verwendet. Kein add/commit/reset/checkout/clean/fetch/pull/push. Kein Build, Patch, Nix-Evaluationslauf, Hardwaretest, Boot, SSH, Blockgerätezugriff oder Medienzugriff. Keine Wiederöffnung der abgeschlossenen I2C3-/ES8328-Untersuchung. Externe Forschung ausschließlich über normalen verfügbaren Zugang; gescheiterte Host-DNS-Auflösung wurde nicht umgangen.

## Prüfmethodik und tatsächlicher Kontrollpunkt

Vor eigener Forschungsarbeit wurden Branch, HEAD, Indexhash, vollständiger Git-Porcelain-Status und ein Byte-/Linkmanifest erfasst. Der erste Lauf enthielt **404 Einträge**: reguläre Repositorydateien plus im Dateilauf sichtbare Symlinks, ohne .git. Er hatte **drei Symlinks auf Verzeichnisse ausgelassen**. Bei der Prüfung vor dem ersten Schreibzugriff wurden diese zusätzlich erfasst; der Vergleich aller ursprünglichen 404 Einträge war unverändert. Die drei ergänzten Symlinks besitzen lokale mtime/ctime vom 07./12./13.09.2026. Diese Zeiten sind Metadaten, kein rückwirkender kryptografischer Vorherbeweis. Ergänzter Vor-Schreib-Kontrollpunkt: **407 Einträge**. Beide Gruppen sind unten getrennt nachvollziehbar.

Der gespeicherte Manifestumfang prüft Dateibytes und Symlinkziele. Er ist keine vollständige Dateisystem-Metadaten-/ACL-/Verzeichniszeitprüfung und verfolgt keine Nix-Store-Symlinkziele. Es wurde nichts in .git geschrieben; die Indexbytes wurden separat verglichen. Ein Git-Commit ersetzt dieses Vorhermanifest nicht.

Unmittelbar nach Anlage von 01–06 und vor der exklusiven Anlage dieser Datei wurde tatsächlich geprüft:

- Alle 407 vor Schreibbeginn erfassten Einträge unverändert; keine fehlenden Dateien, keine geänderten Bytes oder Linkziele.
- Ausschließlich die sechs erlaubten neuen Markdown-Dateien hinzugekommen.
- Branch main, HEAD 0649b5523b2991d3283413bba835c7ee2f7be432, Index-SHA256 b09a511bb8933143de0e4d91560f2762330a9f6ed07e13e020e0a112921f4454: unverändert.
- Vollständiger Git-Porcelain-Status entspricht exakt dem ursprünglichen Status plus sechs neuen Dateien.
- SHA256 aller sechs neuen Dokumente, 32 eindeutige Quellenkennungen, interne Dokumentlinks, Klassen-/Ergebniskategorien geprüft; PASS.
- Inhaltliche Prüfung trennt Originalbefund, Softwarebefund, übernommene Angabe und Hypothese; keine Root-Cause-/Hardware-PASS-Behauptung.

Die danach folgende rein lesende Endprüfung erfasst zusätzlich die Existenz, Quellenverweise und SHA256 **aller sieben** Dateien und den unveränderten Ausgangsbestand. Ihr Resultat einschließlich des Hashes dieser Datei wird im Abschlussbericht der Sitzung ausgegeben. Diese Formulierung behauptet die spätere Prüfung nicht vor ihrer Durchführung. Diese Datei wird nach ihrer Anlage nicht nachträglich geändert.

## SHA256 der abgeschlossenen Dokumente 01–06

| Dokument | SHA256 |
|---|---|
| [01-NXP-ORIGINALQUELLEN.md](01-NXP-ORIGINALQUELLEN.md) | `f19f3b5ec1466fd4ff7536e3ea5d5f7d5b871753cac997dfb3c885a042657c2e` |
| [02-MMDC-REGISTER-UND-ERRATA.md](02-MMDC-REGISTER-UND-ERRATA.md) | `e361b8d98ce981c02eb58a8bce2716a10f6fd67e8ae20887b69f644d90290b82` |
| [03-NOVENA-KOSAGI-ORIGINALQUELLEN.md](03-NOVENA-KOSAGI-ORIGINALQUELLEN.md) | `cada47fc4640530e259e12d7e8c09ef926d376f4ebeb27ee77caecf263723703` |
| [04-U-BOOT-HISTORISCHE-BELEGE.md](04-U-BOOT-HISTORISCHE-BELEGE.md) | `e3f5dcacce2bfd03af2a105e3cdc839f6b58f9f203bbaf0c218b2f396f36408c` |
| [05-PROVENIENZMATRIX.md](05-PROVENIENZMATRIX.md) | `17d3840dfe5d5b9ec6d520d94a33597bb8d7e6f92874029f1e38c3b254fbf4d9` |
| [06-OFFENE-HERSTELLERFRAGEN.md](06-OFFENE-HERSTELLERFRAGEN.md) | `16d9139e24c9135210025a3aec9e3140675576102dbe7e79d9276354bf5b7656` |

Für 07 wird kein Selbsthash im eigenen Inhalt erfunden: Das Einfügen des eigenen Hashes würde die Datei erneut verändern. Der tatsächliche Hash der abgeschlossenen Datei folgt in der Sitzungsausgabe.

## Ausgangs-Gitstatus

Branch: `main`; HEAD: `0649b5523b2991d3283413bba835c7ee2f7be432`; Index-SHA256: `b09a511bb8933143de0e4d91560f2762330a9f6ed07e13e020e0a112921f4454`.

Vorhandene Nutzeränderungen umfassen bereits indexierte Diagnosepatches, die Änderung von default.nix sowie unversionierte Dokumentations-/Forschungsdateien. Sie wurden erhalten. Exakter initialer Porcelain-v1-Status mit --untracked-files=all:

```text
 A boot/u-boot/0004-novena-uboot-diag-d0-d36.patch
 A boot/u-boot/0005-novena-diag-d21a-d21d.patch
 A boot/u-boot/0006-novena-diag-brk-p0-p1.patch
 A boot/u-boot/0007-novena-diag-brk-q1-q2.patch
 A boot/u-boot/0008-novena-diag-control-c0-c2.patch
 M boot/u-boot/default.nix
?? AGENTS.md
?? docs/QUELLENVERZEICHNIS.md
?? docs/TRANSLATION-WORKFLOW.md
?? docs/en/README.md
?? docs/en/docs/BUILD-HOST.md
?? docs/en/docs/BUILD.md
?? docs/en/docs/GOLDEN-BUILD.md
?? docs/en/docs/IMAGE-LAYOUT.md
?? docs/en/docs/QUELLENVERZEICHNIS.md
?? docs/translation-glossary.json
?? docs/translation-manifest.json
?? research/05-gesamtanalyse/00-LESEANLEITUNG.md
?? research/05-gesamtanalyse/01-PROJEKTGESCHICHTE.md
?? research/05-gesamtanalyse/02-LOKALE-QUELLEN.md
?? research/05-gesamtanalyse/03-QUELLENVERZEICHNIS-AUDIT.md
?? research/05-gesamtanalyse/04-QUELLCODE-UND-BOOTPFAD.md
?? research/05-gesamtanalyse/05-TESTLOG-ANALYSE.md
?? research/05-gesamtanalyse/06-FEHLER-UND-RISIKEN.md
?? research/05-gesamtanalyse/07-HYPOTHESEN.md
?? research/05-gesamtanalyse/08-EXTERNE-RECHERCHEFRAGEN.md
?? research/05-gesamtanalyse/09-UNTERSUCHUNGSPLAN.md
?? research/05-gesamtanalyse/10-OFFENE-FRAGEN.md
?? research/05-gesamtanalyse/11-ARCHIV-DATEIAUDIT.md
?? research/05-gesamtanalyse/12-URL-FUNDSTELLEN.md
?? research/05-gesamtanalyse/13-ROHLOG-UND-EVIDENZREGISTER.md
?? research/05-gesamtanalyse/14-HISTORISCHE-GIT-BELEGE.md
?? research/05-gesamtanalyse/15-AUSGANGSCHECKPOINT.json
?? research/05-gesamtanalyse/16-ABSCHLUSSPRUEFUNG.md
?? research/05-gesamtanalyse/16-NXP-MMDC-PRIMAERQUELLEN.md
?? research/05-gesamtanalyse/17-NXP-ERRATA.md
?? research/05-gesamtanalyse/17-VERTIEFENDE-PRUEFGRENZEN.md
?? research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md
?? research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md
?? research/05-gesamtanalyse/20-EXTERNE-QUELLENMATRIX.md
?? research/05-gesamtanalyse/21-EXTERNE-RECHERCHE-LUECKEN.md
```

## Ursprüngliches Manifest: 404 Einträge

Maschinenlesbares JSON: Schlüssel = Pfad relativ zum Repository mit ./; Wert = [file, SHA256] oder [link, wörtliches Linkziel]. .git ausgeschlossen, Index separat. Dieses Manifest dient zugleich als Inventar der vorgefundenen Forschungsdateien. Es enthält keine Inhalte der Dateien und keine Hashes nicht vorhandener Herstelleroriginale.

```json
{
  "./.gitignore": [
    "file",
    "1efd7b82977e398b289f89f01528617d2fa870b5db5b541719789d0322ffb56f"
  ],
  "./AGENTS.md": [
    "file",
    "22563b7c84bdf9463ea13b4f8138b3d673e5b665c277d30e6be09b7f4a4f982a"
  ],
  "./README.md": [
    "file",
    "8e8c0ee331d8cb6efc789aa4987284cb79b33a3f22c077b708cdc18930fe8909"
  ],
  "./configuration.nix": [
    "file",
    "44dce48cf182a075f899b09bd2a12642fcf54a332332e8f226d0d84b4e3aac3d"
  ],
  "./flake.lock": [
    "file",
    "ac1b29b7be5afafd4cb045fa8bb41db1d08b903afcfdbbfc9a042d0bdea0dfd6"
  ],
  "./flake.nix": [
    "file",
    "147f0b39ffeb7b7873ff5aed9adeb8ecd26536d93b9b93e9fb86cd06304a62f8"
  ],
  "./result": [
    "link",
    "/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img"
  ],
  "./result-current-baseline-dtb": [
    "link",
    "/nix/store/0fy98965pkr326mb27wmlw6w0hnr0b51-imx6q-novena.dtb"
  ],
  "./result-dtb": [
    "link",
    "/nix/store/dwjrq51m78hhbr3ip1s9d6g9y00jm8ir-imx6q-novena.dtb"
  ],
  "./result-dtb-no-single-master": [
    "link",
    "/nix/store/4si3w83jsr26r8rrcj3wcgpm692f9rkg-imx6q-novena.dtb"
  ],
  "./result-full-image-0008": [
    "link",
    "/nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img"
  ],
  "./result-kconfig": [
    "link",
    "/nix/store/vslqb6asbfd5l1lg4sp9a4wmvzjnz62y-linux-config-armv7l-unknown-linux-gnueabihf-6.18.49"
  ],
  "./result-stmpe-dtb": [
    "link",
    "/nix/store/w61rdmgawbi0r1vn9vf4ynk28nwhxck2-imx6q-novena.dtb"
  ],
  "./boot/reference/novena-imx6-spl.bin": [
    "file",
    "b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d"
  ],
  "./boot/reference/novena.dtb": [
    "file",
    "2718a20d85aaa32f264630504bb9acde651ec810a08beb5560524d9c12554194"
  ],
  "./boot/reference/u-boot-dtb.img": [
    "file",
    "f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa"
  ],
  "./boot/reference/zImage": [
    "file",
    "71a67ae3715655db900b348944d34bd71a2662ea28cc5bf10e5a117e4cad3093"
  ],
  "./boot/u-boot/0001-novena-spl-boot-second-stage-from-external-sd.patch": [
    "file",
    "393b5da31d3029c02268f801c4d95abb7ddc04f487bdc69a22659e872d4a955d"
  ],
  "./boot/u-boot/0002-novena-boot-external-sd-by-default.patch": [
    "file",
    "d9e647a122f4992166715c1ca2c2b901c0e265910095415a36f71d900872fe74"
  ],
  "./boot/u-boot/0003-novena-use-non-persistent-default-environment.patch": [
    "file",
    "dce5aa79a5e939a50300d337b7e125791ec6f94d8afbe98511043b2a2a4f6617"
  ],
  "./boot/u-boot/0004-novena-uboot-diag-d0-d36.patch": [
    "file",
    "81d0daa23a86a2d7a2f235dfae2fbe15861b54ebaf105c66bab7e773106f2863"
  ],
  "./boot/u-boot/0005-novena-diag-d21a-d21d.patch": [
    "file",
    "d6e66018d14ecd01031349f49aa9140d7e3bc19dec5044feaafdf848b58f697c"
  ],
  "./boot/u-boot/0006-novena-diag-brk-p0-p1.patch": [
    "file",
    "5003dec98da48d311318b9814ed62d7ccaeb519f9bc7224d6d0544dc6f0e9eba"
  ],
  "./boot/u-boot/0007-novena-diag-brk-q1-q2.patch": [
    "file",
    "60ff3d4206f52646312ff493b853667514e1c5f0256328cd8aae2a97e9287849"
  ],
  "./boot/u-boot/0008-novena-diag-control-c0-c2.patch": [
    "file",
    "74309488a0aa9232f68235c6214327c2d3abb79613222f1850a4d7b992f52d47"
  ],
  "./boot/u-boot/default.nix": [
    "file",
    "de6301321f726b8767f80382eb203ca2972b63d7d2c2587df0bc8dfcad627188"
  ],
  "./docs/BUILD-HOST.md": [
    "file",
    "177912b15172f9e4aad593eea44b58b981a4aec437aa58ffb56fcd24dedbff1f"
  ],
  "./docs/BUILD.md": [
    "file",
    "15820789a6d8fdd0d8a3b9e77b0d6196a8d19d0f8e3bef55b97db57da1a42007"
  ],
  "./docs/DECISIONS.md": [
    "file",
    "9d3e2e074c50e3e874e1a7e5d94a62b58a4ffb84f69d053ef4f6591b6e8979e8"
  ],
  "./docs/GOLDEN-BUILD.md": [
    "file",
    "27c18a468cac00f1caebb85e882c84aeab345b8c356b7be2089e2209c0d1bdba"
  ],
  "./docs/HISTORICAL-MEDIA.md": [
    "file",
    "c6ad3550cb1a7574cf553b52eae556566e136fd6890ff1624c8910cfc33a094c"
  ],
  "./docs/IMAGE-LAYOUT.md": [
    "file",
    "b1c595e9bb4c7ec8fa51c3cdfdcabf1a243f2d23f1b2582d1716801847f4dbcc"
  ],
  "./docs/PROJECT-STATE.md": [
    "file",
    "4b22fb261a1bc76a3225593fca642af56fc11a2806a9e0b3d781934f8ae04aca"
  ],
  "./docs/QUELLENVERZEICHNIS.md": [
    "file",
    "b787bfbcaf8390fa089ff97a06515394a6284b9b52fe1056529bec18794def64"
  ],
  "./docs/TEST-LOG.md": [
    "file",
    "b8f834583e0cf7c5d4cfed84c5858e09efe6d78b43c4d4d8b0e9a3d76bb6477b"
  ],
  "./docs/TRANSLATION-WORKFLOW.md": [
    "file",
    "25e01715827de82e3690edb621d04d015935a1a57ab7d6bca6fc8b8384f56a59"
  ],
  "./docs/translation-glossary.json": [
    "file",
    "7a78c0832466a96c8b15aa5eb6eaee691782756593e1f1213c9a96738f749f76"
  ],
  "./docs/translation-manifest.json": [
    "file",
    "1bc94575222008fd75cba502cad88e2c2f8547011ab599bf1caaeecd231644e3"
  ],
  "./docs/en/README.md": [
    "file",
    "0b6b1e9c90925974174351454fa792357bd3535b37ed78e461142bc2e3e31e71"
  ],
  "./docs/en/docs/BUILD-HOST.md": [
    "file",
    "562ad5907f7d426c1282469ea571f05f8abc86d641fb75dc66dda0ed84816586"
  ],
  "./docs/en/docs/BUILD.md": [
    "file",
    "6988b343509d9f63a3742eea4a5e23dc46ab8e022c8e2f819eca5d6fcd5abd18"
  ],
  "./docs/en/docs/GOLDEN-BUILD.md": [
    "file",
    "d3e53db968b4dd2adcf460c7c8fefa7ceec27b6978f2db75263f21ef1b36850e"
  ],
  "./docs/en/docs/IMAGE-LAYOUT.md": [
    "file",
    "dd097e8c760a41a3fe25aef9d39ae25ce680dc0141afc3fe69d24c6fb98e0442"
  ],
  "./docs/en/docs/QUELLENVERZEICHNIS.md": [
    "file",
    "bb07164ea8a4ae6709e0d2192c5ec1a3956b69844471554492d93da216800236"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/README.md": [
    "file",
    "248f90b5e3a35cf09afb7f7b7d692b0ed173ddf2a3b66be937e0a055822c4cee"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS": [
    "file",
    "ea76cabec687549607f422a545cd2f7f4a9354baee469f6367700923abdb9582"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/cmdline.txt": [
    "file",
    "7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/critical-window.txt": [
    "file",
    "260cec51b67d8fc4ba84b2355ba73f8058886bbe1eb1c490b5f0960e54505193"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/date.txt": [
    "file",
    "9c6831ad809b065a5a82080928d70c2e19513c255c0bd4ee0f2fd74756072e4a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/display-status.txt": [
    "file",
    "10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt": [
    "file",
    "d65e04f2a3ea80bfa3467ccaa56e036967ea8490d2d5c732abac666d299bd657"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/es8328-power.txt": [
    "file",
    "37c281e6896572aae5832487f092af0e1c2504773eceb5648c3b13f161ebe3f1"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/i2c3-devices.txt": [
    "file",
    "e5266d5aac13a3ea35f5e5da8a3b81c4f083b87661518794d5ecc47c6d0c7747"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/it6251-start.txt": [
    "file",
    "208de32b8518c0c8a9ccfaf8a4cc7266b58432b50fa29ad64ba3ce1ae10736d5"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/live-dt-audio.txt": [
    "file",
    "dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/live-dt-files.txt": [
    "file",
    "f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/regulator-summary.txt": [
    "file",
    "8366b5e8b476c8a1c9ea27684ddd3548614a860c622fdce25e5aedb875f7fc2c"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt": [
    "file",
    "e8a9d0e5c6f50494b29cbd75d21963d094fe1af1971409a2a9bc140e536d2f21"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/uname.txt": [
    "file",
    "4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS": [
    "file",
    "270034bc7598428a17114cf2fb0ff8dbb22f7b5af7d7fca49fff0bfb33d11278"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/cmdline.txt": [
    "file",
    "7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/critical-window.txt": [
    "file",
    "d5c46eeb166df3c4a611abfbc34b6a46e2ee380e4da6ee03ee742f7d0b51128a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/date.txt": [
    "file",
    "24f56b4bac259a8d11a4ba68b3a3c4ab9826700215bf6ab84d176019744fc9d7"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/display-status.txt": [
    "file",
    "10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt": [
    "file",
    "e0fa393c79cdeb45cd86c45769c5a71f0a4b0a83d42dd24b78894599d63b2e9e"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/es8328-power.txt": [
    "file",
    "c362af2c33c65f96648ec76a90a925f441e7ab0c06d0b1caad541f0905a3b726"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/i2c3-devices.txt": [
    "file",
    "8cb765b59966fb72c39a4494835528f08ae334247db433172eafe22d0f47638d"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/it6251-start.txt": [
    "file",
    "0e4ad744c66c80452aef26bff208e813a8daa758d57f7b897c0fbfa84307026d"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/live-dt-audio.txt": [
    "file",
    "dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/live-dt-files.txt": [
    "file",
    "f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/regulator-summary.txt": [
    "file",
    "bd10fa5eb1e0c4d169b8a9bdb9604f71f71677cc8c59536356cca6eb60033482"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt": [
    "file",
    "9dfc3117e6fd2cd96304885f5ee6ddf98e2b9a73daf510c5eae2c2411c77106a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/uname.txt": [
    "file",
    "4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS": [
    "file",
    "c688a634d11962eb032136801bda83c90dc635503567b4fe0b2073f649cb11d3"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/cmdline.txt": [
    "file",
    "7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/critical-window.txt": [
    "file",
    "7e562cf0c1972189b5a5d977e6129a9efe0ece0f795ffcfe637b9e3f9d36d3a6"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/date.txt": [
    "file",
    "7d85571d497a9b3906b1be1d3de98f03b7f04ef862f104da4577117eb5845459"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/display-status.txt": [
    "file",
    "1e9b2a48132a7900f56e4d2f5e28748b6da3338668c0a4b3b82ee580c8647a9a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt": [
    "file",
    "599bf04560888a697d422588a1d06c483c32b3521eefdeb6a1cf37a5b624702e"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/es8328-power.txt": [
    "file",
    "cb50f0f903a9cdec6acaa7ae21bb14a22ba72798df51826b6ea833c7828beff3"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/i2c3-devices.txt": [
    "file",
    "7e4b39d2669006a4585837ca3e383368c9b1946514bae8771d63c0554705897b"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/it6251-start.txt": [
    "file",
    "19aec81e7c37e9a9d0f4e9971ac79e4f43864e9bf9ec8eac5dea8cdb1f180b3e"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/live-dt-audio.txt": [
    "file",
    "dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/live-dt-files.txt": [
    "file",
    "f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/regulator-summary.txt": [
    "file",
    "8366b5e8b476c8a1c9ea27684ddd3548614a860c622fdce25e5aedb875f7fc2c"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt": [
    "file",
    "2f9d6eda6cd481255b3a84800b2f5de14bf866f62f32d336479b19b6dfd6fc53"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/uname.txt": [
    "file",
    "4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS": [
    "file",
    "f9c028a41b88e3cb124eef7d86e38e8e9e9f44c8b45c8bd5bd126be3e2839036"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/cmdline.txt": [
    "file",
    "7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/critical-window.txt": [
    "file",
    "a21092c8c6dba4cba4a12e4f145938b1a521f1e89017ce43a4129323166ae01b"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/date.txt": [
    "file",
    "cb8808aa77a80506cb11de4a9fdbb2eb7cb6758d94ec21d0666b275966307dc0"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/display-status.txt": [
    "file",
    "10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt": [
    "file",
    "d959a7193f097e58a1b5c70bfabe87ab6fbdddb54c2a97cb55d284cbaf505844"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/es8328-power.txt": [
    "file",
    "ff0254655ab5ceba4e6dc9ba88303a0ec5b62186efa6756e19c7f7dbf9ba1121"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/i2c3-devices.txt": [
    "file",
    "3e46a264204aa89e19c867fd139d35b618d1089419fc231257d0b8c19ffe43cd"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/it6251-start.txt": [
    "file",
    "334537e0e8b31e00f42f468cd2cef3697aa893ed697562c441f8bb14da2af72e"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/live-dt-audio.txt": [
    "file",
    "dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/live-dt-files.txt": [
    "file",
    "f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/regulator-summary.txt": [
    "file",
    "bd10fa5eb1e0c4d169b8a9bdb9604f71f71677cc8c59536356cca6eb60033482"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt": [
    "file",
    "5ab8295b3d65b4531bc838cc1d0a19a98484a8788832775201a32ceb67216ce5"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/uname.txt": [
    "file",
    "4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS": [
    "file",
    "bca629cefbc48118e5d6a5e9093c56c69419d8dbfc278883a6476ada2dff506c"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/cmdline.txt": [
    "file",
    "7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/critical-window.txt": [
    "file",
    "06e222c99235eb884a9de95f2b8e842d154955898968b37d818017ffed56578b"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/date.txt": [
    "file",
    "3427dda5b9f596088ff56edb4d710fed59408c43c4950d7ff29de6ac0c8ae631"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/display-status.txt": [
    "file",
    "1e9b2a48132a7900f56e4d2f5e28748b6da3338668c0a4b3b82ee580c8647a9a"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt": [
    "file",
    "19b9f51046f47be276d7d8545fe695cb8ab576d0942a36f181b1932d38909471"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/es8328-power.txt": [
    "file",
    "d63a1f28e616bf5ec788e03f1b1dc26abfecb6600fbab699b579abbf33e28ede"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/i2c3-devices.txt": [
    "file",
    "a61a598463a3c01f836a4b420e7ff6a6839be4d1b771e4f1b5c562eadab3c0d3"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/it6251-start.txt": [
    "file",
    "a54da85f2a96da13b5dab370208ac8ae07b5fd533573e72d5a9537796532428b"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/live-dt-audio.txt": [
    "file",
    "dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/live-dt-files.txt": [
    "file",
    "f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/regulator-summary.txt": [
    "file",
    "a73d16d88521c68e6f393af5613f6a6b64616034159691321b8b247f368c711f"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt": [
    "file",
    "bb99d51e7ab21d01e5e8572715662b49f5ab6a50c6e427b677e88c11fd45f77d"
  ],
  "./evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/uname.txt": [
    "file",
    "4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS": [
    "file",
    "32882c11c7a00bb312912523f965a8bef67016dc82f0ea1a3bdbc51c3719a525"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt": [
    "file",
    "c20c98e924d368111da97b18930b5c86c03fde42ffb9d20cdee2d5459bc88da4"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/dmesg-it6251-i2c3.txt": [
    "file",
    "d325a8bcc1b34c709f66b1b86795f72b8fbb26b335dc2bc57759b5f50b4b3ee6"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/gpio.txt": [
    "file",
    "0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/i2c3-devices.txt": [
    "file",
    "f56851d185a54902f549b8be6d0d1eb9fc973609f7e171a3c27286286de8eede"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/ldb-binding.txt": [
    "file",
    "09fc9f5328cea83c7b0580e89d6118a8d23033fd18513f6d321f9bc2554259c5"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/live-dt-audio-files.txt": [
    "file",
    "866b6473300f86ab00070624718d15ceb66cab4b55e72aa9208074c28d448008"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/live-dt-audio-hex.txt": [
    "file",
    "6684f147d031928eeaa6c45a6357979a1bdd2c3fc29e1f7d8b1efe8eea3e1577"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/regulator-summary.txt": [
    "file",
    "17b8d66aa0dcde3562cf06b910c02334f1b3e95aac52209647ef1090e2ecf486"
  ],
  "./evidence/block-3.5-h3-1-2026-09-15/system.txt": [
    "file",
    "aa8bb21310ebc9ce594132f1b12cb3df5f535685a57b73c9b62aab94facf5188"
  ],
  "./hardware/novena.nix": [
    "file",
    "5128c9d0e2a80b98d22f5fe87a63e0872186899c4d30a90f2ad56906ddb2a979"
  ],
  "./image/make-ext4-fs-reproducible.nix": [
    "file",
    "f82923db8bdb18e3905fd29e513d3003ce6260ff42f61aac2e0ba91ae2570564"
  ],
  "./image/novena-image.nix": [
    "file",
    "4fba5e1a1174bba5d5d1a22ed0ad8e81a4db693a94f08088ed08adc7ccc40259"
  ],
  "./kernel/0001-drm-bridge-it6251.patch": [
    "file",
    "13be3b7015e7a8613dc887d61534def23b27672188a1e8832a26ab80ede2542b"
  ],
  "./kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch": [
    "file",
    "31cfb4c753d9723b4197faf98ee5b05ac608fa27156b48cae4dc7c80bfa08308"
  ],
  "./kernel/0003-i2c-imx-debug-arbitration-lost.patch": [
    "file",
    "a182a0e8fe57038c7b4461ba273339a3eb13a9bf3fff0b6802c98a361a6d7e45"
  ],
  "./kernel/0004-i2c-imx-debug-start-state.patch": [
    "file",
    "550febf18cbb59cb8d82faeb3a3e1515f915788298ec2524588eb77463503c7b"
  ],
  "./kernel/0005-i2c-imx-debug-start-error.patch": [
    "file",
    "81c2de96d7200a6e7e2684518c711bc5686b8d5295a52d5e7aaa54efed94e556"
  ],
  "./kernel/0006-i2c-imx-debug-start-transition.patch": [
    "file",
    "23a0e0c00860565fca1d8ccb1546b4d8e893d0f423e6c450013e76a47b70903f"
  ],
  "./kernel/it6251.c": [
    "file",
    "e9c6a7d62101f29e9c69cae0f7d1fc43a370d8591951cd8b0bb9eb20ab06d9d7"
  ],
  "./kernel/it6251.c.before-drm-lifecycle": [
    "file",
    "39fbfe86bdc89d0947589351b1ca83254d7345e5a6a6d44bda86ab439020dde0"
  ],
  "./kernel/linux-6.18.49/drivers/gpu/drm/bridge/Kconfig": [
    "file",
    "0caa07f1b9c08f954f9b4d8f21301ffdff7bc5b0ee0373bbbad173c61004c652"
  ],
  "./kernel/linux-6.18.49/drivers/gpu/drm/bridge/Makefile": [
    "file",
    "bc87000528721d840bd39dc4f349ce6e49aafb697313c8a178e06c232ceda6c1"
  ],
  "./kernel/linux-6.18.49/drivers/gpu/drm/bridge/it6251.c": [
    "file",
    "046b41e77ee95b4dd247a6e4dc72a74a4d11ec29a029406b5ddaaf8a6c34e408"
  ],
  "./kernel/linux-6.18.49-orig/drivers/gpu/drm/bridge/Kconfig": [
    "file",
    "b227f5d11b2cbe1d690c3a20a8b7b0aaa3e8513dade6b815784841aee9559464"
  ],
  "./kernel/linux-6.18.49-orig/drivers/gpu/drm/bridge/Makefile": [
    "file",
    "16f2828ad5397c581272558778e9641b819366ce371ee3e0a4575cbf1ae4cb35"
  ],
  "./kernel/panel-patch/a/drivers/gpu/drm/panel/panel-simple.c": [
    "file",
    "2047140aa2b9b264684d0baf2fe131f1a2ce946802705abd19acf8b57682de48"
  ],
  "./kernel/panel-patch/b/drivers/gpu/drm/panel/panel-simple.c": [
    "file",
    "615dcea07608d087d253770ec3f13897ff460178be45a457ad4f1b462b3af3ae"
  ],
  "./novena-i2c-test/i2cget": [
    "file",
    "9d1f8491892d27fcf86721846b941e3ec6e401fe3cd0468797a222f3823d8db0"
  ],
  "./novena-i2c-test/ld-linux-armhf.so.3": [
    "file",
    "8ee5ef79288202573ab461e2fe46176d6a59f1ed2bfcc1ffec2b97f4172bd195"
  ],
  "./novena-i2c-test/libc.so.6": [
    "file",
    "cc2792cb448632fab5770511faddb700dff90d832bf1120e459ac75028cc421a"
  ],
  "./novena-i2c-test/libi2c.so.0": [
    "link",
    "libi2c.so.0.1.1"
  ],
  "./novena-i2c-test/libi2c.so.0.1.1": [
    "file",
    "d5562f407b5be1914abcbaa33e07e20a4f6ddea7b5ca4486c5aeb37349e87db3"
  ],
  "./overlays/reproducible-bash.nix": [
    "file",
    "1a8e4f4de94c0d19ce94cab9f2c9f2999901d6aeb06d4f8dc65a5ae17fa95357"
  ],
  "./patches/bash/0001-bash-cross-build-use-deterministic-pipe-size.patch": [
    "file",
    "f73c57a7146e5307aa844fac8f98063cad1706ccf526f6d1796f2c62dd68728b"
  ],
  "./research/01-original-novena-docs/SHA256SUMS": [
    "file",
    "90493e9fa86be87cf6e9067d6389f7ed7f80ed64c1f9e1182d3122d25ab96439"
  ],
  "./research/01-original-novena-docs/extracts/edp-adapter-dvt1-layout.txt": [
    "file",
    "3f9a984960e91d7929858391300dddae4678d25c25f2e18afccfdf0ac9bf0118"
  ],
  "./research/01-original-novena-docs/extracts/novena_pvt2-layout.txt": [
    "file",
    "ed5761be8c696fe47a97c0b3072dad77c4ff461b900fe1b9669007e0157376e8"
  ],
  "./research/01-original-novena-docs/extracts/pvt-issue-log-content.html": [
    "file",
    "479288aa3adcd8ae11809c0835a56ac2d425ad081e07ac3febb70aa5ad770dc6"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-eco23.txt": [
    "file",
    "261467abaf544073af0fe071056121748f564f8e15378368f8bcfa8b8d6e3664"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-i2c3-index.txt": [
    "file",
    "37c5f383721ede506218766b99e6d5fe8abdccd890f737afcac6e5ee846355b7"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-official-layout.txt": [
    "file",
    "8fe6c7dc4f90f1bb1aa425eb736c7be9df08195426c095bc5eb5c31e8f30da74"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-old-layout.txt": [
    "file",
    "ed5761be8c696fe47a97c0b3072dad77c4ff461b900fe1b9669007e0157376e8"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pdf-text.diff": [
    "file",
    "8b8deccefd5327da8513d1ee2c059709948886af605d2f7a257cb8a69a1cea52"
  ],
  "./research/01-original-novena-docs/extracts/u-boot-novena-content.html": [
    "file",
    "b1d20027d5fc8f7a04a874825c6012866613c32c693ce4137608e975e0166ffc"
  ],
  "./research/01-original-novena-docs/extracts/u-boot-pvt-notes-content.html": [
    "file",
    "71bb3d7aa9f5c13de3245f5c5137fbbb759b2148c6445ddcf35d55a3d589be65"
  ],
  "./research/01-original-novena-docs/extracts/edp-adapter-dvt1-pages/page-1.png": [
    "file",
    "44f29c98b47885ec2af5ae70b81e636ca8a344ff097002d2b20deb9a915d8683"
  ],
  "./research/01-original-novena-docs/extracts/edp-adapter-dvt1-pages/page-2.png": [
    "file",
    "fd80950f0d4088808697e18518a49e8a32b0fba22f6437ba674e1b794f4d9b79"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-1.txt": [
    "file",
    "19a537742dc3c8322427a19524c7d2aa0ed83f1ce6c1d2fc0e5e0ca10dd11e04"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-13.txt": [
    "file",
    "45aa480c5e315c6ed879731bacb39368bf28321fe79f1c135af39b091bdd7f94"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-14.txt": [
    "file",
    "302da4b01371fe79e81ee0fdf13ef49c516fad6f60fe6a4e3d825dbfcbe0898a"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-15.txt": [
    "file",
    "d3ae645654ce737de1dc4d004a0bef0f6032b127fefbb0913022d6d926999c50"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-6.txt": [
    "file",
    "acf42350713665feb78fb0d281b4488b2f8e68cc54f0b8011197d2f5d591d442"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/page-7.txt": [
    "file",
    "32609d841f0e04dcb31a21669289f9f3e89d864e6be231a19ace77a891c382da"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-1.png": [
    "file",
    "3a82d038611e5a3d582cf318bd28e5dec7ff9ab8871cfbd53d293e10a9684420"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-13.png": [
    "file",
    "6f12ad2a5208a6013f14c98a1d45dd27962b3597bb556c5a05c3de78011f7e78"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-14.png": [
    "file",
    "bfd2dcc24ef85025c88447ebf3e8fee84dd51992f849fdd60973d9707939b263"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-15.png": [
    "file",
    "52209c70506659ce1f580b906b6c3de432eb3265afaf4e40e1309e6fe1962cd8"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-6.png": [
    "file",
    "9b80b35e79d9c02e3eb73f7a5cbc748989fe41476a8361c3da513c59a7c53c0b"
  ],
  "./research/01-original-novena-docs/extracts/pvt2-pages/png/page-7.png": [
    "file",
    "365de09a0f1874c8712bf27a505199573b72fc9eb664584f6a60ce76dbe9ae51"
  ],
  "./research/01-original-novena-docs/notes/edp-adapter-dvt1-i2c-power.md": [
    "file",
    "57828074598edacc92b570e8e4cbccd209e5e792f95b8d22b422b8e347255cbd"
  ],
  "./research/01-original-novena-docs/notes/novena-pvt2-source-metadata.md": [
    "file",
    "4e8edb86125410904f7674dcf51524c2d8e88dc6be8faabd9990b251018f95d0"
  ],
  "./research/01-original-novena-docs/notes/pvt-issue-log-analysis.md": [
    "file",
    "c830e716a1fb9c7f76a1ed27420102025928b4652b3f162a71150687c007b1f1"
  ],
  "./research/01-original-novena-docs/notes/pvt2-a-i2c3-topology.md": [
    "file",
    "64d647ced6e6f9cc54073b7df61414b8ee0b26c20458c6264f6696fbb3bdc0d1"
  ],
  "./research/01-original-novena-docs/notes/pvt2-eco-analysis.md": [
    "file",
    "40547909fb840d7e877fb6eec6004697297445951951620241520b92deb28fe6"
  ],
  "./research/01-original-novena-docs/notes/spartan6-i2c3-configuration-state.md": [
    "file",
    "ba67cb1763ed33ce116ca8ae4f7c5e94a9c44f8978ee6d5f74b8d6c23a9401de"
  ],
  "./research/01-original-novena-docs/notes/u-boot-documentation-analysis.md": [
    "file",
    "e42368fa07b0cd5b8dd864020fd4264ce380b0633a81349d661814d1a92e24b6"
  ],
  "./research/01-original-novena-docs/sources/SCH-novena_pvt2.PDF": [
    "file",
    "0285de44da7471e3459b7b8517e7d3e42fbda049540530c404ddbb873a514672"
  ],
  "./research/01-original-novena-docs/sources/SCH-novena_pvt2.http-headers.txt": [
    "file",
    "059050e9ab78988929f1d6832935d60e529fbfef42a153cc744caa8afdebd023"
  ],
  "./research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.http-headers.txt": [
    "file",
    "a7c2ee078a5e6428d8b12f6880b068e5fb4e5ae21ea79bb49ee1e3f0d42995d9"
  ],
  "./research/01-original-novena-docs/sources/amd-xilinx-ug380-spartan6-configuration.pdf": [
    "file",
    "4afb6472018a9b3fa3bc1be906a0d15f86a17567d1dd7e930debfce3de362c9b"
  ],
  "./research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.http-headers.txt": [
    "file",
    "7abac9a97d15f938c9b205410057da82d4df6d53c3efb2504d264122819bef7a"
  ],
  "./research/01-original-novena-docs/sources/amd-xilinx-ug381-spartan6-selectio.pdf": [
    "file",
    "4a0fc9078af54edc1104452fe5fa2bfaa82118b5501ba9fd000c1e1e2310821a"
  ],
  "./research/01-original-novena-docs/sources/eDP_adapter_dvt1.PDF": [
    "file",
    "ca9aa00cf64190d63469795f84381890928b48a98cd8d13298ecfcde46861f2a"
  ],
  "./research/01-original-novena-docs/sources/eDP_adapter_dvt1.http-headers.txt": [
    "file",
    "793519d9545c1a61dc3f0b11005cc1b6aa40b1ec261091734e73b8c2fd93aea2"
  ],
  "./research/01-original-novena-docs/sources/gerbers-novena_pvt2_a.zip": [
    "file",
    "66664c73456ddf5458b1576709374402ca83238d820d747ccc40a3010207a319"
  ],
  "./research/01-original-novena-docs/sources/gerbers-novena_pvt2_a.zip.http-headers.txt": [
    "file",
    "55f7e8eca210462bc260d81607ede6fb3dd2c7adc93b3e9926c25d2deea3ed26"
  ],
  "./research/01-original-novena-docs/sources/kosagi-edp-adapter-evt.html": [
    "file",
    "1fd66b2bf3a9606c34272e950b06131cd080b1e43c8bf7a29ecbc1b31381df0d"
  ],
  "./research/01-original-novena-docs/sources/kosagi-edp-adapter-evt.http-headers.txt": [
    "file",
    "cbe9e3df80575a3f596d3d46a5f2f31fc714f3dc7f0b439969752c757088d447"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-main-page.html": [
    "file",
    "42e922769ef4d3b1c10a3d3b39081a0e636bedb0d529ecae562e8c1fe2f03640"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-main-page.http-headers.txt": [
    "file",
    "b0726c2805572a73711b61e0139ff08a9eec9fa7d56dd18d38952a8e9614f31e"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt-design-source.html": [
    "file",
    "b138095a4b0d9d1ad0086a97de3d886b01cbd85f83f37d3306adacc257fafbca"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt-design-source.http-headers.txt": [
    "file",
    "495c31bc460f757963531af19455c29468faff176c38c04b2ee060c3f5576d9b"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.html": [
    "file",
    "e84812983edf256a2ce9da5960805be4aa475736ce1c6865ebce7c42320b8589"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.http-headers.txt": [
    "file",
    "5bb311c474b809d4d4a2d790ee632cb63d8a403d6367e44e465823c42aad320c"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.html": [
    "file",
    "da71f18836cf1bea58cefbcc84202dda6f8b265692569142d355ca3a4117c5a9"
  ],
  "./research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.http-headers.txt": [
    "file",
    "11e12a24223baaf49d90c10b51ca18b4e74aef4ba70444036ba204722f40eb83"
  ],
  "./research/01-original-novena-docs/sources/kosagi-u-boot-novena.html": [
    "file",
    "88757655b9c1ea920941712099f3dd0b072ced6bd2d7b42adc4068111ec7804e"
  ],
  "./research/01-original-novena-docs/sources/kosagi-u-boot-novena.http-headers.txt": [
    "file",
    "359310bb527e77cb1117820e5430186f5d68c57a2e523bb25979b710a9ebb40c"
  ],
  "./research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.html": [
    "file",
    "354daa562880594e38348b4652c582af0719d92e86b662485524faa040e32f0a"
  ],
  "./research/01-original-novena-docs/sources/kosagi-u-boot-pvt-notes.http-headers.txt": [
    "file",
    "073164d84bd65b76a7bc7777f0feddfd80c257100bcd99ad9584b27a38686d29"
  ],
  "./research/01-original-novena-docs/sources/novena_pvt2.PDF": [
    "file",
    "1d4c1ecdf3399867b68c0b13959b30c7cc7c23d051de2aa2c87cd66652502951"
  ],
  "./research/01-original-novena-docs/sources/novena_pvt2.http-headers.txt": [
    "file",
    "861656c0472819b718aed5cec61312041eb193915244cdcae513fdfef7846ee6"
  ],
  "./research/01-original-novena-docs/sources/novena_pvt2.zip": [
    "file",
    "a8322aaa3147044ae3dbaf229de44980b92d9ec920f20d50e140769281ff5b4c"
  ],
  "./research/01-original-novena-docs/sources/novena_pvt2.zip.http-headers.txt": [
    "file",
    "3ee8e92ced880324f6f2721846a6608b874dc40747e9367169f11191db4d296a"
  ],
  "./research/02-historical-software/SHA256SUMS": [
    "file",
    "345d9f100fb5b86ea0d3ac8ab2071454035480e11993279f40513a0358e0e422"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-alte-analyse-provenienz.txt": [
    "file",
    "0241fa7e96bd181dfb8d9335a1675710e184c41a82acca178c9a3f7ea5a7fcc6"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.27a-vorhandene-git-historie.txt": [
    "file",
    "e5305ecc64d67b0913523bf67e43e0698b793c1a864d0206fd1b06ce514c348c"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.27b-upstream-v6.18-provenienz.txt": [
    "file",
    "c2faf873b1e51b5db96ce70ccccac56b6739fb010e7888022bf2350d3ae40a2a"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.27c-single-multi-master-analyse.txt": [
    "file",
    "4facc0f97a0634309d20390e7280ce3f454d2f676ba2b1ad5cf688caf2480257"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.29a-commit-landkarte.txt": [
    "file",
    "c6d088b091dbb69e6d07aea91e80042cfbb3b1012ae65b18ec3aa565b0ebcddc"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.29a-commits.txt": [
    "file",
    "337b2af6bcbb160e460cb82bd9fc4492126174af505a60594784791bca03ea4a"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.29b-kandidaten.txt": [
    "file",
    "5fc15527b135bf75cba03cd55290fab73850325f74a6da58f3125eacf91389f5"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.29b-klassifizierung.txt": [
    "file",
    "15cd432d8ef9427fa6790752f8216b42f01ac0c7252468561593d6109f486da3"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.29c-ial-serie.txt": [
    "file",
    "d62be25709b43d093729cf472d7cedb46ba1f86e218d1a7521507a6f93ced8ad"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.30a-runtime-pm-pinctrl.txt": [
    "file",
    "ce712c8fb5facd175fb889fea586d088be5beef8b951f18c635ca2b0c5fcfaab"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.30b-pm-pinctrl-semantik.txt": [
    "file",
    "2c608593496db6493deb2180ee7835c7884a01f3dea90bf3a1a31679565b5c8b"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.30c-exakte-pm-pinctrl-aenderungen.txt": [
    "file",
    "e35cc75fb631fa175833321e9bbf92ab706dcb2e8d0e623cf99e2b0a090c2000"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.31a-runtime-pm-commits.txt": [
    "file",
    "3e7bfc6ab561322f851545cb445ff324cb071b2a719cc3b8cb5127a8cb6337f4"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.31a-runtime-pm-historie.txt": [
    "file",
    "d29e54380f325da6f0dd49025057b821cfa656c9aa566c7b1b9d6f086ed96275"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.31b-runtime-pm-5.7-vs-6.18.txt": [
    "file",
    "44d38a2949892a7a185d3d548b13aa2247b5c87efb0a2615677bf0e18224d014"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.32a-register-lebenszyklus.txt": [
    "file",
    "1dfaf5e177e12ad77339e02a9b2a5c4cb1956379b0e6faf02689512e7c133ab9"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.32a-v5.7-rc2.c": [
    "file",
    "0f5f281102fc70cbf54e1dee07634d4b731a153771e0fdd2e51e0042fafcd467"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.32a-v6.18.c": [
    "file",
    "0c991419f06ea063a3af595ff07c2dda16ecafcefb5bf5af26167cfdb4243e21"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-step-2.32b-controller-lebenszyklus.txt": [
    "file",
    "a1f0d002489a1fc6f1163836400036800fb9924d8d7646c86b0decbe907e3edc"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-v5.7-rc2.c": [
    "file",
    "0f5f281102fc70cbf54e1dee07634d4b731a153771e0fdd2e51e0042fafcd467"
  ],
  "./research/02-historical-software/extracts/linux-i2c-imx-v6.18.c": [
    "file",
    "0c991419f06ea063a3af595ff07c2dda16ecafcefb5bf5af26167cfdb4243e21"
  ],
  "./research/02-historical-software/extracts/linux-upstream-1de67a3-i2c-imx-ial-after-every-byte.patch": [
    "file",
    "1cf7f5ae5c63d36177478bfc6523d0eb66ee9040ac75661d12bf902cfcc90799"
  ],
  "./research/02-historical-software/extracts/linux-upstream-358025a-i2c-imx-system-suspend-runtime-pm.patch": [
    "file",
    "8fce80902e8025611f1c100605ebec10f806dde6dcaa0614772b0ce618e8cb0f"
  ],
  "./research/02-historical-software/extracts/linux-upstream-384a956-i2c-imx-ial-reset.patch": [
    "file",
    "1d46ed0f8844aa97b8257ef8911bf46d3b215f0a0b337fcc2e86d3e1087f3dc6"
  ],
  "./research/02-historical-software/extracts/linux-upstream-576eba0-i2c-imx-pinctrl-power-state.patch": [
    "file",
    "bfb2b9b1f2e1322bb0d11cf7d57029c9cb99466e71e68164ff003589e1b3ec2a"
  ],
  "./research/02-historical-software/extracts/linux-upstream-61e6fe5-i2c-imx-no-stop-after-ial.patch": [
    "file",
    "c29e57776c9c482d8921a9c230d1225bf31836c81026bc25e4421b6c0d2e2cfe"
  ],
  "./research/02-historical-software/extracts/linux-upstream-6692694-i2c-imx-single-master.patch": [
    "file",
    "7655bf12bb48f7fe83e5f4d23110a33024c7d4da5a792c7f568cbcb9ffdc718d"
  ],
  "./research/02-historical-software/extracts/linux-upstream-768776d-i2c-imx-single-master-stop-fix.patch": [
    "file",
    "1c66bf3b0e7e8a7fc1ab2b0ea468fcd79bd9eee0e894ad437c90fdf6a89be5c0"
  ],
  "./research/02-historical-software/extracts/nixos-wiki-kosagi-novena-revision-21513-provenance.txt": [
    "file",
    "ba3e5f929ff2f78b9df36cacc2dc47f4c4c0412db12d695f8d3932cf272cade1"
  ],
  "./research/02-historical-software/extracts/nixos-wiki-kosagi-novena-revision-21513-relevant.txt": [
    "file",
    "7a39944b1b6b1d2ce03a659105c4bfd0db6fd281b0f1f56fb1ebaaa29d0214da"
  ],
  "./research/02-historical-software/extracts/novena-linux-5.7-vs-6.18.49-i2c-imx.diff": [
    "file",
    "6ea6b28444ff041a55de0489533e16d6352fc92957e5d5925ccee7342ad96698"
  ],
  "./research/02-historical-software/extracts/novena-linux-5.7-vs-6.18.49-step-2.25e.txt": [
    "file",
    "9456842cf1ac31269053108c1caaade77255953f52ab2b99e118debf54b33902"
  ],
  "./research/02-historical-software/extracts/novena-linux-5.7-vs-6.18.49-step-2.25f-multi-master.txt": [
    "file",
    "591fc451afe0c30b2e40a31b7bede350d68a2533b3a38ace40b902ec369ba066"
  ],
  "./research/02-historical-software/extracts/novena-next-linux-1fda06d-drivers-i2c-busses-i2c-imx.c": [
    "file",
    "0f5f281102fc70cbf54e1dee07634d4b731a153771e0fdd2e51e0042fafcd467"
  ],
  "./research/02-historical-software/extracts/novena-next-linux-1fda06d-step-2.25c-i2c-recovery-analysis.txt": [
    "file",
    "c4beadd1d6fc3d7411167a873d82d79f106e1f8777f1cb62cccbf0ff29076f6f"
  ],
  "./research/02-historical-software/extracts/novena-next-linux-1fda06d-vs-upstream-v5.7-rc2-i2c-analysis.txt": [
    "file",
    "3dd55e7f91a0468a86658a9cadad7c323238d91e043eaa5c736bbfebbe36e768"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-HEAD-it6251-dump-dptx-default.nix": [
    "file",
    "a2813a02dee3795f43c4fc93ee7dc3ac464e1a86c32cc39cd7c14f10235fd2cb"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-HEAD-kernel-default.nix": [
    "file",
    "d75e66561737caa5836edb5edea7f3aea1c4cffb48f2351d300dc11358d61225"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-HEAD-novena-eeprom-default.nix": [
    "file",
    "8dd9f095cf6433a779a7a87203abebbb76534eb6eccbf6f479144563b10b22fd"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-HEAD-novena-usb-hub-default.nix": [
    "file",
    "0e8c4a46537f399236baebb1317b63fdc5ba4701960db6c18c5c4b17e90aeb26"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-it6251-dump-dptx-history.patch": [
    "file",
    "f5a964580d4f240029eef2b6293b5145f43c5dafbc55ddb747fabee640cc7a2b"
  ],
  "./research/02-historical-software/extracts/novena-next-nixos-novena-kernel-history.patch": [
    "file",
    "0fcb913e44bcd7e1e3ff713be7cdaec112f80f21770c1b4c6534ae76c1f23209"
  ],
  "./research/02-historical-software/extracts/xobs-it6251-dump-dptx-70b07ef-it6251-dump-dptx.c": [
    "file",
    "faa8697a47185ad214c460e0eef19c6d34909ab823ff3d473fa44a1900806e13"
  ],
  "./research/02-historical-software/extracts/xobs-it6251-dump-dptx-70b07ef-provenance.txt": [
    "file",
    "99326868d7ac1b6947173f544c54d519bcff893428f279329eecaabc27e9cc26"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-cmd-i2c-after.txt": [
    "file",
    "ccc1afb064ea328df6c0783ac126c3f3e4dd0e16e69e36a87ea93293b80828f3"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-cmd-i2c-diff.txt": [
    "file",
    "002a8265205301687f29d0c2aa084f252f4e40d0b6214325827ae568b9f12e92"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-it6251-check.patch": [
    "file",
    "6eed98f43598d41f0566c2aefb76afdf29a0ec6f36b4f34a6b563642789d63cd"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-novena-h-after.txt": [
    "file",
    "1db549e05075982badb5042ffe0b9b820c28bae688bfaca08b04aa5916b145f3"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-novena-spl-after.txt": [
    "file",
    "67ae574f7c9879ef7c6f9fa67741fd4b3fc805274b0a201e52cd171dabcc2681"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-afe5956-spl-to-it6251-path.txt": [
    "file",
    "85d6b72cbd627b9e78c33f3a6cd76cf324384947f17b69e1cd2f521efc8ccf0f"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-all-novena-i2c-accesses.txt": [
    "file",
    "0a989961b39fb7480f9a49c26c249a0525a8a2d978a2af3da2362bc0a95173bf"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-audio-power-full-tree.txt": [
    "file",
    "3e68ea9bf2fb430eeec0d5dfe2628f41282dfddb4a27767e21881f6489efc332"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-board-files.txt": [
    "file",
    "03891b14dc6569c748da4eb4cbb2bd9f98e3554331300d30cfc815f9d72ef058"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-board-i2c-search.txt": [
    "file",
    "8241548bc4a1db82e76d1ee37e7e4ad32b1655e75105cf4ef7b5d112fea82926"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-bus-i2c-init-locations.txt": [
    "file",
    "a9598b3e9193c436063f465c23caaa6e76f53d35f8930c34d3eeeb6dd916aa79"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-file-inventory.txt": [
    "file",
    "92e6446134120ec052367e8e2d5195184616c373d29e6e13aecae79705800ce1"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-fpga-gpio57-search.txt": [
    "file",
    "8f01fc1fb3271dacb01600b35ba7d684c7e579b92ac1d02f1b18074f768b2415"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-fpga-reset-full-tree.txt": [
    "file",
    "fba61817b2a84d2f9a3772b48a7ff825c18eac72659a9a61e7334963659d6fb7"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-fpga-reset-search.txt": [
    "file",
    "fba61817b2a84d2f9a3772b48a7ff825c18eac72659a9a61e7334963659d6fb7"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-gpio5-07-17-pad-uses.txt": [
    "file",
    "d809a3fe94abb103b1b884433742dc7cbb3769e13b93b6ae2456247c8050360c"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-bus-adap-definitions.txt": [
    "file",
    "54d38d0464ec3e07d88f61402dca7646d12c5e2e5213e811cacbe9d8edf8155e"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-callchain-analysis.log": [
    "file",
    "0d59015db572f7a5e1fb2fe1bb82b09713fcedcf396cf14bd1c7a7826c5b6b54"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-config-search.txt": [
    "file",
    "8855d2ee2a1fa4a916a32f502b70a02aa1564118d583375f8b38e3c01fae6f90"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-core-bus-context.txt": [
    "file",
    "205003f61ad16db698f6c3976f0fcc4c43e75d855d6a9ceb3267dde92a80fc91"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-core-context.txt": [
    "file",
    "df033b970c72f80bec8338813296ec45549f86db11437952b59d09b5eebfe860"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-core-selection.txt": [
    "file",
    "12a17a52fd61e72408e0c900dcd37b1867fef960dea1f20b61146afa563acbfd"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-init-all-calls.txt": [
    "file",
    "2e976b4e9418446efbfc95d12369a5e209671e1fd2cb3ba00fdc36242875a53e"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-init-definitions.txt": [
    "file",
    "a8419b98b0aa73121722895660e8e2d62618f2ee789c434123f7452274f69e69"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-mxv7-numbered.txt": [
    "file",
    "4692ffbbe99fc5beabd00d57ae82b845b17522ad3cbbe1b2044c4c6ad93d4c54"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-search.txt": [
    "file",
    "9441b6806cd4b9a10ebed72b03972fb6088d9e29f0621d23d047c459b4290f85"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c-startup-config.txt": [
    "file",
    "732e96f66e8aba9337f8d512d304e71521b9533d208d7b01837bda34f0ec4492"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-i2c3-pad-search.txt": [
    "file",
    "dda9a151d905c953d0c98cc6af27867cb4330ea5569c3e74f7b6b9e3c9d0bd3c"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-main-call-sites.txt": [
    "file",
    "0c29471876f8efaae9c8620b8201e1510e407961b884c450f837a127778c0451"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-main-eeprom-context.txt": [
    "file",
    "d69f3454c2bbd33694fa9bf5e70b90c3ada4f3d83a54f6d78f446dfc8b7b5d6d"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-main-iomux-search.txt": [
    "file",
    "d3dd7e7aac55f9ce3989b7a16267d94999a2d8d1bd5bc5f457d17a606d612be0"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-main-video-context.txt": [
    "file",
    "74f7bd4b4cf029c3912e8f7f9b9574c5eaafb2f3f94bf2d85ed06d8d54742ef8"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-mxc-i2c-adapters.txt": [
    "file",
    "7f2f2247976fd6ccb04209cc98a35cb708e40b32fae2d146b0de37de9b947217"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-mxc-i2c-init-context.txt": [
    "file",
    "bcfd5075c22d5a11d567013b85886fdadbf766d58c1fe360f4ddb50f0cab5822"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-mxc-i2c-transfer-context.txt": [
    "file",
    "47920d48f4d3ebe41c05753ed046a14e5b59dd6ec527e602526c1f680c05749d"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-mxc-i2c-transfer-key-lines.txt": [
    "file",
    "737187648af45824f6d0194953e7699eebf86580db68993303e7c1c380cc4460"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-novena-disp0-dat13-search.txt": [
    "file",
    "c72a2fc2f4d9d189c8f7ee7fcf724a3e225198ac5df50fe2799325411b4c845d"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-novena-disp0-dat23-search.txt": [
    "file",
    "e4155558a89011e7814f2a27ea25f927d4920f6e73c93bb15bd4667e70c61ac4"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-novena-fpga-audio-sequence.txt": [
    "file",
    "e8059c8f9cc4b1cc7a0d47b5fe9b0438e694f8298e224df44352a7d3dbf4151a"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-novena-gpio-writes.txt": [
    "file",
    "e0e99b2d5be50b2bed3a33aa63fe4cca812712f8c122dfb5d5f6cc9713474b06"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-novena-paths.txt": [
    "file",
    "67e2de091ea968aa7e7f213f24b733aca08d846b396840a0769670575e3f5eaa"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-rc7-rc12-por-i2c-state-comparison.txt": [
    "file",
    "d7a09cff739ac341b074652251b9f17018d347a4643d06dbd2f2c377e042fd00"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-relevant-files.txt": [
    "file",
    "4fc13a485dab60baa87468f7d3f01b196563f2dbd97b0b9b943bbcf0f7b5f231"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-setup-i2c-locations.txt": [
    "file",
    "3ef3a75a04dbd04e88ea0e6f29f3eebc64356b9cad9d82b4eb83276cb3aa4e12"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-source-sha256.txt": [
    "file",
    "1659c74f8d81a45ab7e262556eb7391d5cb886c6c7b59f4d747bd987e60c6ba2"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spd-bus-config-uses.txt": [
    "file",
    "1a4a0bc7045d7c87020355ee54dad2a7bd7e275e11bceccf6b5f75fd77c50382"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spd-i2c-context.txt": [
    "file",
    "d0f8e9e116c9d0e6d7ae3d0b43df394ab033bd99cde2697d4bb291fb39674f61"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spd-search.txt": [
    "file",
    "28856741c796ac3a2005c4b255d1ea88e4d2ea49939b5a98a3f4b61bbe0dae90"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-call-sites.txt": [
    "file",
    "677ee06b98f14b140c8e7ee7a50820ec3ccfbace7407ef71412d36e91bc295b2"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-display-pad-search.txt": [
    "file",
    "9f1c0fdac03b83b77027a9c9e0fe2b256ff65c81ff5638acf873bb2afd3bdf47"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-i2c-context.txt": [
    "file",
    "77fe3e95899474299e62c8001b640d0aebbbd5a16df23d7e9e654bc347095e71"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-i2c-init-callchain.txt": [
    "file",
    "7ff3ff8c966931cc8d861a43f378189f995961a4ac25db1b4bca2abfc8090a7e"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-init-context.txt": [
    "file",
    "c7294579eb27ea4b6b0d6753163a0d03f30bfa8a3da89b2977ab197fd97823e7"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-spl-video-context.txt": [
    "file",
    "3643addf5d542806b91ad13d131fc8ca757f730343985643975b1f96577ccdd6"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-to-rc7-commit-details.txt": [
    "file",
    "7667173fbd83e4f1f2e5c2f33f099922767669efb58838f5eb3c73e482f85ce9"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-to-rc7-commits.txt": [
    "file",
    "5a0e62a2adf1b7fb5804d331fbf8b58ba0db1365da4e713c51090c20910b3b3e"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-to-rc7-novena-files.txt": [
    "file",
    "60a72824d9d801ead84a215d27f68c94c3330a831c35ffda8d543cfb423a78af"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-to-rc7-novena.diff": [
    "file",
    "187bc7c37d5ebbdaeaef34c22a5c6dc0093f1f4bc3f1ab0c4f96660f3a523ca5"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc5-to-rc7-per-commit-relevant.txt": [
    "file",
    "774b994a3e837e7d9f73c931cbc67da34791590aadf8d0fa5e48daf725eb7b61"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc7-to-rc12-commit-details.txt": [
    "file",
    "d65a2038a7a20b9740b597a932db010bf5bae63b527f53ca0b806bdfdd91c957"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc7-to-rc12-commits.txt": [
    "file",
    "25c899ee7dbe96feb9aab9e11518c4c8dbdcc5090afd773b404a09a997ec64bc"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc7-to-rc12-novena-files.txt": [
    "file",
    "1bd24104d4fb3c3a53db5a80b104a939522c6d8b79b000c72250ecf94b001600"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc7-to-rc12-novena.diff": [
    "file",
    "aa7ce80dd00f28cdf2cd57004dc5fea267970f1804c6c25e8b2610b6d14f9dec"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-rc7-to-rc12-per-commit-relevant.txt": [
    "file",
    "71576b19ee5fd8f5c8ec83d11d99f0da9247b7be0088c848f42b7d3a47f2b69c"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-repository-identity.txt": [
    "file",
    "4331ed55bd88cd0868848ac2e96bbcde4f27a201355e4559dffa6c466e815ae4"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-tag-history.txt": [
    "file",
    "8ffd4a112c199d8d06c4c4e8c4787be2245fe37c8ab06007bd9ca7a010823432"
  ],
  "./research/02-historical-software/extracts/xobs-u-boot-novena-v2014.10-novena-rc5-provenance.txt": [
    "file",
    "e77400d8bbfe8c167be049bf6668dff84ecb7378bc519f4567da71d5a7719a3d"
  ],
  "./research/02-historical-software/extracts/quellenuebergreifend/novena-next-linux-dts-power-policy-matrix.txt": [
    "file",
    "6d36235eb40eb6058f6f184e9eaf79a5b45a8a05fc4834daff2231bae6eda550"
  ],
  "./research/02-historical-software/extracts/quellenuebergreifend/novena-next-linux-es8328-i2c3-wechselwirkung.txt": [
    "file",
    "35bbe085ce6c199f15596e0edaa5106e10b1ef1db4a8a416491e0437b27e19a4"
  ],
  "./research/02-historical-software/extracts/quellenuebergreifend/xobs-novena-linux-it6251-powerup-historie.txt": [
    "file",
    "71eea99b3b18d4ed86f6c46f76c0962e740233ee50c5c4ac3679318530481db8"
  ],
  "./research/02-historical-software/extracts/quellenuebergreifend/xobs-u-boot-novena-it6251-power-ready-sequenz.txt": [
    "file",
    "7d39dcfba3486d7a712244fcdc8359d440c336bd33e65e67cd03b99a05d3257e"
  ],
  "./research/02-historical-software/notes/linux-i2c-imx-5.7-vs-6.18-analyse.md": [
    "file",
    "6d4598f498a02d6db1d8c3487a293313c99320cdff6349697e46b3b02adf82d0"
  ],
  "./research/02-historical-software/notes/nixos-wiki-kosagi-novena-revision-21513.md": [
    "file",
    "e97885ffd6f3f645ab051b7591abda7019c2afed866437d2fb506b20be66aea3"
  ],
  "./research/02-historical-software/notes/quellenuebergreifende-novena-sichtung.md": [
    "file",
    "16b30d6b7990d2e98a854339c5107e994416efb93e91ed9de7e1c53c95cce7e4"
  ],
  "./research/02-historical-software/notes/u-boot-v2014.10-novena-rc5-i2c-analysis.md": [
    "file",
    "aabfd8faf5401dd355a1a854d528df12fe115b8b14973a36a22f9bfcc7c2370c"
  ],
  "./research/02-historical-software/notes/xobs-it6251-dump-dptx-70b07ef-analysis.md": [
    "file",
    "2d8d27f092cf26f3307f022901959184e328b0f2b0b6ef9107eca5dee12980c3"
  ],
  "./research/02-historical-software/sources/nixos-wiki-kosagi-novena-revision-21513.html": [
    "file",
    "ab3855d4dbc0190a2d43cf1c7ec5855ba70db5f5da8eab76b1aeecf39a5ac2b6"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-ls-remote.txt": [
    "file",
    "40fb9cb3000c844ca13c504443b60bfb1bc1f72edfb9a916101505c2cff3b967"
  ],
  "./research/02-historical-software/sources/kosagi-hardware-history/Novena_EVT_to_DVT_changes.html": [
    "file",
    "98626b8de199d68449c86225994802ed302ea03f7335153199bb12e5954bcb42"
  ],
  "./research/02-historical-software/sources/kosagi-hardware-history/Novena_EVT_to_DVT_changes.http-headers.txt": [
    "file",
    "5a8ea15b5ebe6eed1fae6e950392b006e92af25c6cb6641880d645000dbc3198"
  ],
  "./research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.html": [
    "file",
    "02a2314f7b3548d625b75719815e9d05c5f04012cb5173702abb8a253920d6d4"
  ],
  "./research/02-historical-software/sources/kosagi-hardware-history/Novena_Issue_Log.http-headers.txt": [
    "file",
    "a5ef0edc4e94c5d0200f7bfd8131d3372a29a499e658617a9b87bb6c47408c2a"
  ],
  "./research/02-historical-software/sources/kosagi-hardware-history/README.md": [
    "file",
    "07178cefec000e82ac3a437fa503665ccc66505d6fe7d01dcd661cda02876634"
  ],
  "./research/02-historical-software/sources/xobs-it6251-dump-dptx/xobs-it6251-dump-dptx.bundle": [
    "file",
    "1c2e391127d62d3ae6951ad94774d8b092312759ae25c334de15b125d70971ec"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/arch/arm/imx-common/i2c-mxv7.c": [
    "file",
    "3810bb22d515b883dfc5166fb95c72762ca175db41af403c0af085628aec027d"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/arch/arm/include/asm/imx-common/mxc_i2c.h": [
    "file",
    "0618c13da0e99b5c1d9ca7f6d7673a6fa7ecf57da002cf828db67a15581ae545"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/Kconfig": [
    "file",
    "9a961d3832988cb881282c543a7ccef244dc7eebed2cd103914b0839b9d7f4af"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/Makefile": [
    "file",
    "d12de3e0318583f6612fce5f7303749aff61cd640d74535459eb5b5a96673489"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/ddr3.c": [
    "file",
    "f75834972afdb596cf885db4d97c69623cb9dc28738dc9cc1615d45e1bacc945"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/novena.c": [
    "file",
    "ffb30695edd8efd83d0813f2d0dc396f4e2a9eb0f0f336d0a48674d20332098c"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/novena_spl.c": [
    "file",
    "47297c6c439f302d08125237a7bbb9a47ce26628145f4e707c892b07f59b1413"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/board/kosagi/novena/setup.cfg": [
    "file",
    "959136b9f8adcd7745564dad0b8310e124c6e13bfbb7f9728ac98be44db6d427"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/drivers/i2c/i2c_core.c": [
    "file",
    "d92d4f8d6ae176027ba27e23d65a0b7b794ba598371a33f56ab04dc9c2ec8c02"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/drivers/i2c/mxc_i2c.c": [
    "file",
    "a2a40d89ecb5cb76b67b07abcfbfbbed6b711c29fb344c342bef1764fb172748"
  ],
  "./research/02-historical-software/sources/xobs-u-boot-novena-v2014.10-novena-rc5/include/configs/novena.h": [
    "file",
    "7de9fea49866a6c768498eda29ede2f55c18bd8357eaaec7c91bd7bb34f64dd0"
  ],
  "./research/03-root-cause-synthesis/block-3.12-root-cause-kandidatenmatrix.md": [
    "file",
    "4ea087452f81e9a45e72d717ef9807407c9287fe7bdbaa920b3c91f74e6924ef"
  ],
  "./research/03-root-cause-synthesis/block-3.13-elektrische-messplanung.md": [
    "file",
    "f146914481482781e0855d75692b34d5eb8135f319a3f302c17c089aea41c460"
  ],
  "./research/03-root-cause-synthesis/block-3.13e-quellenbasierte-eingrenzung.md": [
    "file",
    "f0eb5b156e07c630191dce272f713bc3a4da39c53030819cd459d02e9302afae"
  ],
  "./research/03-root-cause-synthesis/block-3.13f-primaerquellen-eingrenzung.md": [
    "file",
    "b124402ae7340abae8e62265b168f9384fa73332aa12719ae4d57394b0d6f441"
  ],
  "./research/03-root-cause-synthesis/block-3.14a-power-i2c3-zustandsrekonstruktion.md": [
    "file",
    "003ebd274ebf54bfd503f37c34ec262da6cb660a4b2f77664e571a50814e6c2c"
  ],
  "./research/03-root-cause-synthesis/block-3.14b-controller-start-zustandsrekonstruktion.md": [
    "file",
    "ac1df473441a28643b5f86827e9f9dc9324e6461902880271b59e6a0f9b0fbd5"
  ],
  "./research/03-root-cause-synthesis/block-3.14c-imx6-start-arbitration-semantik.md": [
    "file",
    "37db0cc70fd9901c6613fdfc4c0588f77ece2ecacef7c124e1c1440a356777e6"
  ],
  "./research/03-root-cause-synthesis/block-3.15w-es8328-i2c3-historische-root-cause.md": [
    "file",
    "8852e7db4f0df5850595e3f9ae28be75d0ece13ddf1206767ba755188cb751ce"
  ],
  "./research/04-spl-ddr/block-9c-43-dokumentationscheckpoint.md": [
    "file",
    "8a21dac74ab0a6d747ae6d875a797d84e3ef33df34a6a98d7faa6af7c7eaa0fd"
  ],
  "./research/05-gesamtanalyse/00-LESEANLEITUNG.md": [
    "file",
    "da965171f2452d77539957e6e49752dee1b662dc6e8338d09cca31fed8f5be40"
  ],
  "./research/05-gesamtanalyse/01-PROJEKTGESCHICHTE.md": [
    "file",
    "abd43b0ce081415f0ddcd17a920e7fc67bd497013eaec0c1f98e333991918081"
  ],
  "./research/05-gesamtanalyse/02-LOKALE-QUELLEN.md": [
    "file",
    "aca464fa802a20f2625e637b920dcdcee078be9ae7ce29a73de53bc018fbdd4a"
  ],
  "./research/05-gesamtanalyse/03-QUELLENVERZEICHNIS-AUDIT.md": [
    "file",
    "de8181531ced5c4f2c8e69964b276e1d5faf0bf7557c63db2658c46849ddef5e"
  ],
  "./research/05-gesamtanalyse/04-QUELLCODE-UND-BOOTPFAD.md": [
    "file",
    "f9931e2c5bcaa78364598a2e5fc3729571466c5eaaa047bb1312ff43dd47b43d"
  ],
  "./research/05-gesamtanalyse/05-TESTLOG-ANALYSE.md": [
    "file",
    "2abceefe8264c2c040106bc98e434ee480e53c0c4de517e73c500b023ef5eb01"
  ],
  "./research/05-gesamtanalyse/06-FEHLER-UND-RISIKEN.md": [
    "file",
    "465b1e65efc28aa68848edb3f1fe9a29b9e2f71727ce2192165e3bb878ef1d67"
  ],
  "./research/05-gesamtanalyse/07-HYPOTHESEN.md": [
    "file",
    "cda10f9281a6b5d5b539c1849af72b09eca3baa5f5eba062859461238abf7786"
  ],
  "./research/05-gesamtanalyse/08-EXTERNE-RECHERCHEFRAGEN.md": [
    "file",
    "b7727415fc6e16bbb31016ce965001861aba91c326648c81b6fde6fd53e836de"
  ],
  "./research/05-gesamtanalyse/09-UNTERSUCHUNGSPLAN.md": [
    "file",
    "94b63f9ec54d6ea3489bf86823ddd14106c2d44d607aea709101e9a11db1a11b"
  ],
  "./research/05-gesamtanalyse/10-OFFENE-FRAGEN.md": [
    "file",
    "4eb345d578ce2c23c3cd2f8e3bd7fd8cc8c06972b34b9ab277dbd5cc7e11898e"
  ],
  "./research/05-gesamtanalyse/11-ARCHIV-DATEIAUDIT.md": [
    "file",
    "3cdd78e101674931abac362dc4341bfae656118a9a609cc0d2ef960f3c9862eb"
  ],
  "./research/05-gesamtanalyse/12-URL-FUNDSTELLEN.md": [
    "file",
    "08eff2db92d78db3c2016bd8b1f1ac3aff2a8f4efa834183cf6be3a7574c9af2"
  ],
  "./research/05-gesamtanalyse/13-ROHLOG-UND-EVIDENZREGISTER.md": [
    "file",
    "0e47373c6525140e29606bd03362cc59b031059ecb27b6c8200953c5bdaafa57"
  ],
  "./research/05-gesamtanalyse/14-HISTORISCHE-GIT-BELEGE.md": [
    "file",
    "2e7f12d6d62315739dc74b4b08126d8821855e036a1658824d837854f12b898f"
  ],
  "./research/05-gesamtanalyse/15-AUSGANGSCHECKPOINT.json": [
    "file",
    "ce053884d849a260c5ac8b3de9ef99880d2ceb151506c5d13fb77f4d34d8abe7"
  ],
  "./research/05-gesamtanalyse/16-ABSCHLUSSPRUEFUNG.md": [
    "file",
    "838415184ae2ec44b3527ef068ad60ed46efefdd80788c6e06ad2c1bb91cec43"
  ],
  "./research/05-gesamtanalyse/16-NXP-MMDC-PRIMAERQUELLEN.md": [
    "file",
    "68db54fa5d0d38faf6da234c0ba826592bf7e022c446c1072e1c98128403da6e"
  ],
  "./research/05-gesamtanalyse/17-NXP-ERRATA.md": [
    "file",
    "76e1f2a0990117ba8f7286fa79d2126813e6ffb9b71e032025a59d29fa768ab0"
  ],
  "./research/05-gesamtanalyse/17-VERTIEFENDE-PRUEFGRENZEN.md": [
    "file",
    "279d66bde6324f95b6ad11fee26c6052ebe8e4303db1fceadd5e43a76cc6ceed"
  ],
  "./research/05-gesamtanalyse/18-U-BOOT-DDR-HISTORIE.md": [
    "file",
    "ec86efe56a0f2b6aa396e710ad768232eb9a4ace4024835d218d7408f4e4e912"
  ],
  "./research/05-gesamtanalyse/19-KOSAGI-DDR-HARDWARE.md": [
    "file",
    "df953a73a702c2c5f36922f84eb62b79fdf0583759454036dbcd2e4ee1035f80"
  ],
  "./research/05-gesamtanalyse/20-EXTERNE-QUELLENMATRIX.md": [
    "file",
    "cf49cd70f4cabb27d99d0f4df3e6f3c58602c8ef8e129aae0b64c3eeebf91f75"
  ],
  "./research/05-gesamtanalyse/21-EXTERNE-RECHERCHE-LUECKEN.md": [
    "file",
    "db1a835ffabadeeac20da28f53a7f21e0a618ecfb6a266a5a0383fef95ee590f"
  ],
  "./test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log": [
    "file",
    "45fb5e00d5ba7be6a42afc7491522cb8bcc084fcd8142582cbeea4ddbb39595a"
  ],
  "./test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log": [
    "file",
    "e4b11db1cba820097f55bcbd6b6076577d35c66645317f5668ee716a5fbfd7c0"
  ],
  "./test-logs/2026-10-01/block-4.10o-1n-8e-decisions.entwurf.md": [
    "file",
    "8d01cc432ea751f2a0bdb1483d87dc3908c2ebf12bdd88a94228c2dfcf4a8572"
  ],
  "./test-logs/2026-10-01/block-4.10o-1n-8e-project-state.entwurf.md": [
    "file",
    "a016c035477397b680e52c3361490207cfa41d18e0601c2e95e3beccec0dfb86"
  ],
  "./test-logs/2026-10-01/block-4.10o-1n-8e-test-log.entwurf.md": [
    "file",
    "d31fd92d830c19bb3b373e83c1b5d4afd6880f6c952472304ab091fd52ca4028"
  ],
  "./test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log": [
    "file",
    "9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61"
  ],
  "./test-logs/2026-10-01/decisions.vor-8e.md": [
    "file",
    "aebafda36eb679d7a5fea35a9cf2248c3ae6028ba2a63825f610047d33a44e0b"
  ],
  "./test-logs/2026-10-01/project-state.vor-8e.md": [
    "file",
    "a79bb72207d5f6eb04783f6acad7fa2fa42d7d29a2e65027b186caba79336ff3"
  ],
  "./test-logs/2026-10-01/test-log.vor-8e.md": [
    "file",
    "68edd388f394ff89164b95a6ed1a7dfad682ee4dacf250b9bb022ec328ba5796"
  ],
  "./test-logs/p-ext-2026-09-22/p-ext-boot-01.log": [
    "file",
    "506f8eaf409b8d9a6e40fced1da822df43b865df507d08d094d9106046e7debb"
  ],
  "./test-logs/p-ext-2026-09-22/p-ext-diag-boot-01.log": [
    "file",
    "4773f008e9dda2c0c6cfbf5695a73877b60178bb45ba2c755a2a431921dabc41"
  ],
  "./test-logs/p-ext-2026-09-23/block-9h-build.log": [
    "file",
    "3cc38822bf8378ca1b7d3b6ed1d1f0005e3a482309ce70e6f1df290b0cb89089"
  ],
  "./test-logs/p-ext-2026-09-23/p-ext-d0-d36-boot-01.log": [
    "file",
    "28b9b3c79b98e8f9d8548fa6c48755973fa5ea6ce61407a44e0c25bc85d65c67"
  ],
  "./test-logs/p-ext-2026-09-23/p-ext-d21a-d21d-boot-01.log": [
    "file",
    "f123e33b91abdc530a8c3fc440f59d41ece1f1f16b90815f067a97b999466c95"
  ],
  "./test-logs/p-ext-2026-09-23/p-ext-p0-p1-boot-01.log": [
    "file",
    "3f732abfdde8122068752c13aa2d6a6418e926ec89836566e683345e0743507e"
  ],
  "./test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log": [
    "file",
    "5fa66b25959c862e7b985022a1c76037b3668942fa10741f8b3cd7754ba25d54"
  ]
}
```

## Ergänzung vor dem ersten Schreibzugriff: drei Verzeichnissymlinks

Diese Einträge wurden nicht als während des Auftrags neu angelegte Dateien interpretiert. Sie wurden zusätzlich vor den ersten eigenen Schreibvorgängen erfasst und anschließend in die Vergleichsmenge aufgenommen:

```json
{
  "./result-kernel-0004": [
    "link",
    "/nix/store/9g9pdbrbc4khk64xgiln3w23slx8aivk-linux-armv7l-unknown-linux-gnueabihf-6.18.49"
  ],
  "./result-kernel": [
    "link",
    "/nix/store/963ddhz2d6v5cq1n4m0nrnjdrq6hck5k-linux-armv7l-unknown-linux-gnueabihf-6.18.49"
  ],
  "./result-kernel-0005": [
    "link",
    "/nix/store/97pmqlp5xvsh1l0i877lfc4r4sq6i7np-linux-armv7l-unknown-linux-gnueabihf-6.18.49"
  ]
}
```

## Verbindlicher Abschlussstatus

```text
ROOT_CAUSE=UNRESOLVED
IMPLEMENTATION_ALLOWED=NO
BUILD_AUTHORIZED=NO
HARDWARE_TEST_AUTHORIZED=NO
PATCH_0008_SECOND_BOOT_ALLOWED=NO
PUSH_AUTHORIZED=NO
```
