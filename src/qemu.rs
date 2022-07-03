use anyhow::{anyhow, Result};
use std::{
    path::Path,
    process::{Command, ExitStatus},
};

pub fn qemu_run(image: &Path) -> Result<ExitStatus> {
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={}", image.to_str().unwrap()));
    qemu.arg("-s");
    qemu.arg("-S");
    qemu.arg("--no-reboot");
    qemu.arg("-monitor").arg("stdio");
    qemu.arg("-d").arg("in_asm");
    qemu.arg("-m").arg("512");
    let status = qemu.status()?;
    println!("QEMU process exited with value: {status}");
    if status.success() {
        Ok(status)
    } else {
        Err(anyhow!("{status:?}"))
    }
}
