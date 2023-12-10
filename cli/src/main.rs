mod argument_parser;

use std::time::Instant;

use argument_parser::{parse_cli_arguments, AppArgs, ArgumentError};
use log::debug;
use recursive_hash_calculator_core::hasher;

#[tokio::main(worker_threads = 10)]
async fn main() {
    env_logger::init();
    debug!("Execution started.");

    if let Err(err) = run().await {
        println!("Error: {err}");
    }
}

async fn run() -> Result<(), ArgumentError> {
    let start_time = Instant::now();

    let args: AppArgs = parse_cli_arguments()?;

    hasher::process(args.path, args.hash_strategy, args.report_type).await;

    eprintln!("\nTook {} seconds.", start_time.elapsed().as_secs_f32());

    Ok(())
}
