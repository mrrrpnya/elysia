use crate::{
    components::{MyButton, MyNetworkImage, MyNewsWidget},
    context::Context,
};
use freya::prelude::*;
use reqwest::Url;

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
                spacing: "20",
                padding: "32",
                rect {
                    width: "410",

                    MyNewsWidget {
                        game_id: game.id
                    },

                    MyButton {
                        onpress: move |_| println!("Button Pressed!"),

                        rect {
                            font_size: "24",
                            width: "100%",
                            direction: "horizontal",
                            cross_align: "center",
                            main_align: "start",
                            padding: "8",
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
                        label { "Explode" }
                    }
                }
            }
        }
    }
}
