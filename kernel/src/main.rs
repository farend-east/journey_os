#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(journey_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

use journey_kernel::task::{executor::Executor, keyboard, Task};
use journey_kernel::{println, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    journey_kernel::init(boot_info);
    log::info!("Running");

    // Run test harnest for `cargo test`
    #[cfg(test)]
    test_main();

    println!("Hello {}", "World");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // These are unreachable as our task executor currently is blocking the main thread.
    // log::info!("Halting");
    // journey_kernel::hlt_loop();
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
    log::error!("{}", info);
    journey_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    journey_kernel::test_panic_handler(info)
}
