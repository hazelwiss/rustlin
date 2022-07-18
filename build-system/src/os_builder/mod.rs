use std::fmt::Display;

mod bootloader;
mod kernel;

pub mod amd64 {
    use std::fmt::Display;

    pub enum TargetType {
        Bios,
        Uefi,
    }

    impl Display for TargetType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                TargetType::Bios => "bios",
                TargetType::Uefi => "uefi",
            })
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Target {
    amd64 { kind: amd64::TargetType },
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Target::amd64 { kind } => format!("amd64-{kind}"),
        })
    }
}

pub struct OSBuilder {
    pub target: Target,
}
