use std::collections::BTreeMap;

use csv::Writer;

use crate::report::report_entry::{ReportEntry, ResultType};

use super::ReportOutput;

const HEADERS: [&str; 3] = ["Path", "Hash", "Details"];

pub struct Csv {
    output_path: String,
}

impl ReportOutput for Csv {
    fn generate(
        &self,
        entries: BTreeMap<String, crate::report::report_entry::ReportEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::from_path(&self.output_path)?;

        writer.write_record(HEADERS)?;
        for (_, entry) in entries.into_iter() {
            writer.write_record(self.format_csv(entry))?;
        }

        writer.flush()?;
        Ok(())
    }
}

impl Csv {
    pub fn new(output_path: String) -> Self {
        Self { output_path }
    }

    fn format_csv(&self, entry: ReportEntry) -> [String; 3] {
        let path = entry.path;
        let empty_string = String::from("");

        match entry.result {
            ResultType::File(Ok(hash)) => [path, hash, empty_string],
            ResultType::Directory(Ok(())) => [path, empty_string, "Directory".to_string()],
            ResultType::Symlink => [path, empty_string, "Symlink".to_string()],
            ResultType::SpecialFile => [path, empty_string, "Special File".to_string()],
            ResultType::File(Err(err)) | ResultType::Directory(Err(err)) => {
                [path, empty_string, format!("Error: {err}")]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs, io};

    use crate::report::{
        output::{csv::Csv, ReportOutput},
        report_entry::{ReportEntry, ResultType},
    };

    #[test]
    fn test_csv_report() {
        let output_path = "tmp/tests/test_csv_report.csv".to_string();
        fs::create_dir_all("tmp/tests").unwrap();

        let example_directory_error = io::Error::new(io::ErrorKind::InvalidData, "Error reading directory".to_string());
        let example_file_error = io::Error::new(io::ErrorKind::InvalidData, "Error reading file".to_string());

        // Setup
        let mut report_entries = BTreeMap::new();
        add_test_entry(&mut report_entries, "/test/6_directory", ResultType::Directory(Ok(())));
        add_test_entry(
            &mut report_entries,
            "/test/5_error_directory",
            ResultType::Directory(Err(example_directory_error)),
        );
        add_test_entry(&mut report_entries, "/test/3_symlink", ResultType::Symlink);
        add_test_entry(&mut report_entries, "/test/2_special_file", ResultType::SpecialFile);
        add_test_entry(
            &mut report_entries,
            "/test/1_successful_file",
            ResultType::File(Ok(
                "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3".to_string()
            )),
        );
        add_test_entry(
            &mut report_entries,
            "/test/4_error_file",
            ResultType::File(Err(example_file_error)),
        );

        // Result assertion
        let result = Csv::new(output_path.clone()).generate(report_entries);
        assert!(result.is_ok());

        // Output assertion
        let expected_lines = vec![
            "Path,Hash,Details",
            "/test/1_successful_file,315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3,",
            "/test/2_special_file,,Special File",
            "/test/3_symlink,,Symlink",
            "/test/4_error_file,,Error: Error reading file",
            "/test/5_error_directory,,Error: Error reading directory",
            "/test/6_directory,,Directory",
        ];

        let file = fs::read_to_string(output_path).unwrap();
        let lines: Vec<&str> = file.lines().collect();
        assert_eq!(lines, expected_lines);
    }

    fn add_test_entry(btree_map: &mut BTreeMap<String, ReportEntry>, path: &str, result: ResultType) {
        let path = path.to_owned();

        btree_map.insert(path.clone(), ReportEntry { path, result });
    }
}
