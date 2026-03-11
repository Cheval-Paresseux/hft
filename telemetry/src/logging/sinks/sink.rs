use super::super::log::Log;

pub trait Sink<const STR: usize>: Send + 'static {
    fn write(&mut self, log: &Log<STR>);
}

