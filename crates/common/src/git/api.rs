use anyhow::Result;
use reqwest::Client;

use crate::{HTTP_CLIENT, git::proto::ReleasesResponse};

pub async fn github_releases(repo: &str) -> Result<ReleasesResponse> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let releases = HTTP_CLIENT
        .get(&url)
        .send()
        .await?
        .json::<ReleasesResponse>()
        .await?;
    Ok(releases)
}

pub async fn codeberg_releases(repo: &str) -> Result<ReleasesResponse> {
    let url = format!("https://codeberg.org/api/v1/repos/{}/releases", repo);
    let releases = HTTP_CLIENT
        .get(&url)
        .send()
        .await?
        .json::<ReleasesResponse>()
        .await?;
    Ok(releases)
}
