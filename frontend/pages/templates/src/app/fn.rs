use super::*;

/// Builds the landing page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// moment the page was opened and the links the card offers.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: TemplatesState = use_templates_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    templates_page {
                        state: state
                    }
                }
            })
        }
    }
}
