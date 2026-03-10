#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use telemetry::logging::{HotRecorder, HotRouter};
use std::thread;
use std::time::Duration;

fn main() {
    // --- 1. Create the router
    let router = HotRouter::new();

    // --- 2. Spawn threads with their own references
    let mut a_router_ref = router.reference();
    let a = thread::spawn(move || {
        let _0 = HotRecorder::scope("Level 0", &mut a_router_ref);
        thread::sleep(Duration::from_millis(10));
        {
            let _10 = HotRecorder::scope("Level 1 - 0", &mut a_router_ref);
            thread::sleep(Duration::from_millis(10));
        }
        {
            let _11 = HotRecorder::scope("Level 1 - 1", &mut a_router_ref);
            thread::sleep(Duration::from_millis(10));
            {
                let _20 = HotRecorder::scope("Level 2 - 0", &mut a_router_ref);
                thread::sleep(Duration::from_millis(10));
            }
        }
    });

    //let mut b_router_ref = router.reference();
    //let b = thread::spawn(move || {
    //    let _b_recorder = HotRecorder::scope("Thread b", &mut b_router_ref);
    //    thread::sleep(Duration::from_millis(10));
    //});

    // --- 3. Wait for threads to finish (triggers Drop on recorders, sends logs)
    a.join().unwrap();
    //b.join().unwrap();

    // --- 4. Drain and print all logs
    router.print_logs();
}