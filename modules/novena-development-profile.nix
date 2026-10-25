# Novena development-only SSH profile.
#
# Explicit opt-in only.
# Never import this file into public release images.
#
# The private SSH key remains on foobox.
# The public key is stored separately.

{ ... }:

{
  imports = [
    ./novena-development-access.nix
  ];

  novena.development.sshAuthorizedPublicKey =
    builtins.readFile ./novena-development-foobox.pub;
}
