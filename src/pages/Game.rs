use freya::prelude::*;
use reqwest::Url;

use crate::{
    components::{MyButton, MyNetworkImage, MyNewsWidget},
    context::Context,
    settings::GlobalSettings,
};

#[component]
pub fn Game(game_id: String) -> Element {
    let ctx = &use_context::<Context>();
    let games = &ctx.api_games;

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

    let onpress = move |_| {
        println!("Button Pressed!");
        let ctx = &dioxus::hooks::use_context::<Signal<GlobalSettings>>();
        let settings = &ctx.read();
        let installed_games = &settings.installed_games;
        if installed_games.contains_key(&game_id) {
            let game = &installed_games[&game_id];

            game.runner.run_game(game);
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
                width: "100%",
                height: "100%",
                layer: "1",

                MyNetworkImage {
                    url: url,
                    sampling: "catmull-rom",
                }
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

                    MyButton {
                        onpress,

                        rect {
                            font_size: "24",
                            width: "100%",
                            direction: "horizontal",
                            cross_align: "center",
                            main_align: "start",
                            padding: "4",
                            label { "Start Game" }
                        }
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
