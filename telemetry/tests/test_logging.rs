use telemetry::{
    HotRouter, HotRecorder, RouterReference,
    LogLevel, LogEvent,
    Sink, StderrSink, FileSink, format_log
};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::Path;

// ── In-memory sink ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct MemorySink {
    logs: Arc<Mutex<Vec<String>>>,
}

impl MemorySink {
    fn new() -> Self {
        Self { logs: Arc::new(Mutex::new(Vec::new())) }
    }

    fn logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }
}

impl<const STR: usize> Sink<STR> for MemorySink {
    fn write(&mut self, log: &telemetry::Log<STR>) {
        let s = format_log(log, true, true);
        self.logs.lock().unwrap().push(s);
    }
}

// ── Test ───────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn api_compiles() {
    let router = HotRouter::new().start();
    router.shutdown().await;
}

#[tokio::test]
async fn standard_sinks() {
    let dir = "/tmp/telemetry/test_logging";
    let _ = std::fs::remove_dir_all(dir); 
    std::fs::create_dir_all(dir).unwrap();

    let path = format!("{}/log.txt", dir);

    let handle = HotRouter::new()
        .add_sink(StderrSink)
        .add_sink(FileSink::new(&path).unwrap())
        .start();

    let reference = handle.reference();

    nested_logs(reference);

    handle.shutdown().await;

    assert!(Path::new(&path).exists());

    let file = std::fs::read_to_string(&path).unwrap();
    assert!(file.contains("test_log"));
}

#[tokio::test]
async fn nested_scopes_build_tree() {
    let sink = MemorySink::new();
    let sink_ref = sink.clone();

    let handle = HotRouter::new()
        .add_sink(sink_ref)
        .start();
    let reference = handle.reference();

    nested_logs(reference);
    handle.shutdown().await;

    let logs = sink.logs();
    println!("{:#?}", logs);

    assert!(logs.iter().any(|l| l.contains("test_log")));
    assert!(logs.iter().any(|l| l.contains("Level 0")));
    assert!(logs.iter().any(|l| l.contains("Level 1 - 0")));
    assert!(logs.iter().any(|l| l.contains("Level 1 - 1")));
    assert!(logs.iter().any(|l| l.contains("Level 2 - 0")));
}

// ── Utils ─────────────────────────────────────────────────────────────────────

fn nested_logs(mut router_ref: RouterReference<32>) {
    let a = thread::spawn(move || {
        let _0 = HotRecorder::scope("Level 0", &mut router_ref);
        thread::sleep(Duration::from_millis(10));

        {
            let _10 = HotRecorder::scope("Level 1 - 0", &mut router_ref);
            thread::sleep(Duration::from_millis(10));
        }

        {
            let _11 = HotRecorder::scope("Level 1 - 1", &mut router_ref);
            thread::sleep(Duration::from_millis(10));

            {
                let mut _20 = HotRecorder::scope("Level 2 - 0", &mut router_ref);
                thread::sleep(Duration::from_millis(10));

                _20.log(LogLevel::Info, LogEvent::message("test_log"));
            }
        }
    });

    a.join().unwrap();
}