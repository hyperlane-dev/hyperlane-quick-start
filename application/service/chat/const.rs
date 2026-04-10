pub const MAX_GPT_ITERATIONS: usize = 10;
pub const JSON_FIELD_RESPONSE_CONTENT: &str = "response_content";
pub const JSON_FIELD_SHOULD_CONTINUE: &str = "should_continue";
pub const SYSTEM_INSTRUCTION: &str = r#"You are a helpful assistant. You must respond in JSON format following the provided schema. Instructions: 1. Provide your response in the "response_content" field; 2. Set "should_continue" to true ONLY if you have more content to say and want to continue in the next iteration; 3. Set "should_continue" to false if your response is complete; 4. Do NOT include any text outside the JSON structure."#;
