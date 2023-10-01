use crate::cpu_arch::CpuArch;
use crate::ext::Ext;
use crate::os::Os;
use crate::{DownloadURL, Version};

pub struct JdkDesc {
    corretto_version: Version,
    cpu_arch: CpuArch,
    os: Os,
    file_extension: Ext,
}

impl JdkDesc {
    pub fn new(corretto_version: Version, cpu_arch: CpuArch, os: Os, ext: Ext) -> Self {
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
