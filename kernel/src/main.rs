#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate bootloader_api;
extern crate spinning_top;
extern crate conquer_once;
extern crate noto_sans_mono_bitmap;
extern crate uart_16550;
extern crate log;

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo, info::{FrameBufferInfo, FrameBuffer}};
use log::LevelFilter;

mod logger;
mod framebuffer;
mod serial;

entry_point!(start);

fn start(boot_info: &'static mut BootInfo) -> ! {
    let info = boot_info.framebuffer.as_ref().unwrap().info();
    let framebuffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    init_logger(
        framebuffer,
        info, 
        LevelFilter::Debug, 
        true, 
        true
    );

    log::info!("Welcome to TapOS!");
    
    #[cfg(test)]
    test_main();

    loop {}
}

fn init_logger(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    log_level: LevelFilter,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let logger = logger::LOGGER.get_or_init(move || {
        logger::LockedLogger::new(
            framebuffer, 
            info, 
            frame_buffer_logger_status, 
            serial_logger_status
        )
    });

    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log_level);
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