use std::path::Path;

use clap::Parser;

use crate::hash_strategy::HashStrategy;

#[derive(Parser)]
struct CliArgs {
    #[arg()]
    directory: String,

    #[arg(short, long)]
    algorithm: String,
}

pub struct AppArgs {
    pub path: String,
    pub hash_strategy: HashStrategy,
}

pub fn parse_cli_arguments() -> AppArgs {
    let cli_args = CliArgs::parse();

    AppArgs {
        path: parse_path(cli_args.directory),
        hash_strategy: parse_hash_strategy(cli_args.algorithm),
    }
}

fn parse_path(path: String) -> String {
    if !Path::new(&path).try_exists().unwrap() {
        panic!("Directory or file {path} does not exist!");
    }

    path
}

fn parse_hash_strategy(algorithm_name: String) -> HashStrategy {
    let hash_strategy = HashStrategy::strategy_for(&algorithm_name);

    if let Some(hash_strategy) = hash_strategy {
        hash_strategy
    } else {
        panic!("Invalid hash strategy!");
    }
}
