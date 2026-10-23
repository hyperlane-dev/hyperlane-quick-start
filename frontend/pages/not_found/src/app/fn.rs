use super::*;

/// Builds the not found page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// status code and the links the card offers.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: NotFoundState = use_not_found_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    not_found_page {
                        state: state
                    }
                }
            })
        }
    }
}
