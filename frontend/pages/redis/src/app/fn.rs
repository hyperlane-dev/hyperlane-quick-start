use super::*;

/// Builds the redis page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// records of the cache and the record being written.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: RedisState = use_redis_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            reactive_scope {
                content: Rc::new(move || html! {
                    redis_page {
                        state: state
                    }
                })
            }
        }
    }
}
