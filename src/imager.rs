use std::{fs, path::Path};

pub fn create_image(out_path: &Path, bootloader_path: &Path) {
    let bootloader_data =
        fs::read(bootloader_path).expect("Unabel to read from bootloader executable");
    fs::write(out_path, bootloader_data).expect("Unable to write to target/image_out");
}
