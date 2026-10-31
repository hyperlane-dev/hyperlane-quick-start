use super::*;

/// Builds the ide page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// language of the scratchpad and the code it holds.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: IdeState = use_ide_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    ide_page {
                        state: state
                    }
                }
            })
        }
    }
}
