{ pkgs }:

let
  host = pkgs.buildPackages;
  spl = ./historical-2015/SPL;
  proper = ./historical-2015/u-boot.img;
in
# Experimental binary import, not a reconstruction of the original build.
host.runCommand "novena-uboot-2015-bd5c7d3-pair" {
  nativeBuildInputs = [ host.coreutils ];
  strictDeps = true;
} ''
  set -eu

  test "$(stat -c %s ${spl})" = 44032
  test "$(stat -c %s ${proper})" = 302172
  printf '%s  %s\n' \
    e2f2b80692c3ca55d29bce39fc98efd44c09046959acda46c50675ef3728e276 \
    ${spl} \
    7eba2bbdcf48f19f35a495993e5fa6625dc0db305c3e11c7a978b98a0260748a \
    ${proper} |
    sha256sum --check --strict

  mkdir -p "$out"
  cp ${spl} "$out/SPL"
  cp ${proper} "$out/u-boot.img"
  cp ${./historical-2015/PROVENANCE.md} "$out/PROVENANCE.md"
''
