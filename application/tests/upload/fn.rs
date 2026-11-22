use super::*;

fn try_boundary() -> &'static str {
    "----HermesTestBoundary"
}

fn try_headers(content_type: &str) -> RequestHeaders {
    let mut headers: RequestHeaders = hash_map_xx_hash3_64();
    headers.insert(
        "content-type".to_string(),
        VecDeque::from(vec![content_type.to_string()]),
    );
    headers
}

fn try_request(body: Vec<u8>, content_type: String) -> Request {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request()
        .set_method(Method::Post)
        .set_host("localhost".to_string())
        .set_version(HttpVersion::Http1_1)
        .set_path("/api/upload/save".to_string())
        .set_headers(try_headers(&content_type))
        .set_body(body);
    ctx.get_request().clone()
}

fn try_context(body: Vec<u8>, content_type: String) -> Context {
    let mut ctx: Context = Context::default();
    ctx.get_mut_request()
        .set_body(body)
        .set_headers(try_headers(&content_type));
    ctx
}

fn try_text_part(boundary: &str, name: &str, value: &str) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    out.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
    );
    out.extend_from_slice(value.as_bytes());
    out.extend_from_slice(b"\r\n");
    out
}

fn try_file_part(boundary: &str, file_name: &str, content_type: &str, data: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    out.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n")
            .as_bytes(),
    );
    out.extend_from_slice(format!("Content-Type: {content_type}\r\n\r\n").as_bytes());
    out.extend_from_slice(data);
    out.extend_from_slice(b"\r\n");
    out
}

fn try_close(boundary: &str) -> Vec<u8> {
    format!("--{boundary}--\r\n").into_bytes()
}

fn try_content_type() -> String {
    format!("multipart/form-data; boundary={}", try_boundary())
}

fn try_parts(body: Vec<u8>) -> Vec<BodyPart> {
    let request: Request = try_request(body, try_content_type());
    request.try_get_body_form_data().expect("multipart body")
}

#[test]
fn test_text_field_of_a_multipart_body_is_read() {
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "abc123");
    body.extend(try_close(try_boundary()));
    let parts: Vec<BodyPart> = try_parts(body);
    assert_eq!(
        MultipartFormData::try_get_part_text(&parts, "file_id").as_deref(),
        Some("abc123")
    );
}

#[test]
fn test_file_name_of_a_part_is_read_verbatim() {
    let mut body: Vec<u8> = try_file_part(try_boundary(), "报告 v2%.txt", "text/plain", b"data");
    body.extend(try_close(try_boundary()));
    let parts: Vec<BodyPart> = try_parts(body);
    let part: &BodyPart = MultipartFormData::try_get_part(&parts, "file").expect("file part");
    assert_eq!(
        part.try_get_file_name().map(|name: &String| name.as_str()),
        Some("报告 v2%.txt")
    );
    assert_eq!(
        part.try_get_content_type()
            .map(|value: &String| value.as_str()),
        Some("text/plain")
    );
}

#[test]
fn test_binary_content_of_a_part_survives_the_roundtrip() {
    let payload: Vec<u8> = vec![0, 1, 2, 255, 254, 10, 13, 0];
    let mut body: Vec<u8> = try_file_part(
        try_boundary(),
        "data.bin",
        "application/octet-stream",
        &payload,
    );
    body.extend(try_close(try_boundary()));
    let parts: Vec<BodyPart> = try_parts(body);
    let part: &BodyPart = MultipartFormData::try_get_part(&parts, "file").expect("file part");
    assert_eq!(part.try_get_data(), Some(payload.as_slice()));
}

#[test]
fn test_a_body_without_a_boundary_yields_no_parts() {
    let request: Request = try_request(
        b"not a multipart body".to_vec(),
        "multipart/form-data".to_string(),
    );
    assert!(request.try_get_body_form_data().is_none());
}

#[test]
fn test_a_body_that_is_not_multipart_yields_no_parts() {
    let request: Request = try_request(b"not a multipart body at all".to_vec(), try_content_type());
    assert!(request.try_get_body_form_data().is_none());
}

#[test]
fn test_a_file_part_is_absent_when_the_body_carries_none() {
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "abc123");
    body.extend(try_close(try_boundary()));
    let parts: Vec<BodyPart> = try_parts(body);
    assert!(MultipartFormData::try_get_part(&parts, "file").is_none());
}

#[test]
fn test_the_file_name_of_a_metadata_only_part_is_read() {
    let mut body: Vec<u8> =
        try_file_part(try_boundary(), "empty.bin", "application/octet-stream", b"");
    body.extend(try_close(try_boundary()));
    let parts: Vec<BodyPart> = try_parts(body);
    let part: &BodyPart = MultipartFormData::try_get_part(&parts, "file").expect("file descriptor");
    assert_eq!(
        part.try_get_file_name().map(String::as_str),
        Some("empty.bin")
    );
    assert!(part.try_get_data().is_none());
}

#[tokio::test]
async fn test_registration_rejects_missing_metadata_fields() {
    for missing in ["file_id", "total_chunks", "file"] {
        let mut body: Vec<u8> = Vec::new();
        if missing != "file_id" {
            body.extend(try_text_part(
                try_boundary(),
                "file_id",
                "missing-field-test",
            ));
        }
        if missing != "total_chunks" {
            body.extend(try_text_part(try_boundary(), "total_chunks", "1"));
        }
        if missing != "file" {
            body.extend(try_file_part(
                try_boundary(),
                "valid.bin",
                "application/octet-stream",
                b"",
            ));
        }
        body.extend(try_close(try_boundary()));
        let mut ctx: Context = try_context(body, try_content_type());
        assert!(
            UploadService::get_register_file_chunk_data(&mut ctx)
                .await
                .is_none()
        );
        let expected: &str = match missing {
            "file_id" => "Missing file id",
            "total_chunks" => "Missing total chunks",
            _ => "Missing file name",
        };
        assert_eq!(try_response(&ctx)["msg"], expected);
    }
}

#[tokio::test]
async fn test_registration_accepts_valid_chunk_counts() {
    for count in [1, 2, 3] {
        let mut ctx: Context = try_context(
            try_registration_body("valid-count", &count.to_string(), "valid.bin"),
            try_content_type(),
        );
        let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
            .await
            .expect("metadata");
        assert_eq!(data.get_total_chunks(), count);
    }
}

#[tokio::test]
async fn test_invalid_directories_use_a_safe_generated_directory() {
    for directory in ["../escape", "/2026/10/25", "2026//10", ""] {
        let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "safe-directory");
        body.extend(try_text_part(try_boundary(), "total_chunks", "1"));
        body.extend(try_text_part(try_boundary(), "directory_path", directory));
        body.extend(try_file_part(
            try_boundary(),
            "valid.bin",
            "application/octet-stream",
            b"",
        ));
        body.extend(try_close(try_boundary()));
        let mut ctx: Context = try_context(body, try_content_type());
        let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
            .await
            .expect("metadata");
        let path: &str = data.get_base_file_dir();
        assert!(!path.starts_with('/'));
        assert!(!path.contains(".."));
        assert_eq!(path.split('/').count(), 5);
        assert!(
            path.chars()
                .all(|value: char| value.is_ascii_digit() || value == '/')
        );
    }
}

#[tokio::test]
async fn test_missing_chunk_index_is_rejected_by_the_production_extractor() {
    let mut ctx: Context = try_context(
        try_registration_body("missing-index", "1", "data.bin"),
        try_content_type(),
    );
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("metadata");
    UploadService::add_file_id_map(&data).await;
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "missing-index");
    body.extend(try_file_part(
        try_boundary(),
        "data.bin",
        "application/octet-stream",
        b"data",
    ));
    body.extend(try_close(try_boundary()));
    let mut ctx: Context = try_context(body, try_content_type());
    assert!(
        UploadService::get_save_file_chunk_data(&mut ctx)
            .await
            .is_none()
    );
    assert_eq!(try_response(&ctx)["msg"], "Missing chunk index");
    UploadService::remove_file_id_map("missing-index").await;
}

#[tokio::test]
async fn test_a_register_request_reads_its_fields_and_its_file_name() {
    let mut ctx: Context = try_context(
        try_registration_body("session-1", "2", "报告 v2%.txt"),
        try_content_type(),
    );
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("registered metadata");
    assert_eq!(data.get_file_id(), "session-1");
    assert_eq!(data.get_total_chunks(), 2);
    assert_eq!(data.get_file_name(), "报告 v2%.txt");
    assert_eq!(data.get_base_file_dir(), "2026/10/25");
}

#[test]
fn test_a_file_chunk_data_carries_the_read_fields() {
    let mut data: FileChunkData = FileChunkData::default();
    data.set_file_id("session-1".to_string())
        .set_file_name("报告 v2%.txt".to_string())
        .set_chunk_index(1)
        .set_total_chunks(2)
        .set_base_file_dir("2026/10/25".to_string());
    assert_eq!(data.get_file_id(), "session-1");
    assert_eq!(data.get_file_name(), "报告 v2%.txt");
    assert_eq!(data.get_chunk_index(), 1);
    assert_eq!(data.get_total_chunks(), 2);
    assert_eq!(data.get_base_file_dir(), "2026/10/25");
}

fn try_registration_body(file_id: &str, total_chunks: &str, filename: &str) -> Vec<u8> {
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", file_id);
    body.extend(try_text_part(try_boundary(), "total_chunks", total_chunks));
    body.extend(try_text_part(
        try_boundary(),
        "directory_path",
        "2026/10/25",
    ));
    body.extend(try_file_part(
        try_boundary(),
        filename,
        "application/octet-stream",
        b"",
    ));
    body.extend(try_close(try_boundary()));
    body
}

fn try_chunk_body(file_id: &str, index: &str, filename: &str, payload: &[u8]) -> Vec<u8> {
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", file_id);
    body.extend(try_text_part(try_boundary(), "chunk_index", index));
    body.extend(try_file_part(
        try_boundary(),
        filename,
        "application/octet-stream",
        payload,
    ));
    body.extend(try_close(try_boundary()));
    body
}

fn try_response(ctx: &Context) -> Value {
    from_slice(ctx.get_response().get_body()).expect("nonempty upload JSON response")
}

#[tokio::test]
async fn test_empty_and_unsafe_file_identifiers_are_rejected() {
    for id in ["", "../escape", "/absolute", "nested/path", "back\\slash"] {
        let mut ctx: Context = try_context(
            try_registration_body(id, "1", "data.bin"),
            try_content_type(),
        );
        assert!(
            UploadService::get_register_file_chunk_data(&mut ctx)
                .await
                .is_none()
        );
        assert_eq!(try_response(&ctx)["msg"], "Invalid file id");
    }
}

#[tokio::test]
async fn test_empty_and_unsafe_file_names_are_rejected() {
    for name in [
        "",
        ".",
        "..",
        "../escape",
        "/absolute",
        "nested/path",
        "back\\slash",
        "nul\0name",
    ] {
        let mut ctx: Context = try_context(
            try_registration_body("unsafe-filename", "1", name),
            try_content_type(),
        );
        assert!(
            UploadService::get_register_file_chunk_data(&mut ctx)
                .await
                .is_none(),
            "{name:?}"
        );
        assert_ne!(try_response(&ctx)["code"], 200);
    }
}

#[tokio::test]
async fn test_a_plain_text_field_cannot_impersonate_the_file_part() {
    let mut ctx: Context = try_context(
        try_registration_body("plain-file", "1", "data.bin"),
        try_content_type(),
    );
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("metadata");
    UploadService::add_file_id_map(&data).await;
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "plain-file");
    body.extend(try_text_part(try_boundary(), "chunk_index", "0"));
    body.extend(try_text_part(try_boundary(), "file", "not a file upload"));
    body.extend(try_close(try_boundary()));
    let mut ctx: Context = try_context(body, try_content_type());
    assert!(
        UploadService::get_save_file_chunk_data(&mut ctx)
            .await
            .is_none()
    );
    assert_eq!(try_response(&ctx)["msg"], "Missing file");
    UploadService::remove_file_id_map("plain-file").await;
}

#[tokio::test]
async fn test_malformed_requests_return_error_json_for_every_upload_stage() {
    for content_type in [
        "multipart/form-data",
        "application/octet-stream",
        "text/plain; boundary=wrong",
    ] {
        for stage in 0..3 {
            let mut ctx: Context = try_context(b"not multipart".to_vec(), content_type.to_string());
            match stage {
                0 => assert!(
                    UploadService::get_register_file_chunk_data(&mut ctx)
                        .await
                        .is_none()
                ),
                1 => assert!(
                    UploadService::get_save_file_chunk_data(&mut ctx)
                        .await
                        .is_none()
                ),
                _ => assert!(UploadService::get_merge_file_id(&mut ctx).await.is_none()),
            }
            let response: Value = try_response(&ctx);
            assert_eq!(response["msg"], "Invalid multipart form data");
            assert_ne!(response["code"], 200);
        }
    }
}

#[tokio::test]
async fn test_production_registration_reads_file_metadata_without_custom_headers() {
    let mut ctx: Context = try_context(
        try_registration_body("real-register", "2", "报告%41 v2%.bin"),
        try_content_type(),
    );
    assert_eq!(ctx.get_request().get_headers().len(), 1);
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("registered metadata");
    assert_eq!(data.get_file_id(), "real-register");
    assert_eq!(data.get_total_chunks(), 2);
    assert_eq!(data.get_file_name(), "报告%41 v2%.bin");
    assert_eq!(data.get_base_file_dir(), "2026/10/25");
}

#[tokio::test]
async fn test_registration_rejects_invalid_counts_and_missing_file_metadata() {
    for count in ["0", "-1", "many", ""] {
        let mut ctx: Context = try_context(
            try_registration_body("invalid-count", count, "valid.bin"),
            try_content_type(),
        );
        assert!(
            UploadService::get_register_file_chunk_data(&mut ctx)
                .await
                .is_none()
        );
        assert_eq!(try_response(&ctx)["msg"], "Invalid total chunks");
    }
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "missing-file");
    body.extend(try_text_part(try_boundary(), "total_chunks", "1"));
    body.extend(try_close(try_boundary()));
    let mut ctx: Context = try_context(body, try_content_type());
    assert!(
        UploadService::get_register_file_chunk_data(&mut ctx)
            .await
            .is_none()
    );
    assert_eq!(try_response(&ctx)["msg"], "Missing file name");
}

#[tokio::test]
async fn test_chunk_extraction_rejects_unknown_sessions() {
    let mut ctx: Context = try_context(
        try_chunk_body("unknown-session-test", "0", "data.bin", b"data"),
        try_content_type(),
    );
    assert!(
        UploadService::get_save_file_chunk_data(&mut ctx)
            .await
            .is_none()
    );
    assert_eq!(try_response(&ctx)["msg"], "Upload session not registered");
}

#[tokio::test]
async fn test_chunk_extraction_rejects_out_of_range_indices_and_missing_files() {
    let mut ctx: Context = try_context(
        try_registration_body("chunk-boundary-test", "1", "data.bin"),
        try_content_type(),
    );
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("metadata");
    UploadService::add_file_id_map(&data).await;
    for index in ["1", "999", "-1", "bad"] {
        let mut ctx: Context = try_context(
            try_chunk_body("chunk-boundary-test", index, "data.bin", b"data"),
            try_content_type(),
        );
        assert!(
            UploadService::get_save_file_chunk_data(&mut ctx)
                .await
                .is_none()
        );
        assert_eq!(try_response(&ctx)["msg"], "Invalid chunk index");
    }
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", "chunk-boundary-test");
    body.extend(try_text_part(try_boundary(), "chunk_index", "0"));
    body.extend(try_close(try_boundary()));
    let mut ctx: Context = try_context(body, try_content_type());
    assert!(
        UploadService::get_save_file_chunk_data(&mut ctx)
            .await
            .is_none()
    );
    assert_eq!(try_response(&ctx)["msg"], "Missing file");
    UploadService::remove_file_id_map("chunk-boundary-test").await;
}

#[tokio::test]
async fn test_empty_chunk_bytes_are_rejected_before_disk_io() {
    let data: FileChunkData = FileChunkData::default();
    assert!(
        UploadService::save_file_chunk(&data, Vec::new())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_multipart_chunks_merge_to_identical_bytes_on_disk() {
    if var_os("HQS_UPLOAD_ROUNDTRIP_CHILD").is_none() {
        let directory: TempDir = tempdir().expect("owned test directory");
        let output: Output = Command::new(current_exe().expect("test executable"))
            .args([
                "--exact",
                "upload::r#fn::test_multipart_chunks_merge_to_identical_bytes_on_disk",
                "--nocapture",
            ])
            .current_dir(directory.path())
            .env("HQS_UPLOAD_ROUNDTRIP_CHILD", "1")
            .output()
            .expect("isolated disk roundtrip process");
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout: String = String::from_utf8_lossy(&output.stdout).into_owned();
        assert!(stdout.contains("running 1 test"), "{stdout}");
        assert!(stdout.contains("1 passed; 0 failed"), "{stdout}");
        return;
    }
    let id: &str = "binary-disk-roundtrip";
    let filename: &str = "报告%41 v2%.bin";
    let payload: Vec<u8> = (0..(512 * 1024 + 137))
        .map(|index: usize| (index % 256) as u8)
        .collect();
    let mut ctx: Context =
        try_context(try_registration_body(id, "2", filename), try_content_type());
    let data: FileChunkData = UploadService::get_register_file_chunk_data(&mut ctx)
        .await
        .expect("metadata");
    UploadService::add_file_id_map(&data).await;
    assert!(UploadService::merge_file_chunks(&data).await.is_err());
    for (index, chunk) in payload.chunks(512 * 1024).enumerate() {
        let mut ctx: Context = try_context(
            try_chunk_body(id, &index.to_string(), filename, chunk),
            try_content_type(),
        );
        let (metadata, bytes): (FileChunkData, Vec<u8>) =
            UploadService::get_save_file_chunk_data(&mut ctx)
                .await
                .expect("file part and metadata");
        assert_eq!(bytes, chunk);
        assert_eq!(metadata.get_chunk_index(), index);
        UploadService::save_file_chunk(&metadata, bytes)
            .await
            .expect("chunk saved");
    }
    let mut body: Vec<u8> = try_text_part(try_boundary(), "file_id", id);
    body.extend(try_close(try_boundary()));
    let mut ctx: Context = try_context(body, try_content_type());
    let merge_id: String = UploadService::get_merge_file_id(&mut ctx)
        .await
        .expect("merge field");
    let data: FileChunkData = UploadService::get_merge_file_chunk_data(merge_id)
        .await
        .expect("registered session");
    let (directory, url): (String, String) = UploadService::merge_file_chunks(&data)
        .await
        .expect("merged file");
    assert_eq!(
        read(Path::new(&directory).join(filename)).expect("read merged file"),
        payload
    );
    assert!(!url.is_empty());
    let segments: Vec<&str> = url.rsplit('/').collect();
    let (served, content_type): (Vec<u8>, String) =
        UploadService::serve_static_file(segments[1], segments[0])
            .await
            .expect("file served");
    assert_eq!(served, payload);
    assert!(!content_type.is_empty());
    assert!(
        UploadService::get_merge_file_chunk_data(id.to_string())
            .await
            .is_none()
    );
}
