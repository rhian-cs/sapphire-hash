mod argument_parser;
mod ui_message_handler;

use std::sync::mpsc;

use argument_parser::{parse_cli_arguments, AppArgs, ArgumentError};
use log::debug;
use recursive_hash_calculator_core::{hasher, ui_message::UiMessage};
use ui_message_handler::handle_messages;

#[tokio::main(worker_threads = 10)]
async fn main() {
    env_logger::init();
    debug!("Execution started.");

    if let Err(err) = run().await {
        println!("Error: {err}");
    }
}

async fn run() -> Result<(), ArgumentError> {
    let (ui_sender, ui_receiver) = mpsc::channel::<UiMessage>();

    let args: AppArgs = parse_cli_arguments()?;
    let hasher_handle = tokio::spawn(hasher::process(
        ui_sender,
        args.path,
        args.hash_strategy,
        args.report_type,
    ));

    handle_messages(ui_receiver);
    hasher_handle.await.unwrap();

    Ok(())
}
