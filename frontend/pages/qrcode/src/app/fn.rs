use super::*;

/// Builds the QR code page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the list of the codes and of the file that is imported.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: QrcodeState = use_qrcode_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    qrcode_page {
                        state: state
                    }
                }
            })
        }
    }
}
