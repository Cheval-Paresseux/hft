use std::sync::{LazyLock, Mutex, OnceLock};
use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System};
use arrayvec::ArrayString;

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

static SYSTEM: LazyLock<Mutex<System>> = LazyLock::new(|| {
    let mut sys = System::new_all();
    sys.refresh_all();

    SYSTEM_TOTAL_MEMORY.get_or_init(|| sys.total_memory());
    SYSTEM_TOTAL_SWAP.get_or_init(|| sys.total_swap());

    Mutex::new(sys)
});

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

pub fn system_information<const STR: usize>() -> SystemInformation<STR> {
    let _ = &*SYSTEM;

    SystemInformation {
        name: to_opt_array_string::<STR>(SYSTEM_NAME.get_or_init(|| System::name()).as_deref()),
        kernel_version: to_opt_array_string::<STR>(SYSTEM_KERNEL_VERSION.get_or_init(|| System::kernel_version()).as_deref()),
        os_version: to_opt_array_string::<STR>(SYSTEM_OS_VERSION.get_or_init(|| System::os_version()).as_deref()),
        host_name: to_opt_array_string::<STR>(SYSTEM_HOST_NAME.get_or_init(|| System::host_name()).as_deref()),
        cpu_architecture: to_array_string::<STR>(SYSTEM_CPU_ARCHITECTURE.get_or_init(|| System::cpu_arch())),
        core_count: *SYSTEM_CORE_COUNT.get_or_init(|| System::physical_core_count()),
        boot_time: *SYSTEM_BOOT_TIME.get_or_init(|| System::boot_time()),
        uptime: System::uptime(),
        total_memory: *SYSTEM_TOTAL_MEMORY.get().unwrap(),
        total_swap: *SYSTEM_TOTAL_SWAP.get().unwrap(),
    }
}

pub fn global_information() -> GlobalInformation {
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

pub fn process_information() -> ProcessInformation {
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

fn to_array_string<const STR: usize>(s: &str) -> ArrayString<STR> {
    let mut a = ArrayString::new();
    let truncated = &s[..s.len().min(STR)];
    a.push_str(truncated);
    a
}

fn to_opt_array_string<const STR: usize>(s: Option<&str>) -> Option<ArrayString<STR>> {
    s.map(to_array_string::<STR>)
}
