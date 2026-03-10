use super::log::{Log, LogLevel, LogEvent};
use super::metrics::*;
use uuid::Uuid;
use std::time::Instant;
use arrayvec::ArrayVec;

const HOT_VEC_CAP: usize = 8; 
const DEFAULT_VEC_CAP: usize = 32;
const FULL_VEC_CAP: usize = 64;

const HOT_STR_CAP: usize = 32; // Short identifiers only: asset names (e.g. "EUR-USD"), order sides ("BUY"/"SELL"), statuses ("PARTIALLY_FILLED")
const DEFAULT_STR_CAP: usize = 128;
const FULL_STR_CAP: usize = 512;

pub struct Recorder<const VEC: usize, const STR: usize> {
    pub id: Uuid,
    pub name: &'static str,
    pub parent_id: Option<Uuid>,
    pub logs: ArrayVec<Log<STR>, VEC>,

    start_now: Instant,
    start_allocations: u64,
    start_reallocations: u64,
}

impl<const VEC: usize, const STR: usize> Recorder<VEC, STR> {
    fn new(name: &'static str, parent_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            parent_id,
            logs: ArrayVec::new(),

            start_now: Instant::now(),
            start_allocations: ALLOCATOR.allocations(),
            start_reallocations: ALLOCATOR.reallocations(),
        }
    }

    pub fn log(&mut self, level: LogLevel, event: LogEvent<STR>) {
        self.logs.push(Log::new(level, event, self.id, self.parent_id));
    }

    pub fn scope(name: &'static str, parent_id: Option<Uuid>) -> Self {
        let mut recorder = Self::new(name, parent_id);
        recorder.log(LogLevel::Info, LogEvent::Start(name));
        recorder
    }

    pub fn with_system_information(mut self) -> Self {
        let system_information = system_information::<STR>();
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
        let global_information = global_information();
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
        let process_information = process_information();
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessMemory(process_information.memory));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessVirtualMemory(process_information.virtual_memory));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessStartTime(process_information.start_time));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessRunTime(process_information.run_time));
        self.log(LogLevel::ProcessInfo, LogEvent::ProcessCpuUsage(process_information.cpu_usage));

        self
    }
}

impl<const VEC: usize, const STR: usize> Drop for Recorder<VEC, STR> {
    fn drop(&mut self) {
        let allocations = ALLOCATOR.allocations() - self.start_allocations;
        let reallocations = ALLOCATOR.reallocations() - self.start_reallocations;
        let duration = self.start_now.elapsed().as_nanos() as u64;

        self.log(LogLevel::Info, LogEvent::Allocations(allocations));
        self.log(LogLevel::Info, LogEvent::Reallocations(reallocations));
        self.log(LogLevel::Info, LogEvent::Duration(duration));
        self.log(LogLevel::Info, LogEvent::End(self.name));

        for log in &self.logs {
            println!("{}", log);
        }
    }
}

pub type HotRecorder     = Recorder<HOT_VEC_CAP,     HOT_STR_CAP>;
pub type DefaultRecorder = Recorder<DEFAULT_VEC_CAP, DEFAULT_STR_CAP>;
pub type FullRecorder    = Recorder<FULL_VEC_CAP,    FULL_STR_CAP>;

