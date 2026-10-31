use super::*;

fn scratch_dir(label: &str) -> TempDir {
    TempDir::with_prefix(format!("hyperlane-plugin-logger-{label}-")).expect("scratch dir")
}

fn log_files_under(root: &Path) -> Vec<PathBuf> {
    let date_dirs: Vec<PathBuf> = read_dir(root)
        .expect("level dir exists")
        .filter_map(Result::ok)
        .map(|entry: DirEntry| entry.path())
        .collect();
    date_dirs
        .iter()
        .flat_map(|date_dir: &PathBuf| {
            read_dir(date_dir)
                .expect("date dir exists")
                .filter_map(Result::ok)
                .map(|entry: DirEntry| entry.path())
                .collect::<Vec<PathBuf>>()
        })
        .collect()
}

fn read_only_log(root: &Path) -> String {
    let files: Vec<PathBuf> = log_files_under(root);
    assert_eq!(files.len(), 1, "expected exactly one log file: {files:?}");
    read_to_string(files.first().expect("one log file")).expect("log file readable")
}

fn write(logger: &FileLogger, payload: &str) -> String {
    let chained: &FileLogger = logger.info(payload, |data: &str| data.to_string());
    let returned: String = chained.get_path().clone();
    returned
}

fn persisted_lines(root: &Path) -> Vec<String> {
    read_only_log(root).lines().map(String::from).collect()
}

fn file_lines(file: &Path) -> Vec<String> {
    read_to_string(file)
        .expect("log file readable")
        .lines()
        .map(String::from)
        .collect()
}

#[test]
fn a_zero_limit_disables_the_file_logger() {
    let logger: FileLogger = FileLogger::new("./logs", 0);
    assert!(logger.is_disable());
    assert!(!logger.is_enable());
}

#[test]
fn any_non_zero_limit_enables_the_file_logger() {
    assert!(FileLogger::new("./logs", 1).is_enable());
    assert!(FileLogger::new("./logs", 1024).is_enable());
    assert!(FileLogger::default().is_enable());
}

#[test]
fn is_enable_is_the_negation_of_is_disable() {
    let limits: Vec<usize> = vec![0, 1, 1024];
    for limit in limits {
        let logger: FileLogger = FileLogger::new("./logs", limit);
        assert_eq!(logger.is_enable(), !logger.is_disable());
    }
}

#[test]
fn new_keeps_the_path_and_the_limit() {
    let logger: FileLogger = FileLogger::new("/var/log/app", 2048);
    assert_eq!(logger.get_path().as_str(), "/var/log/app");
    assert_eq!(logger.get_limit_file_size(), 2048);
}

#[test]
fn new_defaults_every_level_subdirectory() {
    let logger: FileLogger = FileLogger::new("/var/log/app", 2048);
    assert_eq!(logger.get_trace_dir().as_str(), "trace");
    assert_eq!(logger.get_debug_dir().as_str(), "debug");
    assert_eq!(logger.get_info_dir().as_str(), "info");
    assert_eq!(logger.get_warn_dir().as_str(), "warn");
    assert_eq!(logger.get_error_dir().as_str(), "error");
}

#[test]
fn default_uses_the_default_directory_and_a_large_limit() {
    let logger: FileLogger = FileLogger::default();
    assert_eq!(logger.get_path().as_str(), "./logs");
    assert_eq!(logger.get_limit_file_size(), 1_024_000_000);
}

#[test]
fn a_disabled_logger_writes_nothing() {
    let guard: TempDir = scratch_dir("disabled");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 0);
    logger.info("never persisted", |data: &str| data.to_string());
    logger.error("never persisted either", |data: &str| data.to_string());
    assert!(!root.exists(), "a disabled logger must not create its root");
}

#[test]
fn a_disabled_logger_returns_itself_for_chaining() {
    let logger: FileLogger = FileLogger::new("/dev/null", 0);
    let chained: &FileLogger = logger.trace("x", |data: &str| data.to_string());
    assert!(chained.is_disable());
}

#[test]
fn each_level_writes_into_its_own_subdirectory() {
    let guard: TempDir = scratch_dir("levels");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    let identity: fn(&str) -> String = |data: &str| data.to_string();
    logger.trace("trace payload", identity);
    logger.debug("debug payload", identity);
    logger.info("info payload", identity);
    logger.warn("warn payload", identity);
    logger.error("error payload", identity);
    let trace_dir: PathBuf = root.join(logger.get_trace_dir());
    let debug_dir: PathBuf = root.join(logger.get_debug_dir());
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    let warn_dir: PathBuf = root.join(logger.get_warn_dir());
    let error_dir: PathBuf = root.join(logger.get_error_dir());
    assert!(trace_dir.is_dir(), "trace dir must be created");
    assert!(debug_dir.is_dir(), "debug dir must be created");
    assert!(info_dir.is_dir(), "info dir must be created");
    assert!(warn_dir.is_dir(), "warn dir must be created");
    assert!(error_dir.is_dir(), "error dir must be created");
    assert_eq!(
        persisted_lines(&trace_dir).last(),
        Some(&String::from("trace payload"))
    );
    assert_eq!(
        persisted_lines(&debug_dir).last(),
        Some(&String::from("debug payload"))
    );
    assert_eq!(
        persisted_lines(&info_dir).last(),
        Some(&String::from("info payload"))
    );
    assert_eq!(
        persisted_lines(&warn_dir).last(),
        Some(&String::from("warn payload"))
    );
    assert_eq!(
        persisted_lines(&error_dir).last(),
        Some(&String::from("error payload"))
    );
}

#[test]
fn writing_creates_the_missing_level_and_date_directories() {
    let guard: TempDir = scratch_dir("mkdir");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    assert!(!root.exists());
    write(&logger, "first line");
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    assert!(info_dir.is_dir());
    assert_eq!(
        read_dir(info_dir).expect("info dir").count(),
        1,
        "a dated subdirectory must be created under the level dir"
    );
}

#[test]
fn a_payload_is_persisted_verbatim() {
    let guard: TempDir = scratch_dir("verbatim");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    write(&logger, "the exact payload");
    assert_eq!(
        persisted_lines(&root.join(logger.get_info_dir())),
        vec![String::from("the exact payload")],
        "the identity formatter must reach the file as the only, untouched line"
    );
}

#[test]
fn the_formatter_output_is_what_reaches_the_file() {
    let guard: TempDir = scratch_dir("formatter");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    logger.info("payload", |data: &str| {
        let formatted: String = format!("[{data}]");
        formatted
    });
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    assert_eq!(
        persisted_lines(&info_dir),
        vec![String::from("[payload]")],
        "only the formatter output may reach the file"
    );
}

#[test]
fn the_level_call_returns_the_logger_for_chaining() {
    let guard: TempDir = scratch_dir("chaining");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    let chained: &FileLogger = logger.trace("chained", |data: &str| data.to_string());
    assert_eq!(chained.get_path(), logger.get_path());
    let deeper: &FileLogger = chained.error("chained again", |data: &str| data.to_string());
    assert_eq!(
        persisted_lines(&root.join(deeper.get_trace_dir())),
        vec![String::from("chained")]
    );
    assert_eq!(
        persisted_lines(&root.join(deeper.get_error_dir())),
        vec![String::from("chained again")]
    );
}

#[test]
fn a_multiline_payload_keeps_every_line() {
    let guard: TempDir = scratch_dir("multiline");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    write(&logger, "first\nsecond\nthird");
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    assert_eq!(
        persisted_lines(&info_dir),
        vec![
            String::from("first"),
            String::from("second"),
            String::from("third")
        ],
        "every line of a multiline payload must survive, in order"
    );
}

#[test]
fn repeated_writes_append_to_the_same_file_under_the_limit() {
    let guard: TempDir = scratch_dir("append");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    write(&logger, "first entry");
    write(&logger, "second entry");
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    let files: Vec<PathBuf> = log_files_under(&info_dir);
    assert_eq!(
        files.len(),
        1,
        "under the limit the log must not roll: {files:?}"
    );
    assert_eq!(
        persisted_lines(&info_dir),
        vec![String::from("first entrysecond entry")],
        "an identity formatter frames nothing, so both writes append verbatim into the single file"
    );
}

#[test]
fn a_write_beyond_the_limit_rolls_into_a_new_file() {
    let guard: TempDir = scratch_dir("rollover");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 1);
    write(&logger, "entry that is longer than the limit");
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    assert_eq!(
        log_files_under(&info_dir).len(),
        1,
        "the first write has no previous file to overflow"
    );
    write(&logger, "the next entry cannot fit");
    let files: Vec<PathBuf> = log_files_under(&info_dir);
    assert_eq!(
        files.len(),
        2,
        "an overflowing write must open a new file: {files:?}"
    );
    let mut contents: Vec<Vec<String>> = files
        .iter()
        .map(|file: &PathBuf| file_lines(file))
        .collect();
    contents.sort();
    assert_eq!(
        contents,
        vec![
            vec![String::from("entry that is longer than the limit")],
            vec![String::from("the next entry cannot fit")],
        ],
        "each overflowing file must hold exactly its own entry verbatim"
    );
}

#[test]
fn a_write_exactly_at_the_limit_still_appends() {
    let guard: TempDir = scratch_dir("atlimit");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 5);
    write(&logger, "12345");
    write(&logger, "67890");
    let info_dir: PathBuf = root.join(logger.get_info_dir());
    assert_eq!(
        log_files_under(&info_dir).len(),
        1,
        "a file of exactly the limit size is still writable"
    );
}

#[test]
fn a_trailing_slash_on_the_base_path_does_not_double_separate() {
    let guard: TempDir = scratch_dir("slash");
    let with_slash: PathBuf = guard.path().join("with-slash");
    let without_slash: PathBuf = guard.path().join("without-slash");
    let slashed: FileLogger = FileLogger::new(format!("{}/", with_slash.display()), 65_536);
    let plain: FileLogger = FileLogger::new(without_slash.to_string_lossy(), 65_536);
    write(&slashed, "slashed payload");
    write(&plain, "plain payload");
    assert_eq!(
        persisted_lines(&with_slash.join(slashed.get_info_dir())),
        vec![String::from("slashed payload")]
    );
    assert_eq!(
        persisted_lines(&without_slash.join(plain.get_info_dir())),
        vec![String::from("plain payload")]
    );
}

#[test]
fn two_loggers_with_different_roots_do_not_share_files() {
    let guard: TempDir = scratch_dir("root-pair");
    let first: PathBuf = guard.path().join("root-one");
    let second: PathBuf = guard.path().join("root-two");
    let first_logger: FileLogger = FileLogger::new(first.to_string_lossy(), 65_536);
    let second_logger: FileLogger = FileLogger::new(second.to_string_lossy(), 65_536);
    write(&first_logger, "belongs to the first root");
    write(&second_logger, "belongs to the second root");
    assert_eq!(
        persisted_lines(&first.join(first_logger.get_info_dir())),
        vec![String::from("belongs to the first root")],
        "the first root must hold only its own entry"
    );
    assert_eq!(
        persisted_lines(&second.join(second_logger.get_info_dir())),
        vec![String::from("belongs to the second root")],
        "the second root must hold only its own entry"
    );
}

#[test]
fn the_written_file_name_carries_the_index_and_the_extension() {
    let guard: TempDir = scratch_dir("filename");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 65_536);
    write(&logger, "named payload");
    let files: Vec<PathBuf> = log_files_under(&root.join(logger.get_info_dir()));
    let name: String = files
        .first()
        .expect("one log file")
        .file_name()
        .expect("file name")
        .to_string_lossy()
        .to_string();
    assert!(
        name.ends_with(".log"),
        "log file must carry the extension: {name}"
    );
    assert!(
        name.contains(".1."),
        "the first log file must be index 1: {name}"
    );
}

#[test]
fn a_write_after_rollover_keeps_its_index_in_the_name() {
    let guard: TempDir = scratch_dir("rollindex");
    let root: PathBuf = guard.path().join("root");
    let logger: FileLogger = FileLogger::new(root.to_string_lossy(), 1);
    write(&logger, "entry one is long");
    write(&logger, "entry two is long");
    let files: Vec<PathBuf> = log_files_under(&root.join(logger.get_info_dir()));
    let mut names: Vec<String> = files
        .iter()
        .map(|file: &PathBuf| {
            file.file_name()
                .expect("file name")
                .to_string_lossy()
                .to_string()
        })
        .collect();
    names.sort();
    assert_eq!(names.len(), 2, "two files must exist: {names:?}");
    assert!(
        names.iter().any(|name: &String| name.contains(".1.")),
        "the first file keeps index 1: {names:?}"
    );
    assert!(
        names.iter().any(|name: &String| name.contains(".2.")),
        "the overflow file takes index 2: {names:?}"
    );
}
