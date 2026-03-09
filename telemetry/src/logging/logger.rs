use uuid::Uuid;
use std::time::Instant;
use super::metrics::*;
use super::log::{Log, LogLevel, LogEvent};

const HOT_LOGS_CAPACITY: usize = 8; // 5 standard logs, 3 of margin for user
const DEFAULT_LOGS_CAPACITY: usize = 128;

pub struct Logger<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub parent_id: Option<Uuid>,
    logs: Vec<Log<'a>>,

    start_now: Instant,
    start_allocations: u64,
    start_reallocations: u64,
}

impl<'a> Logger<'a> {
    fn new(name: &'a str, parent_id: Option<Uuid>, capacity: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            parent_id,
            logs: Vec::with_capacity(capacity),

            start_now: Instant::now(),
            start_allocations: ALLOCATOR.allocations(),
            start_reallocations: ALLOCATOR.reallocations(),
        }
    }

    pub fn log(&mut self, level: LogLevel, event: LogEvent<'a>) {
        debug_assert!(
            self.logs.len() < self.logs.capacity(),
            "Logger '{}' exceeded capacity ({}) — reallocation would occur",
            self.name,
            self.logs.capacity()
        );
        self.logs.push(Log::new(level, event, self.id, self.parent_id));
    }

    pub fn scope(name: &'a str, parent_id: Option<Uuid>) -> Self {
        let mut logger = Self::new(name, parent_id, DEFAULT_LOGS_CAPACITY);
        logger.log(LogLevel::Info, LogEvent::Start(name));

        logger
    }

    pub fn with_system_information(mut self) -> Self {
        let system_information: SystemInformation = system_information();

        self.log(LogLevel::SysInfo, LogEvent::SystemName(system_information.name));
        self.log(LogLevel::SysInfo, LogEvent::SystemKernelVersion(system_information.kernel_version));
        self.log(LogLevel::SysInfo, LogEvent::SystemOsVersion(system_information.os_version));
        self.log(LogLevel::SysInfo, LogEvent::SystemHostName(system_information.host_name));
        self.log(LogLevel::SysInfo, LogEvent::SystemCpuArchitecture(system_information.cpu_architecture));
        self.log(LogLevel::SysInfo, LogEvent::SystemCoreCount(system_information.core_count));
        self.log(LogLevel::SysInfo, LogEvent::SystemBootTime(system_information.boot_time));
        self.log(LogLevel::SysInfo, LogEvent::SystemUptime(system_information.uptime));
        self.log(LogLevel::SysInfo, LogEvent::SystemTotalMemory(system_information.total_memory));
        self.log(LogLevel::SysInfo, LogEvent::SystemTotalSwap(system_information.total_swap));

        self
    }

    pub fn with_global_information(mut self) -> Self {
        let global_information: GlobalInformation = global_information();
        let (one, five, fifteen) = global_information.load_average;

        self.log(LogLevel::GlobalInfo, LogEvent::GlobalAvailableMemory(global_information.available_memory));
        self.log(LogLevel::GlobalInfo, LogEvent::GlobalUsedMemory(global_information.used_memory));
        self.log(LogLevel::GlobalInfo, LogEvent::GlobalFreeSwap(global_information.free_swap));
        self.log(LogLevel::GlobalInfo, LogEvent::GlobalUsedSwap(global_information.used_swap));
        self.log(LogLevel::GlobalInfo, LogEvent::GlobalCpuUsage(global_information.cpu_usage));
        self.log(LogLevel::GlobalInfo, LogEvent::GlobalLoadAverage(one, five, fifteen));

        self
    }

    pub fn with_process_information(mut self) -> Self {
        let process_information: ProcessInformation = process_information();
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessMemory(process_information.memory));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessVirtualMemory(process_information.virtual_memory));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessStartTime(process_information.start_time));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessRunTime(process_information.run_time));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessCpuUsage(process_information.cpu_usage));

        self
    }

    pub fn get_logs(&self) -> &[Log<'a>] {
        &self.logs
    }
}

impl Logger<'static> {
    pub fn hot_scope(name: &'static str, parent_id: Option<Uuid>) -> Self {
        let mut logger = Self::new(name, parent_id, HOT_LOGS_CAPACITY);
        logger.log(LogLevel::Info, LogEvent::Start(name));
        logger
    }

    pub fn hot_log(&mut self, level: LogLevel, event: LogEvent<'static>) {
        self.log(level, event);
    }
}

impl<'a> Drop for Logger<'a> {
    fn drop(&mut self) {
        let duration = self.start_now.elapsed().as_nanos() as u64;
        let allocations = ALLOCATOR.allocations() - self.start_allocations;
        let reallocations = ALLOCATOR.reallocations() - self.start_reallocations;

        self.log(LogLevel::Info, LogEvent::End(self.name));
        self.log(LogLevel::Info, LogEvent::Duration(duration));
        self.log(LogLevel::Info, LogEvent::Allocations(allocations));
        self.log(LogLevel::Info, LogEvent::Reallocations(reallocations));

        for log in self.get_logs() {
            println!("{}", log);
        }
    }
}



// --- UNIT TESTS ---
