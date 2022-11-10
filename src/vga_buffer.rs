use core::convert::AsRef;
use core::fmt::{self, Write};
use core::ptr;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn new(byte: u8, color: ColorCode) -> Self {
        Self {
            ascii_character: byte,
            color_code: color,
        }
    }

    fn write_volatile(&mut self, src: Self) {
        // SAFETY: We have exclusive access to self.
        unsafe {
            ptr::addr_of_mut!(*self).write_volatile(src);
        }
    }

    fn read_volatile(&mut self) -> Self {
        // SAFETY: We have exclusive access to self.
        unsafe { ptr::addr_of!(*self).read_volatile() }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGAWriter {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl VGAWriter {
    /// # Safety
    ///
    /// This function can only be called once.
    pub unsafe fn new() -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: &mut *(0xb8000 as *mut Buffer),
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }

        self.column_position = 0;
        self.row_position = 0;
    }

    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }

    pub fn write_string<T: AsRef<[u8]> + ?Sized>(&mut self, s: &T) {
        for &byte in s.as_ref() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write_volatile(ScreenChar::new(byte, color_code));
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.column_position = 0;
            self.row_position += 1;
            return;
        }

        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read_volatile();
                self.buffer.chars[row - 1][col].write_volatile(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write_volatile(blank);
        }
    }
}

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(unsafe { VGAWriter::new() });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_println_simple() {
        println!("test_println_simple output");
    }

    #[test_case]
    fn test_println_many() {
        for _ in 0..100 {
            println!("test_println_many output");
        }
    }

    #[test_case]
    fn test_println_output() {
        without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.clear();

            let s = "Some test string that fits on a single line";
            writeln!(writer, "{}", s).expect("writeln! failed");

            for (i, c) in s.chars().enumerate() {
                let screen_char = writer.buffer.chars[0][i].read_volatile();
                assert_eq!(char::from(screen_char.ascii_character), c);
            }
        });
    }
}
