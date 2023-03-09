#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate bootloader_api;
extern crate spinning_top;
extern crate conquer_once;
extern crate noto_sans_mono_bitmap;
extern crate uart_16550;
extern crate log;
extern crate acpi;
extern crate alloc;
extern crate x86_64;

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};
use log::LevelFilter;

mod logger;
mod framebuffer;
mod serial;
mod acpi_handler;
mod allocator;
mod memory;

entry_point!(start);

fn start(boot_info: &'static mut BootInfo) -> ! {
    let info = boot_info.framebuffer.as_ref().unwrap().info();
    let framebuffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    logger::init_logger(
        framebuffer,
        info, 
        LevelFilter::Debug, 
        true, 
        true
    );

    let rsdp = *boot_info.rsdp_addr.as_ref().unwrap() as usize;

    acpi_handler::init_acpi(rsdp);

    log::info!("Welcome to TapOS!");
    
    #[cfg(test)]
    test_main();

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