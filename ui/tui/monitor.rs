use std::io::{self, Read, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::time::Duration;
use std::thread;
use crossbeam::channel::Sender;

// ── Events ────────────────────────────────────────────────────────────────────

pub struct Log {
    pub path: String,
    pub timestamp: String,
    pub level: String,
    pub event: String,
    pub recorder_id: String,
    pub parent_recorder_id: String,
}

pub enum Event {
    Input(crossterm::event::KeyEvent),
    NewLog(Log),
}

// ── Monitors ──────────────────────────────────────────────────────────────────

pub struct FileMonitor {
    path: String,
    file: File,
    offset: u64,
    tx: Sender<Event>,
}

impl FileMonitor {
    pub fn new(path: &str, tx: Sender<Event>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(Self {
            path: path.to_string(),
            file,
            offset: 0,
            tx,
        })
    }

    pub fn start(mut self) {
        loop {
            self.poll();
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn poll(&mut self) {
        if self.file.seek(SeekFrom::Start(self.offset)).is_err() {
            return;
        }

        let mut new_content = String::new();
        match self.file.read_to_string(&mut new_content) {
            Ok(bytes_read) => self.offset += bytes_read as u64,
            Err(_) => return,
        }

        for line in new_content.lines() {
            if !line.is_empty() {
                if let Some(log) = self.parse_line(line) {
                    let _ = self.tx.send(Event::NewLog(log));
                }
            }
        }
    }

    fn parse_line(&self, line: &str) -> Option<Log> {
        let (timestamp, rest) = line.split_once(" UTC ")?;
        let (level_part, rest) = rest.split_once("] - ")?;
        let level = level_part.strip_prefix('[')?;
        let (event, rest) = rest.split_once(" --- ")?;
        let (recorder_id, parent_recorder_id) = rest.split_once(", Son of ")?;

        Some(Log {
            path: self.path.clone(),
            timestamp: timestamp.to_string(),
            level: level.to_string(),
            event: event.to_string(),
            recorder_id: recorder_id.to_string(),
            parent_recorder_id: parent_recorder_id.to_string(),
        })
    }
}