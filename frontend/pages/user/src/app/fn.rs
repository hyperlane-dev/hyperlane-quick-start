use super::*;

/// Builds the user management page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the user list and of the user whose detail is open.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: UserState = use_user_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    current_path: PAGE_PATH
                    mobile: shell.get_mobile()
                    theme: shell.get_theme()
                    root_class: shell.get_root_class()
                    drawer_open: shell.get_drawer_open()
                    user_page {
                        state: state
                    }
                }
            })
        }
    }
}
