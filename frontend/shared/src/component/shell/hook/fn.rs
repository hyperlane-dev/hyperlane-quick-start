use super::*;

/// Creates the reactive state the application shell renders from.
///
/// The viewport width, the theme and the drawer state are hooks, so they are
/// created once per application and handed to the shell as props.
///
/// # Returns
///
/// - `AppShellState` - The state the shell renders from.
pub fn use_app_shell() -> AppShellState {
    let mobile: Signal<bool> = use_resize();
    let theme_state: ThemeState = ThemeState::use_theme_state(mobile);
    let drawer_open: Signal<bool> = App::use_signal(|| false);
    AppShellState {
        mobile,
        theme: theme_state.get_theme(),
        root_class: theme_state.get_root_class(),
        drawer_open,
    }
}

/// Tracks whether the viewport is narrow enough for the drawer layout.
///
/// The signal starts at the width the page was opened with and follows every
/// later resize, so the shell swaps between the sidebar and the drawer as the
/// window changes.
///
/// # Returns
///
/// - `Signal<bool>` - Whether the mobile layout is in use.
pub fn use_resize() -> Signal<bool> {
    let mobile: Signal<bool> = App::use_signal(is_mobile);
    App::use_window_event(EVENT_RESIZE, move || {
        let next: bool = is_mobile();
        if mobile.get() != next {
            mobile.set(next);
        }
    });
    mobile
}

/// Reports whether the viewport is narrower than the mobile breakpoint.
///
/// # Returns
///
/// - `bool` - Whether the mobile layout is in use.
fn is_mobile() -> bool {
    let Some(window): Option<Window> = window() else {
        return false;
    };
    let width: f64 = window
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    width < MOBILE_BREAKPOINT
}
