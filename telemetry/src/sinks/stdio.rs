use crate::logging::{Log, LogLevel};
use super::common::{Sink, format_log};


// ── Sinks ─────────────────────────────────────────────────────────────────────

pub struct StdoutSink;

impl<const STR: usize> Sink<STR> for StdoutSink {
    fn write(&mut self, log: &Log<STR>) {
        println!("{}", format_log(log, true, true));
    }
}

pub struct StderrSink;

impl<const STR: usize> Sink<STR> for StderrSink {
    fn write(&mut self, log: &Log<STR>) {
        if matches!(log.level, LogLevel::Error | LogLevel::Fatal) {
            eprintln!("{}", format_log(log, true, false));
        }
    }
}