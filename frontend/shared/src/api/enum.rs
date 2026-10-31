use super::*;

/// The HTTP method a request is issued with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMethod {
    /// Reads a resource.
    Get,
    /// Creates a resource.
    Post,
    /// Updates a resource.
    Put,
    /// Removes a resource.
    Delete,
}
