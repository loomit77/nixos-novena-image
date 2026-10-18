//! Byte-preserving Linux host assembler. No bootability qualification.
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
compile_error!("Linux x86_64 host required for the guarded-open UAPI constants");

pub mod assemble;
pub mod layout;
pub mod manifest;
pub mod mbr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
