use super::*;

/// Builds the CI/CD page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the pipelines, of the runs and of the log stream.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: CicdState = use_cicd_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    cicd_page {
                        state: state
                    }
                }
            })
        }
    }
}
