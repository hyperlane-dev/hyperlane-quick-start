/// Prompt message instructing the GPT to re-evaluate task completion status.
pub(crate) const USER_PROMPT: &str = "Please re-evaluate whether the user's latest task has been completed. Continue only if the task is not finished.";

/// Task has completed.
pub(crate) const TASK_HAS_COMPLETED: &str = "task completed.";

/// Task is running.
pub(crate) const TASK_IS_RUNNING: &str = "task is running.";

/// Name constant for system name.
pub(crate) const SYSTEM_NAME: &str = "System";

/// Gpt response schema.
pub(crate) const GPT_RESPONSE_SCHEMA: &str = r#"{
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
pub(crate) const CHAT_SESSION_ID_SCHEME: &str = "uuid";

/// Display name of the built-in assistant.
pub(crate) const CHAT_GPT_ASSISTANT_NAME: &str = "GPT Assistant";

/// Chat role name for an assistant reply.
pub(crate) const CHAT_ROLE_ASSISTANT: &str = "assistant";

/// Schema name of the chat response object.
pub(crate) const CHAT_RESPONSE_SCHEMA_NAME: &str = "GptResponse";

/// Request field naming the model.
pub(crate) const CHAT_REQUEST_MODEL: &str = "model";

/// Request flag enabling model thinking.
pub(crate) const CHAT_FLAG_ENABLE_THINKING: &str = "enable_thinking";

/// Request field naming the message list.
pub(crate) const CHAT_REQUEST_MESSAGES: &str = "messages";

/// Request field naming the response format.
pub(crate) const CHAT_REQUEST_RESPONSE_FORMAT: &str = "response_format";

/// JSON key naming a value type.
pub(crate) const JSON_KEY_TYPE: &str = "type";

/// Structured output format name.
pub(crate) const CHAT_FORMAT_JSON_SCHEMA: &str = "json_schema";

/// JSON key naming a value.
pub(crate) const JSON_KEY_NAME: &str = "name";

/// Response field carrying the chat answer.
pub(crate) const CHAT_RESPONSE_FIELD: &str = "chat_response";

/// Structured output strictness flag.
pub(crate) const CHAT_SCHEMA_STRICT_MODE: &str = "strict";

/// Structured output schema field.
pub(crate) const CHAT_SCHEMA_FIELD: &str = "schema";

/// Model field holding chat template keyword arguments.
pub(crate) const CHAT_TEMPLATE_KWARGS_FIELD: &str = "chat_template_kwargs";

/// Request flag clearing prior thinking.
pub(crate) const CHAT_FLAG_CLEAR_THINKING: &str = "clear_thinking";

/// Error when the upstream API returns an empty body.
pub(crate) const ERROR_EMPTY_API_RESPONSE: &str =
    "API response is empty, possible authentication failure or network issue";

/// Model name used when none is configured.
pub(crate) const MODEL_NAME_UNKNOWN: &str = "Unknown";

/// Keepalive message text.
pub(crate) const CHAT_PING_TEXT: &str = "Ping";

/// Keepalive reply text.
pub(crate) const CHAT_PONG_TEXT: &str = "Pang";

/// JSON key carrying the response payload.
pub(crate) const JSON_KEY_DATA: &str = "data";

/// Chat role name for the system prompt.
pub(crate) const CHAT_ROLE_SYSTEM: &str = "system";

/// Chat role name for the user turn.
pub(crate) const CHAT_ROLE_USER: &str = "user";
