use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crypto::digest::Digest;
use log::debug;

use crate::hash_strategy::HashStrategy;

const READ_BUFFER_SIZE: usize = 8192;

pub struct FileHasher {
    buf_reader: BufReader<File>,
    hasher: Box<dyn Digest>,
}

impl FileHasher {
    pub fn calculate(filename: &str, hash_strategy: HashStrategy) -> Result<String, io::Error> {
        debug!("Calculating hash for file {filename}.");

        let file = File::open(filename)?;

        let mut file_hasher = FileHasher {
            buf_reader: BufReader::new(file),
            hasher: HashStrategy::hasher_for(hash_strategy),
        };

        file_hasher.calculate_digest()
    }

    fn calculate_digest(&mut self) -> Result<String, io::Error> {
        self.read_from_file_and_feed_hasher()?;

        Ok(self.hasher.result_str())
    }

    fn read_from_file_and_feed_hasher(&mut self) -> Result<(), io::Error> {
        let mut input_buffer = [0; READ_BUFFER_SIZE];

        loop {
            let bytes_read_count = self.buf_reader.read(&mut input_buffer)?;

            match bytes_read_count {
                0 => break,
                bytes_read_count => {
                    let bytes = &input_buffer[0..bytes_read_count];

                    self.hasher.input(bytes);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::file_hasher::FileHasher;
    use crate::hash_strategy::HashStrategy;

    #[test]
    fn test_calculate_with_valid_file() {
        let result =
            FileHasher::calculate("tests/fixtures/files/sample_file.txt", HashStrategy::Sha256);

        let expected_hash = "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3";

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_hash.to_string());
    }

    #[test]
    fn test_calculate_with_inexistent_file() {
        let result = FileHasher::calculate(
            "tests/fixtures/files/inexistent_000.txt",
            HashStrategy::Sha256,
        );

        assert_eq!(
            result.err().unwrap().to_string(),
            "No such file or directory (os error 2)".to_string()
        );
    }
}
