/// Role identifier for system messages.
pub const ROLE_SYSTEM: &str = "system";

/// Maximum number of records allowed per page in list queries.
pub(crate) const MAX_LIMIT: u64 = 100;

/// Identifier string for the GPT assistant sender type.
pub(crate) const GPT: &str = "gpt";

/// HTTP header key for x client addr.
pub(crate) const HEADER_X_CLIENT_ADDR: &str = "X-Client-Addr";

/// Prefix string for mention prefix.
pub(crate) const MENTION_PREFIX: char = '@';

/// Gpt mention upper.
pub(crate) const GPT_MENTION_UPPER: &str = "@GPT";

/// Gpt mention full.
pub(crate) const GPT_MENTION_FULL: &str = "@GPT Assistant";

/// Gpt mention lower.
pub(crate) const GPT_MENTION_LOWER: &str = "@gpt";

/// Role identifier for user messages.
pub(crate) const ROLE_USER: &str = "user";

/// Role identifier for assistant messages.
pub(crate) const ROLE_ASSISTANT: &str = "assistant";

/// JSON field name for role.
pub(crate) const JSON_FIELD_ROLE: &str = "role";

/// JSON field name for content.
pub(crate) const JSON_FIELD_CONTENT: &str = "content";

/// JSON field name for result.
pub(crate) const JSON_FIELD_RESULT: &str = "result";

/// JSON field name for response.
pub(crate) const JSON_FIELD_RESPONSE: &str = "response";

/// JSON field name for choices.
pub(crate) const JSON_FIELD_CHOICES: &str = "choices";

/// JSON field name for message.
pub(crate) const JSON_FIELD_MESSAGE: &str = "message";

/// Description string for the current number of online WebSocket connections.
pub(crate) const ONLINE_CONNECTIONS: &str = "Current number of online connections";
