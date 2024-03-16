use std::str::FromStr;

use sapphire_hash_core::{hash_strategy::HashStrategy, hasher, report_type::ReportType};
use strum::VariantNames;

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
    csv_output_filename: String,
) -> CalculateHashesResult {
    let hash_strategy = HashStrategy::from_str(&hash_algorithm).unwrap();

    let result = hasher::process(directory, hash_strategy, ReportType::Csv(csv_output_filename)).await;

    CalculateHashesResult {
        elapsed_time_secs: result.elapsed_time.as_secs_f32(),
        processed_files_count: result.processed_files_count,
    }
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn available_hashing_algorithms() -> Vec<String> {
    HashStrategy::VARIANTS.into_iter().map(|v| v.to_uppercase()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_hashing_algorithms_returns_string_options() {
        let expected_hashing_algorithms = [
            "MD5",
            "SHA1",
            "SHA224",
            "SHA256",
            "SHA384",
            "SHA512",
            "SHA3_224",
            "SHA3_256",
            "SHA3_384",
            "SHA3_512",
            "SHAKE128",
            "SHAKE256",
            "RIPEMD160",
            "SM3",
        ];

        assert_eq!(available_hashing_algorithms(), expected_hashing_algorithms);
    }
}
