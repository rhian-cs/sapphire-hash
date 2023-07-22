use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crypto::digest::Digest;

use crate::hash_strategy::HashStrategy;

const READ_BUFFER_SIZE: usize = 8192;

pub struct FileHasher {
    buf_reader: BufReader<File>,
    hasher: Box<dyn Digest>,
}

impl FileHasher {
    pub fn calculate(filename: &str, hash_strategy: HashStrategy) -> Result<String, io::Error> {
        let file = File::open(filename)?;

        let mut file_hasher = FileHasher {
            buf_reader: BufReader::new(file),
            hasher: HashStrategy::hasher_for(hash_strategy),
        };

        Ok(file_hasher.calculate_digest()?)
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
