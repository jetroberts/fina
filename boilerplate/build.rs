use std::{error::Error, fs::read_dir};

fn main() -> Result<(), Box<dyn Error>> {
    // get all of the files in proto folder

    let dir = read_dir("proto")?;
    dir.for_each(|f| tonic_build::compile_protos(f.expect("Failed to read file").path()));

    Ok(())
}
