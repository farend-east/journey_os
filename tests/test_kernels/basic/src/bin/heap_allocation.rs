#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(journey_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

use journey_kernel::{exit_qemu, QemuExitCode};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // use journey_kernel::allocator;
    // use journey_kernel::memory::{self, BootInfoFrameAllocator};
    // use x86_64::VirtAddr;

    // journey_kernel::init(boot_info);
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // test_main();

    // Failing this test as the allocator is currently broken
    exit_qemu(QemuExitCode::Failed)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    journey_kernel::test_panic_handler(info)
}

// use alloc::boxed::Box;

// #[test_case]
// fn simple_allocation() {
//     let heap_value_1 = Box::new(41);
//     let heap_value_2 = Box::new(13);
//     assert_eq!(*heap_value_1, 41);
//     assert_eq!(*heap_value_2, 13);
// }

// use alloc::vec::Vec;

// #[test_case]
// fn large_vec() {
//     let n = 1000;
//     let mut vec = Vec::new();
//     for i in 0..n {
//         vec.push(i);
//     }
//     assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
// }

// use journey_kernel::allocator::HEAP_SIZE;

// #[test_case]
// fn many_boxes() {
//     for i in 0..HEAP_SIZE {
//         let x = Box::new(i);
//         assert_eq!(*x, i);
//     }
// }

// #[test_case]
// fn many_boxes_long_lived() {
//     let long_lived = Box::new(1);
//     for i in 0..HEAP_SIZE {
//         let x = Box::new(i);
//         assert_eq!(*x, i);
//     }
//     assert_eq!(*long_lived, 1);
// }
