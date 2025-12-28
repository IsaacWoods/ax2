#![no_std]
#![no_main]

use hal::io::IoPort;

#[unsafe(no_mangle)]
pub fn kentry() -> ! {
    unsafe {
        let debug_port = IoPort::new(0xe9);
        debug_port.write(b'H');
        debug_port.write(b'e');
        debug_port.write(b'l');
        debug_port.write(b'l');
        debug_port.write(b'o');
        debug_port.write(b' ');
        debug_port.write(b'f');
        debug_port.write(b'r');
        debug_port.write(b'o');
        debug_port.write(b'm');
        debug_port.write(b' ');
        debug_port.write(b'K');
        debug_port.write(b'e');
        debug_port.write(b'r');
        debug_port.write(b'n');
        debug_port.write(b'e');
        debug_port.write(b'l');
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // TODO
    loop {}
}
