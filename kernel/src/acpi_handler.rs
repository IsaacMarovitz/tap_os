use crate::{apic, memory};
use acpi::platform::{AcpiPlatform, InterruptModel};
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use alloc::boxed::Box;
use aml::{AmlContext, DebugVerbosity, Handler};
use core::fmt::Display;
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

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}

#[derive(Clone)]
pub struct TapAmlHandler;

impl Handler for TapAmlHandler {
    fn read_u8(&self, address: usize) -> u8 {
        read_addr::<u8>(address)
    }

    fn read_u16(&self, address: usize) -> u16 {
        read_addr::<u16>(address)
    }

    fn read_u32(&self, address: usize) -> u32 {
        read_addr::<u32>(address)
    }

    fn read_u64(&self, address: usize) -> u64 {
        read_addr::<u64>(address)
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        write_addr::<u8>(address, value);
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        write_addr::<u16>(address, value);
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        write_addr::<u32>(address, value);
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        write_addr::<u64>(address, value);
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86_64::instructions::port::Port::new(port).read() }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86_64::instructions::port::Port::new(port).read() }
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86_64::instructions::port::Port::new(port).read() }
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe {
            x86_64::instructions::port::Port::new(port).write(value);
        }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe {
            x86_64::instructions::port::Port::new(port).write(value);
        }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe {
            x86_64::instructions::port::Port::new(port).write(value);
        }
    }

    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        todo!()
    }

    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        todo!()
    }

    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        todo!()
    }

    fn write_pci_u8(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u8,
    ) {
        todo!()
    }

    fn write_pci_u16(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u16,
    ) {
        todo!()
    }

    fn write_pci_u32(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u32,
    ) {
        todo!()
    }
}

fn read_addr<T>(addr: usize) -> T
where
    T: Copy,
{
    let virtual_address = memory::phys_to_virt(PhysAddr::new(addr as u64));
    unsafe { *virtual_address.as_ptr::<T>() }
}

fn write_addr<T>(addr: usize, value: T)
where
    T: Copy + Display,
{
    let virtual_address = memory::phys_to_virt(PhysAddr::new(addr as u64));
    unsafe {
        *virtual_address.as_mut_ptr::<T>() = value;
    }
}

pub fn init(rspd: usize) {
    let acpi: AcpiTables<TapHandler>;

    unsafe {
        match AcpiTables::from_rsdp(TapHandler, rspd) {
            Ok(tables) => {
                acpi = tables;
                log::info!("[ACPI]: ACPI table created...")
            }
            Err(_) => {
                panic!("[ACPI]: Failed to create ACPI Table!")
            }
        }
    }

    for (addr, table) in acpi.table_headers() {
        log::info!(
            "[ACPI]: {} {:8x} {:4x} {:2x} {:6} {:8} {:2x} {:4} {:8x}",
            table.signature,
            addr,
            table.length(),
            table.revision(),
            table.oem_id().unwrap_or("??????"),
            table.oem_table_id().unwrap_or("????????"),
            table.oem_revision(),
            table.creator_id().unwrap_or("????"),
            table.creator_revision(),
        );
    }

    if let Ok(dsdt) = acpi.dsdt() {
        let virtual_address = memory::phys_to_virt(PhysAddr::new(dsdt.phys_address as u64));
        let table =
            unsafe { core::slice::from_raw_parts(virtual_address.as_ptr(), dsdt.length as usize) };

        let handler = Box::new(TapAmlHandler);
        let mut aml = AmlContext::new(handler, DebugVerbosity::All);
        if aml.parse_table(table).is_ok() {
            log::info!("[ACPI]: AML DSDT parsed successfully");
        } else {
            log::error!("[ACPI]: Could not parse AML in DSDT");
        }
    }

    match AcpiPlatform::new(acpi) {
        Ok(platform_info) => match platform_info.interrupt_model {
            InterruptModel::Apic(apic) => {
                apic::init(apic.io_apics.to_vec());
            }
            _ => {
                panic!("[ACPI]: Failed to get APIC!")
            }
        },
        Err(_) => {
            panic!("[ACPI]: Failed to get platform info!")
        }
    }
}
