use crate::cpu_arch::CpuArch;
use crate::ext::Ext;
use crate::os::Os;
use crate::{DownloadURL, Version};

/// A struct describing a Corretto JDK.
pub struct JdkDesc {
    /// The JDK version.
    corretto_version: Version,
    /// The CPU architecture.
    cpu_arch: CpuArch,
    /// The operating system.
    os: Os,
    /// The file extension.
    file_extension: Ext,
}

impl JdkDesc {
    /// Creates a new [`JdkDesc`] from the provided JDK version, CPU architecture, OS, and file extension.
    ///
    /// Note that not all combinations of these parameters are valid. Consult [Correto's website] for more.
    ///
    /// [Correto's website]: https://docs.aws.amazon.com/corretto/latest/corretto-20-ug/downloads-list.html
    pub fn new(corretto_version: Version, cpu_arch: CpuArch, os: Os, ext: Ext) -> Self {
        Self {
            corretto_version,
            cpu_arch,
            os,
            file_extension: ext,
        }
    }

    /// Provides the URL from which the JDK can be downloaded.
    ///
    /// Note that this URL may not be valid if the [`JdkDesc`] was created with invalid parameters.
    pub fn url(&self) -> DownloadURL {
        let JdkDesc {
            corretto_version,
            cpu_arch,
            os,
            file_extension,
        } = self;

        let url = format!(
            "https://corretto.aws/downloads/latest/amazon-corretto-{corretto_version}-{cpu_arch}-{os}-jdk.{file_extension}"
        );
        DownloadURL(url)
    }
}
