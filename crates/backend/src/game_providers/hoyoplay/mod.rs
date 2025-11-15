pub mod proto;

use std::{
    collections::HashMap,
    fs::read_dir,
    io::Read,
    path::{Path, PathBuf},
};

use md5::Digest;
use proto::{ApiResponse, Game, GetGameContent, GetGames};
use reqwest;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    game_providers::hoyoplay::proto::{GameExe, GameInfo, GetGameConfigs, GetGameScanInfo},
    settings::GlobalSettings,
};

// FIXME: add all query params that should be there
// TODO: language selection
const API_URL: &str = "https://sg-hyp-api.hoyoverse.com/hyp/hyp-connect/api";
// TODO: other launchers (global, china, 3x bilibili?)
const LAUNCHER_ID: &str = "VYTpXlbWo8"; // Global

pub async fn get_games(settings: &GlobalSettings) -> Result<GetGames, String> {
    let url = format!("{API_URL}/getGames?launcher_id={LAUNCHER_ID}&language=en-us");

    return cached_request(settings, &url).await;
}

pub async fn get_game_content(
    settings: &GlobalSettings,
    game_id: &str,
) -> Result<GetGameContent, String> {
    let url = format!(
        "{API_URL}/getGameContent?game_id={}&launcher_id={LAUNCHER_ID}&language=en-us",
        game_id
    );

    return cached_request(settings, &url).await;
}

pub async fn get_game_scan_info(settings: &GlobalSettings) -> Result<GetGameScanInfo, String> {
    let url = format!("{API_URL}/getGameScanInfo?launcher_id={LAUNCHER_ID}&language=en-us");

    return cached_request(settings, &url).await;
}

pub async fn get_game_configs(settings: &GlobalSettings) -> Result<GetGameConfigs, String> {
    let url = format!("{API_URL}/getGameConfigs?launcher_id={LAUNCHER_ID}&language=en-us");

    return cached_request(settings, &url).await;
}

pub async fn scan_dir(
    settings: &GlobalSettings,
    path: &Path,
) -> Result<Vec<(String, String)>, String> {
    let configs = get_game_configs(settings).await?;
    let map: HashMap<String, String> = configs
        .launch_configs
        .iter()
        .map(|v| (v.exe_file_name.clone(), v.game.id.clone()))
        .collect();

    let mut id_to_exe = HashMap::new();
    scan(path, &map, 0, &mut id_to_exe);

    let scan_info = get_game_scan_info(settings).await?;

    let mut out = Vec::new();
    for game in scan_info.game_scan_info {
        if let Some(exe) = id_to_exe.get(&game.game_id) {
            let hash = md5(exe).await?;
            let version = game.game_exe_list.into_iter().find(|v| v.md5 == hash);

            if let Some(version) = version {
                out.push((exe.to_string_lossy().to_string(), version.version));
            }
        }
    }

    Ok(out)
}

async fn md5(path: &Path) -> Result<String, String> {
    let mut file = File::open(path)
        .await
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut hasher = md5::Context::new();
    let mut buffer = vec![0; 1024];

    loop {
        let size = file
            .read(&mut buffer)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if size == 0 {
            break; // EOF
        }

        hasher.consume(&buffer[..size]);
    }

    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    Ok(hash_hex)
}

fn scan(
    dir: &Path,
    exe_to_id: &HashMap<String, String>,
    depth: u32,
    out: &mut HashMap<String, PathBuf>,
) {
    if depth > 1 {
        return;
    }

    if let Ok(entries) = read_dir(dir) {
        for entry in entries.into_iter().flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan(&path, exe_to_id, depth + 1, out);
            } else if let Some(file_name) = path.file_name()
                && let Some(name_str) = file_name.to_str()
                && exe_to_id.contains_key(&name_str.to_string())
            {
                out.insert(exe_to_id[&name_str.to_string()].clone(), path);
            }
        }
    }
}

// TODO: cache invalidation on demand
async fn cached_request<Type>(settings: &GlobalSettings, url: &str) -> Result<Type, String>
where
    Type: for<'a> Deserialize<'a> + Serialize,
{
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
