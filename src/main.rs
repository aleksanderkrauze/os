#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;

use os::io::vga;
use os::vga_println;

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    vga_println!("{}", info);

    os::hlt_loop()
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    vga_println!("Hello {}!", "World");
    vga_println!();
    vga::WRITER
        .lock()
        .set_color(vga::ColorCode::new(vga::Color::LightRed, vga::Color::Black));
    vga_println!("Here is some data: {}, {}", 42, 2.0 / 3.0);

    os::hlt_loop()
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    test_main();

    os::hlt_loop()
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
