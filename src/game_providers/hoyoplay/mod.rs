use reqwest;

pub mod api;

use api::{ApiResponse, Game, GetGames};

use crate::game_providers::api::GetGameContent;

const API_URL: &str = "https://sg-hyp-api.hoyoverse.com/hyp/hyp-connect/api";
const LAUNCHER_ID: &str = "VYTpXlbWo8";

pub async fn get_games() -> Result<GetGames, String> {
    let client = reqwest::Client::new();
    let url = format!("{API_URL}/getGames?launcher_id={LAUNCHER_ID}&language=en-us");
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Api request error: {e}"))?;
    let response: ApiResponse<GetGames> = response
        .json()
        .await
        .map_err(|e| format!("Api parse error: {e}"))?;

    Ok(response.data)
}

pub async fn get_game_content(game_id: impl AsRef<str>) -> Result<GetGameContent, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{API_URL}/getGameContent?game_id={}&launcher_id={LAUNCHER_ID}&language=en-us",
        game_id.as_ref()
    );
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Api request error: {e}"))?;
    let response: ApiResponse<GetGameContent> = response
        .json()
        .await
        .map_err(|e| format!("Api parse error: {e}"))?;

    Ok(response.data)
}
