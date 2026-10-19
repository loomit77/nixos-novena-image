{ pkgs, ... }:
let
  growRoot = pkgs.writeText "novena-grow-root.py" ''
    import fcntl
    import json
    import os
    from pathlib import Path
    import stat
    import struct
    import subprocess
    import sys
    import tempfile

    # Python integers and file offsets remain wide on ARMv7.
    def require(condition, message):
        if not condition:
            raise RuntimeError(message)

    def run(*args, data=None):
        return subprocess.check_output(args, input=data, text=True, timeout=180)

    def number(path):
        return int(path.read_text().strip())

    def root_device():
        mount = json.loads(run("${pkgs.util-linux}/bin/findmnt", "--json",
                               "--mountpoint", "/", "--output",
                               "MAJ:MIN,FSTYPE,OPTIONS"))["filesystems"]
        require(len(mount) == 1 and mount[0]["fstype"] == "ext4",
                "Root is not a single ext4 mount")
        require("rw" in mount[0]["options"].split(","), "Root is read-only")
        return mount[0]["maj:min"]

    def main():
        identity = root_device()
        part = Path("/sys/dev/block", identity).resolve(strict=True)
        require(number(part / "partition") == 2, "Root is not partition 2")
        disk = part.parent
        # Physical disk only: excludes loop, device mapper, MD and stacked devices.
        require((disk / "device").exists() and not (disk / "partition").exists(),
                "Root parent is not a physical disk")
        for node in (part, disk):
            slaves = node / "slaves"
            require(not slaves.exists() or not any(slaves.iterdir()),
                    "Stacked device rejected")
        require(number(disk / "queue/logical_block_size") == 512,
                "Logical sector size is not 512")
        require(number(disk / "ro") == 0 and number(part / "ro") == 0,
                "Block device is read-only")
        require(number(part / "start") == 278528, "Unexpected kernel root start")
        root = "/dev/" + part.name
        device = "/dev/" + disk.name
        root_stat = os.stat(root)
        require(stat.S_ISBLK(root_stat.st_mode) and
                f"{os.major(root_stat.st_rdev)}:{os.minor(root_stat.st_rdev)}" == identity,
                "Root device identity mismatch")
        require(os.stat("/dev/disk/by-label/NIXOS_SD").st_rdev == root_stat.st_rdev,
                "Root label path does not identify mounted root")
        capacity = number(disk / "size")
        require(278528 < capacity <= 2**32, "Capacity outside supported DOS range")

        def filesystem():
            with open(root, "rb", buffering=0) as handle:
                sb = os.pread(handle.fileno(), 1024, 1024)
            require(len(sb) == 1024 and sb[56:58] == b"\x53\xef",
                    "Invalid ext4 superblock")
            require(sb[120:136].rstrip(b"\0") == b"NIXOS_SD", "Wrong root label")
            require(not (struct.unpack_from("<H", sb, 58)[0] & 2),
                    "Filesystem reports errors; no automatic repair")
            log_size = struct.unpack_from("<I", sb, 24)[0]
            require(log_size <= 6, "Unsupported filesystem block size")
            block_size = 1024 << log_size
            blocks = struct.unpack_from("<I", sb, 4)[0]
            if struct.unpack_from("<I", sb, 96)[0] & 0x80:
                blocks |= struct.unpack_from("<I", sb, 336)[0] << 32
            require(blocks > 0, "Empty filesystem")
            return blocks, block_size

        disk_id = os.stat(device).st_rdev
        require(stat.S_ISBLK(os.stat(device).st_mode) and
                (disk / "dev").read_text().strip() ==
                f"{os.major(disk_id)}:{os.minor(disk_id)}", "Disk identity mismatch")
        # Read-only until all checks and a real growth opportunity are established.
        with open(device, "rb", buffering=0) as handle:
            fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
            table = json.loads(run("${pkgs.util-linux}/bin/sfdisk", "--json",
                                   device))["partitiontable"]
            require(table["label"] == "dos" and table["sectorsize"] == 512,
                    "Partition table is not DOS with 512-byte sectors")
            original = os.pread(handle.fileno(), 512, 0)
            require(len(original) == 512 and original[510:] == b"\x55\xaa",
                    "Invalid MBR signature")
            require(original[440:444] == struct.pack("<I", 0x2178694e),
                    "Wrong DOS disk ID")
            entries = [original[446+i*16:462+i*16] for i in range(4)]
            require(entries[0][4] == 0x0c and
                    struct.unpack_from("<II", entries[0], 8) == (16384, 262144),
                    "Unexpected firmware partition")
            start, length = struct.unpack_from("<II", entries[1], 8)
            require(start == 278528 and entries[1][4] == 0x83 and length > 0,
                    "Unexpected root partition")
            require(all(entry == bytes(16) for entry in entries[2:]),
                    "Partition 3 or 4 is present")
            require(all(entry[0] in (0, 0x80) for entry in entries[:2]),
                    "Invalid boot flags")
            require(start + length <= capacity, "Partition exceeds disk capacity")
            kernel_length = number(part / "size")
            require(0 < kernel_length <= length, "Unexpected kernel partition size")
            blocks, block_size = filesystem()
            require(blocks * block_size <= kernel_length * 512,
                    "Filesystem exceeds kernel partition size")
            # Consume every whole sector, including small tails; ext4 uses whole blocks.
            target = capacity - start
            if length < target:
                # Let sfdisk prepare a DOS entry on a sparse RAM-backed disk copy.
                # Only its verified size field enters the candidate: sfdisk may
                # recalculate CHS bytes, which must remain untouched on Novena.
                # A real pathname avoids /proc access restrictions and dependence
                # on descriptor inheritance or sfdisk's canonicalization of fd paths.
                # The private runtime directory and exclusive creation keep this a
                # regular scratch file; no block device is passed to this command.
                with tempfile.NamedTemporaryFile(
                        dir="/run/novena-grow-root", suffix=".img",
                        buffering=0) as staging:
                    require(stat.S_ISREG(os.fstat(staging.fileno()).st_mode),
                            "Staging target is not a regular file")
                    require(staging.truncate(capacity * 512) == capacity * 512,
                            "Unexpected staging truncate result")
                    require(staging.write(original) == 512, "Short staging MBR write")
                    # truncate creates holes, not a capacity-sized RAM allocation.
                    before = os.fstat(staging.fileno())
                    require(before.st_size == capacity * 512 and
                            os.stat(staging.name).st_ino == before.st_ino,
                            "Staging file size or identity mismatch")
                    run("${pkgs.util-linux}/bin/sfdisk", "--no-reread",
                        "--no-tell-kernel", "--wipe", "never", "--wipe-partitions",
                        "never", "-N", "2", staging.name,
                        data=f"start={start}, size={target}\n")
                    # Nonzero exit, timeout and I/O errors propagate without using
                    # a partial result. Exit zero alone does not validate the MBR.
                    after = os.stat(staging.name, follow_symlinks=False)
                    require(stat.S_ISREG(after.st_mode) and
                            (after.st_dev, after.st_ino, after.st_size) ==
                            (before.st_dev, before.st_ino, capacity * 512),
                            "sfdisk changed staging file identity or size")
                    prepared = os.pread(staging.fileno(), 512, 0)
                require(len(prepared) == 512, "Short prepared MBR read")
                # Only p2 CHS fields and length may differ in the scratch MBR.
                require(prepared[:463] == original[:463] and
                        prepared[466:467] == original[466:467] and
                        prepared[470:474] == original[470:474] and
                        prepared[478:] == original[478:] and
                        struct.unpack_from("<II", prepared, 470) == (start, target),
                        "sfdisk produced unexpected MBR changes")
                updated = original[:474] + prepared[474:478] + original[478:]
                require(length < target <= 0xffffffff and start + target == capacity and
                        updated[:474] == original[:474] and
                        updated[478:] == original[478:], "Invalid growth candidate")
                # NOVENA-NIXOS-113: stop before any writable device open. A four-byte
                # pwrite can still cause sector read/modify/write; neither that nor
                # a full-sector replacement has been practically qualified here.
                # fsync/readback cannot guarantee survival of an SD power loss.
                # Do not enable a speculative writer or attempt automatic rollback.
                raise RuntimeError("MBR growth disabled pending write-path qualification; "
                                   "verified scratch candidate only, disk unchanged")
            # A previously completed partition growth can still resume here on a
            # normal boot. A damaged/partial MBR is not automatically repaired.
            if kernel_length != length:
                run("${pkgs.util-linux}/bin/partx", "--update", "--nr", "2", device)
            require(number(part / "size") == length and number(part / "start") == start,
                    "Kernel has not adopted partition size; retry on next normal boot")
            require(root_device() == identity, "Root mount changed")
            blocks, block_size = filesystem()
            desired_blocks = length * 512 // block_size
            require(blocks <= desired_blocks, "Refusing filesystem shrink")
            if blocks < desired_blocks:
                run("${pkgs.e2fsprogs}/bin/resize2fs", root, str(desired_blocks))
                require(filesystem() == (desired_blocks, block_size),
                        "Filesystem size verification failed")
                print(f"Root filesystem increased to {desired_blocks} blocks", flush=True)
            else:
                print("Root already uses all available whole filesystem blocks", flush=True)

    try:
        main()
    except Exception as error:
        print(f"novena-grow-root: {error}; no repair, rollback or reboot attempted",
              file=sys.stderr, flush=True)
        sys.exit(1)
  '';
in
{
  systemd.services.novena-grow-root = {
    description = "Grow verified Novena SD root partition and ext4 filesystem";
    wantedBy = [ "multi-user.target" ];
    after = [ "local-fs.target" ];
    # No Before/Requires relationship with SSH or any network target.
    serviceConfig = {
      Type = "oneshot";
      ExecStart = "${pkgs.python3}/bin/python3 ${growRoot}";
      Environment = [ "LC_ALL=C" "PYTHONDONTWRITEBYTECODE=1" ];
      RuntimeDirectory = "novena-grow-root";
      RuntimeDirectoryMode = "0700";
      TimeoutStartSec = "10min";
      Restart = "no";
      UMask = "0077";
      StandardOutput = "journal";
      StandardError = "journal";
    };
  };
}
