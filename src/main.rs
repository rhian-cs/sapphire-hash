mod argument_parser;
mod file_hasher;
mod hash_strategy;
mod recursive_hasher;
mod report;

use std::{io, time::Instant};

use argument_parser::{parse_cli_arguments, AppArgs};
use log::debug;
use recursive_hasher::RecursiveHasher;

#[tokio::main(worker_threads = 10)]
async fn main() -> Result<(), io::Error> {
    env_logger::init();
    debug!("Execution started.");

    let start_time = Instant::now();

    let args: AppArgs = parse_cli_arguments();

    RecursiveHasher::process(args.path, args.hash_strategy).await?;

    eprintln!("\nTook {} seconds.", start_time.elapsed().as_secs_f32());

    Ok(())
}
