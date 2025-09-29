use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn MySidebarItem(
    /// Inner content for the SidebarItem.
    children: Element,
) -> Element {
    let font_theme = use_applied_theme!(None, sidebar_item).font_theme;
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();
    let is_active = use_activable_route();

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let background = "rgb(34,34,34,.1)";
    let hover_background = "rgb(34,34,34,.4)";

    let background = match *status.read() {
        _ if is_active => hover_background,
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };

    rsx! {
        rect {
            overflow: "clip",
            margin: "0",
            onmouseenter,
            onmouseleave,
            width: "auto",
            height: "auto",
            color: "{font_theme.color}",
            corner_radius: "8",
            padding: "8",
            background: "{background}",
            {children}
        }
    }
}
