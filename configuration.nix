{ config, pkgs, ... }:
{
  imports = [ ./hardware/novena.nix ];

  nixpkgs.overlays = [
    (import ./overlays/reproducible-bash.nix)
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
