use super::*;

#[test]
fn project_encode_id_and_decode_id_round_trip_positive() {
    let id: i64 = 12345;
    let encoded: String = EuvPlaygroundService::encode_id(id);
    let decoded: i64 = EuvPlaygroundService::decode_id(&encoded).unwrap();
    assert_eq!(decoded, id);
}

#[test]
fn project_encode_id_and_decode_id_round_trip_negative() {
    let id: i64 = -9876;
    let encoded: String = EuvPlaygroundService::encode_id(id);
    let decoded: i64 = EuvPlaygroundService::decode_id(&encoded).unwrap();
    assert_eq!(decoded, id);
}

#[test]
fn project_encode_id_is_not_the_plain_numeric_form() {
    let encoded: String = EuvPlaygroundService::encode_id(42);
    assert_ne!(encoded, "42");
}

#[test]
fn project_decode_id_rejects_non_numeric_input() {
    let error: String = EuvPlaygroundService::decode_id("!!not-a-number!!").unwrap_err();
    assert_eq!(error, ERROR_INVALID_ID_FORMAT);
}

#[test]
fn project_encode_job_id_and_decode_job_id_round_trip() {
    let job_id: BuildJobId = 987654321;
    let encoded: String = EuvPlaygroundService::encode_job_id(job_id);
    let decoded: BuildJobId = EuvPlaygroundService::decode_job_id(&encoded).unwrap();
    assert_eq!(decoded, job_id);
}

#[test]
fn project_decode_job_id_rejects_negative_value() {
    let error: String = EuvPlaygroundService::decode_job_id("-1").unwrap_err();
    assert_eq!(error, ERROR_INVALID_ID_FORMAT);
}

#[test]
fn project_normalize_name_trims_surrounding_whitespace() {
    assert_eq!(EuvPlaygroundService::normalize_name("  spaced  "), "spaced");
}

#[test]
fn project_normalize_name_collapses_internal_whitespace_runs() {
    assert_eq!(EuvPlaygroundService::normalize_name("a \t\n  b"), "a b");
}

#[test]
fn project_normalize_name_strips_html_active_characters() {
    assert_eq!(
        EuvPlaygroundService::normalize_name(r#"<script>alert("x")</script>"#),
        "script alert( x ) script"
    );
}

#[test]
fn project_normalize_name_strips_path_separators() {
    assert_eq!(EuvPlaygroundService::normalize_name(r"a/b\c"), "a b c");
}

#[test]
fn project_normalize_name_strips_control_characters() {
    assert_eq!(
        EuvPlaygroundService::normalize_name("a\u{0}b\u{1}c"),
        "a b c"
    );
}

#[test]
fn project_normalize_name_falls_back_to_untitled_when_everything_is_stripped() {
    assert_eq!(
        EuvPlaygroundService::normalize_name(""),
        UNTITLED_PROJECT_NAME
    );
    assert_eq!(
        EuvPlaygroundService::normalize_name("///"),
        UNTITLED_PROJECT_NAME
    );
}

#[test]
fn project_normalize_name_truncates_to_max_chars_on_char_boundary() {
    let input: String = "é".repeat(EUV_PLAYGROUND_MAX_NAME_LEN + 20);
    let normalized: String = EuvPlaygroundService::normalize_name(&input);
    assert_eq!(normalized.chars().count(), EUV_PLAYGROUND_MAX_NAME_LEN);
    assert!(normalized.chars().all(|ch: char| ch == 'é'));
}

#[test]
fn project_normalize_name_keeps_multibyte_name_under_limit_intact() {
    let input: String = "日本語のプロジェクト名".to_string();
    assert_eq!(EuvPlaygroundService::normalize_name(&input), input);
}

#[test]
fn project_resolve_wasm_pack_binary_prefers_explicit_override() {
    let dir: TempDir = tempdir().unwrap();
    let override_path: PathBuf = dir.path().join("custom-wasm-pack");
    let _: () = write(&override_path, b"binary").unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        Some(override_path.as_os_str()),
        None,
        None,
        None,
    );
    assert_eq!(resolved, override_path);
}

#[test]
fn project_resolve_wasm_pack_binary_ignores_empty_override() {
    let dir: TempDir = tempdir().unwrap();
    let bin_dir: PathBuf = dir.path().join("bin");
    let _: () = create_dir_all(&bin_dir).unwrap();
    let path_binary: PathBuf = bin_dir.join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let _: () = write(&path_binary, b"binary").unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        Some(OsStr::new("")),
        Some(bin_dir.as_os_str()),
        None,
        None,
    );
    assert_eq!(resolved, path_binary);
}

#[test]
fn project_resolve_wasm_pack_binary_scans_path_entries_in_order() {
    let first_dir: TempDir = tempdir().unwrap();
    let second_dir: TempDir = tempdir().unwrap();
    let first_binary: PathBuf = first_dir.path().join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let second_binary: PathBuf = second_dir.path().join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let _: () = write(&second_binary, b"binary").unwrap();
    let joined_path: OsString = {
        let mut joined: OsString = first_dir.path().as_os_str().to_os_string();
        joined.push(":");
        joined.push(second_dir.path().as_os_str());
        joined
    };
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        None,
        Some(joined_path.as_os_str()),
        None,
        None,
    );
    assert_eq!(resolved, second_binary);
    assert!(!first_binary.exists());
}

#[test]
fn project_resolve_wasm_pack_binary_falls_back_to_cargo_home_bin() {
    let cargo_home: TempDir = tempdir().unwrap();
    let bin_dir: PathBuf = cargo_home.path().join(EUV_PLAYGROUND_CARGO_BIN_DIR);
    let _: () = create_dir_all(&bin_dir).unwrap();
    let cargo_binary: PathBuf = bin_dir.join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let _: () = write(&cargo_binary, b"binary").unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        None,
        None,
        Some(cargo_home.path().as_os_str()),
        None,
    );
    assert_eq!(resolved, cargo_binary);
}

#[test]
fn project_resolve_wasm_pack_binary_falls_back_to_home_cargo_bin() {
    let home: TempDir = tempdir().unwrap();
    let bin_dir: PathBuf = home
        .path()
        .join(EUV_PLAYGROUND_CARGO_HOME_DIR)
        .join(EUV_PLAYGROUND_CARGO_BIN_DIR);
    let _: () = create_dir_all(&bin_dir).unwrap();
    let home_binary: PathBuf = bin_dir.join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let _: () = write(&home_binary, b"binary").unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        None,
        None,
        None,
        Some(home.path().as_os_str()),
    );
    assert_eq!(resolved, home_binary);
}

#[test]
fn project_resolve_wasm_pack_binary_prefers_cargo_home_over_home() {
    let cargo_home: TempDir = tempdir().unwrap();
    let home: TempDir = tempdir().unwrap();
    let cargo_bin_dir: PathBuf = cargo_home.path().join(EUV_PLAYGROUND_CARGO_BIN_DIR);
    let home_bin_dir: PathBuf = home
        .path()
        .join(EUV_PLAYGROUND_CARGO_HOME_DIR)
        .join(EUV_PLAYGROUND_CARGO_BIN_DIR);
    let _: () = create_dir_all(&cargo_bin_dir).unwrap();
    let _: () = create_dir_all(&home_bin_dir).unwrap();
    let cargo_binary: PathBuf = cargo_bin_dir.join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let home_binary: PathBuf = home_bin_dir.join(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME);
    let _: () = write(&cargo_binary, b"binary").unwrap();
    let _: () = write(&home_binary, b"binary").unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        None,
        None,
        Some(cargo_home.path().as_os_str()),
        Some(home.path().as_os_str()),
    );
    assert_eq!(resolved, cargo_binary);
}

#[test]
fn project_resolve_wasm_pack_binary_returns_bare_name_when_nothing_found() {
    let empty: TempDir = tempdir().unwrap();
    let resolved: PathBuf = EuvPlaygroundService::resolve_wasm_pack_binary_from(
        None,
        None,
        Some(empty.path().as_os_str()),
        Some(empty.path().as_os_str()),
    );
    assert_eq!(
        resolved,
        PathBuf::from(EUV_PLAYGROUND_WASM_PACK_BINARY_NAME)
    );
}

#[test]
fn project_write_project_persists_code_and_metadata() {
    let dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = dir.path().join("project");
    let timestamp: i64 =
        EuvPlaygroundService::write_project(&project_dir, "demo", "fn main() {}").unwrap();
    let code: String = EuvPlaygroundService::read_code(&project_dir).unwrap();
    assert_eq!(code, "fn main() {}");
    let metadata: Option<(String, i64)> = EuvPlaygroundService::read_metadata(&project_dir);
    let (name, updated_at_ms): (String, i64) = metadata.unwrap();
    assert_eq!(name, "demo");
    assert_eq!(updated_at_ms, timestamp);
    assert!(timestamp > 0);
}

#[test]
fn project_read_metadata_returns_none_for_missing_file() {
    let dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = dir.path().join("absent");
    let metadata: Option<(String, i64)> = EuvPlaygroundService::read_metadata(&project_dir);
    assert!(metadata.is_none());
}

#[test]
fn project_read_metadata_returns_none_for_unparseable_file() {
    let dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = dir.path().join("broken");
    let _: () = create_dir_all(&project_dir).unwrap();
    let _: () = write(
        project_dir.join(EUV_PLAYGROUND_META_FILE),
        b"not json at all",
    )
    .unwrap();
    let metadata: Option<(String, i64)> = EuvPlaygroundService::read_metadata(&project_dir);
    assert!(metadata.is_none());
}

#[test]
fn project_write_metadata_round_trips_name_and_timestamp() {
    let dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = dir.path().join("meta-only");
    let _: () = create_dir_all(&project_dir).unwrap();
    EuvPlaygroundService::write_metadata(&project_dir, "named", 4242);
    let metadata: Option<(String, i64)> = EuvPlaygroundService::read_metadata(&project_dir);
    let (name, updated_at_ms): (String, i64) = metadata.unwrap();
    assert_eq!(name, "named");
    assert_eq!(updated_at_ms, 4242);
}

#[test]
fn project_next_project_id_starts_at_one_and_increments_monotonically() {
    let dir: TempDir = tempdir().unwrap();
    let first: i64 = EuvPlaygroundService::next_project_id(dir.path());
    let second: i64 = EuvPlaygroundService::next_project_id(dir.path());
    let third: i64 = EuvPlaygroundService::next_project_id(dir.path());
    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(third, 3);
}

#[test]
fn project_next_project_id_resumes_from_persisted_sequence() {
    let dir: TempDir = tempdir().unwrap();
    let _: () = write(dir.path().join(EUV_PLAYGROUND_SEQ_FILE), b"41").unwrap();
    let next: i64 = EuvPlaygroundService::next_project_id(dir.path());
    assert_eq!(next, 42);
}

#[test]
fn project_project_name_exists_detects_matching_project() {
    let user_dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = user_dir.path().join("p1");
    let _: i64 = EuvPlaygroundService::write_project(&project_dir, "demo", "").unwrap();
    assert!(EuvPlaygroundService::project_name_exists(user_dir.path(), "demo").unwrap());
    assert!(!EuvPlaygroundService::project_name_exists(user_dir.path(), "other").unwrap());
}

#[test]
fn project_project_name_exists_ignores_excluded_project() {
    let user_dir: TempDir = tempdir().unwrap();
    let project_dir: PathBuf = user_dir.path().join("p1");
    let _: i64 = EuvPlaygroundService::write_project(&project_dir, "demo", "").unwrap();
    let found: bool = EuvPlaygroundService::project_name_exists_excluding(
        user_dir.path(),
        "demo",
        Some(project_dir.as_path()),
    )
    .unwrap();
    assert!(!found);
}

#[test]
fn project_project_name_exists_reports_error_for_absent_user_dir() {
    let dir: TempDir = tempdir().unwrap();
    let absent: PathBuf = dir.path().join("no-such-user");
    let result: Result<bool, String> = EuvPlaygroundService::project_name_exists(&absent, "demo");
    assert!(result.is_err());
}
