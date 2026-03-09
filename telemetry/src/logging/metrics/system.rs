use std::sync::{LazyLock, Mutex, OnceLock};
use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System};

static SYSTEM: LazyLock<Mutex<System>> = LazyLock::new(|| {
    let mut sys = System::new_all();
    sys.refresh_all();
    Mutex::new(sys)
});

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

// --- Information Batches ---
#[derive(Debug, Clone)]
pub struct SystemInformation {
    pub name: Option<&'static str>,
    pub kernel_version: Option<&'static str>,
    pub os_version: Option<&'static str>,
    pub host_name: Option<&'static str>,
    pub cpu_architecture: &'static str,
    pub core_count: Option<usize>,
    pub boot_time: u64,
    pub uptime: u64,
    pub total_memory: u64,
    pub total_swap: u64,
}

pub fn system_information() -> SystemInformation {
    SystemInformation {
        name: system_name(),
        kernel_version: system_kernel_version(),
        os_version: system_os_version(),
        host_name: system_host_name(),
        cpu_architecture: system_cpu_architecture(),
        core_count: system_core_count(),
        boot_time: system_boot_time(),
        uptime: system_uptime(),
        total_memory: system_total_memory(),
        total_swap: system_total_swap(),
    }
}

#[derive(Debug, Clone)]
pub struct GlobalInformation {
    pub available_memory: u64,
    pub used_memory: u64,
    pub free_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: f32,
    pub load_average: (f64, f64, f64),
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

fn current_pid() -> sysinfo::Pid {
    *CURRENT_PID.get_or_init(|| sysinfo::get_current_pid().unwrap())
}

#[derive(Debug, Clone)]
pub struct ProcessInformation {
    pub memory: u64,
    pub virtual_memory: u64,
    pub start_time: u64,
    pub run_time: u64,
    pub cpu_usage: f32,
}

pub fn process_information() -> ProcessInformation {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_memory().with_cpu()
    );

    let process = sys.process(current_pid()).unwrap();

    ProcessInformation {
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        start_time: process_start_time(),
        run_time: process.run_time(),
        cpu_usage: process.cpu_usage(),
    }
}



// --- Single Metric Functions ---
pub fn system_name() -> Option<&'static str> {
    SYSTEM_NAME
        .get_or_init(|| System::name())
        .as_deref()
}

pub fn system_kernel_version() -> Option<&'static str> {
    SYSTEM_KERNEL_VERSION
        .get_or_init(|| System::kernel_version())
        .as_deref()
}

pub fn system_os_version() -> Option<&'static str> {
    SYSTEM_OS_VERSION
        .get_or_init(|| System::os_version())
        .as_deref()
}

pub fn system_host_name() -> Option<&'static str> {
    SYSTEM_HOST_NAME
        .get_or_init(|| System::host_name())
        .as_deref()
}

pub fn system_cpu_architecture() -> &'static str {
    SYSTEM_CPU_ARCHITECTURE
        .get_or_init(|| System::cpu_arch())
}

pub fn system_core_count() -> Option<usize> {
    *SYSTEM_CORE_COUNT.get_or_init(|| System::physical_core_count())
}

pub fn system_boot_time() -> u64 {
    *SYSTEM_BOOT_TIME
    .get_or_init(|| System::boot_time())
}

pub fn system_uptime() -> u64 {
    System::uptime()
}

pub fn system_total_memory() -> u64 {
    *SYSTEM_TOTAL_MEMORY.get_or_init(|| {
        SYSTEM.lock().unwrap().total_memory()
    })
}

pub fn system_total_swap() -> u64 {
    *SYSTEM_TOTAL_SWAP.get_or_init(|| {
        SYSTEM.lock().unwrap().total_swap()
    })
}

// ---

pub fn global_available_memory() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();

    sys.available_memory()
}

pub fn global_used_memory() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();

    sys.used_memory()
}

pub fn global_free_swap() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();

    sys.free_swap()
}

pub fn global_used_swap() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();

    sys.used_swap()
}

pub fn global_cpu_usage() -> f32 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_cpu_usage();

    sys.global_cpu_usage()
}

pub fn global_load_average() -> (f64, f64, f64) {
    let l = System::load_average();
    (l.one, l.five, l.fifteen)
}

// ---

pub fn process_memory() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[current_pid()]),
        true,
        ProcessRefreshKind::nothing().with_memory()
    );
    let process = sys.process(current_pid()).unwrap();

    process.memory()
}

pub fn process_virtual_memory() -> u64 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[current_pid()]),
        true,
        ProcessRefreshKind::nothing().with_memory()
    );
    let process = sys.process(current_pid()).unwrap();

    process.virtual_memory()
}

pub fn process_start_time() -> u64 {
    *PROCESS_START_TIME.get_or_init(|| {
        let sys = SYSTEM.lock().unwrap();
        sys.process(current_pid()).unwrap().start_time()
    })
}

pub fn process_run_time() -> u64 {
    let sys = SYSTEM.lock().unwrap();
    let process = sys.process(current_pid()).unwrap();

    process.run_time()
}

pub fn process_cpu_usage() -> f32 {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[current_pid()]),
        true,
        ProcessRefreshKind::nothing().without_tasks().with_cpu()
    );
    let process = sys.process(current_pid()).unwrap();

    process.cpu_usage()
}