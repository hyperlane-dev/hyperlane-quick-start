/// MySQL table name variants used for schema identification and display.
pub enum MysqlTableName {
    /// The CICD pipeline table.
    CicdPipeline,
    /// The CICD run table.
    CicdRun,
    /// The CICD job table.
    CicdJob,
    /// The CICD step table.
    CicdStep,
}

/// PostgreSQL table name variants used for schema identification and display.
pub enum PostgresqlTableName {
    /// The chat history table.
    ChatHistory,
    /// The tracking record table.
    TrackingRecord,
    /// The shortlink table.
    Shortlink,
    /// The order table.
    Order,
    /// The notification table.
    Notification,
    /// The blog post table.
    BlogPost,
    /// The blog comment table.
    BlogComment,
    /// The blog like table.
    BlogLike,
    /// The blog favorite table.
    BlogFavorite,
    /// The blog image table.
    BlogImage,
}
