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
    reporter::{
        report_entry::{self, ReportEntry},
        report_message::ReportMessage,
        Reporter,
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
        eprintln!("Processing path...");

        let (report_sender, report_receiver) = mpsc::channel();

        let mut recursive_hasher = RecursiveHasher::new(hash_strategy, report_sender);

        let reporter_handle = tokio::spawn(async {
            Reporter::new(report_receiver).process_entries();
        });

        recursive_hasher.process_path(path)?;
        recursive_hasher.wait_for_completion().await;
        reporter_handle.await?;

        Ok(())
    }

    fn process_path(&mut self, path: String) -> Result<(), io::Error> {
        let is_directory = Path::new(&path).is_dir();

        if is_directory {
            self.process_directory_files(path)?;
        } else {
            self.process_file(path);
        }

        Ok(())
    }

    fn process_directory_files(&mut self, parent_path: String) -> Result<(), io::Error> {
        let child_paths = fs::read_dir(&parent_path)?;
        let sender = self.report_sender.clone();

        for child_path in child_paths {
            self.process_directory_child_path(&sender, &parent_path, child_path)?;
        }

        Ok(())
    }

    fn process_directory_child_path(
        &mut self,
        sender: &mpsc::Sender<ReportMessage>,
        parent_path: &String,
        child_path: Result<DirEntry, io::Error>,
    ) -> Result<(), io::Error> {
        let result = match child_path {
            Ok(child_path) => {
                self.process_path(parse_path_dir_entry(child_path))?;

                report_entry::ResultType::Directory(Ok(()))
            }
            Err(err) => report_entry::ResultType::Directory(Err(err)),
        };

        let entry = ReportEntry {
            path: parent_path.clone(),
            result,
            is_directory: true,
        };

        sender.send(ReportMessage::Message(entry)).unwrap();

        Ok(())
    }

    fn process_file(&mut self, path: String) {
        let hash_strategy = self.hash_strategy.clone();
        let sender = self.report_sender.clone();

        let handle = tokio::spawn(async move {
            let result = FileHasher::calculate(&path, hash_strategy);

            let entry = ReportEntry {
                path,
                result: report_entry::ResultType::File(result),
                is_directory: false,
            };

            sender.send(ReportMessage::Message(entry)).unwrap();
        });

        self.join_set.spawn(handle);
    }

    async fn wait_for_completion(&mut self) {
        while let Some(_) = self.join_set.join_next().await {}

        self.report_sender
            .send(ReportMessage::EndTransmission)
            .unwrap();
    }
}

fn parse_path_dir_entry(path: DirEntry) -> String {
    let path = path.path();

    path.to_str().unwrap().to_owned()
}
