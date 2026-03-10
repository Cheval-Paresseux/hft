use std::time::SystemTime;
use std::fmt;
use uuid::Uuid;
use arrayvec::ArrayString;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    SysInfo,
    GlobalInfo,
    ProcessInfo,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogValue<const CAP: usize> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(ArrayString<CAP>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogEvent<const CAP: usize> {
    // -- Standard Events ---
    Start(&'static str),
    End(&'static str),
    Duration(u64),
    Allocations(u64),
    Reallocations(u64),

    // --- User Events ---
    Message(ArrayString<CAP>),
    Metric(ArrayString<CAP>, LogValue<CAP>),

    // --- System Events ---
    SystemName(Option<ArrayString<CAP>>),
    SystemKernelVersion(Option<ArrayString<CAP>>),
    SystemOsVersion(Option<ArrayString<CAP>>),
    SystemHostName(Option<ArrayString<CAP>>),
    SystemCpuArchitecture(ArrayString<CAP>),
    SystemCoreCount(Option<usize>),
    SystemBootTime(u64),
    SystemUptime(u64),
    SystemTotalMemory(u64),
    SystemTotalSwap(u64),

    // --- Global Events ---
    GlobalAvailableMemory(u64),
    GlobalUsedMemory(u64),
    GlobalFreeSwap(u64),
    GlobalUsedSwap(u64),
    GlobalCpuUsage(f32),
    GlobalLoadAverage(f64, f64, f64),

    // --- Process Events ---
    ProcessMemory(u64),
    ProcessVirtualMemory(u64),
    ProcessStartTime(u64),
    ProcessRunTime(u64),
    ProcessCpuUsage(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Log<const CAP: usize> {
    pub timestamp: u64,
    pub level: LogLevel,
    pub event: LogEvent<CAP>,

    pub logger_id: Uuid,
    pub parent_logger_id: Option<Uuid>,
}

impl<const CAP: usize> Log<CAP> {
    pub fn new(level: LogLevel, event: LogEvent<CAP>, logger_id: Uuid, parent_logger_id: Option<Uuid>) -> Self {
        let timestamp: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64;

        Self {
            timestamp,
            level,
            event,

            logger_id,
            parent_logger_id,
        }
    }
}

impl<const CAP: usize> fmt::Display for Log<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{:?} [{:?}] - {:?} --- [id: {:?} | parent_id: {:?}]", 
            self.timestamp, self.level, self.event, self.logger_id, self.parent_logger_id
        )
    }
}