use novena_image_assembler::{
    assemble::{assemble, read_manifest},
    Result,
};
use std::{env, path::Path};

fn run() -> Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 3 {
        return Err("usage: novena-image-assembler MANIFEST OUTPUT_DIR".into());
    }
    let manifest = read_manifest(Path::new(&args[1]))?;
    let identity = assemble(&manifest, Path::new(&args[2]))?;
    println!("size={} sha256={}", identity.size, identity.sha256);
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("novena-image-assembler: {error}");
        std::process::exit(1);
    }
}
