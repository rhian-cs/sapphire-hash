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

    pub fn format_text(&self) -> String {
        match &self.result {
            ResultType::File(Ok(hash)) => format!("{}\tHash: {}", self.path, hash),
            ResultType::Directory(Ok(())) => self.path.clone(),
            ResultType::Symlink => format!("{}\tSymlink", self.path),
            ResultType::SpecialFile => format!("{}\tSpecial File", self.path),
            ResultType::File(Err(err)) | ResultType::Directory(Err(err)) => {
                format!("{}\tError: {}", self.path, err)
            }
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self.result, ResultType::File(_))
    }
}
