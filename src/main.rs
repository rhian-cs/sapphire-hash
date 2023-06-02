use std::{fs, path::Path};

use clap::Parser;
use hash_calculator::file_hash_calculator;

#[derive(Parser)]
struct Args {
    #[arg()]
    directory: String,
}

fn main() {
    let args = Args::parse();
    let path = args.directory;

    if !Path::new(&path).try_exists().unwrap() {
        panic!("Directory or file {path} does not exist!");
    }

    calculate_hash_for_path(&path);
}

fn calculate_hash_for_path(path: &str) {
    if Path::new(path).is_dir() {
        calculate_hash_for_directory(&path);
        return;
    }

    calculate_hash_for_file(&path);
}

fn calculate_hash_for_directory(path: &str) {
    let paths = fs::read_dir(&path).unwrap();

    for path in paths {
        calculate_hash_for_path(&path.unwrap().path().to_str().unwrap());
    }
}

fn calculate_hash_for_file(path: &str) {
    let hex = file_hash_calculator::calculate(&path).unwrap();

    println!("{path}\t{hex}");
}
