//! This module contains constants related to the chat business logic.

pub const HEADER_X_CLIENT_ADDR: &str = "X-Client-Addr";

pub const MENTION_PREFIX: char = '@';

pub const GPT_MENTION_UPPER: &str = "@GPT";
pub const GPT_MENTION_FULL: &str = "@GPT Assistant";
pub const GPT_MENTION_LOWER: &str = "@gpt";

pub const ROLE_USER: &str = "user";
pub const ROLE_ASSISTANT: &str = "assistant";

pub const JSON_FIELD_ROLE: &str = "role";
pub const JSON_FIELD_CONTENT: &str = "content";
pub const JSON_FIELD_RESULT: &str = "result";
pub const JSON_FIELD_RESPONSE: &str = "response";
pub const JSON_FIELD_CHOICES: &str = "choices";
pub const JSON_FIELD_MESSAGE: &str = "message";
pub const JSON_FIELD_MESSAGES: &str = "messages";
pub const JSON_FIELD_ERRORS: &str = "errors";
pub const GPT_MODEL: &str = "model";
