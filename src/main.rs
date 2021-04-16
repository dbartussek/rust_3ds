#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn pre_main() -> ! {
    unsafe {
        core::ptr::null::<usize>().read_volatile();
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
