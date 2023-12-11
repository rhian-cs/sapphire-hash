use std::path::Path;

pub fn report_output_path(output_path: &str) -> String {
    Path::new(&output_path)
        .join(report_filename())
        .to_str()
        .unwrap()
        .to_owned()
}

fn report_filename() -> String {
    chrono::Local::now().format("%Y-%m-%d_%H%M%S.csv").to_string()
}
