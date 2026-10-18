use {
    hyperlane_plugin::{
        database::{
            AutoCreationConfig, AutoCreationResult, ConnectionCache, DatabaseSchema,
            PluginAutoCreationConfig, PluginType, TableSchema,
        },
        logger::*,
    },
    std::{collections::HashSet, str::FromStr, time::Duration},
};

fn try_table(name: &str, dependencies: Vec<&str>) -> TableSchema {
    TableSchema::new(
        dependencies
            .iter()
            .map(|dep: &&str| String::from(*dep))
            .collect(),
        name.to_string(),
        format!("CREATE TABLE {name} (id INT);"),
    )
}

fn try_names(tables: Vec<&TableSchema>) -> Vec<String> {
    tables
        .iter()
        .map(|table: &&TableSchema| table.get_name().clone())
        .collect()
}

#[test]
fn test_ordered_tables_puts_ready_tables_first() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(try_table("users", vec![]))
        .add_table(try_table("sessions", vec!["users"]));
    let names: Vec<String> = try_names(schema.ordered_tables());
    assert_eq!(names, vec![String::from("users"), String::from("sessions")]);
}

#[test]
fn test_ordered_tables_orders_by_dependency_not_declaration() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(try_table("sessions", vec!["users"]))
        .add_table(try_table("users", vec![]));
    let names: Vec<String> = try_names(schema.ordered_tables());
    let users_at: usize = names
        .iter()
        .position(|name: &String| name.as_str() == "users")
        .expect("users present");
    let sessions_at: usize = names
        .iter()
        .position(|name: &String| name.as_str() == "sessions")
        .expect("sessions present");
    assert!(
        users_at < sessions_at,
        "dependency must be created first: {names:?}"
    );
}

#[test]
fn test_ordered_tables_sorts_a_transitive_chain() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(try_table("c", vec!["b"]))
        .add_table(try_table("a", vec![]))
        .add_table(try_table("b", vec!["a"]));
    assert_eq!(
        try_names(schema.ordered_tables()),
        vec![String::from("a"), String::from("b"), String::from("c")]
    );
}

#[test]
fn test_ordered_tables_survives_a_dependency_cycle() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(try_table("a", vec!["b"]))
        .add_table(try_table("b", vec!["a"]));
    let names: Vec<String> = try_names(schema.ordered_tables());
    assert_eq!(names.len(), 2, "a cycle must still return every table");
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(unique.len(), 2, "no table may be emitted twice: {names:?}");
}

#[test]
fn test_ordered_tables_appends_tables_with_unknown_dependency() {
    let schema: DatabaseSchema =
        DatabaseSchema::default().add_table(try_table("a", vec!["missing_table"]));
    assert_eq!(try_names(schema.ordered_tables()), vec![String::from("a")]);
}

#[test]
fn test_ordered_tables_of_empty_schema_is_empty() {
    assert!(DatabaseSchema::default().ordered_tables().is_empty());
}

#[test]
fn test_ordered_tables_returns_each_table_once() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(try_table("a", vec![]))
        .add_table(try_table("b", vec![]))
        .add_table(try_table("c", vec![]));
    let names: Vec<String> = try_names(schema.ordered_tables());
    assert_eq!(names.len(), 3);
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(unique.len(), 3, "duplicates in {names:?}");
}

#[test]
fn test_has_changes_is_false_for_an_untouched_result() {
    assert!(!AutoCreationResult::default().has_changes());
}

#[test]
fn test_has_changes_is_true_when_database_created() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_database_created(true);
    assert!(result.has_changes());
}

#[test]
fn test_has_changes_is_false_for_errors_only() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_errors(vec![String::from("permission denied")]);
    assert!(!result.has_changes());
    assert!(result.has_errors());
}

#[test]
fn test_has_errors_is_false_when_no_errors() {
    assert!(!AutoCreationResult::default().has_errors());
}

#[test]
fn test_is_plugin_enabled_accepts_known_names() {
    assert!(
        AutoCreationConfig::for_plugin("MySQL").is_plugin_enabled(),
        "MySQL must be enabled"
    );
    assert!(
        AutoCreationConfig::for_plugin("PostgreSQL").is_plugin_enabled(),
        "PostgreSQL must be enabled"
    );
    assert!(
        AutoCreationConfig::for_plugin("Redis").is_plugin_enabled(),
        "Redis must be enabled"
    );
}

#[test]
fn test_is_plugin_enabled_rejects_unknown_name() {
    assert!(!AutoCreationConfig::for_plugin("mongodb").is_plugin_enabled());
    assert!(!AutoCreationConfig::for_plugin("").is_plugin_enabled());
    assert!(!AutoCreationConfig::for_plugin("mysql").is_plugin_enabled());
}

#[test]
fn test_for_plugin_keeps_the_plugin_name() {
    let config: PluginAutoCreationConfig = AutoCreationConfig::for_plugin("Redis");
    assert_eq!(config.get_plugin_name().as_str(), "Redis");
}

#[test]
fn test_plugin_type_from_str_is_case_sensitive() {
    assert_eq!(
        PluginType::from_str("PostgreSQL").expect("exact name parses"),
        PluginType::PostgreSQL
    );
    assert!(PluginType::from_str("postgresql").is_err());
    assert!(PluginType::from_str("POSTGRESQL").is_err());
}

#[test]
fn test_plugin_type_parses_all_display_names() {
    assert_eq!(
        PluginType::from_str("MySQL").expect("mysql parses"),
        PluginType::MySQL
    );
    assert_eq!(
        PluginType::from_str("PostgreSQL").expect("postgresql parses"),
        PluginType::PostgreSQL
    );
    assert_eq!(
        PluginType::from_str("Redis").expect("redis parses"),
        PluginType::Redis
    );
}

#[test]
fn test_auto_creation_config_is_constructible() {
    let config: PluginAutoCreationConfig = AutoCreationConfig::for_plugin("MySQL");
    assert!(
        config.is_plugin_enabled(),
        "the built config must be usable"
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("Redis")
            .get_plugin_name()
            .as_str(),
        "Redis"
    );
}

#[test]
fn test_file_logger_zero_limit_is_disable() {
    let logger: FileLogger = FileLogger::new("./logs", 0);
    assert!(logger.is_disable());
    assert!(!logger.is_enable());
}

#[test]
fn test_file_logger_non_zero_limit_is_enable() {
    let logger: FileLogger = FileLogger::new("./logs", 1024);
    assert!(logger.is_enable());
    assert!(!logger.is_disable());
}

#[test]
fn test_file_logger_new_keeps_path_and_limit() {
    let logger: FileLogger = FileLogger::new("/var/log/app", 2048);
    assert_eq!(logger.get_path().as_str(), "/var/log/app");
    assert_eq!(logger.get_limit_file_size(), 2048);
    assert_eq!(logger.get_trace_dir().as_str(), "trace");
    assert_eq!(logger.get_error_dir().as_str(), "error");
}

#[test]
fn test_file_logger_default_uses_default_dir() {
    assert_eq!(FileLogger::default().get_path().as_str(), "./logs");
}

#[test]
fn test_fresh_connection_cache_is_not_expired() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert!(!cache.is_expired(Duration::from_secs(60)));
    assert!(!cache.should_retry(Duration::from_secs(60)));
}

#[test]
fn test_expired_failure_is_retryable() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert!(cache.is_expired(Duration::from_nanos(0)));
    assert!(cache.should_retry(Duration::from_nanos(0)));
}

#[test]
fn test_cached_success_is_never_retryable() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Ok(String::from("connected")));
    assert!(!cache.should_retry(Duration::from_nanos(0)));
    assert_eq!(
        cache
            .try_get_result()
            .clone()
            .expect("the cached success must be readable"),
        String::from("connected")
    );
}
