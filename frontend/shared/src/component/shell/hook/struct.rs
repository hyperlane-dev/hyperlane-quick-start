use super::*;

/// The reactive state the application shell renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct AppShellState {
    /// Whether the viewport is narrow enough for the drawer layout.
    #[get(type(copy))]
    pub mobile: Signal<bool>,
    /// The active theme name.
    #[get(type(copy))]
    pub theme: Signal<String>,
    /// The class list of the application root.
    #[get(type(copy))]
    pub root_class: Signal<String>,
    /// Whether the mobile navigation drawer is open.
    #[get(type(copy))]
    pub drawer_open: Signal<bool>,
}
