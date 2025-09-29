use freya::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MyButtonProps {
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,
}

#[component]
pub fn MyButton(props: MyButtonProps) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let a11y_id = focus.attribute();

    let MyButtonProps {
        children,
        onpress,
        enabled,
    } = props;

    let ButtonTheme {
        background,
        hover_background,
        disabled_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius: _,
        width,
        height,
        font_theme,
        shadow: _,
    } = use_applied_theme!(&None, filled_button);

    let onpointerpress = {
        to_owned![onpress];
        move |ev: PointerEvent| {
            if !enabled {
                return;
            }
            focus.request_focus();
            if let Some(onpress) = &onpress {
                let is_valid = match ev.data.pointer_type {
                    PointerType::Mouse {
                        trigger_button: Some(MouseButton::Left),
                    } => true,
                    PointerType::Touch { phase, .. } => phase == TouchPhase::Ended,
                    _ => false,
                };
                if is_valid {
                    onpress.call(PressEvent::Pointer(ev))
                }
            }
        }
    };

    use_effect(use_reactive!(|enabled| {
        if *status.peek() == ButtonStatus::Hovering && !enabled {
            platform.set_cursor(CursorIcon::default());
        }
    }));

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering && enabled {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onpointerenter = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::Pointer);
            status.set(ButtonStatus::Hovering);
        }
    };

    let onpointerleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if focus.validate_keydown(&ev)
            && enabled
            && let Some(onpress) = &onpress
        {
            onpress.call(PressEvent::Key(ev))
        }
    };

    let a11y_focusable = if enabled { "true" } else { "false" };
    let background = match *status.read() {
        _ if !enabled => disabled_background,
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };
    let border = if focus.is_focused_with_keyboard() {
        format!("2 inner {focus_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    rsx! {
        rect {
            onpointerpress,
            onpointerenter,
            onpointerleave,
            onkeydown,
            a11y_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            overflow: "clip",
            a11y_role:"button",
            a11y_focusable,
            color: "{font_theme.color}",
            shadow: "2 2 5 6 rgb(0, 0, 0, 30)",
            border,
            corner_radius: "99",
            background: "{background}",
            background_opacity: "0.6",
            text_height: "disable-least-ascent",
            main_align: "center",
            cross_align: "center",
            backdrop_blur: "16",

            {&children}
        }
    }
}
