mod cli;
mod subcommands;

use anyhow::{Context, Result};
use clap::Parser;

use std::io::{self, Write};

use cli::MyGit;

fn main() -> Result<()> {
    let cli = MyGit::parse();

    match cli.command {
        cli::Commands::Init => {
            subcommands::init::execute()?;
            println!("Initialized git directory")
        }
        cli::Commands::CatFile { blob, pretty } => {
            let blob = blob.expect("missing argument");
            let (kind, buf) = subcommands::cat_file::execute(pretty, blob)?;

            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            use subcommands::cat_file::Kind;

            match kind {
                Kind::Blob => stdout
                    .write_all(&buf)
                    .context("write objects content to stdout")?,
            }
        }
        cli::Commands::HashObject { write, file } => {
            let file = file.expect("missing argument");
            let hash = subcommands::hash_object::execute(write, &file)?;
            println!("{hash}");
        }
    }

    Ok(())
}
