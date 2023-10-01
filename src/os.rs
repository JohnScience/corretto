use std::fmt::{Display, Formatter};

#[non_exhaustive]
pub enum Os {
    Linux,
    Windows,
    MacOS,
    Alpine,
}

impl Display for Os {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Os::Linux => write!(f, "linux"),
            Os::Windows => write!(f, "windows"),
            Os::MacOS => write!(f, "macos"),
            Os::Alpine => write!(f, "alpine"),
        }
    }
}
