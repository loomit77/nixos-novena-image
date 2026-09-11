{ config, lib, pkgs, ... }:

{
  nixpkgs.buildPlatform = "x86_64-linux";
  nixpkgs.hostPlatform = "armv7l-linux";

  fileSystems."/" = {
    device = "/dev/disk/by-label/NIXOS_SD";
    fsType = "ext4";
  };

  fileSystems."/boot/firmware" = {
    device = "/dev/disk/by-label/FIRMWARE";
    fsType = "vfat";
    options = [
      "nofail"
      "noauto"
    ];
  };

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = false;

  boot.initrd.compressor = "gzip";

  boot.kernelParams = [
    "console=tty0"
    "loglevel=7"
  ];

  boot.kernelPatches = [
    {
      name = "novena-it6251";
      patch = ../kernel/0001-drm-bridge-it6251.patch;

      extraConfig = ''
        DRM_IT6251 y
        PWM_IMX27 y
      '';
    }

    {
      name = "novena-innolux-n133hse-ea1";
      patch = ../kernel/0002-drm-panel-add-innolux-n133hse-ea1.patch;
    }

    {
      name = "novena-i2c-imx-debug-arbitration-lost";
      patch = ../kernel/0003-i2c-imx-debug-arbitration-lost.patch;
    }
  ];

  hardware.deviceTree.overlays = [
    {
      name = "novena-sd-disable-wp";

      dtsText = ''
        /dts-v1/;
        /plugin/;

        / {
          compatible = "kosagi,imx6q-novena";
        };

        &usdhc2 {
          disable-wp;
        };
      '';
    }

    {
      name = "novena-display";

      dtsText = ''
        /dts-v1/;
        /plugin/;

        / {
          compatible = "kosagi,imx6q-novena";
        };

        &clks {
          assigned-clocks =
            <&clks 33>,
            <&clks 34>;

          assigned-clock-parents =
            <&clks 6>,
            <&clks 6>;
        };

        &iomuxc {
          pinctrl_lvdsbridge_novena: lvdsbridge-novenagrp {
            fsl,pins = <
              0x280 0x650 0x00 0x05 0x00 0x1b0b1
            >;
          };

          pinctrl_backlight_novena: backlight-novenagrp {
            fsl,pins = <
              0x190 0x4a4 0x00 0x02 0x00 0x1b0b0
              0x21c 0x5ec 0x00 0x05 0x00 0x1b0b1
            >;
          };
        };

        &reg_display {
          startup-delay-us = <2000000>;
        };

        &reg_lvds_lcd {
        };

        &i2c3 {
          single-master;

          it6251: it6251@5c {
            compatible = "it,it6251";
            reg = <0x5c>;

            power-supply = <&reg_display>;

            pinctrl-names = "default";
            pinctrl-0 = <&pinctrl_lvdsbridge_novena>;

            ports {
              #address-cells = <1>;
              #size-cells = <0>;

              port@0 {
                reg = <0>;

                it6251_panel: endpoint {
                  remote-endpoint = <&panel_it6251>;
                };
              };

              port@1 {
                reg = <1>;

                it6251_ldb: endpoint {
                  remote-endpoint = <&ldb_it6251>;
                };
              };
            };
          };
        };

        &panel {
          port {
            panel_it6251: endpoint {
              remote-endpoint = <&it6251_panel>;
            };
          };
        };

        &ldb {
          lvds-channel@0 {
            port@4 {
              reg = <4>;

              ldb_it6251: endpoint {
                remote-endpoint = <&it6251_ldb>;
              };
            };
          };
        };
      '';
    }
  ];
}
