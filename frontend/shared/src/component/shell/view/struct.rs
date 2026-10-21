use super::*;

/// The props of the application shell.
#[derive(Clone, Copy, Debug, Default)]
pub struct AppShellProps {
    /// The static path of the page being displayed, used to mark the active navigation entry.
    pub current_path: &'static str,
    /// Whether the viewport is narrow enough for the drawer layout.
    pub mobile: Signal<bool>,
    /// The active theme name.
    pub theme: Signal<String>,
    /// The class list of the application root.
    pub root_class: Signal<String>,
    /// Whether the mobile navigation drawer is open.
    pub drawer_open: Signal<bool>,
}

/// The props of the navigation entries.
#[derive(Clone, Copy, Debug, Default)]
pub struct NavEntriesProps {
    /// The static path of the page being displayed, used to mark the active navigation entry.
    pub current_path: &'static str,
}

/// The props of the theme button.
#[derive(Clone, Copy, Debug, Default)]
pub struct ThemeButtonProps {
    /// The active theme name.
    pub theme: Signal<String>,
    /// The class name the button carries.
    pub class: &'static str,
}
