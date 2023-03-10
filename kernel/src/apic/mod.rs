use acpi::{platform::interrupt::IoApic};
use alloc::vec::Vec;

extern crate bit_field;
extern crate volatile;

pub mod io_apic;
pub mod values;

pub fn init(io_apics: Vec<IoApic>) {
    log::info!("Init APIC, found {} IOAPICs", io_apics.iter().count());
    for io_apic in io_apics.iter() {
        log::info!("IOAPIC id: {}, address: {}, GSIB: {}", io_apic.id, io_apic.address, io_apic.global_system_interrupt_base);
        
    }
}