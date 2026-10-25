# Novena – Image-Inventur

Stand: 25.10.2026

## Umfang und Integrität

- Erfasste Artefakte: **209**
- Unterschiedliche SHA256-Dateiinhalte: **116**
- foobox: 174
- VM102: 24
- VM104: 11
- Scanwarnungen: 1

Quelle: NOVENA-IMAGE-INVENTORY-96 (Original-TXT und SHA256).
Keine neuen Builds oder Hardwaretests.
Die Erfassung deckt die festgelegten Suchbereiche ab,
nicht sämtliche Verzeichnisse und Datenträger.

## Bekannte Images und Tests

| Image | SHA256 | Stand |
|---|---|---|
| Historisches U-Boot 2015.04 / NixOS | `4748322366a17dcfcafa75d4591f60ebc34629d9e4973807ab81a7794117170c` | Hardwareboot bis Login am 18.10. dokumentiert |
| Image 2 mit korrigiertem MBR-Bootflag | `144d71c37536d09e5e47d41eb270fa131bdd154244fdd2d21a37a4b0d958777d` | Build und Offline-Prüfung; eigener Hardwareboot nicht zugeordnet |
| VM102/VM104-Reproduzierbarkeitsreferenz | `40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a` | Byteidentische VM-Images; Hardwareboot nicht zugeordnet |
| U-Boot-2026.07-Baseline | `c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750` | VM102/VM104 byteidentisch; SPL offline qualifiziert |
| foobox-Displayvariante | `99c34a52484e3d43598fe6487c97d76fb5bdafc58efb934769b6923f7863d5a9` | Offline qualifiziert; Hardwaretest steht aus |

## Hardware-Referenz vom 24.10.2026

Automatischer externer SD-Boot mit SPL/U-Boot 2026.07,
Linux 6.18.49 und NixOS 26.05 bis zum seriellen Login
ist dokumentiert. NOVENA-BOOT-VERIFY-02 bestätigte
SSH, Root auf `/dev/mmcblk1p2` und systemd running.

Die exakte SHA256-Identität des damals gebooteten
vollständigen SD-Inhalts bleibt in dieser Inventur offen.
Dieser Hardwareerfolg wird neueren Images daher
nicht ohne zusätzlichen Identitätsnachweis zugeordnet.

## Bereits untersuchte Hostunterschiede

Der Vergleich foobox/VM102 vom 20.10. dokumentiert
unterschiedliche Root-Dateisysteminhalte bei gleichem
Store-Pfad, insbesondere Boost, Coreutils und
`/nix-path-registration`. MBR, erste 8 MiB und FAT32
waren dort identisch. Die tieferliegende Ursache
blieb offen. Siehe `docs/NOVENA-DUAL-REPRO-2026-10-20.md`.

VM102/VM104 lieferten für die dokumentierten
Referenzderivationen byteidentische SD-Images.

## Hostabhängige Store-Pfade

| Pfad | Host | SHA256 |
|---|---|---|
| `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` | foobox | `b808e9de0a707b1647e4d1a9ade175ef05e3d6b6d6ab4b7acdb50f8c33d35e45` |
| `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` | VM102 | `40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a` |
| `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` | VM104 | `40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a` |
| `/nix/store/cd8kj8nr539kkv7xl5zmvz0xss4n95gf-nixos-novena-sd-image.img` | foobox | `5e4413cba9d284041fe3b1ca42e403e46d1cc53abdadc6c9346c62214a8d961c` |
| `/nix/store/cd8kj8nr539kkv7xl5zmvz0xss4n95gf-nixos-novena-sd-image.img` | VM102 | `67f7899000584867467c8740e39df46ef67d3320fd06ca69d0607b01c8af528c` |
| `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` | foobox | `907ae2fa24eeec2f4a3514ffcd08c6a3057c97c93c60c96728264ec52943743d` |
| `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` | VM102 | `c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750` |
| `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` | VM104 | `c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750` |
| `/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img` | foobox | `3ed8c49d23ab2d5cdc3a0905c6d01938fafcb8dde91c9a1418dc45ee66eb3ddc` |
| `/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img` | VM102 | `b1ae43b3ce973377faa92faa5d466a94f3cda4357dacca26e1390a0e67cc7ad9` |

## Vollständige Artefaktliste

| Host | Typ | Bytes | SHA256 | Pfad |
|---|---|---:|---|---|
| VM102 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-cross/boot/reference/novena-imx6-spl.bin` |
| VM102 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-cross/boot/reference/u-boot-dtb.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-cross/boot/u-boot/historical-2015/u-boot.img` |
| VM102 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-current-image-20261023/source/boot/reference/novena-imx6-spl.bin` |
| VM102 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-current-image-20261023/source/boot/reference/u-boot-dtb.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-current-image-20261023/source/boot/u-boot/historical-2015/u-boot.img` |
| VM102 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-dual-build-20261020/source/boot/reference/novena-imx6-spl.bin` |
| VM102 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-dual-build-20261020/source/boot/reference/u-boot-dtb.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-dual-build-20261020/source/boot/u-boot/historical-2015/u-boot.img` |
| VM102 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/novena-imx6-spl.bin` |
| VM102 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/u-boot-dtb.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-dual-build-20261022/source/boot/u-boot/historical-2015/u-boot.img` |
| VM102 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/reference/novena-imx6-spl.bin` |
| VM102 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/reference/u-boot-dtb.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/u-boot/historical-2015/u-boot.img` |
| VM102 | NixOS-SD-Image | 3240607744 | `40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a` | `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` |
| VM102 | Firmware | 134217728 | `03cd047b6714d0bc9dfcea65cae6206f79d2f30c5388a1587165daa76df18e25` | `/nix/store/bqqb5nn43w31c7608v5dign54amqwkry-novena-firmware.img` |
| VM102 | NixOS-SD-Image | 2588422144 | `67f7899000584867467c8740e39df46ef67d3320fd06ca69d0607b01c8af528c` | `/nix/store/cd8kj8nr539kkv7xl5zmvz0xss4n95gf-nixos-novena-sd-image.img` |
| VM102 | NixOS-SD-Image | 3240607744 | `c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750` | `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` |
| VM102 | Firmware | 134217728 | `9ce335050e08d3547dd7cf65fa301cacecb09e64106a5d2f5ce7900c20aa26db` | `/nix/store/g8jzngpggxfcx1qwjjfpmigcb32h5sy7-novena-firmware.img` |
| VM102 | Firmware | 134217728 | `0b4728cd1ea8ae17408d9ba4b9d7de9e55710f56cbc8725f8b11982d7873be1c` | `/nix/store/ggw5q3zkgwx6c99995df75gcp6iwjyyg-novena-firmware.img` |
| VM102 | Firmware | 134217728 | `fc7c0b5734fde85d731efeff2135e035fe9ba945518d76b01fd628276dce545f` | `/nix/store/p6klk6ylgkhhkhmf90hnx8msg40c8285-novena-firmware.img` |
| VM102 | NixOS-SD-Image | 2588422144 | `b1ae43b3ce973377faa92faa5d466a94f3cda4357dacca26e1390a0e67cc7ad9` | `/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img` |
| VM102 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/nix/store/r31sbqjyqn1qm3k8yfy41difnmb692d6-u-boot.img` |
| VM104 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/novena-imx6-spl.bin` |
| VM104 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/u-boot-dtb.img` |
| VM104 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-dual-build-20261022/source/boot/u-boot/historical-2015/u-boot.img` |
| VM104 | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/reference/novena-imx6-spl.bin` |
| VM104 | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/reference/u-boot-dtb.img` |
| VM104 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-uboot26-20261024-262c2d5/source/boot/u-boot/historical-2015/u-boot.img` |
| VM104 | NixOS-SD-Image | 3240607744 | `40169e4b275f1c8c882312f2dacfd28a9b64e63d0147fd562bf43fa9c4934a1a` | `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` |
| VM104 | NixOS-SD-Image | 3240607744 | `c858261155ce83813feaef20ed062ea66e5989b170e43c995270f70ede763750` | `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` |
| VM104 | Firmware | 134217728 | `0b4728cd1ea8ae17408d9ba4b9d7de9e55710f56cbc8725f8b11982d7873be1c` | `/nix/store/ggw5q3zkgwx6c99995df75gcp6iwjyyg-novena-firmware.img` |
| VM104 | Firmware | 134217728 | `fc7c0b5734fde85d731efeff2135e035fe9ba945518d76b01fd628276dce545f` | `/nix/store/p6klk6ylgkhhkhmf90hnx8msg40c8285-novena-firmware.img` |
| VM104 | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/nix/store/r31sbqjyqn1qm3k8yfy41difnmb692d6-u-boot.img` |
| foobox | Sonstiges Image/Binärartefakt | 409600000 | `40b5bba993ef8cf5b641281b13c12c09fbdb25a633e7bb8d4718178d02c831b8` | `/home/loomit/Downloads/novena-installer-microsd.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/nixos-novena-image/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/nixos-novena-image/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/nixos-novena-image/boot/u-boot/historical-2015/u-boot.img` |
| foobox | Sonstiges Image/Binärartefakt | 14331100 | `9935b49963b01826588edf427f16beaf0785d0bd943813c05903f5793fd49ac6` | `/home/loomit/novena-arch/ArchLinuxARM-armv7-latest/boot/initramfs-linux.img` |
| foobox | Sonstiges Image/Binärartefakt | 31666995200 | `7e9028d5b127ead9bf20867bd8d6882ae5ff6b256f0504c21de5fe8267f3ead9` | `/home/loomit/novena-backups/historical-novena-usb-2026-09-28/novena-historical-usb-ddrescue-2026-09-28.img` |
| foobox | Sonstiges Image/Binärartefakt | 11098128384 | `660cf1f33ff4ce93de07f2b8a78488ab415eebee85eebf6b9a9d9764248835af` | `/home/loomit/novena-backups/historical-novena-usb-2026-09-28/novena-historical-usb-full-2026-09-28.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-backups/historical-uboot-2015-analysis/u-boot-2015.04-bd5c7d3.img` |
| foobox | Sonstiges Image/Binärartefakt | 15068037120 | `0648f1dbf360ee58a424c26c44683ed0e9e9ed0d0ac0381118ef9d12c2f897b2` | `/home/loomit/novena-backups/images/managed/0648f1dbf360ee58-novena-jessie-43-experimental.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `1817c25fe4180d3b683bc3610ea979fdef5d9c0e0ef7d4b0c799f20aaeae5a98` | `/home/loomit/novena-backups/images/managed/1817c25fe4180d3b-historical-swap.img` |
| foobox | U-Boot | 305148 | `3616d1e203e79702163bff359d52b718502483fd0bd098d23646654224942a72` | `/home/loomit/novena-backups/images/managed/3616d1e203e79702-u-boot.img` |
| foobox | Sonstiges Image/Binärartefakt | 2437636096 | `400d6a10e3d8367f89ad889e1c4840d08c562eb051ff3b215633650fb9946fbd` | `/home/loomit/novena-backups/images/managed/400d6a10e3d8367f-rootfs-build-b.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `4d80a1d1e1d7235aa08cd48a00fcd5e0d5001d5a5cc39b19cc64d51086b79bde` | `/home/loomit/novena-backups/images/managed/4d80a1d1e1d7235a-historical-fat.img` |
| foobox | NixOS-SD-Image | 2581291008 | `5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b` | `/home/loomit/novena-backups/images/managed/5e65bfa6b7c599bd-nixos-novena-sd-image.img` |
| foobox | Sonstiges Image/Binärartefakt | 14999879680 | `644c944e85e8e9f742e0f43a6a68c1823adb9f89df1a5dff759be9a922266ed8` | `/home/loomit/novena-backups/images/managed/644c944e85e8e9f7-historical-rootfs.img` |
| foobox | Sonstiges Image/Binärartefakt | 11098128384 | `660cf1f33ff4ce93de07f2b8a78488ab415eebee85eebf6b9a9d9764248835af` | `/home/loomit/novena-backups/images/managed/660cf1f33ff4ce93-novena-historical-usb-full-2026-09-28.img` |
| foobox | Sonstiges Image/Binärartefakt | 31666995200 | `7e9028d5b127ead9bf20867bd8d6882ae5ff6b256f0504c21de5fe8267f3ead9` | `/home/loomit/novena-backups/images/managed/7e9028d5b127ead9-novena-historical-usb-ddrescue-2026-09-28.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-backups/images/managed/7eba2bbdcf48f19f-u-boot-2015.04-bd5c7d3.img` |
| foobox | Sonstiges Image/Binärartefakt | 2437636096 | `8663fb3e84a54698e8e38b278968e0420029e3a2604d2f0691d76edd301da037` | `/home/loomit/novena-backups/images/managed/8663fb3e84a54698-rootfs-build-d.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `c6ca9a29804c2b13f1a1e9de516b3c44d289de2b701f3c33e412e5cceabc81a6` | `/home/loomit/novena-backups/images/managed/c6ca9a29804c2b13-jessie-2015-fat-candidate.img` |
| foobox | Firmware | 134217728 | `e9763020cfd9de2f4426e8d5c91f48723fc594599d91062b58bfdbddda9c63aa` | `/home/loomit/novena-backups/images/managed/e9763020cfd9de2f-novena-test-sd-firmware-before-0006.img` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-backups/images/managed/f1ec961dae762ace-u-boot-dtb.img` |
| foobox | Sonstiges Image/Binärartefakt | 2581291008 | `fac73d584ac5f7e48916a53981bc1051231c583ae89833dae0bee0dc4807c2a4` | `/home/loomit/novena-backups/images/managed/fac73d584ac5f7e4-image-build-f.img` |
| foobox | Sonstiges Image/Binärartefakt | 15068037120 | `0648f1dbf360ee58a424c26c44683ed0e9e9ed0d0ac0381118ef9d12c2f897b2` | `/home/loomit/novena-backups/images/novena-jessie-43-experimental.img` |
| foobox | NixOS-SD-Image | 2581291008 | `5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b` | `/home/loomit/novena-backups/nixos-display-working-2026-09-11/image/nixos-novena-sd-image.img` |
| foobox | SPL | 52224 | `c79d5797695109d8545a99df6252325930c1e6cbbba2744e21677dc7afc1d1f5` | `/home/loomit/novena-backups/novena-test-sd-pre-d0-d36-spl-2026-09-23.bin` |
| foobox | SPL | 56320 | `d0d72af7fa28dc59a67016028797b594ac2c5ea793b4011771deb31c8e3b44b3` | `/home/loomit/novena-backups/novena-test-sd-pre-d21a-d21d-spl-2026-09-23.bin` |
| foobox | SPL | 56320 | `6b16b53c6f2346b24ea010a1b6bb3b44beedfe09f61652450cf76e84180133d0` | `/home/loomit/novena-backups/novena-test-sd-pre-p0-p1-spl-2026-09-23.bin` |
| foobox | SPL | 56320 | `a4c85598132fe313c625a30bcf81d03024b896944bb4d6455e95c88ac0e8f1fc` | `/home/loomit/novena-backups/novena-test-sd-pre-q1-q2-p0-0007-spl-2026-09-23.bin` |
| foobox | Firmware | 134217728 | `e9763020cfd9de2f4426e8d5c91f48723fc594599d91062b58bfdbddda9c63aa` | `/home/loomit/novena-backups/test-sd-before-0006-2026-09-14/novena-test-sd-firmware-before-0006.img` |
| foobox | Sonstiges Image/Binärartefakt | 16777216 | `247bd36898839f7ae471d621c7cbbbb4a2bee63973fc5d3cb1f723978b12dc79` | `/home/loomit/novena-backups/test-sd-before-0006-2026-09-14/novena-test-sd-first-16MiB-before-0006.bin` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-current-image-20261023/source/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-current-image-20261023/source/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-current-image-20261023/source/boot/u-boot/historical-2015/u-boot.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-display-variant-84/source/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-display-variant-84/source/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-display-variant-84/source/boot/u-boot/historical-2015/u-boot.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-dual-build-20261020/source/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-dual-build-20261020/source/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-dual-build-20261020/source/boot/u-boot/historical-2015/u-boot.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-dual-build-20261022/source/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/home/loomit/novena-dual-build-20261022/source/boot/u-boot/historical-2015/u-boot.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/home/loomit/novena-jessie-20-codex/boot/reference/novena-imx6-spl.bin` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/home/loomit/novena-jessie-20-codex/boot/reference/u-boot-dtb.img` |
| foobox | U-Boot | 305148 | `3616d1e203e79702163bff359d52b718502483fd0bd098d23646654224942a72` | `/home/loomit/novena-jessie-artifacts/historical-fat-files/u-boot.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `4d80a1d1e1d7235aa08cd48a00fcd5e0d5001d5a5cc39b19cc64d51086b79bde` | `/home/loomit/novena-jessie-artifacts/historical-fat.img` |
| foobox | Sonstiges Image/Binärartefakt | 14999879680 | `644c944e85e8e9f742e0f43a6a68c1823adb9f89df1a5dff759be9a922266ed8` | `/home/loomit/novena-jessie-artifacts/historical-rootfs.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `1817c25fe4180d3b683bc3610ea979fdef5d9c0e0ef7d4b0c799f20aaeae5a98` | `/home/loomit/novena-jessie-artifacts/historical-swap.img` |
| foobox | Sonstiges Image/Binärartefakt | 33554432 | `c6ca9a29804c2b13f1a1e9de516b3c44d289de2b701f3c33e412e5cceabc81a6` | `/home/loomit/novena-jessie-artifacts/jessie-2015-fat-candidate.img` |
| foobox | Sonstiges Image/Binärartefakt | 2581291008 | `fac73d584ac5f7e48916a53981bc1051231c583ae89833dae0bee0dc4807c2a4` | `/home/loomit/novena-repro/image-build-f.img` |
| foobox | Sonstiges Image/Binärartefakt | 2437636096 | `400d6a10e3d8367f89ad889e1c4840d08c562eb051ff3b215633650fb9946fbd` | `/home/loomit/novena-repro/rootfs-build-b.img` |
| foobox | Sonstiges Image/Binärartefakt | 2437636096 | `8663fb3e84a54698e8e38b278968e0420029e3a2604d2f0691d76edd301da037` | `/home/loomit/novena-repro/rootfs-build-d.img` |
| foobox | SPL | 47200 | `869e88415651f08d063c320073af17bfe5cef5fd08599d68af2a792586f77403` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/spl/u-boot-spl-nodtb.bin` |
| foobox | SPL | 47200 | `869e88415651f08d063c320073af17bfe5cef5fd08599d68af2a792586f77403` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/spl/u-boot-spl.bin` |
| foobox | U-Boot | 602168 | `bec21fbb3d43029a7af0c43e58087122f1909a9e5e7cfb242994ac41bc17d587` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/u-boot-dtb.bin` |
| foobox | U-Boot | 602232 | `5433fdb86950a8dac6bd5025827a29910180d38dca16fa59c9b9c1a8b5e2d4f4` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/u-boot-dtb.img` |
| foobox | U-Boot | 559144 | `da973c614af7035a3d590a07fded67a3e271d1ac76c22bd40d1309537b19da1e` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/u-boot-nodtb.bin` |
| foobox | U-Boot | 602168 | `bec21fbb3d43029a7af0c43e58087122f1909a9e5e7cfb242994ac41bc17d587` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/u-boot.bin` |
| foobox | U-Boot | 602232 | `5433fdb86950a8dac6bd5025827a29910180d38dca16fa59c9b9c1a8b5e2d4f4` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-build-obsolete/u-boot.img` |
| foobox | U-Boot | 602928 | `80d6975313b539128c706133121a884b93ea54a1c0c232f6b715b263dd038179` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/diagnostic-nix-output/u-boot-dtb.img` |
| foobox | SPL | 52224 | `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/sd-spl/novena-test-sd-current-spl-4.8d.bin` |
| foobox | SPL | 52224 | `c79d5797695109d8545a99df6252325930c1e6cbbba2744e21677dc7afc1d1f5` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/sd-spl/novena-test-sd-diagnostic-spl-final-readback.bin` |
| foobox | SPL | 52224 | `c79d5797695109d8545a99df6252325930c1e6cbbba2744e21677dc7afc1d1f5` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/sd-spl/novena-test-sd-diagnostic-spl-readback.bin` |
| foobox | SPL | 52224 | `c79b6efadee73f461b564f0cfc3e11a4123dc5c61c1e6d6b7d229c0880bdbcf0` | `/home/loomit/novena-research/checkpoints/2026-09-22-p-ext-d0-d1-pause/sd-spl/novena-test-sd-production-spl-before-diag.bin` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/spl/u-boot-spl-nodtb.bin` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/spl/u-boot-spl.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/u-boot-dtb.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/u-boot-dtb.img` |
| foobox | U-Boot | 612400 | `be7d22759f5c2f0a288f355fd5dc58d17856fff04210de5898423dff5eb75fd8` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/u-boot-nodtb.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/u-boot.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-2/u-boot.img` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/spl/u-boot-spl-nodtb.bin` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/spl/u-boot-spl.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/u-boot-dtb.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/u-boot-dtb.img` |
| foobox | U-Boot | 612400 | `be7d22759f5c2f0a288f355fd5dc58d17856fff04210de5898423dff5eb75fd8` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/u-boot-nodtb.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/u-boot.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-build-3/u-boot.img` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/spl/u-boot-spl-nodtb.bin` |
| foobox | SPL | 51832 | `8db9f220c2421e9146cd39c04dba39c01da4d9e03df819a07c1e9bc538605c05` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/spl/u-boot-spl.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/u-boot-dtb.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/u-boot-dtb.img` |
| foobox | U-Boot | 612400 | `be7d22759f5c2f0a288f355fd5dc58d17856fff04210de5898423dff5eb75fd8` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/u-boot-nodtb.bin` |
| foobox | U-Boot | 655424 | `5e25c25d33550aee3685dc4b5dc28a72a295c5181064c004de08c80a50018690` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/u-boot.bin` |
| foobox | U-Boot | 655488 | `7784a6a7159d4e5db4d79d8bcd44ed475d611df430d64bce7cff06519febed41` | `/home/loomit/novena-research/u-boot-v2026.07-novena-external-nogit-build/u-boot.img` |
| foobox | Firmware | 134217728 | `cd8a554dca46fe62411768e728e9560105c92f7a9a287d5cd5214a274e214cae` | `/nix/store/06c023zr35iq5ifa73g8skwrvpmkyv5x-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `90405b094423b8e515d670f3a72cb3146c68d0ad5f65d5d71d88a965966f0fe3` | `/nix/store/0b97xgj92hhrj0p5998k4rmazknaaag2-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `ca1f07a3b425b713f842200c1711457768f19d0ab5ba969badb53aa983f6c718` | `/nix/store/0ri6j53q0gw7h9mw5c7ca9mn4jj6lmdm-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2580951040 | `13bb64ab21111267e3f0ba732ffdd480e0561a70a49c7ce3f79bcd52059c331c` | `/nix/store/0xhl3p3wvmqd0pssnzc29zaazjkni9x3-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `37686416d84f81bc29cf163f19736b7126ab465eb00a719fd0025e0d02923d27` | `/nix/store/1acbdciklyna0560ksbbnp2k1aqml623-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `4106f62659f980611085c9c7ff3a5d8921068a002a3f699aad0bdc1a0f7ffd33` | `/nix/store/1y5f4d1zjbbrwr1h0yp4h8572qj4s95s-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `32b17519dee56addf29bd6b9c105be9ea0934c25817f02800787145d9ac88cbd` | `/nix/store/23f1flbp5dsyr9qxdh2mvkbfqaf3kjmi-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `ecbe5e72afcd36911378875133da27546cfa49c7a17b5e8de777dcbc27dc40ce` | `/nix/store/2h9qi4qsinnilwgz9l9q0kbmhf2wbzsl-novena-firmware.img` |
| foobox | Firmware | 134217728 | `7262ce1352c809da9e04e06e8e64d0bf62720bbcd25b95817f142564ffa6335b` | `/nix/store/33g7bypi489q2b8j8g5dbk8847y9vbps-novena-firmware.img` |
| foobox | Firmware | 134217728 | `73703be3fc9e60a2b7cef7a84aa5b49867c767abc0e197e69b2912440ba1ab2a` | `/nix/store/3wadmprkamn9k0q8i2ldcq913pf8imvy-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `c695321cb8e1b685d9aaf005a4068ab0d5cc1a86374285f952d56d1855ddcaf9` | `/nix/store/3yp24cyr6m9x694r3j6yy9p2fpr0i6n4-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `fd7925c1bf30086fb2978d846327000b7d1956334f7ede9da280efd14dcd7126` | `/nix/store/473fhss6pcrs0giwx4zvp97sjd4vwi9q-novena-firmware.img` |
| foobox | Firmware | 134217728 | `daeca8f8284da3746b455b8f8fa3ef85332453d1522e68edc4d9d9f110fae956` | `/nix/store/49b78mc1lyi2kly5gpws61dcxwr7djj2-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `6b66b83ae7e51f641c9d77354207680b7d0bc59fbf64d7b6aefc6fbacc2942dd` | `/nix/store/4bvfyl480nmcaj12lzkir94zqw9z7aia-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `4ddd0b7565fedab5866f4327acb420ee190007a1a6847eeb9c64ab06928a3873` | `/nix/store/4hy11rbp3j29gsdnvr0g1r0lrn98x8sl-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `d65c113f188e4b9d91ca617eef95b39745142948629ccf57cb61135c38982f5d` | `/nix/store/4lrgc6nj8a3qpr2mc5x4bw95jkaxwc9h-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `3098595763da59e0242d75e6fc91346e8f70735291ceb791040069df0f21bab9` | `/nix/store/4ywiljnw7p7m3dnhv479qlz25isbpdwy-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `67050fe1948e35a5a41ed8fb04288e1921fb6c1e0eb85e163afad7d1b1e71b1b` | `/nix/store/56v90fj1x6sax5dv1h2imp6qg2k3msgl-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `6291dd8521c43cf599ce982ab57c0ff6654f2fc0d7e7c65841097461c11a0c06` | `/nix/store/5bp2ff3rirhxkv8vas6ykqm515wa3xms-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `1600eebae47bf35190a49e5bd90055d59bc2d2309ec34fe62e6dd41d22e384c6` | `/nix/store/5mq4vi3yqrcfp58cp8y5grif94m9vnis-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `bbcfa910cc085ae5bc75f3ae02645b7cf709b78c80253329e1963b9b844fd1d7` | `/nix/store/61391i7ik30bmiagd9pzfrw5fqvgil3i-novena-firmware.img` |
| foobox | Firmware | 134217728 | `a1c6083ae02891af1e0ad9191d2e0d9d99f2363ffc156c7b7574ea451f03e940` | `/nix/store/690gnz0s2bdw6x9h4c05f0rkbx13pfw0-novena-firmware.img` |
| foobox | NixOS-SD-Image | 3240656896 | `b808e9de0a707b1647e4d1a9ade175ef05e3d6b6d6ab4b7acdb50f8c33d35e45` | `/nix/store/6d32c794lxlhd2pyfi53ivk3qylnwsir-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `d5400e782b98ec8a6b786970bf744918063539adeef54a2d528803dcb1a1c132` | `/nix/store/7b41wnr8nqvgm48jkfyishpy9jpgzw56-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `7960fbd2322265bb251292599429de3cb60150fcc82ffbf597a294bb1c6ead1a` | `/nix/store/7im4xgrhfj1s7847ngprapq4zkk0zczx-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `cd3f27b3410d97233086e923e16b4daa23d6f64ccf570e3a3e5362efc6bf8585` | `/nix/store/7k35fsdk7b3mf7jv3vdlx27k4l08ggkd-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `1e1ecbf127aeea31a80c83c4b3e5aa06dd3301a11a734c61a48962a1768f83a0` | `/nix/store/8vsbp3bs5hl8lh4g4w9l1c0rbh9q1ghh-novena-firmware.img` |
| foobox | Firmware | 134217728 | `2ff845ad97bf8b083b544eb4f4601bbcc03f8e09302cef42bce04974177ada65` | `/nix/store/8z125qlggpcddpg4sad1db49n592p581-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `ec44de089d2dbe99f19165c84d107f9f4946411cb2895f14d185e417ae97e8f4` | `/nix/store/97w3dnmcljq4czihrqfpsxiz6nfwg37v-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `72cc667e44993e82910dc0831c5f062bcb042678524f631f458f30484fff6260` | `/nix/store/9np0mrf37sasffgwjma39nb8pc9fl35p-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `6ba256e763d51c6191f288a7b457fe03b4941d9248c9e9c34fed03a7401a8dbd` | `/nix/store/9zmbxslzgycgswf92ga68h4l444fddfq-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `86e209e38bf2b20952cac25f6bfafcfc8be688c89a68b13f42f67fd823d117cc` | `/nix/store/a53k3l3gljf1ynd6wfhzzzjgzap8mb4n-novena-firmware.img` |
| foobox | Firmware | 134217728 | `1c63e3011f0b24f9ff619273153f444af844b7cf107c15122626d79704681afc` | `/nix/store/b1g17380lb506iv5islbc45wamlms2py-novena-firmware.img` |
| foobox | Firmware | 134217728 | `03cd047b6714d0bc9dfcea65cae6206f79d2f30c5388a1587165daa76df18e25` | `/nix/store/bqqb5nn43w31c7608v5dign54amqwkry-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2588422144 | `5e4413cba9d284041fe3b1ca42e403e46d1cc53abdadc6c9346c62214a8d961c` | `/nix/store/cd8kj8nr539kkv7xl5zmvz0xss4n95gf-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `3c0ea761baf210e968e49fb5885c5da07e1e4f9a2b612a5f87c2af47adef865c` | `/nix/store/cmh47hrxpfwsx5mhdkrxq7y8qq716nba-novena-firmware.img` |
| foobox | NixOS-SD-Image | 3240607744 | `907ae2fa24eeec2f4a3514ffcd08c6a3057c97c93c60c96728264ec52943743d` | `/nix/store/cn9ycl4fgjk1ld9nsqxf7adimxldsfbp-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `c4be73bc199108c30454a7c81839267bd110f40bc39b7e3db696e2ac57da2156` | `/nix/store/cpd2gla0g8bj1qbwawjfcsp8ryr29s8g-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `481fac61e0f47579edd5c09327a086c9a7f8a574b9b4a03ff46c70c7fd107dc3` | `/nix/store/dr576pxn22cw5p4p3a9qi1hhp8j4fqyr-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2580951040 | `47eaecd679665b8956ff64f93916148d805b26eec17ef786ff95f3c9fb2be4c9` | `/nix/store/f74acmfm42bpq3w2yh9wrgp6aw0fsv4s-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `2922203ecfe83f61a1c183be3f8d512199275963ab1e26ae41e961c88905d6ce` | `/nix/store/f8kbf41r0c99qy4badjdp2pbawwmqxkh-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `e11376127b20e04393fe524614dd558b36f1e16f049fd6bad0613144e4fafbe1` | `/nix/store/fzx1m29zrs783d1l7y56s81y891nibnw-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `9ce335050e08d3547dd7cf65fa301cacecb09e64106a5d2f5ce7900c20aa26db` | `/nix/store/g8jzngpggxfcx1qwjjfpmigcb32h5sy7-novena-firmware.img` |
| foobox | U-Boot | 510240 | `f1ec961dae762ace49b9f45baa920b7c65dd54f3e53b8a86616543a0b2ce67fa` | `/nix/store/g9mxrs2kpgw64qb93kkz6rmd7hbfgwqm-u-boot-dtb.img` |
| foobox | Firmware | 134217728 | `0b4728cd1ea8ae17408d9ba4b9d7de9e55710f56cbc8725f8b11982d7873be1c` | `/nix/store/ggw5q3zkgwx6c99995df75gcp6iwjyyg-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `8a2b8ac8681a78f9697b4a68582e6afcb1e0164e60a44286b7576d81242226dc` | `/nix/store/gxbxadqv2lpkx5k4b4pz08s9jw185q12-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `fac73d584ac5f7e48916a53981bc1051231c583ae89833dae0bee0dc4807c2a4` | `/nix/store/hb2s0idzllgb22fnqgcrln6g89kfxk4y-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `4748322366a17dcfcafa75d4591f60ebc34629d9e4973807ab81a7794117170c` | `/nix/store/hs4ygzgcd53hpy5hnxrrgvryw95fk5c8-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `42c187faf0e970187553a8cd79c9c9185d2ed302a1e0ccf7dbfaf7c65b328e0f` | `/nix/store/idcypj7b26z23jg204rzxq837p1w84x4-novena-firmware.img` |
| foobox | Firmware | 134217728 | `a1c6083ae02891af1e0ad9191d2e0d9d99f2363ffc156c7b7574ea451f03e940` | `/nix/store/isby0ppgg41p2srn62d1mccwfc8sc7jv-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `0c176764e410139e2cbbd6fdb331d222589351256aa7bed10ce7ea452c726a35` | `/nix/store/jld22ny37wwvsaxgmqz90yqv4jcfi9h7-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `c662f8c69522709b57f0c3acd2f1d9ca743d642d833d2adbfc445b3d3335f88f` | `/nix/store/kl7ykfhld7a9qpryx6qzhbwx9q6j17wy-novena-firmware.img` |
| foobox | Firmware | 134217728 | `a1c6083ae02891af1e0ad9191d2e0d9d99f2363ffc156c7b7574ea451f03e940` | `/nix/store/l9iavqlk3iv7948h4l43zxzymraykb9f-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `c8430e7177707ad72260c2a38cb9b663f40e03e2b7cbcd755eb90c1f9160e822` | `/nix/store/lxdg2lhzcds015yl2b50whhs27qrnn17-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `a1c6083ae02891af1e0ad9191d2e0d9d99f2363ffc156c7b7574ea451f03e940` | `/nix/store/m09wc1228c9ajr9j7qwnsrhfp1cjfqiz-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2588422144 | `144d71c37536d09e5e47d41eb270fa131bdd154244fdd2d21a37a4b0d958777d` | `/nix/store/n7bm1lbqbhxk62h1n45k0xwis9nzk091-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `de467dcee83689f79b2474f00be674fdad460fdadd3b0e0b483ee78c751fc3d9` | `/nix/store/n8qwbvh7mzw69vv3w6cizrzwdrxcisrb-novena-firmware.img` |
| foobox | NixOS-SD-Image | 3240656896 | `2e3eed0a0a1474c288695c8fa2e6552cbf6b1fb96a8ececaf31df8a16cba295e` | `/nix/store/nnr38p6njqs8xry83mlyrmjay9iaccya-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `91eafa5e5d85e307481c1bf4f54ad18530baea52c8ca2c5ab25f176c27197220` | `/nix/store/npp942by2630vq1jk39vjnp6s2wap4x5-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `c6857f7c6407f25e26cb9087fc42fcaf3963e225727f1123caa492baf1a2c4cc` | `/nix/store/p2xjimg1vx7z3rw8fifajng86k44rqjr-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `937776dbb17f0a870fca22519c20a04845dd530de7db8c0496b3e5d286cc2934` | `/nix/store/p4fwzdzm8j57xji213vcva4wg5a79hiq-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `fc7c0b5734fde85d731efeff2135e035fe9ba945518d76b01fd628276dce545f` | `/nix/store/p6klk6ylgkhhkhmf90hnx8msg40c8285-novena-firmware.img` |
| foobox | Firmware | 134217728 | `4e4569b8a801fbe865c542ccb0c64f437ed49fb8c5672bea2478cfcc53f82a97` | `/nix/store/ppy7n6x4ca7khmm38f44qyq47qfm69wg-novena-firmware.img` |
| foobox | Firmware | 134217728 | `399fb8f4533a1636b563c9b74c15e12dee4d77f918a8ce5b08040f3b5cc8cf55` | `/nix/store/qsagrz3fgm6cd3dkjil4id6gi1ypvhbr-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2588422144 | `3ed8c49d23ab2d5cdc3a0905c6d01938fafcb8dde91c9a1418dc45ee66eb3ddc` | `/nix/store/qw34m98di75dsbwygfhmyapp9rk28fk4-nixos-novena-sd-image.img` |
| foobox | U-Boot | 302172 | `7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a` | `/nix/store/r31sbqjyqn1qm3k8yfy41difnmb692d6-u-boot.img` |
| foobox | NixOS-SD-Image | 2581291008 | `129ef7cde9b2ff32f188dc44a9f49c4cf6c9812eb3ff31dc41f0dc0a11e439a9` | `/nix/store/sadb8h8yqc7mm74g73cl81hxih6rlzis-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `f3d52414233f5d71c4f5cfb5d120504ac7fd04f1458492eda85353335bd25782` | `/nix/store/sidibq1hdjyq5w5fv9lqqsdm8413748j-novena-firmware.img` |
| foobox | Firmware | 134217728 | `50b0fd065ae0f3570e7db03b6c1fdce350cdc8d2dea9cca407cc41e8cc91dbfb` | `/nix/store/sk6mhkw4z2l8mdhqrbpcnrpf87n9r26k-novena-firmware.img` |
| foobox | NixOS-SD-Image | 3240607744 | `99c34a52484e3d43598fe6487c97d76fb5bdafc58efb934769b6923f7863d5a9` | `/nix/store/sl0daijv655id7zshmrnrmhr27cf95a8-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `a7d3d530bf915bc1bb3ee1b8a065f2added45df89d1d23d6cfd32d803237dd08` | `/nix/store/spg8m7z2k7bq6v7lqhrxvhf15z049aja-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `5e65bfa6b7c599bdf507c2f1957c99136255e74eff01dec0dfb0bf8fb31ab94b` | `/nix/store/sz31gh0cxpm9yksi89vc990k4nid7k4y-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `6a12775b85a110da8ddf17fd625e9d7b1bdcdcf8e0fdf973e5655380982bf99b` | `/nix/store/vfva9cjq1qd78aw5ksywngyh97zc1zpw-novena-firmware.img` |
| foobox | Firmware | 134217728 | `f026d069aa5d5f1b3944c503e92b4e4f2c9d4914e6ed752a195154eabf9ddc38` | `/nix/store/w14vnfmg0gvcvz8y412z2v63lacyq4d4-novena-firmware.img` |
| foobox | Firmware | 134217728 | `3d78fc55dbdfddcab72a56c209b84d32810722986390dc3322780951e1e8e22f` | `/nix/store/w1z1d2m6aswq26dsbr42sc4hd101j1am-novena-firmware.img` |
| foobox | Firmware | 134217728 | `ac6d51233d05a4e7c01eff5860ac6b43d7b5ef0af449b4b1814f272a25849f12` | `/nix/store/wa1mbk3narx4m093553b3499a74bvnl5-novena-firmware.img` |
| foobox | Firmware | 134217728 | `90ea06f84d0241502fd71d352688192e43af3771955007802b4bb2bfb9376596` | `/nix/store/wr1825k6aqy1135chsz4bh20kwhj2z90-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2588422144 | `1c854d6a148bd212aca5ebe7e3705c84bc4f6b0e5ff62c1f4a211cf5b0fd3406` | `/nix/store/wzkx9cn7la3qpwqxp3lvjhlpjgsam24h-nixos-novena-sd-image.img` |
| foobox | NixOS-SD-Image | 2581291008 | `ff9ea88aa0d26db148e51751f32a3d4b5afc48dcc8c974957f0005ae6d569b8a` | `/nix/store/xgp0l80pcbaimirw5sqswnrg1qrci396-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `84aac9f7188df88dc9ea52991653b7bb63aa3f6a8f2a1778274b0f654b8da79d` | `/nix/store/xna5vh32i3gc1wb23dzy483yvsr5yqa7-novena-firmware.img` |
| foobox | Firmware | 134217728 | `5598a0b41c29d86fa3a8f338a018807f6d1cbec675b2f13799412bbca394cbf6` | `/nix/store/xqiryij1csajpsqr6jrjmhgc5nm1q5vk-novena-firmware.img` |
| foobox | Firmware | 134217728 | `c662f8c69522709b57f0c3acd2f1d9ca743d642d833d2adbfc445b3d3335f88f` | `/nix/store/xvffl1p23c7ich3yi73ps8pgg3b5i8qq-novena-firmware.img` |
| foobox | NixOS-SD-Image | 2581291008 | `8d08a28500e48407f0241041b0366f66ca941db4cb83ddd9cd2b5b4975412166` | `/nix/store/xzi7lcygiwn4g1bx2k501sdf4i6d1fcq-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `225218ccd9e42605b52186a322ea0acdd99fc484c219550117c81778527d6079` | `/nix/store/y2vlnkh2xx5kgj1dz2blqv10qcnqyw55-novena-firmware.img` |
| foobox | Firmware | 134217728 | `6c4ddd307352345df325d752c2487ca63a770e97ddf80a9e72756d6fcdde3200` | `/nix/store/y7d8plz04m7wjggmn0swriqb1pqq0cfq-novena-firmware.img` |
| foobox | SPL | 52224 | `b78fea3f115a097bada3109f75a5fe51889752cbd13035e56796df24abf85c8d` | `/nix/store/ycmza78cga4isg1q6pggrx7klas2nkh6-novena-imx6-spl.bin` |
| foobox | NixOS-SD-Image | 2588422144 | `f6002d0688cb3b81447943485ebcf4f8ff19facad52038fa7f0d2e6a1fbc8b43` | `/nix/store/ycwfjyfgjy6qymrgz21z8ns95a94n0zv-nixos-novena-sd-image.img` |
| foobox | Firmware | 134217728 | `832eb947825074cad3ff5bde89fdc60bb8620093ebe757da647eba1c8d6d590d` | `/nix/store/zqvddgy07scmjli4wcvlz4b2hfhxiwz0-novena-firmware.img` |

## Quellen

- [Inventur 96](https://github.com/loomit77/project-block-logs/commit/265fb17af95719d28186264f85f79d3e6c4c1625)
- [Dreirechnervergleich 56](https://github.com/loomit77/project-block-logs/commit/e67612e0f26bb8e4fc8203bc4ef4c7279c4b036f)
- [VM-Inventur 66](https://github.com/loomit77/project-block-logs/commit/f110373357050afa5f10ba7f6edd75abb524fa8d)
- [Hardwareprüfung BOOT-VERIFY-02](https://github.com/loomit77/project-block-logs/commit/7affc61a1333b76634edada533cc0d8d1ef66072)
- `docs/VM102-VM104-REPRODUCIBILITY-2026-10-24.md`
- `docs/UBOOT26-DISPLAY-QUALIFICATION-2026-10-25.md`

## Offene Punkte

- Exakten Hash des funktionierenden SD-Referenzstands zuordnen.
- Unzugänglichen Bereich `/mnt/novena-backup-recovery-ro` gesondert behandeln.
- Hardwaretest der neuen Displayvariante durchführen.
- Inventur in späteren Tagesabschluss integrieren.
