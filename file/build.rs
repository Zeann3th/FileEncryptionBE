use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("file_descriptor.bin"))
        .compile_protos(&["proto/file.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/file.proto")?;

    Ok(())
}
