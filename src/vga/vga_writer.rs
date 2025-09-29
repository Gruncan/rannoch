use crate::vga::vga_image::VgaImage;
use crate::vga::{Buffer, ColorCode, ScreenChar};
use core::fmt;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

const BLANK_CHAR: ScreenChar = ScreenChar {
    ascii_character: b' ',
    color_code: ColorCode::default(),
};

impl Writer {
    pub(crate) fn new(column_position: usize, color_code: ColorCode, buffer: &'static mut Buffer) -> Self {
        Self {
            column_position,
            color_code,
            buffer,
        }
    }

    pub fn write_image(&mut self, image: &VgaImage) {
        for row in 0..crate::vga::BUFFER_HEIGHT {
            for column in 0..crate::vga::BUFFER_WIDTH {
                if let Some(screen_char) = image.buf[row][column] {
                    self.buffer.chars[row][column].write(screen_char)
                } else {
                    self.buffer.chars[row][column].write(BLANK_CHAR)
                }
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= crate::vga::BUFFER_WIDTH {
                    self.new_line();
                }

                let row = crate::vga::BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..crate::vga::BUFFER_HEIGHT {
            for col in 0..crate::vga::BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(crate::vga::BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..crate::vga::BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
