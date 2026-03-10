use super::log::Log;
use uuid::Uuid;
use crossbeam::channel::{Sender, Receiver, bounded};
use std::sync::{Arc, Mutex};

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

pub struct Router<const STR: usize> {
    pub tx: Sender<Log<STR>>,
    pub rx: Receiver<Log<STR>>,
}

impl<const STR: usize> Router<STR> {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);
        Self { tx, rx }
    }

    pub fn reference(&self) -> Reference<STR> {
        Reference::new(self.tx.clone())
    }

    pub fn print_logs(&self) {
        while let Ok(log) = self.rx.try_recv() {
            println!("{}", log);
        }
    }
}