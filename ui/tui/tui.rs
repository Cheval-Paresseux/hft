use super::monitor::{FileMonitor, Event, Log};
use super::widget::{MENU_ITEMS};
use std::io;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use crossbeam::channel::{Sender, Receiver, bounded};
use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{self, KeyCode, KeyEventKind, Event as CrosstermEvent};

// ── TUI ───────────────────────────────────────────────────────────────────────

pub struct TelemetryTUI {
    pub tx: Sender<Event>,
    pub rx: Receiver<Event>,
    pub exit: bool,
    pub selected: usize,
    pub applications: HashMap<String, Vec<String>>,
}

impl TelemetryTUI {
    pub fn new() -> Self {
        let (tx, rx) = bounded(10_000);
        Self {
            tx,
            rx,
            exit: false,
            selected: 0,
            applications: HashMap::new(),
        }
    }

    pub fn add_application(&mut self, path: &str) -> io::Result<()> {
        let tx = self.tx.clone();
        let monitor = FileMonitor::new(path, tx)?;
        self.applications.insert(path.to_string(), Vec::new());
        thread::spawn(move || monitor.start());
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tx_input = self.tx.clone();
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(CrosstermEvent::Key(key)) = event::read() {
                    let _ = tx_input.send(Event::Input(key));
                }
            }
        });

        while !self.exit {
            while let Ok(ev) = self.rx.try_recv() {
                match ev {
                    Event::NewLog(log) => self.handle_new_log(log),
                    Event::Input(key) => self.handle_key_event(key)?,
                }
            }
            terminal.draw(|frame| self.draw(frame))?;
            thread::sleep(Duration::from_millis(16));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Up => self.selected = self.selected.saturating_sub(1),
                KeyCode::Down => {
                    self.selected = (self.selected + 1).min(MENU_ITEMS.len() - 1)
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_new_log(&mut self, log: Log) {
        let entry = self.applications.entry(log.path.clone()).or_default();
        let line = format!(
            "[{}] {} | {} --- id:{} parent:{}",
            log.level, log.timestamp, log.event,
            log.recorder_id, log.parent_recorder_id
        );
        entry.push(line);
        if entry.len() > 1000 {
            entry.remove(0);
        }
    }
}

