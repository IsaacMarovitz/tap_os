use core::ptr::NonNull;
use acpi::{AcpiHandler, PhysicalMapping, AcpiTables};
use x86_64::PhysAddr;
use memory;

// Referenced from https://github.com/vinc/moros/blob/trunk/src/sys/acpi.rs

#[derive(Clone)]
pub struct TapHandler;

impl AcpiHandler for TapHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        let virtual_address = memory::phys_to_virt(PhysAddr::new(physical_address as u64));
        PhysicalMapping::new(physical_address, NonNull::new(virtual_address.as_mut_ptr()).unwrap(), size, size, Self)
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        unimplemented!();
    }
}

pub fn init_acpi(rspd: usize) {
    unsafe {
        let handler: TapHandler = TapHandler;
        match AcpiTables::from_rsdp(handler, rspd) {
            Ok(_) => {log::info!("ACPI Table creation succeeded")},
            Err(_) => {panic!("Failed to create ACPI Table!")},
        }
    }
}