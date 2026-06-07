use super::*;

/// build auth redirect url.
#[instrument_trace]
pub fn build_auth_redirect_url(ctx: &Context) -> String {
    let current_path: String = ctx.get_request().get_path().clone();
    let querys: &RequestQuerys = ctx.get_request().get_querys();
    let query_parts: Vec<String> = querys
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();
    let query_string: String = query_parts.join("&");
    let full_path: String = if query_string.is_empty() {
        current_path
    } else {
        format!("{current_path}?{query_string}")
    };
    let encoded_path: String = urlencoding::encode(&full_path).to_string();
    format!("/auth?{LOCATION}={encoded_path}")
}
