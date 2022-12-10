use conquer_once::spin::OnceCell;
use core::fmt::Write;
use spinning_top::Spinlock;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts;

use crate::{print, serial_println};

/// The global screen instance used for the `screen` crate.
pub static SERIAL1: OnceCell<LockedSerialPort> = OnceCell::uninit();

pub struct LockedSerialPort(Spinlock<SerialPort>);

impl Default for LockedSerialPort {
    fn default() -> Self {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        LockedSerialPort(Spinlock::new(serial_port))
    }
}

impl log::Log for LockedSerialPort {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        serial_println!("{:5}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        if let Some(serial) = SERIAL1.get() {
            serial
                .0
                .lock()
                .write_fmt(args)
                .expect("Printing to serial failed");
        }
        // Print the log to screen if screen is available
        print!("{}", args);
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
