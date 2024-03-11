use std::str::FromStr;

use recursive_hash_calculator_core::{hash_strategy::HashStrategy, hasher, report_type::ReportType};

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub struct CalculateHashesResult {
    pub elapsed_time_secs: f32,
    pub processed_files_count: i32,
}

pub async fn hasher_process(
    directory: String,
    hash_algorithm: String,
    csv_output_directory: String,
) -> CalculateHashesResult {
    let hash_strategy = HashStrategy::from_str(&hash_algorithm).unwrap();

    let csv_output_filename = csv_output_filename(csv_output_directory);
    let result = hasher::process(directory, hash_strategy, ReportType::Csv(csv_output_filename)).await;

    CalculateHashesResult {
        elapsed_time_secs: result.elapsed_time.as_secs_f32(),
        processed_files_count: result.processed_files_count,
    }
}

fn csv_output_filename(csv_output_directory: String) -> String {
    format!("{csv_output_directory}/output.csv")
}
