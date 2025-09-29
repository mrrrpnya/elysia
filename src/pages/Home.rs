use std::hash::{DefaultHasher, Hash, Hasher};

use freya::prelude::*;
use freya_elements::attributes::font_style;
use reqwest::Url;

use crate::components::MyNetworkImage;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
    let url = "https://cdn.discordapp.com/emojis/1136945196752453642.webp?size=32"
        .parse::<Url>()
        .unwrap();

    let xdds = &[
        "https://cdn.discordapp.com/emojis/1392622950464749619.webp?size=64",
        "https://cdn.discordapp.com/emojis/1136945196752453642.webp?size=64",
        "https://cdn.discordapp.com/emojis/1392624394592850192.webp?size=64",
        "https://cdn.discordapp.com/emojis/1392620921994612797.webp?size=64",
    ];

    let imgs = xdds
        .iter()
        .map(|url| url.parse::<Url>().unwrap())
        .collect::<Vec<_>>();

    rsx! {
        rect {
            width: "100%",
            height: "100%",
            cross_align: "center",
            main_align: "center",
            direction: "horizontal",
            spacing: "12",

            rect { // Background
                position: "absolute",
                position_top: "0",
                position_left: "0",
                main_align: "center",
                cross_align: "center",
                width: "100%",
                height: "100%",
                layer: "5",
                direction: "vertical",

                for x in 0..32 {
                    rect {
                        direction:"horizontal",
                        for y in 0..32 {
                            MyNetworkImage {
                                url: imgs[pseudo_random_from_two(x, y)].clone(),
                                sampling: "catmull-rom",
                                cover: "center",
                                width: "64",
                                height: "64"
                            }
                        }
                    }
                }
            },

            label {
                font_family: "Noto Sans",
                font_size: "20",
                "probably last opened game"
            },
            MyNetworkImage {
                url: url,
                sampling: "catmull-rom",
                width: "32",
                height: "32",
            }
        }
    }
}

fn pseudo_random_from_two(a: usize, b: usize) -> usize {
    let a = a & 0b1_1111;
    let b = b & 0b1_1111;

    // Combine into a single 10‑bit value
    let seed: u16 = ((a as u16) << 5) | (b as u16);

    // Hash the seed
    let mut hasher = DefaultHasher::new(); // FNV‑1a on most platforms
    seed.hash(&mut hasher);
    let hash = hasher.finish();

    // Low two bits give a value 0‑3
    (hash & 0b11) as usize
}
