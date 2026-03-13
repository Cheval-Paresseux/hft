#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use telemetry::tui::TelemetryTUI;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = TelemetryTUI::new();

    for path in std::env::args().skip(1) {
        if let Err(e) = app.add_application(&path) {
            eprintln!("Could not watch {path}: {e}");
        }
    }

    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}