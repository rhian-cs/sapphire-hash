use std::sync::mpsc;

use recursive_hash_calculator_core::ui_message::UiMessage;
use spinoff::{spinners, Spinner};

pub fn handle_messages(ui_receiver: mpsc::Receiver<UiMessage>) {
    let mut spinner = create_spinner("Now processing files...");

    for ui_message in ui_receiver {
        match ui_message {
            UiMessage::ReporterFinish(count) => {
                let files_have = if count == 1 { "file has" } else { "files have" };

                spinner.stop_with_message(&format!("{count} {files_have} been processed!"));
            }
            UiMessage::ExecutionFinish(elapsed_time) => {
                eprintln!("\nTook {} seconds.", elapsed_time.as_secs_f32());
                break;
            }
        }
    }
}

fn create_spinner(message: &'static str) -> Spinner {
    Spinner::new_with_stream(spinners::Dots, message, None, spinoff::Streams::Stderr)
}
