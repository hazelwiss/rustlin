use std::{fs, path::Path, process::Command};

pub fn create_image(out_path: &Path, bootloader_path: &Path) {
    let bootloader_data =
        fs::read(bootloader_path).expect("Unabel to read from bootloader executable");
    fs::write(out_path, bootloader_data).expect("Unable to write to target/image_out");
    strip_elf(out_path);
}

fn strip_elf(path: &Path) {
    let output = Command::new("objcopy")
        .arg(path)
        .arg("-O")
        .arg("binary")
        .output()
        .expect("Unable to run objcopy to create the final image");
    assert!(
        output.status.success(),
        "Failed to run objcopy!\n{}",
        String::from_utf8(output.stderr).expect("stderr from objcopy is not valid utf8")
    );
}
