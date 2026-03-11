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
pub enum LogValue<const STR: usize> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(ArrayString<STR>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogEvent<const STR: usize> {
    // -- Standard Events ---
    Start(&'static str),
    End(&'static str),
    Duration(u64),
    Allocations(u64),
    Reallocations(u64),

    // --- User Events ---
    Message(ArrayString<STR>),
    Metric(ArrayString<STR>, LogValue<STR>),

    // --- System Events ---
    SystemName(Option<ArrayString<STR>>),
    SystemKernelVersion(Option<ArrayString<STR>>),
    SystemOsVersion(Option<ArrayString<STR>>),
    SystemHostName(Option<ArrayString<STR>>),
    SystemCpuArchitecture(ArrayString<STR>),
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

impl<const STR: usize> LogEvent<STR> {
    fn to_array_string(s: &str) -> ArrayString<STR> {
        let mut a = ArrayString::new();
        let end = s.floor_char_boundary(STR.min(s.len()));
        a.push_str(&s[..end]);
        a
    }

    pub fn message(s: &str) -> Self {
        Self::Message(Self::to_array_string(s))
    }

    pub fn metric(key: &str, value: LogValue<STR>) -> Self {
        Self::Metric(Self::to_array_string(key), value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Log<const STR: usize> {
    pub timestamp: u64,
    pub level: LogLevel,
    pub event: LogEvent<STR>,

    pub logger_id: Uuid,
    pub parent_logger_id: Option<Uuid>,
}

impl<const STR: usize> Log<STR> {
    pub fn new(level: LogLevel, event: LogEvent<STR>, logger_id: Uuid, parent_logger_id: Option<Uuid>) -> Self {
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

impl<const STR: usize> fmt::Display for Log<STR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{:?} [{:?}] - {:?} --- [id: {:?} | parent_id: {:?}]", 
            self.timestamp, self.level, self.event, self.logger_id, self.parent_logger_id
        )
    }
}

