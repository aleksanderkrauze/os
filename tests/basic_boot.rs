#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use os::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
