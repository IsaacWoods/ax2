use crate::bootinfo::BootInfo;
use acpi::{AcpiTables, Handle, PciAddress, PhysicalMapping, aml::AmlError};
use core::ptr::NonNull;
use tracing::info;

pub fn find_tables(boot_info: &BootInfo) -> AcpiTables<AcpiHandler> {
    let Some(rsdp_addr) = boot_info.rsdp_addr() else {
        panic!("Cannot find RSDP address! Booting without ACPI is not supported");
    };

    let tables = match unsafe { AcpiTables::from_rsdp(AcpiHandler, rsdp_addr as usize) } {
        Ok(tables) => tables,
        Err(err) => panic!("Error parsing ACPI tables: {:?}", err),
    };

    info!("Found {} ACPI tables:", tables.table_headers().count());
    for (addr, table) in tables.table_headers() {
        info!(
            "    {} {:8x} {:4x} {:2x} {:6} {:8} {:2x} {:4} {:8x}",
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

    tables
}

#[derive(Clone)]
pub struct AcpiHandler;

impl acpi::Handler for AcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new(
                (hal::mem::kernel_map::PHYSICAL_MAPPING_BASE + physical_address).mut_ptr(),
            )
            .unwrap(),
            region_length: size,
            mapped_length: size,
            handler: self.clone(),
        }
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}

    fn read_u8(&self, address: usize) -> u8 {
        todo!()
    }

    fn read_u16(&self, address: usize) -> u16 {
        todo!()
    }

    fn read_u32(&self, address: usize) -> u32 {
        todo!()
    }

    fn read_u64(&self, address: usize) -> u64 {
        todo!()
    }

    fn write_u8(&self, address: usize, value: u8) {
        todo!()
    }

    fn write_u16(&self, address: usize, value: u16) {
        todo!()
    }

    fn write_u32(&self, address: usize, value: u32) {
        todo!()
    }

    fn write_u64(&self, address: usize, value: u64) {
        todo!()
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        todo!()
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        todo!()
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        todo!()
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        todo!()
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        todo!()
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        todo!()
    }

    fn read_pci_u8(&self, address: PciAddress, offset: u16) -> u8 {
        todo!()
    }

    fn read_pci_u16(&self, address: PciAddress, offset: u16) -> u16 {
        todo!()
    }

    fn read_pci_u32(&self, address: PciAddress, offset: u16) -> u32 {
        todo!()
    }

    fn write_pci_u8(&self, address: PciAddress, offset: u16, value: u8) {
        todo!()
    }

    fn write_pci_u16(&self, address: PciAddress, offset: u16, value: u16) {
        todo!()
    }

    fn write_pci_u32(&self, address: PciAddress, offset: u16, value: u32) {
        todo!()
    }

    fn nanos_since_boot(&self) -> u64 {
        todo!()
    }

    fn stall(&self, microseconds: u64) {
        todo!()
    }

    fn sleep(&self, milliseconds: u64) {
        todo!()
    }

    fn create_mutex(&self) -> Handle {
        todo!()
    }

    fn acquire(&self, mutex: Handle, timeout: u16) -> Result<(), AmlError> {
        todo!()
    }

    fn release(&self, mutex: Handle) {
        todo!()
    }
}
