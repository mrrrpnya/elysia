use freya::{
    core::custom_attributes::NodeReferenceLayout,
    prelude::{Link, *},
};
use freya_router::prelude::*;
use reqwest::Url;

use crate::Context;
use crate::components::{MyNetworkImage, MySidebarItem};
use crate::game_providers::hoyoplay::api::Game;
use crate::pages::{ErrorPage, Game, Home};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home,
        #[route("/games/:game_id")]
        Game {game_id: String},
    #[end_layout]
    #[route("/..route")]
    ErrorPage {},
}

#[component]
pub fn app() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn FromRouteToCurrent(
    from: Element,
    upwards: bool,
    node_size: ReadOnlySignal<NodeReferenceLayout>,
) -> Element {
    let mut animated_router = use_animated_router::<Route>();
    let animations = use_animation_with_dependencies(&upwards, move |_conf, upwards| {
        let (start, end) = if upwards { (1., 0.) } else { (0., 1.) };
        AnimNum::new(start, end)
            .time(600)
            .ease(Ease::Out)
            .function(Function::Cubic)
    });

    // Run the animation when any prop changes
    use_memo(use_reactive((&upwards, &from), move |_| {
        animations.run(AnimDirection::Forward)
    }));

    // Only render the destination route once the animation has finished
    use_effect(move || {
        if !animations.is_running() && animations.has_run_yet() {
            animated_router.write().settle();
        }
    });

    let offset = animations.get().read().read();
    let height = node_size.read().area.height();

    let offset = height - (offset * height);
    let to = rsx!(Outlet::<Route> {});
    let (top, bottom) = if upwards { (from, to) } else { (to, from) };

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            offset_y: "-{offset}",
            Expand { {top} }
            Expand { {bottom} }
        }
    )
}

#[component]
fn Expand(children: Element) -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            {children}
        }
    )
}

#[component]
fn AnimatedOutlet(children: Element) -> Element {
    let (reference, node_size) = use_node_signal();
    let animated_router = use_context::<Signal<AnimatedRouterContext<Route>>>();

    let from_route = match animated_router() {
        AnimatedRouterContext::FromTo(Route::Home, Route::Game { game_id: id }) => {
            Some((rsx!(Game { game_id: id }), true))
        }
        AnimatedRouterContext::FromTo(
            Route::Game { game_id: id1 },
            Route::Game { game_id: id2 },
        ) => {
            let ctx = &use_context::<Context>();
            let games = &ctx.api_games;
            let order = games
                .iter()
                .position(|game| game.id == id1)
                .unwrap_or_default();
            let order2 = games
                .iter()
                .position(|game| game.id == id2)
                .unwrap_or_default();
            let upwards = order < order2;
            Some((rsx!(Game { game_id: id1 }), upwards))
        }
        AnimatedRouterContext::FromTo(Route::Game { game_id: id }, Route::Home) => {
            Some((rsx!(Game { game_id: id }), false))
        }
        _ => None,
    };

    rsx!(
        rect {
            reference,
            if let Some((from, upwards)) = from_route {
                FromRouteToCurrent { upwards, from, node_size }
            } else {
                Expand {
                    Outlet::<Route> {}
                }
            }
        }
    )
}

fn make_links(games: &[Game]) -> Vec<Element> {
    games
        .iter()
        .map(|game| {
            let route = Route::Game {
                game_id: game.id.clone(),
            };

            rsx!(
                Link {
                    key: game.id.clone(),
                    to: route.clone(),

                    ActivableRoute {
                        route: route,
                        exact: true,
                        MySidebarItem {
                            match game.display.icon.url.parse::<Url>() {
                                Ok(url) => rsx!(
                                        MyNetworkImage {
                                        url: url,
                                        aspect_ratio: "max",
                                        cover: "center",
                                        width: "48",
                                        height: "48",
                                        sampling: "catmull-rom"
                                    }
                                ),
                                Err(_) => rsx!(
                                    rect {
                                        label {
                                            {game.display.name.clone()}
                                        }
                                    }
                                )
                            }
                        }
                    }
                }
            )
        })
        .collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn AppLayout() -> Element {
    let ctx_resource = &use_context::<Resource<Context>>();

    rsx! {
        NativeRouter {
            AnimatedRouter::<Route> {
                rect {
                    width: "100%",
                    height: "100%",
                    direction: "horizontal",

                    rect {
                        position: "absolute",
                        position_top: "0",
                        position_left: "0",
                        height: "100%",
                        width: "84",
                        overflow: "clip",
                        background: "rgb(34,34,34,0.4)",
                        shadow: "2 0 5 0 rgb(0, 0, 0, 30)",
                        layer: "-10",
                        backdrop_blur: "16",
                        ScrollView {
                            padding: "8",
                            spacing: "8",
                            height: "90%",
                            match &*ctx_resource.read_unchecked() {
                                Some(ctx) => {
                                    use_context_provider(|| ctx.clone());
                                    rsx! {
                                        for route in make_links(&ctx.api_games) {
                                            {route}
                                        }
                                    }
                                },
                                _ => rsx! {
                                    label {
                                        "Loading..."
                                    }
                                }
                            }
                        }
                        rect {
                            height: "10%",
                            width: "100%",
                            font_size: "40",
                            main_align: "center",
                            cross_align: "center",
                            Link {
                                key: "settings",
                                to: Route::Home,

                                ActivableRoute {
                                    route: Route::Home,
                                    exact: true,
                                    MySidebarItem {
                                        label {
                                            "⚙️"
                                        }
                                    }
                                }
                            }

                        }

                    }
                    rect {
                        overflow: "clip",
                        width: "fill",
                        height: "100%",
                        Body {
                            AnimatedOutlet { }
                        }
                    }
                }

            }
        }
    }
}
