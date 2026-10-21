use super::*;

/// Builds the euv playground page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// projects of the visitor, the code of the editor and the build that runs
/// in the background.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: PlaygroundState = use_playground_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            reactive_scope {
                content: Rc::new(move || html! {
                    playground_page {
                        state: state
                    }
                })
            }
        }
    }
}
