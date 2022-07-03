#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Targets {
    Custom(String),
}

impl ToString for Targets {
    fn to_string(&self) -> String {
        match self {
            Self::Custom(path) => path.as_str(),
        }
        .to_string()
    }
}
