#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use telemetry::logging::{HotRecorder, DefaultRecorder, FullRecorder};

fn main () {
    let mut _r = FullRecorder::scope("Root", None)
    .with_system_information()
    .with_global_information()
    .with_process_information();
    {
        let mut _n = DefaultRecorder::scope("Nested", None)
        .with_process_information();
    }
}
