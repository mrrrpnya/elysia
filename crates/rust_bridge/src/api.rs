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
    // Video background fields from getAllGameBasicInfo API
    pub video_background_url: String,
    pub theme_image_url: String,
    pub background_type: String,
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

use backend::game_providers::hoyoplay::proto::BackgroundInfo;

// ============================================================================
// Conversion functions
// ============================================================================

fn game_to_dto(
    game: &backend::game_providers::hoyoplay::proto::Game,
    background_info: Option<&BackgroundInfo>,
) -> GameDto {
    // Use background info from getAllGameBasicInfo API if available
    // This provides more up-to-date backgrounds including video backgrounds
    let (background_url, video_url, theme_url, bg_type) = background_info
        .map(|bg| {
            (
                // Use background from getAllGameBasicInfo if available, otherwise fallback to getGames
                if bg.background.url.is_empty() {
                    game.display.background.url.clone()
                } else {
                    bg.background.url.clone()
                },
                bg.video.url.clone(),
                bg.theme.url.clone(),
                bg.bg_type.clone(),
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
        name: game.display.name.clone(),
        title: game.display.title.clone(),
        subtitle: game.display.subtitle.clone(),
        icon_url: game.display.icon.url.clone(),
        background_url,
        logo_url: game.display.logo.url.clone(),
        display_status: game.display_status.clone(),
        video_background_url: video_url,
        theme_image_url: theme_url,
        background_type: bg_type,
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
    use std::collections::HashMap;

    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };

    let mut games: Vec<GameDto> = Vec::new();

    // Fetch background info from getAllGameBasicInfo API
    let background_map: HashMap<String, BackgroundInfo> =
        match backend::game_providers::hoyoplay::get_all_game_basic_info(&settings).await {
            Ok(response) => response
                .game_info_list
                .into_iter()
                .filter_map(|info| {
                    // Get the first background (usually the most relevant one)
                    info.backgrounds.into_iter().next().map(|bg| (info.game.id, bg))
                })
                .collect(),
            Err(e) => {
                eprintln!("[WARN] Failed to load game basic info: {}", e);
                HashMap::new()
            }
        };

    // Get HoYoPlay games
    match backend::game_providers::hoyoplay::get_games(&settings).await {
        Ok(response) => {
            for game in &response.games {
                let bg_info = background_map.get(&game.id);
                games.push(game_to_dto(game, bg_info));
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
                let bg_info = background_map.get(&game.id);
                games.push(game_to_dto(game, bg_info));
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

/// Install/download a game
/// Returns "ok" if installation started, or an error message
pub async fn install_game(game_id: String, biz: String) -> String {
    use std::sync::{Arc, RwLock};
    use backend::game_providers::installer::InstallerManager;
    
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };
    
    let temp_dir = settings.temp_directory.clone();
    let components_dir = settings.components_directory.clone();
    
    // Create installer for this game
    match InstallerManager::create_installer(&game_id, &biz, temp_dir, components_dir) {
        Some(installer) => {
            // Wrap settings in Arc<RwLock> for the async task
            let settings_arc = Arc::new(RwLock::new(settings));
            
            // Spawn the installation task
            InstallerManager::spawn_install(settings_arc, installer, game_id.clone());
            
            eprintln!("[INFO] Installation started for game: {}", game_id);
            "ok".to_string()
        }
        None => {
            let err = format!("No installer available for game {} with biz {}", game_id, biz);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

/// Launch an installed game
/// Returns "ok" if launch started, or an error message
pub async fn launch_game(game_id: String) -> String {
    use backend::runners::Runner;
    
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };
    
    // Get the installed game info
    match settings.installed_games.get(&game_id) {
        Some(installed_game) => {
            match installed_game.runner.run_game(&settings, installed_game) {
                Ok(_) => {
                    eprintln!("[INFO] Game launched: {}", game_id);
                    "ok".to_string()
                }
                Err(e) => {
                    let err = format!("Failed to launch game: {}", e);
                    eprintln!("[ERROR] {}", err);
                    err
                }
            }
        }
        None => {
            let err = format!("Game {} is not installed", game_id);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

// ============================================================================
// Component Management APIs
// ============================================================================

/// Available runner DTO for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableRunnerDto {
    pub name: String,
    pub display_name: String,
    pub runner_type: String,
    pub version: String,
    pub download_url: String,
    pub folder_name: String,
    pub is_installed: bool,
}

/// Available component DTO for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableComponentDto {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub component_type: String,
    pub version: String,
    pub is_installed: bool,
}

/// Get available runners as JSON
#[flutter_rust_bridge::frb(sync)]
pub fn get_available_runners_json() -> String {
    use backend::components::runners::{get_available_runners, is_runner_installed, RunnerType};
    
    let runners = get_available_runners();
    let dtos: Vec<AvailableRunnerDto> = runners
        .iter()
        .map(|r| AvailableRunnerDto {
            name: r.name.clone(),
            display_name: r.display_name.clone(),
            runner_type: match r.runner_type {
                RunnerType::Wine => "wine".to_string(),
                RunnerType::Proton => "proton".to_string(),
            },
            version: r.version.clone(),
            download_url: r.download_url.clone(),
            folder_name: r.folder_name.clone(),
            is_installed: is_runner_installed(r),
        })
        .collect();
    
    serde_json::to_string(&dtos).unwrap_or_else(|_| "[]".to_string())
}

/// Get available components (umu-launcher, jadeite) as JSON
#[flutter_rust_bridge::frb(sync)]
pub fn get_available_components_json() -> String {
    let components_dir = backend::globals::DATA_PATH.join("components");
    
    let mut components = Vec::new();
    
    // UMU Launcher
    let umu_installed = components_dir.join("umu/umu-run").exists();
    components.push(AvailableComponentDto {
        name: "umu-launcher".to_string(),
        display_name: "UMU Launcher".to_string(),
        description: "Required for running games with Proton".to_string(),
        component_type: "umu".to_string(),
        version: "1.2.9".to_string(),
        is_installed: umu_installed,
    });
    
    // Jadeite
    let jadeite_installed = components_dir.join("tweaks/jadeite/jadeite.exe").exists();
    components.push(AvailableComponentDto {
        name: "jadeite".to_string(),
        display_name: "Jadeite".to_string(),
        description: "Required for certain games (anti-cheat compatibility)".to_string(),
        component_type: "jadeite".to_string(),
        version: "v5.0.1".to_string(),
        is_installed: jadeite_installed,
    });
    
    serde_json::to_string(&components).unwrap_or_else(|_| "[]".to_string())
}

/// Install a runner by name
/// Returns "ok" if installation succeeded, or an error message
pub async fn install_runner(runner_name: String) -> String {
    use backend::components::runners::{get_available_runners, install_runner as backend_install_runner};
    
    let runners = get_available_runners();
    let runner = runners.iter().find(|r| r.name == runner_name);
    
    match runner {
        Some(r) => {
            match backend_install_runner(r).await {
                Ok(_) => {
                    eprintln!("[INFO] Runner installed: {}", runner_name);
                    "ok".to_string()
                }
                Err(e) => {
                    let err = format!("Failed to install runner: {}", e);
                    eprintln!("[ERROR] {}", err);
                    err
                }
            }
        }
        None => {
            let err = format!("Runner not found: {}", runner_name);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

/// Delete a runner by name
/// Returns "ok" if deletion succeeded, or an error message
pub async fn delete_runner(runner_name: String) -> String {
    use backend::components::runners::{get_available_runners, get_components_directory};
    
    let runners = get_available_runners();
    let runner = runners.iter().find(|r| r.name == runner_name);
    
    match runner {
        Some(r) => {
            let components_dir = get_components_directory();
            let runner_dir = components_dir
                .join(r.runner_type.directory_name())
                .join(&r.folder_name);
            
            if runner_dir.exists() {
                match std::fs::remove_dir_all(&runner_dir) {
                    Ok(_) => {
                        eprintln!("[INFO] Runner deleted: {}", runner_name);
                        "ok".to_string()
                    }
                    Err(e) => {
                        let err = format!("Failed to delete runner: {}", e);
                        eprintln!("[ERROR] {}", err);
                        err
                    }
                }
            } else {
                "ok".to_string()
            }
        }
        None => {
            let err = format!("Runner not found: {}", runner_name);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

/// Install UMU launcher component
/// Returns "ok" if installation succeeded, or an error message
pub async fn install_umu_launcher() -> String {
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };
    
    match backend::components::umu::setup_umu(&settings).await {
        Ok(_) => {
            eprintln!("[INFO] UMU Launcher installed");
            "ok".to_string()
        }
        Err(e) => {
            let err = format!("Failed to install UMU Launcher: {}", e);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

/// Install Jadeite component
/// Returns "ok" if installation succeeded, or an error message
pub async fn install_jadeite() -> String {
    let settings = match backend::settings::GlobalSettings::load() {
        Ok(s) => s,
        Err(_) => {
            let mut s = backend::settings::GlobalSettings::default();
            s.validate();
            s
        }
    };
    
    let tweaks_dir = settings.components_directory.join("tweaks");
    
    match backend::components::tweaks::downloader::jade_download(tweaks_dir).await {
        Ok(_) => {
            eprintln!("[INFO] Jadeite installed");
            "ok".to_string()
        }
        Err(e) => {
            let err = format!("Failed to install Jadeite: {}", e);
            eprintln!("[ERROR] {}", err);
            err
        }
    }
}

/// Delete UMU launcher component
/// Returns "ok" if deletion succeeded, or an error message
pub async fn delete_umu_launcher() -> String {
    let components_dir = backend::globals::DATA_PATH.join("components");
    let umu_dir = components_dir.join("umu");
    
    if umu_dir.exists() {
        match std::fs::remove_dir_all(&umu_dir) {
            Ok(_) => {
                eprintln!("[INFO] UMU Launcher deleted");
                "ok".to_string()
            }
            Err(e) => {
                let err = format!("Failed to delete UMU Launcher: {}", e);
                eprintln!("[ERROR] {}", err);
                err
            }
        }
    } else {
        "ok".to_string()
    }
}

/// Delete Jadeite component
/// Returns "ok" if deletion succeeded, or an error message
pub async fn delete_jadeite() -> String {
    let components_dir = backend::globals::DATA_PATH.join("components");
    let jadeite_dir = components_dir.join("tweaks/jadeite");
    
    if jadeite_dir.exists() {
        match std::fs::remove_dir_all(&jadeite_dir) {
            Ok(_) => {
                eprintln!("[INFO] Jadeite deleted");
                "ok".to_string()
            }
            Err(e) => {
                let err = format!("Failed to delete Jadeite: {}", e);
                eprintln!("[ERROR] {}", err);
                err
            }
        }
    } else {
        "ok".to_string()
    }
}

// ============================================================================
// Video Frame Streaming API
// ============================================================================

/// Video frame DTO for FFI
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoFrameDto {
    /// RGBA image data
    pub data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Frame timestamp in milliseconds
    pub timestamp_ms: i64,
}

/// Start streaming video frames from a URL
/// Streams video frames via the provided sink
pub async fn stream_video_frames(
    url: String,
    sink: crate::frb_generated::StreamSink<VideoFrameDto>,
) {
    use backend::video_decoder::{VideoDecoder, VideoFrame};
    use tokio::sync::mpsc;
    
    let (frame_tx, mut frame_rx) = mpsc::channel::<VideoFrame>(1); // Buffer 1 frame
    
    // Start video decoder in background with abort handle
    let url_clone = url.clone();
    let decoder_task = tokio::spawn(async move {
        let decoder = VideoDecoder::new(url_clone, frame_tx);
        if let Err(e) = decoder.start().await {
            eprintln!("[ERROR] Video decoder failed: {}", e);
        }
    });
    
    // Forward frames to Flutter via sink
    // This runs in the current task, so when Flutter cancels the stream,
    // this function is dropped/cancelled and the loop stops
    while let Some(frame) = frame_rx.recv().await {
        // Check if add() succeeds - if it fails, Flutter closed the stream
        if sink.add(VideoFrameDto {
            data: frame.data,
            width: frame.width,
            height: frame.height,
            timestamp_ms: frame.timestamp_ms,
        }).is_err() {
            // Flutter closed the stream, stop immediately
            println!("Flutter closed stream, stopping forwarding");
            break;
        }
    }
    
    // Close the channel so decoder can detect it via is_closed()
    drop(frame_rx);
    println!("Frame channel closed, waiting for decoder to stop");
    
    // Wait for decoder to detect closed channel and stop naturally (max 100ms)
    // This gives it time to check is_closed() and exit cleanly
    // Use select! to race without moving decoder_task
    tokio::select! {
        _ = &mut decoder_task => {
            println!("Video decoder stopped gracefully");
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
            // If decoder doesn't stop within 100ms, abort it
            println!("Decoder timeout, aborting task");
            decoder_task.abort();
        }
    }
    
    println!("Video stream ended, all cleanup done");
}
