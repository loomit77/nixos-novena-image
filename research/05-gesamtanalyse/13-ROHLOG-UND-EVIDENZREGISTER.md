Stand: 2026-10-03. BLOCK 9C-46A. Lokaler lesender Audit. Hashprüfung und Texterfassung sind keine externe Herkunfts- oder vollständige Inhaltsverifikation.

# Rohlog- und Evidenzregister

Alle folgenden Dateien wurden vollständig lokal gelesen. Hashidentität, Textfund und experimentelle Durchführung sind getrennte Ebenen. Entwürfe/vor-8e-Kopien sind Dokumentation, Buildlogs keine Hardwarelogs.

## Repository-Testlogs

| Datei | Byte | Zeilen (LF) | SHA256 | Belegart |
|---|---:|---:|---|---|
| `test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log` | 1480 | 26 | `45fb5e00d5ba7be6a42afc7491522cb8bcc084fcd8142582cbeea4ddbb39595a` | Buildausgabe |
| `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log` | 2434 | 32 | `e4b11db1cba820097f55bcbd6b6076577d35c66645317f5668ee716a5fbfd7c0` | Buildausgabe |
| `test-logs/2026-10-01/block-4.10o-1n-8e-decisions.entwurf.md` | 1275 | 38 | `8d01cc432ea751f2a0bdb1483d87dc3908c2ebf12bdd88a94228c2dfcf4a8572` | Dokumentkopie/Entwurf |
| `test-logs/2026-10-01/block-4.10o-1n-8e-project-state.entwurf.md` | 1284 | 28 | `a016c035477397b680e52c3361490207cfa41d18e0601c2e95e3beccec0dfb86` | Dokumentkopie/Entwurf |
| `test-logs/2026-10-01/block-4.10o-1n-8e-test-log.entwurf.md` | 2925 | 122 | `d31fd92d830c19bb3b373e83c1b5d4afd6880f6c952472304ab091fd52ca4028` | Dokumentkopie/Entwurf |
| `test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log` | 830 | 17 | `9d4af3d5b5e846072ef14977eea947ba5b3c244c1079cd9389dcb23d0a32cf61` | Serieller Originalrohtext |
| `test-logs/2026-10-01/decisions.vor-8e.md` | 37803 | 929 | `aebafda36eb679d7a5fea35a9cf2248c3ae6028ba2a63825f610047d33a44e0b` | Dokumentkopie/Entwurf |
| `test-logs/2026-10-01/project-state.vor-8e.md` | 190341 | 5680 | `a79bb72207d5f6eb04783f6acad7fa2fa42d7d29a2e65027b186caba79336ff3` | Dokumentkopie/Entwurf |
| `test-logs/2026-10-01/test-log.vor-8e.md` | 68325 | 2207 | `68edd388f394ff89164b95a6ed1a7dfad682ee4dacf250b9bb022ec328ba5796` | Dokumentkopie/Entwurf |
| `test-logs/p-ext-2026-09-22/p-ext-boot-01.log` | 35434 | 502 | `506f8eaf409b8d9a6e40fced1da822df43b865df507d08d094d9106046e7debb` | Serieller Originalrohtext |
| `test-logs/p-ext-2026-09-22/p-ext-diag-boot-01.log` | 148 | 4 | `4773f008e9dda2c0c6cfbf5695a73877b60178bb45ba2c755a2a431921dabc41` | Serieller Originalrohtext |
| `test-logs/p-ext-2026-09-23/block-9h-build.log` | 202 | 2 | `3cc38822bf8378ca1b7d3b6ed1d1f0005e3a482309ce70e6f1df290b0cb89089` | Buildausgabe |
| `test-logs/p-ext-2026-09-23/p-ext-d0-d36-boot-01.log` | 492 | 12 | `28b9b3c79b98e8f9d8548fa6c48755973fa5ea6ce61407a44e0c25bc85d65c67` | Serieller Originalrohtext |
| `test-logs/p-ext-2026-09-23/p-ext-d21a-d21d-boot-01.log` | 627 | 14 | `f123e33b91abdc530a8c3fc440f59d41ece1f1f16b90815f067a97b999466c95` | Serieller Originalrohtext |
| `test-logs/p-ext-2026-09-23/p-ext-p0-p1-boot-01.log` | 729 | 16 | `3f732abfdde8122068752c13aa2d6a6418e926ec89836566e683345e0743507e` | Serieller Originalrohtext |
| `test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log` | 772 | 16 | `5fa66b25959c862e7b985022a1c76037b3668942fa10741f8b3cd7754ba25d54` | Serieller Originalrohtext |

## Vollständige kurze P_EXT-Seriellogs

Nur Darstellungsnormalisierung: CR wird für die Anzeige entfernt. Literal `\n` bleibt erhalten. Die Rohdateien bleiben unverändert und ihre Hashes stehen oben. Das lange p-ext-boot-01.log wird unten durch alle Bootsegmentgrenzen und das Ende dokumentiert; keine zweite Kopie des gesamten Linuxtexts wird als neue Originalquelle ausgegeben.

### test-logs/2026-10-01/block-4.10o-1n-patch-0008-por-01.raw.log

```text

U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
NOVENA-DIAG Q1/Q2 q1=00000000 q2=00000000\nNOVENA-DIAG C0/C1/C2 c0=00000000 c1=00000000 c2=00000000
NOVENA-DIAG P0 after mem_malloc_init brk=00000000
Trying to boot from MMC1
NOVENA-DIAG P1 before loader callback brk=00000000
NOVENA-DIAG D0 before spl_mmc_find_device dev=0
NOVENA-DIAG D16 enter mmc_initialize
NOVENA-DIAG D17 after mmc_list_init
NOVENA-DIAG D18 enter board_mmc_init
NOVENA-DIAG D19 before mxc_get_clock
NOVENA-DIAG D20 after mxc_get_clock clk=198000000
NOVENA-DIAG D21 enter fsl_esdhc_initialize
NOVENA-DIAG D21A before priv calloc start=18300000 end=18400000 brk=00000000
NOVENA-DIAG D21B after priv calloc priv=? brk=00000000
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
NOVENA-DIAG D33 after board_mmc_init ret=-12

```

### test-logs/p-ext-2026-09-22/p-ext-boot-01.log

- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:2`: U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:5`: U-Boot SPL 2015.04-rc1-02853-gbd5c7d3 (Sep 04 2015 - 10:11:08)
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:7`: Internal SD card
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:12`: U-Boot 2015.04-rc1-02853-gbd5c7d3 (Sep 04 2015 - 10:11:08)
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:15`: Reset cause: POR
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:51`: Starting kernel ...
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:54`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:72`: [    0.000000] Kernel command line: systemConfig=/nix/store/axbsx57xnrbqdya5bk877g21g87kish7-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/axbsx57xnrbqdya5bk877g21g87kish7-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:246`: [    2.647969] imx-sdma 20ec000.dma-controller: external firmware not found, using ROM firmware
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:380`: [    4.954587] systemd[1]: Reached target Path Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:381`: [    4.954938] systemd[1]: Reached target Slice Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:382`: [    4.955307] systemd[1]: Reached target Swaps.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:383`: [    4.955634] systemd[1]: Reached target Timer Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:389`: [    4.960159] systemd[1]: Reached target Socket Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:398`: [    5.073101] systemd[1]: Mounted Kernel Configuration File System.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:413`: [   14.493662] EXT4-fs (mmcblk1p2): mounted filesystem 44444444-4444-4444-8888-888888888888 r/w with ordered data mode. Quota mode: none.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:432`: [   31.403879] systemd[1]: Reached target Local Encrypted Volumes.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:436`: [   31.405337] systemd[1]: Reached target Virtual Machines and Containers.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:437`: [   31.405788] systemd[1]: Reached target Path Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:438`: [   31.406169] systemd[1]: Reached target Remote File Systems.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:439`: [   31.406537] systemd[1]: Reached target Slice Units.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:440`: [   31.406928] systemd[1]: Reached target Swaps.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:462`: [   31.731971] systemd[1]: Starting Remount Root and Kernel File Systems...
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:473`: [   31.941007] EXT4-fs (mmcblk1p2): re-mounted 44444444-4444-4444-8888-888888888888.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:474`: [   31.949406] systemd[1]: Mounted POSIX Message Queue File System.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:475`: [   31.951730] systemd[1]: Mounted Kernel Debug File System.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:476`: [   31.954796] systemd[1]: Mounted Kernel Trace File System.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:477`: [   31.956732] systemd[1]: Mounted Kernel Configuration File System.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:479`: [   31.966452] systemd[1]: Finished Remount Root and Kernel File Systems.
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:499`: [1;32m<<< Welcome to NixOS 26.05.20260903.a5cc6f2 (armv7l) - ttymxc1 >>>[0m
- `test-logs/p-ext-2026-09-22/p-ext-boot-01.log:503`: novena login: 

Letzte erhaltene Zeilen:

```text
[   32.056590] systemd[1]: Started Journal Service.
[   44.315575] coda 2040000.vpu: firmware request failed
[   44.406602] rtc-pcf8523 0-0068: hctosys: unable to read the hardware clock
[   44.416491] mma8452 0-001c: powering off device failed
[   44.417108] mma8452 0-001c: powering off device failed
[   44.419423] mma8452 0-001c: powering off device failed
[   44.420592] mma8452 0-001c: probe with driver mma8452 failed with error -11
[   44.925331] clk: failed to reparent pll4_audio to osc: -22
[   44.973374] imx-es8328 sound: ASoC: driver name too long 'imx-audio-es8328' -> 'imx-audio-es832'
[   45.877716] it6251 2-005c: error -6 writing to eDP addr 0x5
[   45.880358] it6251 2-005c: error -6 writing to LVDS addr 0x5
[!p]104\[0m[?7h[1G[0J[6n[32766;32766H[6n�[!p]104\[0m[?7h[1G[0J[6n[32766;32766H[6n[62;1H]3008;start=5bb1b5d72346435b9b7222c5329d21f1;user=root;hostname=novena;machineid=ec2c00fb2634468ba302b7bc8d8545dc;bootid=aca482215bbf45b09332bafa5472e714;pid=750;pidfdid=751;comm=(agetty);servicename=serial-getty@ttymxc1.service;invocationid=8f34c67d9c9249b0b0d83211d02bb4da;type=service\P+q6E616D65\

[1;32m<<< Welcome to NixOS 26.05.20260903.a5cc6f2 (armv7l) - ttymxc1 >>>[0m

Run 'nixos-help' for the NixOS manual.

novena login: 
```

### test-logs/p-ext-2026-09-22/p-ext-diag-boot-01.log

```text

U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
Trying to boot from MMC1
NOVENA-DIAG D0 before spl_mmc_find_device dev=0

```

### test-logs/p-ext-2026-09-23/p-ext-d0-d36-boot-01.log

```text

U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
Trying to boot from MMC1
NOVENA-DIAG D0 before spl_mmc_find_device dev=0
NOVENA-DIAG D16 enter mmc_initialize
NOVENA-DIAG D17 after mmc_list_init
NOVENA-DIAG D18 enter board_mmc_init
NOVENA-DIAG D19 before mxc_get_clock
NOVENA-DIAG D20 after mxc_get_clock clk=198000000
NOVENA-DIAG D21 enter fsl_esdhc_initialize
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
NOVENA-DIAG D33 after board_mmc_init ret=-12

```

### test-logs/p-ext-2026-09-23/p-ext-d21a-d21d-boot-01.log

```text
�
U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
Trying to boot from MMC1
NOVENA-DIAG D0 before spl_mmc_find_device dev=0
NOVENA-DIAG D16 enter mmc_initialize
NOVENA-DIAG D17 after mmc_list_init
NOVENA-DIAG D18 enter board_mmc_init
NOVENA-DIAG D19 before mxc_get_clock
NOVENA-DIAG D20 after mxc_get_clock clk=198000000
NOVENA-DIAG D21 enter fsl_esdhc_initialize
NOVENA-DIAG D21A before priv calloc start=18300000 end=18400000 brk=00000000
NOVENA-DIAG D21B after priv calloc priv=? brk=00000000
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
NOVENA-DIAG D33 after board_mmc_init ret=-12

```

### test-logs/p-ext-2026-09-23/p-ext-p0-p1-boot-01.log

```text

U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
NOVENA-DIAG P0 after mem_malloc_init brk=00000000
Trying to boot from MMC1
NOVENA-DIAG P1 before loader callback brk=00000000
NOVENA-DIAG D0 before spl_mmc_find_device dev=0
NOVENA-DIAG D16 enter mmc_initialize
NOVENA-DIAG D17 after mmc_list_init
NOVENA-DIAG D18 enter board_mmc_init
NOVENA-DIAG D19 before mxc_get_clock
NOVENA-DIAG D20 after mxc_get_clock clk=198000000
NOVENA-DIAG D21 enter fsl_esdhc_initialize
NOVENA-DIAG D21A before priv calloc start=18300000 end=18400000 brk=00000000
NOVENA-DIAG D21B after priv calloc priv=? brk=00000000
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
NOVENA-DIAG D33 after board_mmc_init ret=-12

```

### test-logs/p-ext-2026-09-23/p-ext-q1-q2-p0-boot-01.log

```text

U-Boot SPL 2026.07-00003-gf8baca04e22d (Sep 21 2026 - 09:52:43 +0000)
NOVENA-DIAG Q1/Q2 q1=00000000 q2=00000000\nNOVENA-DIAG P0 after mem_malloc_init brk=00000000
Trying to boot from MMC1
NOVENA-DIAG P1 before loader callback brk=00000000
NOVENA-DIAG D0 before spl_mmc_find_device dev=0
NOVENA-DIAG D16 enter mmc_initialize
NOVENA-DIAG D17 after mmc_list_init
NOVENA-DIAG D18 enter board_mmc_init
NOVENA-DIAG D19 before mxc_get_clock
NOVENA-DIAG D20 after mxc_get_clock clk=198000000
NOVENA-DIAG D21 enter fsl_esdhc_initialize
NOVENA-DIAG D21A before priv calloc start=18300000 end=18400000 brk=00000000
NOVENA-DIAG D21B after priv calloc priv=? brk=00000000
NOVENA-DIAG D32 after fsl_esdhc_initialize ret=-12
NOVENA-DIAG D33 after board_mmc_init ret=-12

```

## Buildlogfundstellen

Die Ausgabe wird vollständig gelesen und nur relevante End-/Fehlerzeilen dargestellt. Ein früher Fehlversuch und erfolgreicher späterer Build sind keine widersprüchlichen Ergebnisse desselben Versuchs.

### test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log

- `test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log:2`: error:
- `test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log:11`:          whose name attribute is located at «github:NixOS/nixpkgs/a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4?narHash=sha256-r2f1oUwixlgq9zOdYLqJLfS/lWBT60/IITjhTKI59JU%3D»/pkgs/stdenv/generic/make-derivation.nix:647:11
- `test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log:14`:          at «github:NixOS/nixpkgs/a5cc6f2c37bf518436dc8d1c288ccd0c43c2f4c4?narHash=sha256-r2f1oUwixlgq9zOdYLqJLfS/lWBT60/IITjhTKI59JU%3D»/pkgs/build-support/trivial-builders/default.nix:81:17:
- `test-logs/2026-09-30/block-4.10o-1n-7b-2-full-image-build.log:22`:        error: Path 'boot/u-boot/0004-novena-uboot-diag-d0-d36.patch' in the repository "/home/loomit/nixos-novena-image" is not tracked by Git.

### test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log

- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:4`:   /nix/store/3yp813aa65y6y5039wyr4s53a3whf73k-nixos-novena-sd-image.img.drv
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:7`: building '/nix/store/3yp813aa65y6y5039wyr4s53a3whf73k-nixos-novena-sd-image.img.drv'...
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:10`: nixos-novena-sd-image.img> Disk /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img: 2.4 GiB, 2581291008 bytes, 5041584 sectors
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:20`: nixos-novena-sd-image.img> /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img1: Created a new partition 1 of type 'W95 FAT32 (LBA)' and of size 128 MiB.
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:21`: nixos-novena-sd-image.img> /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img2: Created a new partition 2 of type 'Linux' and of size 2.3 GiB.
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:22`: nixos-novena-sd-image.img> /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img3: Done.
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:29`: nixos-novena-sd-image.img> /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img1       16384  278527  262144 128M  c W95
- `test-logs/2026-09-30/block-4.10o-1n-7b-2r1-full-image-build.log:30`: nixos-novena-sd-image.img> /nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img2 *    278528 5039535 4761008 2.3G 83 Linu

### test-logs/p-ext-2026-09-23/block-9h-build.log


## Evidence-Dateien und Manifestprüfung

| Datei | Byte | SHA256 | Manifeststatus |
|---|---:|---|---|
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/README.md` | 4833 | `248f90b5e3a35cf09afb7f7b7d692b0ed173ddf2a3b66be937e0a055822c4cee` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS` | 1137 | `ea76cabec687549607f422a545cd2f7f4a9354baee469f6367700923abdb9582` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/cmdline.txt` | 244 | `7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/critical-window.txt` | 62119 | `260cec51b67d8fc4ba84b2355ba73f8058886bbe1eb1c490b5f0960e54505193` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/date.txt` | 26 | `9c6831ad809b065a5a82080928d70c2e19513c255c0bd4ee0f2fd74756072e4a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:1 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/display-status.txt` | 97 | `10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:13 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt` | 116990 | `d65e04f2a3ea80bfa3467ccaa56e036967ea8490d2d5c732abac666d299bd657` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/es8328-power.txt` | 268 | `37c281e6896572aae5832487f092af0e1c2504773eceb5648c3b13f161ebe3f1` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/i2c3-devices.txt` | 408 | `e5266d5aac13a3ea35f5e5da8a3b81c4f083b87661518794d5ecc47c6d0c7747` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:10 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/it6251-start.txt` | 49001 | `208de32b8518c0c8a9ccfaf8a4cc7266b58432b50fa29ad64ba3ce1ae10736d5` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/live-dt-audio.txt` | 84 | `dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:12 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/live-dt-files.txt` | 738 | `f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:11 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/regulator-summary.txt` | 4787 | `8366b5e8b476c8a1c9ea27684ddd3548614a860c622fdce25e5aedb875f7fc2c` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt` | 43 | `e8a9d0e5c6f50494b29cbd75d21963d094fe1af1971409a2a9bc140e536d2f21` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:14 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/uname.txt` | 80 | `4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS` | 1137 | `270034bc7598428a17114cf2fb0ff8dbb22f7b5af7d7fca49fff0bfb33d11278` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/cmdline.txt` | 244 | `7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/critical-window.txt` | 60785 | `d5c46eeb166df3c4a611abfbc34b6a46e2ee380e4da6ee03ee742f7d0b51128a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/date.txt` | 26 | `24f56b4bac259a8d11a4ba68b3a3c4ab9826700215bf6ab84d176019744fc9d7` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:1 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/display-status.txt` | 97 | `10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:13 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt` | 115112 | `e0fa393c79cdeb45cd86c45769c5a71f0a4b0a83d42dd24b78894599d63b2e9e` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/es8328-power.txt` | 267 | `c362af2c33c65f96648ec76a90a925f441e7ab0c06d0b1caad541f0905a3b726` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/i2c3-devices.txt` | 408 | `8cb765b59966fb72c39a4494835528f08ae334247db433172eafe22d0f47638d` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:10 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/it6251-start.txt` | 47995 | `0e4ad744c66c80452aef26bff208e813a8daa758d57f7b897c0fbfa84307026d` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/live-dt-audio.txt` | 84 | `dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:12 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/live-dt-files.txt` | 738 | `f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:11 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/regulator-summary.txt` | 4787 | `bd10fa5eb1e0c4d169b8a9bdb9604f71f71677cc8c59536356cca6eb60033482` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt` | 43 | `9dfc3117e6fd2cd96304885f5ee6ddf98e2b9a73daf510c5eae2c2411c77106a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:14 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/uname.txt` | 80 | `4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS` | 1137 | `c688a634d11962eb032136801bda83c90dc635503567b4fe0b2073f649cb11d3` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/cmdline.txt` | 244 | `7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/critical-window.txt` | 60828 | `7e562cf0c1972189b5a5d977e6129a9efe0ece0f795ffcfe637b9e3f9d36d3a6` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/date.txt` | 26 | `7d85571d497a9b3906b1be1d3de98f03b7f04ef862f104da4577117eb5845459` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:1 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/display-status.txt` | 97 | `1e9b2a48132a7900f56e4d2f5e28748b6da3338668c0a4b3b82ee580c8647a9a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:13 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt` | 115900 | `599bf04560888a697d422588a1d06c483c32b3521eefdeb6a1cf37a5b624702e` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/es8328-power.txt` | 331 | `cb50f0f903a9cdec6acaa7ae21bb14a22ba72798df51826b6ea833c7828beff3` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/i2c3-devices.txt` | 408 | `7e4b39d2669006a4585837ca3e383368c9b1946514bae8771d63c0554705897b` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:10 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/it6251-start.txt` | 47981 | `19aec81e7c37e9a9d0f4e9971ac79e4f43864e9bf9ec8eac5dea8cdb1f180b3e` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/live-dt-audio.txt` | 84 | `dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:12 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/live-dt-files.txt` | 738 | `f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:11 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/regulator-summary.txt` | 4787 | `8366b5e8b476c8a1c9ea27684ddd3548614a860c622fdce25e5aedb875f7fc2c` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt` | 43 | `2f9d6eda6cd481255b3a84800b2f5de14bf866f62f32d336479b19b6dfd6fc53` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:14 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/uname.txt` | 80 | `4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS` | 1137 | `f9c028a41b88e3cb124eef7d86e38e8e9e9f44c8b45c8bd5bd126be3e2839036` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/cmdline.txt` | 244 | `7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/critical-window.txt` | 59374 | `a21092c8c6dba4cba4a12e4f145938b1a521f1e89017ce43a4129323166ae01b` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/date.txt` | 26 | `cb8808aa77a80506cb11de4a9fdbb2eb7cb6758d94ec21d0666b275966307dc0` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:1 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/display-status.txt` | 97 | `10a314757692c01b6a33ba08f3ebf007b29cb341d0c32672d7262e857d8dd27a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:13 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt` | 113985 | `d959a7193f097e58a1b5c70bfabe87ab6fbdddb54c2a97cb55d284cbaf505844` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/es8328-power.txt` | 203 | `ff0254655ab5ceba4e6dc9ba88303a0ec5b62186efa6756e19c7f7dbf9ba1121` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/i2c3-devices.txt` | 408 | `3e46a264204aa89e19c867fd139d35b618d1089419fc231257d0b8c19ffe43cd` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:10 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/it6251-start.txt` | 46979 | `334537e0e8b31e00f42f468cd2cef3697aa893ed697562c441f8bb14da2af72e` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/live-dt-audio.txt` | 84 | `dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:12 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/live-dt-files.txt` | 738 | `f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:11 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/regulator-summary.txt` | 4787 | `bd10fa5eb1e0c4d169b8a9bdb9604f71f71677cc8c59536356cca6eb60033482` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt` | 43 | `5ab8295b3d65b4531bc838cc1d0a19a98484a8788832775201a32ceb67216ce5` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:14 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/uname.txt` | 80 | `4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS` | 1137 | `bca629cefbc48118e5d6a5e9093c56c69419d8dbfc278883a6476ada2dff506c` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/cmdline.txt` | 244 | `7fa692c5348822e1fb0c49b101c17c8ddc051ed636e328222c024dd6a3798063` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/critical-window.txt` | 51366 | `06e222c99235eb884a9de95f2b8e842d154955898968b37d818017ffed56578b` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/date.txt` | 26 | `3427dda5b9f596088ff56edb4d710fed59408c43c4950d7ff29de6ac0c8ae631` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:1 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/display-status.txt` | 97 | `1e9b2a48132a7900f56e4d2f5e28748b6da3338668c0a4b3b82ee580c8647a9a` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:13 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt` | 103582 | `19b9f51046f47be276d7d8545fe695cb8ab576d0942a36f181b1932d38909471` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/es8328-power.txt` | 329 | `d63a1f28e616bf5ec788e03f1b1dc26abfecb6600fbab699b579abbf33e28ede` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/i2c3-devices.txt` | 408 | `a61a598463a3c01f836a4b420e7ff6a6839be4d1b771e4f1b5c562eadab3c0d3` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:10 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/it6251-start.txt` | 40853 | `a54da85f2a96da13b5dab370208ac8ae07b5fd533573e72d5a9537796532428b` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/live-dt-audio.txt` | 84 | `dc1231cadba58bdc61dabd5a4e7d871a00e64ac9cd5633a05f8a93a045c29f79` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:12 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/live-dt-files.txt` | 738 | `f05c57dfa5dddf4bca39ac63267bbd5869e4bad6c8b1ae49f0c76aaac3136d88` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:11 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/regulator-summary.txt` | 4787 | `a73d16d88521c68e6f393af5613f6a6b64616034159691321b8b247f368c711f` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt` | 43 | `bb99d51e7ab21d01e5e8572715662b49f5ab6a50c6e427b677e88c11fd45f77d` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:14 BELEGT: Hash gleich |
| `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/uname.txt` | 80 | `4f5bca55efe3543489248f36caff0be11acbef13b8f18b1c7dfcc50341dfe1b9` | evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS` | 752 | `32882c11c7a00bb312912523f965a8bef67016dc82f0ea1a3bdbc51c3719a525` | nicht im Manifest (u. a. Manifest selbst/README) |
| `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt` | 94042 | `c20c98e924d368111da97b18930b5c86c03fde42ffb9d20cdee2d5459bc88da4` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:2 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/dmesg-it6251-i2c3.txt` | 53417 | `d325a8bcc1b34c709f66b1b86795f72b8fbb26b335dc2bc57759b5f50b4b3ee6` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:3 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/gpio.txt` | 1262 | `0e1d41ebb9f12f17f1012ad8eea5ebbc406bd0dc935218d996c03cc17ef6d636` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:5 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/i2c3-devices.txt` | 408 | `f56851d185a54902f549b8be6d0d1eb9fc973609f7e171a3c27286286de8eede` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:8 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/ldb-binding.txt` | 309 | `09fc9f5328cea83c7b0580e89d6118a8d23033fd18513f6d321f9bc2554259c5` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:9 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/live-dt-audio-files.txt` | 666 | `866b6473300f86ab00070624718d15ceb66cab4b55e72aa9208074c28d448008` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:6 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/live-dt-audio-hex.txt` | 1002 | `6684f147d031928eeaa6c45a6357979a1bdd2c3fc29e1f7d8b1efe8eea3e1577` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:7 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/regulator-summary.txt` | 4787 | `17b8d66aa0dcde3562cf06b910c02334f1b3e95aac52209647ef1090e2ecf486` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:4 BELEGT: Hash gleich |
| `evidence/block-3.5-h3-1-2026-09-15/system.txt` | 350 | `aa8bb21310ebc9ce594132f1b12cb3df5f535685a57b73c9b62aab94facf5188` | evidence/block-3.5-h3-1-2026-09-15/SHA256SUMS:1 BELEGT: Hash gleich |

## Chronologische Kernel-/Trial-Signaturen

Zeitwerte hier sind Rohlog-Zeitwerte, keine neue Uhrmessung. Postboot-Dateien belegen ihren Snapshot, keinen analogen Anfangszustand. Nichtgefundene Zeichenfolgen sind nur textuelle Negativbefunde.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/cmdline.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/cmdline.txt:1`: systemConfig=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/date.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/date.txt:1`: 2026-09-15T15:02:03+00:00

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:249`: [    2.903099] usbcore: registered new interface driver net1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:306`: [    3.361463] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:360`: [    5.192023] systemd[1]: Starting Create Static Device Nodes in /dev...
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:486`: [   42.464991] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:487`: [   42.465059] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:495`: [   44.471653] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:496`: [   44.471767] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:499`: [   44.472493] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:502`: [   44.472849] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:507`: [   44.474070] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:510`: [   44.474296] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:515`: [   44.475402] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:518`: [   44.475625] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:523`: [   44.476736] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:526`: [   44.476959] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:530`: [   44.477689] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:539`: [   44.491643] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:542`: [   44.491856] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:545`: [   44.492061] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:549`: [   44.492628] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:552`: [   44.492832] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:555`: [   44.493039] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:559`: [   44.493563] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:562`: [   44.493769] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:565`: [   44.493974] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:568`: [   44.494457] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:571`: [   44.494662] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:574`: [   44.494865] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:580`: [   44.503038] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:583`: [   44.503255] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:586`: [   44.503548] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:589`: [   44.504241] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:592`: [   44.504536] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:595`: [   44.504830] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:598`: [   44.505499] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:601`: [   44.505792] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:604`: [   44.506087] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:607`: [   44.506763] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:610`: [   44.507057] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:613`: [   44.507350] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:616`: [   44.508012] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:619`: [   44.508304] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:622`: [   44.508597] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:625`: [   44.509283] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:628`: [   44.509576] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:631`: [   44.509868] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:634`: [   44.510527] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:637`: [   44.510822] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:640`: [   44.511116] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:643`: [   44.511849] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:646`: [   44.512170] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:649`: [   44.512519] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:652`: [   44.513183] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:655`: [   44.513412] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:658`: [   44.513636] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:661`: [   44.514186] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:664`: [   44.514409] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:667`: [   44.514630] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:670`: [   44.515176] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:673`: [   44.515399] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:676`: [   44.515620] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:679`: [   44.516158] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:682`: [   44.516381] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:685`: [   44.516602] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:688`: [   44.517148] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:691`: [   44.517370] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:694`: [   44.517595] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:697`: [   44.518132] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:700`: [   44.518355] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:703`: [   44.518577] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:706`: [   44.519117] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:709`: [   44.519340] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:712`: [   44.519562] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:715`: [   44.520101] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:718`: [   44.520324] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:721`: [   44.520547] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:724`: [   44.521064] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:727`: [   44.521240] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:730`: [   44.521416] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:733`: [   44.521869] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:736`: [   44.522047] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:739`: [   44.522220] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:742`: [   44.522652] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:745`: [   44.522827] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:748`: [   44.523000] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:751`: [   44.523424] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:754`: [   44.523600] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:757`: [   44.523773] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:760`: [   44.524204] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:763`: [   44.524378] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:766`: [   44.524553] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:769`: [   44.524976] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:772`: [   44.525150] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:775`: [   44.525324] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:778`: [   44.525750] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:781`: [   44.525925] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:784`: [   44.526098] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:787`: [   44.526531] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:790`: [   44.526705] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:793`: [   44.526879] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:798`: [   44.527332] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:801`: [   44.527509] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:804`: [   44.527683] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:807`: [   44.528104] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:810`: [   44.528277] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:815`: [   44.531200] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:818`: [   44.531375] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:823`: [   44.534341] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:826`: [   44.534518] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:831`: [   44.535435] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:834`: [   44.535608] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:839`: [   44.538529] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:842`: [   44.538704] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:847`: [   44.539616] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:850`: [   44.539789] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:855`: [   44.542790] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:858`: [   44.543000] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:863`: [   44.544055] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:866`: [   44.544261] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:871`: [   44.547307] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:874`: [   44.547513] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:879`: [   44.548548] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:882`: [   44.548752] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:887`: [   44.551837] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:890`: [   44.551997] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:895`: [   44.552870] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:898`: [   44.553028] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:903`: [   44.555915] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:906`: [   44.556074] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:911`: [   44.556941] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:914`: [   44.557099] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:919`: [   44.559974] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:922`: [   44.560133] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:927`: [   44.561000] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:930`: [   44.561161] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:935`: [   44.564091] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:938`: [   44.564256] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:943`: [   44.565147] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:946`: [   44.565309] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:951`: [   44.568193] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:954`: [   44.568356] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:959`: [   44.569239] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:962`: [   44.569401] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:967`: [   44.572390] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:970`: [   44.572600] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:975`: [   44.573666] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:978`: [   44.573872] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:983`: [   44.576910] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:986`: [   44.577115] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:991`: [   44.578160] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:994`: [   44.578364] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:999`: [   44.581437] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1002`: [   44.581618] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1007`: [   44.582471] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1010`: [   44.582623] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1015`: [   44.585473] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1018`: [   44.585625] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1023`: [   44.586472] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1026`: [   44.586624] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1031`: [   44.589471] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1034`: [   44.589622] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1039`: [   44.590460] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1042`: [   44.590612] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1047`: [   44.593471] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1050`: [   44.593629] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1055`: [   44.594496] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1058`: [   44.594651] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1063`: [   44.597505] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1066`: [   44.597660] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1071`: [   44.598510] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1074`: [   44.598667] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1079`: [   44.601521] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1082`: [   44.601678] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1087`: [   44.602547] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1090`: [   44.602702] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1095`: [   44.605565] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1098`: [   44.605719] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1103`: [   44.606572] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1106`: [   44.606727] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1111`: [   44.609577] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1114`: [   44.609733] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1119`: [   44.610576] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1122`: [   44.610731] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1127`: [   44.613622] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1130`: [   44.613778] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1135`: [   44.614642] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1138`: [   44.614798] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1145`: [   44.615658] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1148`: [   44.615813] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1154`: [   44.616672] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1157`: [   44.616825] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1162`: [   44.617669] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1165`: [   44.617824] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1182`: [   44.622394] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1185`: [   44.622550] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1190`: [   44.623400] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1193`: [   44.623556] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1196`: [   44.624075] it6251 2-005c: hactive: 1920
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1199`: [   44.624413] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1202`: [   44.624568] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1207`: [   44.625417] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1210`: [   44.625573] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1213`: [   44.626093] it6251 2-005c: vactive: 1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/dmesg-full.txt:1214`: [   44.626100] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=0; es8328-power: disabling=0. Diese Zählung ist keine neue Teststatistik.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt:1`: H3-1R_BOOT=01
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt:2`: POR_COLD_BOOT=YES
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-01/trial.txt:3`: DISPLAY=ON

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/cmdline.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/cmdline.txt:1`: systemConfig=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/date.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/date.txt:1`: 2026-09-15T15:04:39+00:00

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:247`: [    2.901080] usbcore: registered new interface driver smsc75xx
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:249`: [    2.901328] usbcore: registered new interface driver net1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:306`: [    3.356096] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:490`: [   43.086833] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:491`: [   43.086849] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:523`: [   45.126639] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:524`: [   45.126729] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:527`: [   45.127542] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:530`: [   45.127766] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:535`: [   45.129239] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:538`: [   45.129556] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:543`: [   45.130972] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:546`: [   45.131292] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:551`: [   45.132716] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:554`: [   45.133035] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:558`: [   45.133968] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:567`: [   45.137050] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:570`: [   45.137338] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:573`: [   45.137579] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:577`: [   45.138177] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:580`: [   45.138414] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:583`: [   45.138650] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:587`: [   45.139235] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:590`: [   45.139472] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:593`: [   45.139709] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:596`: [   45.140262] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:599`: [   45.140502] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:602`: [   45.140739] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:608`: [   45.148992] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:611`: [   45.149175] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:614`: [   45.149354] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:617`: [   45.149781] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:620`: [   45.149959] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:623`: [   45.150139] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:626`: [   45.150562] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:629`: [   45.150741] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:632`: [   45.150920] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:635`: [   45.151350] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:638`: [   45.151529] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:641`: [   45.151707] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:644`: [   45.152134] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:647`: [   45.152316] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:650`: [   45.152495] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:653`: [   45.152918] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:656`: [   45.153097] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:659`: [   45.153275] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:662`: [   45.153695] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:665`: [   45.153874] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:668`: [   45.154054] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:671`: [   45.154480] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:674`: [   45.154659] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:677`: [   45.154837] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:680`: [   45.155261] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:683`: [   45.155441] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:686`: [   45.155620] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:689`: [   45.156038] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:692`: [   45.156220] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:695`: [   45.156398] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:698`: [   45.156839] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:701`: [   45.157017] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:704`: [   45.157195] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:707`: [   45.157632] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:710`: [   45.157814] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:713`: [   45.157994] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:716`: [   45.158431] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:719`: [   45.158609] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:722`: [   45.158788] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:725`: [   45.159216] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:728`: [   45.159396] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:731`: [   45.159574] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:734`: [   45.160013] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:737`: [   45.160191] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:740`: [   45.160371] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:743`: [   45.160814] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:746`: [   45.160995] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:749`: [   45.161172] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:752`: [   45.161611] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:755`: [   45.161791] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:758`: [   45.161971] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:761`: [   45.162402] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:764`: [   45.162582] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:767`: [   45.162761] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:770`: [   45.163199] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:773`: [   45.163377] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:776`: [   45.163555] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:779`: [   45.163991] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:782`: [   45.164170] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:785`: [   45.164348] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:788`: [   45.164780] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:791`: [   45.164957] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:794`: [   45.165141] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:797`: [   45.165579] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:800`: [   45.165758] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:803`: [   45.165937] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:806`: [   45.166376] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:809`: [   45.166556] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:812`: [   45.166734] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:815`: [   45.167164] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:818`: [   45.167343] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:821`: [   45.167524] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:826`: [   45.167982] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:829`: [   45.168161] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:832`: [   45.168338] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:835`: [   45.168767] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:838`: [   45.168946] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:843`: [   45.171879] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:846`: [   45.172061] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:851`: [   45.175013] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:854`: [   45.175194] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:859`: [   45.176124] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:862`: [   45.176303] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:867`: [   45.179260] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:870`: [   45.179441] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:875`: [   45.180366] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:878`: [   45.180545] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:883`: [   45.183475] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:886`: [   45.183656] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:891`: [   45.184583] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:894`: [   45.184763] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:899`: [   45.187709] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:902`: [   45.187932] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:907`: [   45.189018] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:910`: [   45.189236] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:915`: [   45.192325] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:918`: [   45.192546] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:923`: [   45.193631] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:926`: [   45.193853] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:931`: [   45.196903] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:934`: [   45.197072] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:939`: [   45.197966] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:942`: [   45.198133] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:947`: [   45.201028] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:950`: [   45.201195] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:955`: [   45.202078] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:958`: [   45.202244] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:963`: [   45.205136] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:966`: [   45.205304] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:971`: [   45.206300] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:974`: [   45.206520] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:979`: [   45.209616] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:982`: [   45.209836] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:987`: [   45.210917] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:990`: [   45.211138] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:995`: [   45.214208] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:998`: [   45.214429] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1003`: [   45.215491] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1006`: [   45.215711] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1011`: [   45.218755] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1014`: [   45.218914] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1019`: [   45.219775] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1022`: [   45.219933] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1027`: [   45.222799] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1030`: [   45.222957] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1035`: [   45.223815] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1038`: [   45.223975] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1043`: [   45.226847] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1046`: [   45.227007] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1051`: [   45.227877] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1054`: [   45.228035] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1059`: [   45.230896] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1062`: [   45.231054] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1067`: [   45.231914] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1070`: [   45.232072] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1075`: [   45.234938] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1078`: [   45.235097] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1083`: [   45.235958] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1086`: [   45.236120] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1091`: [   45.238990] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1094`: [   45.239151] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1099`: [   45.240017] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1102`: [   45.240174] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1107`: [   45.243036] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1110`: [   45.243195] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1115`: [   45.244054] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1118`: [   45.244213] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1123`: [   45.247087] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1126`: [   45.247246] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1131`: [   45.248117] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1134`: [   45.248275] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1139`: [   45.251140] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1142`: [   45.251299] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1147`: [   45.252157] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1150`: [   45.252318] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1157`: [   45.253185] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1160`: [   45.253343] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1166`: [   45.254209] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1169`: [   45.254370] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1174`: [   45.255232] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1177`: [   45.255391] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1194`: [   45.259971] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1197`: [   45.260130] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1202`: [   45.260981] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1205`: [   45.261141] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1208`: [   45.261666] it6251 2-005c: hactive: 1920
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1211`: [   45.262006] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1214`: [   45.262165] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1219`: [   45.263022] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1222`: [   45.263181] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1225`: [   45.263701] it6251 2-005c: vactive: 1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/dmesg-full.txt:1226`: [   45.263708] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=0; es8328-power: disabling=0. Diese Zählung ist keine neue Teststatistik.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt:1`: H3-1R_BOOT=02
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt:2`: POR_COLD_BOOT=YES
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-02/trial.txt:3`: DISPLAY=ON

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/cmdline.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/cmdline.txt:1`: systemConfig=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/date.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/date.txt:1`: 2026-09-15T15:07:45+00:00

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:249`: [    2.904552] usbcore: registered new interface driver net1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:306`: [    3.361357] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:361`: [    5.192007] systemd[1]: Starting Create Static Device Nodes in /dev...
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:482`: [   42.334995] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:483`: [   42.335053] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:495`: [   44.392045] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:496`: [   44.392131] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:499`: [   44.392903] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:502`: [   44.393127] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:507`: [   44.394163] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:510`: [   44.394342] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:515`: [   44.395278] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:518`: [   44.395459] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:523`: [   44.396409] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:526`: [   44.396589] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:530`: [   44.397181] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:539`: [   44.401946] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:542`: [   44.402127] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:545`: [   44.402307] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:549`: [   44.402783] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:552`: [   44.402962] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:555`: [   44.403142] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:559`: [   44.403586] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:562`: [   44.403766] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:565`: [   44.403945] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:568`: [   44.404378] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:571`: [   44.404557] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:574`: [   44.404734] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:580`: [   44.412402] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:583`: [   44.412582] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:586`: [   44.412761] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:589`: [   44.413193] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:592`: [   44.413378] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:595`: [   44.413561] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:598`: [   44.413995] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:601`: [   44.414174] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:604`: [   44.414355] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:607`: [   44.414777] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:610`: [   44.414956] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:613`: [   44.415134] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:616`: [   44.415564] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:619`: [   44.415742] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:622`: [   44.415919] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:625`: [   44.416349] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:628`: [   44.416531] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:631`: [   44.416708] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:634`: [   44.417135] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:637`: [   44.417314] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:640`: [   44.417493] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:643`: [   44.417923] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:646`: [   44.418103] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:649`: [   44.418281] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:652`: [   44.418708] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:655`: [   44.418887] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:658`: [   44.419064] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:661`: [   44.419492] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:664`: [   44.419675] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:667`: [   44.419853] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:670`: [   44.420275] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:673`: [   44.420456] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:676`: [   44.420634] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:679`: [   44.421064] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:682`: [   44.421243] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:685`: [   44.421425] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:688`: [   44.421864] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:691`: [   44.422042] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:694`: [   44.422219] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:697`: [   44.422661] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:700`: [   44.422841] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:703`: [   44.423021] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:706`: [   44.423547] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:709`: [   44.423767] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:712`: [   44.423987] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:715`: [   44.424510] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:718`: [   44.424728] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:721`: [   44.424945] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:724`: [   44.425457] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:727`: [   44.425676] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:730`: [   44.425894] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:733`: [   44.426413] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:736`: [   44.426630] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:739`: [   44.426847] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:742`: [   44.427367] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:745`: [   44.427585] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:748`: [   44.427803] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:751`: [   44.428328] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:754`: [   44.428548] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:757`: [   44.428765] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:760`: [   44.429283] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:763`: [   44.429502] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:766`: [   44.429721] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:769`: [   44.430236] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:772`: [   44.430460] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:775`: [   44.430680] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:778`: [   44.431204] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:781`: [   44.431436] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:784`: [   44.431657] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:787`: [   44.432146] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:790`: [   44.432312] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:793`: [   44.432477] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:798`: [   44.432911] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:801`: [   44.433078] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:804`: [   44.433244] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:807`: [   44.433649] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:810`: [   44.433817] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:815`: [   44.436718] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:818`: [   44.436887] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:823`: [   44.439779] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:826`: [   44.439946] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:831`: [   44.440830] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:834`: [   44.440998] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:839`: [   44.444023] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:842`: [   44.444245] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:847`: [   44.445334] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:850`: [   44.445554] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:855`: [   44.448642] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:858`: [   44.448863] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:863`: [   44.449942] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:866`: [   44.450162] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:871`: [   44.453261] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:874`: [   44.453484] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:879`: [   44.454428] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:882`: [   44.454586] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:887`: [   44.457450] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:890`: [   44.457609] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:895`: [   44.458469] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:898`: [   44.458628] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:903`: [   44.461513] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:906`: [   44.461673] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:911`: [   44.462533] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:914`: [   44.462694] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:919`: [   44.465565] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:922`: [   44.465724] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:927`: [   44.466579] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:930`: [   44.466739] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:935`: [   44.469595] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:938`: [   44.469754] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:943`: [   44.470606] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:946`: [   44.470765] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:951`: [   44.473632] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:954`: [   44.473794] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:959`: [   44.474667] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:962`: [   44.474825] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:967`: [   44.477710] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:970`: [   44.477870] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:975`: [   44.478728] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:978`: [   44.478890] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:983`: [   44.481774] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:986`: [   44.481937] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:991`: [   44.482818] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:994`: [   44.482980] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:999`: [   44.485845] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1002`: [   44.486005] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1007`: [   44.486861] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1010`: [   44.487022] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1015`: [   44.489888] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1018`: [   44.490047] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1023`: [   44.490905] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1026`: [   44.491065] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1031`: [   44.493959] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1034`: [   44.494117] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1039`: [   44.494973] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1042`: [   44.495132] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1047`: [   44.497991] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1050`: [   44.498152] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1055`: [   44.499004] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1058`: [   44.499163] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1063`: [   44.502034] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1066`: [   44.502195] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1071`: [   44.503067] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1074`: [   44.503225] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1079`: [   44.506088] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1082`: [   44.506247] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1087`: [   44.507104] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1090`: [   44.507262] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1095`: [   44.510116] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1098`: [   44.510275] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1103`: [   44.511132] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1106`: [   44.511291] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1111`: [   44.514176] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1114`: [   44.514336] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1119`: [   44.515195] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1122`: [   44.515354] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1129`: [   44.516215] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1132`: [   44.516374] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1138`: [   44.517240] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1141`: [   44.517399] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1146`: [   44.518254] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1149`: [   44.518414] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1166`: [   44.523005] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1169`: [   44.523164] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1174`: [   44.524017] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1177`: [   44.524176] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1180`: [   44.524697] it6251 2-005c: hactive: 1920
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1183`: [   44.525040] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1186`: [   44.525199] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1191`: [   44.526045] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1194`: [   44.526203] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1197`: [   44.526727] it6251 2-005c: vactive: 1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/dmesg-full.txt:1198`: [   44.526733] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=0; es8328-power: disabling=0. Diese Zählung ist keine neue Teststatistik.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt:1`: H3-1R_BOOT=03
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt:2`: POR_COLD_BOOT=YES
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-03/trial.txt:3`: DISPLAY=ON

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/cmdline.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/cmdline.txt:1`: systemConfig=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/date.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/date.txt:1`: 2026-09-15T15:10:42+00:00

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:249`: [    2.907807] usbcore: registered new interface driver net1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:292`: [    3.111920] mdio_bus 2188000.ethernet-1: scan phy ethernet-phy at address 0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:306`: [    3.362021] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:491`: [   42.533026] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:492`: [   42.533049] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:531`: [   44.552059] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:532`: [   44.552104] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:535`: [   44.552553] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:538`: [   44.552762] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:543`: [   44.553843] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:546`: [   44.554057] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:551`: [   44.555124] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:554`: [   44.555340] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:559`: [   44.556417] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:562`: [   44.556634] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:566`: [   44.557314] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:575`: [   44.571349] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:578`: [   44.571567] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:581`: [   44.571778] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:585`: [   44.572295] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:588`: [   44.572513] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:591`: [   44.572727] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:595`: [   44.573249] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:598`: [   44.573463] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:601`: [   44.573676] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:604`: [   44.574191] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:607`: [   44.574406] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:610`: [   44.574620] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:616`: [   44.582829] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:619`: [   44.583053] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:622`: [   44.583245] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:625`: [   44.583651] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:628`: [   44.583815] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:631`: [   44.583978] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:634`: [   44.584383] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:637`: [   44.584546] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:640`: [   44.584712] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:643`: [   44.585118] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:646`: [   44.585284] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:649`: [   44.585446] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:652`: [   44.585852] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:655`: [   44.586020] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:658`: [   44.586186] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:661`: [   44.586583] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:664`: [   44.586747] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:667`: [   44.586911] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:670`: [   44.587318] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:673`: [   44.587481] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:676`: [   44.587642] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:679`: [   44.588041] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:682`: [   44.588205] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:685`: [   44.588367] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:688`: [   44.588774] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:691`: [   44.588937] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:694`: [   44.589099] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:697`: [   44.589497] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:700`: [   44.589660] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:703`: [   44.589822] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:706`: [   44.590222] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:709`: [   44.590387] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:712`: [   44.590549] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:715`: [   44.590945] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:718`: [   44.591110] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:721`: [   44.591307] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:724`: [   44.591818] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:727`: [   44.592072] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:730`: [   44.592293] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:733`: [   44.592805] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:736`: [   44.593020] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:739`: [   44.593233] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:742`: [   44.593751] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:745`: [   44.593965] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:748`: [   44.594180] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:751`: [   44.594726] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:754`: [   44.594941] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:757`: [   44.595155] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:760`: [   44.595653] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:763`: [   44.595868] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:766`: [   44.596080] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:769`: [   44.596587] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:772`: [   44.596801] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:775`: [   44.597014] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:778`: [   44.597516] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:781`: [   44.597729] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:784`: [   44.597940] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:787`: [   44.598445] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:790`: [   44.598658] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:793`: [   44.598870] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:796`: [   44.599360] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:799`: [   44.599574] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:802`: [   44.599788] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:805`: [   44.600288] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:808`: [   44.600504] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:811`: [   44.600718] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:814`: [   44.601227] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:817`: [   44.601428] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:820`: [   44.601584] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:823`: [   44.601974] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:826`: [   44.602138] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:829`: [   44.602295] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:834`: [   44.602703] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:837`: [   44.602860] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:840`: [   44.603018] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:843`: [   44.603412] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:846`: [   44.603570] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:851`: [   44.606591] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:854`: [   44.606747] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:859`: [   44.609613] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:862`: [   44.609771] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:867`: [   44.610622] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:870`: [   44.610778] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:875`: [   44.613658] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:878`: [   44.613816] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:883`: [   44.614674] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:886`: [   44.614830] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:891`: [   44.617696] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:894`: [   44.617852] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:899`: [   44.618706] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:902`: [   44.618861] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:907`: [   44.621712] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:910`: [   44.621871] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:915`: [   44.622730] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:918`: [   44.622886] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:923`: [   44.625748] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:926`: [   44.625903] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:931`: [   44.626757] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:934`: [   44.626913] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:939`: [   44.629778] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:942`: [   44.629934] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:947`: [   44.630786] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:950`: [   44.630942] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:955`: [   44.633801] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:958`: [   44.633957] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:963`: [   44.634810] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:966`: [   44.634966] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:971`: [   44.637838] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:974`: [   44.637993] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:979`: [   44.638842] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:982`: [   44.638997] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:987`: [   44.641858] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:990`: [   44.642032] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:995`: [   44.642899] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:998`: [   44.643055] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1003`: [   44.645924] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1006`: [   44.646083] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1011`: [   44.646946] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1014`: [   44.647103] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1019`: [   44.649960] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1022`: [   44.650116] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1027`: [   44.650969] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1030`: [   44.651125] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1035`: [   44.653981] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1038`: [   44.654139] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1043`: [   44.654990] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1046`: [   44.655146] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1051`: [   44.658008] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1054`: [   44.658164] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1059`: [   44.659008] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1062`: [   44.659163] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1067`: [   44.662020] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1070`: [   44.662181] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1075`: [   44.663041] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1078`: [   44.663196] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1083`: [   44.666059] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1086`: [   44.666214] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1091`: [   44.667062] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1094`: [   44.667218] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1099`: [   44.670071] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1102`: [   44.670228] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1107`: [   44.671075] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1110`: [   44.671230] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1115`: [   44.674127] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1118`: [   44.674288] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1123`: [   44.675167] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1126`: [   44.675323] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1131`: [   44.678198] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1134`: [   44.678353] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1139`: [   44.679198] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1142`: [   44.679355] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1149`: [   44.680234] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1152`: [   44.680390] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1158`: [   44.681246] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1161`: [   44.681404] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1166`: [   44.682267] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1169`: [   44.682425] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1186`: [   44.687009] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1189`: [   44.687167] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1194`: [   44.688014] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1197`: [   44.688170] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1200`: [   44.688687] it6251 2-005c: hactive: 1920
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1203`: [   44.689026] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1206`: [   44.689181] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1211`: [   44.690026] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1214`: [   44.690182] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1217`: [   44.690698] it6251 2-005c: vactive: 1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/dmesg-full.txt:1218`: [   44.690706] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=0; es8328-power: disabling=0. Diese Zählung ist keine neue Teststatistik.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt:1`: H3-1R_BOOT=04
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt:2`: POR_COLD_BOOT=YES
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-04/trial.txt:3`: DISPLAY=ON

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/cmdline.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/cmdline.txt:1`: systemConfig=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2 init=/nix/store/ifl71s7rzhk16746d8p5r85pjyc1k5gj-nixos-system-novena-26.05.20260903.a5cc6f2/init loglevel=7 console=ttymxc1,115200 console=tty0

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/date.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/date.txt:1`: 2026-09-15T15:13:37+00:00

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:249`: [    2.899312] usbcore: registered new interface driver net1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:306`: [    3.357981] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:446`: [   33.019204] missing module BTF, cannot register kfunc
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:481`: [   42.492074] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:482`: [   42.492091] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:518`: [   44.573645] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:519`: [   44.573739] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:522`: [   44.576313] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:525`: [   44.576485] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:530`: [   44.577658] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:533`: [   44.577829] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:538`: [   44.579188] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:541`: [   44.579363] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:546`: [   44.580760] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:549`: [   44.580936] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:553`: [   44.581635] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:562`: [   44.582155] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:565`: [   44.582325] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:568`: [   44.582497] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:572`: [   44.583011] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:575`: [   44.583182] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:578`: [   44.583360] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:582`: [   44.583842] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:585`: [   44.584009] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:588`: [   44.584177] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:591`: [   44.584698] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:594`: [   44.584891] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:597`: [   44.585087] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:603`: [   44.593801] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:606`: [   44.593970] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:609`: [   44.594160] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:612`: [   44.594782] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:615`: [   44.594947] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:618`: [   44.595114] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:621`: [   44.597843] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:624`: [   44.598010] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:627`: [   44.598181] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:630`: [   44.598776] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:633`: [   44.598950] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:636`: [   44.599139] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:639`: [   44.599602] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:642`: [   44.599765] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:645`: [   44.599926] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:648`: [   44.600379] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:651`: [   44.600543] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:654`: [   44.600707] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:657`: [   44.601168] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:660`: [   44.601334] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:663`: [   44.601503] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:666`: [   44.601996] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:669`: [   44.602196] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:672`: [   44.602392] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:675`: [   44.602922] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:678`: [   44.603130] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:681`: [   44.603325] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:684`: [   44.603825] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:687`: [   44.604005] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:690`: [   44.604179] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:693`: [   44.604705] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=a1/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:696`: [   44.604879] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=a1/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:699`: [   44.605047] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=a1/a0 M4=a1/a0 M5=a1/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:702`: [   44.605519] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:705`: [   44.605688] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:708`: [   44.605852] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:711`: [   44.606340] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:714`: [   44.606505] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:717`: [   44.606670] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:720`: [   44.607157] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:723`: [   44.607321] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:726`: [   44.607483] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:729`: [   44.607920] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:732`: [   44.608099] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:735`: [   44.608266] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:738`: [   44.608725] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:741`: [   44.608886] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:744`: [   44.609045] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:747`: [   44.616740] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:750`: [   44.616906] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:753`: [   44.617069] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:756`: [   44.617497] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:759`: [   44.617658] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:762`: [   44.617821] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:765`: [   44.618360] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:768`: [   44.618520] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:771`: [   44.618677] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:774`: [   44.619101] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:777`: [   44.619257] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:780`: [   44.619414] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:783`: [   44.619804] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:786`: [   44.619960] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:789`: [   44.620115] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:792`: [   44.620513] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:795`: [   44.620668] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:798`: [   44.620823] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:801`: [   44.621210] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:804`: [   44.621365] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:807`: [   44.621524] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:810`: [   44.621918] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:813`: [   44.622074] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:816`: [   44.622228] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:821`: [   44.622631] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:824`: [   44.622787] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:827`: [   44.622942] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:830`: [   44.623332] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:833`: [   44.623491] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:838`: [   44.626349] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:841`: [   44.626505] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:846`: [   44.629363] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:849`: [   44.629520] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:854`: [   44.630382] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:857`: [   44.630537] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:862`: [   44.633558] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:865`: [   44.633715] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:870`: [   44.634563] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:873`: [   44.634720] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:878`: [   44.637572] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:881`: [   44.637727] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:886`: [   44.638644] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:889`: [   44.638800] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:894`: [   44.641664] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:897`: [   44.641820] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:902`: [   44.642675] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:905`: [   44.642831] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:910`: [   44.645685] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:913`: [   44.645841] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:918`: [   44.646690] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:921`: [   44.646847] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:926`: [   44.649773] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:929`: [   44.649933] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:934`: [   44.650812] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:937`: [   44.650967] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:942`: [   44.653833] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:945`: [   44.653988] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:950`: [   44.654845] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:953`: [   44.655001] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:958`: [   44.657891] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:961`: [   44.658062] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:966`: [   44.658942] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:969`: [   44.659098] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:974`: [   44.661958] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:977`: [   44.662115] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:982`: [   44.662971] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:985`: [   44.663127] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:990`: [   44.665984] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:993`: [   44.666138] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:998`: [   44.666985] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1001`: [   44.667139] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1006`: [   44.670011] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1009`: [   44.670170] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1014`: [   44.671037] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1017`: [   44.671192] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1022`: [   44.674056] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1025`: [   44.674211] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1030`: [   44.675065] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1033`: [   44.675219] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1040`: [   44.676082] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1043`: [   44.676236] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1049`: [   44.677092] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1052`: [   44.677247] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1057`: [   44.678208] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1060`: [   44.678365] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1077`: [   44.682966] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1080`: [   44.683120] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1085`: [   44.683972] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1088`: [   44.684126] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1091`: [   44.684646] it6251 2-005c: hactive: 1920
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1094`: [   44.684983] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1097`: [   44.685137] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1102`: [   44.685987] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1105`: [   44.686142] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1108`: [   44.686660] it6251 2-005c: vactive: 1080
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/dmesg-full.txt:1109`: [   44.686669] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=0; es8328-power: disabling=0. Diese Zählung ist keine neue Teststatistik.

### evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt

- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt:1`: H3-1R_BOOT=05
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt:2`: POR_COLD_BOOT=YES
- `evidence/block-3.10-h3-1r-por-series-2026-09-15/boot-05/trial.txt:3`: DISPLAY=ON

### evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt

- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:2`: [    0.000000] Linux version 6.18.49 (nixbld@localhost) (armv7l-unknown-linux-gnueabihf-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.46) #1-NixOS SMP Wed Sep  2 12:31:51 UTC 2026
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:249`: [    2.917138] usbcore: registered new interface driver net1080
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:306`: [    3.371369] ata1: SATA link up 3.0 Gbps (SStatus 123 SControl 300)
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:448`: [   33.761742] es8328-power: disabling
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:455`: [   38.610805] Micrel KSZ9021 Gigabit PHY 2188000.ethernet-1:07: attached PHY driver (mii_bus:phy_addr=2188000.ethernet-1:07, irq=POLL)
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:486`: [   42.395306] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:487`: [   42.395351] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:524`: [   44.471549] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:525`: [   44.471609] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:526`: [   44.478167] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:528`: [   44.478258] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:530`: [   44.478522] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:532`: [   44.478550] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:534`: [   44.478784] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:536`: [   44.478812] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:538`: [   44.479063] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:540`: [   44.479091] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:542`: [   44.479124] it6251 2-005c: DEBUG: power_up: product ID read incomplete, sleeping
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:546`: [   44.671400] it6251 2-005c: DEBUG: power_up: product ID attempt 2/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:547`: [   44.671618] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:549`: [   44.671726] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:551`: [   44.672025] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:553`: [   44.672076] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:555`: [   44.672319] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:557`: [   44.672341] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:559`: [   44.672553] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:561`: [   44.672573] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:563`: [   44.672598] it6251 2-005c: DEBUG: power_up: product ID read incomplete, sleeping
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:564`: [   44.872661] it6251 2-005c: DEBUG: power_up: product ID attempt 3/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:565`: [   44.872918] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:567`: [   44.873002] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:569`: [   44.873305] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:571`: [   44.873357] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:573`: [   44.873601] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:575`: [   44.873624] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:577`: [   44.873831] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:579`: [   44.873852] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:581`: [   44.873876] it6251 2-005c: DEBUG: power_up: product ID read incomplete, sleeping
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:582`: [   45.073944] it6251 2-005c: DEBUG: power_up: product ID attempt 4/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:583`: [   45.074221] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:585`: [   45.074300] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:587`: [   45.074589] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:589`: [   45.074640] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:591`: [   45.074917] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:593`: [   45.074971] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:595`: [   45.075252] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:597`: [   45.075304] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:599`: [   45.075364] it6251 2-005c: DEBUG: power_up: product ID read incomplete, sleeping
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:600`: [   45.275415] it6251 2-005c: DEBUG: power_up: product ID attempt 5/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:601`: [   45.275682] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:603`: [   45.275768] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:605`: [   45.276068] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:607`: [   45.276121] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:609`: [   45.276392] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:611`: [   45.276442] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:613`: [   45.276709] i2c i2c-2: NOVENA-I2C: arbitration lost in bus_busy, I2SR=0x93
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:615`: [   45.276757] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=93/80 M1=93/80 M2=93/80 M3=93/80 M4=93/80 M5=93/80 M6=93/80 M7=93/80
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:617`: [   45.276816] it6251 2-005c: DEBUG: power_up: product ID read incomplete, sleeping
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:618`: [   45.476865] it6251 2-005c: unable to read product ID
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:619`: [   45.476886] it6251 2-005c: DEBUG: power_up: disabling regulator after ID failure
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:620`: [   45.476911] it6251 2-005c: DEBUG: power_up: exit failure
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:660`: [  176.341814] it6251 2-005c: DEBUG: power_up: enter powered=0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:661`: [  176.341933] it6251 2-005c: DEBUG: power_up: regulator_enable
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:662`: [  178.389783] it6251 2-005c: DEBUG: power_up: regulator enabled
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:663`: [  178.389871] it6251 2-005c: DEBUG: power_up: product ID attempt 1/5
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:666`: [  178.390607] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:669`: [  178.399194] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:674`: [  178.429678] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:677`: [  178.439514] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=a1/a0 M7=a1/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:682`: [  178.469620] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:685`: [  178.481521] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:690`: [  178.511758] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:693`: [  178.520215] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:697`: [  178.541866] it6251 2-005c: DEBUG: power_up: exit success
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:706`: [  178.550206] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:709`: [  178.560054] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:712`: [  178.571952] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:716`: [  178.580471] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:719`: [  178.590322] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:722`: [  178.602182] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:726`: [  178.610649] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:729`: [  178.620496] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:732`: [  178.632353] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:735`: [  178.640853] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:738`: [  178.650725] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:741`: [  178.662575] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:747`: [  179.003729] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:750`: [  179.012176] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:753`: [  179.022013] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:756`: [  179.033848] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:759`: [  179.042276] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:762`: [  179.052107] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:765`: [  179.063914] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:768`: [  179.072346] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:771`: [  179.082168] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:774`: [  179.093990] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:777`: [  179.102420] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:780`: [  179.112235] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:783`: [  179.124085] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:786`: [  179.132505] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:789`: [  179.142336] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:792`: [  179.154159] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:795`: [  179.162584] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:798`: [  179.172418] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:801`: [  179.184226] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:804`: [  179.192660] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:807`: [  179.202546] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:810`: [  179.214408] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:813`: [  179.222837] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:816`: [  179.232649] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:819`: [  179.244497] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:822`: [  179.252914] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:825`: [  179.262746] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:828`: [  179.274563] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:831`: [  179.282985] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:834`: [  179.292818] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:837`: [  179.304626] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:840`: [  179.313064] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:843`: [  179.322885] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:846`: [  179.334717] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:849`: [  179.343149] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:852`: [  179.352961] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:855`: [  179.364808] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:858`: [  179.373231] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:861`: [  179.383056] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:864`: [  179.394881] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:867`: [  179.403300] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:870`: [  179.413130] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:873`: [  179.424937] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:876`: [  179.433373] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:879`: [  179.443196] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:882`: [  179.455009] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:885`: [  179.463446] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:888`: [  179.473263] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:891`: [  179.485108] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:894`: [  179.493537] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:897`: [  179.493701] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:900`: [  179.503647] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:903`: [  179.515511] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:906`: [  179.523933] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:909`: [  179.533772] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:912`: [  179.545587] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:915`: [  179.554008] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:918`: [  179.563851] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:921`: [  179.575656] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:924`: [  179.584094] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:927`: [  179.593911] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:930`: [  179.605750] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:933`: [  179.614177] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:936`: [  179.624007] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:939`: [  179.635837] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:942`: [  179.644266] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:945`: [  179.654119] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:948`: [  179.665937] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:951`: [  179.674357] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:954`: [  179.684199] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:957`: [  179.696008] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:960`: [  179.704446] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:965`: [  179.714266] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:968`: [  179.726110] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:971`: [  179.734538] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:974`: [  179.744352] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:977`: [  179.756176] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:982`: [  179.786274] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:985`: [  179.794727] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:992`: [  179.824820] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:995`: [  179.834674] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1001`: [  179.864915] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1004`: [  179.876769] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1009`: [  179.906765] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1012`: [  179.915188] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1029`: [  180.104733] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1032`: [  180.116560] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1037`: [  180.146547] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1040`: [  180.154973] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1043`: [  180.176649] it6251 2-005c: hactive: 1920
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1046`: [  180.185023] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1049`: [  180.194851] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1054`: [  180.225116] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1057`: [  180.236944] i2c i2c-2: NOVENA-I2C: IT6251 START transition M0=81/a0 M1=81/a0 M2=81/a0 M3=81/a0 M4=81/a0 M5=81/a0 M6=81/a0 M7=81/a0
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1060`: [  180.255184] it6251 2-005c: vactive: 1080
- `evidence/block-3.5-h3-1-2026-09-15/dmesg-full.txt:1061`: [  180.255191] it6251 2-005c: DEBUG: is_stable: stable 1920x1080

Textzählung: arbitration lost=20; es8328-power: disabling=1. Diese Zählung ist keine neue Teststatistik.
