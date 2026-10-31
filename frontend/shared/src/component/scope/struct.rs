use super::*;

/// The props of the reactive scope.
#[derive(Clone)]
pub struct ReactiveScopeProps {
    /// The content of the scope.
    pub content: Rc<dyn Fn() -> VirtualNode>,
}
