mod argument_parser;
mod file_hasher;
mod hash_strategy;
mod hasher;
mod report;

use std::time::Instant;

use argument_parser::{parse_cli_arguments, AppArgs};
use log::debug;

#[tokio::main(worker_threads = 10)]
async fn main() {
    env_logger::init();
    debug!("Execution started.");

    let start_time = Instant::now();

    let args: AppArgs = parse_cli_arguments();

    hasher::process(args.path, args.hash_strategy).await;

    eprintln!("\nTook {} seconds.", start_time.elapsed().as_secs_f32());
}
