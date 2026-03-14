use super::log::Log;
use crate::sinks::Sink;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crossbeam::channel::{Sender, Receiver, bounded};
use tokio::{self, task::JoinHandle};

// ── Router ────────────────────────────────────────────────────────────────────

pub struct Router<const CAP: usize, const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub rx: Receiver<Log<STR>>,
    sinks: Vec<Box<dyn Sink<STR>>>,
}

impl<const CAP: usize, const STR: usize> Router<CAP, STR> {
    pub fn new() -> Self {
        let (tx, rx) = bounded(CAP);
        Self { tx, rx, sinks: Vec::new() }
    }

    pub fn add_sink(mut self, sink: impl Sink<STR>) -> Self {
        self.sinks.push(Box::new(sink));
        self
    }

    pub fn start(mut self) -> RouterHandle<STR> {
        let tx = self.tx.clone();
        let join = tokio::task::spawn_blocking(move || {
            while let Ok(log) = self.rx.recv() {
                for sink in &mut self.sinks {
                    sink.write(&log);
                }
            }
        });
        RouterHandle { tx, join }
    }
}

impl<const CAP: usize, const STR: usize> Default for Router<CAP, STR> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Router Handle ─────────────────────────────────────────────────────────────

pub struct RouterHandle<const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub join: JoinHandle<()>,
}

impl<const STR: usize> RouterHandle<STR> {
    pub fn reference(&self) -> RouterReference<STR> {
        RouterReference::new(self.tx.clone())
    }

    pub async fn shutdown(self) {
        drop(self.tx);
        self.join.await.unwrap();
    }
}

// ── Router Reference ──────────────────────────────────────────────────────────

pub struct RouterReference<const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub recorders_context: Arc<Mutex<Vec<Uuid>>>,
}

impl<const STR: usize> RouterReference<STR> {
    pub fn new(tx: Sender<Log<STR>>) -> Self {
        Self {
            tx,
            recorders_context: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push_recorder(&mut self, id: Uuid) {
        self.recorders_context.lock().unwrap().push(id);
    }

    pub fn pop_recorder(&self) {
        self.recorders_context.lock().unwrap().pop();
    }

    pub fn current(&self) -> Option<Uuid> {
        self.recorders_context.lock().unwrap().last().copied()
    }
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{Log, LogLevel, LogEvent};
    use std::sync::{Arc, Mutex};
    use crossbeam::channel::unbounded;
    use uuid::Uuid;

    #[derive(Clone)]
    struct MemorySink<const STR: usize> {
        logs: Arc<Mutex<Vec<Log<STR>>>>,
    }

    impl<const STR: usize> MemorySink<STR> {
        fn new() -> Self {
            Self { logs: Arc::new(Mutex::new(Vec::new())) }
        }
    }

    impl<const STR: usize> Sink<STR> for MemorySink<STR> {
        fn write(&mut self, log: &Log<STR>) {
            self.logs.lock().unwrap().push(log.clone());
        }
    }

    #[tokio::test]
    async fn test_router_creation() {
        let sink = MemorySink::new();
        let router_handle = Router::<16, 32>::new()
            .add_sink(sink)
            .start();

        assert!(router_handle.tx.clone().send(Log::new(
            LogLevel::Info,
            LogEvent::message("test"),
            Uuid::new_v4(),
            None
        )).is_ok());

        router_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_logs_sent_to_sink() {
        let sink = MemorySink::new();
        let logs_ref = sink.logs.clone();
        let router_handle = Router::<16, 32>::new()
            .add_sink(sink)
            .start();

        let log = Log::new(
            LogLevel::Info,
            LogEvent::message("hello"),
            Uuid::new_v4(),
            None
        );

        router_handle.tx.send(log).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let collected = logs_ref.lock().unwrap();
        assert_eq!(collected.len(), 1);
        assert!(matches!(collected[0].event, LogEvent::Message(_)));

        router_handle.shutdown().await;
    }

    #[test]
    fn test_router_reference() {
        let (tx, _rx) = unbounded();
        let mut refe = RouterReference::<32>::new(tx);
        assert!(refe.current().is_none());

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        refe.push_recorder(id1);
        assert_eq!(refe.current(), Some(id1));

        refe.push_recorder(id2);
        assert_eq!(refe.current(), Some(id2));

        refe.pop_recorder();
        assert_eq!(refe.current(), Some(id1));

        refe.pop_recorder();
        assert!(refe.current().is_none());
    }
}