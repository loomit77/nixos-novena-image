# Boost 1.89.0 – Reproduzierbarkeit

Datum: 2026-10-23

## Ausgangslage

Die ursprüngliche Boost-Derivation für ARMv7 erzeugte auf
foobox und VM102 unterschiedliche NAR-Hashes.

Ursache war die automatische Erkennung der optionalen
Boost.Stacktrace-Addr2line-Unterstützung.

- foobox: boost.stacktrace.addr2line = yes
- VM102: boost.stacktrace.addr2line = no

## Korrektur

Ein Overlay deaktiviert die automatische Addr2line-Erkennung
für Cross-Builds ausdrücklich:

    boost.stacktrace.addr2line=off

Native Builds bleiben unverändert.

Betroffene Dateien:

- configuration.nix
- overlays/reproducible-boost.nix

## Verifikation

Derivation:

    /nix/store/36hj7w9614i95j3arpkhmvzjrivip7jw-boost-armv7l-unknown-linux-gnueabihf-1.89.0.drv

Output:

    /nix/store/2ir0pah1qyhfw1ciz1gazs7nhaggc1gm-boost-armv7l-unknown-linux-gnueabihf-1.89.0

NAR-Hash foobox:

    sha256-+1uNPp9FqbV76PHiO6nPUX+CFMMTGm5JBSxAiKw4aY0=

NAR-Hash VM102:

    sha256-+1uNPp9FqbV76PHiO6nPUX+CFMMTGm5JBSxAiKw4aY0=

Ergebnis:

    BOOST_REPRODUCIBILITY_VERIFIED=PASS

## Protokolle

- NOVENA-BOOST-REPRODUCIBILITY-30R
- NOVENA-BOOST-RESULT-VERIFY-31

Die vollständigen TXT-Protokolle und Prüfsummen sind im
Repository project-block-logs archiviert.

## Geltungsbereich

Die Hash-Übereinstimmung ist für den getesteten Boost-Output
nachgewiesen. Eine vollständige Reproduzierbarkeit des
Novena-SD-Images ist dadurch noch nicht belegt.
