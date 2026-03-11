#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use telemetry::logging::{HotRecorder, HotRouter, LogLevel, LogEvent};
use telemetry::logging::{StdoutSink, StderrSink};
use std::thread;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    // --- 1. Create the router
    let router_handle = HotRouter::new()
    .add_sink(StdoutSink)
    .start();

    // --- 2. Spawn threads with their own references
    let mut a_router_ref = router_handle.reference();
    let a = thread::spawn(move || {
        let _0 = HotRecorder::scope("Level 0", &mut a_router_ref);
        // thread::sleep(Duration::from_millis(10));
        {
            let _10 = HotRecorder::scope("Level 1 - 0", &mut a_router_ref);
            // thread::sleep(Duration::from_millis(10));
        }
        {
            let _11 = HotRecorder::scope("Level 1 - 1", &mut a_router_ref);
            // thread::sleep(Duration::from_millis(10));
            {
                let mut _20 = HotRecorder::scope("Level 2 - 0", &mut a_router_ref);
                // thread::sleep(Duration::from_millis(10));

                _20.log(LogLevel::Fatal, LogEvent::message("test_error"))
            }
        }
    });

    // --- 3. Wait for threads to finish (triggers Drop on recorders, sends logs)
    a.join().unwrap();
    router_handle.shutdown().await;
}