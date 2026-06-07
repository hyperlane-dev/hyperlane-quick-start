/// SQL statement for creating the chat history table in Postgresql.
pub const POSTGRESQL_CHAT_HISTORY_TABLE_SQL: &str =
    include_str!("./postgresql/chat/history_table.sql");

/// SQL statement for creating indexes on the chat history table in Postgresql.
pub const POSTGRESQL_CHAT_HISTORY_INDEX_SQL: &str = include_str!("./postgresql/chat/index.sql");

/// SQL statement for creating the tracking record table in Postgresql.
pub const POSTGRESQL_TRACKING_RECORD_TABLE_SQL: &str =
    include_str!("./postgresql/tracking/record_table.sql");

/// SQL statement for creating indexes on the tracking record table in Postgresql.
pub const POSTGRESQL_TRACKING_RECORD_INDEX_SQL: &str =
    include_str!("./postgresql/tracking/index.sql");

/// SQL statement for creating the shortlink table in Postgresql.
pub const POSTGRESQL_SHORTLINK_TABLE_SQL: &str =
    include_str!("./postgresql/shortlink/shortlink_table.sql");

/// SQL statement for creating indexes on the shortlink table in Postgresql.
pub const POSTGRESQL_SHORTLINK_INDEX_SQL: &str = include_str!("./postgresql/shortlink/index.sql");

/// SQL statement for creating the auth user table in Postgresql.
pub const POSTGRESQL_AUTH_USER_TABLE_SQL: &str = include_str!("./postgresql/auth/user_table.sql");

/// SQL statement for creating indexes on the auth table in Postgresql.
pub const POSTGRESQL_AUTH_INDEX_SQL: &str = include_str!("./postgresql/auth/index.sql");

/// SQL statement for inserting initial data into the auth table in Postgresql.
pub const POSTGRESQL_AUTH_DATA_SQL: &str = include_str!("./postgresql/auth/data.sql");

/// SQL statement for creating the order record table in Postgresql.
pub const POSTGRESQL_ORDER_RECORD_TABLE_SQL: &str =
    include_str!("./postgresql/order/record_table.sql");

/// SQL statement for creating the order record image table in Postgresql.
pub const POSTGRESQL_ORDER_RECORD_IMAGE_TABLE_SQL: &str =
    include_str!("./postgresql/order/record_image_table.sql");

/// SQL statement for creating indexes on the order table in Postgresql.
pub const POSTGRESQL_ORDER_INDEX_SQL: &str = include_str!("./postgresql/order/index.sql");

/// SQL statement for creating the notification table in Postgresql.
pub const POSTGRESQL_NOTIFICATION_TABLE_SQL: &str =
    include_str!("./postgresql/notification/notification_table.sql");

/// SQL statement for creating indexes on the notification table in Postgresql.
pub const POSTGRESQL_NOTIFICATION_INDEX_SQL: &str =
    include_str!("./postgresql/notification/index.sql");

/// SQL statement for creating the blog post table in Postgresql.
pub const POSTGRESQL_BLOG_POST_TABLE_SQL: &str = include_str!("./postgresql/blog/post_table.sql");

/// SQL statement for creating the blog comment table in Postgresql.
pub const POSTGRESQL_BLOG_COMMENT_TABLE_SQL: &str =
    include_str!("./postgresql/blog/comment_table.sql");

/// SQL statement for creating the blog like table in Postgresql.
pub const POSTGRESQL_BLOG_LIKE_TABLE_SQL: &str = include_str!("./postgresql/blog/like_table.sql");

/// SQL statement for creating the blog favorite table in Postgresql.
pub const POSTGRESQL_BLOG_FAVORITE_TABLE_SQL: &str =
    include_str!("./postgresql/blog/favorite_table.sql");

/// SQL statement for creating the blog image table in Postgresql.
pub const POSTGRESQL_BLOG_IMAGE_TABLE_SQL: &str = include_str!("./postgresql/blog/image_table.sql");

/// SQL statement for creating indexes on the blog table in Postgresql.
pub const POSTGRESQL_BLOG_INDEX_SQL: &str = include_str!("./postgresql/blog/index.sql");

/// SQL statement for creating the cicd pipeline table in Mysql.
pub const MYSQL_CICD_PIPELINE_TABLE_SQL: &str = include_str!("./mysql/cicd/pipeline_table.sql");

/// SQL statement for creating the cicd run table in Mysql.
pub const MYSQL_CICD_RUN_TABLE_SQL: &str = include_str!("./mysql/cicd/run_table.sql");

/// SQL statement for creating the cicd job table in Mysql.
pub const MYSQL_CICD_JOB_TABLE_SQL: &str = include_str!("./mysql/cicd/job_table.sql");

/// SQL statement for creating the cicd step table in Mysql.
pub const MYSQL_CICD_STEP_TABLE_SQL: &str = include_str!("./mysql/cicd/step_table.sql");
