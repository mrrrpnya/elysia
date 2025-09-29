use freya::prelude::*;
use reqwest::Url;

use crate::components::MyNetworkImage;
use crate::context::Context;
use crate::game_providers::get_game_content;

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
                content: "flex",
                direction: "horizontal",
                spacing: "0",
                width: "690",
                height: "320",
                padding: "0",

                for (i, url) in content.banners.iter().enumerate() {
                    ImageCard {
                        key: "{i}",
                        selected: i == selected(),
                        MyNetworkImage {
                            url: url.image.url.parse::<Url>().unwrap(),
                            aspect_ratio: "max",
                            cover: "center",
                        }
                    }
                }
            },

            rect { // Event List

            }
        }
    }
}

#[component]
fn ImageCard(selected: ReadOnlySignal<bool>, children: Element) -> Element {
    let animations = use_animation(move |conf| {
        conf.on_deps_change(OnDepsChange::Rerun);
        conf.on_creation(OnCreation::Run);
        let (from, to) = if selected() { (0.0, 1.0) } else { (1.0, 0.0) };
        AnimNum::new(from, to)
            .time(250)
            .ease(Ease::Out)
            .function(Function::Expo)
    });

    let width = animations.get().read().read();

    rsx!(
        rect {
            corner_radius: "16",
            height: "100%",
            width: "flex({width})",
            overflow: "clip",
            {children}
        }
    )
}
