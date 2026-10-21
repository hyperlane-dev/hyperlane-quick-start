use super::*;

/// Implementation of `HttpMethod`.
impl HttpMethod {
    /// Returns the method name used on the wire.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The HTTP method name.
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => METHOD_GET,
            HttpMethod::Post => METHOD_POST,
            HttpMethod::Put => METHOD_PUT,
            HttpMethod::Delete => METHOD_DELETE,
        }
    }
}
