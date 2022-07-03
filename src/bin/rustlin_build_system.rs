use std::{path::PathBuf, str::FromStr};

fn main() {
    let os_builder = os::OSBuilder {
        target: os::Target::amd64 {
            kind: os::amd64::TargetType::BIOS,
        },
    };
    let mut bootloader = os_builder.build_bootloader();
    //let kernel = os::build_kernel();
    assert!(
        bootloader.metadata.executables.len() == 1,
        "Bootloader may only produce one executable on build.\n{:?}",
        bootloader.metadata.executables
    );
    let bootloader_path = bootloader
        .metadata
        .executables
        .pop()
        .expect("Invalid bootloader path");
    let image_path = PathBuf::from_str("target/rustlin_image")
        .expect("Unalble to make a path to the image path.");
    rustlin::imager::create_image(&image_path, &bootloader_path);
    rustlin::qemu::qemu_run(&image_path).expect("QEMU failed on exit.");
}
