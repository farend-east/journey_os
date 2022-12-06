fn main() {
    // set by cargo, build scripts should use this directory for output files
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);

    // set by cargo's artifact dependency feature, see
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel = std::env::var_os("CARGO_BIN_FILE_JOURNEY_KERNEL").unwrap();
    let kernel = std::path::Path::new(&kernel);

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    // create a BIOS disk image (optional)
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
