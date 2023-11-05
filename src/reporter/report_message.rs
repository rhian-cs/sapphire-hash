use super::report_entry::ReportEntry;

#[derive(Debug)]
pub enum ReportMessage {
    Message(ReportEntry),
    EndTransmission,
}
