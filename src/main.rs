use std::path::Path;

use clap::Parser;
use hash_calculator::recursive_hasher;

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

    recursive_hasher::process(&path);
}
