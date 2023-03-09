use alloc::alloc::{ GlobalAlloc, Layout };
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: DummyAlloc = DummyAlloc;

pub struct DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("Dealloc should never be called!")
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}