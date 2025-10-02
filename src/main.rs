#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod components;
mod context;
mod game_providers;
mod globals;
mod layout;
mod pages;
mod runners;
mod settings;
mod utils;

use std::collections::HashMap;
use std::path::PathBuf;

use freya::prelude::*;

use crate::{
    context::Context,
    game_providers::hoyoplay::{get_game_content, get_games},
    runners::{Runner, Wine},
    settings::{GlobalSettings, InstalledGame, RuntimeComponent},
    utils::umu::setup_umu,
};

fn main() {
    launch_cfg(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(1280.0, 720.0)
                .with_decorations(true)
                .with_transparency(true)
                .with_title("Freya App")
                .with_window_attributes(|attributes| attributes.with_resizable(false)),
        ),
    );
}

fn app() -> Element {
    let mut settings = use_signal(|| match GlobalSettings::load() {
        Ok(mut settings) => {
            settings.validate();
            settings
        }
        Err(_) => {
            let mut settings = GlobalSettings::default();
            settings.validate();
            settings.save();
            settings
        }
    });

    settings.write().installed_games.insert(
        "U5hbdsT9W7".to_string(),
        InstalledGame {
            id: "U5hbdsT9W7".to_string(),
            biz_name: "nap_global".to_string(),
            command_arguments: None,
            command_wrapper: None,
            environment: HashMap::new(),
            executable_path: PathBuf::from("ZenlessZoneZero.exe"),
            install_path: PathBuf::from("/path/to/Zenless Zone Zero/"),
            runner: Runner::Wine(Wine {
                version: "Spritz-Wine-TkG-10.15-3".to_string(),
            }),
            runtime_components: vec![RuntimeComponent::Dxvk("2.7.1".to_string())],
        },
    );

    to_owned![settings];
    use_drop(move || {
        settings().save();
    });

    use_context_provider(move || settings);

    use_init_theme(|| DARK_THEME);

    let _umu_handle = use_resource(|| async {
        if let Err(e) = setup_umu().await {
            println!("Failed to set up umu-launcher: {e}");
        }
    });

    let ctx = use_resource(move || async move {
        let api_games = get_games()
            .await
            .map_err(|e| e.to_string())
            .map(|v| v.games)
            .unwrap_or_else(|e| {
                println!("Failed to load games from api: {e}");
                Vec::new()
            });

        let mut api_news = HashMap::new();

        for game in &api_games {
            let id = game.id.to_owned();
            let response = get_game_content(&id).await;

            match response {
                Ok(response) => {
                    api_news.insert(id, response.content);
                }
                Err(e) => {
                    println!("Failed to load game content: {e}");
                }
            }
        }

        Context {
            api_games,
            api_news,
        }
    });
    use_context_provider(move || ctx);
    layout::app()
}
