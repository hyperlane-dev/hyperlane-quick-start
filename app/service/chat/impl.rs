use super::*;

impl ServerHook for ChatConnectedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path.clone());
        let receiver_count: ReceiverCount = websocket.receiver_count(key.clone());
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(ctx, receiver_count.to_string()).await;
        ctx.set_response_body(resp_data.clone()).await;
        ChatService::broadcast_online_count(key, resp_data.clone());
        ChatService::save_message_from_response(&path, &resp_data).await;
    }
}

impl ServerHook for ChatRequestHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[request_body_json(req_data_res: WebSocketReqData)]
    async fn handle(self, ctx: &Context) {
        let req_data: WebSocketReqData = req_data_res.unwrap();
        if ChatService::handle_ping_request(ctx, &req_data).await {
            return;
        }
        let resp_data: WebSocketRespData = req_data.into_resp(ctx).await;
        let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
        ctx.set_response_body(&resp_data).await;
        let session_id: String = ChatService::get_name(ctx).await;
        clone!(req_data, ctx, session_id => {
            let req_msg: &String = req_data.get_data();
            if ChatService::is_gpt_mentioned(req_msg) {
                let req_msg_clone: String = req_msg.clone();
                spawn(async move {
                    ChatService::process_gpt_request(session_id, req_msg_clone, ctx).await;
                });
            }
        });
    }
}

impl ServerHook for ChatSendedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let path: String = ctx.get_request_path().await;
        let request_string: String = ctx.get_request().await.get_body_string();
        let request: String = ctx.get_request().await.get_string();
        let response: String = ctx.get_response().await.get_string();
        log_info(&format!("{request}{BR}{request_string}{BR}{response}")).await;
        let response_body: ResponseBody = ctx.get_response().await.get_body().clone();
        ChatService::save_message_from_response(&path, &response_body).await;
    }
}

impl ServerHook for ChatClosedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path.clone());
        let receiver_count: ReceiverCount = websocket.receiver_count_after_closed(key);
        let username: String = ChatService::get_name(ctx).await;
        ChatDomain::remove_online_user(&username);
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(ctx, receiver_count.to_string()).await;
        ctx.set_response_body(&resp_data).await;
        ChatService::save_message_from_response(&path, &resp_data).await;
    }
}

impl ChatService {
    pub async fn pre_ws_upgrade(ctx: Context) {
        let addr: String = ctx.get_socket_addr_string().await;
        let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
        ctx.set_response_header(HEADER_X_CLIENT_ADDR, &encode_addr)
            .await;
    }

    pub async fn create_online_count_message(
        ctx: &Context,
        receiver_count: String,
    ) -> ResponseBody {
        let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
        WebSocketRespData::get_json_data(MessageType::OnlineCount, ctx, data)
            .await
            .unwrap()
    }

    pub fn broadcast_online_count(key: BroadcastType<String>, message: ResponseBody) {
        let websocket: &WebSocket = get_global_websocket();
        let _: BroadcastMapSendResult<Vec<u8>> = websocket.send(key, message);
    }

    fn remove_mentions(text: &str) -> String {
        text.split_whitespace()
            .filter(|word| !word.starts_with(MENTION_PREFIX))
            .collect::<Vec<&str>>()
            .join(SPACE)
    }

    pub async fn handle_ping_request(ctx: &Context, req_data: &WebSocketReqData) -> bool {
        if req_data.is_ping() {
            let resp_data: WebSocketRespData =
                WebSocketRespData::new(MessageType::Pang, ctx, "").await;
            let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
            ctx.set_response_body(&resp_data).await;
            return true;
        }
        false
    }

    pub fn is_gpt_mentioned(message: &str) -> bool {
        message.contains(GPT_MENTION_UPPER)
            || message.contains(GPT_MENTION_FULL)
            || message.contains(GPT_MENTION_LOWER)
    }

    pub async fn process_gpt_request(session_id: String, message: String, ctx: Context) {
        let mut session: ChatSession = ChatDomain::get_or_create_session(&session_id);
        let cleaned_msg: String = Self::remove_mentions(&message);
        session.add_message(ROLE_USER.to_string(), cleaned_msg);
        let api_response: String = match Self::call_gpt_api_with_context(&session).await {
            Ok(gpt_response) => {
                session.add_message(ROLE_ASSISTANT.to_string(), gpt_response.clone());
                ChatDomain::update_session(session);
                format!("{MENTION_PREFIX}{session_id}{SPACE}{gpt_response}")
            }
            Err(error) => format!("API call failed: {error}"),
        };
        let gpt_resp_data: WebSocketRespData =
            WebSocketRespData::new(MessageType::GptResponse, &ctx, &api_response).await;
        let gpt_resp_json: ResponseBody = serde_json::to_vec(&gpt_resp_data).unwrap();
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let _: BroadcastMapSendResult<Vec<u8>> = websocket.send(key, gpt_resp_json.clone());
        ctx.set_response_body(&gpt_resp_json).await;
    }

    fn build_gpt_request_messages(session: &ChatSession) -> Vec<JsonValue> {
        session
            .get_messages()
            .iter()
            .map(|msg| {
                json_value!({
                    JSON_FIELD_ROLE: msg.get_role(),
                    JSON_FIELD_CONTENT: msg.get_content()
                })
            })
            .collect()
    }

    fn build_gpt_request_headers() -> HashMapXxHash3_64<&'static str, String> {
        let mut headers: HashMapXxHash3_64<&'static str, String> = hash_map_xx_hash3_64();
        headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string());
        headers
    }

    fn extract_response_content(response_json: &JsonValue) -> Option<String> {
        response_json
            .get(JSON_FIELD_RESULT)
            .and_then(|result: &JsonValue| result.get(JSON_FIELD_RESPONSE))
            .and_then(|response: &JsonValue| response.as_str())
            .filter(|data: &&str| !data.is_empty())
            .map(String::from)
            .or_else(|| {
                response_json
                    .get(JSON_FIELD_CHOICES)
                    .and_then(|choices: &JsonValue| choices.get(0))
                    .and_then(|choice: &JsonValue| choice.get(JSON_FIELD_MESSAGE))
                    .and_then(|message: &JsonValue| message.get(JSON_FIELD_CONTENT))
                    .and_then(|content: &JsonValue| content.as_str())
                    .map(String::from)
            })
    }

    fn extract_error_message(response_json: &JsonValue) -> Option<String> {
        response_json
            .get(JSON_FIELD_ERRORS)
            .and_then(|errors| errors.get(0))
            .and_then(|error| error.get(JSON_FIELD_MESSAGE))
            .and_then(|message| message.as_str())
            .map(|msg| format!("API error: {msg}"))
            .or_else(|| Some("API error: Unknown error".to_string()))
    }

    fn handle_gpt_api_response(response_text: &str) -> Result<String, String> {
        if response_text.trim().is_empty() {
            return Err(
                "API response is empty, possible authentication failure or network issue"
                    .to_string(),
            );
        }
        let response_json: JsonValue = serde_json::from_str(response_text).map_err(|error| {
            format!("JSON parsing failed: {error} (response content: {response_text})",)
        })?;
        if let Some(content) = Self::extract_response_content(&response_json) {
            return Ok(content);
        }
        if let Some(error) = Self::extract_error_message(&response_json) {
            return Err(error);
        }
        Err(format!("Incorrect API response format: {response_text}"))
    }

    pub async fn get_name(ctx: &Context) -> String {
        #[request_query("uuid" => uuid_opt)]
        async fn inner(_ctx: &Context) -> String {
            uuid_opt.unwrap_or_default()
        }
        inner(ctx).await
    }

    async fn call_gpt_api_with_context(session: &ChatSession) -> Result<String, String> {
        let config: &EnvConfig = get_global_env_config();
        let gtp_model: &str = config.get_gtp_model();
        let messages: Vec<JsonValue> = Self::build_gpt_request_messages(session);
        let body: JsonValue = json_value!({
            GPT_MODEL: gtp_model,
            JSON_FIELD_MESSAGES: messages
        });
        let headers: HashMapXxHash3_64<&str, String> = Self::build_gpt_request_headers();
        let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
            .post(config.get_gpt_api_url())
            .json(body)
            .headers(headers)
            .redirect()
            .http1_1_only()
            .build_async();
        match request_builder.send().await {
            Ok(response) => {
                let response_text: String = response.text().get_body();
                Self::handle_gpt_api_response(&response_text)
            }
            Err(error) => Err(format!("Request sending failed: {error}")),
        }
    }

    pub async fn save_message_from_response(path: &str, response_body: &ResponseBody) {
        let response_body_string: String = String::from_utf8_lossy(response_body).into_owned();
        if let Ok(resp_data) = serde_json::from_str::<serde_json::Value>(&response_body_string) {
            let message_type: String = resp_data
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            if message_type == "Ping" || message_type == "Pang" {
                return;
            }
            let sender_name: String = resp_data
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let content: String = resp_data
                .get("data")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let session_id: String = path.trim_start_matches("/chat/").to_string();
            let sender_type: &str = if sender_name == "System" {
                "system"
            } else if sender_name == "GPT Assistant" || message_type == "GptResponse" {
                "assistant"
            } else {
                "user"
            };
            spawn(async move {
                let save_res: Result<(), String> = Self::save_message(
                    &session_id,
                    &sender_name,
                    sender_type,
                    &message_type,
                    &content,
                )
                .await;
                if save_res.is_err() {
                    log_error(&format!(
                        "Failed to save message for session {}: {}",
                        session_id,
                        save_res.err().unwrap_or_default()
                    ))
                    .await;
                }
            });
        }
    }

    pub async fn save_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        ChatHistoryMapper::insert_message(
            session_id,
            sender_name,
            sender_type,
            message_type,
            content,
        )
        .await
    }

    pub async fn get_chat_history(offset: i64, limit: i64) -> Result<ChatHistoryResponse, String> {
        let messages: Vec<ChatHistory> = ChatHistoryMapper::get_history(offset, limit).await?;
        let total: i64 = ChatHistoryMapper::count_messages().await?;
        let has_more: bool = (offset + limit) < total;
        Ok(ChatHistoryResponse {
            messages,
            total: total as usize,
            has_more,
        })
    }
}
