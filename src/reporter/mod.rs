use std::collections::BTreeMap;
use std::sync::mpsc;

use spinoff::{spinners, Spinner};

use self::report_entry::ReportEntry;
use self::report_message::ReportMessage;

pub mod report_entry;
pub mod report_message;

pub struct Reporter {
    entries: BTreeMap<String, ReportEntry>,
    receiver: mpsc::Receiver<ReportMessage>,
}

impl Reporter {
    pub fn new(report_receiver: mpsc::Receiver<ReportMessage>) -> Self {
        Reporter {
            entries: BTreeMap::new(),
            receiver: report_receiver,
        }
    }

    pub fn process_entries(&mut self) {
        self.receive_entries();
        self.output_report();
    }

    fn receive_entries(&mut self) {
        let mut spinner = create_spinner("Now processing files...");
        let mut counter = 0;

        for entry in self.receiver.iter() {
            match entry {
                ReportMessage::Message(entry) => {
                    let path = entry.path.clone();

                    if !entry.is_directory {
                        counter += 1;
                    }

                    self.entries.insert(path, entry);
                }
                ReportMessage::EndTransmission => break,
            }
        }

        spinner.stop_with_message(&format!("{counter} files have been processed!"));
    }

    fn output_report(&self) {
        for (_, entry) in self.entries.iter() {
            println!("{}", entry.format_text());
        }
    }
}

fn create_spinner(message: &'static str) -> Spinner {
    Spinner::new_with_stream(spinners::Dots, message, None, spinoff::Streams::Stderr)
}
