#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use freya::prelude::*;

mod components;
mod context;
mod game_providers;
mod layout;
mod pages;

use crate::context::Context;
use crate::game_providers::hoyoplay::{get_game_content, get_games};

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
    use_init_theme(|| DARK_THEME);

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
