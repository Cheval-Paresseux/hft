use std::time::SystemTime;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogValue<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogEvent<'a> {
    // -- Standard Events ---
    Start(&'a str),
    End(&'a str),
    Duration(u64),
    Allocations(u64),
    Reallocations(u64),

    // --- User Events ---
    Message(&'a str),
    Metric(&'a str, LogValue<'a>),

    // --- System Events ---
    SystemName(Option<&'a str>),
    SystemKernelVersion(Option<&'a str>),
    SystemOsVersion(Option<&'a str>),
    SystemHostName(Option<&'a str>),
    SystemCpuArchitecture(&'a str),
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
pub enum LogLevel {
    SysInfo,
    GlobalInfo,
    ProcessInfo,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct Log<'a> {
    pub timestamp: u64,
    pub level: LogLevel,
    pub event: LogEvent<'a>,

    pub logger_id: Uuid,
    pub parent_logger_id: Option<Uuid>,
}

impl<'a> Log<'a> {
    pub fn new(level: LogLevel, event: LogEvent<'a>, logger_id: Uuid, parent_logger_id: Option<Uuid>) -> Self {
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

impl<'a> fmt::Display for Log<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{:?} [{:?}] - {:?} --- [id: {:?} | parent_id: {:?}]", 
            self.timestamp, self.level, self.event, self.logger_id, self.parent_logger_id
        )
    }
}



// ========== UNIT TESTS ==========
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn new_log() {
        let level = LogLevel::Info;
        let event = LogEvent::Start("test");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let log = Log::new(level, event, logger_id, parent_logger_id);
        assert_eq!(log.level, level);
        assert_eq!(log.event, event);
        assert_eq!(log.logger_id, logger_id);
        assert_eq!(log.parent_logger_id, parent_logger_id);
    }

    #[test]
    fn timestamp_is_nonzero() {
        let level = LogLevel::Info;
        let event = LogEvent::Start("test");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let log = Log::new(level, event, logger_id, parent_logger_id);
        assert!(log.timestamp > 0);
    }

    #[test]
    fn timestamps_are_monotonic() {
        let level = LogLevel::Info;
        let event = LogEvent::Start("test");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let a = Log::new(level, event, logger_id, parent_logger_id);
        let b = Log::new(level, event, logger_id, parent_logger_id);
        assert!(b.timestamp >= a.timestamp);
    }

    #[test]
    fn log_levels() {
        let levels = [
            LogLevel::SysInfo,
            LogLevel::GlobalInfo,
            LogLevel::ProcessInfo,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Fatal,
        ];
        let event = LogEvent::Start("test");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        for level in levels {
            let log = Log::new(level, event, logger_id, parent_logger_id);
            let output = format!("{}", log);
            assert!(!output.is_empty());
        }
    }

    #[test]
    fn event_message() {
        let level = LogLevel::Info;
        let event = LogEvent::Message("hello");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let log = Log::new(level, event, logger_id, parent_logger_id);
        assert!(matches!(log.event, LogEvent::Message("hello")));
    }

    #[test]
    fn event_metric() {
        let level = LogLevel::Info;
        let int_event = LogEvent::Metric("latency", LogValue::Int(42));
        let float_event = LogEvent::Metric("cpu", LogValue::Float(0.95));
        let bool_event = LogEvent::Metric("active", LogValue::Bool(true));
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());
        
        let int_log = Log::new(level, int_event, logger_id, parent_logger_id);
        let float_log = Log::new(level, float_event, logger_id, parent_logger_id);
        let bool_log = Log::new(level, bool_event, logger_id, parent_logger_id);
        assert!(matches!(int_log.event, LogEvent::Metric("latency", LogValue::Int(42))));
        assert!(matches!(float_log.event, LogEvent::Metric("cpu", LogValue::Float(0.95))));
        assert!(matches!(bool_log.event, LogEvent::Metric("active", LogValue::Bool(true))));
    }

    #[test]
    fn display() {
        let level = LogLevel::Info;
        let event = LogEvent::Message("hello");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let log = Log::new(level, event, logger_id, parent_logger_id);
        assert!(format!("{}", log).contains("Info"));
        assert!(format!("{}", log).contains("hello"));
        assert!(format!("{}", log).contains(&logger_id.to_string()));
        assert!(format!("{}", log).contains(&parent_logger_id.expect("").to_string()));
    }

    #[test]
    fn log_clone() {
        let level = LogLevel::Info;
        let event = LogEvent::Message("hello");
        let logger_id = Uuid::new_v4();
        let parent_logger_id = Some(Uuid::new_v4());

        let log = Log::new(level, event, logger_id, parent_logger_id);
        let cloned = log.clone();
        assert_eq!(log.timestamp, cloned.timestamp);
        assert_eq!(log.logger_id, cloned.logger_id);
    }
}