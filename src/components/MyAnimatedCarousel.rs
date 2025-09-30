use freya::{core::custom_attributes::NodeReferenceLayout, prelude::*};

use crate::components::Expand;

#[component]
pub fn MyAnimatedCarousel(items: Vec<Element>, width: usize) -> Element {
    let (reference, node_size) = use_node_signal();
    let mut state = use_signal(|| CarouselState::Stopped(0));

    let len = items.len();
    let onwheel = move |e: Event<WheelData>| {
        let direction = e.get_delta_y().signum();

        let current: CarouselState = *state.read();

        match current {
            CarouselState::Stopped(index) => {
                if direction > 0.0 && index < len - 1 {
                    *state.write() = CarouselState::Running(index, index + 1);
                } else if direction < 0.0 && index > 0 {
                    *state.write() = CarouselState::Running(index, index - 1);
                }
            }
            CarouselState::Running(_, _) => {}
        };
    };

    rsx!(
        rect {
            onwheel,
            reference,
            Carousel { width, items, state, node_size }
        }
    )
}

#[component]
fn Carousel(
    width: usize,
    items: Vec<Element>,
    state: Signal<CarouselState>,
    node_size: ReadOnlySignal<NodeReferenceLayout>,
) -> Element {
    let animation = use_animation(move |_conf| {
        AnimNum::new(1.0, 0.0)
            .time(300)
            .ease(Ease::Out)
            .function(Function::Cubic)
    });

    use_effect(move || {
        let current: CarouselState = *state.read();

        match current {
            CarouselState::Running(_, to) => {
                if !animation.is_running() {
                    if animation.has_run_yet() {
                        *state.write() = CarouselState::Stopped(to);
                    } else {
                        animation.run(AnimDirection::Forward);
                    }
                }
            }
            CarouselState::Stopped(_) => {
                animation.reset();
            }
        };
    });

    let offset = animation.get().read().read();
    let width = width as f32;

    rsx!(
        rect {
            overflow: "clip",
            width: "{width}",
            corner_radius: "16",
            direction: "horizontal",

            match *state.read() {
                CarouselState::Stopped(index) => {
                    let index = if index >= items.len() {0} else {index};
                    rsx! {
                        rect {
                            width: "{width}",
                            {&items[index]}
                        }
                    }
                }
                CarouselState::Running(from, to) => {
                    let direction = to as f32 - from as f32;
                    let offset_x = (offset * width) * direction.signum() - width;
                    let from = if from >= items.len() {0} else {from};
                    let to = if to >= items.len() {0} else {to};

                    rsx! {
                        rect {
                            width: "{width}",
                            {&items[from]}
                        }

                        rect {
                            width: "{width}",
                            offset_x: "{offset_x}",

                            {&items[to]}
                        }
                    }
                }
            }
        }
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CarouselState {
    Stopped(usize),
    Running(usize, usize),
}
