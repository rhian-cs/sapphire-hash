use std::time::Duration;

pub enum UiMessage {
    ReporterFinish(i32 /* processed files count */),
    ExecutionFinish(Duration /* elapsed time */),
}
