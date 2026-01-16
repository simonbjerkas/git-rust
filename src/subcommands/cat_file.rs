use anyhow::{Context, Result};
use codecrafters_git::GitError;
use flate2::read::ZlibDecoder;

use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

pub fn execute(pretty: bool, blob: String) -> Result<(Kind, Vec<u8>)> {
    anyhow::ensure!(
        pretty,
        "mode must be given without -p, and we dont support mode"
    );

    let file = fs::File::open(format!(".git/objects/{}/{}", &blob[..2], &blob[2..]))
        .context("open in .git/objects")?;

    let decoder = ZlibDecoder::new(file);
    let mut reader = BufReader::new(decoder);
    let mut buf = Vec::new();

    reader
        .read_until(0, &mut buf)
        .context("read header from .git/objects")?;

    let header = CStr::from_bytes_until_nul(&buf)
        .expect("know there is exactly one nul and thats at the end");
    let header = header.to_str().expect("header isn't valid utf-8");

    let Some((kind, size)) = header.split_once(' ') else {
        anyhow::bail!(".git/objects did not start with a known type");
    };

    let kind = kind
        .parse::<Kind>()
        .context(format!("don't know how to print kind: {kind}"))?;
    let size = size
        .parse::<usize>()
        .context(format!(".git/objects file header has invalid size: {size}"))?;

    buf.clear();
    buf.resize(size, 0);

    reader
        .read_exact(&mut buf)
        .context("read true contents of .git/objects file")?;

    let eof = reader
        .read(&mut [0])
        .context("validate EOF in .git/objects file")?;
    anyhow::ensure!(eof == 0, ".git/objects file had {eof} trailing bytes");

    Ok((kind, buf))
}

pub enum Kind {
    Blob,
}

impl FromStr for Kind {
    type Err = GitError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Kind::Blob),
            other => Err(GitError::Usupported(other.to_string())),
        }
    }
}
