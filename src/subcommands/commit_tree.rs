use anyhow::{Context, Result};

use std::{fmt::Write, io::Cursor, time};

use super::Object;

pub fn execute(
    tree_hash: String,
    parent_hash: Option<String>,
    message: Option<String>,
) -> Result<()> {
    let mut commit = String::new();
    writeln!(commit, "tree {tree_hash}")?;

    if let Some(parent) = parent_hash {
        writeln!(commit, "parent {parent}")?;
    }

    writeln!(
        commit,
        "author Simon Bjerkås <simon.bjerkas@example.com> {} +0100",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("time sinze epoch")
            .as_secs()
    )?;
    writeln!(
        commit,
        "committer Simon Bjerkås <simon.bjerkas@example.com> {} +0100",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("time sinze epoch")
            .as_secs()
    )?;

    if let Some(msg) = message {
        writeln!(commit, "")?;
        writeln!(commit, "{msg}")?;
    }

    let hash = Object::new_commit(commit.len(), Cursor::new(commit))
        .write_to_objects()
        .context("write commit object to .git/objects")?;

    println!("{}", hex::encode(hash));

    Ok(())
}
