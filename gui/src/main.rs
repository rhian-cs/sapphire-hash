use std::str::FromStr;

use recursive_hash_calculator_core::{hash_strategy::HashStrategy, hasher, report_type::ReportType};
use report_path::report_output_path;

slint::include_modules!();

mod file_dialog;
mod report_path;
mod result_formatter;

const CALCULATE_BUTTON_IDLE_TEXT: &str = "Calculate";
const CALCULATE_BUTTON_IN_PROGRESS_TEXT: &str = "Calculating...";

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    window.on_input_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_input_path(file_dialog::open_directory_dialog().into())
    });

    window.on_output_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_output_path(file_dialog::open_directory_dialog().into())
    });

    window.on_calculate_button_clicked({
        let weak_window = window.as_weak();

        move || {
            let window = weak_window.unwrap();
            let input_path = window.get_input_path().to_string();
            let output_path = window.get_output_path().to_string();

            let hash_algorithm = window.get_hash_algorithm().to_lowercase();
            let hash_strategy = HashStrategy::from_str(&hash_algorithm).unwrap();

            window.set_error_message("".into());
            window.set_info_message("".into());

            if input_path.is_empty() {
                window.set_error_message("Please select an input directory.".into());
                return;
            }

            if output_path.is_empty() {
                window.set_error_message("Please select a directory for the report to be saved.".into());
                return;
            }

            window.set_is_processing(true);
            window.set_calculate_button_text(CALCULATE_BUTTON_IN_PROGRESS_TEXT.into());

            tokio::spawn(calculate_hashes(
                weak_window.clone(),
                hash_strategy,
                input_path,
                report_output_path(&output_path, hash_strategy),
            ));
        }
    });

    window.run()
}

async fn calculate_hashes(
    window: slint::Weak<MainWindow>,
    hash_strategy: HashStrategy,
    input_path: String,
    output_path: String,
) {
    let result = hasher::process(input_path, hash_strategy, ReportType::Csv(output_path.clone())).await;
    let formatted_result = result_formatter::format(&result, &output_path);

    window
        .upgrade_in_event_loop(move |window| {
            window.set_is_processing(false);
            window.set_calculate_button_text(CALCULATE_BUTTON_IDLE_TEXT.into());
            window.set_info_message(formatted_result.into());
        })
        .unwrap();
}
