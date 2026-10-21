use super::*;

/// Builds the landing page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// moment the page was opened and the links the card offers.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: TemplatesState = use_templates_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            templates_page {
                state: state
            }
        }
    }
}
