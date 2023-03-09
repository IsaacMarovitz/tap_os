use acpi::{ AcpiHandler, PhysicalMapping };

#[derive(Clone)]
pub struct TapHandler;

impl AcpiHandler for TapHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        unimplemented!();
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {
        unimplemented!();
    }
}