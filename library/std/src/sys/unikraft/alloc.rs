use unikraft_abi::alloc::System as UkSystem;

use crate::alloc::{GlobalAlloc, Layout, System};

static GLOBAL: UkSystem = UkSystem;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        GLOBAL.alloc(layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        GLOBAL.alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        GLOBAL.dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        GLOBAL.realloc(ptr, layout, new_size)
    }
}
