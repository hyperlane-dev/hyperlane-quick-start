use super::*;

impl ServerHook for ChatConnectedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: ReceiverCount = websocket.receiver_count(key.clone());
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(ctx, receiver_count.to_string()).await;
        ctx.get_mut_response().set_body(resp_data.clone());
        ChatService::broadcast_online_count(key, resp_data.clone());
    }
}

impl ServerHook for ChatRequestHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_body_json_result(req_data_res: WebSocketReqData)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let req_data: WebSocketReqData = req_data_res.unwrap();
        if ChatService::handle_ping_request(ctx, &req_data).await {
            return;
        }
        let resp_data: WebSocketRespData = req_data.into_resp(ctx).await;
        let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
        ctx.get_mut_response().set_body(&resp_data);
        let session_id: String = ChatService::get_name(ctx).await;
        clone!(req_data, session_id => {
            let req_msg: &String = req_data.get_data();
            if ChatService::is_gpt_mentioned(req_msg) {
                let req_msg_clone: String = req_msg.clone();
                let ctx: &'static mut Context = context!(ctx);
                spawn(async move {
                    ChatService::process_gpt_request(session_id, req_msg_clone, ctx).await;
                });
            }
        });
    }
}

impl ServerHook for ChatSendedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: String = get_request_json(ctx).await;
        let response: String = get_response_json(ctx).await;
        info!("{request}{BR}{response}");
        let response_body: ResponseBody = ctx.get_response().get_body().clone();
        if let Ok(resp_data) = serde_json::from_slice::<WebSocketRespData>(&response_body) {
            if *resp_data.get_type() == MessageType::OnlineCount {
                ctx.set_aborted(true);
                return;
            }
        }
        let session_id: String = ChatService::get_name(ctx).await;
        ChatService::save_message_from_response(&session_id, &response_body).await;
    }
}

impl ServerHook for ChatClosedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path.clone());
        let receiver_count: ReceiverCount = websocket.receiver_count_after_closed(key);
        let username: String = ChatService::get_name(ctx).await;
        ChatDomain::remove_online_user(&username).await;
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(ctx, receiver_count.to_string()).await;
        ctx.get_mut_response().set_body(&resp_data);
    }
}

impl ChatService {
    #[instrument_trace]
    pub async fn pre_ws_upgrade(ctx: &mut Context) {
        let addr: String = ctx.get_socket_addr_string().await;
        let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
        ctx.get_mut_response()
            .set_header(HEADER_X_CLIENT_ADDR, &encode_addr);
    }

    #[instrument_trace]
    pub async fn create_online_count_message(
        ctx: &mut Context,
        receiver_count: String,
    ) -> ResponseBody {
        let data: String = format!("{ONLINE_CONNECTIONS} {receiver_count}");
        WebSocketRespData::get_json_data(MessageType::OnlineCount, ctx, data)
            .await
            .unwrap()
    }

    #[instrument_trace]
    pub fn broadcast_online_count(key: BroadcastType<String>, message: ResponseBody) {
        let websocket: &WebSocket = get_global_websocket();
        let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> = websocket.try_send(key, message);
    }

    #[instrument_trace]
    fn remove_mentions(text: &str) -> String {
        text.split_whitespace()
            .filter(|word| !word.starts_with(MENTION_PREFIX))
            .collect::<Vec<&str>>()
            .join(SPACE)
    }

    #[instrument_trace]
    pub async fn handle_ping_request(ctx: &mut Context, req_data: &WebSocketReqData) -> bool {
        if req_data.is_ping() {
            let resp_data: WebSocketRespData =
                WebSocketRespData::from(MessageType::Pang, ctx, EMPTY_STR).await;
            let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
            ctx.get_mut_response().set_body(&resp_data);
            return true;
        }
        false
    }

    #[instrument_trace]
    pub fn is_gpt_mentioned(message: &str) -> bool {
        message.contains(GPT_MENTION_UPPER)
            || message.contains(GPT_MENTION_FULL)
            || message.contains(GPT_MENTION_LOWER)
    }

    #[instrument_trace]
    pub async fn process_gpt_request(session_id: String, message: String, ctx: &mut Context) {
        let mut session: ChatSession = ChatDomain::get_or_create_session(&session_id).await;
        let cleaned_msg: String = Self::remove_mentions(&message);
        session.add_message(ROLE_USER.to_string(), cleaned_msg);
        let api_response: String = match Self::call_gpt_api_with_context(&session).await {
            Ok(gpt_response) => {
                session.add_message(ROLE_ASSISTANT.to_string(), gpt_response.clone());
                ChatDomain::update_session(session).await;
                format!("{MENTION_PREFIX}{session_id}{SPACE}{gpt_response}")
            }
            Err(error) => format!("API call failed {error}"),
        };
        let gpt_resp_data: WebSocketRespData =
            WebSocketRespData::from(MessageType::GptResponse, ctx, &api_response).await;
        let gpt_resp_json: ResponseBody = serde_json::to_vec(&gpt_resp_data).unwrap();
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
            websocket.try_send(key, gpt_resp_json.clone());
        ctx.get_mut_response().set_body(&gpt_resp_json);
        let session_id_clone: String = session_id.clone();
        let api_response_clone: String = api_response.clone();
        spawn(async move {
            let save_res: Result<(), String> = Self::save_message(
                &session_id_clone,
                "GPT Assistant",
                "assistant",
                "GptResponse",
                &api_response_clone,
            )
            .await;
            if save_res.is_err() {
                error!(
                    "Failed to save GPT response for session {session_id_clone} {}",
                    save_res.err().unwrap_or_default()
                );
            }
        });
    }

    #[instrument_trace]
    fn build_gpt_request_messages(session: &ChatSession) -> Vec<serde_json::Value> {
        session
            .get_messages()
            .iter()
            .map(|msg| {
                json!({
                    JSON_FIELD_ROLE: msg.get_role(),
                    JSON_FIELD_CONTENT: msg.get_content()
                })
            })
            .collect()
    }

    #[instrument_trace]
    fn build_gpt_request_headers() -> HashMapXxHash3_64<&'static str, String> {
        let mut headers: HashMapXxHash3_64<&'static str, String> = hash_map_xx_hash3_64();
        headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string());
        headers
    }

    #[instrument_trace]
    fn extract_response_content(response_json: &serde_json::Value) -> Option<String> {
        response_json
            .get(JSON_FIELD_RESULT)
            .and_then(|result: &serde_json::Value| result.get(JSON_FIELD_RESPONSE))
            .and_then(|response: &serde_json::Value| response.as_str())
            .filter(|data: &&str| !data.is_empty())
            .map(String::from)
            .or_else(|| {
                response_json
                    .get(JSON_FIELD_CHOICES)
                    .and_then(|choices: &serde_json::Value| choices.get(0))
                    .and_then(|choice: &serde_json::Value| choice.get(JSON_FIELD_MESSAGE))
                    .and_then(|message: &serde_json::Value| message.get(JSON_FIELD_CONTENT))
                    .and_then(|content: &serde_json::Value| content.as_str())
                    .map(String::from)
            })
    }

    #[instrument_trace]
    fn extract_error_message(response_json: &serde_json::Value) -> Option<String> {
        response_json
            .get(JSON_FIELD_ERRORS)
            .and_then(|errors| errors.get(0))
            .and_then(|error| error.get(JSON_FIELD_MESSAGE))
            .and_then(|message| message.as_str())
            .map(|msg| format!("API error {msg}"))
            .or_else(|| Some("API error: Unknown error".to_string()))
    }

    #[instrument_trace]
    fn handle_gpt_api_response(response_text: &str) -> Result<String, String> {
        if response_text.trim().is_empty() {
            return Err(
                "API response is empty, possible authentication failure or network issue"
                    .to_string(),
            );
        }
        let response_json: serde_json::Value =
            serde_json::from_str(response_text).map_err(|error| {
                format!("JSON parsing failed {error} (response content {response_text})",)
            })?;
        if let Some(content) = Self::extract_response_content(&response_json) {
            return Ok(content);
        }
        if let Some(error) = Self::extract_error_message(&response_json) {
            return Err(error);
        }
        Err(format!("Incorrect API response format {response_text}"))
    }

    #[instrument_trace]
    pub async fn get_name(ctx: &mut Context) -> String {
        #[request_query_option("uuid" => uuid_opt)]
        #[instrument_trace]
        async fn inner(_ctx: &mut Context) -> String {
            uuid_opt.unwrap_or_default()
        }
        inner(ctx).await
    }

    #[instrument_trace]
    async fn call_gpt_api_with_context(session: &ChatSession) -> Result<String, String> {
        let config: &EnvConfig = EnvPlugin::get_or_init();
        let gpt_model: &str = config.get_gpt_model();
        let messages: Vec<serde_json::Value> = Self::build_gpt_request_messages(session);
        let body: serde_json::Value = json!({
            GPT_MODEL: gpt_model,
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
            Err(error) => Err(format!("Request sending failed {error}")),
        }
    }

    #[instrument_trace]
    pub async fn save_message_from_response(session_id: &str, response_body: &ResponseBody) {
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
            let session_id: String = session_id.to_string();
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
                    error!(
                        "Failed to save message for session {session_id} {}",
                        save_res.err().unwrap_or_default()
                    );
                }
            });
        }
    }

    #[instrument_trace]
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

    #[instrument_trace]
    pub async fn get_chat_history(
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<ChatHistoryResponse, String> {
        let messages: Vec<ChatHistory> = ChatHistoryMapper::get_history(before_id, limit).await?;
        let total: i64 = ChatHistoryMapper::count_messages().await?;
        let has_more: bool = messages.len() as i64 == limit;
        let mut response = ChatHistoryResponse::default();
        response
            .set_messages(messages)
            .set_total(total as usize)
            .set_has_more(has_more);
        Ok(response)
    }
}
