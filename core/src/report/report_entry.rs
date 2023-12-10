use std::io;

#[derive(Debug)]
pub enum ResultType {
    File(Result<String, io::Error>),
    Directory(Result<(), io::Error>),
    Symlink,
    SpecialFile,
}

#[derive(Debug)]
pub struct ReportEntry {
    pub path: String,
    pub result: ResultType,
}

impl ReportEntry {
    pub fn new(path: String, result: ResultType) -> Self {
        Self { path, result }
    }

    pub fn is_file(&self) -> bool {
        matches!(self.result, ResultType::File(_))
    }
}
