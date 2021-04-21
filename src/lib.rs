#![feature(restricted_std)]

use libctru::raw::{
    aptMainLoop, consoleInit, gfxExit, gfxFlushBuffers, gfxInitDefault, gfxScreen_t_GFX_BOTTOM,
    gfxSwapBuffers, gspWaitForEvent, hidKeysDown, hidScanInput, GSPGPU_Event_GSPGPU_EVENT_VBlank0,
    KEY_START,
};
use std::{
    os::raw::{c_char, c_int},
    ptr::null_mut,
};

#[no_mangle]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    std::panic::set_hook(Box::new(|info| unsafe {
        consoleInit(gfxScreen_t_GFX_BOTTOM, null_mut());

        println!("Rust panic: \n  {}\nPress start to crash", info);

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
    }));

    unsafe {
        gfxInitDefault();

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
