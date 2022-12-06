#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(journey_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::info::FrameBuffer;
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use journey_kernel::memory::{self, BootInfoFrameAllocator};
use journey_kernel::task::{executor::Executor, keyboard, Task};
use journey_kernel::{allocator, println, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);
// entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // println!("Hello World{}", "!");

    // journey_kernel::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // #[cfg(test)]
    // test_main();
    //println!("Running");

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.run();

    // These are unreachable as our task executor currently is blocking the main thread.
    //println!("Exiting");
    journey_kernel::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    journey_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    journey_kernel::test_panic_handler(info)
}
