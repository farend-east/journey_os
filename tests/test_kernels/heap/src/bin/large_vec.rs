#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use bootloader_api::{entry_point, BootInfo};

use journey_kernel::{exit_qemu, QemuExitCode, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    journey_kernel::init(boot_info);

    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);

    exit_qemu(QemuExitCode::Success)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
}
