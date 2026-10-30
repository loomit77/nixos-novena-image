# Novena – Chatstarter – 30.10.2026

Vor Arbeitsbeginn die private Transportdokumentation lesen:
\`regeln/arbeitsregeln.yaml\`, \`regeln/github-transport.yaml\`,
\`regeln/sicherheitsgrenzen.yaml\`, \`status/bridge-current.json\`,
\`status/bridge-1.0-plan.yaml\` und \`uebergaben/CHATSTART.md\`
im Repository \`loomit77/novena-command-files\`.

## Aktueller Stand
- RC406: äußerer Runner PASS (308 Tests, 0 FAIL, 2 ignoriert).
  RC406-Entwicklungsbericht PARTIAL/BLOCKED: M3/M5 technisch offen.
- Review407: RC406-Snapshot SHA256 und Git-Schutz PASS.
- Backup409: USB-Borg und VM103-Borg PASS; neue Archive,
  USB ausgehängt, Restore nicht getestet.
- Git410: Hauptarbeitskopie HEAD
  \`d20281687076ca971d5d2ee5877340cdcd58e7aa\`,
  geschützter Index SHA256
  \`ec128c81a56644ebd9a16de5134bc9fd37b52359386a9b8c8f61e4f1938d995a\`,
  11 geänderte + 6 unversionierte Pfade. Nicht resetten/stagen.
- Tagesabschluss411 erstellt einen Dokumentationscommit in **isolierter**
  Arbeitskopie; GitHub/Codeberg-Status nur aus Runner-RAW belegen.
  Hauptarbeitskopie bleibt auf bisherigem HEAD, bis später explizit
  und sicher synchronisiert wird.

## Nächster Auftrag
Ein zusammenhängender, isolierter M3/M5-Integrationsauftrag.
Produktive Bridge, Upload-/Publisherfreigabe, Hardwaretests,
neue Builds, last_verified und unnötige Sicherungswiederholungen
bleiben gesperrt, soweit nicht ausdrücklich freigegeben.
Runner/Wrapper und vollständig archiviertes TXT/SHA256 sind Pflicht.
