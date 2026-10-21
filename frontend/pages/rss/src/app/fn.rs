use super::*;

/// Builds the rss page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// items of the feed and the page of the feed being displayed.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: RssState = use_rss_state();
    html! {
        app_shell {
            current_path: PAGE_PATH
            mobile: shell.get_mobile()
            theme: shell.get_theme()
            root_class: shell.get_root_class()
            drawer_open: shell.get_drawer_open()
            rss_page {
                state: state
            }
        }
    }
}
