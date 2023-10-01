use serde::Deserialize;
use sorted_vec2::SortedVec;

use crate::Version;

/// A struct describing the data from
/// <https://github.com/corretto/corretto-downloads/blob/ace4e5da8accabdcd7a214d86dbbdb7417fbf11a/latest_links/version-info.json>.
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct VersionInfo {
    pub supported_lts_releases: SortedVec<Version>,
    pub supported_feature_releases: SortedVec<Version>,
    pub end_of_life_releases: SortedVec<Version>,
}

impl VersionInfo {
    pub const URL: &'static str = "https://raw.githubusercontent.com/corretto/corretto-downloads/main/latest_links/version-info.json";

    pub async fn fetch() -> Result<Self, reqwest::Error> {
        let version_info: reqwest::Response = reqwest::get(Self::URL).await?;
        let version_info: VersionInfo = version_info.json().await?;
        Ok(version_info)
    }

    pub fn latest_lts(&self) -> Version {
        match self.supported_lts_releases.last().copied() {
            Some(version) => version,
            None => unreachable!("No LTS releases found"),
        }
    }

    pub fn latest(&self) -> Version {
        let latest_release = [
            &self.supported_lts_releases,
            &self.supported_feature_releases,
            &self.end_of_life_releases,
        ]
        .iter()
        .map(|v| v.last().copied())
        .filter_map(|v| v)
        .max();
        match latest_release {
            Some(version) => version,
            None => unreachable!("No releases found"),
        }
    }

    fn downloadable_len(&self) -> usize {
        self.supported_lts_releases.len()
            + self.supported_feature_releases.len()
            + self.end_of_life_releases.len()
    }

    pub fn downloadable(&self) -> Vec<Version> {
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
    async fn check_latest() {
        let version_info = VersionInfo::fetch().await.unwrap();
        println!("{:#?}", version_info);
    }
}
