use super::*;

/// Builds the order page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the records, of the statistics, of the users and of the profile.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: OrderState = use_order_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    order_page {
                        state: state
                    }
                }
            })
        }
    }
}
