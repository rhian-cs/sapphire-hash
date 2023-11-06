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
        let (report_sender, report_receiver) = mpsc::channel();

        let reporter_handle = tokio::spawn(async {
            Reporter::new(report_receiver).process_entries();
        });

        let mut recursive_hasher = RecursiveHasher::new(hash_strategy, report_sender);
        recursive_hasher.process_path(path)?;
        recursive_hasher.wait_for_completion().await;
        reporter_handle.await?;

        Ok(())
    }

    fn process_path(&mut self, path_string: String) -> Result<(), io::Error> {
        let path = Path::new(&path_string);

        if path.is_dir() {
            self.process_directory(path_string)?;
        } else {
            self.process_file(path_string);
        }

        Ok(())
    }

    fn process_directory(&mut self, parent_path: String) -> Result<(), io::Error> {
        let child_paths = fs::read_dir(&parent_path)?;
        let sender = self.report_sender.clone();

        let entry = ReportEntry {
            path: parent_path.to_owned(),
            result: report_entry::ResultType::Directory(Ok(())),
        };

        sender.send(ReportMessage::Message(entry)).unwrap();

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

            let entry = ReportEntry {
                path,
                result: report_entry::ResultType::File(result),
            };

            sender.send(ReportMessage::Message(entry)).unwrap();
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

fn parse_path_dir_entry(path: DirEntry) -> String {
    let path = path.path();

    path.to_str().unwrap().to_owned()
}
