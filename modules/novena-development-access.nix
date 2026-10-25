# Development-only. Do not import into release builds.
# Example invocation:
#   imports = [ ./modules/novena-development-access.nix ];
#   novena.development.sshAuthorizedPublicKey =
#     builtins.readFile /path/to/public-key;
#
# Never place a private key in a Nix expression, repository or Nix store.
{ config, lib, ... }:

let
  cfg = config.novena.development;
in
{
  options.novena.development.sshAuthorizedPublicKey =
    lib.mkOption {
      type = lib.types.str;
      default = "";
      description = ''
        Development-only public SSH client key for root access.
        Must be absent from release images.
      '';
    };

  config = lib.mkIf (cfg.sshAuthorizedPublicKey != "") {
    services.openssh.enable = true;

    users.users.root.openssh.authorizedKeys.keys = [
      cfg.sshAuthorizedPublicKey
    ];
  };
}
