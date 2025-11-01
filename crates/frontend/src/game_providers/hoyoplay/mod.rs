pub mod api;

use std::path::PathBuf;

use api::{ApiResponse, Game, GetGameContent, GetGames};
use freya::prelude::{Readable, Signal};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::settings::GlobalSettings;

// FIXME: add all query params that should be there
// TODO: language selection
const API_URL: &str = "https://sg-hyp-api.hoyoverse.com/hyp/hyp-connect/api";
// TODO: other launchers (global, china, 3x bilibili?)
const LAUNCHER_ID: &str = "VYTpXlbWo8"; // Global

pub async fn get_games() -> Result<GetGames, String> {
    let url = format!("{API_URL}/getGames?launcher_id={LAUNCHER_ID}&language=en-us");

    return cached_request(&url).await;
}

pub async fn get_game_content(game_id: &str) -> Result<GetGameContent, String> {
    let url = format!(
        "{API_URL}/getGameContent?game_id={}&launcher_id={LAUNCHER_ID}&language=en-us",
        game_id
    );

    return cached_request(&url).await;
}

// TODO: cache invalidation on demand
async fn cached_request<Type>(url: &str) -> Result<Type, String>
where
    Type: for<'a> Deserialize<'a> + Serialize,
{
    let ctx = &dioxus::hooks::use_context::<Signal<GlobalSettings>>();
    let settings = &ctx.read();
    let cache_path = &settings.cache_directory;

    if let Ok(asset) = cacache::read_sync(cache_path, url) {
        let parsed = serde_json::from_slice(&asset).map_err(|e| format!("Cache parse error: {e}"));
        if let Ok(games) = parsed {
            return Ok(games);
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Api request error: {e}"))?;
    let response: ApiResponse<Type> = response
        .json()
        .await
        .map_err(|e| format!("Api parse error: {e}"))?;

    let _ = cacache::write_sync(cache_path, url, serde_json::to_vec(&response.data).unwrap());

    Ok(response.data)
}
