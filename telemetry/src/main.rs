#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

// ── Main TUI ─────────────────────────────────────────────────────────────────────

use std::io;
use ratatui::{DefaultTerminal, Frame, widgets::Widget, text::Line, style::Stylize};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = TelemetryTUI { exit: false };
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct TelemetryTUI {
    exit: bool,
}

impl TelemetryTUI {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_key_event(&mut self)
}

impl Widget for &TelemetryTUI {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Line::from("Process overview").bold().render(area, buf);
    }
}

// ── Main Logs ─────────────────────────────────────────────────────────────────────

// use telemetry::logging::{HotRouter, StdoutSink, StderrSink, FileSink};
// use telemetry::logging::{HotRecorder, RouterReference, LogLevel, LogEvent};
// use std::time::Duration;
// use std::thread;
// use tokio;

// #[tokio::main]
// async fn main() {
//     // --- Logger ---
//     let router_handle = HotRouter::new()
//         .add_sink(StderrSink)
//         .add_sink(FileSink::new("/tmp/hft_logs").unwrap())
//         .start();

//     let router_ref = router_handle.reference();
//     foo_with_logs(router_ref);                

//     router_handle.shutdown().await;
// }

// fn foo_with_logs(mut router_ref: RouterReference<32>) {
//     let a = thread::spawn(move || {
//         let _0 = HotRecorder::scope("Level 0", &mut router_ref);
//         thread::sleep(Duration::from_millis(10));
//         {
//             let _10 = HotRecorder::scope("Level 1 - 0", &mut router_ref);
//             thread::sleep(Duration::from_millis(10));
//         }
//         {
//             let _11 = HotRecorder::scope("Level 1 - 1", &mut router_ref);
//             thread::sleep(Duration::from_millis(10));
//             {
//                 let mut _20 = HotRecorder::scope("Level 2 - 0", &mut router_ref);
//                 thread::sleep(Duration::from_millis(10));
//                 _20.log(LogLevel::Fatal, LogEvent::message("test_error"))
//             }
//         }
//     });

//     a.join().unwrap();
// }