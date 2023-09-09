use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

use tokio::task::{JoinError, JoinSet};

use crate::{file_hasher::FileHasher, hash_strategy::HashStrategy};

pub struct RecursiveHasher {
    join_set: JoinSet<Result<(), JoinError>>,
    hash_strategy: HashStrategy,
}

impl RecursiveHasher {
    pub async fn process(path: &str, hash_strategy: HashStrategy) -> Result<(), io::Error> {
        let mut recursive_hasher = RecursiveHasher {
            join_set: JoinSet::new(),
            hash_strategy: hash_strategy,
        };

        recursive_hasher.process_path(path.to_owned().clone())?;
        recursive_hasher.wait_for_completion().await;

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

        for child_path in child_paths {
            match child_path {
                Ok(child_path) => self.process_path(parse_path_dir_entry(child_path))?,
                Err(err) => println!("{parent_path}\tError: {err}"),
            }
        }

        Ok(())
    }

    fn process_file(&mut self, path: String) {
        let hash_strategy = self.hash_strategy.clone();

        let handle = tokio::spawn(async move {
            let result = FileHasher::calculate(&path, hash_strategy);

            match result {
                Ok(hex) => println!("{path}\tHash: {hex}"),
                Err(err) => println!("{path}\tError: {err}"),
            }
        });

        self.join_set.spawn(handle);
    }

    async fn wait_for_completion(&mut self) {
        while let Some(_) = self.join_set.join_next().await {}
    }
}

fn parse_path_dir_entry(path: DirEntry) -> String {
    let path = path.path();

    path.to_str().unwrap().to_owned()
}
