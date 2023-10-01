use std::fmt::Display;

#[non_exhaustive]
pub enum Ext {
    Deb,
    Rpm,
    TarGz,
    Msi,
    Pkg,
    Zip,
}

impl Display for Ext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ext::Deb => write!(f, "deb"),
            Ext::Rpm => write!(f, "rpm"),
            Ext::TarGz => write!(f, "tar.gz"),
            Ext::Msi => write!(f, "msi"),
            Ext::Pkg => write!(f, "pkg"),
            Ext::Zip => write!(f, "zip"),
        }
    }
}
