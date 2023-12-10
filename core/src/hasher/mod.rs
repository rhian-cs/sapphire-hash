use std::sync::mpsc;

use log::debug;

use crate::{hash_strategy::HashStrategy, hasher::recursive_hasher::RecursiveHasher, report::Report};

mod file_hasher;
mod recursive_hasher;

pub async fn process(path: String, hash_strategy: HashStrategy) {
    let (report_sender, report_receiver) = mpsc::channel();

    // Spawn worker to build the report while everything else is being processed
    let reporter_handle = tokio::spawn(async {
        Report::new(report_receiver).process_entries();
    });

    RecursiveHasher::new(hash_strategy, report_sender)
        .process(path)
        .wait_for_completion()
        .await;

    debug!("Waiting for reporter to complete.");
    reporter_handle.await.unwrap();
}
