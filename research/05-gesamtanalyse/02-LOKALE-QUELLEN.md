Stand: 2026-10-03. BLOCK 9C-46A. Ausschließlich lokale Analyse; keine neue externe Verifikation, Implementierung, Builds, Medienzugriffe oder Hardwaretests.


# Lokale Quellen und ihre Identität

## Maßgebliche Ebenen

| Ebene | Quellenart | Tragfähige Aussage | Grenze |
|---|---|---|---|
| Serielle Rohlogs | ORIGINALQUELLE | Erhaltene Ausgaben und deren Reihenfolge | POR, Mediumhash und Versuchsanzahl nicht aus Bytes allein |
| Buildbaum REF | ORIGINALQUELLE für statischen Bytebestand | Gepatchter Quellcode, Konfiguration, Map, ELF | Historischer Buildablauf und physische Registerzustände nicht bewiesen |
| research/sources | LOKALE_KOPIE | Lokal gespeicherter historischer Inhalt | Onlineerreichbarkeit und aktuelle Authentizität nicht neu geprüft |
| research/extracts | REKONSTRUKTION bzw. abgeleitete lokale Kopie | Extrahierter Inhalt mit Bezug zur Archivquelle | Auswahl und Extraktionsfehler möglich |
| Projektberichte/notes | REKONSTRUKTION; teilweise SEKUNDAERQUELLE | Dokumentierter Ablauf und Interpretation | Fehlende Primärausgaben bleiben fehlend |
| Chat-/Auftragsangaben | NICHT_VERIFIZIERT | Überlieferte Zuordnung | Kein lokales Originalbericht-Gate |

## Gepinnte aktuelle Grundlagen

Nixpkgs: `a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4`, NAR-Hash im Lockfile (`flake.lock:9`). U-Boot: `ece349ade2973e220f524ce59e59711cc919263f`, fetch-Hash und Patchfolge in `boot/u-boot/default.nix`. Working-Tree-Nix enthält acht Patches (`boot/u-boot/default.nix:19`). Quellen- und Versionsstrings sind unterschiedlich starke Identitätsmerkmale: die festgesetzte Localversion enthält f8baca04e22d, obwohl spätere Diagnosepatches aktiv sind (`boot/u-boot/default.nix:31`). Das Banner allein identifiziert keinen Patchstand.

REF bedeutet exakt:

`/nix/store/z92l810x94i1fgqbmhqja67gyj0lf29g-uboot-novena_defconfig-locked-debug-tree-armv7l-unknown-linux-gnueabihf-armv7l-unknown-linux-gnueabihf-2026.07/build-tree`

Heute erneut lokal gehasht:

| Artefakt | Bytegröße | SHA256 |
|---|---:|---|
| ELF spl/u-boot-spl | 937916 | 69bb3a4dd2efe059cefc197cad244f13459b0ef08ccb17c886059337ca25e469 |
| SPL-Wrapper | 56320 | 4611111bf2412257bf537a0c3b95a79b796d7359a8ff4de54046cbe2fe56a25c |
| Raw-SPL | 49480 | 42b47045d5aa9cde5e7ec04c6579ececed596cd36643961f7e4ae66f96c307f3 |
| Map | 227195 | 0fcf601af9d23c6997bd1cbd944fa56f580ed86dc34099fda89ce0ce26d1bf54 |
| .config | 53032 | 48f2371872392021f1caaba6590fc7c4344b115ee117ca1386925f427da6eddb |

BELEGT: Übereinstimmung mit dem 9C-44-Checkpoint. Keine neue Build- oder Medienqualifikation. Quellhash Board: `ef3252c640f2b4c2dbc52b1c3b0c4fe0f0056a4cb95033e4839109e63b09474f`; DDR-Helfer: `2c2c41d7c0c3ce07f73f4e20f405b17b0cd8bbda37505be124e597d696182c95`.

## Historische lokale Git-Bäume

Heute lesend geprüft, ohne Lazy Fetch:

| Pfad | HEAD | Shallow | Arbeitskopie |
|---|---|---|---|
| /home/loomit/novena-historical-research/u-boot-novena | da65dfe4eb70bda1fac5238718c7c8ef763af3bd | nein | sauber |
| /home/loomit/novena-historical-research/novena-image | f62c4f3b452199ab882e2c3dba16d4e31dfb1d56 | nein | sauber |
| /home/loomit/novena-historical-research/novena-debian-installer | ebdb816b93f91bf32809a8c4c1454d4ef7f2bb5f | nein | sauber |
| /home/loomit/novena-linux | 1fda06deecb61538ca3d07d256eb7c43d4e3432a | ja | sauber |
| /home/loomit/novena-linux-old | d9d2e8b619f17e8394d62c08d60b4ea17154e9d6 | nein | sauber |
| /home/loomit/novena-research/u-boot-current | 211de43d0f954a00a490220c1aac9db298287c40 | nein | sauber |

Sauber/nicht shallow ist keine Garantie vollständig vorhandener Remote-Historie. /novena-research enthält zusätzlich v2020.07, v2026.07-Experimente und Buildbäume. Diese wurden lokal inventarisiert, aber nicht sämtlich zeilenweise verglichen; Namen sind keine qualifizierte Identität. Das separate allgemeine `~/novena-system` bleibt eigenständiges Projekt, ebenso frühere `nixos-novena*`-Ansätze. Keine L14-Befunde werden übernommen.

## Archivvollständigkeit

Vorbestand research: 209 reguläre Dateien, 58 Einträge im Hardwaremanifest und 134 im Softwaremanifest. Sämtliche 192 Einträge stimmen heute lokal mit dem jeweiligen SHA256 überein (11). Die übrigen Dateien sind nicht durch diese beiden Manifeste abgedeckt; 11 bewahrt aktuelle eigene Hashes, ohne sie rückwirkend als historische Manifestprüfung zu behandeln.

Die beiden Forschungswurzeln haben keine README.md auf Wurzelebene. Orientierung kommt aus Notes, Provenienzentwürfen, Manifesten und Projektberichten. Das ist eine Navigationslücke, kein fehlender Quellbestand.

QUELLENLUECKE: originaler Herstellerbeleg zu ERR050070 und belegte MPDGHWST-Lesesemantik; Originalberichte 9C-33–42 und Gate-Primärausgaben 4.10O-1L-3C/3C-R1. Der genaue Bytebestand des veröffentlichten r1-Images ist nicht aus dem lokalen Hybridmedium abzuleiten (`docs/HISTORICAL-MEDIA.md:334`). Barebox-Vergleich wurde nicht als lokaler vollständiger Quellstand nachgewiesen.
