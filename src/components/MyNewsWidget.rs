use freya::prelude::*;
use reqwest::Url;

use crate::components::{MyAnimatedCarousel, MyNetworkImage};
use crate::context::Context;
use crate::game_providers::hoyoplay::get_game_content;

#[component]
pub fn MyNewsWidget(game_id: String) -> Element {
    let ctx = &use_context::<Context>();
    let Some(content) = ctx.api_news.get(&game_id) else {
        return rsx! {};
    };

    let mut selected = use_signal(|| 0);

    let len = content.banners.len();
    let onwheel = move |e: Event<WheelData>| {
        let current = selected();

        if e.get_delta_y() > 0.0 && current < len - 1 {
            *selected.write() += 1
        } else if e.get_delta_y() < 0.0 && current > 0 {
            *selected.write() -= 1
        };
    };

    rsx! {
        rect {
            direction: "vertical",
            main_align: "start",

            rect { // Image Carousel
                onwheel,
                direction: "horizontal",
                spacing: "0",
                padding: "0",

                MyAnimatedCarousel {
                    items: content.banners.iter().map(|url| {
                        rsx!{
                            MyNetworkImage {
                                url: url.image.url.parse::<Url>().unwrap(),
                                aspect_ratio: "min",
                                //cover: "fill",
                                sampling: "catmull-rom"
                            }
                        }
                    }).collect(),
                }
            },

            rect { // Event List

            }
        }
    }
}
