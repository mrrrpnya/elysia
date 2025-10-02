#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::runners::{Runner, Wine};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

use freya::prelude::*;

mod components;
mod context;
mod game_providers;
mod layout;
mod pages;
mod runners;
mod settings;

use crate::context::Context;
use crate::game_providers::hoyoplay::{get_game_content, get_games};
use crate::settings::{GlobalSettings, InstalledGame, RuntimeComponent};

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
    let data_path = get_data_path_or_panic();
    let mut settings = use_signal(|| match GlobalSettings::load(&data_path) {
        Ok(settings) => settings,
        Err(_) => {
            let settings = GlobalSettings::default(&data_path);
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

    let ctx = use_resource(move || {
        to_owned![data_path];
        async move {
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
                data_path,
            }
        }
    });
    use_context_provider(move || ctx);
    layout::app()
}

fn get_data_path_or_panic() -> PathBuf {
    let data_dir = env::var("XDG_DATA_HOME")
        .map(|v| PathBuf::from(v).join("elysia"))
        .or_else(|_| {
            let var = env::var("HOME");
            match var {
                Ok(home) => Ok(PathBuf::from(home).join(".local/share/elysia")),
                Err(e) => Err(e),
            }
        })
        .or_else(|e| {
            let cwd = env::current_dir();
            match cwd {
                Ok(path) => Ok(path.join("elysia")),
                Err(_) => Err(e),
            }
        });

    match data_dir {
        Ok(data_dir) => match fs::exists(&data_dir) {
            Ok(exists) => {
                if !exists && let Err(e) = fs::create_dir(&data_dir) {
                    panic!("Cannot create data directory {data_dir:?}: {e}");
                }

                data_dir
            }
            Err(e) => panic!("Cannot check if data directory {data_dir:?} exists: {e}"),
        },
        Err(e) => {
            panic!("Cannot pick a data directory, last error: {e}")
        }
    }
}
