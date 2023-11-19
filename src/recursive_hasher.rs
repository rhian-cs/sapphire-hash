use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
    sync::mpsc,
};

use log::debug;
use tokio::task::{JoinError, JoinSet};

use crate::{
    file_hasher::FileHasher,
    hash_strategy::HashStrategy,
    report::{
        report_entry::{self, ReportEntry},
        report_message::ReportMessage,
        Report,
    },
};

pub struct RecursiveHasher {
    join_set: JoinSet<Result<(), JoinError>>,
    hash_strategy: HashStrategy,
    report_sender: mpsc::Sender<ReportMessage>,
}

impl RecursiveHasher {
    fn new(
        hash_strategy: HashStrategy,
        report_sender: mpsc::Sender<ReportMessage>,
    ) -> RecursiveHasher {
        RecursiveHasher {
            join_set: JoinSet::new(),
            hash_strategy,
            report_sender,
        }
    }

    pub async fn process(path: String, hash_strategy: HashStrategy) -> Result<(), io::Error> {
        let (report_sender, report_receiver) = mpsc::channel();

        let reporter_handle = tokio::spawn(async {
            Report::new(report_receiver).process_entries();
        });

        let mut recursive_hasher = RecursiveHasher::new(hash_strategy, report_sender);
        recursive_hasher.process_path(path)?;

        debug!("Waiting for all hasher threads to complete.");
        recursive_hasher.wait_for_completion().await;

        debug!("Waiting for reporter to complete.");
        reporter_handle.await?;

        Ok(())
    }

    fn process_path(&mut self, path_string: String) -> Result<(), io::Error> {
        let path = Path::new(&path_string);

        match path {
            p if p.is_symlink() => {
                publish_result(
                    self.report_sender.clone(),
                    ReportEntry {
                        path: path_string,
                        result: report_entry::ResultType::Symlink,
                    },
                );
            }
            p if p.is_dir() => self.process_directory(path_string)?,
            p if p.is_file() => self.process_file(path_string),
            _ => {
                publish_result(
                    self.report_sender.clone(),
                    ReportEntry {
                        path: path_string,
                        result: report_entry::ResultType::SpecialFile,
                    },
                );
            }
        }

        Ok(())
    }

    fn process_directory(&mut self, parent_path: String) -> Result<(), io::Error> {
        let child_paths = fs::read_dir(&parent_path)?;

        publish_result(
            self.report_sender.clone(),
            ReportEntry {
                path: parent_path.to_owned(),
                result: report_entry::ResultType::Directory(Ok(())),
            },
        );

        for child_path in child_paths {
            self.process_path(parse_path_dir_entry(child_path?))?;
        }

        Ok(())
    }

    fn process_file(&mut self, path: String) {
        let hash_strategy = self.hash_strategy;
        let sender = self.report_sender.clone();

        let handle = tokio::spawn(async move {
            let result = FileHasher::calculate(&path, hash_strategy);

            publish_result(
                sender,
                ReportEntry {
                    path,
                    result: report_entry::ResultType::File(result),
                },
            );
        });

        self.join_set.spawn(handle);
    }

    async fn wait_for_completion(&mut self) {
        while (self.join_set.join_next().await).is_some() {}

        self.report_sender
            .send(ReportMessage::EndTransmission)
            .unwrap();
    }
}

fn publish_result(sender: mpsc::Sender<ReportMessage>, report_entry: ReportEntry) {
    sender.send(ReportMessage::Message(report_entry)).unwrap();
}

fn parse_path_dir_entry(path: DirEntry) -> String {
    let path = path.path();

    path.to_str().unwrap().to_owned()
}
