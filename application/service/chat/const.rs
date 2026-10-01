/// Prompt message instructing the GPT to re-evaluate task completion status.
pub const USER_PROMPT: &str = "Please re-evaluate whether the user's latest task has been completed. Continue only if the task is not finished.";

/// Task has completed.
pub const TASK_HAS_COMPLETED: &str = "task completed.";

/// Task is running.
pub const TASK_IS_RUNNING: &str = "task is running.";

/// Name constant for system name.
pub const SYSTEM_NAME: &str = "System";

/// Gpt response schema.
pub const GPT_RESPONSE_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "data": {
      "type": "string",
      "description": "The response content data"
    },
    "continue_flag": {
      "type": "boolean",
      "description": "Set to true only if the task is not finished and requires more iterations. Set to false when the task is complete or no further processing is needed."
    }
  },
  "required": ["data", "continue_flag"]
}"#;
/// Identifier scheme for a chat session.
pub const CHAT_SESSION_ID_SCHEME: &str = "uuid";

/// Display name of the built-in assistant.
pub const CHAT_GPT_ASSISTANT_NAME: &str = "GPT Assistant";

/// Chat role name for an assistant reply.
pub const CHAT_ROLE_ASSISTANT: &str = "assistant";

/// Schema name of the chat response object.
pub const CHAT_RESPONSE_SCHEMA_NAME: &str = "GptResponse";

/// Request field naming the model.
pub const CHAT_REQUEST_MODEL: &str = "model";

/// Request flag enabling model thinking.
pub const CHAT_FLAG_ENABLE_THINKING: &str = "enable_thinking";

/// Request field naming the message list.
pub const CHAT_REQUEST_MESSAGES: &str = "messages";

/// Request field naming the response format.
pub const CHAT_REQUEST_RESPONSE_FORMAT: &str = "response_format";

/// JSON key naming a value type.
pub const JSON_KEY_TYPE: &str = "type";

/// Structured output format name.
pub const CHAT_FORMAT_JSON_SCHEMA: &str = "json_schema";

/// JSON key naming a value.
pub const JSON_KEY_NAME: &str = "name";

/// Response field carrying the chat answer.
pub const CHAT_RESPONSE_FIELD: &str = "chat_response";

/// Structured output strictness flag.
pub const CHAT_SCHEMA_STRICT_MODE: &str = "strict";

/// Structured output schema field.
pub const CHAT_SCHEMA_FIELD: &str = "schema";

/// Model field holding chat template keyword arguments.
pub const CHAT_TEMPLATE_KWARGS_FIELD: &str = "chat_template_kwargs";

/// Request flag clearing prior thinking.
pub const CHAT_FLAG_CLEAR_THINKING: &str = "clear_thinking";

/// Error when the upstream API returns an empty body.
pub const ERROR_EMPTY_API_RESPONSE: &str =
    "API response is empty, possible authentication failure or network issue";

/// Model name used when none is configured.
pub const MODEL_NAME_UNKNOWN: &str = "Unknown";

/// Keepalive message text.
pub const CHAT_PING_TEXT: &str = "Ping";

/// Keepalive reply text.
pub const CHAT_PONG_TEXT: &str = "Pang";

/// JSON key carrying the response payload.
pub const JSON_KEY_DATA: &str = "data";

/// Chat role name for the system prompt.
pub const CHAT_ROLE_SYSTEM: &str = "system";

/// Chat role name for the user turn.
pub const CHAT_ROLE_USER: &str = "user";
