#![no_std]
#![no_main]

mod vga;

use crate::vga::vga_image::LOAD_SCREEN_BUFFER;
use crate::vga::WRITER;
use core::fmt::Write;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    WRITER.lock().write_image(&LOAD_SCREEN_BUFFER);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
