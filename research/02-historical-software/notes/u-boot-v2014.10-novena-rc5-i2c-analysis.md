# U-Boot v2014.10-novena-rc5 – I2C3-/POR-Analyse

## Untersuchter Stand

- Repository: `https://github.com/xobs/u-boot-novena.git`
- Tag: `v2014.10-novena-rc5`
- Tag-Typ: lightweight tag
- Commit: `9402489e9a3e52a9df5464b4ce3c792cee2ba3a5`
- Commit-Datum: 2014-10-11
- Gegenstand dieser Notiz ist ausschließlich dieser historische rc5-Quellstand.
- Aussagen über spätere Novena-U-Boot-Versionen oder das aktuell getestete U-Boot 2020.07 werden daraus nicht abgeleitet.

## I2C3-Hardwarezuordnung in rc5

`board/kosagi/novena/novena_spl.c` dokumentiert für I2C3:

- `0x11 ... ES8283` (historische Schreibweise im Quelltext)
- `0x50 ... LCD EDID`
- `0x56 ... EEPROM`

Die I2C3-Pads sind:

- SCL: `MX6_PAD_EIM_D17__I2C3_SCL` / GPIO-Fallback `GPIO3_IO17`
- SDA: `MX6_PAD_EIM_D18__I2C3_SDA` / GPIO-Fallback `GPIO3_IO18`

Die konfigurierte I2C-Geschwindigkeit beträgt `100000` Hz.

## SPL-Reihenfolge

In `board_init_f()` erfolgt die relevante Initialisierung in dieser Reihenfolge:

1. Audio-IOMUX
2. Button-IOMUX
3. Ethernet-IOMUX
4. FPGA-IOMUX
5. I2C-IOMUX
6. PCIe
7. SDHC
8. SPI
9. UART
10. Video

Damit wird der FPGA-Zustand vor der Initialisierung von I2C3 gesetzt.

## FPGA_RESET_N

`NOVENA_FPGA_RESET_N_GPIO` ist `GPIO5_IO07` auf dem Pad `DISP0_DAT13`.

`novena_spl_setup_iomux_fpga()` konfiguriert das Pad als GPIO und führt aus:

`gpio_direction_output(NOVENA_FPGA_RESET_N_GPIO, 0);`

Die vollständige Suche im rc5-Tree und im Novena-spezifischen Code ergab keine spätere explizite Freigabe dieses Signals auf High.

Die zusätzliche Pad-/Video-Untersuchung ergab außerdem:

- `DISP0_DAT13` wird im Novena-spezifischen rc5-Code nur an dieser FPGA-Reset-Stelle referenziert.
- `novena_spl_setup_iomux_video()` verändert nur `EIM_A24 / GPIO5_IO04` für HDMI Ghost HPD.
- `setup_display()` in `novena.c` konfiguriert IPU/LDB-Takte und IOMUXC-GPR2/GPR3, muxt `DISP0_DAT13` aber nicht neu.

Damit bleibt der durch rc5 konfigurierte FPGA-Reset-Zustand innerhalb des untersuchten Novena-spezifischen U-Boot-Codes erhalten. Eine außerhalb dieses Codes verursachte elektrische Zustandsänderung ist damit nicht ausgeschlossen.

## Audio-Power

`NOVENA_AUDIO_PWRON` ist `GPIO5_IO17` auf `DISP0_DAT23`.

SPL führt aus:

`gpio_direction_output(NOVENA_AUDIO_PWRON, 1);`

Im untersuchten Novena-spezifischen rc5-Code wurde weder ein späterer GPIO-Write auf diesen Anschluss noch ein Remux von `DISP0_DAT23` gefunden.

## setup_i2c() und Bus-Recovery

SPL ruft für alle drei Controller `setup_i2c()` auf, für I2C3 konkret:

`setup_i2c(2, CONFIG_SYS_I2C_SPEED, 0x7f, &i2c_pad_info2);`

`setup_i2c()`:

1. aktiviert den I2C-Clock,
2. ruft `force_idle_bus()` auf,
3. initialisiert anschließend den Controller über `bus_i2c_init()` und hinterlegt `force_idle_bus()` als Idle-Bus-Funktion.

`force_idle_bus()` schaltet SDA und SCL zunächst auf GPIO-Eingänge und liest beide Leitungspegel.

Sind SDA und SCL bereits High, wird der Bus als idle betrachtet.

Ist mindestens eine Leitung Low, werden neun Low/Release-Zyklen auf SCL erzeugt, jeweils mit 50 us Low- und 50 us Release-Zeit. Danach wartet die Funktion bis zu ungefähr 0,2 Sekunden darauf, dass SDA und SCL gleichzeitig High werden. Abschließend werden beide Pads wieder auf I2C-Funktion gemuxt.

Dieser Mechanismus ist eine explizite historische Bus-Recovery-/Idle-Maßnahme während der Initialisierung.

## I2C-Transferpfad

Vor einem Transfer aktiviert der historische MXC-I2C-Treiber den Controller und wartet 50 us auf Stabilisierung.

Vor START wird auf `ST_BUS_IDLE` gewartet. Anschließend setzt der Treiber `I2CR_MSTA` und wartet auf `ST_BUS_BUSY`.

`wait_for_sr_state()` prüft dabei explizit `I2SR_IAL`. Wird Arbitration Lost erkannt, wird das IAL-Bit behandelt und `-ERESTART` zurückgegeben.

`i2c_init_transfer()` besitzt bis zu drei Transfer-Versuche. Nach einem Fehler wird STOP ausgelöst; nach 100 us wird `i2c_idle_bus()` aufgerufen. Fehler vom Typ `-ERESTART` werden dabei anders behandelt als andere Fehler, da der Controller in diesem Fall nicht explizit deaktiviert wird.

## EEPROM-Zugriff in Main U-Boot

`misc_init_r()` enthält einen Zugriff auf die Novena-EEPROM auf I2C-Bus 2, Adresse `0x56`.

Dieser Zugriff ist in rc5 nicht unbedingt bei jedem Boot ausgeführt: Ist die Umgebungsvariable `ethaddr` bereits gesetzt, kehrt `misc_init_r()` vorher zurück.

Daher ist bewiesen, dass rc5 den EEPROM-Zugriff implementiert und ausführen kann; ein EEPROM-Transfer auf I2C3 bei jedem einzelnen Boot ist damit nicht bewiesen.

## IT6251 und ES8328

Die gezielte Suche im untersuchten rc5-Novena-Code ergab keine Treffer für `IT6251`, `it6251`, `ES8328` oder `es8328`.

Insbesondere darf später gefundener IT6251-spezifischer U-Boot-Code daher nicht ohne Versionsnachweis rc5 zugeschrieben werden.

## Bedeutung für die aktuelle Linux-6.18-Untersuchung

Der historische rc5-Stand zeigt mehrere POR-relevante Maßnahmen:

- FPGA wird vor I2C3-Setup aktiv in Reset gebracht.
- Audio-Power wird vor I2C3-Setup gesetzt.
- I2C3-Pads werden beim Setup zunächst als GPIO gelesen.
- Ein nicht-idler Bus wird mit neun SCL-Pulsen behandelt.
- Erst danach wird der MXC-I2C-Controller initialisiert.
- Der Transferpfad behandelt Arbitration Lost explizit und besitzt Retry-/Idle-Bus-Logik.

Diese Unterschiede sind für die aktuelle Cold-POR-Fehlersuche relevant, beweisen aber keine Ursache des Linux-6.18-IAL-Fehlers.

Insbesondere ist noch nicht bewiesen, dass das auf der aktuellen Novena verwendete U-Boot 2020.07 dieselbe FPGA-, Audio-, I2C3- oder Bus-Recovery-Sequenz besitzt.

## Vorläufiges Ergebnis

Der rc5-Quellstand liefert einen belastbaren historischen Referenzpunkt: Novena-U-Boot behandelte I2C3 beim frühen Boot nicht nur durch Controller-Initialisierung, sondern kombinierte diese mit definierter FPGA-/Audio-GPIO-Konfiguration und einer GPIO-basierten Prüfung bzw. Recovery des I2C-Busses.

Für die aktuelle Root-Cause-Analyse ist deshalb besonders wichtig zu untersuchen, wann und warum diese Maßnahmen in späteren Novena-U-Boot-Ständen verändert wurden und welchen Zustand das tatsächlich eingesetzte U-Boot 2020.07 an Linux übergibt.
