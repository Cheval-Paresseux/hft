use crate::logging::Log;
use super::common::{Sink, format_log};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io;

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
    use super::*;
    use crate::logging::{Log, LogLevel, LogEvent};
    use uuid::Uuid;
    use std::fs;
    use std::io::Read;

    fn make_log<const STR: usize>(level: LogLevel, msg: &str) -> Log<STR> {
        Log::new(level, LogEvent::message(msg), Uuid::new_v4(), None)
    }

    fn tmp_path(name: &str) -> String {
        format!("/tmp/filesink_test_{name}_{}.log", Uuid::new_v4())
    }

    fn read_file(path: &str) -> String {
        let mut f = fs::File::open(path).expect("log file should exist");
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents
    }

    #[test]
    fn test_filesink_creates_file() {
        let path = tmp_path("creates");
        assert!(!fs::exists(&path).unwrap_or(false), "file should not exist before sink creation");

        let _sink = FileSink::new(&path).expect("sink creation should succeed");

        assert!(fs::exists(&path).unwrap_or(false), "file should exist after sink creation");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_filesink_writes_log_entry() {
        let path = tmp_path("writes");
        let mut sink = FileSink::new(&path).unwrap();

        let log = make_log::<32>(LogLevel::Info, "hello from filesink");
        sink.write(&log);

        let contents = read_file(&path);
        assert!(contents.contains("hello from filesink"), "log message should appear in file, got:\n{contents}");
        assert!(contents.contains("UTC"), "timestamp should be date-formatted, got:\n{contents}");

        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_filesink_appends_multiple_entries() {
        let path = tmp_path("appends");
        let mut sink = FileSink::new(&path).unwrap();

        sink.write(&make_log::<32>(LogLevel::Info, "first"));
        sink.write(&make_log::<32>(LogLevel::Warn, "second"));
        sink.write(&make_log::<32>(LogLevel::Error, "third"));

        let contents = read_file(&path);
        let line_count = contents.lines().count();

        assert_eq!(line_count, 3, "expected 3 lines, got {line_count}:\n{contents}");
        assert!(contents.contains("first"));
        assert!(contents.contains("second"));
        assert!(contents.contains("third"));

        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_filesink_appends_across_instances() {
        let path = tmp_path("across_instances");

        let mut sink_a = FileSink::new(&path).unwrap();
        sink_a.write(&make_log::<32>(LogLevel::Info, "from first instance"));
        drop(sink_a);

        let mut sink_b = FileSink::new(&path).unwrap();
        sink_b.write(&make_log::<32>(LogLevel::Info, "from second instance"));
        drop(sink_b);

        let contents = read_file(&path);
        assert!(contents.contains("from first instance"), "first write should be preserved");
        assert!(contents.contains("from second instance"), "second write should be appended");
        assert_eq!(contents.lines().count(), 2, "both lines should exist without truncation");

        fs::remove_file(&path).ok();
    }
}