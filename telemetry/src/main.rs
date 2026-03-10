use telemetry::logging::{HotLogger};

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main () {
    let logger_name = "Logger1";
    let mut _logger = HotLogger::scope(logger_name, None);
}
