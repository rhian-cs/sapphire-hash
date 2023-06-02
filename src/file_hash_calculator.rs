use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crypto::{digest::Digest, sha1::Sha1};

const DIGEST_BUFSIZE: usize = 8192;

struct HashCalculator {
    buf_reader: BufReader<File>,
    hasher: Sha1,
}

impl HashCalculator {
    fn build(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);

        let hasher = Sha1::new();

        let calculator = HashCalculator { buf_reader, hasher };

        Ok(calculator)
    }

    fn calculate_digest(&mut self) -> Result<String, io::Error> {
        let mut buf: [u8; DIGEST_BUFSIZE] = [0; DIGEST_BUFSIZE];

        loop {
            let bytes_read = self.buf_reader.read(&mut buf)?;

            if bytes_read == 0 {
                break;
            }

            let bytes = &buf[0..bytes_read];

            self.hasher.input(bytes);
        }

        Ok(self.hasher.result_str())
    }
}

pub fn calculate(filename: &str) -> Result<String, io::Error> {
    let mut calculator = HashCalculator::build(&filename)?;

    Ok(calculator.calculate_digest()?)
}
