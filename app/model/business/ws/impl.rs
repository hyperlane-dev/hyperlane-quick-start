use super::*;

impl Default for MessageType {
    fn default() -> Self {
        Self::Text
    }
}

impl MessageType {
    fn is_ping(&self) -> bool {
        matches!(self, MessageType::Ping)
    }
}

impl WebSocketReqData {
    pub fn new<T: ToString>(r#type: MessageType, data: T) -> Self {
        let mut resp_data: Self = Self::default();
        resp_data.set_type(r#type).set_data(data.to_string());
        resp_data
    }

    pub fn is_ping(&self) -> bool {
        self.get_type().is_ping()
    }

    pub async fn into_resp(&self, ctx: &Context) -> WebSocketRespData {
        let name: String = get_name(&ctx).await;
        let mut resp: WebSocketRespData = WebSocketRespData::default();
        resp.set_type(*self.get_type())
            .set_name(name)
            .set_data(self.get_data().clone())
            .set_time(time());
        resp
    }
}

impl WebSocketRespData {
    pub async fn new<T: ToString>(r#type: MessageType, ctx: &Context, data: T) -> Self {
        let name: String = get_name(&ctx).await;
        let mut resp_data: Self = Self::default();
        resp_data
            .set_type(r#type)
            .set_data(data.to_string())
            .set_time(time());
        if r#type == MessageType::OnlineCount {
            resp_data.set_name("System".to_string());
        } else {
            resp_data.set_name(name.to_string());
        }
        resp_data
    }

    pub async fn get_json_data<T: ToString>(
        r#type: MessageType,
        ctx: &Context,
        data: T,
    ) -> ResultJsonError<String> {
        json_stringify_string(&WebSocketRespData::new(r#type, ctx, data).await)
    }
}

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let env_content: Vec<u8> = match read_from_file(WS_ENV_FILE_PATH) {
            Ok(content) => content,
            Err(_) => {
                let example_content: &str = "GPT_API_URL=\nGPT_API_KEY=";
                let _ = write_to_file(WS_ENV_FILE_PATH, example_content.as_bytes())
                    .map_err(|e| format!("Failed to create example env file: {}", e))?;
                return Self::load();
            }
        };
        let env_content: Cow<'_, str> = String::from_utf8_lossy(&env_content);
        let mut config_map: HashMap<String, String> = HashMap::new();
        for line in env_content.lines() {
            let line: &str = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        let gpt_api_url: String = config_map
            .get("GPT_API_URL")
            .ok_or("GPT_API_URL not found in /shell/env")?
            .clone();
        let gpt_api_key: String = config_map
            .get("GPT_API_KEY")
            .ok_or("GPT_API_KEY not found in /shell/env")?
            .clone();
        Ok(EnvConfig {
            gpt_api_url,
            gpt_api_key,
        })
    }
}

impl ChatSession {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            messages: Vec::new(),
            last_activity: std::time::Instant::now(),
        }
    }

    pub fn add_message(&mut self, role: String, content: String) {
        self.messages.push(ChatMessage { role, content });
        self.last_activity = std::time::Instant::now();

        if self.messages.len() > 20 {
            self.messages.drain(0..self.messages.len() - 20);
        }
    }

    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.last_activity.elapsed().as_secs() > timeout_minutes * 60
    }
}
