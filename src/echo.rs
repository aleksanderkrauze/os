use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;

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

pub fn echo_prompt() {
    vga_print!("> ");
}

pub fn echo(c: char) {
    match c {
        '\n' => {
            let buff = DEFAULT_ECHO.lock().clear();
            let mut writer = crate::io::vga::WRITER.lock();

            write!(writer, "\n@ ").unwrap();
            let mut char_buffer = [0; 4];
            for c in buff.data() {
                let c = c.encode_utf8(&mut char_buffer);
                writer.write_string(c);
            }
            write!(writer, "\n\n> ").unwrap();
        }
        c => {
            DEFAULT_ECHO.lock().push(c);
            vga_print!("{}", c);
        }
    }
}