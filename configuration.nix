{ config, pkgs, ... }:
{
  imports = [
    ./hardware/novena.nix
  ];

  nixpkgs.overlays = [
    (import ./overlays/reproducible-bash.nix)
    (import ./overlays/reproducible-boost.nix)
  ];

  boot.growPartition = true;

  hardware.graphics.enable = true;

  # Development image: diagnostic tools for Novena hardware tests.
  environment.systemPackages = with pkgs; [
    alsa-utils

    # Network and Ethernet diagnostics
    iproute2
    iputils
    ethtool

    # I2C, USB and PCI diagnostics
    i2c-tools
    usbutils
    pciutils

    # SATA storage diagnostics
    smartmontools

    # Process and file diagnostics
    strace
    lsof
  ];

  networking.hostName = "novena";

  services.openssh.enable = true;
  services.openssh.settings = {
    PasswordAuthentication = true;
    PermitRootLogin = "yes";
  };

  users.users.root.initialPassword = "nixos";

  system.stateVersion = "26.05";
}
