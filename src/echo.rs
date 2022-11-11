use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::io::vga::{Color, ColorCode, WRITER};
use crate::vga_print;

const BUFFER_SIZE: usize = 75;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoBuffer {
    read_characters: usize,
    data: [char; BUFFER_SIZE],
}

impl EchoBuffer {
    pub fn new() -> Self {
        Self {
            read_characters: 0,
            data: [char::default(); BUFFER_SIZE],
        }
    }

    pub fn push(&mut self, c: char) {
        match self.read_characters {
            n if n < BUFFER_SIZE => {
                self.data[n] = c;
                self.read_characters += 1;
            }
            n if n == BUFFER_SIZE => {
                // Do nothing, buffer is full
            }
            _ => unreachable!(),
        }
    }

    pub fn clear(&mut self) -> Self {
        let b = self.clone();
        self.read_characters = 0;
        b
    }

    pub fn data(&self) -> &[char] {
        match self.read_characters {
            n if n <= BUFFER_SIZE => &self.data[..n],
            _ => unreachable!(),
        }
    }
}

lazy_static! {
    static ref DEFAULT_ECHO: Mutex<EchoBuffer> = Mutex::new(EchoBuffer::new());
}

pub fn init() {
    let mut writer = WRITER.lock();
    let light_blue = ColorCode::new_with_black_background(Color::LightBlue);
    let old_color = writer.set_color(light_blue);

    writeln!(writer, "You have fallen into deep cave").unwrap();
    writeln!(writer, "There is no one to help you").unwrap();
    writeln!(writer, "You try screaming for help").unwrap();
    writeln!(writer, "But the only thing you can hear...\n").unwrap();
    writeln!(writer, "Is the Echo\n").unwrap();

    writer.set_color(old_color);
    write!(writer, "> ").unwrap();
}

pub fn process(c: char) {
    match c {
        '\n' => {
            let buff = DEFAULT_ECHO.lock().clear();
            let mut writer = WRITER.lock();

            write!(writer, "\n").unwrap();
            let light_blue = ColorCode::new_with_black_background(Color::LightBlue);
            let old_color = writer.set_color(light_blue);
            write!(writer, "@ ").unwrap();

            let mut char_buffer = [0; 4];
            for c in buff.data() {
                let c = c.encode_utf8(&mut char_buffer);
                writer.write_string(c);
            }

            writer.set_color(old_color);
            write!(writer, "\n\n> ").unwrap();
        }
        c => {
            DEFAULT_ECHO.lock().push(c);
            vga_print!("{}", c);
        }
    }
}
