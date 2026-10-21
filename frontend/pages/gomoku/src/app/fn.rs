use super::*;

/// Builds the gomoku page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the room and of the board.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: GomokuState = use_gomoku_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            reactive_scope {
                content: Rc::new(move || html! {
                    gomoku_page {
                        state: state
                    }
                })
            }
        }
    }
}
