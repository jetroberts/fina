use std::{error::Error, fs::read_dir, io};

fn main() -> Result<(), Box<dyn Error>> {
    // get all of the files in proto folder

    let dir = match read_dir("proto") {
        Ok(d) => d,
        Err(e) => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Unable to read proto dir, most likely the folder is missing, Error: {}",
                    e
                ),
            )))
        }
    };

    for path in dir {
        let path_res = path?;
        let path_name = path_res.path();
        tonic_build::compile_protos(
            path_name
                .to_str()
                .expect("Unable to convert path name to str"),
        )?;
    }

    Ok(())
}
