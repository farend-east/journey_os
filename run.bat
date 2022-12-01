cargo build
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target\x86_64-rust_os_journey\debug\bootimage-rust_os_journey.bin