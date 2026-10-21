use super::*;

/// Builds the path of the endpoint that answers with the trace.
///
/// The identifier of the trace is escaped, so a trace that holds a slash
/// or a space still reaches the server as one path segment.
///
/// # Arguments
///
/// - `&str` - The identifier of the trace.
///
/// # Returns
///
/// - `String` - The path of the endpoint.
pub fn trace_path(trace: &str) -> String {
    format!("{PATH_TRACE}{PATH_SEPARATOR}{}", encode_uri_component(trace))
}
