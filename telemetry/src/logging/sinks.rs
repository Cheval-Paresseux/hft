mod common;
mod stdio;
mod file;

pub use common::{Sink, format_log};
pub use stdio::{StdoutSink, StderrSink};
pub use file::FileSink;