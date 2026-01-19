use anyhow::{Context, Result};

use std::{cmp, env, fs, io, os::unix::fs::PermissionsExt, path::PathBuf};

use super::Object;

pub fn execute() -> Result<()> {
    let cwd = env::current_dir()?;
    let Some(hash) = create_tree(&cwd).context("creating root tree object")? else {
        anyhow::bail!("asked to create tree of empty directory")
    };

    println!("{}", hex::encode(hash));

    Ok(())
}

fn create_tree(path: &PathBuf) -> Result<Option<[u8; 20]>> {
    let mut dir =
        fs::read_dir(path).with_context(|| format!("open directory {}", path.display()))?;
    let mut entries = Vec::new();

    while let Some(entry) = dir.next() {
        let entry = entry.with_context(|| format!("bad directory entry in {}", path.display()))?;
        let file_name = entry.file_name();
        let meta = entry.metadata().context("metadata for directory entry")?;

        entries.push((entry, file_name, meta));
    }

    entries.sort_by(|a, b| {
        let afn = &a.1.as_encoded_bytes();
        let bfn = &b.1.as_encoded_bytes();

        let common_len = cmp::min(afn.len(), bfn.len());

        match afn[..common_len].cmp(&bfn[..common_len]) {
            cmp::Ordering::Equal => {}
            o => return o,
        }

        if afn.len() == bfn.len() {
            return cmp::Ordering::Equal;
        }

        let c1 = if let Some(c) = afn.get(common_len).copied() {
            Some(c)
        } else if a.2.is_dir() {
            Some(b'/')
        } else {
            None
        };

        let c2 = if let Some(c) = bfn.get(common_len).copied() {
            Some(c)
        } else if b.2.is_dir() {
            Some(b'/')
        } else {
            None
        };

        c1.cmp(&c2)
    });

    let mut trees = Vec::new();
    for (entry, file_name, meta) in entries {
        if file_name == ".git" {
            continue;
        }
        let mode = if meta.is_dir() {
            "40000"
        } else if (meta.permissions().mode() & 0o111) != 0 {
            "100755"
        } else {
            "100644"
        };

        let path = entry.path();
        let hash = if meta.is_dir() {
            let Some(hash) = create_tree(&path)? else {
                //in case of empty directory
                continue;
            };
            hash
        } else {
            let hash = Object::blob_from_file(&path)
                .with_context(|| format!("create blob from {}", path.display()))?
                .write_to_objects()
                .context("write blob into .git/objects")?;

            hash
        };

        trees.extend(mode.as_bytes());
        trees.push(b' ');
        trees.extend(file_name.as_encoded_bytes());
        trees.push(0);
        trees.extend(hash);
    }

    if trees.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            Object::new_tree(trees.len(), io::Cursor::new(trees))
                .write_to_objects()
                .context("write tree to .git/objects")?,
        ))
    }
}
