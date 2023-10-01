mod cpu_arch;
mod ext;
mod os;
mod version_info;

use std::fmt::Display;

use cpu_arch::CpuArch;
use ext::Ext;
use os::Os;
pub use version_info::VersionInfo;

pub(crate) type Version = u8;

#[derive(Clone, Copy)]
pub struct DownloadableVersion(Version);

impl Display for DownloadableVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct DownloadURL(String);

pub struct JdkDesc {
    corretto_version: DownloadableVersion,
    cpu_arch: CpuArch,
    os: Os,
    file_extension: Ext,
}

impl JdkDesc {
    pub fn new(corretto_version: DownloadableVersion, cpu_arch: CpuArch, os: Os, ext: Ext) -> Self {
        Self {
            corretto_version,
            cpu_arch,
            os,
            file_extension: ext,
        }
    }
}

impl From<JdkDesc> for DownloadURL {
    fn from(desc: JdkDesc) -> Self {
        let JdkDesc {
            corretto_version,
            cpu_arch,
            os,
            file_extension,
        } = desc;

        let url = format!(
            "https://corretto.aws/downloads/latest/amazon-corretto-{corretto_version}-{cpu_arch}-{os}-jdk.{file_extension}"
        );
        DownloadURL(url)
    }
}
