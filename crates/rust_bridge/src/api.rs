// Flutter Rust Bridge API
// This module exposes the backend APIs to Flutter via flutter_rust_bridge
// Uses JSON serialization for complex types to ensure compatibility

use serde::{Deserialize, Serialize};

// ============================================================================
// Simple DTO types for FFI (JSON serializable)
// ============================================================================

/// Simple game data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameDto {
    pub id: String,
    pub biz: String,
    pub name: String,
    pub title: String,
    pub subtitle: String,
    pub icon_url: String,
    pub background_url: String,
    pub logo_url: String,
    pub display_status: String,
}

/// Simple game content data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentDto {
    pub game_id: String,
    pub game_biz: String,
    pub language: String,
    pub banners: Vec<BannerDto>,
    pub posts: Vec<PostDto>,
}

/// Simple banner data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerDto {
    pub id: String,
    pub image_url: String,
    pub link: String,
}

/// Simple post data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostDto {
    pub id: String,
    pub post_type: String,
    pub title: String,
    pub link: String,
    pub date: String,
}

/// Download progress information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DownloadProgressDto {
    pub downloaded: u64,
    pub total: u64,
    pub mb_per_second: f32,
    pub part_index: usize,
    pub parts_total: usize,
    pub status: String,
    pub is_busy: bool,
}

/// Settings paths for display
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsDto {
    pub wineprefixes_directory: String,
    pub components_directory: String,
    pub temp_directory: String,
    pub cache_directory: String,
}

// ============================================================================
// Conversion functions
// ============================================================================

fn game_to_dto(game: &backend::game_providers::hoyoplay::proto::Game) -> GameDto {
    GameDto {
        id: game.id.clone(),
        biz: game.biz.clone(),
        name: game.display.name.clone(),
        title: game.display.title.clone(),
        subtitle: game.display.subtitle.clone(),
        icon_url: game.display.icon.url.clone(),
        background_url: game.display.background.url.clone(),
        logo_url: game.display.logo.url.clone(),
        display_status: game.display_status.clone(),
    }
}

fn content_to_dto(content: &backend::game_providers::hoyoplay::proto::Content) -> ContentDto {
    ContentDto {
        game_id: content.game.id.clone(),
        game_biz: content.game.biz.clone(),
        language: content.language.clone(),
        banners: content
            .banners
            .iter()
            .map(|b| BannerDto {
                id: b.id.clone(),
                image_url: b.image.url.clone(),
                link: b.image.link.clone(),
            })
            .collect(),
        posts: content
            .posts
            .iter()
            .map(|p| PostDto {
                id: p.id.clone(),
                post_type: p.post_type.clone(),
                title: p.title.clone(),
                link: p.link.clone(),
                date: p.date.clone(),
            })
            .collect(),
    }
}

// ============================================================================
// API Functions - All return JSON strings for maximum compatibility
// ============================================================================

/// Initialize the backend (call on app startup)
#[flutter_rust_bridge::frb(sync)]
pub fn init_backend() -> String {
    "ok".to_string()
}

/// Get settings as JSON
#[flutter_rust_bridge::frb(sync)]
pub fn get_settings_json() -> String {
    match backend::settings::GlobalSettings::load() {
        Ok(settings) => {
            let dto = SettingsDto {
                wineprefixes_directory: settings.wineprefixes_directory.to_string_lossy().to_string(),
                components_directory: settings.components_directory.to_string_lossy().to_string(),
                temp_directory: settings.temp_directory.to_string_lossy().to_string(),
                cache_directory: settings.cache_directory.to_string_lossy().to_string(),
            };
            serde_json::to_string(&dto).unwrap_or_else(|_| "{}".to_string())
        }
        Err(_) => {
            // Return default settings
            let mut settings = backend::settings::GlobalSettings::default();
            settings.validate();
            let dto = SettingsDto {
                wineprefixes_directory: settings.wineprefixes_directory.to_string_lossy().to_string(),
                components_directory: settings.components_directory.to_string_lossy().to_string(),
                temp_directory: settings.temp_directory.to_string_lossy().to_string(),
                cache_directory: settings.cache_directory.to_string_lossy().to_string(),
            };
            serde_json::to_string(&dto).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

/// Get all games as JSON array
pub async fn get_all_games_json() -> String {
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };

    let mut games: Vec<GameDto> = Vec::new();

    // Get HoYoPlay games
    match backend::game_providers::hoyoplay::get_games(&settings).await {
        Ok(response) => {
            for game in &response.games {
                games.push(game_to_dto(game));
            }
        }
        Err(e) => {
            eprintln!("[WARN] Failed to load HoYoPlay games: {}", e);
        }
    }

    // Get Endfield games
    match backend::game_providers::endfield::get_games().await {
        Ok(response) => {
            for game in &response.games {
                games.push(game_to_dto(game));
            }
        }
        Err(e) => {
            eprintln!("[WARN] Failed to load Endfield games: {}", e);
        }
    }

    serde_json::to_string(&games).unwrap_or_else(|_| "[]".to_string())
}

/// Get game content as JSON
pub async fn get_game_content_json(game_id: String, biz: String) -> String {
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };

    let result = if biz == "endfield" {
        backend::game_providers::endfield::get_game_content(&game_id).await
    } else {
        backend::game_providers::hoyoplay::get_game_content(&settings, &game_id).await
    };

    match result {
        Ok(response) => {
            let dto = content_to_dto(&response.content);
            serde_json::to_string(&dto).unwrap_or_else(|_| "null".to_string())
        }
        Err(e) => {
            eprintln!("[WARN] Failed to get game content: {}", e);
            "null".to_string()
        }
    }
}

/// Check if a game is installed
#[flutter_rust_bridge::frb(sync)]
pub fn is_game_installed(game_id: String, biz: String) -> bool {
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };

    backend::game_providers::installer::InstallerManager::is_game_installed(
        &settings,
        &game_id,
        &biz,
        settings.temp_directory.clone(),
        settings.components_directory.clone(),
    )
}

/// Get download progress as JSON (returns "null" if no download in progress)
#[flutter_rust_bridge::frb(sync)]
pub fn get_download_progress_json(game_id: String) -> String {
    let key = format!("{}_streaming", game_id);
    match backend::game_providers::endfield::get_progress(&key) {
        Some(p) => {
            let dto = DownloadProgressDto {
                downloaded: p.downloaded,
                total: p.total,
                mb_per_second: p.mb_s,
                part_index: p.part_index,
                parts_total: p.parts_total,
                status: p.status,
                is_busy: p.is_busy,
            };
            serde_json::to_string(&dto).unwrap_or_else(|_| "null".to_string())
        }
        None => "null".to_string(),
    }
}

/// Get data directory path
#[flutter_rust_bridge::frb(sync)]
pub fn get_data_path() -> String {
    backend::globals::DATA_PATH.to_string_lossy().to_string()
}

/// Get config file path
#[flutter_rust_bridge::frb(sync)]
pub fn get_config_path() -> String {
    backend::globals::CONFIG_PATH.to_string_lossy().to_string()
}
