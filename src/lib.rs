mod error;

use sha1::{Digest, Sha1};

use std::io::Write;

pub use error::GitError;

pub struct HashWriter<W> {
    writer: W,
    pub hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W> HashWriter<W>
where
    W: Write,
{
    pub fn new(writer: W) -> HashWriter<W> {
        HashWriter {
            writer,
            hasher: Sha1::new(),
        }
    }
}
