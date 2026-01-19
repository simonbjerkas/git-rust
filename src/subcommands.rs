pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

use std::{
    ffi::CStr,
    fmt::Display,
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Context, Result};
use codecrafters_git::{GitError, HashWriter};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::Digest;

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

    fn blob_from_file(path: &PathBuf) -> Result<Object<impl Read>> {
        let file = fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
        let meta = file
            .metadata()
            .with_context(|| format!("metadata {}", path.display()))?;

        Ok(Object {
            kind: Kind::Blob,
            size: meta.len() as usize,
            reader: file,
        })
    }
}

impl<R> Object<R>
where
    R: Read,
{
    fn new_tree(size: usize, reader: R) -> Object<R> {
        Object {
            kind: Kind::Tree,
            size,
            reader,
        }
    }

    fn write(&mut self, writer: impl Write) -> Result<[u8; 20]> {
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter::new(writer);

        write!(writer, "{} {}\0", self.kind, self.size)?;

        io::copy(&mut self.reader, &mut writer).context("stream file into blob")?;

        writer.flush()?;
        let hash = writer.hasher.finalize();

        Ok(hash.into())
    }

    pub fn write_to_objects(&mut self) -> Result<[u8; 20]> {
        let tmp = "temp";
        let writer = fs::File::create(tmp).context("creating temp file for object")?;
        let hash = self
            .write(writer)
            .context("streaming object into temp file")?;

        let hex_hash = hex::encode(hash);

        fs::create_dir_all(format!(".git/objects/{}", &hex_hash[..2]))
            .context("create subdirectory to .git/objects")?;
        fs::rename(
            &tmp,
            format!(".git/objects/{}/{}", &hex_hash[..2], &hex_hash[2..]),
        )
        .context("move temp object file into .git/objects")?;

        Ok(hash)
    }
}
