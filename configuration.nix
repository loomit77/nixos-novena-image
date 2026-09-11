{ config, pkgs, ... }:

{
  imports = [
    ./hardware/novena.nix
  ];

  networking.hostName = "novena";

  services.openssh.enable = true;

  users.users.root.initialPassword = "nixos";

  system.stateVersion = "26.05";
}
