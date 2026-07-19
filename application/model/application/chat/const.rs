/// Maximum number of records allowed per page in list queries.
pub const MAX_LIMIT: u64 = 100;

/// Identifier string for the GPT assistant sender type.
pub const GPT: &str = "gpt";

/// HTTP header key for x client addr.
pub const HEADER_X_CLIENT_ADDR: &str = "X-Client-Addr";

/// Prefix string for mention prefix.
pub const MENTION_PREFIX: char = '@';

/// Gpt mention upper.
pub const GPT_MENTION_UPPER: &str = "@GPT";

/// Gpt mention full.
pub const GPT_MENTION_FULL: &str = "@GPT Assistant";

/// Gpt mention lower.
pub const GPT_MENTION_LOWER: &str = "@gpt";

/// Role identifier for system messages.
pub const ROLE_SYSTEM: &str = "system";

/// Role identifier for user messages.
pub const ROLE_USER: &str = "user";

/// Role identifier for assistant messages.
pub const ROLE_ASSISTANT: &str = "assistant";

/// JSON field name for role.
pub const JSON_FIELD_ROLE: &str = "role";

/// JSON field name for content.
pub const JSON_FIELD_CONTENT: &str = "content";

/// JSON field name for result.
pub const JSON_FIELD_RESULT: &str = "result";

/// JSON field name for response.
pub const JSON_FIELD_RESPONSE: &str = "response";

/// JSON field name for choices.
pub const JSON_FIELD_CHOICES: &str = "choices";

/// JSON field name for message.
pub const JSON_FIELD_MESSAGE: &str = "message";

/// Identifier string for the system sender type.
pub const SYSTEM: &str = "system";

/// Description string for the current number of online WebSocket connections.
pub const ONLINE_CONNECTIONS: &str = "Current number of online connections";
