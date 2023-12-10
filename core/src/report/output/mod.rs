use std::collections::BTreeMap;

use crate::report_type::ReportType;

use super::report_entry::ReportEntry;

pub mod stdout;

pub trait ReportOutput {
    fn generate(&self, entries: &BTreeMap<String, ReportEntry>);
}

pub fn report_output_for(report_type: ReportType) -> Box<dyn ReportOutput> {
    match report_type {
        ReportType::Stdout => Box::new(stdout::Stdout),
    }
}
