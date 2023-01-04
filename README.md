# Journey OS

Rust OS Learning Journey.

## Learning Source

This OS development followed the tutorial [https://os.phil-opp.com/](https://os.phil-opp.com/)
Note that `bootloader` version `0.11.0` is incompatible with the guide shown above.
This OS has upgraded to its own fork of `bootloader` thus implements own code to cater for this changes.

## Compilation Tools and Libraries

To turn our compiled kernel into a bootable disk image, we need to link it with a bootloader.

### Bootloader

[bootloader](https://github.com/fadhliazhari/bootloader)

The bootloader for the OS. The original bootloader repo is [bootloader](https://github.com/rust-osdev/bootloader)

## Running

In order to run the os, you can call `cargo run`

The runner application is using [Qemu](https://www.qemu.org/) to boot the os

Runner:

- Build Script
  - Compile kernel
  - Create disk image
- Runner
  - Run using QEMU
