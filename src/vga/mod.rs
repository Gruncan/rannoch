use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

mod vga_buffer;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<vga_buffer::Writer> = Mutex::new(vga_buffer::Writer::new(
        0,
        vga_buffer::ColorCode::new(vga_buffer::Color::Yellow, vga_buffer::Color::Black),
        unsafe { &mut *(0xb8000 as *mut crate::vga::vga_buffer::Buffer) }
    ));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
