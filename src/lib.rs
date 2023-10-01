mod cpu_arch;
mod ext;
mod jdk_desc;
mod os;
mod version_info;

use std::fmt::Display;

use serde::{Deserialize, Serialize};
pub use version_info::VersionInfo;

/// A newtype wrapper around a Corretto version.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Version(u8);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) struct DownloadURL(String);
