#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os::vga_println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::testing::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    test_main();

    os::hlt_loop()
}

#[test_case]
fn test_println() {
    vga_println!("test_println output");
}
