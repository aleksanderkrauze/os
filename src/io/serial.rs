use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts::without_interrupts;

const SERIAL_UART_IO_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL_UART_IO_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::io::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}
