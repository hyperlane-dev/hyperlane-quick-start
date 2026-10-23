use super::*;

/// The reactive state the application shell renders from.
///
/// The viewport width and the theme are hooks, so they are created once per
/// application and handed to the shell as props.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct AppShellState {
    /// Whether the viewport is narrow enough for the mobile layout.
    #[get(type(copy))]
    pub mobile: Signal<bool>,
    /// The class list of the application root.
    #[get(type(copy))]
    pub root_class: Signal<String>,
}
