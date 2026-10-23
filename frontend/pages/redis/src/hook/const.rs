/// The endpoint of the records of the cache.
pub(crate) const PATH_REDIS_LIST: &str = "/api/redis/list";

/// The endpoint that creates a record.
pub(crate) const PATH_REDIS_CREATE: &str = "/api/redis/create";

/// The endpoint that updates a record.
pub(crate) const PATH_REDIS_UPDATE: &str = "/api/redis/update";

/// The endpoint that removes a record.
pub(crate) const PATH_REDIS_DELETE: &str = "/api/redis/delete";

/// The query parameter that names the record to remove.
pub(crate) const QUERY_KEY: &str = "key";

/// The empty text of a field.
pub(crate) const TEXT_EMPTY: &str = "";

/// The message reported when the visitor leaves a field empty.
pub(crate) const MESSAGE_FIELDS_REQUIRED: &str = "Please fill in all fields";

/// The message reported after a record has been created.
pub(crate) const MESSAGE_CREATED: &str = "The record has been created";

/// The message reported after a record has been updated.
pub(crate) const MESSAGE_UPDATED: &str = "The record has been updated";

/// The message reported after a record has been removed.
pub(crate) const MESSAGE_DELETED: &str = "The record has been removed";
