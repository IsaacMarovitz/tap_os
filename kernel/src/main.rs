#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate acpi;
extern crate alloc;
extern crate bootloader_api;
extern crate conquer_once;
extern crate lazy_static;
extern crate good_memory_allocator;
extern crate log;
extern crate noto_sans_mono_bitmap;
extern crate spinning_top;
extern crate uart_16550;
extern crate x86_64;

use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use log::LevelFilter;

mod acpi_handler;
mod allocator;
mod apic;
mod framebuffer;
mod interrupts;
mod logger;
mod memory;
mod serial;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(start, config = &BOOTLOADER_CONFIG);

fn start(boot_info: &'static mut BootInfo) -> ! {
    interrupts::init();

    let info = boot_info.framebuffer.as_ref().unwrap().info();
    let framebuffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    logger::init(framebuffer, info, LevelFilter::Debug, true, true);

    log::info!("[KERN]: Logger initialized...");

    let physical_memory_offset = *boot_info.physical_memory_offset.as_ref().unwrap();
    let memory_info = &boot_info.memory_regions;
    memory::init(memory_info, physical_memory_offset);

    log::info!("[KERN]: Memory initialized...");

    let rsdp = *boot_info.rsdp_addr.as_ref().unwrap() as usize;
    acpi_handler::init(rsdp);

    log::info!("[KERN]: ACPI initialized...");

    #[cfg(test)]
    test_main();

    log::info!("[KERN]: Welcome to TapOS!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{}", _info);

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    log::info!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    log::info!("trivial assertion... ");
    assert_eq!(1, 1);
    log::info!("[ok]");
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
