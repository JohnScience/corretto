use serde::Deserialize;
use sorted_vec2::SortedVec;

use crate::Version;

/// A struct describing the data from
/// <https://github.com/corretto/corretto-downloads/blob/ace4e5da8accabdcd7a214d86dbbdb7417fbf11a/latest_links/version-info.json>.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct VersionInfo {
    /// Supported Long-Term Support (LTS) releases.
    pub supported_lts_releases: SortedVec<Version>,
    /// Supported feature releases.
    pub supported_feature_releases: SortedVec<Version>,
    /// End-of-life releases.
    pub end_of_life_releases: SortedVec<Version>,
}

impl VersionInfo {
    /// The URL from which the version info is fetched.
    pub const URL: &'static str = "https://raw.githubusercontent.com/corretto/corretto-downloads/main/latest_links/version-info.json";

    /// Fetches the version info from the URL.
    pub async fn fetch() -> Result<Self, reqwest::Error> {
        let version_info: reqwest::Response = reqwest::get(Self::URL).await?;
        let version_info: VersionInfo = version_info.json().await?;
        Ok(version_info)
    }

    /// Returns the latest LTS release.
    ///
    /// # Panics
    ///
    /// Panics if no LTS releases are found.
    pub fn latest_lts(&self) -> Version {
        match self.latest_lts_checked() {
            Some(version) => version,
            None => unreachable!("No LTS releases found"),
        }
    }

    /// Returns the latest LTS release, if any available.
    pub fn latest_lts_checked(&self) -> Option<Version> {
        self.supported_lts_releases.last().copied()
    }

    /// Returns the latest release.
    ///
    /// # Panics
    ///
    /// Panics if no releases are found.
    pub fn latest(&self) -> Version {
        match self.latest_checked() {
            Some(version) => version,
            None => unreachable!("No releases found"),
        }
    }

    /// Returns the latest release, if any available.
    pub fn latest_checked(&self) -> Option<Version> {
        [
            &self.supported_lts_releases,
            &self.supported_feature_releases,
            &self.end_of_life_releases,
        ]
        .iter()
        .filter_map(|v| v.last().copied())
        .max()
    }

    fn downloadable_len(&self) -> usize {
        self.supported_lts_releases.len()
            + self.supported_feature_releases.len()
            + self.end_of_life_releases.len()
    }

    /// Returns a vector of all downloadable versions, locally sorted within
    /// LTS, feature, and end-of-life releases.
    pub fn downloadable_locally_sorted(&self) -> Vec<Version> {
        let mut versions = Vec::with_capacity(self.downloadable_len());
        versions.extend(self.supported_lts_releases.iter().copied());
        versions.extend(self.supported_feature_releases.iter().copied());
        versions.extend(self.end_of_life_releases.iter().copied());
        versions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn check_version_info() {
        let version_info = VersionInfo::fetch().await.unwrap();
        println!("{:#?}", version_info);
    }

    #[tokio::test]
    #[ignore]
    async fn check_latest_lts() {
        let version_info = VersionInfo::fetch().await.unwrap();
        println!("Latest LTS: {:#?}", version_info.latest_lts());
    }

    #[tokio::test]
    #[ignore]
    async fn check_downloadable_locally_sorted() {
        let version_info = VersionInfo::fetch().await.unwrap();
        println!(
            "Downloadable versions: {:#?}",
            version_info.downloadable_locally_sorted()
        );
    }
}
