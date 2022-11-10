#![no_std]
#![no_main]

use core::panic::PanicInfo;

use os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn handler(_: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    os::hlt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();

    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    os::hlt_loop()
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
