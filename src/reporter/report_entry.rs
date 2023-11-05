use std::io;

#[derive(Debug)]
pub enum ResultType {
    File(Result<String, io::Error>),
    Directory(Result<(), io::Error>),
}

#[derive(Debug)]
pub struct ReportEntry {
    pub path: String,
    pub result: ResultType,
    pub is_directory: bool,
}

impl ReportEntry {
    pub fn format_text(&self) -> String {
        match &self.result {
            ResultType::File(Ok(hash)) => format!("{}\tHash: {}", self.path, hash),
            ResultType::Directory(Ok(())) => self.path.clone(),
            ResultType::File(Err(err)) | ResultType::Directory(Err(err)) => {
                format!("{}\tError: {}", self.path, err)
            }
        }
    }
}
