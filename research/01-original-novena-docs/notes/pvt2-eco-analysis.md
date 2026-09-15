# Novena PVT2 – Auswertung der ECO-Liste

## Zweck

Diese Notiz dokumentiert die für die aktuelle Novena-I2C3-
Kaltstartuntersuchung relevanten Befunde aus der offiziellen
„Novena PVT2 ECO List“.

Die ECO-Liste wird als Primärquelle dafür verwendet, Änderungen zwischen
PVT1, PVT2 und nachträglichen PVT2-Rev-A-Fertigungsänderungen zu
rekonstruieren.

Die Befunde werden in drei Gruppen getrennt:

1. direkte I2C3-relevante Änderungen,
2. indirekt relevante Power-/Reset-/POR-Änderungen,
3. relevante Negativbefunde.

Aus der ECO-Liste allein wird keine Ursache für den aktuellen
I2C3-Arbitration-Loss abgeleitet.

---

## Quellenherkunft

Archivierte offizielle Kosagi-Seite:

    research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.html

SHA-256:

    da71f18836cf1bea58cefbcc84202dda6f8b265692569142d355ca3a4117c5a9

Archivierte HTTP-Header:

    research/01-original-novena-docs/sources/kosagi-novena-pvt2-eco-list.http-headers.txt

SHA-256:

    11e12a24223baaf49d90c10b51ca18b4e74aef4ba70444036ba204722f40eb83

MediaWiki-Seitenidentität:

    wgPageName: Novena_PVT2_ECO_List
    wgRevisionId: 1372
    wgArticleId: 391

Die archivierte HTML-Datei besitzt:

    760 Zeilen
    37781 Byte

Der HTTP-Header meldet:

    HTTP/2 200
    last-modified: Sat, 02 May 2026 23:23:50 GMT
    content-length: 37781
    content-type: text/html

---

# 1. Umfang der ECO-Liste

Die archivierte Seite dokumentiert insgesamt ECO1 bis ECO23.

ECO1 bis ECO21 stehen im ursprünglichen PVT1/PVT2-Änderungsbereich.

Danach folgt ein eigener Abschnitt:

    Post bring-up ECO

Dieser enthält:

    ECO22
    ECO23

ECO22 und ECO23 wurden nach dem PVT2-Rev-A-Bring-up als gesonderte
Fertigungsänderungen ausgegeben.

Diese zeitliche Trennung ist für die Interpretation verschiedener
PVT2-Schaltplanstände wichtig.

---

# 2. Direkter I2C-relevanter ECO: ECO4

ECO4 trägt den Titel:

    Improve kernel panic logging

Die Begründung lautet sinngemäß, dass U10S für eine brauchbare
Kernel-Panic-Protokollierung zu klein war und deshalb durch ein
größeres I2C-EEPROM ersetzt werden sollte.

Die dokumentierte Änderung lautet:

PVT1:

    U10S 24LC32A-I/ST

PVT2:

    U10S FT24C512AUTR-T

Begründung:

    improve KP logging

Dieser ECO betrifft direkt einen bekannten Teilnehmer des
Novena-I2C3-Busses.

Der bereits untersuchte PVT2-Schaltplan zeigt U10S als Utility-EEPROM
an I2C3.

Der PVT2-Schaltplan nennt für das Bauteil die Adresse 0xAC in
8-Bit-Schreibweise, entsprechend der 7-Bit-I2C-Adresse 0x56.

Damit bestätigt ECO4, dass sich die Bestückung dieses I2C3-Teilnehmers
zwischen PVT1 und PVT2 geändert hat.

Die ECO-Liste dokumentiert für ECO4 jedoch keine Änderung an:

- I2C3_SCL,
- I2C3_SDA,
- den I2C3-Pull-ups,
- Serienwiderständen,
- der Versorgung des EEPROMs,
- dem CPU-Pinmux,
- oder der grundlegenden Busverdrahtung.

Aus ECO4 lässt sich daher bislang nur eine Änderung des EEPROM-Bauteils
bzw. seiner Kapazität ableiten.

Eine elektrische Ursache für den aktuellen IAL-Fehler ist daraus nicht
belegt.

---

# 3. ECO14 – FPGA und zusätzliche I2C-Möglichkeiten

ECO14 trägt den Titel:

    Rework front panel connector

Der Frontpanel-Anschluss wurde auf einen 30-poligen Anschluss erweitert.

Unter anderem wurden sechs GPIO-Leitungen vom FPGA herausgeführt.

Die ECO-Liste beschreibt ausdrücklich, dass diese sechs Leitungen unter
anderem verwendet werden können, um:

    3x I2C

zu emulieren.

Außerdem wurde U14F entfernt.

U14F war ein SPI-ROM am FPGA, das laut ECO-Dokumentation nie in einem
Design verwendet wurde.

Die ehemaligen Signale wurden stattdessen zum neuen 30-poligen
Anschluss geführt.

Dieser ECO belegt Änderungen an der externen FPGA-Konnektivität.

Er dokumentiert jedoch keine Änderung der bereits im PVT2-Schaltplan
identifizierten direkten Verbindung:

    FPGA P4 -> I2C3_SCL
    FPGA P3 -> I2C3_SDA

Damit ist ECO14 für die allgemeine FPGA-Revisionsgeschichte relevant,
liefert aber bislang keinen Beleg für eine Änderung des untersuchten
I2C3-Buspfades.

---

# 4. ECO15 – Zusammenhang mit dem LCD-Adapter

ECO15 trägt den Titel:

    DNP digital mic header

Als Begründung wird angegeben, dass der betreffende Header nicht
gleichzeitig mit dem LCD-Adapterboard verwendet werden kann und deshalb
redundant ist.

Entfernt wurden unter anderem:

    P12A
    P13A
    C33A
    R31A
    R34A

Dieser ECO bestätigt eine physikalische bzw. funktionale Überschneidung
mit dem LCD-Adapterbereich.

Die ECO-Liste dokumentiert dabei jedoch keine Änderung an:

- I2C3_SCL,
- I2C3_SDA,
- JPLCD,
- dem eDP-I2C-Pfad,
- oder dem IT6251.

Damit ergibt sich aus ECO15 kein direkter Hinweis auf die aktuelle
I2C3-Fehlerursache.

---

# 5. ECO17 – USB-Hub-Resetleitungen

ECO17 trägt den Titel:

    USB hub resets

Hintergrund war, dass die USB-Hubs über Soft-Power-Zyklen teilweise
nicht zuverlässig zurückgesetzt wurden.

Daher wurden CPU-gesteuerte Resetmöglichkeiten vorgesehen.

Die ECO-Liste nennt:

    DI0_PIN4 / ball P25 -> USB_HUB1_RST
    DISP0_DAT6 / ball R23 -> USB_HUB2_RST

Weitere Validierung stellte den Nutzen dieser Resetsteuerung infrage.

Daher wurden Teile der Resetpfade als DNP-Option ausgeführt.

Die ECO-Liste erklärt außerdem:

    Reset lines also have pull-ups added to them,
    so in case they float they go to a sane value.

Die dazu aufgeführten Widerstände umfassen:

    R26U 1k, 1%
    R45U 1k, 1%

Diese Pull-ups gehören zum Kontext der USB-Hub-Resetleitungen.

Sie sind nicht als I2C3-Pull-ups und nicht als IT6251-RESET_N-Pull-up
dokumentiert.

Damit darf ECO17 nicht als Hinweis auf eine Änderung der
IT6251-Resetbeschaltung interpretiert werden.

---

# 6. ECO18 – langsamer sichtbarer Displaystart

ECO18 trägt den Titel:

    Power LED turn-off

Die eigentliche Änderung betrifft die Power-LED.

Für die Displayuntersuchung ist jedoch eine Nebenbemerkung interessant.

Die ECO-Dokumentation erklärt, dass die Power-LED wichtig sei, weil der
LCD-Bildschirm nach dem Drücken des Einschaltknopfes mehrere Sekunden
benötigt, bevor er sich einschaltet.

Dies ist ein historischer Hinweis darauf, dass ein verzögerter sichtbarer
Displaystart auf der Novena bekannt bzw. erwartet war.

Der Eintrag dokumentiert jedoch nicht:

- warum der LCD-Start mehrere Sekunden dauert,
- ob dies durch den IT6251 verursacht wird,
- ob während dieser Zeit I2C-Zugriffe stattfinden,
- oder ob dies mit dem aktuellen IAL-Fehler zusammenhängt.

Der Befund wird daher nur als historischer Kontext festgehalten.

---

# 7. ECO22 – Post-Bring-up-Änderung an der Audiosteuerung

Nach ECO21 beginnt der Abschnitt:

    Post bring-up ECO

ECO22 trägt den Titel:

    swap audio control option

Die Änderung betrifft die Widerstandsoptionen der Audiosteuerung.

Dokumentiert ist außerdem:

    Issued to factory as ECO-0001 against PVT2 rev A
    on September 30 2014.

Damit ist belegt, dass nach PVT2 Rev A weitere Fertigungsänderungen
existierten.

ECO22 betrifft jedoch nicht die bereits identifizierten
I2C3-Audio-Serienwiderstände:

    R26A
    R27A

und dokumentiert keine Änderung am ES8328-I2C-Zweig.

Der Eintrag liefert daher keinen direkten Hinweis auf die aktuelle
I2C3-Fehlerursache.

---

# 8. ECO23 – Reglerinstabilität beim Kaltstart

ECO23 ist für die aktuelle POR-Untersuchung besonders relevant.

Titel:

    improve regulator stability on cold power-on

Die ECO-Liste erklärt ausdrücklich, dass ECO13 den Leistungsinduktor
geändert hatte.

Danach wurde Instabilität beim kalten Einschalten beobachtet.

Die Primärquelle beschreibt Oszilloskopmessungen mit erheblichem Rauschen
auf der 5-V-Schiene unmittelbar nach dem Einstecken der Versorgung.

Als Folge konnte sich der 5-V-Regler abschalten.

Als Gegenmaßnahme wurde ein zusätzlicher 22-uF-Kondensator auf der
5-V-Ausgangsfilterung bestückt.

Die konkrete Änderung lautet:

PVT1:

    C23N 22uF, 10V, X5R 20% (DNP)

PVT2 nach ECO:

    C23N 22uF, 10V, X5R 20%

Die Quelle erklärt, dass die zusätzliche Kapazität das beobachtete
Rauschen beseitigte und das System über mehrere Kaltstarts stabil
erschien.

ECO23 wurde ausgegeben als:

    ECO-0002 against PVT2 rev A

Datum:

    October 15 2014

---

# 9. Separates ECO23-Artefakt

Wegen seiner besonderen Bedeutung für die Kaltstartuntersuchung wurde
ECO23 zusätzlich separat aus der archivierten HTML-Quelle extrahiert.

Datei:

    research/01-original-novena-docs/extracts/pvt2-eco23.txt

Eigenschaften:

    40 Zeilen
    1495 Byte

SHA-256:

    261467abaf544073af0fe071056121748f564f8e15378368f8bcfa8b8d6e3664

Die zugrunde liegende vollständige ECO-HTML-Datei blieb dabei unverändert.

SHA-256:

    da71f18836cf1bea58cefbcc84202dda6f8b265692569142d355ca3a4117c5a9

---

# 10. Bedeutung von ECO23 für die aktuelle Untersuchung

Der aktuelle Fehler zeigt einen deutlichen Unterschied zwischen:

    echtem Kaltstart / POR

und:

    erfolgreichem LDB-Rebind innerhalb desselben Boots

Die aktuelle Kernel-Instrumentierung zeigt:

Kaltstart:

    A=81/80 -> M0=93/80

Same-Boot-LDB-Rebind:

    A=81/80 -> M0=81/a0

ECO23 beweist unabhängig davon, dass bei der Novena-Hardware historisch
tatsächlich ein elektrisches Problem existierte, das ausschließlich bzw.
besonders beim kalten Einschalten sichtbar wurde.

Das macht die Power-on-Versorgungssequenz zu einem legitimen Bestandteil
der Ursachenanalyse.

ECO23 beweist jedoch NICHT, dass der heutige I2C3-Fehler dieselbe Ursache
hat.

Insbesondere ist derzeit nicht belegt:

- ob die getestete Novena vor oder nach ECO-0002 gefertigt wurde,
- ob C23N auf der getesteten Platine tatsächlich bestückt ist,
- ob die 5-V-Schiene beim aktuellen Kaltstart instabil ist,
- ob die IT6251-Versorgung davon beeinflusst wird,
- ob I2C3_SCL oder I2C3_SDA dadurch beeinflusst werden,
- oder ob ECO23 irgendeinen Zusammenhang mit IAL besitzt.

ECO23 wird deshalb als wichtige POR-bezogene Spur dokumentiert, nicht als
Ursachennachweis.

---

# 11. I2C3-Pull-up-Diskrepanz

Die bisher untersuchten Mainboard-Unterlagen enthalten weiterhin eine
ungeklärte Diskrepanz.

Die Dokumentübersicht nennt:

    I2C3: 2.2k pull-up

Das detaillierte Schaltplanblatt zeigt dagegen:

    R10B = 1k
    R11B = 1k

Die vollständige ECO-Liste enthält keinen identifizierten Eintrag für:

    R10B
    R11B
    2.2k
    2k2

und keinen dokumentierten ECO, der ausdrücklich die
I2C3-Pull-up-Werte ändert.

Damit erklärt die PVT2-ECO-Liste die 2,2-kOhm/1-kOhm-Diskrepanz nicht.

Die Ursache dieser Diskrepanz bleibt offen.

---

# 12. Weitere relevante Negativbefunde

In der vollständigen ECO-Liste wurde keine explizite Änderung gefunden an:

- I2C3_SCL,
- I2C3_SDA,
- EIM_D17,
- EIM_D18,
- R10B,
- R11B,
- R26A,
- R27A,
- IT6251,
- AUX_I2C_SCL,
- AUX_I2C_SDA.

Es wurde ebenfalls kein ECO identifiziert, der ausdrücklich eine Änderung
der direkten FPGA-Verbindung zu I2C3 beschreibt.

Die ECO-Liste enthält keine dokumentierte Änderung der
IT6251-RESET_N-Schaltung.

Diese Negativbefunde bedeuten nicht, dass solche Änderungen niemals
existierten.

Sie bedeuten nur, dass sie in der archivierten Revision 1372 der
„Novena PVT2 ECO List“ nicht identifiziert wurden.

---

# 13. Beziehung zwischen PVT2 Rev A und den Schaltplanartefakten

Es liegen zwei unterschiedliche PVT2-Schaltplan-PDFs vor.

Älteres Artefakt:

    novena_pvt2.PDF

SHA-256:

    1d4c1ecdf3399867b68c0b13959b30c7cc7c23d051de2aa2c87cd66652502951

Offiziell über die PVT-Design-Source-Seite verlinktes Artefakt:

    SCH-novena_pvt2.PDF

SHA-256:

    0285de44da7471e3459b7b8517e7d3e42fbda049540530c404ddbb873a514672

Die PDFs sind nicht byte-identisch.

Die bisherige Textanalyse zeigt Unterschiede, hat aber noch keine
eindeutige elektrische I2C3-Änderung zwischen diesen beiden PDFs
nachgewiesen.

Die ECO-Liste dokumentiert zusätzlich mindestens zwei Änderungen gegen
PVT2 Rev A nach dem Bring-up:

    ECO-0001 / ECO22 – 30. September 2014
    ECO-0002 / ECO23 – 15. Oktober 2014

Damit muss bei zukünftigen Vergleichen sorgfältig zwischen folgenden
Ständen unterschieden werden:

- PVT1,
- PVT2 / PVT2 Rev A,
- PVT2 Rev A mit späteren Fertigungs-ECOs.

Die genaue Beziehung der beiden vorhandenen PVT2-PDFs zu diesen
Fertigungsständen ist noch nicht vollständig geklärt.

---

# 14. Aktueller Erkenntnisstand nach der ECO-Auswertung

Direkt für I2C3 belegt:

- U10S wurde zwischen PVT1 und PVT2 von 24LC32A auf FT24C512A geändert.
- Keine dokumentierte ECO-Änderung der I2C3-Pull-ups wurde gefunden.
- Keine dokumentierte ECO-Änderung des IT6251-I2C-Pfades wurde gefunden.
- Keine dokumentierte ECO-Änderung der direkten FPGA-I2C3-Verbindung
  wurde gefunden.

Für POR relevant:

- Die Novena hatte historisch nach ECO13 eine reale
  Kaltstartinstabilität des 5-V-Reglers.
- Diese wurde mit ECO23 durch Bestückung von C23N mit 22 uF adressiert.
- ECO23 wurde nach PVT2 Rev A als Fertigungs-ECO ausgegeben.

Für Reset relevant:

- ECO17 ergänzt Pull-ups an USB-Hub-Resetleitungen.
- Diese dürfen nicht mit IT6251 RESET_N verwechselt werden.

Weiterhin offen:

- tatsächlicher C23N-Bestückungszustand der getesteten Novena,
- tatsächlicher I2C3-Pull-up-Wert der getesteten Novena,
- Erklärung der 2,2-kOhm/1-kOhm-Diskrepanz,
- FPGA-Zustand während POR,
- ES8328-Zustand während POR,
- IT6251-Versorgungs- und Reset-Timing,
- Verhalten des konkreten Bootloaders vor Linux.

---

# 15. Nächster Schritt

Der nächste Primärquellenblock innerhalb der Originaldokumentation ist:

    Novena PVT Issue Log

Dort soll gezielt gesucht werden nach:

- I2C,
- I2C3,
- arbitration,
- FPGA,
- LCD,
- eDP,
- IT6251,
- display,
- audio,
- ES8328,
- EEPROM,
- reset,
- cold boot,
- power-on,
- regulator,
- 5V,
- C23N,
- ECO23.

Anschließend folgen:

- U-boot PVT Notes,
- IT6251-Diagnosedokumentation,
- gegebenenfalls originale Altium-Designquellen.

---

# Checkpoint-Regel

Aus der ECO-Auswertung wird kein Kernel-Patch 0007 abgeleitet.

ECO23 wird als POR-relevanter historischer Hardwarebefund behandelt,
nicht als Erklärung des aktuellen IAL.

Die Novena bleibt während dieser Dokumentationsphase ausgeschaltet.

Die externe Test-SD und der dokumentierte Kernel-0006-Zustand bleiben
unverändert.
