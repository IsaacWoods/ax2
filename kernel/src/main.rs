#![no_std]
#![no_main]
#![feature(str_from_raw_parts)]

mod bootinfo;
mod trace;

use crate::bootinfo::BootInfo;
use core::alloc::GlobalAlloc;
use embla::cmdline::Cmdline;
use hal::mem::VAddr;
use tracing::{info, trace};

#[unsafe(no_mangle)]
pub fn kentry(boot_info_ptr: VAddr) -> ! {
    tracing::dispatch::set_global_default(tracing::Dispatch::from_static(&trace::SUBSCRIBER))
        .unwrap();
    info!("Hello, World!");

    let bootinfo = BootInfo::new(boot_info_ptr);
    info!("Kernel cmdline: {:?}", bootinfo.cmdline());
    let cmdline = Cmdline::new(bootinfo.cmdline());
    trace::SUBSCRIBER.configure(&cmdline);

    for entry in bootinfo.memory_map() {
        trace!("Memmap entry: {:?}", entry);
    }

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
