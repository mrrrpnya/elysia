use freya::{core::custom_attributes::NodeReferenceLayout, prelude::*};

use crate::components::Expand;

#[component]
pub fn MyAnimatedCarousel(items: Vec<Element>) -> Element {
    let (reference, node_size) = use_node_signal();
    let mut selected = use_signal(|| 0);

    let len = items.len();
    let onwheel = move |e: Event<WheelData>| {
        let direction = e.get_delta_y().signum();

        let current = selected();

        if direction > 0.0 && current < len - 1 {
            *selected.write() += 1
        } else if direction < 0.0 && current > 0 {
            *selected.write() -= 1
        };
    };

    rsx!(
        rect {
            onwheel,
            reference,
            Carousel { items, selected, node_size }
        }
    )
}

#[component]
fn Carousel(
    items: Vec<Element>,
    selected: Signal<usize>,
    node_size: ReadOnlySignal<NodeReferenceLayout>,
) -> Element {
    let previous = use_signal(|| selected.read().to_owned());
    let animation = use_animation_with_dependencies(&selected, move |_conf, _selected| {
        let (start, end) = if selected() % 2 == 0 {
            (1., 0.)
        } else {
            (0., 1.)
        };

        AnimNum::new(start, end)
            .time(600)
            .ease(Ease::Out)
            .function(Function::Cubic)
    });

    // Only render the destination route once the animation has finished
    use_effect(move || {
        if !animation.is_running() && !animation.has_run_yet() {
            animation.run(AnimDirection::Forward);
            println!("Selected: {}, Previous: {}", selected(), previous());
        }
    });

    let offset = animation.get().read().read();
    let width = node_size.read().area.width();

    let offset_x = width - (offset * width);

    rsx!(
        rect {
            height: "fill",
            width: "fill",

            direction: "horizontal",
            offset_x: "-{offset_x}",
            Expand { {items[previous()].clone()} },
            Expand { {items[selected()].clone()} }
        }
    )
}
