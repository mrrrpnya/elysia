use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use enum_table::{EnumTable, Enumable};
use reqwest::Url;
use serde::{Deserialize, Serialize};

mod dxvk;
mod jadeite;
pub mod tweaks;

use crate::components::{dxvk::Dxvk, jadeite::Jadeite};

#[derive(Serialize, PartialEq, Eq, Hash, Deserialize, Debug, Clone, Enumable, Copy)]
#[repr(u8)]
pub enum ComponentType {
    Dxvk,
    Jadeite,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentVersion {
    pub version: String,
    pub download_url: Url,
}

#[async_trait]
pub trait Component: Send + Sync {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    async fn fetch_versions(&self) -> Result<Vec<ComponentVersion>>;
}

#[derive(Serialize, Default, Deserialize)]
pub struct VersionCache {
    pub entries: HashMap<ComponentType, Vec<ComponentVersion>>,
}

pub struct ComponentManager {
    pub cache: VersionCache,
    pub components: EnumTable<ComponentType, Arc<dyn Component>, { ComponentType::COUNT }>,
}

impl ComponentManager {
    pub async fn new() -> Self {
        // TODO: save/load cache from file

        let components =
            EnumTable::<ComponentType, Arc<dyn Component>, { ComponentType::COUNT }>::new_with_fn(
                |t| match t {
                    ComponentType::Dxvk => Arc::new(Dxvk {}),
                    ComponentType::Jadeite => Arc::new(Jadeite {}),
                },
            );

        Self {
            cache: VersionCache::default(),
            components,
        }
    }

    pub async fn refresh_index(&mut self) -> Result<()> {
        let handles = self
            .components
            .iter()
            .map(|(name, component)| {
                let component = Arc::clone(component);
                let name = *name;
                tokio::spawn(async move { (name, component.fetch_versions().await) })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            match handle.await {
                Ok(handle) => {
                    let (name, versions) = handle;

                    if let Ok(versions) = versions {
                        self.cache.entries.insert(name, versions);
                    } else if let Err(err) = versions {
                        eprintln!("Failed to fetch versions for component {:?}: {}", name, err);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch versions for component: {}", err);
                }
            }
        }

        Ok(())
    }
}
