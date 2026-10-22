# Novena development image: diagnostic tools

The development SD image provides convenient access for repeated
hardware testing and complete cross-build cycles.

## Diagnostic tools

The following tools are explicitly included in configuration.nix:

- alsa-utils
- iproute2
- iputils
- ethtool
- i2c-tools
- usbutils
- pciutils
- smartmontools
- strace
- lsof

All ten package attributes resolved to ARMv7 cross derivations during
NOVENA-DUAL-BUILD-04B. This does not establish build success.

## Development access

For development only, the current configuration retains:

- Root initial password: nixos
- SSH password authentication enabled
- Root SSH login enabled

These settings must be replaced with a secure first-access mechanism
before distributing a production or public image.

## Build requirements

- Complete image builds, not partial substitutes
- Independent cross-compilation on foobox and VM102
- Identical source and lock file
- Existing novenaImage target and historical evaluation module
- SHA256 comparison of the resulting images

The development image is intended for a trusted, isolated test network.
