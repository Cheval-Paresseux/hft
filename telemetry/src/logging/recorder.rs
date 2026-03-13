use super::log::{Log, LogLevel, LogEvent};
use super::router::{RouterReference};
use crate::metrics::*;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use arrayvec::ArrayVec;
use crossbeam::channel::Sender;

// ── Recorder ──────────────────────────────────────────────────────────────────

pub struct Recorder<const VEC: usize, const STR: usize> {
    pub id: Uuid,
    pub name: &'static str,
    pub parent_id: Option<Uuid>,
    pub logs: ArrayVec<Log<STR>, VEC>,

    start_now: Instant,
    start_allocations: u64,
    start_reallocations: u64,

    tx: Sender<Log<STR>>,
    context: Arc<Mutex<Vec<Uuid>>>,
}

impl<const VEC: usize, const STR: usize> Recorder<VEC, STR> {
    fn new(name: &'static str, id: Uuid, parent_id: Option<Uuid>, tx: Sender<Log<STR>>, context: Arc<Mutex<Vec<Uuid>>>) -> Self {
        Self {
            id,
            name,
            parent_id,
            logs: ArrayVec::new(),
            start_now: Instant::now(),
            start_allocations: ALLOCATOR.allocations(),
            start_reallocations: ALLOCATOR.reallocations(),
            tx,
            context,
        }
    }

    pub fn get_logs(&self) -> ArrayVec<Log<STR>, VEC> {
        self.logs.clone()
    }

    pub fn log(&mut self, level: LogLevel, event: LogEvent<STR>) {
        self.logs.push(Log::new(level, event, self.id, self.parent_id));
    }

    pub fn scope(name: &'static str, reference: &mut RouterReference<STR>) -> Self {
        let parent_id = reference.current();
        let tx = reference.tx.clone();
        let context = Arc::clone(&reference.recorders_context);
        let id = Uuid::new_v4();

        reference.push_recorder(id);

        let mut recorder = Self::new(name, id, parent_id, tx, context);
        recorder.log(LogLevel::Info, LogEvent::Start(name));
        recorder
    }

    pub fn with_system_information(mut self) -> Self {
        let system_information = system_info::<STR>();
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
        let global_information = global_info();
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
        let process_information = process_info();
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
            let _ = self.tx.send(*log);
        }

        self.context.lock().unwrap().pop();
    }
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use crossbeam::channel::unbounded;

    #[test]
    fn test_scope_creation() {
        let (tx, _) = unbounded();
        let context = Arc::new(Mutex::new(Vec::new()));
        let mut router_ref = RouterReference {
            tx: tx.clone(),
            recorders_context: Arc::clone(&context),
        };

        let recorder = Recorder::<8, 32>::scope("TestScope", &mut router_ref);

        assert_eq!(recorder.name, "TestScope");
        assert!(recorder.logs.len() >= 1);

        match recorder.logs[0].event {
            LogEvent::Start(name) => assert_eq!(name, "TestScope"),
            _ => panic!("Expected LogEvent::Start"),
        }
    }

    #[test]
    fn test_log_message() {
        let (tx, _) = unbounded();
        let context = Arc::new(Mutex::new(Vec::new()));
        let mut router_ref = RouterReference {
            tx: tx.clone(),
            recorders_context: Arc::clone(&context),
        };

        let mut recorder = Recorder::<8, 32>::scope("TestScope", &mut router_ref);
        recorder.log(LogLevel::Info, LogEvent::message("Hello"));

        match recorder.logs[1].event {
            LogEvent::Message(ref msg) => assert_eq!(msg.as_str(), "Hello"),
            _ => panic!("Expected LogEvent::Message"),
        }
    }

    #[test]
    fn test_with_system_global_process() {
        let (tx, _) = unbounded();
        let context = Arc::new(Mutex::new(Vec::new()));
        let mut router_ref = RouterReference {
            tx: tx.clone(),
            recorders_context: Arc::clone(&context),
        };

        let recorder = Recorder::<32, 64>::scope("TestScope", &mut router_ref)
            .with_system_information()
            .with_global_information()
            .with_process_information();

        let has_sys_info = recorder.logs.iter().any(|l| l.level == LogLevel::SysInfo);
        let has_global_info = recorder.logs.iter().any(|l| l.level == LogLevel::GlobalInfo);
        let has_proc_info = recorder.logs.iter().any(|l| l.level == LogLevel::ProcessInfo);

        assert!(has_sys_info);
        assert!(has_global_info);
        assert!(has_proc_info);
    }

    #[test]
    fn test_drop_sends_logs() {
        let (tx, rx) = unbounded();
        let context = Arc::new(Mutex::new(Vec::new()));
        let mut router_ref = RouterReference {
            tx: tx.clone(),
            recorders_context: Arc::clone(&context),
        };

        {
            let _recorder = Recorder::<8, 32>::scope("TestScope", &mut router_ref);
        }

        let logs: Vec<Log<32>> = rx.try_iter().collect();
        assert!(logs.iter().any(|l| match l.event {
            LogEvent::End(name) => name == "TestScope",
            _ => false,
        }));
    }
}