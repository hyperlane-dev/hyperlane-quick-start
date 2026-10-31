use super::*;

/// Builds the JSON editor page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the document, of the tree and of the field that is edited.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: JsonState = use_json_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    json_page {
                        state: state
                    }
                }
            })
        }
    }
}
