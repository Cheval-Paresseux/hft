use super::super::log::{Log, LogLevel};
use super::common::{Sink, format_log};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io;
use uuid::Uuid;
use chrono::{Utc, TimeZone};

// ── Sinks ─────────────────────────────────────────────────────────────────────

pub struct FileSink {
    file: File,
}

impl FileSink {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self { file })
    }
}

impl<const STR: usize> Sink<STR> for FileSink {
    fn write(&mut self, log: &Log<STR>) {
        writeln!(self.file, "{}", format_log(log, true, false)).ok();
    }
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    
}