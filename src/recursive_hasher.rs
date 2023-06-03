use std::{fs, path::Path};

use crate::file_hasher;

pub fn process(path: &str) {
    if Path::new(path).is_dir() {
        calculate_hash_for_directory(&path);
    } else {
        calculate_hash_for_file(&path);
    }
}

fn calculate_hash_for_directory(path: &str) {
    let paths = fs::read_dir(&path).unwrap();

    for path in paths {
        process(&path.unwrap().path().to_str().unwrap());
    }
}

fn calculate_hash_for_file(path: &str) {
    let hex = file_hasher::calculate(&path).unwrap();

    println!("{path}\t{hex}");
}
