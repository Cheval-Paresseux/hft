use std::sync::{LazyLock, Mutex, OnceLock};
use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System};
use arrayvec::ArrayString;

// ── Global System Variable ────────────────────────────────────────────────────

static SYSTEM: LazyLock<Mutex<System>> = LazyLock::new(|| {
    let mut sys = System::new_all();
    sys.refresh_all();

    SYSTEM_TOTAL_MEMORY.get_or_init(|| sys.total_memory());
    SYSTEM_TOTAL_SWAP.get_or_init(|| sys.total_swap());

    Mutex::new(sys)
});

// ── Computed Only Once -───────────────────────────────────────────────────────

static SYSTEM_NAME: OnceLock<Option<String>> = OnceLock::new();
static SYSTEM_KERNEL_VERSION: OnceLock<Option<String>> = OnceLock::new();
static SYSTEM_OS_VERSION: OnceLock<Option<String>> = OnceLock::new();
static SYSTEM_HOST_NAME: OnceLock<Option<String>> = OnceLock::new();
static SYSTEM_CPU_ARCHITECTURE: OnceLock<String> = OnceLock::new();
static SYSTEM_CORE_COUNT: OnceLock<Option<usize>> = OnceLock::new();
static SYSTEM_BOOT_TIME: OnceLock<u64> = OnceLock::new();
static SYSTEM_TOTAL_MEMORY: OnceLock<u64> = OnceLock::new();
static SYSTEM_TOTAL_SWAP: OnceLock<u64> = OnceLock::new();
static CURRENT_PID: OnceLock<sysinfo::Pid> = OnceLock::new();
static PROCESS_START_TIME: OnceLock<u64> = OnceLock::new();

// ── Informations -────────────────────-────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SystemInformation<const STR: usize> {
    pub name: Option<ArrayString<STR>>,
    pub kernel_version: Option<ArrayString<STR>>,
    pub os_version: Option<ArrayString<STR>>,
    pub host_name: Option<ArrayString<STR>>,
    pub cpu_architecture: ArrayString<STR>,
    pub core_count: Option<usize>,
    pub boot_time: u64,
    pub uptime: u64,
    pub total_memory: u64,
    pub total_swap: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalInformation {
    pub available_memory: u64,
    pub used_memory: u64,
    pub free_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: f32,
    pub load_average: (f64, f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessInformation {
    pub memory: u64,
    pub virtual_memory: u64,
    pub start_time: u64,
    pub run_time: u64,
    pub cpu_usage: f32,
}

// ── Retrieve Infos -────────────────────-──────────────────────────────────────

pub fn system_info<const STR: usize>() -> SystemInformation<STR> {
    let _ = &*SYSTEM;

    SystemInformation {
        name: to_opt_array_string::<STR>(SYSTEM_NAME.get_or_init(System::name).as_deref()),
        kernel_version: to_opt_array_string::<STR>(SYSTEM_KERNEL_VERSION.get_or_init(System::kernel_version).as_deref()),
        os_version: to_opt_array_string::<STR>(SYSTEM_OS_VERSION.get_or_init(System::os_version).as_deref()),
        host_name: to_opt_array_string::<STR>(SYSTEM_HOST_NAME.get_or_init(System::host_name).as_deref()),
        cpu_architecture: to_array_string::<STR>(SYSTEM_CPU_ARCHITECTURE.get_or_init(System::cpu_arch)),
        core_count: *SYSTEM_CORE_COUNT.get_or_init(System::physical_core_count),
        boot_time: *SYSTEM_BOOT_TIME.get_or_init(System::boot_time),
        uptime: System::uptime(),
        total_memory: *SYSTEM_TOTAL_MEMORY.get().unwrap(),
        total_swap: *SYSTEM_TOTAL_SWAP.get().unwrap(),
    }
}

pub fn global_info() -> GlobalInformation {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    let l = System::load_average();

    GlobalInformation {
        available_memory: sys.available_memory(),
        used_memory: sys.used_memory(),
        free_swap: sys.free_swap(),
        used_swap: sys.used_swap(),
        cpu_usage: sys.global_cpu_usage(),
        load_average: (l.one, l.five, l.fifteen),
    }
}

pub fn process_info() -> ProcessInformation {
    let mut sys = SYSTEM.lock().unwrap();
    let pid = *CURRENT_PID.get_or_init(|| sysinfo::get_current_pid().unwrap());

    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::nothing().with_memory().with_cpu()
    );

    let process = sys.process(pid).unwrap();
    let start_time = *PROCESS_START_TIME.get_or_init(|| process.start_time());

    ProcessInformation {
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        start_time,
        run_time: process.run_time(),
        cpu_usage: process.cpu_usage(),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn to_array_string<const STR: usize>(s: &str) -> ArrayString<STR> {
    let mut a = ArrayString::new();
    let truncated = &s[..s.len().min(STR)];
    a.push_str(truncated);
    a
}

fn to_opt_array_string<const STR: usize>(s: Option<&str>) -> Option<ArrayString<STR>> {
    s.map(to_array_string::<STR>)
}

// ── Unit Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── to_array_string
    #[test]
    fn test_to_array_string_fits_exactly() {
        let result = to_array_string::<5>("hello");
        assert_eq!(result.as_str(), "hello");
    }

    #[test]
    fn test_to_array_string_truncates_when_too_long() {
        let result = to_array_string::<4>("hello");
        assert_eq!(result.as_str(), "hell");
    }

    #[test]
    fn test_to_array_string_handles_empty() {
        let result = to_array_string::<8>("");
        assert_eq!(result.as_str(), "");
    }

    // ── to_opt_array_string
    #[test]
    fn test_to_opt_array_string_some_value() {
        let result = to_opt_array_string::<8>(Some("linux"));

        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str(), "linux");
    }

    #[test]
    fn test_to_opt_array_string_none_value() {
        let result = to_opt_array_string::<8>(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_to_opt_array_string_truncates() {
        let result = to_opt_array_string::<3>(Some("abcdef"));
        assert_eq!(result.unwrap().as_str(), "abc");
    }

    // ── system_info
    #[test]
    fn test_system_info_returns_valid_struct() {
        let info = system_info::<64>();

        assert!(info.boot_time > 1_000_000_000, "boot_time looks implausible: {}", info.boot_time);
        assert!(info.total_memory > 0, "total_memory should be non-zero");
    }

    #[test]
    fn test_system_info_cpu_architecture_non_empty() {
        let info = system_info::<64>();
        assert!(!info.cpu_architecture.is_empty(), "cpu_architecture should not be empty");
    }

    #[test]
    fn test_system_info_truncates_to_const_str() {
        let info = system_info::<4>();

        assert!(info.cpu_architecture.len() <= 4);
        if let Some(name) = info.name {
            assert!(name.len() <= 4);
        }
    }

    #[test]
    fn test_system_info_is_stable_across_calls() {
        let a = system_info::<64>();
        let b = system_info::<64>();

        assert_eq!(a.total_memory, b.total_memory);
        assert_eq!(a.total_swap, b.total_swap);
        assert_eq!(a.boot_time, b.boot_time);
        assert_eq!(a.cpu_architecture, b.cpu_architecture);
    }

    // ── global_info
    #[test]
    fn test_global_info_memory_bounds() {
        let total = system_info::<64>().total_memory;
        let info = global_info();

        assert!(info.used_memory <= total, "used_memory ({}) exceeds total ({})", info.used_memory, total);
        assert!(info.available_memory <= total, "available_memory ({}) exceeds total ({})", info.available_memory, total);
    }

    #[test]
    fn test_global_info_swap_bounds() {
        let total_swap = system_info::<64>().total_swap;
        let info = global_info();

        assert!(info.used_swap <= total_swap, "used_swap ({}) exceeds total_swap ({})", info.used_swap, total_swap);
        assert!(info.free_swap <= total_swap, "free_swap ({}) exceeds total_swap ({})", info.free_swap, total_swap);
    }

    #[test]
    fn test_global_info_cpu_usage_in_range() {
        let info = global_info();
        assert!(info.cpu_usage >= 0.0 && info.cpu_usage <= 100.0,
            "cpu_usage out of range: {}", info.cpu_usage);
    }

    #[test]
    fn test_global_info_load_average_non_negative() {
        let (one, five, fifteen) = global_info().load_average;
        assert!(one >= 0.0, "1m load average is negative: {one}");
        assert!(five >= 0.0, "5m load average is negative: {five}");
        assert!(fifteen >= 0.0, "15m load average is negative: {fifteen}");
    }

    // ── process_info
    #[test]
    fn test_process_info_memory_non_zero() {
        let info = process_info();

        assert!(info.memory > 0, "current process should have non-zero memory usage");
        assert!(info.virtual_memory >= info.memory,"virtual_memory ({}) should be >= physical memory ({})", info.virtual_memory, info.memory);
    }

    #[test]
    fn test_process_info_start_time_plausible() {
        let info = process_info();
        assert!(info.start_time > 1_000_000_000,"start_time looks implausible: {}", info.start_time);
    }

    #[test]
    fn test_process_info_start_time_is_stable() {
        let a = process_info();
        let b = process_info();
        
        assert_eq!(a.start_time, b.start_time, "start_time changed between calls");
    }

    #[test]
    fn test_process_info_cpu_usage_in_range() {
        let info = process_info();
        assert!(info.cpu_usage >= 0.0, "cpu_usage is negative: {}", info.cpu_usage);
    }
}