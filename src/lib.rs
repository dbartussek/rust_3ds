#![no_std]

use core::{panic::PanicInfo, ptr::null_mut};
use libctru::raw::{
    aptMainLoop, c_char, c_int, consoleInit, gfxExit, gfxFlushBuffers, gfxInitDefault,
    gfxScreen_t_GFX_TOP, gfxSwapBuffers, gspWaitForEvent, hidKeysDown, hidScanInput,
    GSPGPU_Event_GSPGPU_EVENT_VBlank0, KEY_START,
};

extern "C" {
    fn puts(string: *const c_char);
}

#[no_mangle]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    unsafe {
        gfxInitDefault();

        consoleInit(gfxScreen_t_GFX_TOP, null_mut());

        puts("Hello World from Rust".as_ptr());

        while aptMainLoop() {
            hidScanInput();
            let keys = hidKeysDown();

            if keys & (KEY_START as u32) != 0 {
                break;
            }

            gfxFlushBuffers();
            gfxSwapBuffers();

            gspWaitForEvent(GSPGPU_Event_GSPGPU_EVENT_VBlank0, true);
        }

        gfxExit();
    }

    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
