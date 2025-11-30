use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;

use crate::components::{Component, ComponentVersion};

pub struct Jadeite {}

#[async_trait]
impl Component for Jadeite {
    fn name(&self) -> &'static str {
        "jadeite"
    }

    fn display_name(&self) -> &'static str {
        "Jadeite"
    }

    async fn fetch_versions(&self) -> Result<Vec<ComponentVersion>> {
        let repo = "mkrsym1/jadeite";
        let releases = common::git::codeberg_releases(repo).await?;
        let versions = releases.into_iter().filter_map(|rel| {
            rel.assets
                .iter()
                .filter_map(|asset| {
                    if asset.name == format!("{}.zip", &rel.tag_name) {
                        Some(ComponentVersion {
                            version: rel.tag_name.clone(),
                            download_url: Url::parse(&asset.browser_download_url).ok()?,
                        })
                    } else {
                        None
                    }
                })
                .next()
        });
        Ok(versions.collect())
    }
}
