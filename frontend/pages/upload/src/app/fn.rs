use super::*;

/// Builds the upload page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the files that are uploaded and of the upload history.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: UploadState = use_upload_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    upload_page {
                        state: state
                    }
                }
            })
        }
    }
}
