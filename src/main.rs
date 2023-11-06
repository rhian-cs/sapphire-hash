mod argument_parser;
mod file_hasher;
mod hash_strategy;
mod recursive_hasher;
mod report;

use std::io;

use argument_parser::{parse_cli_arguments, AppArgs};
use log::debug;
use recursive_hasher::RecursiveHasher;

#[tokio::main(worker_threads = 10)]
async fn main() -> Result<(), io::Error> {
    env_logger::init();
    debug!("Execution started.");

    let args: AppArgs = parse_cli_arguments();

    RecursiveHasher::process(args.path, args.hash_strategy).await?;

    Ok(())
}
