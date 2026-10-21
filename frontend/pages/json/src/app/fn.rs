use super::*;

/// Builds the JSON editor page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the document, of the tree and of the field that is edited.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: JsonState = use_json_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            json_page {
                state: state
            }
        }
    }
}
