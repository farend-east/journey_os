use crate::serial::{LockedSerialPort, SERIAL1};

pub fn init() {
    log::set_logger(SERIAL1.get_or_init(LockedSerialPort::default)).expect("logger already set");
    log::set_max_level(log::LevelFilter::Trace);
}
