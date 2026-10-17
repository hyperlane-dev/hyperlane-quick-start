use super::*;

#[test]
fn upload_parse_range_header_reads_explicit_start_and_end() {
    let range: RangeRequest = UploadService::parse_range_header("bytes=0-1023", 4096).unwrap();
    assert_eq!(range.get_start(), 0);
    assert_eq!(range.try_get_end(), Some(1023));
}

#[test]
fn upload_parse_range_header_reads_open_ended_tail_as_unbounded() {
    let range: RangeRequest = UploadService::parse_range_header("bytes=2048-", 4096).unwrap();
    assert_eq!(range.get_start(), 2048);
    assert_eq!(range.try_get_end(), None);
}

#[test]
fn upload_parse_range_header_reads_suffix_length_from_end_of_file() {
    let range: RangeRequest = UploadService::parse_range_header("bytes=-512", 4096).unwrap();
    assert_eq!(range.get_start(), 3584);
    assert_eq!(range.try_get_end(), Some(4095));
}

#[test]
fn upload_parse_range_header_clamps_suffix_start_to_zero_when_suffix_exceeds_size() {
    let range: RangeRequest = UploadService::parse_range_header("bytes=-9999", 4096).unwrap();
    assert_eq!(range.get_start(), 0);
    assert_eq!(range.try_get_end(), Some(4095));
}

#[test]
fn upload_parse_range_header_clamps_explicit_end_to_last_byte() {
    let range: RangeRequest = UploadService::parse_range_header("bytes=0-99999", 4096).unwrap();
    assert_eq!(range.get_start(), 0);
    assert_eq!(range.try_get_end(), Some(4095));
}

#[test]
fn upload_parse_range_header_rejects_zero_length_suffix() {
    let error: String = UploadService::parse_range_header("bytes=-0", 4096).unwrap_err();
    assert_eq!(error, "Invalid range: zero-length suffix");
}

#[test]
fn upload_parse_range_header_rejects_any_range_on_empty_file() {
    let start_error: String = UploadService::parse_range_header("bytes=0-", 0).unwrap_err();
    assert_eq!(
        start_error,
        service::upload::ERROR_RANGE_START_EXCEEDS_FILE_SIZE
    );
    let suffix_error: String = UploadService::parse_range_header("bytes=-10", 0).unwrap_err();
    assert_eq!(
        suffix_error,
        service::upload::ERROR_RANGE_START_EXCEEDS_FILE_SIZE
    );
    let single_error: String = UploadService::parse_range_header("bytes=0-0", 0).unwrap_err();
    assert_eq!(
        single_error,
        service::upload::ERROR_RANGE_START_EXCEEDS_FILE_SIZE
    );
}

#[test]
fn upload_parse_range_header_rejects_suffix_length_that_overflows_u64() {
    let error: String =
        UploadService::parse_range_header("bytes=-99999999999999999999999", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_END_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_start_length_that_overflows_u64() {
    let error: String =
        UploadService::parse_range_header("bytes=99999999999999999999999-", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_START_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_whitespace_padded_bounds() {
    let error: String = UploadService::parse_range_header("bytes= 0 - 100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_START_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_signed_start_bound() {
    let error: String = UploadService::parse_range_header("bytes=+10-100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_START_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_signed_end_bound() {
    let error: String = UploadService::parse_range_header("bytes=10-+100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_END_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_signed_suffix_bound() {
    let error: String = UploadService::parse_range_header("bytes=-+100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_END_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_missing_bytes_prefix() {
    let error: String = UploadService::parse_range_header("0-1023", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_HEADER_FORMAT);
}

#[test]
fn upload_parse_range_header_rejects_empty_specification() {
    let error: String = UploadService::parse_range_header("bytes=", 4096).unwrap_err();
    assert_eq!(error, service::upload::ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn upload_parse_range_header_rejects_specification_with_both_bounds_empty() {
    let error: String = UploadService::parse_range_header("bytes=-", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_RANGE_BOTH_EMPTY);
}

#[test]
fn upload_parse_range_header_rejects_multi_range_specification() {
    let error: String = UploadService::parse_range_header("bytes=0-100,200-300", 4096).unwrap_err();
    assert_eq!(error, service::upload::ERROR_INVALID_RANGE_SPECIFICATION);
}

#[test]
fn upload_parse_range_header_rejects_non_numeric_start() {
    let error: String = UploadService::parse_range_header("bytes=abc-100", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_START_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_non_numeric_end() {
    let error: String = UploadService::parse_range_header("bytes=0-xyz", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_END_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_non_numeric_suffix() {
    let error: String = UploadService::parse_range_header("bytes=-abc", 4096).unwrap_err();
    assert_eq!(error, ERROR_INVALID_END_RANGE);
}

#[test]
fn upload_parse_range_header_rejects_start_at_file_size() {
    let error: String = UploadService::parse_range_header("bytes=4096-", 4096).unwrap_err();
    assert_eq!(error, service::upload::ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
}

#[test]
fn upload_parse_range_header_rejects_start_past_file_size() {
    let error: String = UploadService::parse_range_header("bytes=5000-6000", 4096).unwrap_err();
    assert_eq!(error, service::upload::ERROR_RANGE_START_EXCEEDS_FILE_SIZE);
}

#[test]
fn upload_parse_range_header_rejects_inverted_bounds() {
    let error: String = UploadService::parse_range_header("bytes=300-100", 4096).unwrap_err();
    assert_eq!(
        error,
        service::upload::ERROR_INVALID_RANGE_START_GREATER_THAN_END
    );
}

#[test]
fn upload_parse_range_header_rejects_inverted_bounds_after_clamping() {
    let error: String = UploadService::parse_range_header("bytes=4000-10", 4096).unwrap_err();
    assert_eq!(
        error,
        service::upload::ERROR_INVALID_RANGE_START_GREATER_THAN_END
    );
}

#[tokio::test]
async fn upload_read_file_range_returns_requested_window() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("payload.bin");
    let content: Vec<u8> = (0u8..=63).collect();
    let _: () = write(&path, &content).unwrap();
    let data: Vec<u8> = UploadService::read_file_range(path.to_str().unwrap(), 10, 20)
        .await
        .unwrap();
    assert_eq!(data, content[10..30].to_vec());
}

#[tokio::test]
async fn upload_read_file_range_returns_empty_when_length_is_zero() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("payload.bin");
    let content: Vec<u8> = b"abcdefgh".to_vec();
    let _: () = write(&path, &content).unwrap();
    let data: Vec<u8> = UploadService::read_file_range(path.to_str().unwrap(), 0, 0)
        .await
        .unwrap();
    assert!(data.is_empty());
}

#[tokio::test]
async fn upload_read_file_range_returns_empty_when_start_is_at_end_of_file() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("payload.bin");
    let content: Vec<u8> = b"abcdefgh".to_vec();
    let _: () = write(&path, &content).unwrap();
    let data: Vec<u8> = UploadService::read_file_range(path.to_str().unwrap(), 8, 16)
        .await
        .unwrap();
    assert!(data.is_empty());
}

#[tokio::test]
async fn upload_read_file_range_truncates_to_available_bytes_when_length_past_end() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("payload.bin");
    let content: Vec<u8> = b"abcdefgh".to_vec();
    let _: () = write(&path, &content).unwrap();
    let data: Vec<u8> = UploadService::read_file_range(path.to_str().unwrap(), 4, 16)
        .await
        .unwrap();
    assert_eq!(data, b"efgh".to_vec());
}

#[tokio::test]
async fn upload_read_file_range_reports_open_failure_for_missing_file() {
    let dir: TempDir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("absent.bin");
    let error: String = UploadService::read_file_range(path.to_str().unwrap(), 0, 4)
        .await
        .unwrap_err();
    assert!(error.starts_with("Failed to open file"));
}
