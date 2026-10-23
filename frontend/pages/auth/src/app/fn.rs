use super::*;

/// Builds the authentication page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the two forms and of the message the server answered with.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: AuthState = use_auth_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    auth_page {
                        state: state
                    }
                }
            })
        }
    }
}
