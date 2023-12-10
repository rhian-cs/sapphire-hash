use std::collections::BTreeMap;
use std::sync::mpsc;

use log::debug;
use spinoff::{spinners, Spinner};

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
    pub fn new(report_receiver: mpsc::Receiver<ReportMessage>, report_type: ReportType) -> Self {
        Report {
            entries: BTreeMap::new(),
            receiver: report_receiver,
            report_output: report_output_for(report_type),
        }
    }

    pub fn process_entries(mut self) {
        self.receive_entries();
        self.output_report();
    }

    fn receive_entries(&mut self) {
        let mut spinner = create_spinner("Now processing files...");
        let mut counter = 0;

        for entry in self.receiver.iter() {
            debug!("Received entry {:?}.", entry);

            match entry {
                ReportMessage::Message(entry) => {
                    let path = entry.path.clone();

                    if entry.is_file() {
                        counter += 1;
                    }

                    self.entries.insert(path, entry);
                }
                ReportMessage::EndTransmission => break,
            }
        }

        spinner.stop_with_message(&format!("{counter} files have been processed!"));
    }

    fn output_report(self) {
        debug!("Outputting report.");

        // TODO: Treat errors properly
        self.report_output.generate(self.entries).unwrap();
    }
}

fn create_spinner(message: &'static str) -> Spinner {
    Spinner::new_with_stream(spinners::Dots, message, None, spinoff::Streams::Stderr)
}
