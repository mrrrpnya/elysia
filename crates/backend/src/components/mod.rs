use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, Serialize};

mod dxvk;

use crate::components::dxvk::Dxvk;

#[derive(Serialize, PartialEq, Eq, Hash, Deserialize, Debug, Clone)]
pub enum ComponentType {
    Dxvk,
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
    pub components: HashMap<ComponentType, Arc<dyn Component>>,
}

impl ComponentManager {
    pub async fn new() -> Self {
        // TODO: save/load cache from file

        let mut components: HashMap<ComponentType, Arc<dyn Component>> = HashMap::new();

        components.insert(ComponentType::Dxvk, Arc::new(Dxvk {}));

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
                let name = name.clone();
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
