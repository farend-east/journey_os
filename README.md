# rust_os_journey

Rust OS Learning Journey.

## Learning Source

This OS development will follow the tutorial [https://os.phil-opp.com/](https://os.phil-opp.com/)

## Compilation Tools and Libraries

To turn our compiled kernel into a bootable disk image, we need to link it with a bootloader.

### Bootloader

[bootloader](https://github.com/rust-osdev/bootloader)
A crate that is build to handle the bootloader creation process for the OS.

### Bootimage

[bootimage](https://github.com/rust-osdev/bootimage)

A tool that links the kernal to the bootloader. Since cargo does not provide post compilation script, this tool will combine the the compiled bootloader and kernal file into one binary.
