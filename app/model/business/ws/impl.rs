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
        let env_content: Vec<u8> = read_from_file(WS_ENV_FILE_PATH)
            .map_err(|e| format!("Failed to read /shell/env file: {}", e))?;
        let env_content: Cow<'_, str> = String::from_utf8_lossy(&env_content);
        let mut config_map: HashMap<String, String> = HashMap::new();
        for line in env_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        let gpt_api_url = config_map
            .get("GPT_API_URL")
            .ok_or("GPT_API_URL not found in /shell/env")?
            .clone();

        let gpt_api_key = config_map
            .get("GPT_API_KEY")
            .ok_or("GPT_API_KEY not found in /shell/env")?
            .clone();

        Ok(EnvConfig {
            gpt_api_url,
            gpt_api_key,
        })
    }
}
