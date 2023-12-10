use std::collections::BTreeMap;

use crate::report::report_entry::{ReportEntry, ResultType};

use super::ReportOutput;

pub struct Stdout;

impl ReportOutput for Stdout {
    fn generate(&self, entries: &BTreeMap<String, ReportEntry>) {
        for (_, entry) in entries.iter() {
            println!("{}", self.format_text(entry));
        }
    }
}

impl Stdout {
    fn format_text(&self, entry: &ReportEntry) -> String {
        let path = &entry.path;

        match &entry.result {
            ResultType::File(Ok(hash)) => format!("{}\tHash: {}", path, hash),
            ResultType::Directory(Ok(())) => path.clone(),
            ResultType::Symlink => format!("{}\tSymlink", path),
            ResultType::SpecialFile => format!("{}\tSpecial File", path),
            ResultType::File(Err(err)) | ResultType::Directory(Err(err)) => {
                format!("{}\tError: {}", path, err)
            }
        }
    }
}
