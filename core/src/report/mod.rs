use std::collections::BTreeMap;
use std::sync::mpsc;

use log::debug;

use crate::report_type::ReportType;

use self::output::{report_output_for, ReportOutput};
use self::report_entry::ReportEntry;
use self::report_message::ReportMessage;

pub mod report_entry;
pub mod report_message;

mod output;

pub struct Report {
    entries: BTreeMap<String, ReportEntry>,
    receiver: mpsc::Receiver<ReportMessage>,
    report_output: Box<dyn ReportOutput>,
}

impl Report {
    pub fn spawn(
        report_receiver: mpsc::Receiver<ReportMessage>,
        report_type: ReportType,
    ) -> tokio::task::JoinHandle<i32> {
        tokio::spawn(async move {
            let mut report = Report {
                entries: BTreeMap::new(),
                receiver: report_receiver,
                report_output: report_output_for(report_type),
            };

            let processed_files_count = report.receive_entries();
            report.output_report();

            processed_files_count
        })
    }

    fn receive_entries(&mut self) -> i32 {
        let mut processed_files_count = 0;

        for entry in self.receiver.iter() {
            debug!("Received entry {:?}.", entry);

            match entry {
                ReportMessage::Message(entry) => {
                    let path = entry.path.clone();

                    if entry.is_file() {
                        processed_files_count += 1;
                    }

                    self.entries.insert(path, entry);
                }
                ReportMessage::EndTransmission => break,
            }
        }

        processed_files_count
    }

    fn output_report(self) {
        debug!("Outputting report.");

        // TODO: Treat errors properly
        self.report_output.generate(self.entries).unwrap();
    }
}
