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
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            upload_page {
                state: state
            }
        }
    }
}
