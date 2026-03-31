# git-rust

A Git implementation written in Rust. Supports core plumbing commands for reading and writing Git objects, building trees, and creating commits.

## Commands

| Command | Description |
|---|---|
| `init` | Initialize a new `.git` repository |
| `cat-file -p <hash>` | Print the contents of a Git object |
| `hash-object [-w] <file>` | Hash a file as a blob object, optionally writing it to the object store |
| `ls-tree [--name-only] <hash>` | List the contents of a tree object |
| `write-tree` | Write the current working directory as a tree object |
| `commit-tree <tree> [-p <parent>] [-m <message>]` | Create a commit object from a tree |

## Building

```sh
cargo build --release
```

## Usage

```sh
# Initialize a repo
./target/release/mygit init

# Hash and store a file
./target/release/mygit hash-object -w hello.txt

# Read an object
./target/release/mygit cat-file -p <hash>

# Write current directory as a tree and commit it
tree=$(./target/release/mygit write-tree)
./target/release/mygit commit-tree "$tree" -m "initial commit"
```

## Tech

- [`clap`](https://github.com/clap-rs/clap) — CLI parsing
- [`flate2`](https://github.com/rust-lang/flate2-rs) — zlib compression/decompression for object storage
- [`sha1`](https://github.com/RustCrypto/hashes) — SHA-1 hashing for object addressing
- [`anyhow`](https://github.com/dtolnay/anyhow) — error handling
