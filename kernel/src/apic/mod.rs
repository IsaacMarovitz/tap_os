use acpi::platform::interrupt::IoApic;
use alloc::vec::Vec;

use crate::apic::io_apic::IoApicBase;

use self::values::RedirectionTableEntry;

extern crate bit_field;
extern crate volatile;

pub mod io_apic;
pub mod values;

pub fn init(io_apics: Vec<IoApic>) {
    log::info!("Init APIC, found {} IOAPICs", io_apics.iter().count());
    for io_apic in io_apics.iter() {
        log::info!(
            "IOAPIC id: {}, address: {}, GSIB: {}",
            io_apic.id,
            io_apic.address,
            io_apic.global_system_interrupt_base
        );
        let base_addr: *mut u8 = &mut (io_apic.address as u8);
        let mut io_apic_base = unsafe { IoApicBase::new(base_addr) };
        // TODO: Currently breaks
        // io_apic_base.read_redirection_table_entry(0).set_masked(true);
        // io_apic_base.update_redirection_table_entry(0, test);
    }
}

fn test(value: &mut RedirectionTableEntry) {
    log::info!("Hello from IRQ 1");
}
