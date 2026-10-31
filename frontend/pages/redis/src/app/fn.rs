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
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    redis_page {
                        state: state
                    }
                }
            })
        }
    }
}
