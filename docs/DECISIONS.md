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
