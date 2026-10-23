use super::*;

/// Renders the application shell around a page.
///
/// The shell is the frame of the page: the root carries the theme of the
/// application and the page content fills the frame. Every page of the
/// application is loaded at its own path, so the frame carries no navigation
/// of its own; the visitor reaches a page by its address.
///
/// # Arguments
///
/// - `VirtualNode<AppShellProps>` - The props node carrying the shell state.
///
/// # Returns
///
/// - `VirtualNode` - The shell around the page content.
#[component]
pub fn app_shell(node: VirtualNode<AppShellProps>) -> VirtualNode {
    let AppShellProps {
        mobile,
        root_class,
    }: AppShellProps = node.try_get_props().unwrap_or_default();
    let content: VirtualNode = node.get_children().into();
    inject_shared_css();
    html! {
        div {
            class: root_class
            main {
                class: if { mobile.get() } {
                    c_mobile_main()
                } else {
                    c_app_main()
                }
                content
            }
        }
    }
}
