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

    pub fn is_file(&self) -> bool {
        matches!(self.result, ResultType::File(_))
    }
}
