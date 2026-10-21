use super::*;

/// Builds the notification page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the list, of the filters and of the notification that is written.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: NotificationState = use_notification_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            reactive_scope {
                content: Rc::new(move || html! {
                    notification_page {
                        state: state
                    }
                })
            }
        }
    }
}
