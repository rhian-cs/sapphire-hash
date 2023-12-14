use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use log::debug;

use crate::{
    hash_strategy::HashStrategy, hasher::recursive_hasher::RecursiveHasher, report::Report, report_type::ReportType,
};

mod file_hasher;
mod recursive_hasher;

pub struct HasherResult {
    pub elapsed_time: Duration,
    pub processed_files_count: i32,
}

pub async fn process(path: String, hash_strategy: HashStrategy, report_type: ReportType) -> HasherResult {
    let start_time = Instant::now();
    let (report_sender, report_receiver) = mpsc::channel();

    let report_handle = Report::spawn(report_receiver, report_type);
    RecursiveHasher::new(hash_strategy, report_sender).process(path).await;

    debug!("Waiting for reporter to complete.");
    let processed_files_count = report_handle.await.unwrap();

    HasherResult {
        processed_files_count,
        elapsed_time: start_time.elapsed(),
    }
}
