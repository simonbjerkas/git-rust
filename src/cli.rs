use clap::{Parser, Subcommand};

use std::path;

#[derive(Parser)]
#[command(name = "mygit")]
#[command(
    about = "A version control system, is it git perhaps?",
    long_about = "Me using clap for the first time. This seems pretty awesome! Also, this is a codecrafters implementation of Git"
)]
pub struct MyGit {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    CatFile {
        ///pretty print the result
        #[arg(short)]
        pretty_print: bool,

        ///blob hash
        blob_hash: Option<String>,
    },
    HashObject {
        ///write to object database
        #[arg(short)]
        write: bool,

        ///file to hash
        file: Option<path::PathBuf>,
    },
    LsTree {
        ///return names only, no metadata
        #[arg(long)]
        name_only: bool,
        ///tree hash
        tree_hash: Option<String>,
    },
    WriteTree,
    CommitTree {
        ///tree hash
        tree_hash: Option<String>,
        ///hash to parent commit
        #[arg(short)]
        parent_hash: Option<String>,
        ///commit message
        #[arg(short)]
        message: Option<String>,
    },
}
