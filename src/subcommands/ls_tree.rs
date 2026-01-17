use anyhow::{Context, Result};

use std::{
    ffi::CStr,
    io::{self, BufRead, Read, Write},
    str,
};

use super::{Kind, Object};

pub fn execute(tree_hash: String, name_only: bool) -> Result<()> {
    let mut object = Object::read(&tree_hash).context("parse out tree object file")?;

    anyhow::ensure!(matches!(object.kind, Kind::Tree), "expected a tree object");

    let mut hashbuf = [0; 20];
    let mut buf = Vec::new();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    loop {
        buf.clear();
        let n = object.reader.read_until(0, &mut buf)?;
        if n == 0 {
            break;
        };

        object.reader.read_exact(&mut hashbuf)?;

        let entry = CStr::from_bytes_until_nul(&mut buf)
            .expect("there is exactly one nul and thats between name and hash of object");

        let Some((mode, name)) = split_mode_name(entry.to_bytes()) else {
            anyhow::bail!(".git/objects was corrupted");
        };

        if name_only {
            stdout
                .write_all(name)
                .context("write tree entry name to stdout")?;
            writeln!(stdout, "");
            continue;
        }

        let hash = hex::encode(hashbuf);
        let object =
            Object::read(&hash).with_context(|| format!("read object for tree entry {hash}"))?;

        let mode = str::from_utf8(mode).context("mode should always be valid utf-8")?;
        write!(stdout, "{mode} {} {hash}", object.kind)
            .context("writing tree entry meta to stdout")?;

        stdout
            .write_all(name)
            .context("write tree entry name to stdout")?;

        writeln!(stdout, "")?;
    }

    Ok(())
}

fn split_mode_name(entry: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut mode_name = entry.splitn(2, |&b| b == b' ');
    let mode = mode_name
        .next()
        .expect("header shjould always split mode and name");
    let name = mode_name.next().expect("tree entry has no file name");

    Some((mode, name))
}
