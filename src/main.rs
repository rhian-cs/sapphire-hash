pub mod file_hasher;
pub mod recursive_hasher;

use std::{io, path::Path};

use clap::Parser;

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

    recursive_hasher::process(&path).await?;

    Ok(())
}
