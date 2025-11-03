#![no_std]
#![no_main]

#[macro_use]
mod util;

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    println!("Booting AX/2 - loader v{}", env!("CARGO_PKG_VERSION"));
    println!(
        "Firmware: {} (revision {}; implementing UEFI {})",
        uefi::system::firmware_vendor(),
        uefi::system::firmware_revision(),
        uefi::system::uefi_revision()
    );

    boot::stall(3_000_000);
    Status::SUCCESS
}
