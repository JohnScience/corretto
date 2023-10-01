#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod cpu_arch;
mod ext;
mod jdk_desc;
mod os;
mod version_info;

use std::{
    fmt::Display,
    io::Write,
    path::{Path, PathBuf},
};

pub use cpu_arch::CpuArch;
pub use ext::Ext;
pub use jdk_desc::JdkDesc;
pub use os::Os;
use serde::{Deserialize, Serialize};
pub use version_info::VersionInfo;

/// An error that can occur while downloading a JDK.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    /// An IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// A [reqwest] error.
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Options for downloading a JDK.
pub struct DownloadOpts {
    /// Whether to create the directory (and all parent directories) in which the JDK is downloaded.
    pub create_dir: bool,
}

impl Default for DownloadOpts {
    fn default() -> Self {
        Self { create_dir: true }
    }
}

/// A newtype wrapper around a version of Corretto distribution of Open Java Development Kit ([OpenJDK]).
///
/// [OpenJDK]: https://en.wikipedia.org/wiki/OpenJDK
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Version(u8);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A URL from which a JDK can be downloaded.
pub struct DownloadURL(String);

impl DownloadURL {
    /// Downloads the JDK from the URL to the specified directory. Returns the path to the downloaded file.
    pub async fn download(
        &self,
        dir_path: impl AsRef<Path>,
        opts: DownloadOpts,
    ) -> Result<PathBuf, DownloadError> {
        let DownloadOpts { create_dir } = opts;

        let DownloadURL(url) = self;
        let filename = url
            .strip_prefix("https://corretto.aws/downloads/latest/")
            .unwrap();
        let response = reqwest::get(url).await?;
        let path = dir_path.as_ref().join(filename);
        if create_dir {
            std::fs::create_dir_all(&dir_path)?;
        }
        let mut file = std::fs::File::create(&path)?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes)?;
        Ok(path)
    }

    /// Downloads the JDK from the URL to a temporary directory. Returns the path to the downloaded file.
    pub async fn download_tmp(&self) -> Result<PathBuf, DownloadError> {
        let dir = tempfile::tempdir()?;
        self.download(dir.path(), DownloadOpts { create_dir: false })
            .await
    }
}
