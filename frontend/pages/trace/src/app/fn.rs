use super::*;

/// Builds the trace page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// trace being looked for and the answer of the server.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: TraceState = use_trace_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    trace_page {
                        state: state
                    }
                }
            })
        }
    }
}
