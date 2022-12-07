use bootloader_api::info::FrameBufferInfo;

use crate::{
    println,
    screen::{LockedScreen, SCREEN},
};

pub fn init(info: FrameBufferInfo) {
    if let Some(screen) = SCREEN.get() {
        log::set_logger(screen).expect("screen already set");
        log::set_max_level(log::LevelFilter::Trace);
        log::info!("Framebuffer info: {:?}", info);
    }
}

impl log::Log for LockedScreen {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("{:5}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
