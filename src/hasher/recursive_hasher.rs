use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
    sync::mpsc,
};

use tokio::task::{JoinError, JoinSet};

use crate::{
    file_hasher::FileHasher,
    hash_strategy::HashStrategy,
    report::{
        report_entry::{self, ReportEntry},
        report_message::ReportMessage,
    },
};

type ReportResultType = report_entry::ResultType;

pub struct RecursiveHasher {
    join_set: JoinSet<Result<(), JoinError>>,
    hash_strategy: HashStrategy,
    report_sender: mpsc::Sender<ReportMessage>,
}

impl RecursiveHasher {
    pub fn new(hash_strategy: HashStrategy, report_sender: mpsc::Sender<ReportMessage>) -> RecursiveHasher {
        RecursiveHasher {
            join_set: JoinSet::new(),
            hash_strategy,
            report_sender,
        }
    }

    pub fn process_path_recursively(&mut self, path_string: String) {
        let path = Path::new(&path_string);

        let result_type = match path {
            p if p.is_symlink() => Some(ReportResultType::Symlink),
            p if p.is_dir() => Some(self.process_directory(&path_string)),
            p if p.is_file() => {
                self.process_file(&path_string);
                None // file result report is published in a separate thread
            }
            _ => Some(ReportResultType::SpecialFile),
        };

        if let Some(result_type) = result_type {
            publish_result_to_reporter(self.report_sender.clone(), ReportEntry::new(path_string, result_type));
        }
    }

    fn process_directory(&mut self, path: &String) -> ReportResultType {
        let result = self.process_directory_children(&path);

        ReportResultType::Directory(result)
    }

    fn process_directory_children(&mut self, parent_path: &String) -> Result<(), io::Error> {
        let child_paths = fs::read_dir(parent_path)?;

        for child_path in child_paths.flatten() {
            self.process_path_recursively(parse_path_dir_entry(child_path));
        }

        Ok(())
    }

    fn process_file(&mut self, path: &String) {
        let path = path.clone();
        let hash_strategy = self.hash_strategy;
        let sender = self.report_sender.clone();

        let handle = tokio::spawn(async move {
            let result = FileHasher::calculate(&path, hash_strategy);

            publish_result_to_reporter(sender, ReportEntry::new(path, ReportResultType::File(result)));
        });

        self.join_set.spawn(handle);
    }

    pub async fn wait_for_completion(&mut self) {
        while (self.join_set.join_next().await).is_some() {}

        self.report_sender.send(ReportMessage::EndTransmission).unwrap();
    }
}

fn publish_result_to_reporter(sender: mpsc::Sender<ReportMessage>, report_entry: ReportEntry) {
    sender.send(ReportMessage::Message(report_entry)).unwrap();
}

fn parse_path_dir_entry(dir_entry: DirEntry) -> String {
    dir_entry.path().to_str().unwrap_or("<Invalid UTF-8 String>").to_owned()
}
