# Separate byte-preserving assembler; no NixOS module and no source downloads.
{ pkgs
, rootfs
, firmware
, swap
, bootloader2015
, kernel
, dtb
, layout
, bootContract ? null
}:
let
  lib = pkgs.lib;
  host = pkgs.buildPackages;
  assemblerRoot = ../tools/novena-image-assembler;
  assembler = host.rustPlatform.buildRustPackage {
    pname = "novena-image-assembler";
    version = "0.1.0";
    # Include only build sources, never local target/, caches or result links.
    src = lib.fileset.toSource {
      root = assemblerRoot;
      fileset = lib.fileset.unions [
        (assemblerRoot + "/Cargo.toml")
        (assemblerRoot + "/Cargo.lock")
        (assemblerRoot + "/src")
      ];
    };
    cargoLock.lockFile = assemblerRoot + "/Cargo.lock";
    cargoBuildFlags = [ "--frozen" ];
    # Tests are a separately authorized qualification, not part of this builder.
    doCheck = false;
    strictDeps = true;
  };
  reference = "bd5c7d3290bb94bf8af17760c321df67d2641343";
  requiredEvidence = [
    "pairProvenance" "romSplProperContract" "externalMmcEnvironment"
    "eepromAndHooks" "bootFilesAndArguments" "kernelDtbInitrdContract"
    "externalRootAndFstab" "partitionIdentityAndCollisions"
  ];
  nonempty = x: builtins.isString x && builtins.match ".*[^[:space:]].*" x != null;
  artifactValid = a:
    builtins.isAttrs a
    && a ? path && a ? origin && a ? size && a ? sha256
    && nonempty a.origin
    && builtins.isInt a.size && a.size > 0
    && builtins.isString a.sha256
    && builtins.match "[0-9a-f]{64}" a.sha256 != null
    && lib.hasPrefix (builtins.storeDir + "/") (toString a.path);
  pairValid = builtins.isAttrs bootloader2015
    && bootloader2015 ? sourceRevision
    && bootloader2015.sourceRevision == reference
    && bootloader2015 ? spl && bootloader2015 ? proper;
  contractValid = builtins.isAttrs bootContract
    && lib.all (key: builtins.hasAttr key bootContract
      && nonempty bootContract.${key}) requiredEvidence;
  inputs = {
    inherit rootfs firmware swap kernel dtb;
    spl = bootloader2015.spl;
    proper = bootloader2015.proper;
  };
  # Serialize paths explicitly while retaining their Nix string contexts.
  manifestInputs = lib.mapAttrs (_: a: a // { path = toString a.path; }) inputs;
in
assert lib.assertMsg (host.stdenv.hostPlatform.system == "x86_64-linux")
  "Debian reference: Rust assembler requires an x86_64-linux build host";
assert lib.assertMsg pairValid "Debian reference: explicit matched 2015 SPL/Proper pair required";
assert lib.assertMsg contractValid "Debian reference: unresolved boot prerequisites (bootContract evidence required)";
assert lib.assertMsg (lib.all artifactValid (builtins.attrValues inputs))
  "Debian reference: every input needs store path, origin, positive byte size and lowercase SHA256";
host.runCommand "novena-debian-reference-image" {
  strictDeps = true;
  manifest = builtins.toJSON {
    schemaVersion = 1;
    profile = "novena-debian-2015";
    sectorSize = 512;
    inputs = manifestInputs;
    inherit layout bootContract;
    sourceRevision = reference;
  };
  passAsFile = [ "manifest" ];
} ''
  set -eu
  # Rust invokes only blkid and mcopy; do not inherit any host search path.
  export PATH=${lib.escapeShellArg (lib.makeBinPath [ host.util-linux host.mtools ])}
  export LC_ALL=C
  ${assembler}/bin/novena-image-assembler "$manifestPath" "$out"
''
