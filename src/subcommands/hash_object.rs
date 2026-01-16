use anyhow::{Context, Result};
use codecrafters_git::HashWriter;
use flate2::{Compression, write::ZlibEncoder};
use sha1::Digest;

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub fn execute(write: bool, file: &Path) -> Result<String> {
    let hash = if write {
        let tmp = format!("{}.tmp", file.display());
        let writer = fs::File::create(&tmp)?;
        let hash = write_blob(&file, writer).context("write out blob object")?;

        fs::create_dir_all(format!(".git/objects/{}", &hash[..2]))
            .context("create subdirectory to .git/objects")?;
        fs::rename(&tmp, format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("move blob file into .git/objects")?;

        hash
    } else {
        write_blob(&file, io::sink())?
    };

    Ok(hash)
}

fn write_blob<W>(file: &Path, writer: W) -> Result<String>
where
    W: Write,
{
    let stat = fs::metadata(file).with_context(|| format!("stat {}", file.display()))?;
    let writer = ZlibEncoder::new(writer, Compression::default());
    let mut writer = HashWriter::new(writer);

    write!(writer, "blob")?;
    write!(writer, "{}\0", stat.len())?;

    let mut file = fs::File::open(&file).with_context(|| format!("open {}", file.display()))?;

    io::copy(&mut file, &mut writer).context("stream file into blob")?;

    writer.flush()?;
    let hash = writer.hasher.finalize();

    Ok(hex::encode(hash))
}
