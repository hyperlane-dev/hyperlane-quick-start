pub const USER_PROMPT: &str = "Please re-evaluate whether the user's latest task has been completed. Continue only if the task is not finished.";
pub const TASK_HAS_COMPLETED: &str = "task completed.";
pub const TASK_IS_RUNNING: &str = "task is running.";
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
