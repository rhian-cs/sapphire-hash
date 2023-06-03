use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crypto::{digest::Digest, sha1::Sha1};

const READ_BUFFER_SIZE: usize = 8192;

struct FileHasher {
    buf_reader: BufReader<File>,
    hasher: Sha1,
}

impl FileHasher {
    fn build(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;

        Ok(FileHasher {
            buf_reader: BufReader::new(file),
            hasher: Sha1::new(),
        })
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

pub fn calculate(filename: &str) -> Result<String, io::Error> {
    let mut calculator = FileHasher::build(&filename)?;

    Ok(calculator.calculate_digest()?)
}
