pub const USER_PROMPT: &str = "If task is not complete, please continue.";
pub const TASK_HAS_COMPLETED: &str = "task has completed.";
pub const TASK_IS_RUNNING: &str = "task is running.";
pub const RESPONSE_FORMAT_TYPE: &str = "json_schema";
pub const RESPONSE_FORMAT_SCHEMA_NAME: &str = "chat_response";
pub const GPT_RESPONSE_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "data": {
      "type": "string",
      "description": "The response content data"
    },
    "continue_flag": {
      "type": "boolean",
      "description": "Whether the assistant needs to continue processing in the next iteration. If true, the assistant will continue processing in the next iteration. If false, the assistant will stop processing."
    }
  },
  "required": ["data", "continue_flag"]
}"#;
