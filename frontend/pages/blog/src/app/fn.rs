use super::*;

/// Builds the blog page application.
///
/// The shell owns the navigation and the theme, the page itself owns the
/// state of the post lists and of the post being written.
///
/// # Returns
///
/// - `VirtualNode` - The application tree.
pub(crate) fn app() -> VirtualNode {
    let shell: AppShellState = use_app_shell();
    let state: BlogState = use_blog_state();
    html! {
        reactive_scope {
            content: Rc::new(move || html! {
                app_shell {
                    mobile: shell.get_mobile()
                    root_class: shell.get_root_class()
                    blog_page {
                        state: state
                    }
                }
            })
        }
    }
}
