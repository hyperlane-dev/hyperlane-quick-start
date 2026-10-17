use super::*;

const RSS_CHILD_MARKER: &str = "HYPERLANE_APPLICATION_RSS_CHILD";
const RSS_FIXTURE_MTIME_BASE_SECS: u64 = 1_700_000_000;
const RSS_TEST_WITHOUT_DIRECTORY: &str =
    "rss::r#fn::rss_feed_without_an_upload_directory_is_still_well_formed_xml";
const RSS_TEST_PAGINATION: &str =
    "rss::r#fn::rss_feed_echoes_the_base_url_link_and_applies_pagination";
const RSS_TEST_PAST_THE_END: &str =
    "rss::r#fn::rss_feed_with_a_limit_and_offset_past_the_end_yields_no_items";
const RSS_TEST_CONSTANTS: &str =
    "rss::r#fn::rss_feed_title_and_description_are_the_configured_constants";
const RSS_TEST_MISSING_SCAN: &str =
    "rss::r#fn::uploaded_file_scan_returns_an_empty_list_when_the_upload_dir_is_missing";

fn rss_upload_dir(root: &Path) -> PathBuf {
    root.join(UPLOAD_DIR)
}

fn rss_seed_fixture(path: &Path, contents: &[u8], mtime_offset_secs: u64) {
    write(path, contents).expect("the fixture file is written");
    let file: File = File::options()
        .write(true)
        .open(path)
        .expect("the fixture file is reopened");
    let modified: SystemTime =
        UNIX_EPOCH + Duration::from_secs(RSS_FIXTURE_MTIME_BASE_SECS + mtime_offset_secs);
    file.set_times(FileTimes::new().set_modified(modified))
        .expect("the fixture modification time is pinned");
    assert!(
        metadata(path)
            .expect("the fixture metadata is readable")
            .is_file(),
        "the fixture file is on disk before its timestamp is read back"
    );
}

fn rss_seed_upload_dir(root: &Path) {
    let upload_dir: PathBuf = rss_upload_dir(root);
    create_dir_all(upload_dir.join("notes")).expect("the nested upload fixture directory");
    create_dir_all(upload_dir.join("assets")).expect("the sibling upload fixture directory");
    rss_seed_fixture(&upload_dir.join("notes").join("beta.txt"), b"beta", 0);
    rss_seed_fixture(&upload_dir.join("notes").join("gamma.json"), b"{}", 60);
    rss_seed_fixture(&upload_dir.join("assets").join("delta.md"), b"# delta", 120);
}

fn rss_isolated_child(test_name: &str, seed: bool) -> bool {
    if var(RSS_CHILD_MARKER).is_ok_and(|marker: String| marker == test_name) {
        return true;
    }
    let root: TempDir = tempdir().expect("an isolated upload root");
    if seed {
        rss_seed_upload_dir(root.path());
    }
    let output: Output = Command::new(current_exe().expect("this test binary"))
        .current_dir(root.path())
        .env(RSS_CHILD_MARKER, test_name)
        .args([test_name, "--exact", "--nocapture", "--test-threads=1"])
        .output()
        .expect("the isolated child test runs");
    let report: String = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        report.contains("1 passed; 0 failed") && output.status.success(),
        "the isolated child ran exactly one test: {report}"
    );
    false
}

#[tokio::test]
async fn rss_feed_without_an_upload_directory_is_still_well_formed_xml() {
    if !rss_isolated_child(RSS_TEST_WITHOUT_DIRECTORY, false) {
        return;
    }
    let feed: String = RssService::generate_rss_feed("https://example.com", None, None, None).await;
    assert!(feed.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(feed.contains("<rss version=\"2.0\">"));
    assert!(feed.contains("<title>Uploaded Resources Feed</title>"));
    assert!(feed.contains("<channel>"));
    assert!(feed.contains("<link>https://example.com</link>"));
    assert!(!feed.contains("<item>"));
    assert!(feed.contains("</channel>"));
    assert!(feed.contains("</rss>"));
}

#[tokio::test]
async fn rss_feed_echoes_the_base_url_link_and_applies_pagination() {
    if !rss_isolated_child(RSS_TEST_PAGINATION, true) {
        return;
    }
    let files: Vec<UploadedFile> = RssService::get_uploaded_files().await;
    assert_eq!(files.len(), 3);
    let names: Vec<String> = files
        .iter()
        .map(|file: &UploadedFile| file.get_file_name().to_string())
        .collect();
    assert!(names.contains(&"beta.txt".to_string()));
    assert!(names.contains(&"gamma.json".to_string()));
    assert!(names.contains(&"delta.md".to_string()));
    let nested_file: UploadedFile = files
        .iter()
        .find(|file: &&UploadedFile| file.get_file_name() == "beta.txt")
        .expect("the nested file is scanned")
        .clone();
    assert_eq!(nested_file.get_file_path(), "notes/beta.txt");
    assert_eq!(nested_file.get_file_size(), 4);
    assert!(nested_file.get_file_url().starts_with("/upload/file/"));
    assert_eq!(nested_file.get_file_url().matches('/').count(), 4);
    let json_file: UploadedFile = files
        .iter()
        .find(|file: &&UploadedFile| file.get_file_name() == "gamma.json")
        .expect("the second nested file is scanned")
        .clone();
    assert_eq!(json_file.get_file_path(), "notes/gamma.json");
    let json: &UploadedFile = &json_file;
    assert!(json.get_content_type().contains("json"));
    let sibling_file: UploadedFile = files
        .iter()
        .find(|file: &&UploadedFile| file.get_file_name() == "delta.md")
        .expect("the sibling directory file is scanned")
        .clone();
    assert_eq!(sibling_file.get_file_path(), "assets/delta.md");
    assert_ne!(nested_file.get_file_url(), sibling_file.get_file_url());
    assert_ne!(nested_file.get_file_url(), json_file.get_file_url());
    let full: String =
        RssService::generate_rss_feed("https://a.example", None, None, Some(Timezone::Utc)).await;
    assert_eq!(full.matches("<item>").count(), 3);
    assert!(full.contains("<link>https://a.example</link>"));
    assert!(full.contains("<title>beta.txt</title>"));
    assert!(full.contains("<title>gamma.json</title>"));
    assert!(full.contains("<title>delta.md</title>"));
    assert_eq!(full.matches("<language>en-US</language>").count(), 1);
    let first_two: String =
        RssService::generate_rss_feed("https://a.example", Some(2), Some(0), None).await;
    let last_two: String =
        RssService::generate_rss_feed("https://a.example", Some(2), Some(1), None).await;
    assert_eq!(first_two.matches("<item>").count(), 2);
    assert_eq!(last_two.matches("<item>").count(), 2);
    assert_ne!(first_two, last_two);
    assert!(first_two.contains("<link>https://a.example/upload/file/"));
    let mut windowed_names: Vec<String> = vec![];
    for offset in 0usize..3 {
        let single: String = RssService::generate_rss_feed(
            "https://b.example",
            Some(1),
            Some(offset),
            Some(Timezone::Pst),
        )
        .await;
        assert_eq!(single.matches("<item>").count(), 1);
        assert!(single.contains("<link>https://b.example</link>"));
        let item_title: String = single
            .split("<item>")
            .nth(1)
            .and_then(|item: &str| item.split("<title>").nth(1))
            .and_then(|rest: &str| rest.split("</title>").next())
            .expect("the item carries its own title after the channel title")
            .to_string();
        windowed_names.push(item_title);
    }
    assert_eq!(
        windowed_names,
        vec![
            "delta.md".to_string(),
            "gamma.json".to_string(),
            "beta.txt".to_string()
        ],
        "the pinned modification times order the windows newest first"
    );
    let shifted: String =
        RssService::generate_rss_feed("https://b.example", None, Some(1), None).await;
    assert_eq!(shifted.matches("<item>").count(), 2);
}

#[tokio::test]
async fn rss_feed_with_a_limit_and_offset_past_the_end_yields_no_items() {
    if !rss_isolated_child(RSS_TEST_PAST_THE_END, true) {
        return;
    }
    let feed: String =
        RssService::generate_rss_feed("https://example.com", Some(5), Some(usize::MAX), None).await;
    assert!(!feed.contains("<item>"));
    assert!(feed.contains("</channel>"));
    assert!(feed.contains("<link>https://example.com</link>"));
}

#[tokio::test]
async fn rss_feed_title_and_description_are_the_configured_constants() {
    if !rss_isolated_child(RSS_TEST_CONSTANTS, false) {
        return;
    }
    let feed: String = RssService::generate_rss_feed("https://example.com", None, None, None).await;
    assert!(feed.contains("<title>Uploaded Resources Feed</title>"));
    assert!(
        feed.contains("<description>Subscribe to the latest uploaded resource files</description>")
    );
    assert_eq!(feed.matches("<rss version=\"2.0\">").count(), 1);
}

#[tokio::test]
async fn uploaded_file_scan_returns_an_empty_list_when_the_upload_dir_is_missing() {
    if !rss_isolated_child(RSS_TEST_MISSING_SCAN, false) {
        return;
    }
    let working_dir: PathBuf = current_dir().expect("the isolated child working directory");
    let upload_dir: PathBuf = rss_upload_dir(&working_dir);
    let files: Vec<UploadedFile> = RssService::get_uploaded_files().await;
    assert!(files.is_empty());
    assert!(
        !upload_dir.exists(),
        "the isolated child starts without the upload directory"
    );
    create_dir_all(&upload_dir).expect("the real upload directory is created under the child cwd");
    assert!(
        upload_dir.is_dir(),
        "the upload directory the service scans now really exists"
    );
    let seeded_files: Vec<UploadedFile> = RssService::get_uploaded_files().await;
    assert!(
        seeded_files.is_empty(),
        "an existing but empty upload directory scans to no files"
    );
}
