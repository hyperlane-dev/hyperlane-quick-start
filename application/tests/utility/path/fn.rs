use super::*;

#[test]
fn path_is_safe_path_accepts_plain_relative_path() {
    assert!(is_safe_path("assets/logo.png"));
}

#[test]
fn path_is_safe_path_accepts_nested_relative_path() {
    assert!(is_safe_path("a/b/c/d.css"));
}

#[test]
fn path_is_safe_path_rejects_parent_traversal() {
    assert!(!is_safe_path("../etc/passwd"));
    assert!(!is_safe_path("a/../../b"));
    assert!(!is_safe_path(".."));
}

#[test]
fn path_is_safe_path_rejects_backslash_segment() {
    assert!(!is_safe_path("..\\windows"));
    assert!(!is_safe_path("a\\b"));
}

#[test]
fn path_is_safe_path_rejects_absolute_path() {
    assert!(!is_safe_path("/etc/passwd"));
}

#[test]
fn path_extract_resource_paths_collects_tag_attributes() {
    let html: String =
        r#"<link href="style.css"><script src="app.js"></script><img src="logo.png">"#.to_string();
    let paths: Vec<String> = extract_resource_paths(&html);
    assert_eq!(paths, vec!["app.js", "logo.png", "style.css"]);
}

#[test]
fn path_extract_resource_paths_reads_object_data_attribute() {
    let html: String = r#"<object data="clip.swf"></object>"#.to_string();
    let paths: Vec<String> = extract_resource_paths(&html);
    assert_eq!(paths, vec!["clip.swf"]);
}

#[test]
fn path_extract_resource_paths_filters_external_and_data_uris() {
    let html: String = concat!(
        "<script src=\"https://cdn.example.com/a.js\"></script>",
        "<script src=\"http://cdn.example.com/b.js\"></script>",
        "<img src=\"//cdn.example.com/c.png\">",
        r#"<img src="data:image/png;base64,AAAA">"#,
        r#"<script src="local.js"></script>"#,
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths(&html);
    assert_eq!(paths, vec!["local.js"]);
}

#[test]
fn path_extract_resource_paths_filters_template_variables() {
    let css: String =
        r#".a { background: url("${ASSET_BASE}/a.png"); } .b { background: url("b.png"); }"#
            .to_string();
    let paths: Vec<String> = extract_resource_paths(&css);
    assert_eq!(paths, vec!["b.png"]);
}

#[test]
fn path_extract_resource_paths_deduplicates_and_sorts() {
    let html: String =
        r#"<script src="z.js"></script><script src="a.js"></script><script src="z.js"></script>"#
            .to_string();
    let paths: Vec<String> = extract_resource_paths(&html);
    assert_eq!(paths, vec!["a.js", "z.js"]);
}

#[test]
fn path_extract_resource_paths_collects_css_import_and_url() {
    let css: String = concat!(
        r#"@import "reset.css";"#,
        r#"body { background-image: url('bg.png'); }"#,
        r#"a { background-image: url(icon.svg); }"#,
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths(&css);
    assert_eq!(paths, vec!["bg.png", "icon.svg", "reset.css"]);
}

#[test]
fn path_extract_resource_paths_collects_module_import_and_new_url() {
    let js: String = concat!(
        "import { a } from './mod.js';\n",
        "const u = new URL('./worker.js', import.meta.url);\n",
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths(&js);
    assert_eq!(paths, vec!["./mod.js", "./worker.js"]);
}

#[test]
fn path_extract_resource_paths_by_extension_uses_all_extractors_for_html() {
    let html: String = concat!(
        r#"<script src="app.js"></script>"#,
        "import { a } from './mod.js';",
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths_by_extension(&html, RESOURCE_EXTENSION_HTML);
    assert_eq!(paths, vec!["./mod.js", "app.js"]);
}

#[test]
fn path_extract_resource_paths_by_extension_skips_tag_attributes_for_js() {
    let html: String = concat!(
        r#"<script src="app.js"></script>"#,
        "import { a } from './mod.js';",
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths_by_extension(&html, "js");
    assert_eq!(paths, vec!["./mod.js"]);
}

#[test]
fn path_extract_resource_paths_by_extension_only_reads_urls_for_css() {
    let html: String = concat!(
        r#"<img src="logo.png">"#,
        r#"body { background-image: url('bg.png'); }"#,
    )
    .to_string();
    let paths: Vec<String> = extract_resource_paths_by_extension(&html, "css");
    assert_eq!(paths, vec!["bg.png"]);
}

#[test]
fn path_extract_resource_paths_by_extension_returns_empty_for_unknown_extension() {
    let html: String = r#"<script src="app.js"></script>"#.to_string();
    let paths: Vec<String> = extract_resource_paths_by_extension(&html, "wasm");
    assert!(paths.is_empty());
}

#[test]
fn path_parse_range_header_clamps_end_to_last_byte() {
    let range: (u64, u64) = parse_range_header("bytes=0-99999", 4096).unwrap();
    assert_eq!(range, (0, 4095));
}

#[test]
fn path_parse_range_header_resolves_open_ended_to_last_byte() {
    let range: (u64, u64) = parse_range_header("bytes=4000-", 4096).unwrap();
    assert_eq!(range, (4000, 4095));
}

#[test]
fn path_parse_range_header_resolves_suffix_to_file_tail() {
    let range: (u64, u64) = parse_range_header("bytes=-100", 4096).unwrap();
    assert_eq!(range, (3996, 4095));
}

#[test]
fn path_parse_range_header_clamps_suffix_longer_than_file_to_whole_file() {
    let range: (u64, u64) = parse_range_header("bytes=-9999", 4096).unwrap();
    assert_eq!(range, (0, 4095));
}

#[test]
fn path_parse_range_header_rejects_zero_length_suffix() {
    let error: String = parse_range_header("bytes=-0", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_any_range_on_empty_file() {
    let open_error: String = parse_range_header("bytes=0-", 0).unwrap_err();
    assert_eq!(open_error, ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
    let suffix_error: String = parse_range_header("bytes=-10", 0).unwrap_err();
    assert_eq!(suffix_error, ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
    let single_error: String = parse_range_header("bytes=0-0", 0).unwrap_err();
    assert_eq!(single_error, ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
}

#[test]
fn path_parse_range_header_rejects_suffix_length_that_overflows_u64() {
    let error: String = parse_range_header("bytes=-99999999999999999999999", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_start_length_that_overflows_u64() {
    let error: String = parse_range_header("bytes=99999999999999999999999-", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_whitespace_padded_bounds() {
    let error: String = parse_range_header("bytes= 0 - 100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_signed_start_bound() {
    let error: String = parse_range_header("bytes=+10-100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_signed_end_bound() {
    let error: String = parse_range_header("bytes=10-+100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_signed_suffix_bound() {
    let error: String = parse_range_header("bytes=-+100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_multi_range_specification() {
    let error: String = parse_range_header("bytes=0-100,200-300", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn path_parse_range_header_rejects_start_at_file_size() {
    let error: String = parse_range_header("bytes=4096-", 4096).unwrap_err();
    assert_eq!(error, ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
}

#[test]
fn path_parse_range_header_rejects_inverted_bounds() {
    let error: String = parse_range_header("bytes=300-100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_START_GREATER_THAN_END);
}

#[test]
fn path_parse_range_header_rejects_inverted_bounds_after_clamping() {
    let error: String = parse_range_header("bytes=4000-10", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_START_GREATER_THAN_END);
}

#[test]
fn path_parse_range_header_rejects_specification_without_dash() {
    let error: String = parse_range_header("bytes=0", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_SPECIFICATION);
}

#[tokio::test]
async fn path_read_file_range_reads_exact_window() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("asset.bin");
    let content: Vec<u8> = (0u8..=31).collect();
    let _: () = write(&path, &content).unwrap();
    let data: Vec<u8> = read_file_range(path.to_str().unwrap(), 8, 8).await.unwrap();
    assert_eq!(data, content[8..16].to_vec());
}

#[tokio::test]
async fn path_read_file_range_reports_short_read_when_length_past_end() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("short.bin");
    let content: Vec<u8> = b"0123456789".to_vec();
    let _: () = write(&path, &content).unwrap();
    let result: Result<Vec<u8>, String> = read_file_range(path.to_str().unwrap(), 8, 8).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn path_read_file_range_reports_open_failure_for_missing_file() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("absent.bin");
    let error: String = read_file_range(path.to_str().unwrap(), 0, 4)
        .await
        .unwrap_err();
    assert!(error.starts_with("Failed to open file"));
}
