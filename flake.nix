{
  description = "Reproducible NixOS image for Kosagi Novena";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs = { self, nixpkgs }:
    {
      nixosConfigurations.novena = nixpkgs.lib.nixosSystem {
        modules = [
          ./configuration.nix
          ./image/novena-image.nix
        ];
      };
    };
}
