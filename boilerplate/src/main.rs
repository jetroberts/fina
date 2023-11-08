use std::{error::Error, fs::read_dir};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // for all files in proto, run tonic_build::compile_proto

    let dir = read_dir("proto")?;

    dir.map_err(|f| {
        let p = f.expect("Failed to read file").path();
        tonic_build::compile_protos(p)?;
    });

    Ok(())
}
