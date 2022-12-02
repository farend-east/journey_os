#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_journey::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_journey::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os_journey::init();

    #[cfg(test)]
    test_main();

    println!("Running");
    rust_os_journey::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os_journey::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_journey::test_panic_handler(info)
}
