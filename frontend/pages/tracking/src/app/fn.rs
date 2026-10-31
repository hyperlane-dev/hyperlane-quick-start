use super::*;

/// Builds the tracking page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// filters of the query and the records the query answered with.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: TrackingState = use_tracking_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    tracking_page {
                        state: state
                    }
                }
            })
        }
    }
}
