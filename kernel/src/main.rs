#![no_std]
#![no_main]

mod trace;

use core::alloc::GlobalAlloc;
use tracing::info;

#[unsafe(no_mangle)]
pub fn kentry() -> ! {
    tracing::dispatch::set_global_default(tracing::Dispatch::from_static(&trace::SUBSCRIBER))
        .unwrap();
    info!("Hello, World!");

    loop {}
}

#[global_allocator]
static ALLOC: FakeAlloc = FakeAlloc;

struct FakeAlloc;

unsafe impl GlobalAlloc for FakeAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
