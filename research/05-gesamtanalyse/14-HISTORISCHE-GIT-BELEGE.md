Stand: 2026-10-03. BLOCK 9C-46A. Lokaler lesender Audit. Hashprüfung und Texterfassung sind keine externe Herkunfts- oder vollständige Inhaltsverifikation.

# Lokal erhobene historische Git-Belege

Diese Ausgaben wurden jetzt lesend aus vorhandenen lokalen Git-Objekten erhoben. Kein Netzwerk, Checkout, Fetch oder Patchauftrag. Inhalt historischer Commits wird nicht als bewiesene Ursache des aktuellen 8B ausgegeben.

## Objekt 9402489e9a3e52a9df5464b4ce3c792cee2ba3a5

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat 9402489e9a3e52a9df5464b4ce3c792cee2ba3a5`. Exitstatus 0.

```text
commit 9402489e9a3e52a9df5464b4ce3c792cee2ba3a5
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Sat Oct 11 21:18:50 2014 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Sat Oct 11 22:23:16 2014 +0800

    novena: Change config to add hdmi, lcd hooks
    
    Add hooks to call out to uenv.txt to configure HDMI, LCD, and other bits.
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 debian/changelog         | 7 +++++++
 include/configs/novena.h | 8 ++++++++
 2 files changed, 15 insertions(+)

```
## Objekt 2ab1217f8c

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat 2ab1217f8c`. Exitstatus 0.

```text
commit 2ab1217f8c70cc266f0157bf9baec86e34e56805
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Fri Oct 17 17:56:14 2014 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Fri Oct 17 17:56:14 2014 +0800

    novena: ddr3: Properly decode page size
    
    We now properly decode byte 29: page size.
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 board/kosagi/novena/ddr3.c | 41 ++++++++++++++++++++++++++++++++++++++---
 1 file changed, 38 insertions(+), 3 deletions(-)

```
## Objekt e08b7460f1

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat e08b7460f1`. Exitstatus 0.

```text
commit e08b7460f140010442c7ec4707cd2c93c6b44b38
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Mon May 18 19:09:17 2015 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Mon May 18 19:09:42 2015 +0800

    board: kosagi: Use canned SPD when it can't be read
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 board/kosagi/novena/ddr3.c | 5 +++--
 1 file changed, 3 insertions(+), 2 deletions(-)

```
## Objekt bd5c7d3290bb94bf8af17760c321df67d2641343

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat bd5c7d3290bb94bf8af17760c321df67d2641343`. Exitstatus 0.

```text
commit bd5c7d3290bb94bf8af17760c321df67d2641343
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Fri Sep 4 10:08:38 2015 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Fri Sep 4 10:08:38 2015 +0800

    debian: Update to v2014.10.r7-novena.1
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 debian/changelog | 6 ++++++
 1 file changed, 6 insertions(+)

```
## Objekt 96284935c2

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat 96284935c2`. Exitstatus 0.

```text
commit 96284935c2b357f9ba6d3c3452e01d3b7edf0edb
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Thu Feb 12 12:36:18 2015 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Mon Feb 16 14:25:43 2015 +0800

    novena: SPL: Add support for ddr3 calibration
    
    Novena allows swapping DDR3 modules, which must be calibrated each
    time the system boots.  Add support for using the i.MX6 MMDC
    calibration routines to perform this function.
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 board/kosagi/novena/Makefile     |   2 +-
 board/kosagi/novena/ddr3.c       | 978 +++++++++++++++++++++++++++++++++++++++
 board/kosagi/novena/novena_spl.c |  21 +-
 3 files changed, 997 insertions(+), 4 deletions(-)

```
## Objekt b5a15faf85

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`, `git show --format=fuller --stat b5a15faf85`. Exitstatus 0.

```text
commit b5a15faf85a52789308f3f59898693b005532833
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Wed Oct 15 19:47:46 2014 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Wed Oct 15 19:47:46 2014 +0800

    novena-spl: Add support for booting from external SD
    
    The factory image will be built using the external SD card, so it is
    important that U-Boot supports starting from the external SD card.
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

 board/kosagi/novena/novena_spl.c | 89 ++++++++++++++++++++++++++++++++++++++--
 1 file changed, 86 insertions(+), 3 deletions(-)

```

## Versionsauszug 9402489e9a3e52a9df5464b4ce3c792cee2ba3a5:board/kosagi/novena/ddr3.c

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`; SHA256 des vollständigen durch git show ausgegebenen Dateitextes: `f75834972afdb596cf885db4d97c69623cb9dc28738dc9cc1615d45e1bacc945`. Exitstatus 0. Nummern sind Zeilen dieser Commitdatei, nicht dieses Berichts. Auslassungen werden markiert.

```text
[Auslassung]
775: 	debug("\tMPDGCTRL0 PHY0 (0x021b083c) = 0x%08X\n", readl(MPDGCTRL0_PHY0));
776: 	debug("\tMPDGCTRL1 PHY0 (0x021b0840) = 0x%08X\n", readl(MPDGCTRL1_PHY0));
777: 	debug("\tMPDGCTRL0 PHY1 (0x021b483c) = 0x%08X\n", readl(MPDGCTRL0_PHY1));
778: 	debug("\tMPDGCTRL1 PHY1 (0x021b4840) = 0x%08X\n", readl(MPDGCTRL1_PHY1));
779: 	debug("Read calibration:\n");
780: 	debug("\tMPRDDLCTL PHY0 (0x021b0848) = 0x%08X\n", readl(MPRDDLCTL_PHY0));
781: 	debug("\tMPRDDLCTL PHY1 (0x021b4848) = 0x%08X\n", readl(MPRDDLCTL_PHY1));
782: 	debug("Write calibration:\n");
783: 	debug("\tMPWRDLCTL PHY0 (0x021b0850) = 0x%08X\n", readl(MPWRDLCTL_PHY0));
784: 	debug("\tMPWRDLCTL PHY1 (0x021b4850) = 0x%08X\n", readl(MPWRDLCTL_PHY1));
785: 	/*
786: 	 * Registers below are for debugging purposes.  These print out
787: 	 * the upper and lower boundaries captured during
788: 	 * read DQS gating calibration.
789: 	 */
790: 	debug("Status registers bounds for read DQS gating:\n");
791: 	debug("\tMPDGHWST0 PHY0 (0x021b087c) = 0x%08X\n", readl(0x021b087c));
792: 	debug("\tMPDGHWST1 PHY0 (0x021b0880) = 0x%08X\n", readl(0x021b0880));
793: 	debug("\tMPDGHWST2 PHY0 (0x021b0884) = 0x%08X\n", readl(0x021b0884));
794: 	debug("\tMPDGHWST3 PHY0 (0x021b0888) = 0x%08X\n", readl(0x021b0888));
795: 	debug("\tMPDGHWST0 PHY1 (0x021b487c) = 0x%08X\n", readl(0x021b487c));
796: 	debug("\tMPDGHWST1 PHY1 (0x021b4880) = 0x%08X\n", readl(0x021b4880));
797: 	debug("\tMPDGHWST2 PHY1 (0x021b4884) = 0x%08X\n", readl(0x021b4884));
798: 	debug("\tMPDGHWST3 PHY1 (0x021b4888) = 0x%08X\n", readl(0x021b4888));
799: 
800: 	debug("Final do_dqs_calibration error mask: 0x%x\n", errors);
801: 
802: 	return errors;
803: }
804: 
805: uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
806: 			 struct mx6_ddr3_cfg *cfg)
807: {
808: 	struct ddr3_spd_eeprom_s spd;
809: 	int ret;
810: 	int chip   = 0x50;	/* Address of the SPD I2C device */
811: 	int addr   = 0x0;	/* Start of SPD address */
812: 	int alen   = 0x1;	/* Address is one-byte long */
813: 	uint32_t capacity;
814: 	uint32_t mtb_scale;
815: 
816: 	i2c_init(CONFIG_SYS_I2C_SPEED, 0);
817: 
818: 	ret = i2c_read(chip, addr, alen, (uint8_t *)&spd, sizeof(spd));
819: 	if (ret != 0) {
820: 		printf("Error reading SPD on DDR3.\n");
821: 		dram_fatal(-ret);
822: 	}
823: 
824: 	switch (spd.tck_min) {
825: 	case 0x14:
826: 		cfg->mem_speed = 800;
827: 		break;
828: 	case 0x0f:
829: 		cfg->mem_speed = 1066;
830: 		break;
831: 	case 0x0c:
832: 		cfg->mem_speed = 1333;
833: 		break;
834: 	case 0x0a:
835: 		cfg->mem_speed = 1600;
836: 		break;
837: 	case 0x09:
838: 		cfg->mem_speed = 1866;
839: 		break;
840: 	case 0x08:
841: 		cfg->mem_speed = 2133;
842: 		break;
843: 	default:
844: 		if (spd.tck_min <= 0xf) {
845: 			debug("**undecodable but sufficiently fast "
846: 					"clock rate detected\n");
847: 			cfg->mem_speed = 1066;
848: 		} else {
849: 			debug("**undecodable but too slow "
850: 					"clock rate detected\n");
851: 			cfg->mem_speed = 800;
852: 		}
853: 		break;
854: 	}
855: 	debug("\tDDR3-%d speed rating detected\n", cfg->mem_speed);
856: 	if (cfg->mem_speed < 1066) {
857: 		printf("memory is too slow.\n");
858: 		dram_fatal(0x05);
859: 	}
860: 
861: 	cfg->density = (256 * (1 << (spd.density_banks & 0xf))) / 1024;
862: 	debug("\tIndividual chip density is %d Gib\n", cfg->density);
863: 
864: 	cfg->width = (1 << (spd.organization & 0x7)) * 4;
865: 	debug("\tChips have a width of %d bits\n", cfg->width);
866: 
867: 	if ((spd.density_banks & 0x30) != 0) {
868: 		printf("\tWarning: memory has an unsupported bank size\n");
869: 		dram_fatal(0x06);
870: 	}
871: 	cfg->banks = 8;
872: 	debug("\t%d address banks\n", cfg->banks);
873: 
874: 	cfg->rowaddr = ((spd.addressing >> 3) & 0x7) + 12;
875: 	cfg->coladdr = (spd.addressing & 0x7) + 9;
876: 	debug("\tRows: %d, Cols: %d\n", cfg->rowaddr, cfg->coladdr);
877: 
878: 	cfg->pagesz = 1;
879: 	debug("\t!!! Guessing a pagesz of 1 !!!\n");
880: 
881: 	sysinfo->ncs = ((spd.organization >> 3) & 0x7) + 1;
882: 	debug("\tThere are %d chip selects (ranks)\n", sysinfo->ncs);
883: 
884: 	capacity = (64 / cfg->width) * sysinfo->ncs * (cfg->density * 1024);
885: 	debug("\tTotal memory capacity: %d Mib (%d MiB)\n",
886: 			capacity, capacity / 8);
887: 
888: 	/* We only support 64-bit modules */
889: 	if (spd.bus_width != 0x3) {
890: 		printf("Unsupported device width, fatal.\n");
891: 		dram_fatal(0x03);
892: 	}
893: 	sysinfo->dsize = 2;
894: 	debug("\tModule width is 64 bits, no ECC\n");
895: 
896: 	sysinfo->cs_density = (capacity / (256 * sysinfo->ncs)) / 4;
897: 	debug("\tCS density: %d Gb\n", sysinfo->cs_density);
898: 
899: 	/*
900: 	 * MTB is in periods of some fraction of nanoseconds.  Convert to
901: 	 * ns*100, which is what MX6 uses.
902: 	 */
903: 	mtb_scale = (spd.mtb_dividend * 100) / spd.mtb_divisor;
904: 
905: 	cfg->trcd = spd.trcd_min + 1;
906: 	cfg->trcd *= mtb_scale;
907: 	debug("\ttRCD: %d\n", cfg->trcd);
908: 
909: 	cfg->trcmin = (((spd.tras_trc_ext & 0xf0) << 4) | spd.trc_min_lsb) + 1;
910: 	cfg->trcmin *= mtb_scale;
911: 	debug("\ttRC: %d\n", cfg->trcmin);
912: 
913: 	cfg->trasmin = (((spd.tras_trc_ext & 0xf) << 8) | spd.tras_min_lsb) + 1;
914: 	cfg->trasmin *= mtb_scale;
915: 	debug("\ttRAS: %d\n", cfg->trasmin);
916: 
917: 	/* Set self-refresh temperature flag to extended range, if present */
918: 	cfg->SRT = 0;
919: 	if ((spd.therm_ref_opt & 0x1) && !(spd.therm_ref_opt & 0x2))
920: 		cfg->SRT = 1;
921: 
922: 	/* Items with (*) will be calibrated later */
923: 	sysinfo->cs1_mirror = 0;	/* Novena doesn't use mirroring */
924: 	sysinfo->bi_on = 0;		/* Novena doesn't use interleaving */
925: 	sysinfo->rtt_nom = 2;		/* (*) RTT_Nom = RZQ/2 */
926: 	sysinfo->rtt_wr = 2;		/* (*) RTT_Wr = RZQ/4 */
927: 	sysinfo->ralat = 3;		/* (*) Read additional latency */
928: 	sysinfo->walat = 3;		/* (*) Write additional latency */
929: 	sysinfo->mif3_mode = 3;		/* Command prediction working mode */
930: 	sysinfo->rst_to_cke = 0x23;	/* 33 cycles, 500us (JEDEC default) */
931: 	sysinfo->sde_to_rst = 0x10;	/* 14 cycles, 200us (JEDEC default) */
932: 
933: 	return 0;
934: }
935: 
936: int novena_memory_test(void)
937: {
938: #ifdef CONFIG_PERFORM_RAM_TEST
939: 	write_random_data(0);
940: 	return read_random_data(0);
941: #endif
942: 	return 0;
943: }
```

## Versionsauszug bd5c7d3290bb94bf8af17760c321df67d2641343:board/kosagi/novena/ddr3.c

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`; SHA256 des vollständigen durch git show ausgegebenen Dateitextes: `3453a653e08e2f30a1132dcc8eb5c85a16eb03be678d1426d9e50282bb31c0fc`. Exitstatus 0. Nummern sind Zeilen dieser Commitdatei, nicht dieses Berichts. Auslassungen werden markiert.

```text
[Auslassung]
277: 
278: 	writel(val_ctrl, reg_ctrl);
279: }
280: 
281: int do_write_level_calibration(void)
282: {
283: 	int esdmisc_val, zq_val;
284: 	int errors = 0;
285: 	int ldectrl[4];
286: 	uint32_t ddr_mr1 = 0x4;
287: 
288: 	/*
289: 	 * Stash old values in case calibration fails,
290: 	 * we need to restore them
291: 	 */
292: 	ldectrl[0] = read_mmdcp0(MPWLDECTRL0);
293: 	ldectrl[1] = read_mmdcp0(MPWLDECTRL1);
294: 	ldectrl[2] = read_mmdcp1(MPWLDECTRL0);
[Auslassung]
412: 
413: 	return errors;
414: }
415: 
416: int do_dqs_calibration(void)
417: {
418: 	int esdmisc_val;
419: 	int bus_size;
420: 	int temp_ref;
421: 	int cs0_enable;
422: 	int cs1_enable;
423: 	int cs0_enable_initial;
424: 	int cs1_enable_initial;
425: 	int PDDWord = 0x00ffff00; /* best so far, place into MPPDCMPR1 */
426: 	int errors = 0;
427: 	unsigned int initdelay = 0x40404040;
428: 
429: 	/* check to see which chip selects are enabled */
[Auslassung]
798: 	debug("\tMPDGHWST1 PHY1 (0x021b4880) = 0x%08X\n", readl(0x021b4880));
799: 	debug("\tMPDGHWST2 PHY1 (0x021b4884) = 0x%08X\n", readl(0x021b4884));
800: 	debug("\tMPDGHWST3 PHY1 (0x021b4888) = 0x%08X\n", readl(0x021b4888));
801: 
802: 	debug("Final do_dqs_calibration error mask: 0x%x\n", errors);
803: 
804: 	return errors;
805: }
806: 
807: uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
808: 			 struct mx6_ddr3_cfg *cfg)
809: {
810: 	struct ddr3_spd_eeprom_s spd;
811: 	int ret;
812: 	int chip   = 0x50;	/* Address of the SPD I2C device */
813: 	int addr   = 0x0;	/* Start of SPD address */
814: 	int alen   = 0x1;	/* Address is one-byte long */
815: 	uint32_t capacity;
816: 	uint32_t mtb_scale;
817: 
818: 	i2c_init(CONFIG_SYS_I2C_SPEED, 0);
819: 
820: 	ret = i2c_read(chip, addr, alen, (uint8_t *)&spd, sizeof(spd));
821: 	if (ret != 0) {
822: 		printf("Error reading SPD on DDR3: %d\n", ret);
823: 		printf("Using canned SPD instead.\n");
824: 		memcpy((void *)&spd, default_spd, sizeof(spd));
825: 	}
826: 
827: 	if (ddr3_spd_check(&spd)) {
828: 		printf("Using canned SPD\n");
829: 		memcpy((void *)&spd, default_spd, sizeof(spd));
830: 	}
831: 
832: 	switch (spd.tck_min) {
833: 	case 0x14:
834: 		cfg->mem_speed = 800;
835: 		if (spd.tfaw_min == 0x40) {
836: 			cfg->pagesz = 1;
837: 			debug("\t1K page size\n");
838: 		} else if (spd.tfaw_min == 0x90) {
839: 			cfg->pagesz = 2;
840: 			debug("\t2K page size\n");
841: 		}
842: 		else
[Auslassung]
935: 	}
936: 	sysinfo->dsize = 2;
937: 	debug("\tModule width is 64 bits, no ECC\n");
938: 
939: 	sysinfo->cs_density = (capacity / (256 * sysinfo->ncs)) / 4;
940: 	debug("\tCS density: %d Gb\n", sysinfo->cs_density);
941: 
942: 	/*
943: 	 * MTB is in periods of some fraction of nanoseconds.  Convert to
944: 	 * ns*100, which is what MX6 uses.
945: 	 */
946: 	mtb_scale = (spd.mtb_dividend * 100) / spd.mtb_divisor;
947: 
948: 	cfg->trcd = spd.trcd_min + 1;
949: 	cfg->trcd *= mtb_scale;
950: 	debug("\ttRCD: %d\n", cfg->trcd);
951: 
952: 	cfg->trcmin = (((spd.tras_trc_ext & 0xf0) << 4) | spd.trc_min_lsb) + 1;
953: 	cfg->trcmin *= mtb_scale;
954: 	debug("\ttRC: %d\n", cfg->trcmin);
955: 
956: 	cfg->trasmin = (((spd.tras_trc_ext & 0xf) << 8) | spd.tras_min_lsb) + 1;
957: 	cfg->trasmin *= mtb_scale;
958: 	debug("\ttRAS: %d\n", cfg->trasmin);
959: 
[Auslassung]
975: 
976: 	return 0;
977: }
978: 
979: int novena_memory_test(void)
980: {
981: #ifdef CONFIG_PERFORM_RAM_TEST
982: 	write_random_data(0);
983: 	return read_random_data(0);
984: #endif
985: 	return 0;
986: }
```

## Versionsauszug bd5c7d3290bb94bf8af17760c321df67d2641343:board/kosagi/novena/novena_spl.c

Quelle: `/home/loomit/novena-historical-research/u-boot-novena`; SHA256 des vollständigen durch git show ausgegebenen Dateitextes: `91ef39f992e72cfda108d8693b00659f1b9faaac53a6d371fb001e1cee2af483`. Exitstatus 0. Nummern sind Zeilen dieser Commitdatei, nicht dieses Berichts. Auslassungen werden markiert.

```text
[Auslassung]
479: 	/* There is no CD for a microSD card, assume always present. */
480: 	return 1;
481: }
482: 
483: int board_mmc_init(bd_t *bis)
484: {
485: 	enum boot_device dev = get_boot_device();
486: 
487: 	/* Internal MMC */
488: 	switch (dev) {
489: 	case MX6_SD0_BOOT:	/* Internal SD card */
490: 		puts("Internal SD card\n");
491: 		usdhc3_cfg.sdhc_clk = mxc_get_clock(MXC_ESDHC3_CLK);
492: 		return fsl_esdhc_initialize(bis, &usdhc3_cfg);
493: 
494: 	case MX6_SD1_BOOT:	/* External SD card */
495: 		puts("External SD card\n");
496: 		usdhc2_cfg.sdhc_clk = mxc_get_clock(MXC_ESDHC2_CLK);
[Auslassung]
507: 	return 0;
508: }
509: #endif
510: 
511: uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
512: 		         struct mx6_ddr3_cfg *cfg);
513: int do_write_level_calibration(void);
514: int do_dqs_calibration(void);
515: int novena_memory_test(void);
516: 
517: /* Configure MX6Q/DUAL mmdc DDR io registers */
518: static struct mx6dq_iomux_ddr_regs novena_ddr_ioregs = {
519: 	/* SDCLK[0:1], CAS, RAS, Reset: Differential input, 40ohm */
520: 	.dram_sdclk_0		= 0x00020038,
521: 	.dram_sdclk_1		= 0x00020038,
522: 	.dram_cas		= 0x00000038,
523: 	.dram_ras		= 0x00000038,
524: 	.dram_reset		= 0x00000038,
525: 	/* SDCKE[0:1]: 100k pull-up */
526: 	.dram_sdcke0		= 0x00003000,
527: 	.dram_sdcke1		= 0x00003000,
528: 	/* SDBA2: pull-up disabled */
[Auslassung]
596: static struct mx6_ddr_sysinfo novena_ddr_info = {
597: 	/* Width of data bus: 0=16, 1=32, 2=64 */
598: 	.dsize		= 2,
599: 	/* Config for full 4GB range so that get_mem_size() works */
600: 	.cs_density	= 32,	/* 32Gb per CS */
601: 	/* Single chip select */
602: 	.ncs		= 1,
603: 	.cs1_mirror	= 0,
604: 	.rtt_wr		= 1,	/* RTT_Wr = RZQ/4 */
605: 	.rtt_nom	= 2,	/* RTT_Nom = RZQ/2 */
606: 	.walat		= 3,	/* Write additional latency */
607: 	.ralat		= 7,	/* Read additional latency */
608: 	.mif3_mode	= 3,	/* Command prediction working mode */
609: 	.bi_on		= 1,	/* Bank interleaving enabled */
610: 	.sde_to_rst	= 0x10,	/* 14 cycles, 200us (JEDEC default) */
611: 	.rst_to_cke	= 0x23,	/* 33 cycles, 500us (JEDEC default) */
612: };
613: 
[Auslassung]
686: 	/* UART clocks enabled and gd valid - init serial console */
687: 	preloader_console_init();
688: 
689: 	/* Start the DDR DRAM */
690: 	novena_read_spd(&novena_ddr_info, &novena_ddr3_cfg);
691: 	mx6dq_dram_iocfg(novena_ddr3_cfg.width,
692: 			 &novena_ddr_ioregs, &novena_grp_ioregs);
693: 	mx6_dram_cfg(&novena_ddr_info, &novena_mmdc_calib, &novena_ddr3_cfg);
694: 	do_write_level_calibration();
695: 	do_dqs_calibration();
696: 	printf("Running post-config memory test... ");
697: 	if (novena_memory_test())
698: 		printf("Fail!\n");
699: 	else
700: 		printf("Pass\n");
701: 
702: 	/* Clear the BSS. */
703: 	memset(__bss_start, 0, __bss_end - __bss_start);
704: 
705: 	/* load/boot image from boot device */
706: 	board_init_r(NULL, 0);
707: }
708: 
709: void reset_cpu(ulong addr)
710: {
```

## Historisches Delta 2ab1217f8c

```diff
commit 2ab1217f8c70cc266f0157bf9baec86e34e56805
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Fri Oct 17 17:56:14 2014 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Fri Oct 17 17:56:14 2014 +0800

    novena: ddr3: Properly decode page size
    
    We now properly decode byte 29: page size.
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

diff --git a/board/kosagi/novena/ddr3.c b/board/kosagi/novena/ddr3.c
index eb78def774..1f8b8377d2 100644
--- a/board/kosagi/novena/ddr3.c
+++ b/board/kosagi/novena/ddr3.c
@@ -824,20 +824,58 @@ uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
 	switch (spd.tck_min) {
 	case 0x14:
 		cfg->mem_speed = 800;
+		if (spd.tfaw_min == 0x40) {
+			cfg->pagesz = 1;
+			debug("\t1K page size\n");
+		} else if (spd.tfaw_min == 0x90) {
+			cfg->pagesz = 2;
+			debug("\t2K page size\n");
+		}
+		else
+			debug("Unknown page size: 0x%02x\n", spd.tfaw_min);
 		break;
 	case 0x0f:
 		cfg->mem_speed = 1066;
+		if (spd.tfaw_min == 0x2c) {
+			cfg->pagesz = 1;
+			debug("\t1K page size\n");
+		} else if (spd.tfaw_min == 0x90) {
+			cfg->pagesz = 2;
+			debug("\t2K page size\n");
+		}
+		else
+			debug("Unknown page size: 0x%02x\n", spd.tfaw_min);
 		break;
 	case 0x0c:
 		cfg->mem_speed = 1333;
+		if (spd.tfaw_min == 0xf0) {
+			cfg->pagesz = 1;
+			debug("\t1K page size\n");
+		} else if (spd.tfaw_min == 0x68) {
+			cfg->pagesz = 2;
+			debug("\t2K page size\n");
+		}
+		else
+			debug("Unknown page size: 0x%02x\n", spd.tfaw_min);
 		break;
 	case 0x0a:
 		cfg->mem_speed = 1600;
+		if (spd.tfaw_min == 0xf0) {
+			cfg->pagesz = 1;
+			debug("\t1K page size\n");
+		} else if (spd.tfaw_min == 0x40) {
+			cfg->pagesz = 2;
+			debug("\t2K page size\n");
+		}
+		else
+			debug("Unknown page size: 0x%02x\n", spd.tfaw_min);
 		break;
 	case 0x09:
+		cfg->pagesz = 1;
 		cfg->mem_speed = 1866;
 		break;
 	case 0x08:
+		cfg->pagesz = 1;
 		cfg->mem_speed = 2133;
 		break;
 	default:
@@ -875,9 +913,6 @@ uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
 	cfg->coladdr = (spd.addressing & 0x7) + 9;
 	debug("\tRows: %d, Cols: %d\n", cfg->rowaddr, cfg->coladdr);
 
-	cfg->pagesz = 1;
-	debug("\t!!! Guessing a pagesz of 1 !!!\n");
-
 	sysinfo->ncs = ((spd.organization >> 3) & 0x7) + 1;
 	debug("\tThere are %d chip selects (ranks)\n", sysinfo->ncs);
 

```

## Historisches Delta e08b7460f1

```diff
commit e08b7460f140010442c7ec4707cd2c93c6b44b38
Author:     Sean Cross <xobs@kosagi.com>
AuthorDate: Mon May 18 19:09:17 2015 +0800
Commit:     Sean Cross <xobs@kosagi.com>
CommitDate: Mon May 18 19:09:42 2015 +0800

    board: kosagi: Use canned SPD when it can't be read
    
    Signed-off-by: Sean Cross <xobs@kosagi.com>

diff --git a/board/kosagi/novena/ddr3.c b/board/kosagi/novena/ddr3.c
index 8a1cd452c4..a9fc1eb442 100644
--- a/board/kosagi/novena/ddr3.c
+++ b/board/kosagi/novena/ddr3.c
@@ -819,8 +819,9 @@ uint32_t novena_read_spd(struct mx6_ddr_sysinfo *sysinfo,
 
 	ret = i2c_read(chip, addr, alen, (uint8_t *)&spd, sizeof(spd));
 	if (ret != 0) {
-		printf("Error reading SPD on DDR3.\n");
-		dram_fatal(-ret);
+		printf("Error reading SPD on DDR3: %d\n", ret);
+		printf("Using canned SPD instead.\n");
+		memcpy((void *)&spd, default_spd, sizeof(spd));
 	}
 
 	if (ddr3_spd_check(&spd)) {

```

## Image-/Installerbaum /home/loomit/novena-historical-research/novena-image

HEAD: `f62c4f3b452199ab882e2c3dba16d4e31dfb1d56`.

- `/home/loomit/novena-historical-research/novena-image/README.md:19`: non-ARM system is a "cross-build" and uses qemu-user to complete the
- `/home/loomit/novena-historical-research/novena-image/README.md:20`: installation. This require the executable `qemu-arm-static` to be
- `/home/loomit/novena-historical-research/novena-image/README.md:22`: `qemu-arm-static` on Debian is `qemu-user-static`, on Ubuntu it's
- `/home/loomit/novena-historical-research/novena-image/README.md:23`: `qemu-arm-static`.
- `/home/loomit/novena-historical-research/novena-image/README.md:34`: For the 4 GB "Bare Board" model and laptop recovery partition, we recommend
- `/home/loomit/novena-historical-research/novena-image/README.md:40`:         avahi-daemon avahi-dnsconfd libnss-mdns debootstrap psutils \
- `/home/loomit/novena-historical-research/novena-image/README.md:66`:         libnss-mdns btrfs-tools dosfstools parted debootstrap python \
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:9`: mirror="http://127.0.0.1:3142/ftp.hk.debian.org/debian"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:28`: # Set to 1 (using -q) to skip partitioning, formatting, and bootstrapping.
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:41`: novena_repo="http://127.0.0.1:3142/repo.novena.io/repo"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:44`: # and enable cross-debootstrapping
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:113`: 		rm -f "${root}/usr/bin/qemu-arm-static"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:136`: partition_disk() {
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:149`: 		# "NovM"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:150`: 		disksig=4e6f764d
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:154`: 		# "NovS"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:155`: 		disksig=4e6f7653
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:162`: 	fdisk ${diskname} -C32 -H32 <<EOF
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:195`: 		fail "fdisk returned error $stat"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:211`: 		partition_disk "${filename}" "${disktype}"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:224`: 	local root="$3"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:233`: 	# MMC devices are weirdly labeled mmcblk0p1 rather than mmcblk01
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:234`: 	elif echo "${diskname}" | grep -q mmcblk
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:241`: 		mkfs.vfat ${diskname}1 || fail "Unable to make boot partition"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:255`: 	local root="$2"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:260`: 	local DEBOOTSTRAP_OPTS=""
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:263`: 	    DEBOOTSTRAP_OPTS="--foreign --arch=armhf"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:266`: 	debootstrap ${DEBOOTSTRAP_OPTS} "${suite}" "${root}" "${mirror}" || fail "Unable to debootstrap"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:270`: 	    info "Finishing debootstrap via QEMU"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:271`: 	    cp `which qemu-arm-static` "${root}/usr/bin"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:272`: 	    chroot "${root}" /debootstrap/debootstrap --second-stage
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:277`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:305`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:314`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:325`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:332`: 	    # "debootstrap --foreign" creates a blank sources.list
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:358`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:375`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:394`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:418`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:425`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:438`: 	local root="$1"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:448`: 	info "Writing U-Boot SPL file '${spl}' to disk"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:467`: 	echo "	  -d  --disk	       A path to the block device to partition,"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:480`: 	echo "                         at /usr/share/debootstrap/scripts"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:484`: 	echo "    -q  --quick          Don't repartition, reformat, or botstrap."
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:507`: 		-r|--root) root="$2"; shift 2 ;;
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:533`:     if ! which qemu-arm-static > /dev/null
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:535`: 	fail "Host system is not non-armhf, so qemu-arm-static must be available on PATH for cross-building."
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:537`:     info "Cross-building Novena image using qemu"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:551`: 		root="/tmp/newroot.$$"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:564`: 		# prepare_loopback will partition the disk if necessary
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:568`: 		# Partition a regular disk here
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:571`: 			partition_disk "${diskname}" "${disktype}"
- `/home/loomit/novena-historical-research/novena-image/novena-image.sh:637`: 	write_uboot_spl "${root}" "/boot/u-boot.spl" "${diskname}"

## Image-/Installerbaum /home/loomit/novena-historical-research/novena-debian-installer

HEAD: `ebdb816b93f91bf32809a8c4c1454d4ef7f2bb5f`.

- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:6`: At the moment it is designed to do a installation from a MicroSD card to a SSD or Harddisk on the SATA port, where the root partition will end up in /dev/sda2, or the system uses an LUKS encrypted partition.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:7`: If you have a different partitioning layout, please adapt the uEnv*.txt accordingly.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:9`: To use the installer, you can get a ready-made image from http://www2.futureware.at/novena/novena-installer-microsd.img (sha512sum is fb4a59938ea2fea47af401825c532e5637e62e839abc4c638334e4aedb067e81b4eb886e61653c39d02c365dc55636c7f83c5b96f3ea1c48f6793c1c7efa42d0), copy that to a MicroSD card with dd or some other imaging tool, and then boot your Novena from that MicroSD card.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:19`: To build the image yourself, take the standard Novena Micro-SD image ( http://repo.novena.io/novena/images/novena-mmc-disk-r1.img ), enlarge the boot partition (or copy the files and create a new msdos partition), run makeinstall.sh on a Novena system, and then copy the files that are generated in boot/ to the boot partition.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:21`: The installer will automatically rename the uInitrd to uInitrd-install at the end of the installation, to make sure that the system boots after the installation instead of looping in the installer.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:22`: If you want to run the installer again after an installation is completed, just rename uInitrd-install back to uInitrd on the MicroSD card, and the installer will run again on the next boot.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:25`: The first one /etc/initramfs-tools/hooks/novena.hook.sh works inside mkinitramfs to add missing kernel modules and firmware images. The second one /etc/kernel/postinst.d/zzz-novena-mkimage runs after mkinitramfs to convert the image to U-Boot format and to copy it onto the SD-card. If you have a non-standard Novena configuration, you might have to adapt those scripts to your environment.
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:37`: KNOWN BUGS
- `/home/loomit/novena-historical-research/novena-debian-installer/README.md:39`: If you get a problem with the clock set up, the install will fail. you must disconnect AC and battery power and restart the install. This is related to problems described here: http://www.kosagi.com/forums/viewtopic.php?id=369
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:2`: wget -O initrd.gz http://linux.citylink.co.nz/debian/dists/stable/main/installer-armhf/current/images/netboot/initrd.gz
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:5`: wget -O hooks.tgz http://www2.futureware.at/novena/hooks.tgz
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:9`: wget -O kosagi.gpg https://github.com/xobs/kosagi-repo/raw/master/etc/apt/trusted.gpg.d/kosagi.gpg
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:12`: sudo apt-get install u-boot-tools lvm2 cryptsetup kosagi-repo initramfs-tools build-essential u-boot-novena exfat-utils linux-image-novena linux-firmware-image-novena linux-headers-novena
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:16`: gunzip <initrd.gz >initrd-unpacked
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:24`: cpio -i <../initrd-unpacked
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:41`: echo Copying fdisk for partitioning
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:42`: cp /sbin/fdisk sbin/fdisk
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:44`: echo Copying mkimage
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:45`: cp /usr/bin/mkimage bin/mkimage
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:52`: find . | cpio -H newc -o >../newinitrd 
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:57`: gzip <newinitrd >newinitrd.gz
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:61`: mkimage -A arm -T ramdisk -C none -n uInitrd -d ./newinitrd.gz ./boot/uInitrd
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:65`: echo "Now copy boot/uInitrd and boot/uEnv.txt into the /boot partition"
- `/home/loomit/novena-historical-research/novena-debian-installer/makeimage.sh:66`: echo "If you want to rerun the installer later on, just rename uInitrd-install to uInitrd again"
- `/home/loomit/novena-historical-research/novena-debian-installer/uEnv.txt:1`: initrd_addr_r=0x12800000
- `/home/loomit/novena-historical-research/novena-debian-installer/uEnv.txt:2`: fh_initramfs_try=fatload \${bootsrc} \${bootdev} \${initrd_addr_r} uInitrd\${rec}
- `/home/loomit/novena-historical-research/novena-debian-installer/uEnv.txt:3`: fh_initramfs_ok=echo initramfs uInitrd\${rec} loaded; setenv bootargs rootwait ro; setenv rootdev UUID=fa7f372b-d890-4a8c-877e-7e201e997682
- `/home/loomit/novena-historical-research/novena-debian-installer/uEnv.txt:4`: fh_initramfs_no=echo initramfs uInitrd\${rec} not found; setenv initrd_addr_r -; setenv bootargs rootwait rw ; setenv rootdev /dev/sda2
- `/home/loomit/novena-historical-research/novena-debian-installer/uEnv.txt:5`: finalhook=if run fh_initramfs_try; then run fh_initramfs_ok; else run fh_initramfs_no; fi; setenv bootargs \${bootargs} root=\${rootdev} console=\${consdev}

## Interpretation

BELEGT: rc5 besitzt SPD-Auswertung; ein historischer Commit korrigiert page size; der spätere bd5c7d3-Dateistand enthält Lesefehler-/CRC-Ersatz durch canned SPD. Dies ist kein Beweis, dass der im Test benutzte aktuelle DIMM SPD-Lesefehler hat oder dass Ersatz-SPD universell geeignet ist. Mainline-Mailpatchzuordnung, sämtliche Kalibrierungsdeltas und Registersemantik bleiben offen. Historische Image-/Installerwerkzeuge wurden ausschließlich gelesen, niemals ausgeführt.
