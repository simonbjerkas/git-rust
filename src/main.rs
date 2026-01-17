mod cli;
mod subcommands;

use anyhow::Result;
use clap::Parser;

use cli::MyGit;

fn main() -> Result<()> {
    let cli = MyGit::parse();

    match cli.command {
        cli::Commands::Init => subcommands::init::execute()?,
        cli::Commands::CatFile {
            blob_hash,
            pretty_print,
        } => {
            let blob = blob_hash.expect("missing argument");
            subcommands::cat_file::execute(pretty_print, blob)?;
        }
        cli::Commands::HashObject { write, file } => {
            let file = file.expect("missing argument");
            subcommands::hash_object::execute(write, &file)?;
        }
        cli::Commands::LsTree {
            name_only,
            tree_hash,
        } => {
            let tree_hash = tree_hash.expect("missing argument");
            subcommands::ls_tree::execute(tree_hash, name_only)?;
        }
    }

    Ok(())
}
