mod common;
mod stdio;
mod file;

pub use self::{
    common::{Sink, format_log},
    stdio::{StdoutSink, StderrSink},
    file::FileSink,
};