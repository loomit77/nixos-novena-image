# Novena PVT2-A – I2C3 hardware topology

## Status

Research block 1: original Novena documentation.

This document records facts derived from the archived Novena PVT2-A
mainboard schematic.

No root-cause conclusion is made here.

Original schematic:

- File: `sources/novena_pvt2.PDF`
- SHA-256: `1d4c1ecdf3399867b68c0b13959b30c7cc7c23d051de2aa2c87cd66652502951`
- PDF title blocks: `Novena PVT2-A`
- Sheet date: `7/5/2014`
- Source format indicated by title blocks: Altium `.SchDoc`

## Proven schematic facts

### i.MX6Q connection

PVT2-A sheet 06 (`06cpu_soc.SchDoc`) connects:

- i.MX6Q EIM_D17, ball F21 -> `I2C3_SCL`
- i.MX6Q EIM_D18, ball D24 -> `I2C3_SDA`

No intervening series resistor, jumper, MOSFET or level shifter is shown
between these i.MX6Q pins and the global I2C3 nets on this sheet.

### Utility EEPROM

PVT2-A sheet 07 (`07sdcard.SchDoc`) contains U10S, an FT24C512A-UTR-T
utility EEPROM.

Connections:

- pin 6 SCL -> `I2C3_SCL`
- pin 5 SDA -> `I2C3_SDA`
- VCC -> `P3.3V_DELAYED`
- A0/A1/A2 -> GND

The schematic labels the device address as `0xAC`.

This is the 8-bit address representation and corresponds to 7-bit I2C
address `0x56`.

### Audio codec branch

PVT2-A sheet 14 (`14audio.SchDoc`) contains U11A, Everest ES8328E.

The codec is connected through series resistors:

- `I2C3_SCL` -> R26A 330 ohm -> `AUD_I2C3_SCL` -> ES8328E
- `I2C3_SDA` -> R27A 330 ohm -> `AUD_I2C3_SDA` -> ES8328E

Visual verification of the archived original schematic page 14 shows
that the 1 kohm pull-ups are on the codec side of these series
resistors:

- R10B: 1 kohm from `P3.3V_DELAYED` to `AUD_I2C3_SCL`
- R11B: 1 kohm from `P3.3V_DELAYED` to `AUD_I2C3_SDA`

This corrects the earlier text-extraction-based interpretation that
placed R10B/R11B directly on the global `I2C3_SCL`/`I2C3_SDA` side.

R11A is not an I2C pull-up. Visual verification of the original
schematic places R11A in the microphone circuitry; it is labelled
10 kohm and is associated with `MIC_DIFF_P`. The earlier
text-extraction-based interpretation of R11A as a 1 kohm pull-up from
`AUD_P3.3V` to `AUD_I2C3_SDA` was incorrect.

The ES8328E is powered from the switched audio power domain
`AUD_P3.3V`.

The audio power-management circuit on the same sheet uses:

- Q11A: FDN304P high-side device between `P3.3V_DELAYED` and
  `AUD_P3.3V`
- Q10A: 2N7002W in the `AUD_PWRON` control path
- Q12A: 2N7002W in a circuit explicitly labelled
  `active pulldown to ensure audio codec reset`

Therefore the codec power/reset state is intentionally sequenced in
hardware.

An electrically important consequence of the schematic topology is
that `AUD_I2C3_SCL` and `AUD_I2C3_SDA` can remain pulled toward
`P3.3V_DELAYED` through R10B/R11B while the codec supply
`AUD_P3.3V` is switched off and can be actively pulled down by the
audio power-management circuit.

This establishes a concrete circuit condition relevant to possible
clamp, back-power or power-domain interaction mechanisms. It does not
by itself prove that current actually flows through an internal ES8328E
I/O protection path, nor does it prove which electrical mechanism
causes the observed I2C3 arbitration-lost condition.

### FPGA connection

PVT2-A sheet 15 (`15fpga.SchDoc`) connects the global I2C3 nets directly
to FPGA I/O pins:

- `I2C3_SCL` -> FPGA pin P4, `IO_L2P_3`
- `I2C3_SDA` -> FPGA pin P3, `IO_L2N_3`

No intervening series resistor, jumper or level shifter is shown on this
sheet between the global I2C3 nets and these FPGA pins.

The same sheet contains FPGA reset/configuration and multiple FPGA power
domains. Their POR behaviour has not yet been analysed.

### LCD connector

PVT2-A sheet 13 (`13hdmi_lcd.SchDoc`) routes I2C3 to the LCD interface
connector JPLCD:

- JPLCD pin 6 -> `I2C3_SDA`
- JPLCD pin 7 -> `I2C3_SCL`

No additional series resistor or level shifter is shown on these two
signals on this sheet.

The sheet also contains the note `combo logic+EDID power` in the LCD
power area.

This mainboard schematic alone does not establish which devices on the
external display/eDP adapter are electrically connected to these I2C3
signals.

## Reconstructed partial topology

Global I2C3:

    I2C3_SCL
       +------------ i.MX6Q EIM_D17 / F21
       +------------ FT24C512A utility EEPROM @ 0x56
       +------------ FPGA P4 / IO_L2P_3
       +------------ JPLCD pin 7
       |
      R26A 330R
       |
       +---- AUD_I2C3_SCL ----> ES8328E
       |
      R10B 1k
       |
    P3.3V_DELAYED

    I2C3_SDA
       +------------ i.MX6Q EIM_D18 / D24
       +------------ FT24C512A utility EEPROM @ 0x56
       +------------ FPGA P3 / IO_L2N_3
       +------------ JPLCD pin 6
       |
      R27A 330R
       |
       +---- AUD_I2C3_SDA ----> ES8328E
       |
      R11B 1k
       |
    P3.3V_DELAYED

Audio power:

    P3.3V_DELAYED
          |
        Q11A
       FDN304P
          |
      AUD_P3.3V ----> ES8328E supply
          |
      Q12A active-pulldown path
          |
         GND

`AUD_PWRON` controls the audio power-management circuit through Q10A.

The schematic therefore permits the following distinct power-domain
condition:

    P3.3V_DELAYED = powered
    AUD_I2C3_SCL/SDA = pulled toward P3.3V_DELAYED through 1k
    AUD_P3.3V = switched off / actively pulled down

Whether this condition produces a clamp or back-power current through
the ES8328E remains an open electrical question.

## Cross-source consistency

The schematic i.MX6Q pin assignment agrees with the currently inspected
Linux DTS pinmux:

- EIM_D17 -> I2C3_SCL
- EIM_D18 -> I2C3_SDA

The utility EEPROM schematic address also agrees with historical Novena
U-Boot source using I2C bus 2 and address `0x56`.

These cross-source observations should be preserved, but the historical
U-Boot source itself belongs to the historical-source research block and
must be archived separately there.

## Documentation discrepancy

The PVT2-A document-map sheet contains the statement:

`I2C3: 2.2k pull-up`

However visual verification of sheet 14 shows:

- R10B = 1 kohm from `P3.3V_DELAYED` to `AUD_I2C3_SCL`
- R11B = 1 kohm from `P3.3V_DELAYED` to `AUD_I2C3_SDA`

These pull-ups are on the codec side of R26A/R27A rather than directly
on the global I2C3 side.

The relationship between this detailed sheet and the document-map
statement remains unresolved.

Possible explanations include schematic revision/ECO differences or a
summary-document mismatch. No conclusion should be made until the
PVT/PVT2 ECO and design-source history have been checked.

## Relevance to the cold-boot investigation

The physical I2C3 bus contains more electrically relevant nodes than the
initial Linux-driver analysis alone exposed.

Confirmed nodes include:

1. i.MX6Q I2C3 controller
2. FT24C512A utility EEPROM
3. ES8328E audio codec branch
4. FPGA I/O pins
5. LCD/display connector

The ES8328E branch additionally has a distinct power-domain condition:
its I2C pins are connected to `P3.3V_DELAYED` through R10B/R11B while
the codec itself is supplied from switched `AUD_P3.3V`.

This does not by itself identify the source or exact electrical
mechanism of the observed arbitration-lost condition.

The corrected schematic topology makes the electrical state of the
ES8328E branch while `AUD_P3.3V` is off a concrete subject for further
investigation.

Possible clamp, back-power and audio power-domain interaction
mechanisms remain hypotheses derived from the circuit topology.

The schematic alone does not establish that any such mechanism occurs
on the tested hardware and does not distinguish among them.

## Open questions

1. What exactly is connected to I2C3 on the Novena eDP adapter?
2. Is IT6251 itself directly connected to these mainboard I2C3 nets?
3. What are the FPGA P3/P4 electrical states before, during and after
   FPGA configuration?
4. What is the exact FPGA configuration/reset sequence during cold POR?
5. What happens electrically on the ES8328E I2C pins while `AUD_P3.3V`
   is off but `P3.3V_DELAYED` is present?
6. Which pull-up values are actually fitted on the tested PVT2-A board?
7. Why does the document-map state 2.2 kohm while sheet 14 shows
   1 kohm pull-ups on `AUD_I2C3_SCL`/`AUD_I2C3_SDA`?
8. Are there relevant PVT2 ECO changes affecting I2C3, FPGA, LCD or the
   audio power circuit?
9. What devices and pull-ups exist on the eDP adapter/flex-cable side?
10. Does the bootloader access I2C3 before Linux on the exact tested
    boot configuration?
11. Does either ES8328E I2C pin source or sink measurable current into
    the powered-down audio domain?
12. Do the global and codec-side SDA/SCL voltages differ measurably
    across R26A/R27A while `AUD_P3.3V` is off?

## Next original-document targets

Continue research block 1 with original Novena documentation:

1. eDP adapter EVT schematic/design source
2. Novena PVT2 ECO list
3. Novena PVT issue log
4. U-Boot PVT notes
5. IT6251 diagnostic documentation

Do not start kernel patch 0007 from these findings alone.
