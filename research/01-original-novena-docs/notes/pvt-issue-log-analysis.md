# Novena PVT Issue Log – Auswertung

## Zweck

Diese Notiz dokumentiert die Auswertung des offiziellen
„Novena PVT Issue Log“ im Rahmen der Untersuchung des aktuellen
I2C3-Kaltstartfehlers der Novena.

Die Quelle wurde vollständig ausgewertet und nicht nur anhand einzelner
Suchbegriffe untersucht.

Aus ihr wird keine Ursache für den aktuellen I2C3-Arbitration-Loss
abgeleitet.

---

# 1. Quellenherkunft

Archivierte offizielle Kosagi-Seite:

    research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.html

SHA-256:

    e84812983edf256a2ce9da5960805be4aa475736ce1c6865ebce7c42320b8589

Archivierte HTTP-Header:

    research/01-original-novena-docs/sources/kosagi-novena-pvt-issue-log.http-headers.txt

SHA-256:

    5bb311c474b809d4d4a2d790ee632cb63d8a403d6367e44e465823c42aad320c

MediaWiki-Seitenidentität:

    wgPageName: Novena_PVT_Issue_Log
    wgRevisionId: 1057
    wgArticleId: 364

Die archivierte HTML-Datei besitzt:

    153 Zeilen
    14262 Byte

HTTP-Metadaten:

    HTTP/2 200
    last-modified: Sat, 02 May 2026 23:23:50 GMT
    content-length: 14262
    content-type: text/html

---

# 2. Separat gesicherter Inhaltsbereich

Da die Seite sehr kurz ist, wurde ihr vollständiger eigentlicher
MediaWiki-Inhaltsbereich zusätzlich separat gesichert.

Datei:

    research/01-original-novena-docs/extracts/pvt-issue-log-content.html

Eigenschaften:

    30 Zeilen
    1665 Byte

SHA-256:

    479288aa3adcd8ae11809c0835a56ac2d425ad081e07ac3febb70aa5ad770dc6

Damit wurde der gesamte sachliche Inhalt der Seite geprüft.

---

# 3. Hardware Bringup Notes

Der Abschnitt:

    Hardware Bringup Notes

enthält genau einen dokumentierten Hardwarepunkt.

Die Quelle beschreibt eine zu niedrige Eingangsimpedanz des
i.MX6-HPD-Eingangs von 10 kOhm.

Dadurch sei HPD auf einigen Boards nur grenzwertig stabil gewesen.

Als Änderung wird angegeben:

    R29L 10k, 1%
        ->
    R29L 1k, 1%

Ziel war eine zuverlässigere HPD-Funktion.

Dieser Punkt betrifft HPD und liefert keinen dokumentierten Bezug zu:

- I2C3,
- I2C3_SCL,
- I2C3_SDA,
- IT6251,
- AUX_I2C,
- oder dem aktuellen IAL.

Er zeigt allerdings, dass im Bring-up reale elektrische
Marginalitätsprobleme auf einzelnen Boards gefunden und durch
Widerstandsänderungen korrigiert wurden.

---

# 4. Running Changes

Der zweite Abschnitt trägt den Titel:

    Running changes - available in PVT1E, but doc check needed

Besonders wichtig ist die ausdrückliche Warnung der Quelle:

    These changes need to be verified in production
    as they were running changes during the previous run
    and were not propagated through all the docs.

Damit dokumentiert die Primärquelle selbst, dass bestimmte
Produktionsänderungen nicht vollständig in alle Novena-Dokumente
übernommen wurden.

Dies ist für die Bewertung unterschiedlicher Schaltplan- und
Dokumentstände wichtig.

Widersprüche zwischen historischen Unterlagen dürfen deshalb nicht
automatisch als Fehler unserer Extraktion oder Analyse interpretiert
werden.

Umgekehrt beweist dieser Hinweis nicht, dass jede gefundene
Dokumentabweichung durch eine solche Running Change entstanden ist.

---

# 5. STMPE610 -> STMPE811

Als erste Running Change wird dokumentiert:

    U11D STMPE610
        ->
    U11D STMPE811

Grund:

    EOL of STMPE610

Die Quelle bezeichnet die Bauteile als pin-kompatibel.

Dieser Befund passt zur bekannten Novena-Revisionsgeschichte des
STMPE-Bausteins.

Für die aktuelle Untersuchung ist wichtig, diesen historischen
Hardwarewechsel nicht mit dem aktuellen I2C3-START-Fehler
gleichzusetzen.

Der STMPE811 wurde in der aktuellen Testkonfiguration bereits
gezielt deaktiviert.

Die vollständige PVT-Issue-Log-Seite enthält keinen Hinweis darauf,
dass diese Running Change einen I2C3-Kaltstart- oder
Arbitration-Loss-Fehler verursacht.

---

# 6. Utility-EEPROM U10S

Als zweite Running Change wird dokumentiert:

    U10S 24LC32A-I/ST
        ->
    U10S FT24C512AUTR-T

Als Zweck nennt die Quelle:

    kernel panic logging

Dieser Befund stimmt mit ECO4 der separat untersuchten
PVT2-ECO-Liste überein.

U10S ist nach dem untersuchten PVT2-Schaltplan ein Teilnehmer des
I2C3-Busses.

Damit bestätigen zwei offizielle historische Quellen den
Bauteilwechsel.

Das PVT Issue Log dokumentiert jedoch keine Änderung an:

- I2C3_SCL,
- I2C3_SDA,
- den I2C3-Pull-ups,
- der Versorgung von U10S,
- dem CPU-Pinmux,
- oder der grundlegenden I2C3-Busverdrahtung.

Daraus ergibt sich kein Ursachennachweis für den aktuellen IAL.

---

# 7. Audioverstärker

Als dritte Running Change wird dokumentiert:

    U10A, U12A NS4890
        ->
    U10A, U12A NS4890B

Grund:

    EOL of NS4890

Dies betrifft die Audioverstärker.

Das Issue Log nennt dabei nicht:

- ES8328,
- den I2C3-Audiozweig,
- R26A,
- R27A,
- oder einen I2C-Fehler.

Damit ergibt sich aus diesem Eintrag kein direkter Hinweis auf die
aktuelle I2C3-Untersuchung.

---

# 8. Relevanter Negativbefund

Der vollständige sachliche Inhalt des PVT Issue Logs wurde geprüft.

Es wurde kein Eintrag gefunden zu:

- I2C3,
- I2C arbitration,
- IAL,
- SCL,
- SDA,
- IT6251,
- eDP,
- AUX_I2C,
- LVDS,
- EDID,
- Display-Kaltstart,
- FPGA-I2C3-Zuständen,
- ES8328-I2C-Problemen,
- einem I2C-Bus-Hang,
- einem I2C-START-Fehler,
- Reglerinstabilität beim Kaltstart,
- 5-V-Rauschen,
- C23N,
- oder ECO23.

Insbesondere enthält diese Revision des PVT Issue Logs keine
dokumentierte Vorgeschichte des heute beobachteten Fehlermusters:

    Cold FAIL:
    A=81/80 -> M0=93/80

gegenüber:

    Same-Boot-LDB-Rebind PASS:
    A=81/80 -> M0=81/a0

Das ist ein belastbarer Negativbefund für genau diese archivierte
Revision der Quelle.

Es beweist nicht, dass ein entsprechendes Problem historisch nie
beobachtet wurde.

---

# 9. Verhältnis zur PVT2-ECO-Liste

Das PVT Issue Log und die PVT2-ECO-Liste überschneiden sich teilweise.

Gemeinsam dokumentiert werden unter anderem:

- Änderung des HPD-Widerstands,
- STMPE610 -> STMPE811,
- U10S 24LC32A -> FT24C512A,
- NS4890 -> NS4890B.

Die ECO-Liste ist wesentlich umfangreicher und dokumentiert spätere
Änderungen bis ECO23.

Insbesondere enthält nur die bislang untersuchte ECO-Liste den
wichtigen späteren Befund:

    ECO23:
    improve regulator stability on cold power-on

Das PVT Issue Log kann daher nicht als vollständiges Register aller
PVT2- oder Post-Bring-up-Probleme betrachtet werden.

---

# 10. Bedeutung der Dokumentationswarnung

Für die weitere Forschung ist die Warnung über nicht vollständig
propagierte Running Changes besonders wichtig.

Sie liefert einen historischen Primärquellenbeleg dafür, dass:

- Produktionsstände,
- Schaltplanstände,
- BOM-Stände,
- und Wiki-Dokumentation

nicht zwingend zu jedem Zeitpunkt vollständig synchron waren.

Dies ist relevant für die bereits gefundene Diskrepanz:

    Dokumentübersicht:
    I2C3: 2.2k pull-up

gegenüber:

    detaillierter PVT2-Schaltplan:
    R10B = 1k
    R11B = 1k

Das PVT Issue Log erklärt diese konkrete Diskrepanz NICHT.

Es zeigt lediglich, dass nicht vollständig propagierte
Hardwareänderungen historisch tatsächlich vorkamen.

Die 2,2-kOhm/1-kOhm-Frage bleibt deshalb weiterhin offen.

---

# 11. Aktueller Erkenntnisstand

Aus dem PVT Issue Log neu bzw. zusätzlich abgesichert:

1. Die Seite ist kein umfangreiches Fehlerregister, sondern eine sehr
   kurze Bring-up-/Running-Change-Liste.

2. Ein HPD-Marginalitätsproblem wurde historisch auf einzelnen Boards
   beobachtet und durch eine Widerstandsänderung adressiert.

3. U10S wurde als Running Change von 24LC32A auf FT24C512A geändert.

4. Die Quelle warnt ausdrücklich vor Running Changes, die nicht durch
   alle Dokumente propagiert wurden.

5. Es gibt in dieser Quelle keinen dokumentierten Hinweis auf einen
   historischen I2C3-/IT6251-Kaltstartfehler.

6. ECO23 ist in dieser Quelle nicht enthalten.

---

# 12. Nächster Schritt

Als nächste originale Novena-Quelle werden die:

    U-boot PVT Notes

untersucht.

Dort ist insbesondere zu klären:

- welche I2C-Busse U-Boot initialisiert,
- ob I2C3 vor Linux benutzt wird,
- welche I2C3-Teilnehmer angesprochen werden,
- ob U10S bei jedem Boot gelesen wird,
- ob Display- oder IT6251-Initialisierung erfolgt,
- welche GPIO-, Reset-, Pinmux- und Power-Sequenzen U-Boot ausführt,
- und ob sich daraus ein POR-spezifischer Zustand ableiten lässt.

Die historische U-Boot-Dokumentation wird zunächst als Primärquelle
archiviert, bevor sie mit historischem Quellcode oder dem tatsächlich
verwendeten U-Boot der Test-SD verglichen wird.

---

# Checkpoint-Regel

Aus dem PVT Issue Log wird kein Kernel-Patch 0007 abgeleitet.

Der Negativbefund wird ebenso dokumentiert wie positive Treffer.

Die Novena bleibt während dieser Dokumentationsphase ausgeschaltet.

Die externe Test-SD und der dokumentierte Kernel-0006-Zustand bleiben
unverändert.
