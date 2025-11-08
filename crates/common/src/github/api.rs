use anyhow::Result;
use reqwest::Client;

use crate::{HTTP_CLIENT, github::proto::ReleasesResponse};

pub async fn releases(repo: &str) -> Result<ReleasesResponse> {
    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let releases = HTTP_CLIENT
        .get(&url)
        .send()
        .await?
        .json::<ReleasesResponse>()
        .await?;
    Ok(releases)
}
