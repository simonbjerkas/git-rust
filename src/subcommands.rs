pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;

use std::{
    ffi::CStr,
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use anyhow::{Context, Result};
use codecrafters_git::GitError;
use flate2::read::ZlibDecoder;

enum Kind {
    Blob,
    Tree,
}

impl FromStr for Kind {
    type Err = GitError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Kind::Blob),
            "tree" => Ok(Kind::Tree),
            other => Err(GitError::Usupported(other.to_string())),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
        }
    }
}

struct Object<R> {
    kind: Kind,
    size: usize,
    reader: R,
}

impl Object<()> {
    fn read(hash: &str) -> Result<Object<impl BufRead>> {
        let file = fs::File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
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
            .with_context(|| format!("don't know how to print kind: {kind}"))?;
        let size = size
            .parse::<u64>()
            .with_context(|| format!(".git/objects file header has invalid size: {size}"))?;

        let reader = reader.take(size);

        Ok(Object {
            kind,
            size: size as usize,
            reader,
        })
    }
}
