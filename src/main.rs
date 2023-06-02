use std::{
    fs::File,
    io::{BufReader, Read},
};

use crypto::{digest::Digest, sha1::Sha1};

fn main() {
    let mut hasher = Sha1::new();

    let filename = "tmp/hello-world.txt";

    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);

    const BUFSIZE: usize = 8192;
    let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];

    loop {
        let bytes_read = reader.read(&mut buf).unwrap();

        if bytes_read == 0 {
            break;
        }

        let bytes = &buf[0..bytes_read];

        hasher.input(bytes);
    }

    let hex = hasher.result_str();

    println!("{hex}");
}
