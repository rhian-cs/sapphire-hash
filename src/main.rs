use clap::Parser;
use hash_calculator::file_hash_calculator;

#[derive(Parser)]
struct Args {
    #[arg()]
    directory: String,
}

fn main() {
    let args = Args::parse();
    let hex = file_hash_calculator::calculate(&args.directory).unwrap();

    println!("{hex}");
}
