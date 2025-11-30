// Flutter Rust Bridge API
// This module exposes the backend APIs to Flutter via flutter_rust_bridge

use std::collections::HashMap;

// Re-export types for Flutter
pub use backend::game_providers::hoyoplay::proto::{
    Banner, Content, Display, Game, GameInfo, GetGameContent, GetGames, Image, ImageLink, Post,
    SocialMedia,
};
pub use backend::game_providers::Progress;
pub use backend::runners::{Proton, Runners, Wine};
pub use backend::settings::{GlobalSettings, InstalledGame, RuntimeComponents};

// ============================================================================
// Settings API
// ============================================================================

/// Load global settings from disk
#[flutter_rust_bridge::frb(sync)]
pub fn load_settings() -> Result<GlobalSettings, String> {
    GlobalSettings::load()
}

/// Save global settings to disk
#[flutter_rust_bridge::frb(sync)]
pub fn save_settings(settings: &GlobalSettings) -> Result<(), String> {
    settings.save()
}

/// Create default settings
#[flutter_rust_bridge::frb(sync)]
pub fn create_default_settings() -> GlobalSettings {
    let mut settings = GlobalSettings::default();
    settings.validate();
    settings
}

// ============================================================================
// Game Provider API - HoYoPlay
// ============================================================================

/// Get list of games from HoYoPlay API
pub async fn get_hoyoplay_games(settings: &GlobalSettings) -> Result<Vec<Game>, String> {
    backend::game_providers::hoyoplay::get_games(settings)
        .await
        .map(|r| r.games)
}

/// Get game content (banners, news, etc.) from HoYoPlay API
pub async fn get_hoyoplay_game_content(
    settings: &GlobalSettings,
    game_id: String,
) -> Result<Content, String> {
    backend::game_providers::hoyoplay::get_game_content(settings, &game_id)
        .await
        .map(|r| r.content)
}

// ============================================================================
// Game Provider API - Endfield
// ============================================================================

/// Get list of Endfield games
pub async fn get_endfield_games() -> Result<Vec<Game>, String> {
    backend::game_providers::endfield::get_games()
        .await
        .map(|r| r.games)
}

/// Get Endfield game content
pub async fn get_endfield_game_content(game_id: String) -> Result<Content, String> {
    backend::game_providers::endfield::get_game_content(&game_id)
        .await
        .map(|r| r.content)
}

/// Get Endfield background image URL
pub async fn get_endfield_background_image(app_code: String) -> Result<String, String> {
    backend::game_providers::endfield::get_main_bg_image(&app_code).await
}

// ============================================================================
// Combined Game API
// ============================================================================

/// Get all games (HoYoPlay + Endfield combined)
/// Note: Errors from individual providers are logged but don't fail the entire operation
pub async fn get_all_games(settings: &GlobalSettings) -> Result<Vec<Game>, String> {
    let mut games = match get_hoyoplay_games(settings).await {
        Ok(g) => g,
        Err(e) => {
            eprintln!("[WARN] Failed to load HoYoPlay games: {}", e);
            Vec::new()
        }
    };
    
    match get_endfield_games().await {
        Ok(endfield_games) => games.extend(endfield_games),
        Err(e) => eprintln!("[WARN] Failed to load Endfield games: {}", e),
    }
    
    Ok(games)
}

/// Get game content for any game (routes to correct provider)
pub async fn get_game_content(
    settings: &GlobalSettings,
    game_id: String,
    biz: String,
) -> Result<Content, String> {
    if biz == "endfield" {
        get_endfield_game_content(game_id).await
    } else {
        get_hoyoplay_game_content(settings, game_id).await
    }
}

// ============================================================================
// Installation API
// ============================================================================

/// Check if a game is installed
#[flutter_rust_bridge::frb(sync)]
pub fn is_game_installed(
    settings: &GlobalSettings,
    game_id: String,
    biz: String,
) -> bool {
    backend::game_providers::installer::InstallerManager::is_game_installed(
        settings,
        &game_id,
        &biz,
        settings.temp_directory.clone(),
        settings.components_directory.clone(),
    )
}

/// Get download progress for a game
#[flutter_rust_bridge::frb(sync)]
pub fn get_download_progress(game_id: String) -> Option<DownloadProgress> {
    let key = format!("{}_streaming", game_id);
    backend::game_providers::endfield::get_progress(&key).map(|p| DownloadProgress {
        downloaded: p.downloaded,
        total: p.total,
        mb_per_second: p.mb_s,
        part_index: p.part_index,
        parts_total: p.parts_total,
        status: p.status,
        is_busy: p.is_busy,
    })
}

/// Progress information for downloads
#[derive(Clone, Debug)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub mb_per_second: f32,
    pub part_index: usize,
    pub parts_total: usize,
    pub status: String,
    pub is_busy: bool,
}

// ============================================================================
// Runner API
// ============================================================================

/// Run an installed game
#[flutter_rust_bridge::frb(sync)]
pub fn run_game(settings: &GlobalSettings, game_id: String) -> Result<(), String> {
    use backend::runners::Runner;

    let game = settings
        .installed_games
        .get(&game_id)
        .ok_or_else(|| format!("Game {} not found in installed games", game_id))?;

    game.runner.run_game(settings, game)
}

// ============================================================================
// Utility Functions
// ============================================================================

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

/// Initialize the runtime (call this on app startup)
pub fn init() {
    // Initialize tokio runtime for async operations if needed
    // This is handled by flutter_rust_bridge automatically
}

// ============================================================================
// Context for UI state
// ============================================================================

/// Application context holding games and news data
#[derive(Clone, Debug, Default)]
pub struct AppContext {
    pub games: Vec<Game>,
    pub news: HashMap<String, Content>,
}

/// Load initial app context (games and news)
pub async fn load_app_context(settings: &GlobalSettings) -> AppContext {
    let games = match get_all_games(settings).await {
        Ok(g) => g,
        Err(e) => {
            eprintln!("[WARN] Failed to load games for app context: {}", e);
            Vec::new()
        }
    };
    let mut news = HashMap::new();

    for game in &games {
        match get_game_content(settings, game.id.clone(), game.biz.clone()).await {
            Ok(content) => {
                news.insert(game.id.clone(), content);
            }
            Err(e) => {
                eprintln!("[WARN] Failed to load content for game {}: {}", game.id, e);
            }
        }
    }

    AppContext { games, news }
}
