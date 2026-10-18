use crate::{
    layout::{self, Plan, Range},
    manifest::{self, Artifact, Manifest},
    mbr, require, Result,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    path::Path,
    process::{Command, Stdio},
};

const BUFFER: usize = 64 * 1024;
// Linux x86_64 UAPI flags. O_PATH pins an inode without opening devices for I/O.
const O_NOFOLLOW: i32 = 0x20000;
const O_PATH: i32 = 0x200000;

pub fn open_regular(path: &Path) -> Result<File> {
    let pinned = OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW | O_PATH)
        .open(path)?;
    require(
        pinned.metadata()?.is_file(),
        "input is not a regular file (symlinks forbidden)",
    )?;
    // Reopen the pinned regular inode, not the replaceable original pathname.
    let file = File::open(format!("/proc/self/fd/{}", pinned.as_raw_fd()))?;
    require(
        file.metadata()?.is_file(),
        "opened input is not a regular file",
    )?;
    Ok(file)
}

pub fn read_manifest(path: &Path) -> Result<Manifest> {
    let mut file = open_regular(path)?;
    require(
        file.metadata()?.len() <= manifest::MAX_MANIFEST,
        "manifest exceeds 1 MiB",
    )?;
    let mut bytes = Vec::new();
    (&mut file)
        .take(manifest::MAX_MANIFEST + 1)
        .read_to_end(&mut bytes)?;
    manifest::parse(&bytes)
}

/// Bounded memory; exact byte count, then EOF. Used for source identity checks.
pub fn verify_input(file: &mut File, artifact: &Artifact) -> Result<()> {
    require(
        file.metadata()?.len() == artifact.size,
        "input size mismatch",
    )?;
    file.seek(SeekFrom::Start(0))?;
    let actual = hash_range(file, artifact.size)?;
    require(actual == artifact.sha256, "input SHA256 mismatch")?;
    let mut extra = [0];
    require(
        file.read(&mut extra)? == 0,
        "input grew during verification",
    )
}

fn hash_range(reader: &mut impl Read, mut remaining: u64) -> Result<String> {
    let mut hash = Sha256::new();
    let mut buffer = [0u8; BUFFER];
    while remaining != 0 {
        let n = usize::try_from(remaining.min(BUFFER as u64))?;
        reader.read_exact(&mut buffer[..n])?;
        hash.update(&buffer[..n]);
        remaining = remaining
            .checked_sub(n as u64)
            .ok_or("read counter underflow")?;
    }
    Ok(format!("{:x}", hash.finalize()))
}

struct Source<'a> {
    name: &'static str,
    artifact: &'a Artifact,
    file: File,
}

// Helpers see only the already-open read-only file via stdin, never a source
// pathname that could be replaced between preflight and helper invocation.
fn helper(
    file: &mut File,
    program: &str,
    args: &[&str],
    limit: u64,
) -> Result<(u64, String, Vec<u8>)> {
    file.seek(SeekFrom::Start(0))?;
    let search_path = std::env::var_os("PATH").ok_or("PATH required for blkid and mcopy")?;
    let mut child = Command::new(program)
        .args(args)
        .env_clear()
        .env("PATH", search_path)
        .env("LC_ALL", "C")
        .env("MTOOLSRC", "/dev/null")
        .stdin(Stdio::from(file.try_clone()?))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let result = (|| -> Result<(u64, String, Vec<u8>)> {
        let mut stdout = child.stdout.take().ok_or("helper stdout unavailable")?;
        let mut hash = Sha256::new();
        let mut prefix = Vec::new();
        let mut size = 0u64;
        let mut buffer = [0u8; BUFFER];
        loop {
            let n = stdout.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            size = size.checked_add(n as u64).ok_or("helper output overflow")?;
            require(size <= limit, "helper output exceeds declared size")?;
            hash.update(&buffer[..n]);
            // Only blkid needs textual output; never accumulate FAT file data.
            if program == "blkid" {
                prefix.extend_from_slice(&buffer[..n]);
            }
        }
        require(child.wait()?.success(), "read-only helper failed")?;
        Ok((size, format!("{:x}", hash.finalize()), prefix))
    })();
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result
}

fn preflight_filesystems(sources: &mut [Source<'_>], m: &Manifest) -> Result<()> {
    for (name, expected) in [("firmware", "vfat"), ("swap", "swap"), ("rootfs", "ext4")] {
        let source = sources
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or("missing input")?;
        let (_, _, output) = helper(
            &mut source.file,
            "blkid",
            &["-p", "-s", "TYPE", "-o", "value", "/proc/self/fd/0"],
            1024,
        )?;
        require(
            std::str::from_utf8(&output)?.trim() == expected,
            "filesystem type mismatch",
        )?;
    }
    let fat = sources
        .iter_mut()
        .find(|s| s.name == "firmware")
        .ok_or("missing FAT")?;
    for (path, expected) in [
        ("::/u-boot.img", &m.inputs.proper),
        ("::/zImage", &m.inputs.kernel),
        ("::/novena.dtb", &m.inputs.dtb),
    ] {
        let (size, hash, _) = helper(
            &mut fat.file,
            "mcopy",
            &["-i", "/proc/self/fd/0", path, "-"],
            expected.size,
        )?;
        require(
            size == expected.size && hash == expected.sha256,
            "FAT boot file identity mismatch",
        )?;
    }
    // Also detect mutation while the filesystem tools were reading.
    for source in sources {
        verify_input(&mut source.file, source.artifact)?;
    }
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputIdentity {
    pub size: u64,
    pub sha256: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Report<'a> {
    result_schema_version: u32,
    assembler_version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualification: Option<&'static str>,
    manifest: &'a Manifest,
    output: &'a OutputIdentity,
}
pub fn report_bytes(m: &Manifest, identity: &OutputIdentity) -> Result<Vec<u8>> {
    m.validate()?;
    let mut bytes = serde_json::to_vec_pretty(&Report {
        result_schema_version: if m.is_experimental() { 2 } else { 1 },
        assembler_version: env!("CARGO_PKG_VERSION"),
        qualification: m
            .is_experimental()
            .then_some("experimental-unqualified-no-boot-evidence"),
        manifest: m,
        output: identity,
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn hash_zeros(hash: &mut Sha256, mut size: u64) -> Result<()> {
    let zeros = [0u8; BUFFER];
    while size > 0 {
        let n = usize::try_from(size.min(BUFFER as u64))?;
        hash.update(&zeros[..n]);
        size = size.checked_sub(n as u64).ok_or("zero counter underflow")?;
    }
    Ok(())
}

fn write_image(
    target: &mut File,
    plan: &Plan,
    sources: &mut [Source<'_>],
) -> Result<OutputIdentity> {
    let mbr = mbr::encode(plan)?;
    target.set_len(plan.total_size)?;
    target.seek(SeekFrom::Start(0))?;
    target.write_all(&mbr)?;
    let mut whole = Sha256::new();
    whole.update(mbr);
    let mut position = 512u64;
    for (name, range) in [
        ("spl", plan.spl),
        ("firmware", plan.partitions[0]),
        ("swap", plan.partitions[1]),
        ("rootfs", plan.partitions[2]),
    ] {
        hash_zeros(
            &mut whole,
            range.offset.checked_sub(position).ok_or("output overlap")?,
        )?;
        target.seek(SeekFrom::Start(range.offset))?;
        let source = sources
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or("missing source")?;
        source.file.seek(SeekFrom::Start(0))?;
        let mut remaining = range.size;
        let mut copied = Sha256::new();
        let mut buffer = [0u8; BUFFER];
        while remaining > 0 {
            let n = usize::try_from(remaining.min(BUFFER as u64))?;
            source.file.read_exact(&mut buffer[..n])?;
            target.write_all(&buffer[..n])?;
            copied.update(&buffer[..n]);
            whole.update(&buffer[..n]);
            remaining = remaining
                .checked_sub(n as u64)
                .ok_or("copy counter underflow")?;
        }
        require(
            format!("{:x}", copied.finalize()) == source.artifact.sha256,
            "input changed during copy",
        )?;
        require(
            source.file.metadata()?.len() == range.size,
            "input size changed during copy",
        )?;
        position = range.end()?;
    }
    require(position == plan.total_size, "unexpected final image extent")?;
    target.sync_all()?;
    let expected_hash = format!("{:x}", whole.finalize());
    verify_output(target, plan, sources, &expected_hash)?;
    Ok(OutputIdentity {
        size: plan.total_size,
        sha256: expected_hash,
    })
}

fn verify_output(
    target: &mut File,
    plan: &Plan,
    sources: &[Source<'_>],
    expected_hash: &str,
) -> Result<()> {
    require(
        target.metadata()?.len() == plan.total_size,
        "output size mismatch",
    )?;
    target.seek(SeekFrom::Start(0))?;
    let mut actual = [0u8; 512];
    target.read_exact(&mut actual)?;
    require(actual == mbr::encode(plan)?, "MBR readback mismatch")?;
    for (name, Range { offset, size }) in [
        ("spl", plan.spl),
        ("firmware", plan.partitions[0]),
        ("swap", plan.partitions[1]),
        ("rootfs", plan.partitions[2]),
    ] {
        let source = sources
            .iter()
            .find(|s| s.name == name)
            .ok_or("missing source")?;
        target.seek(SeekFrom::Start(offset))?;
        require(
            hash_range(target, size)? == source.artifact.sha256,
            "target range SHA256 mismatch",
        )?;
    }
    target.seek(SeekFrom::Start(0))?;
    require(
        hash_range(target, plan.total_size)? == expected_hash,
        "whole image SHA256 mismatch",
    )
}

/// OUTPUT_DIR must not exist. Errors preserve partial output, never a success report.
pub fn assemble(m: &Manifest, output_dir: &Path) -> Result<OutputIdentity> {
    m.validate()?;
    let plan = layout::plan(m)?;
    let mut sources = Vec::new();
    for (name, artifact) in m.inputs.entries() {
        let mut file = open_regular(Path::new(&artifact.path))?;
        verify_input(&mut file, artifact)?;
        sources.push(Source {
            name,
            artifact,
            file,
        });
    }
    preflight_filesystems(&mut sources, m)?;
    // No output directory or image before all preflight checks pass.
    fs::create_dir(output_dir)?;
    let mut target = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(output_dir.join(m.image_name()))?;
    let identity = write_image(&mut target, &plan, &mut sources)?;
    // Recheck ALL standalone artifacts as well, including those only compared on FAT.
    for source in &mut sources {
        verify_input(&mut source.file, source.artifact)?;
    }
    let mut report = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_dir.join("manifest.json"))?;
    report.write_all(&report_bytes(m, &identity)?)?;
    report.sync_all()?;
    File::open(output_dir)?.sync_all()?;
    Ok(identity)
}

#[cfg(test)]
mod tests;
