use anyhow::{Context, Result};

use std::{io, path::PathBuf};

use super::Object;

pub fn execute(write: bool, file: &PathBuf) -> Result<()> {
    let mut object = Object::blob_from_file(file)
        .with_context(|| format!("creating object from {}", file.display()))?;
    let hash = if write {
        let hash = object
            .write_to_objects()
            .context("write object to .git/objects")?;

        hex::encode(hash)
    } else {
        let hash = object.write(io::sink()).context("writing to sink")?;

        hex::encode(hash)
    };

    println!("{hash}");

    Ok(())
}
