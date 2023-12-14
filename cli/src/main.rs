mod argument_parser;

use argument_parser::{parse_cli_arguments, AppArgs, ArgumentError};
use log::debug;
use recursive_hash_calculator_core::hasher;
use spinoff::{spinners, Spinner};

#[tokio::main(worker_threads = 10)]
async fn main() {
    env_logger::init();
    debug!("Execution started.");

    if let Err(err) = run().await {
        println!("Error: {err}");
    }
}

async fn run() -> Result<(), ArgumentError> {
    let args: AppArgs = parse_cli_arguments()?;

    let spinner = create_spinner("Now processing files...");

    let result = hasher::process(args.path, args.hash_strategy, args.report_type).await;

    print_processed_files_count(spinner, result.processed_files_count);
    eprintln!("\nTook {} seconds.", result.elapsed_time.as_secs_f32());

    Ok(())
}

fn create_spinner(message: &'static str) -> Spinner {
    Spinner::new_with_stream(spinners::Dots, message, None, spinoff::Streams::Stderr)
}

fn print_processed_files_count(mut spinner: Spinner, count: i32) {
    let files_have = if count == 1 { "file has" } else { "files have" };

    spinner.stop_with_message(&format!("{count} {files_have} been processed!"));
}
