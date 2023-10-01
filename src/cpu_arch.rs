use std::fmt::{Display, Formatter};

#[non_exhaustive]
pub enum CpuArch {
    X64,
    Aarch64,
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
