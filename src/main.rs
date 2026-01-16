mod subcommands;

use anyhow::Result;
use clap::Parser;
use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::{self, Read},
    path,
};
use subcommands::MyGit;

enum MyGitPaths {
    Objects { blob: String },
}

impl MyGitPaths {
    fn path(&self) -> path::PathBuf {
        match self {
            MyGitPaths::Objects { blob } => {
                path::Path::new(&format!(".git/objects/{}/{}", &blob[..2], &blob[2..])).to_owned()
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = MyGit::parse();

    match cli.command {
        subcommands::Commands::Init => {
            std::fs::create_dir(".git").unwrap();
            std::fs::create_dir(".git/objects").unwrap();
            std::fs::create_dir(".git/refs").unwrap();
            std::fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        subcommands::Commands::CatFile { blob, .. } => {
            let path = MyGitPaths::Objects {
                blob: blob.unwrap_or(String::new()),
            }
            .path()
            .to_owned();
            let reader = io::BufReader::new(fs::File::open(path)?);
            let mut decoder = ZlibDecoder::new(reader);
            let mut res = String::new();

            decoder.read_to_string(&mut res)?;

            println!("{res}");
        }
    }

    Ok(())
}
