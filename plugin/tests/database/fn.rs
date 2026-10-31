use super::*;

fn table(name: &str, dependencies: Vec<&str>) -> TableSchema {
    TableSchema::new(
        dependencies
            .iter()
            .map(|dependency: &&str| String::from(*dependency))
            .collect(),
        String::from(name),
        format!("CREATE TABLE {name} (id INT)"),
    )
}

fn ordered_names(schema: &DatabaseSchema) -> Vec<String> {
    schema
        .ordered_tables()
        .iter()
        .map(|ordered: &&TableSchema| ordered.get_name().clone())
        .collect()
}

#[test]
fn ordered_tables_of_an_empty_schema_is_empty() {
    let schema: DatabaseSchema = DatabaseSchema::default();
    assert!(schema.ordered_tables().is_empty());
}

#[test]
fn ordered_tables_keeps_declaration_order_when_nothing_depends() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("a", vec![]))
        .add_table(table("b", vec![]))
        .add_table(table("c", vec![]));
    assert_eq!(
        ordered_names(&schema),
        vec![String::from("a"), String::from("b"), String::from("c")]
    );
}

#[test]
fn ordered_tables_puts_a_dependent_after_its_dependency() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("sessions", vec!["users"]))
        .add_table(table("users", vec![]));
    assert_eq!(
        ordered_names(&schema),
        vec![String::from("users"), String::from("sessions")]
    );
}

#[test]
fn ordered_tables_resolves_a_transitive_chain() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("audit", vec!["sessions"]))
        .add_table(table("sessions", vec!["users"]))
        .add_table(table("users", vec![]));
    assert_eq!(
        ordered_names(&schema),
        vec![
            String::from("users"),
            String::from("sessions"),
            String::from("audit")
        ]
    );
}

#[test]
fn ordered_tables_orders_a_diamond_by_layer() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("reports", vec!["orders", "users"]))
        .add_table(table("orders", vec!["users"]))
        .add_table(table("users", vec![]));
    let names: Vec<String> = ordered_names(&schema);
    let users_at: usize = names
        .iter()
        .position(|name: &String| name.as_str() == "users")
        .expect("users present");
    let orders_at: usize = names
        .iter()
        .position(|name: &String| name.as_str() == "orders")
        .expect("orders present");
    let reports_at: usize = names
        .iter()
        .position(|name: &String| name.as_str() == "reports")
        .expect("reports present");
    assert!(
        users_at < orders_at && orders_at < reports_at,
        "diamond layers must stay ordered: {names:?}"
    );
}

#[test]
fn ordered_tables_never_repeats_a_table() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("users", vec![]))
        .add_table(table("sessions", vec!["users", "users"]))
        .add_table(table("audit", vec!["sessions"]));
    let names: Vec<String> = ordered_names(&schema);
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(names.len(), 3, "every table appears once: {names:?}");
    assert_eq!(unique.len(), 3, "no duplicate emission: {names:?}");
}

#[test]
fn ordered_tables_emits_every_table_of_a_cycle() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("a", vec!["b"]))
        .add_table(table("b", vec!["a"]));
    let names: Vec<String> = ordered_names(&schema);
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(names.len(), 2, "a cycle must not lose a table: {names:?}");
    assert_eq!(
        unique.len(),
        2,
        "a cycle must not duplicate a table: {names:?}"
    );
}

#[test]
fn ordered_tables_appends_a_three_node_cycle() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("a", vec!["c"]))
        .add_table(table("b", vec!["a"]))
        .add_table(table("c", vec!["b"]));
    let names: Vec<String> = ordered_names(&schema);
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(names.len(), 3, "every cyclic table survives: {names:?}");
    assert_eq!(
        unique.len(),
        3,
        "every cyclic table appears once: {names:?}"
    );
}

#[test]
fn ordered_tables_keeps_a_table_with_an_unknown_dependency() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("orphan", vec!["not_declared"]))
        .add_table(table("users", vec![]));
    let names: Vec<String> = ordered_names(&schema);
    assert_eq!(
        names.len(),
        2,
        "an unresolvable dependency is not dropped: {names:?}"
    );
    assert_eq!(
        names.last().map(|last: &String| last.as_str()),
        Some("orphan"),
        "an unresolvable dependency falls back to append order: {names:?}"
    );
}

#[test]
fn ordered_tables_keeps_a_lone_table_whose_dependency_is_unknown() {
    let schema: DatabaseSchema =
        DatabaseSchema::default().add_table(table("orphan", vec!["not_declared"]));
    assert_eq!(ordered_names(&schema), vec![String::from("orphan")]);
}

#[test]
fn ordered_tables_leaves_the_schema_untouched() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_table(table("sessions", vec!["users"]))
        .add_table(table("users", vec![]));
    let first: Vec<String> = ordered_names(&schema);
    let second: Vec<String> = ordered_names(&schema);
    assert_eq!(first, second);
    assert_eq!(schema.get_tables().len(), 2);
}

#[test]
fn add_index_constraint_and_init_data_accumulate() {
    let schema: DatabaseSchema = DatabaseSchema::default()
        .add_index(String::from("CREATE INDEX idx_users ON users (id)"))
        .add_index(String::from("CREATE INDEX idx_sessions ON sessions (id)"))
        .add_constraint(String::from(
            "ALTER TABLE users ADD CONSTRAINT uq UNIQUE (id)",
        ))
        .add_init_data(String::from("INSERT INTO users VALUES (1)"));
    assert_eq!(schema.get_indexes().len(), 2);
    assert_eq!(schema.get_constraints().len(), 1);
    assert_eq!(schema.get_init_data().len(), 1);
    assert!(
        schema
            .get_indexes()
            .iter()
            .any(|index: &String| index.as_str() == "CREATE INDEX idx_users ON users (id)")
    );
}

#[test]
fn with_dependency_appends_to_the_dependency_list() {
    let schema: TableSchema = table("audit", vec![]).with_dependency(String::from("users"));
    assert_eq!(schema.get_dependencies(), &vec![String::from("users")]);
}

#[test]
fn table_schema_keeps_the_name_and_sql_it_was_built_with() {
    let built: TableSchema = TableSchema::new(
        vec![String::from("users")],
        String::from("sessions"),
        String::from("CREATE TABLE sessions (id INT)"),
    );
    assert_eq!(built.get_name().as_str(), "sessions");
    assert_eq!(built.get_sql().as_str(), "CREATE TABLE sessions (id INT)");
    assert_eq!(built.get_dependencies(), &vec![String::from("users")]);
}

#[test]
fn a_fresh_result_reports_no_change_and_no_error() {
    let result: AutoCreationResult = AutoCreationResult::default();
    assert!(!result.has_changes());
    assert!(!result.has_errors());
}

#[test]
fn creating_the_database_counts_as_a_change() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_database_created(true);
    assert!(result.has_changes());
    assert!(!result.has_errors());
}

#[test]
fn creating_a_table_counts_as_a_change() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_tables_created(vec![String::from("users"), String::from("sessions")]);
    assert!(result.has_changes());
    assert_eq!(result.get_tables_created().len(), 2);
}

#[test]
fn errors_alone_are_not_a_change() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_errors(vec![
        String::from("table users already exists"),
        String::from("index missing"),
    ]);
    assert!(!result.has_changes());
    assert!(result.has_errors());
    assert_eq!(result.get_errors().len(), 2);
}

#[test]
fn clearing_the_errors_clears_the_error_flag() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_errors(vec![String::from("table users already exists")]);
    result.set_errors(vec![]);
    assert!(!result.has_errors());
    assert!(!result.has_changes());
}

#[test]
fn a_failure_with_a_created_table_is_both_change_and_error() {
    let mut result: AutoCreationResult = AutoCreationResult::default();
    result.set_tables_created(vec![String::from("users")]);
    result.set_errors(vec![String::from("table sessions already exists")]);
    assert!(result.has_changes());
    assert!(result.has_errors());
}

#[test]
fn a_fresh_failure_cache_is_not_expired_and_not_retryable() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert!(!cache.is_expired(Duration::from_secs(3600)));
    assert!(!cache.should_retry(Duration::from_secs(3600)));
}

#[test]
fn a_zero_threshold_expires_a_failure_cache() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert!(cache.is_expired(Duration::from_secs(0)));
    assert!(cache.should_retry(Duration::from_secs(0)));
}

#[test]
fn an_expired_failure_cache_reports_the_cached_error() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert_eq!(
        cache
            .try_get_result()
            .clone()
            .expect_err("a failure cache must not report a connection"),
        String::from("refused")
    );
}

#[test]
fn a_cached_success_is_never_expired_into_a_retry() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Ok(String::from("connected")));
    assert!(!cache.should_retry(Duration::from_secs(0)));
    assert_eq!(
        cache
            .try_get_result()
            .clone()
            .expect("the cached success must be readable"),
        String::from("connected")
    );
}

#[test]
fn a_success_cache_is_reported_as_a_connection_result() {
    let cache: ConnectionCache<u32> = ConnectionCache::new(Ok(7));
    match cache.try_get_result() {
        Ok(value) => assert_eq!(*value, 7),
        Err(error) => panic!("expected a cached success, got {error}"),
    }
}

#[test]
fn the_expiry_threshold_follows_elapsed_time() {
    let cache: ConnectionCache<String> = ConnectionCache::new(Err(String::from("refused")));
    assert!(!cache.is_expired(Duration::from_secs(3600)));
    sleep(Duration::from_millis(30));
    assert!(
        cache.is_expired(Duration::from_millis(5)),
        "an elapsed attempt must expire a shorter threshold"
    );
    assert!(
        !cache.is_expired(Duration::from_secs(3600)),
        "an elapsed attempt must not expire a longer threshold"
    );
}

#[test]
fn plugin_type_parses_every_display_name() {
    assert_eq!(
        PluginType::from_str("MySQL").expect("MySQL parses"),
        PluginType::MySQL
    );
    assert_eq!(
        PluginType::from_str("PostgreSQL").expect("PostgreSQL parses"),
        PluginType::PostgreSQL
    );
    assert_eq!(
        PluginType::from_str("Redis").expect("Redis parses"),
        PluginType::Redis
    );
}

#[test]
fn plugin_type_round_trips_through_display() {
    let all: Vec<PluginType> = vec![PluginType::MySQL, PluginType::PostgreSQL, PluginType::Redis];
    for plugin_type in all {
        let rendered: String = plugin_type.to_string();
        assert_eq!(
            PluginType::from_str(rendered.as_str()).expect("display name parses"),
            plugin_type,
            "display output {rendered} must parse back"
        );
    }
}

#[test]
fn plugin_type_rejects_a_name_that_is_not_exactly_the_display_name() {
    assert!(PluginType::from_str("mysql").is_err());
    assert!(PluginType::from_str("MYSQL").is_err());
    assert!(PluginType::from_str("postgresql").is_err());
    assert!(PluginType::from_str("Postgres").is_err());
    assert!(PluginType::from_str("Redis ").is_err());
    assert!(PluginType::from_str("").is_err());
}

#[test]
fn plugin_type_rejects_an_unknown_backend() {
    assert!(PluginType::from_str("mongodb").is_err());
    assert!(PluginType::from_str("sqlite").is_err());
}

#[test]
fn for_plugin_keeps_the_plugin_name() {
    let config: PluginAutoCreationConfig = AutoCreationConfig::for_plugin("PostgreSQL");
    assert_eq!(config.get_plugin_name().as_str(), "PostgreSQL");
}

#[test]
fn is_plugin_enabled_accepts_the_known_backends() {
    assert!(AutoCreationConfig::for_plugin("MySQL").is_plugin_enabled());
    assert!(AutoCreationConfig::for_plugin("PostgreSQL").is_plugin_enabled());
    assert!(AutoCreationConfig::for_plugin("Redis").is_plugin_enabled());
}

#[test]
fn is_plugin_enabled_rejects_an_unknown_backend() {
    assert!(!AutoCreationConfig::for_plugin("mongodb").is_plugin_enabled());
    assert!(!AutoCreationConfig::for_plugin("mysql").is_plugin_enabled());
    assert!(!AutoCreationConfig::for_plugin("").is_plugin_enabled());
}

#[test]
fn redis_reports_the_default_database_name() {
    let config: PluginAutoCreationConfig = AutoCreationConfig::for_plugin("Redis");
    assert_eq!(config.get_database_name(), String::from("default"));
}

#[test]
fn an_unconfigured_backend_reports_an_unknown_database_name() {
    assert_eq!(
        AutoCreationConfig::for_plugin("MySQL").get_database_name(),
        String::from("unknown")
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("PostgreSQL").get_database_name(),
        String::from("unknown")
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("mongodb").get_database_name(),
        String::from("unknown")
    );
}

#[test]
fn an_unconfigured_backend_reports_unknown_connection_info() {
    assert_eq!(
        AutoCreationConfig::for_plugin("MySQL").get_connection_info(),
        String::from("unknown")
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("PostgreSQL").get_connection_info(),
        String::from("unknown")
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("Redis").get_connection_info(),
        String::from("unknown")
    );
    assert_eq!(
        AutoCreationConfig::for_plugin("mongodb").get_connection_info(),
        String::from("unknown")
    );
}

#[test]
fn validate_reports_the_missing_mysql_instance_first() {
    assert_eq!(
        AutoCreationConfig::validate().expect_err("nothing is configured"),
        String::from("At least one MySQL instance is required")
    );
}

#[test]
fn a_connection_failure_aborts_auto_creation() {
    let error: AutoCreationError = AutoCreationError::ConnectionFailed(String::from("refused"));
    assert!(!error.should_continue());
}

#[test]
fn the_other_auto_creation_errors_keep_going() {
    let permission: AutoCreationError =
        AutoCreationError::InsufficientPermissions(String::from("denied"));
    let schema: AutoCreationError = AutoCreationError::SchemaError(String::from("bad ddl"));
    let timeout: AutoCreationError = AutoCreationError::Timeout(String::from("too slow"));
    let database: AutoCreationError = AutoCreationError::DatabaseError(String::from("unknown"));
    assert!(permission.should_continue());
    assert!(schema.should_continue());
    assert!(timeout.should_continue());
    assert!(database.should_continue());
}

#[test]
fn user_message_returns_the_payload_untouched() {
    let error: AutoCreationError =
        AutoCreationError::SchemaError(String::from("table users already exists"));
    assert_eq!(error.user_message(), "table users already exists");
}

#[test]
fn every_error_variant_labels_its_own_payload() {
    let permission: AutoCreationError =
        AutoCreationError::InsufficientPermissions(String::from("denied"));
    let connection: AutoCreationError =
        AutoCreationError::ConnectionFailed(String::from("refused"));
    let schema: AutoCreationError = AutoCreationError::SchemaError(String::from("bad ddl"));
    let timeout: AutoCreationError = AutoCreationError::Timeout(String::from("too slow"));
    let database: AutoCreationError = AutoCreationError::DatabaseError(String::from("unknown"));
    assert_eq!(
        permission.to_string(),
        String::from("Insufficient permissions denied")
    );
    assert_eq!(
        connection.to_string(),
        String::from("Connection failed refused")
    );
    assert_eq!(schema.to_string(), String::from("Schema error bad ddl"));
    assert_eq!(timeout.to_string(), String::from("Timeout too slow"));
    assert_eq!(database.to_string(), String::from("Database error unknown"));
}

#[test]
fn an_auto_creation_error_is_a_standard_error() {
    let error: Box<dyn Error> = Box::new(AutoCreationError::Timeout(String::from("too slow")));
    assert_eq!(error.to_string(), String::from("Timeout too slow"));
    assert!(Error::source(error.as_ref()).is_none());
}

#[test]
fn two_errors_with_the_same_payload_differ_by_variant() {
    let timeout: AutoCreationError = AutoCreationError::Timeout(String::from("too slow"));
    let database: AutoCreationError = AutoCreationError::DatabaseError(String::from("too slow"));
    assert_ne!(timeout.to_string(), database.to_string());
    assert_eq!(
        timeout.user_message(),
        database.user_message(),
        "the variant label lives in Display, user_message carries the bare payload"
    );
    assert_eq!(timeout.to_string(), String::from("Timeout too slow"));
    assert_eq!(
        database.to_string(),
        String::from("Database error too slow")
    );
}
