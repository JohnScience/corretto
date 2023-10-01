# Library for installing and finding info about Corretto OpenJDK

[![Crates.io](https://img.shields.io/crates/v/corretto)](https://crates.io/crates/corretto)
[![Downloads](https://img.shields.io/crates/d/corretto.svg)](https://crates.io/crates/corretto)
[![Documentation](https://docs.rs/corretto/badge.svg)](https://docs.rs/corretto)
[![License](https://img.shields.io/crates/l/corretto)](https://crates.io/crates/corretto)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/corretto/status.svg)](https://deps.rs/repo/github/JohnScience/corretto)

Get programmatic access to the [Corretto OpenJDK](https://aws.amazon.com/corretto/) distributions.

According to Amazon,

> Amazon Corretto is a no-cost, multiplatform, production-ready distribution of the Open Java Development Kit (OpenJDK). Corretto comes with long-term support that includes performance enhancements and security fixes. Corretto is certified as compatible with the Java SE standard and is used internally at Amazon for many production services. With Corretto, you can develop and run Java applications on operating systems such as Amazon Linux 2, Windows, and macOS.

## Usage

```rust,no_run
use corretto::{VersionInfo, Version, Os, CpuArch, Ext, JdkDesc, DownloadError};

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let version_info = VersionInfo::fetch().await?;
    let latest_lts = version_info.latest_lts();
    let jdk_desc = JdkDesc::new(
        latest_lts,
        CpuArch::X64,
        Os::Linux,
        Ext::TarGz,
    );
    let download_url = jdk_desc.url();
    let path = download_url.download_tmp().await?;
    // ..
    Ok(())
}
```
