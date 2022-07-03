use super::OSBuilder;
use std::path::{Path, PathBuf};

pub struct BootloaderBuild {
    pub metadata: cargo::Metadata,
}

impl OSBuilder {
    pub fn build_bootloader(&self) -> BootloaderBuild {
        let path = &Path::new("os/bootloader").canonicalize().unwrap();
        let target = PathBuf::from("os/bootloader/targets/amd64-bios.json")
            .canonicalize()
            .unwrap();
        let metadata = cargo::build(
            path,
            cargo::LinkFlags::default(),
            cargo::CompileFlags {
                release: true,
                target: Some(cargo::targets::Targets::Custom(
                    target.to_str().unwrap().to_string(),
                )),
                bin: Some("amd64-bios".to_string()),
            },
        )
        .expect("Failed to run cargo build when building bootloader");
        BootloaderBuild { metadata }
    }
}
