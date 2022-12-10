#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use bootloader_api::{entry_point, BootInfo};

use journey_kernel::{exit_qemu, QemuExitCode, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    journey_kernel::init(boot_info);

    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);

    exit_qemu(QemuExitCode::Success)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
}
