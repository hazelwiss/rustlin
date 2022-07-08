mod bootloader;
mod kernel;

pub mod amd64 {
    pub enum TargetType {
        BIOS,
        UEFI,
    }
}

#[allow(non_camel_case_types)]
pub enum Target {
    amd64 { kind: amd64::TargetType },
}

pub struct OSBuilder {
    pub target: Target,
}
