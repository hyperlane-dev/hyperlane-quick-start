use super::*;

/// Renders its content inside a reactive scope.
///
/// The application mounts the page into the document root, and the root of
/// the framework renders the tree without a scope of its own. A signal that a
/// page reads in plain Rust code is therefore not tracked, and the value the
/// page read stays the one of the first render. The scope gives the content a
/// scope of its own, so the signals the content reads are tracked and the
/// content renders again when they change.
///
/// # Arguments
///
/// - `VirtualNode<ReactiveScopeProps>` - The props node carrying the content.
///
/// # Returns
///
/// - `VirtualNode` - The content inside its scope.
#[component]
pub fn reactive_scope(node: VirtualNode<ReactiveScopeProps>) -> VirtualNode {
    let ReactiveScopeProps { content }: ReactiveScopeProps = node.try_get_props().unwrap_or_default();
    let active: Signal<bool> = Signal::create(true);
    html! {
        match { active.get() } {
            true => {
                content()
            }
            false => {
                ""
            }
        }
    }
}
