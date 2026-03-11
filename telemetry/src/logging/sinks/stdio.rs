use super::super::log::{Log, LogLevel};
use super::sink::Sink;

pub struct StdoutSink;

impl<const STR: usize> Sink<STR> for StdoutSink {
    fn write(&mut self, log: &Log<STR>) {
        println!("{}", log);
    }
}

pub struct StderrSink;

impl<const STR: usize> Sink<STR> for StderrSink {
    fn write(&mut self, log: &Log<STR>) {
        if matches!(log.level, LogLevel::Error | LogLevel::Fatal)  {
            eprintln!("{}", log); 
        }
    }
}