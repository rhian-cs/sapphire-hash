use std::{sync::mpsc, time::Instant};

use log::debug;

use crate::{
    hash_strategy::HashStrategy, hasher::recursive_hasher::RecursiveHasher, report::Report, report_type::ReportType,
    ui_message::UiMessage,
};

mod file_hasher;
mod recursive_hasher;

pub async fn process(
    ui_sender: mpsc::Sender<UiMessage>,
    path: String,
    hash_strategy: HashStrategy,
    report_type: ReportType,
) {
    let start_time = Instant::now();
    let (report_sender, report_receiver) = mpsc::channel();

    // Spawn worker to build the report while everything else is being processed
    let report_ui_sender = ui_sender.clone();
    let reporter_handle = tokio::spawn(async {
        Report::new(report_ui_sender, report_receiver, report_type).process_entries();
    });

    RecursiveHasher::new(hash_strategy, report_sender)
        .process(path)
        .wait_for_completion()
        .await;

    debug!("Waiting for reporter to complete.");
    reporter_handle.await.unwrap();

    ui_sender
        .send(UiMessage::ExecutionFinish(start_time.elapsed()))
        .unwrap();
}
