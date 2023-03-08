#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Don't know why we need this but it complains otherwise
extern crate bootloader_api;

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

entry_point!(start);

fn start(boot_info: &'static mut BootInfo) -> ! {
    // println!("Welcome to TapOS!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", _info);

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    // print!("trivial assertion... ");
    assert_eq!(1, 1);
    // println!("[ok]");
}