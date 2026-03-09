use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicU64, Ordering};

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



// ========== UNIT TESTS ==========
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn snapshot() -> (u64, u64) {
        (ALLOCATOR.allocations(), ALLOCATOR.reallocations())
    }

    fn delta(before: (u64, u64)) -> (u64, u64) {
        (
            ALLOCATOR.allocations() - before.0,
            ALLOCATOR.reallocations() - before.1,
        )
    }

    #[test]
    fn reset_counters() {
        let _lock = TEST_LOCK.lock().unwrap();

        let _ = Box::new(72);
        ALLOCATOR.reset();

        assert_eq!(ALLOCATOR.allocations(), 0);
        assert_eq!(ALLOCATOR.reallocations(), 0);
    }

    #[test]
    fn dealloc_does_not_affect() {
        let _lock = TEST_LOCK.lock().unwrap();
        ALLOCATOR.reset();

        let b = Box::new(72);
        let before = snapshot();
        drop(b);
        let (allocs, reallocs) = delta(before);

        assert_eq!(allocs, 0);
        assert_eq!(reallocs, 0);
    }

    #[test]
    fn alloc_counter() {
        let _lock = TEST_LOCK.lock().unwrap();
        ALLOCATOR.reset();

        let before = snapshot();
        let _a = Box::new(72);
        let _b = Box::new(72);
        let (allocs, reallocs) = delta(before);

        assert_eq!(allocs, 2);
        assert_eq!(reallocs, 0);
    }

    #[test]
    fn realloc_counter() {
        let _lock = TEST_LOCK.lock().unwrap();
        ALLOCATOR.reset();

        let before = snapshot();
        let mut v: Vec<u8> = Vec::new();
        for i in 0..100 {v.push(i);}
        let (_, reallocs) = delta(before);

        assert!(reallocs > 0);
    }

    #[test]
    fn vec_with_capacity_does_not_reallocate() {
        let _lock = TEST_LOCK.lock().unwrap();
        ALLOCATOR.reset();

        let mut v: Vec<u8> = Vec::with_capacity(100);
        let before = snapshot();
        for i in 0..100 {v.push(i);}
        let (_, reallocs) = delta(before);

        assert_eq!(reallocs, 0);
    }
}