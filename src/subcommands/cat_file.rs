use std::io;

use anyhow::{Context, Result};

use super::{Kind, Object};

pub fn execute(pretty: bool, blob: String) -> Result<()> {
    anyhow::ensure!(
        pretty,
        "mode must be given without -p, and we dont support mode"
    );

    let mut object = Object::read(&blob).context("parse out blob object file")?;

    match object.kind {
        Kind::Blob => {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            let n = io::copy(&mut object.reader, &mut stdout)
                .context("write objects content to stdout")? as usize;

            anyhow::ensure!(
                n == object.size,
                ".git/object file was not the expected size (expected: {}, actual: {}",
                object.size,
                n
            );
        }
        _ => anyhow::bail!("expected a blob object"),
    };

    Ok(())
}
