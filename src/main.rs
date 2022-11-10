#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

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
    }

    #[test_case]
    fn trivial_assertion() {
        print!("trivial assertion... ");
        assert_eq!(1, 1);
        println!("[ok]");
    }
}
