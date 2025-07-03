use acpi::platform::interrupt::IoApic;
use alloc::vec::Vec;

use crate::apic::io_apic::IoApicBase;

use self::values::RedirectionTableEntry;

extern crate bit_field;
extern crate volatile;

pub mod io_apic;
pub mod values;

pub fn init(io_apics: Vec<IoApic>) {
    log::info!("[APIC]: Init APIC, found {} IOAPICs", io_apics.iter().count());
    
    for io_apic in io_apics.iter() {
        log::info!(
            "[APIC]: IOAPIC id: {}, address: {}, GSIB: {}",
            io_apic.id,
            io_apic.address,
            io_apic.global_system_interrupt_base
        );
    }
}
