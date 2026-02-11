#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(str_from_raw_parts)]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod bootinfo;
mod heap;
mod trace;

use crate::bootinfo::BootInfo;
use embla::cmdline::Cmdline;
use hal::mem::{PageTable, VAddr};
use tracing::{info, trace};

#[unsafe(no_mangle)]
pub fn kentry(boot_info_ptr: VAddr) -> ! {
    tracing::dispatch::set_global_default(tracing::Dispatch::from_static(&trace::SUBSCRIBER))
        .unwrap();
    info!("Hello, World!");

    let mut bootinfo = BootInfo::new(boot_info_ptr);
    info!("Kernel cmdline: {:?}", bootinfo.cmdline());
    let cmdline = Cmdline::new(bootinfo.cmdline());
    trace::SUBSCRIBER.configure(&cmdline);

    for entry in bootinfo.memory_map() {
        trace!("Memmap entry: {:?}", entry);
    }

    let mut kernel_page_table =
        unsafe { PageTable::current(hal::mem::kernel_map::PHYSICAL_MAPPING_BASE) };
    heap::bootstrap(&mut kernel_page_table, &mut bootinfo);

    loop {}
}
