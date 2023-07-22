mod file_hasher;
mod hash_strategy;
mod hasher_strategies;
mod recursive_hasher;

use std::{io, path::Path};

use clap::Parser;
use recursive_hasher::RecursiveHasher;

#[derive(Parser)]
struct Args {
    #[arg()]
    directory: String,
}

#[tokio::main(worker_threads = 10)]
async fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let path = args.directory;

    if !Path::new(&path).try_exists().unwrap() {
        panic!("Directory or file {path} does not exist!");
    }

    RecursiveHasher::process(&path).await?;

    Ok(())
}
