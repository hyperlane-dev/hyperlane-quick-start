use super::*;

/// Builds the shortlink page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the link being written and of the link that was created.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: ShortlinkState = use_shortlink_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    shortlink_page {
                        state: state
                    }
                }
            })
        }
    }
}
