#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "World");
    println!();
    println!("Here is some data: {}, {}", 42, 2.0 / 3.0);

    loop {}
}
