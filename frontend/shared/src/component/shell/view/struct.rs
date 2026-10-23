use super::*;

/// The props of the application shell.
#[derive(Clone, Copy, Debug, Default)]
pub struct AppShellProps {
    /// Whether the viewport is narrow enough for the mobile layout.
    pub mobile: Signal<bool>,
    /// The class list of the application root.
    pub root_class: Signal<String>,
}
