#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, allow(unused_imports))]

use core::fmt::Write;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use os::io::vga::{Color, ColorCode, WRITER};
use os::vga_println;

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    vga_println!("{}", info);

    os::hlt_loop()
}

fn test() {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    vga_println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    os::hlt_loop();
}

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test_kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::init();

    vga_println!("BootInfo\n{:#?}", boot_info);

    test();

    let mut writer = WRITER.lock();
    let light_blue = ColorCode::new(Color::LightBlue, Color::Black);
    let old_color = writer.set_color(light_blue);

    writeln!(writer, "You have fallen into deep cave").unwrap();
    writeln!(writer, "There is no one to help you").unwrap();
    writeln!(writer, "You try screaming for help").unwrap();
    writeln!(writer, "But the only thing you can hear...\n").unwrap();
    writeln!(writer, "Is the Echo\n").unwrap();

    writer.set_color(old_color);
    drop(writer);

    os::echo::echo_prompt();

    os::hlt_loop()
}

#[cfg(test)]
fn test_kernel_main(_: &'static BootInfo) -> ! {
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
