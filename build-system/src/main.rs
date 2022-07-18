pub mod imager;
pub mod os_builder;
pub mod qemu;

use argh::FromArgs;
use colored::Colorize;
use std::{path::PathBuf, str::FromStr};

const RUSTLIN_VERSION: &'static str = "0.0.1";

#[derive(FromArgs)]
/// rustlin build utility.
struct Args {
    /// automatically boot qemu after build
    #[argh(switch)]
    qemu: bool,
}

fn main() {
    let arg: Args = argh::from_env();
    let os_builder = os_builder::OSBuilder {
        target: os_builder::Target::amd64 {
            kind: os_builder::amd64::TargetType::Bios,
        },
    };
    println!(
        "{}",
        format!("rustlin v.{RUSTLIN_VERSION}-{}", os_builder.target).bright_green()
    );
    println!("{}", format!("Building bootloader.").purple());
    let mut bootloader = os_builder.build_bootloader();
    println!("{}", format!("Building kernel.").purple());
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
    imager::create_image(&image_path, &bootloader_path);
    println!(
        "{}",
        format!(
            "Created rustlin image at {:?}",
            image_path.canonicalize().unwrap()
        )
        .blue()
    );
    if arg.qemu {
        qemu::qemu_run(&image_path).expect("QEMU failed on exit.");
    }
}
