{ config, lib, pkgs, ... }:

let
  historical = config.novena.image.bootloader == "historical-2015";

  system = config.system.build.toplevel;

  kernel = config.system.build.kernel;

  zImage = "${kernel}/zImage";

  deviceTree = config.hardware.deviceTree.package;

  novenaDtb = pkgs.runCommand "imx6q-novena.dtb" { } ''
    cp ${deviceTree}/imx6q-novena.dtb $out
  '';

  initrd = "${config.system.build.initialRamdisk}/initrd";

  # Select one complete pair; never select SPL and Proper independently.
  novenaUboot =
    if historical then
      import ../boot/u-boot/historical-2015.nix { inherit pkgs; }
    else
      import ../boot/u-boot { inherit pkgs; };
  properFilename = if historical then "u-boot.img" else "u-boot-dtb.img";
  novenaSpl = "${novenaUboot}/SPL";
  novenaUbootImage = "${novenaUboot}/${properFilename}";

  bootCmd = pkgs.writeText "boot.cmd" (if historical then ''
    echo NOVENA_NIXOS_HISTORICAL_2015_EXPERIMENTAL
    setenv bootargs 'systemConfig=${system} init=${system}/init loglevel=7 console=ttymxc1,115200 console=tty0'
    if mmc dev 1 && mmc rescan; then
      if fatload mmc 1:1 0x12000000 zImage; then
        if fatload mmc 1:1 0x20000000 initrd.uimg; then
          if fatload mmc 1:1 0x1f000000 novena.dtb; then
            bootz 0x12000000 0x20000000 0x1f000000
          fi
        fi
      fi
    fi
    echo NOVENA_NIXOS_BOOT_FAILED
    while test 1 = 1; do
      sleep 1
    done
  '' else ''
    setenv bootargs 'systemConfig=${system} init=${system}/init loglevel=7 console=ttymxc1,115200 console=tty0'
    fatload mmc 1:1 0x12000000 zImage
    fatload mmc 1:1 0x20000000 initrd.uimg
    fatload mmc 1:1 0x1f000000 novena.dtb
    bootz 0x12000000 0x20000000 0x1f000000
  '');

  bootScr = pkgs.runCommand "boot.scr"
    {
      nativeBuildInputs = [
        pkgs.buildPackages.ubootTools
      ];
    }
    ''
      mkimage \
        -A arm \
        -O linux \
        -T script \
        -C none \
        -n NixOS \
        -d ${bootCmd} \
        $out
    '';

  initrdUimg = pkgs.runCommand "initrd.uimg"
    {
      nativeBuildInputs = [
        pkgs.buildPackages.ubootTools
      ];
    }
    ''
      mkimage \
        -A arm \
        -O linux \
        -T ramdisk \
        -C gzip \
        -n initrd \
        -d ${initrd} \
        $out
    '';

  rootfsImage = pkgs.callPackage ./make-ext4-fs-reproducible.nix {
    storePaths = [
      config.system.build.toplevel
    ];

    compressImage = false;

    populateImageCommands = ''
      mkdir -p files/etc
      mkdir -p files/boot/firmware

      ln -s ${config.system.build.toplevel}/etc files/etc/static
    '';

    volumeLabel = "NIXOS_SD";
    uuid = "44444444-4444-4444-8888-888888888888";
    hashSeed = "44444444-4444-4444-8888-888888888888";
  };

  firmwareImage = pkgs.runCommand "novena-firmware.img"
    {
      nativeBuildInputs = [
        pkgs.buildPackages.dosfstools
        pkgs.buildPackages.mtools
      ];
    }
    ''
      truncate -s 128M $out

      mkfs.vfat \
        -F 32 \
        -n FIRMWARE \
        -i 2178694E \
        $out

      mcopy -i $out ${novenaUbootImage} ::${properFilename}
      mcopy -i $out ${zImage} ::zImage
      mcopy -i $out ${initrdUimg} ::initrd.uimg
      mcopy -i $out ${novenaDtb} ::novena.dtb
      mcopy -i $out ${bootCmd} ::boot.cmd
      mcopy -i $out ${bootScr} ::boot.scr
    '';

  novenaImage = pkgs.runCommand "nixos-novena-sd-image.img"
    {
      nativeBuildInputs = [
        pkgs.buildPackages.util-linux
        pkgs.buildPackages.coreutils
      ];
    }
    ''
      # Original Novena layout:
      #
      # sector 0       MBR
      # byte 0x400     i.MX6 IVT/SPL
      # sector 16384   FIRMWARE
      #
      # FIRMWARE is enlarged from 30 MiB to 128 MiB because
      # the current kernel + initrd no longer fit in 30 MiB.

      firmwareStart=16384
      firmwareSectors=262144
      rootStart=278528

      splBytes=$(stat -c %s ${novenaSpl})
      test "$splBytes" -gt 0
      test $((1024 + splBytes)) -le $((firmwareStart * 512))

      rootBytes=$(stat -c %s ${rootfsImage})
      rootSectors=$(( (rootBytes + 511) / 512 ))

      # Leave 1 MiB unused space after the root partition.
      diskSectors=$(( rootStart + rootSectors + 2048 ))
      diskBytes=$(( diskSectors * 512 ))

      truncate -s "$diskBytes" $out

      cat > partition-table.sfdisk <<EOF
label: dos
label-id: 0x2178694e
unit: sectors
sector-size: 512

1 : start=$firmwareStart, size=$firmwareSectors, type=c, bootable
2 : start=$rootStart, size=$rootSectors, type=83
EOF

      sfdisk $out < partition-table.sfdisk

      # Raw Novena i.MX6 IVT/SPL at byte offset 0x400 = 1024.
      dd \
        if=${novenaSpl} \
        of=$out \
        bs=1 \
        seek=1024 \
        conv=notrunc \
        status=none

      # FAT FIRMWARE partition at sector 16384 = 8 MiB.
      dd \
        if=${firmwareImage} \
        of=$out \
        bs=512 \
        seek=$firmwareStart \
        conv=notrunc \
        status=none

      # NixOS ext4 root filesystem.
      dd \
        if=${rootfsImage} \
        of=$out \
        bs=512 \
        seek=$rootStart \
        conv=notrunc \
        status=none
    '';

in
{
  options.novena.image.bootloader = lib.mkOption {
    type = lib.types.enum [ "current-2026" "historical-2015" ];
    default = "current-2026";
    description = ''
      Complete SPL/Proper pair for the Novena image.
      historical-2015 is experimental and not NixOS-boot-qualified.
    '';
  };

  config = {
    system.build.novenaBootCmd = bootCmd;
    system.build.novenaBootScr = bootScr;
    system.build.novenaInitrdUimg = initrdUimg;

    system.build.novenaKernel = zImage;
    system.build.novenaDtb = novenaDtb;

    system.build.novenaRootfs = rootfsImage;
    system.build.novenaFirmware = firmwareImage;

    system.build.novenaImage = novenaImage;
  };
}
