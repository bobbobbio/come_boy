// copyright 2021 Remi Bernotavicius

use ::alloc::alloc::*;
use core::ffi::c_void;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

pub struct Allocator;

impl Allocator {
    const fn new() -> Self {
        Self
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = malloc(layout.size()) as *mut u8;
        assert!(!ptr.is_null(), "Out of memory");
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }
}

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::new();
