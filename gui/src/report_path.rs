use std::path::Path;

use recursive_hash_calculator_core::hash_strategy::HashStrategy;

pub fn report_output_path(output_path: &str, hash_strategy: HashStrategy) -> String {
    Path::new(&output_path)
        .join(report_filename(hash_strategy))
        .to_str()
        .unwrap()
        .to_owned()
}

fn report_filename(hash_strategy: HashStrategy) -> String {
    let time = chrono::Local::now();
    let fmt = format!("report_{hash_strategy}_%Y-%m-%d_%H%M%S.csv");

    time.format(&fmt).to_string()
}
