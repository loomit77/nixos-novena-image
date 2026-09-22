final: prev:

let
  isCross =
    prev.stdenv.hostPlatform != prev.stdenv.buildPlatform;

  deterministicCrossBash =
    bash:
    bash.overrideAttrs (old: {
      patches = (old.patches or [ ]) ++ [
        ../patches/bash/0001-bash-cross-build-use-deterministic-pipe-size.patch
      ];

      makeFlags = (old.makeFlags or [ ]) ++ [
        "NIX_CROSS_PIPESIZE=4096"
      ];
    });
in
{
  # In this nixpkgs revision bashInteractive aliases bash.
  # Keep native Bash unchanged and apply the deterministic pipe-size
  # override only when Bash is cross-compiled.
  bash =
    if isCross then
      deterministicCrossBash prev.bash
    else
      prev.bash;

  # The non-interactive variant is a separate derivation.
  bashNonInteractive =
    if isCross then
      deterministicCrossBash prev.bashNonInteractive
    else
      prev.bashNonInteractive;
}
