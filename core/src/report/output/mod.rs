use std::{collections::BTreeMap, error::Error};

use crate::report_type::ReportType;

use super::report_entry::ReportEntry;

mod csv;
mod stdout;

pub trait ReportOutput {
    fn generate(&self, entries: BTreeMap<String, ReportEntry>) -> Result<(), Box<dyn Error>>;
}

pub fn report_output_for(report_type: ReportType) -> Box<dyn ReportOutput> {
    match report_type {
        ReportType::Stdout => Box::new(stdout::Stdout),
        ReportType::Csv(output_path) => Box::new(csv::Csv::new(output_path)),
    }
}
