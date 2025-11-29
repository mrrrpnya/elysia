use std::sync::{Arc, RwLock};
use std::rc::Rc;

use freya::prelude::*;
use reqwest::Url;

use crate::{
    components::{DownloadControl, DownloadProgress, MyButton, MyNetworkImage, MyNewsWidget},
    context::Context,
};
use backend::{
    settings::GlobalSettings,
    game_providers::installer::InstallerManager,
    runners::Runner,
};

#[component]
pub fn Game(game_id: String) -> Element {
    let ctx = &use_context::<Context>();
    let games = &ctx.api_games;
    let settings_sig = use_context::<Signal<Arc<RwLock<GlobalSettings>>>>();

    let Some(game) = games.iter().find(|g| g.id == game_id).cloned() else {
        return rsx! {
            rect {
                label {
                    "Game not found"
                }
            }
        };
    };

    let Ok(url) = game.display.background.url.parse::<Url>() else {
        return rsx! {
            rect {
                label {
                    "Cannot parse background image URL"
                }
            }
        };
    };

    let is_installed = {
        let settings = settings_sig.read();
        if let Ok(s) = settings.read() {
            InstallerManager::is_game_installed(
                &game_id,
                &game.biz,
                s.temp_directory.clone(),
                s.components_directory.clone(),
            )
        } else {
            false
        }
    };

    let progress_key = format!("{}_streaming", game_id);

    let installer = {
        let settings = settings_sig.read();
        if let Ok(s) = settings.read() {
            InstallerManager::create_installer(
                &game.id,
                &game.biz,
                s.temp_directory.clone(),
                s.components_directory.clone(),
            )
        } else {
            None
        }
    };

    let get_progress_fn: Rc<dyn Fn(&str) -> Option<DownloadProgress>> = Rc::new(move |key: &str| {
        installer.as_ref()?.get_progress(key).map(|p| DownloadProgress {
            downloaded: p.downloaded,
            total: p.total,
            mb_s: p.mb_s,
            part_index: p.part_index,
            parts_total: p.parts_total,
            status: p.status,
            is_busy: p.is_busy,
        })
    });

    let onpress = {
        let settings_sig = settings_sig.clone();
        let game_id_clone = game.id.clone();
        let biz = game.biz.clone();

        move |_| {
            let binding = settings_sig.read();
            let settings_result = binding.read();
            if let Ok(settings) = settings_result {
                if let Some(installed_game) = settings.installed_games.get(&game_id_clone) {
                    if let Err(e) = installed_game.runner.run_game(installed_game) {
                        eprintln!("Error running game: {}", e);
                    }
                    return;
                }
                
                let installer = InstallerManager::create_installer(
                    &game_id_clone,
                    &biz,
                    settings.temp_directory.clone(),
                    settings.components_directory.clone(),
                );
                
                if let Some(inst) = installer {
                    InstallerManager::spawn_install(inst, game_id_clone.clone());
                }
            }
        }
    };

    rsx! {
        rect {
            width: "fill",
            height: "fill",

            rect { // Background
                position: "absolute",
                position_top: "0",
                position_left: "0",
                cross_align: "end",
                main_align: "end",
                width: "100%",
                height: "100%",
                layer: "1",

                MyNetworkImage {
                    url: url.clone(),
                    sampling: "trilinear",
                }

                rect { // Background bullshit
                    position: "absolute",
                    position_top: "0",
                    position_left: "0",
                    cross_align: "start",
                    main_align: "start",
                    width: "100%",
                    height: "100%",
                    layer: "1",

                    MyNetworkImage {
                        url: url,
                        sampling: "nearest",
                    }
                },
            },

            rect { // Bottom Left
                position: "absolute",
                position_top: "0",
                position_left: "96",
                width: "100%",
                height: "100%",
                direction: "horizontal",
                main_align: "start",
                cross_align: "end",
                padding: "32",
                rect {
                    width: "500",
                    spacing: "32",

                    MyNewsWidget {
                        game_id: game.id
                    },

                    DownloadControl {
                        game_id: game_id.clone(),
                        progress_key: progress_key,
                        installed: is_installed,
                        get_progress: get_progress_fn,
                        accent_color: "#ff9500".to_string(),
                        onpress: onpress,
                    },
                }
            },
            rect { // Bottom Right
                position: "absolute",
                position_top: "0",
                position_left: "0",
                width: "100%",
                height: "100%",
                direction: "horizontal",
                main_align: "end",
                cross_align: "end",
                spacing: "20",
                padding: "32",

                MyButton {
                    onpress: move |_| println!("Button Pressed!"),
                    rect {
                        font_size: "32",
                        direction: "horizontal",
                        cross_align: "center",
                        spacing: "8",
                        padding: "4",
                        label { "Meow" }
                    }
                }
            },
            rect { // Top Right
                position: "absolute",
                position_top: "0",
                position_left: "0",
                width: "100%",
                height: "100%",
                direction: "horizontal",
                main_align: "end",
                cross_align: "start",
                spacing: "20",
                padding: "32",

                MyButton {
                    onpress: move |_| println!("Button Pressed!"),
                    rect {
                        font_size: "32",
                        direction: "horizontal",
                        cross_align: "center",
                        spacing: "8",
                        padding: "4",
                        label { "Explode" }
                    }
                }
            }
        }
    }
}