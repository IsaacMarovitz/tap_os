use acpi::platform::interrupt::IoApic;
use alloc::vec::Vec;

pub fn init(io_apics: Vec<IoApic>) {
    for io_apic in io_apics.iter() {
    }
}