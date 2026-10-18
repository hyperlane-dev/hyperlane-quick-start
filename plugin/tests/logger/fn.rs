use super::*;

#[test]
fn test_common_log_prefixes_and_terminates_each_line() {
    let logged: String = common_log("first\nsecond");
    let lines: Vec<&str> = logged.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].ends_with(" first"));
    assert!(lines[1].ends_with(" second"));
    assert!(logged.ends_with('\n'), "every line must be terminated");
}

#[test]
fn test_common_log_of_empty_is_empty() {
    assert!(common_log("").is_empty());
}

#[test]
fn test_log_handler_matches_common_log() {
    assert_eq!(log_handler("payload"), common_log("payload"));
}
