use crate::{framebuffer::FrameBufferWriter, serial::SerialPort};
use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use core::fmt::Write;
use log::LevelFilter;
use spinning_top::Spinlock;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub struct LockedLogger {
    framebuffer: Option<Spinlock<FrameBufferWriter>>,
    serial: Option<Spinlock<SerialPort>>,
    level: log::Level,
}

impl LockedLogger {
    pub fn new(
        framebuffer: &'static mut [u8],
        info: FrameBufferInfo,
        frame_buffer_logger_status: bool,
        serial_logger_status: bool,
    ) -> Self {
        let framebuffer = match frame_buffer_logger_status {
            true => Some(Spinlock::new(FrameBufferWriter::new(framebuffer, info))),
            false => None,
        };

        let serial = match serial_logger_status {
            true => Some(Spinlock::new(unsafe { SerialPort::init() })),
            false => None,
        };

        LockedLogger {
            framebuffer,
            serial,
            level: log::Level::Trace,
        }
    }

    pub unsafe fn force_unlock(&self) {
        if let Some(framebuffer) = &self.framebuffer {
            unsafe { framebuffer.force_unlock() };
        }

        if let Some(serial) = &self.serial {
            unsafe { serial.force_unlock() };
        }
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if let Some(framebuffer) = &self.framebuffer {
            let mut framebuffer = framebuffer.lock();
            writeln!(framebuffer, "{:5}: {}", record.level(), record.args()).unwrap();
        }
        if let Some(serial) = &self.serial {
            let mut serial = serial.lock();
            writeln!(serial, "{:5}: {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

pub fn init(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    log_level: LevelFilter,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let logger = LOGGER.get_or_init(move || {
        LockedLogger::new(
            framebuffer,
            info,
            frame_buffer_logger_status,
            serial_logger_status,
        )
    });

    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log_level);
}
