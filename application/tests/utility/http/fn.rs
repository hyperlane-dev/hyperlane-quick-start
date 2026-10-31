use super::*;

#[test]
fn http_format_content_type_appends_charset_for_text_extension() {
    let formatted: String = format_content_type("text/html", "html");
    assert_eq!(formatted, "text/html; charset=utf-8");
}

#[test]
fn http_format_content_type_leaves_binary_content_type_unchanged() {
    let formatted: String = format_content_type("video/mp4", "mp4");
    assert_eq!(formatted, "video/mp4");
}

#[test]
fn http_format_content_type_passes_through_empty_content_type() {
    let formatted: String = format_content_type("", "html");
    assert_eq!(formatted, "; charset=utf-8");
}

#[test]
fn http_format_content_type_uses_extension_not_content_type_prefix() {
    let formatted: String = format_content_type("video/mp4", "html");
    assert_eq!(formatted, "video/mp4; charset=utf-8");
}

#[test]
fn http_is_gzip_compressible_accepts_whitelisted_extension() {
    assert!(is_gzip_compressible("html"));
    assert!(is_gzip_compressible("wasm"));
    assert!(is_gzip_compressible("png"));
}

#[test]
fn http_is_gzip_compressible_rejects_unlisted_extension() {
    assert!(!is_gzip_compressible("mp4"));
    assert!(!is_gzip_compressible("zip"));
}

#[test]
fn http_is_gzip_compressible_is_case_sensitive() {
    assert!(!is_gzip_compressible("HTML"));
}

#[test]
fn http_build_auth_redirect_url_without_query() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request().set_path("/orders".to_string());
    let url: String = build_auth_redirect_url(&ctx);
    assert_eq!(url, "/auth?location=%2Forders");
}

#[test]
fn http_build_auth_redirect_url_encodes_query_values() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request().set_path("/search".to_string());
    ctx.get_mut_request().querys.insert(
        "q".to_string(),
        VecDeque::from(vec!["rust wasm".to_string()]),
    );
    let url: String = build_auth_redirect_url(&ctx);
    assert!(url.starts_with("/auth?location="));
    let encoded: String = url.trim_start_matches("/auth?location=").to_string();
    let decoded: String = decode(&encoded).unwrap().into_owned();
    assert!(decoded.contains("/search?q=rust wasm"));
}

#[test]
fn http_build_auth_redirect_url_joins_repeated_query_values() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request().set_path("/list".to_string());
    ctx.get_mut_request().querys.insert(
        "tag".to_string(),
        VecDeque::from(vec!["a".to_string(), "b".to_string()]),
    );
    let url: String = build_auth_redirect_url(&ctx);
    let encoded: String = url.trim_start_matches("/auth?location=").to_string();
    let decoded: String = decode(&encoded).unwrap().into_owned();
    assert!(decoded.contains("tag=a&tag=b"));
}

#[tokio::test]
async fn http_get_request_json_replaces_body_with_its_length() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request().set_path("/submit".to_string());
    ctx.get_mut_request().set_body(b"0123456789".to_vec());
    let json: String = get_request_json(&ctx).await;
    let value: Value = from_str(&json).unwrap();
    assert_eq!(value.get("path").unwrap().as_str().unwrap(), "/submit");
    let body: Vec<u8> = from_value(value.get("body").unwrap().clone()).unwrap();
    assert_eq!(String::from_utf8(body).unwrap(), "10");
}

#[tokio::test]
async fn http_get_request_json_preserves_method_and_host() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request().set_method(Method::Post);
    ctx.get_mut_request().set_host("example.com".to_string());
    let json: String = get_request_json(&ctx).await;
    let value: Value = from_str(&json).unwrap();
    assert_eq!(value.get("method").unwrap().as_str().unwrap(), "Post");
    assert_eq!(value.get("host").unwrap().as_str().unwrap(), "example.com");
}

#[tokio::test]
async fn http_get_request_json_reports_zero_length_for_empty_body() {
    let ctx: Context = Context::default();
    let json: String = get_request_json(&ctx).await;
    let value: Value = from_str(&json).unwrap();
    let body: Vec<u8> = from_value(value.get("body").unwrap().clone()).unwrap();
    assert_eq!(String::from_utf8(body).unwrap(), "0");
}

#[tokio::test]
async fn http_get_response_json_replaces_body_with_its_length() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_response().set_status_code(200);
    ctx.get_mut_response().set_body(b"12345");
    let json: String = get_response_json(&ctx).await;
    let value: Value = from_str(&json).unwrap();
    assert_eq!(value.get("status_code").unwrap().as_u64().unwrap(), 200);
    let body: Vec<u8> = from_value(value.get("body").unwrap().clone()).unwrap();
    assert_eq!(String::from_utf8(body).unwrap(), "5");
}

#[tokio::test]
async fn http_get_response_json_reports_zero_length_for_empty_body() {
    let ctx: Context = Context::default();
    let json: String = get_response_json(&ctx).await;
    let value: Value = from_str(&json).unwrap();
    let body: Vec<u8> = from_value(value.get("body").unwrap().clone()).unwrap();
    assert_eq!(String::from_utf8(body).unwrap(), "0");
}

#[tokio::test]
async fn http_get_response_json_leaves_context_response_body_untouched() {
    let mut ctx: Context = Context::default();
    ctx.get_mut_response().set_body(b"12345");
    let _: String = get_response_json(&ctx).await;
    assert_eq!(ctx.get_response().get_body(), b"12345");
}
