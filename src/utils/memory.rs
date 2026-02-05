use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

const MEMORY_LIMIT: usize = {
    #[cfg(all(__wasm32__, not(__desktop__)))]
    {
        3_000_000_000
    }
    #[cfg(all(__desktop__, not(__wasm32__)))]
    {
        16_000_000_000
    }
    #[cfg(not(any(__wasm32__, __desktop__)))]
    {
        16_000_000_000
    }
};

pub struct TrackingAlloc {
    used: AtomicUsize,
}

unsafe impl GlobalAlloc for TrackingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let new_used = self.used.fetch_add(size, Ordering::SeqCst) + size;

        if new_used > MEMORY_LIMIT {
            self.used.fetch_sub(size, Ordering::SeqCst);
            panic!("Memory Limit Exceeded {}", MEMORY_LIMIT / 1_000_000_000);
        }

        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.used.fetch_sub(layout.size(), Ordering::SeqCst);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOC: TrackingAlloc = TrackingAlloc {
    used: AtomicUsize::new(0),
};

#[allow(unused)]
pub fn memory_used() -> usize {
    return ALLOC.used.load(Ordering::SeqCst);
}
