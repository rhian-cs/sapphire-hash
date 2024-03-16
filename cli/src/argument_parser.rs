use std::{io, path::Path};

use clap::Parser;

use sapphire_hash_core::{hash_strategy::HashStrategy, report_type::ReportType};
use thiserror::Error;

#[derive(Parser)]
struct CliArgs {
    #[arg()]
    directory: String,

    #[arg(short, long)]
    algorithm: HashStrategy,

    #[arg(short, long, default_value = None)]
    output: Option<String>,
}

#[derive(Error, Debug)]
pub enum ArgumentError {
    #[error("Directory or file `{0}` does not exist!")]
    InexistentFile(String),

    #[error("Unexpected IO Error!")]
    IoError(#[from] io::Error),

    #[error("Unrecognized output format!")]
    InvalidOutputType,
}

pub struct AppArgs {
    pub path: String,
    pub hash_strategy: HashStrategy,
    pub report_type: ReportType,
}

pub fn parse_cli_arguments() -> Result<AppArgs, ArgumentError> {
    let cli_args = CliArgs::parse();

    Ok(AppArgs {
        path: parse_path(cli_args.directory)?,
        hash_strategy: cli_args.algorithm,
        report_type: parse_output_type(cli_args.output)?,
    })
}

fn parse_path(path: String) -> Result<String, ArgumentError> {
    match Path::new(&path).try_exists() {
        Ok(true) => Ok(path),
        Ok(false) => Err(ArgumentError::InexistentFile(path)),
        Err(err) => Err(ArgumentError::IoError(err)),
    }
}

fn parse_output_type(output_path: Option<String>) -> Result<ReportType, ArgumentError> {
    if output_path.is_none() {
        return Ok(ReportType::Stdout);
    }

    match output_path.unwrap().trim() {
        output_path if output_path.ends_with(".csv") => Ok(ReportType::Csv(output_path.to_owned())),
        _ => Err(ArgumentError::InvalidOutputType),
    }
}

#[cfg(test)]
mod parse_output_type_tests {
    use sapphire_hash_core::report_type::ReportType;

    use crate::argument_parser::parse_output_type;

    #[test]
    fn test_none() {
        let result = parse_output_type(None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ReportType::Stdout);
    }

    #[test]
    fn test_csv() {
        let result = parse_output_type(Some("output.csv".to_string()));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ReportType::Csv("output.csv".to_string()));
    }

    #[test]
    fn test_with_full_path() {
        let result = parse_output_type(Some("/tmp/test/123/output.csv".to_string()));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ReportType::Csv("/tmp/test/123/output.csv".to_string()));
    }

    #[test]
    fn test_with_trailing_whitespace() {
        let result = parse_output_type(Some("   test/output.csv  ".to_string()));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ReportType::Csv("test/output.csv".to_string()));
    }

    #[test]
    fn test_with_invalid() {
        let result = parse_output_type(Some("output.txt".to_string()));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Unrecognized output format!");
    }
}
