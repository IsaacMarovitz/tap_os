use core::convert::TryInto;
use core::ptr::NonNull;
use crate::apic::values;
use super::bit_field::BitField;
use super::volatile::VolatilePtr;

// Referenced from https://github.com/rust-osdev/apic/blob/master/src/io_apic/mod.rs

pub struct IoApicBase<'a> {
    pub select: VolatilePtr<'a, Select>,
    pub window: VolatilePtr<'a, u32>,
}

impl IoApicBase<'_> {
    pub unsafe fn new(base_addr: *mut u8) -> Self {
        Self {
            select: Self::offset(base_addr, Offset::Select),
            window: Self::offset(base_addr, Offset::Window),
        }
    }

    pub fn read_id(&mut self) -> u8 {
        self.select.update(|mut v| v.set_index(Index::Id));
        self.window.read().get_bits(24..28).try_into().unwrap()
    }

    pub fn read_version(&mut self) -> values::Version {
        self.select.update(|mut v| v.set_index(Index::Version));
        values::Version::from_raw(self.window.read())
    }

    pub fn read_arbitration(&mut self) -> values::Arbitration {
        self.select.update(|mut v| v.set_index(Index::Arbitration));
        values::Arbitration::from_raw(self.window.read())
    }

    pub fn write_arbitration(&mut self, value: values::Arbitration) {
        self.select.update(|mut v| v.set_index(Index::Arbitration));
        self.window.write(value.into_raw());
    }

    pub fn read_redirection_table_entry(&mut self, irq: u8) -> values::RedirectionTableEntry {
        assert!(irq < 24);

        let index_low = Index::RedirectionTableEntryBase as u8 + irq * 2;
        let index_high = index_low + 1;

        self.select.update(|mut v| v.set_index(index_low));
        let low = self.window.read();

        self.select.update(|mut v| v.set_index(index_high));
        let high = self.window.read();

        values::RedirectionTableEntry::from_raw(low, high)
    }

    pub fn write_redirection_table_entry(&mut self, irq: u8, value: values::RedirectionTableEntry) {
        assert!(irq < 24);

        let index_low = Index::RedirectionTableEntryBase as u8 + irq * 2;
        let index_high = index_low + 1;

        let (low, high) = value.into_raw();

        self.select.update(|mut v| v.set_index(index_low));
        self.window.write(low);

        self.select.update(|mut v| v.set_index(index_high));
        self.window.write(high);
    }

    pub fn update_redirection_table_entry<F>(&mut self, irq: u8, f: F)
    where
        F: FnOnce(&mut values::RedirectionTableEntry),
    {
        assert!(irq < 24);

        let mut value = self.read_redirection_table_entry(irq);
        f(&mut value);
        self.write_redirection_table_entry(irq, value);
    }

    unsafe fn offset<'a, T>(base_addr: *mut u8, offset: Offset) -> VolatilePtr<'a, T> {
        let ptr = Self::offset_ptr(base_addr, offset).cast();
        VolatilePtr::new(NonNull::new_unchecked(ptr))
    }

    fn offset_ptr(base_addr: *mut u8, offset: Offset) -> *mut u8 {
        base_addr.wrapping_add(offset as usize)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Offset {
    Select = 0x0,
    Window = 0x10,
}

#[repr(u8)]
pub enum Index {
    Id = 0x0,
    Version = 0x1,
    Arbitration = 0x2,
    RedirectionTableEntryBase = 0x10,
}

impl Into<u8> for Index {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Select(u32);

impl Select {
    pub fn new(index: u8) -> Self {
        Self(index.into())
    }

    pub fn set_index(&mut self, index: impl Into<u8>) -> Self {
        self.0.set_bits(0..8, index.into().into());
        *self
    }
}
