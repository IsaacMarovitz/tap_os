use crate::{apic, memory};
use acpi::{AcpiHandler, AcpiTables, InterruptModel, PhysicalMapping, PlatformInfo};
use core::ptr::NonNull;
use x86_64::PhysAddr;

// Referenced from https://github.com/vinc/moros/blob/trunk/src/sys/acpi.rs

#[derive(Clone)]
pub struct TapHandler;

impl AcpiHandler for TapHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let virtual_address = memory::phys_to_virt(PhysAddr::new(physical_address as u64));
        PhysicalMapping::new(
            physical_address,
            NonNull::new(virtual_address.as_mut_ptr()).unwrap(),
            size,
            size,
            Self,
        )
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        log::warn!("ACPI unmap not implemented");
    }
}

pub fn init(rspd: usize) {
    unsafe {
        let handler: TapHandler = TapHandler;
        let tables: AcpiTables<TapHandler>;
        match AcpiTables::from_rsdp(handler, rspd) {
            Ok(new_tables) => {
                tables = new_tables;
                log::info!("ACPI table created...")
            }
            Err(_) => {
                panic!("Failed to create ACPI Table!")
            }
        }

        match PlatformInfo::new(&tables) {
            Ok(platform_info) => match platform_info.interrupt_model {
                InterruptModel::Apic(apic) => {
                    apic::init(apic.io_apics.to_vec());
                }
                _ => {
                    panic!("Failed to get APIC!")
                }
            },
            Err(_) => {
                panic!("Failed to get platform info!")
            }
        }
    }
}
