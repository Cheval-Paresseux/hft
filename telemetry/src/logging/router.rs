use super::log::Log;
use super::sinks::Sink;
use uuid::Uuid;
use crossbeam::channel::{Sender, Receiver, bounded};
use std::sync::{Arc, Mutex};
use tokio;
use tokio::task::JoinHandle;

pub struct Reference<const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub recorders_context: Arc<Mutex<Vec<Uuid>>>,
}

impl<const STR: usize> Reference<STR> {
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
            loop {
                match self.rx.recv() {
                    Ok(log) => {
                        for sink in &mut self.sinks {
                            sink.write(&log);
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        RouterHandle { tx, join }
    }
}

pub struct RouterHandle<const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub join: JoinHandle<()>,
}

impl<const STR: usize> RouterHandle<STR> {
    pub fn reference(&self) -> Reference<STR> {
        Reference::new(self.tx.clone())
    }

    pub async fn shutdown(self) {
        drop(self.tx);
        self.join.await.unwrap();
    }
}