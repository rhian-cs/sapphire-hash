use std::{io, path::Path};

use clap::Parser;

use crate::hash_strategy::HashStrategy;
use thiserror::Error;

#[derive(Parser)]
struct CliArgs {
    #[arg()]
    directory: String,

    #[arg(short, long)]
    algorithm: HashStrategy,
}

#[derive(Error, Debug)]
pub enum ArgumentError {
    #[error("Directory or file `{0}` does not exist!")]
    InexistentFile(String),

    #[error("Unexpected IO Error!")]
    IoError(#[from] io::Error),
}

pub struct AppArgs {
    pub path: String,
    pub hash_strategy: HashStrategy,
}

pub fn parse_cli_arguments() -> Result<AppArgs, ArgumentError> {
    let cli_args = CliArgs::parse();

    Ok(AppArgs {
        path: parse_path(cli_args.directory)?,
        hash_strategy: cli_args.algorithm,
    })
}

fn parse_path(path: String) -> Result<String, ArgumentError> {
    match Path::new(&path).try_exists() {
        Ok(true) => Ok(path),
        Ok(false) => Err(ArgumentError::InexistentFile(path)),
        Err(err) => Err(ArgumentError::IoError(err)),
    }
}
