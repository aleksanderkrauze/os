#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use os::mem;
use os::vga_println;

#[panic_handler]
fn handler(info: &PanicInfo) -> ! {
    vga_println!("{}", info);

    os::hlt_loop()
}

#[allow(dead_code)]
fn test(boot_info: &'static BootInfo) {
    use x86_64::{structures::paging::PageTable, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { mem::active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            vga_println!("L4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            let old = os::io::vga::WRITER.lock().set_color(
                os::io::vga::ColorCode::new_with_black_background(os::io::vga::Color::LightRed),
            );

            // print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    vga_println!("  L3 Entry {}: {:?}", i, entry);
                }
            }

            os::io::vga::WRITER.lock().set_color(old);
        }
    }

    os::hlt_loop();
}

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test_kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::init();

    test(boot_info);

    os::echo::init();

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
