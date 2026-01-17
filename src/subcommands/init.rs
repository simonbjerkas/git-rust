use anyhow::{Context, Result};
use std::fs;

pub fn execute() -> Result<()> {
    fs::create_dir(".git").context("creating .git directory")?;
    fs::create_dir(".git/objects").context("creating .git/objects directory")?;
    fs::create_dir(".git/refs").context("creating .git/refs directory")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").context("writing head to .git/HEAD")?;

    println!("Initialized git directory");

    Ok(())
}
