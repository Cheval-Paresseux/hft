mod alloc;
mod system;

pub use self::{
    alloc::ALLOCATOR,
    system::{
        SystemInformation,
        GlobalInformation,
        ProcessInformation,
        system_info,
        global_info,
        process_info,
    },
};