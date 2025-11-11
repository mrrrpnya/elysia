use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;

use crate::components::{Component, ComponentVersion};

pub struct Dxvk {}

#[async_trait]
impl Component for Dxvk {
    fn name(&self) -> &'static str {
        "dxvk"
    }

    fn display_name(&self) -> &'static str {
        "DXVK"
    }

    async fn fetch_versions(&self) -> Result<Vec<ComponentVersion>> {
        let repo = "doitsujin/dxvk";
        let releases = common::github::releases(repo).await?;
        let versions = releases.into_iter().filter_map(|rel| {
            rel.assets
                .iter()
                .filter_map(|asset| {
                    let version = &rel.tag_name[1..];
                    if asset.name == format!("dxvk-{}.tar.gz", version) {
                        Some(ComponentVersion {
                            version: rel.tag_name.clone(),
                            download_url: Url::parse(&rel.assets_url).ok()?,
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
