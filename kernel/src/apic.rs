use acpi::AcpiHandler;
use crossbeam_utils::atomic::AtomicCell;
use raw_cpuid::CpuId;

/// The interrupt chip that is currently configured on this machine.
/// The default is `InterruptChip::PIC`, but the typical case is `APIC` or `X2APIC`,
/// which will be set once those chips have been initialized.
pub static INTERRUPT_CHIP: AtomicCell<InterruptChip> = AtomicCell::new(InterruptChip::PIC);

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum InterruptChip {
    APIC,
    X2APIC,
    PIC,
}

pub fn init() {
    let cpuid = CpuId::new();

    if let Some(id) = cpuid.get_processor_brand_string() {
        log::info!("CPUID: {}", id.as_str());
    }

    let interrupt_chip = cpuid
        .get_feature_info()
        .map(|finfo| {
            if finfo.has_apic() {
                log::info!("APIC supported!");
                return InterruptChip::APIC;
            }

            if finfo.has_x2apic() {
                log::info!("x2APIC supported!");
                return InterruptChip::X2APIC;
            }

            InterruptChip::PIC
        })
        .expect("Could not retrieve CPUID feature info");

    INTERRUPT_CHIP.store(interrupt_chip);
}

#[derive(Clone)]
struct JourneyAcpiHandler;

impl AcpiHandler for JourneyAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        _physical_address: usize,
        _size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        todo!()
    }

    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) {
        todo!()
    }
}
