use super::*;

/// Builds the chat page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the room, of the message being written and of the visitors
/// that are online.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: ChatState = use_chat_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    chat_page {
                        state: state
                    }
                }
            })
        }
    }
}
