#[derive(PartialEq, Debug)]
pub enum ReportType {
    Stdout,
    Csv(String),
}
