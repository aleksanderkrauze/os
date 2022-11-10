#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use x86_64::instructions::port::Port;

mod vga_buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

const QEMU_EXIT_IO_PORT: u16 = 0xf4;

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(QEMU_EXIT_IO_PORT);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "World");
    println!();
    vga_buffer::WRITER
        .lock()
        .set_color(vga_buffer::ColorCode::new(
            vga_buffer::Color::LightRed,
            vga_buffer::Color::Black,
        ));
    println!("Here is some data: {}, {}", 42, 2.0 / 3.0);

    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());

        for test in tests {
            test();
        }

        exit_qemu(QemuExitCode::Success);
    }

    #[test_case]
    fn trivial_assertion() {
        print!("trivial assertion... ");
        assert_eq!(1, 1);
        println!("[ok]");
    }
}
