use std::fmt::{Display, Formatter};

/// An operating system.
#[non_exhaustive]
pub enum Os {
    /// Linux.
    Linux,
    /// Windows.
    Windows,
    /// macOS.
    MacOS,
    /// Alpine Linux.
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
