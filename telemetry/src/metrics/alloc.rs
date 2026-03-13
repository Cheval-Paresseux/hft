use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicU64, Ordering};

// ── Allocators Counter ────────────────────────────────────────────────────────

pub struct CountingAllocator {
    allocations: AtomicU64,
    reallocations: AtomicU64,
}

#[global_allocator]
pub static ALLOCATOR: CountingAllocator = CountingAllocator {
    allocations: AtomicU64::new(0),
    reallocations: AtomicU64::new(0),
};

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.reallocations.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

impl CountingAllocator {
    pub fn allocations(&self) -> u64 {
        self.allocations.load(Ordering::Relaxed)
    }
    pub fn reallocations(&self) -> u64 {
        self.reallocations.load(Ordering::Relaxed)
    }
    pub fn reset(&self) {
        self.allocations.store(0, Ordering::Relaxed);
        self.reallocations.store(0, Ordering::Relaxed);
    }
}