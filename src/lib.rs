#![feature(restricted_std)]

use core::ptr::null_mut;
use libctru::raw::{
    aptMainLoop, c_char, c_int, consoleInit, gfxExit, gfxFlushBuffers, gfxInitDefault,
    gfxScreen_t_GFX_BOTTOM, gfxScreen_t_GFX_TOP, gfxSwapBuffers, gspWaitForEvent, hidKeysDown,
    hidScanInput, GSPGPU_Event_GSPGPU_EVENT_VBlank0, KEY_START,
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

        consoleInit(gfxScreen_t_GFX_TOP, null_mut());

        libctru::println!("Standard print in next line:");
        println!("Standard print");

        std::thread::spawn(|| {
            println!("Hello thread world");
        })
        .join()
        .unwrap();

        libctru::println!("Check end");

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
