use reqwest;

pub mod api;

use api::{ApiResponse, Game, GetGames};
use serde::{Deserialize, Serialize};

use crate::game_providers::hoyoplay::api::GetGameContent;

const API_URL: &str = "https://sg-hyp-api.hoyoverse.com/hyp/hyp-connect/api";
const LAUNCHER_ID: &str = "VYTpXlbWo8";

const CACHE_DIR: &str = "./cache";

pub async fn get_games() -> Result<GetGames, String> {
    let url = format!("{API_URL}/getGames?launcher_id={LAUNCHER_ID}&language=en-us");

    return cached_request(&url).await;
}

pub async fn get_game_content(game_id: impl AsRef<str>) -> Result<GetGameContent, String> {
    let url = format!(
        "{API_URL}/getGameContent?game_id={}&launcher_id={LAUNCHER_ID}&language=en-us",
        game_id.as_ref()
    );

    return cached_request(&url).await;
}

async fn cached_request<Type>(url: &str) -> Result<Type, String>
where
    Type: for<'a> Deserialize<'a> + Serialize,
{
    if let Ok(asset) = cacache::read_sync(CACHE_DIR, url) {
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

    let _ = cacache::write_sync(CACHE_DIR, url, serde_json::to_vec(&response.data).unwrap());

    Ok(response.data)
}
