use std::fmt::{Display, Formatter};

/// A CPU architecture.
#[non_exhaustive]
pub enum CpuArch {
    /// x64 architecture (also known as amd64).
    X64,
    /// ARM 64-bit architecture.
    Aarch64,
    /// x86 64-bit architecture.
    X86_64,
}

impl Display for CpuArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuArch::X64 => write!(f, "x64"),
            CpuArch::Aarch64 => write!(f, "aarch64"),
            CpuArch::X86_64 => write!(f, "x86_64"),
        }
    }
}
