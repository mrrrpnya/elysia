// Flutter Rust Bridge API
// This module exposes the backend APIs to Flutter via flutter_rust_bridge
// Modern approach: Return structs directly, let FRB handle serialization

use serde::{Deserialize, Serialize};

// ============================================================================
// DTO types for FFI - FRB will auto-generate Dart classes for these
// ============================================================================

/// Game data for FFI
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
    pub video_background_url: String,
    pub theme_image_url: String,
    pub background_type: String,
}

/// Game content data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentDto {
    pub game_id: String,
    pub game_biz: String,
    pub language: String,
    pub banners: Vec<BannerDto>,
    pub posts: Vec<PostDto>,
}

/// Banner data for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerDto {
    pub id: String,
    pub image_url: String,
    pub link: String,
}

/// Post data for FFI
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

/// Settings paths
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsDto {
    pub wineprefixes_directory: String,
    pub components_directory: String,
    pub temp_directory: String,
    pub cache_directory: String,
}

/// Available runner information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableRunnerDto {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub is_installed: bool,
    pub install_path: String,
}

/// Available component information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableComponentDto {
    pub name: String,
    pub display_name: String,
    pub is_installed: bool,
}

/// Video frame data for streaming
#[derive(Clone, Debug)]
pub struct VideoFrameDto {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp_ms: i64,
}

use crate::game_providers::hoyoplay::proto::BackgroundInfo;

// ============================================================================
// Conversion functions
// ============================================================================

fn game_to_dto(
    game: &crate::game_providers::hoyoplay::proto::Game,
    background_info: Option<&BackgroundInfo>,
) -> GameDto {
    let (background_url, video_url, theme_url, bg_type) = background_info
        .map(|bg| {
            (
                if bg.background.url.is_empty() {
                    game.display.background.url.clone()
                } else {
                    bg.background.url.clone()
                },
                bg.video.url.clone(),
                bg.theme.url.clone(),
                bg.background_type.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                game.display.background.url.clone(),
                String::new(),
                String::new(),
                String::new(),
            )
        });

    GameDto {
        id: game.id.clone(),
        biz: game.biz.clone(),
        name: game.name.clone(),
        title: game.display.title.clone(),
        subtitle: game.display.subtitle.clone(),
        icon_url: game.display.icon.url.clone(),
        background_url,
        logo_url: game.display.logo.url.clone(),
        display_status: game.display.display_status.clone(),
        video_background_url: video_url,
        theme_image_url: theme_url,
        background_type: bg_type,
    }
}

fn content_to_dto(content: &crate::game_providers::hoyoplay::proto::Content) -> ContentDto {
    ContentDto {
        game_id: content.game_id.clone(),
        game_biz: content.game_biz.clone(),
        language: content.language.clone(),
        banners: content
            .banners
            .iter()
            .map(|b| BannerDto {
                id: b.id.clone(),
                image_url: b.image.url.clone(),
                link: b.link.clone(),
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
// FFI API Functions - Return structs directly!
// ============================================================================

/// Initialize the backend (call on app startup)
pub fn init_backend() -> String {
    "ok".to_string()
}

/// Get settings - returns struct directly!
#[flutter_rust_bridge::frb(sync)]
pub fn get_settings() -> SettingsDto {
    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });
    
    SettingsDto {
        wineprefixes_directory: settings.wineprefixes_directory.to_string_lossy().to_string(),
        components_directory: settings.components_directory.to_string_lossy().to_string(),
        temp_directory: settings.temp_directory.to_string_lossy().to_string(),
        cache_directory: settings.cache_directory.to_string_lossy().to_string(),
    }
}

/// Get all games - returns Vec<GameDto> directly!
pub async fn get_all_games() -> Vec<GameDto> {
    use std::collections::HashMap;

    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });

    let mut games = Vec::new();
    let mut background_map: HashMap<String, BackgroundInfo> = HashMap::new();

    // Fetch background info from getAllGameBasicInfo API
    if let Ok(basic_info) = crate::game_providers::hoyoplay::get_all_game_basic_info(&settings).await {
        for game_info in basic_info.game_info_list {
            background_map.insert(game_info.game_id.clone(), game_info.backgrounds);
        }
    }

    // Fetch HoYoPlay games
    if let Ok(hoyoplay_games) = crate::game_providers::hoyoplay::get_games(&settings).await {
        for game in hoyoplay_games.game_info_list {
            let background_info = background_map.get(&game.game.id);
            games.push(game_to_dto(&game.game, background_info));
        }
    }

    // Fetch Endfield games
    if let Ok(endfield_games) = crate::game_providers::endfield::get_games().await {
        for game in endfield_games.game_list {
            games.push(game_to_dto(&game.game, None));
        }
    }

    games
}

/// Get game content - returns ContentDto or None
pub async fn get_game_content(game_id: String, biz: String) -> Option<ContentDto> {
    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });

    let content = if biz.starts_with("nap") {
        crate::game_providers::endfield::get_game_content(&game_id).await
    } else {
        crate::game_providers::hoyoplay::get_game_content(&settings, &game_id).await
    };

    content.ok().map(|c| content_to_dto(&c))
}

/// Check if a game is installed
#[flutter_rust_bridge::frb(sync)]
pub fn is_game_installed(game_id: String, biz: String) -> bool {
    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });

    let key = format!("{}_{}", biz, game_id);
    crate::game_providers::installer::InstallerManager::is_game_installed(
        &settings.wineprefixes_directory,
        &key,
    )
}

/// Get download progress - returns DownloadProgressDto or None
#[flutter_rust_bridge::frb(sync)]
pub fn get_download_progress(game_id: String) -> Option<DownloadProgressDto> {
    let key = game_id;
    
    if let Some(progress) = crate::game_providers::endfield::get_progress(&key) {
        return Some(DownloadProgressDto {
            downloaded: progress.downloaded,
            total: progress.total,
            mb_per_second: progress.mb_per_second,
            part_index: progress.part_index,
            parts_total: progress.parts_total,
            status: progress.status.clone(),
            is_busy: progress.is_busy,
        });
    }
    None
}

/// Get data directory path
#[flutter_rust_bridge::frb(sync)]
pub fn get_data_path() -> String {
    crate::globals::DATA_PATH.to_string_lossy().to_string()
}

/// Get config file path
#[flutter_rust_bridge::frb(sync)]
pub fn get_config_path() -> String {
    crate::globals::CONFIG_PATH.to_string_lossy().to_string()
}

/// Install/download a game - returns "ok" or error message
pub async fn install_game(game_id: String, biz: String) -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let key = format!("{}_{}", biz, game_id);
    
    let result = if biz.starts_with("nap") {
        crate::game_providers::endfield::install_game(
            &settings.wineprefixes_directory,
            &game_id,
            &key,
        ).await
    } else {
        crate::game_providers::hoyoplay::install_game(
            &settings,
            &game_id,
            &key,
        ).await
    };

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Installation failed: {}", e),
    }
}

/// Launch an installed game - returns "ok" or error message
pub async fn launch_game(game_id: String) -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::game_providers::hoyoplay::launch_game(&settings, &game_id).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Launch failed: {}", e),
    }
}

/// Get available runners - returns Vec<AvailableRunnerDto>
#[flutter_rust_bridge::frb(sync)]
pub fn get_available_runners() -> Vec<AvailableRunnerDto> {
    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });

    crate::runners::list_available_runners(&settings.components_directory)
        .into_iter()
        .map(|r| AvailableRunnerDto {
            name: r.name,
            display_name: r.display_name,
            version: r.version,
            is_installed: r.is_installed,
            install_path: r.install_path.to_string_lossy().to_string(),
        })
        .collect()
}

/// Get available components - returns Vec<AvailableComponentDto>
#[flutter_rust_bridge::frb(sync)]
pub fn get_available_components() -> Vec<AvailableComponentDto> {
    let settings = crate::settings::GlobalSettings::load()
        .unwrap_or_else(|_| {
            let mut s = crate::settings::GlobalSettings::default();
            s.validate();
            s
        });

    let umu_installed = crate::components::runners::is_umu_installed(&settings.components_directory);
    let jadeite_installed = crate::components::jadeite::is_installed(&settings.components_directory);

    vec![
        AvailableComponentDto {
            name: "umu-launcher".to_string(),
            display_name: "UMU Launcher".to_string(),
            is_installed: umu_installed,
        },
        AvailableComponentDto {
            name: "jadeite".to_string(),
            display_name: "Jadeite".to_string(),
            is_installed: jadeite_installed,
        },
    ]
}

/// Install a runner by name - returns "ok" or error message
pub async fn install_runner(runner_name: String) -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::runners::install_runner(
        &runner_name,
        &settings.components_directory,
        &settings.temp_directory,
    ).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Installation failed: {}", e),
    }
}

/// Delete a runner by name - returns "ok" or error message
pub async fn delete_runner(runner_name: String) -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::runners::delete_runner(
        &runner_name,
        &settings.components_directory,
    ).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Deletion failed: {}", e),
    }
}

/// Install UMU launcher component - returns "ok" or error message
pub async fn install_umu_launcher() -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::runners::install_umu_launcher(
        &settings.components_directory,
        &settings.temp_directory,
    ).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Installation failed: {}", e),
    }
}

/// Install Jadeite component - returns "ok" or error message
pub async fn install_jadeite() -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::jadeite::install(
        &settings.components_directory,
        &settings.temp_directory,
    ).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Installation failed: {}", e),
    }
}

/// Delete UMU launcher component - returns "ok" or error message
pub async fn delete_umu_launcher() -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::runners::delete_umu_launcher(&settings.components_directory).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Deletion failed: {}", e),
    }
}

/// Delete Jadeite component - returns "ok" or error message
pub async fn delete_jadeite() -> String {
    let settings = match crate::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(e) => return format!("Failed to load settings: {}", e),
    };

    let result = crate::components::jadeite::delete(&settings.components_directory).await;

    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("Deletion failed: {}", e),
    }
}

/// Stream video frames from a URL
pub async fn stream_video_frames(
    url: String,
    sink: crate::frb_generated::StreamSink<VideoFrameDto>,
) {
    use crate::video_decoder::{VideoDecoder, VideoFrame};
    use tokio::sync::mpsc;
    
    let (frame_tx, mut frame_rx) = mpsc::channel::<VideoFrame>(1);
    
    let url_clone = url.clone();
    let mut decoder_task = tokio::spawn(async move {
        let decoder = VideoDecoder::new(url_clone, frame_tx);
        if let Err(e) = decoder.start().await {
            eprintln!("[ERROR] Video decoder failed: {}", e);
        }
    });
    
    while let Some(frame) = frame_rx.recv().await {
        if sink.add(VideoFrameDto {
            data: frame.data,
            width: frame.width,
            height: frame.height,
            timestamp_ms: frame.timestamp_ms,
        }).is_err() {
            break;
        }
    }
    
    drop(frame_rx);
    
    tokio::select! {
        _ = &mut decoder_task => {}
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
            decoder_task.abort();
        }
    }
}
