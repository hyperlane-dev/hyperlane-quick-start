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
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    gomoku_page {
                        state: state
                    }
                }
            })
        }
    }
}
