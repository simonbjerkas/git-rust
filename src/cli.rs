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
        pretty: bool,

        ///blob
        blob: Option<String>,
    },
    HashObject {
        ///write to object database
        #[arg(short)]
        write: bool,

        ///file to hash
        file: Option<path::PathBuf>,
    },
}
