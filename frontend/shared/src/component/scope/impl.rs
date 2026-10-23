use super::*;

/// Implementation of `ReactiveScopeProps`.
impl Default for ReactiveScopeProps {
    /// Builds the props of a scope without content.
    ///
    /// # Returns
    ///
    /// - `ReactiveScopeProps` - The props of an empty scope.
    fn default() -> Self {
        Self {
            content: Rc::new(|| VirtualNode::default()),
        }
    }
}
