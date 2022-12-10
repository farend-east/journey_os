#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use bootloader_api::{entry_point, BootInfo};
use journey_kernel::allocator::HEAP_SIZE;

use journey_kernel::{exit_qemu, QemuExitCode, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    journey_kernel::init(boot_info);

    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);

    exit_qemu(QemuExitCode::Success)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
}
