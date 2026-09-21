{ pkgs }:

let
  version = "2026.07";
  sourceDateEpoch = "1789984363";

  src = pkgs.fetchFromGitHub {
    owner = "u-boot";
    repo = "u-boot";
    rev = "ece349ade2973e220f524ce59e59711cc919263f";
    hash = "sha256-MJf4+MifD7Bzb2Fz11lIgFOaiAPgSbFnbM0Q3k5xWwQ=";
  };
in
pkgs.buildUBoot {
  inherit version src;

  defconfig = "novena_defconfig";

  extraPatches = [
    ./0001-novena-spl-boot-second-stage-from-external-sd.patch
    ./0002-novena-boot-external-sd-by-default.patch
    ./0003-novena-use-non-persistent-default-environment.patch
  ];

  extraConfig = ''
    CONFIG_LOCALVERSION="-00003-gf8baca04e22d"
    # CONFIG_LOCALVERSION_AUTO is not set
  '';

  SOURCE_DATE_EPOCH = sourceDateEpoch;

  enableParallelBuilding = false;

  filesToInstall = [
    "SPL"
    "u-boot-dtb.img"
  ];

  extraMeta.platforms = [ "armv7l-linux" ];
}
