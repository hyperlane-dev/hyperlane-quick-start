use super::*;

impl ServerHook for ChatConnectedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_query_option("uuid" => uuid_opt)]
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
        let uuid: String = uuid_opt.unwrap_or_default();
        if !uuid.is_empty() {
            ChatDomain::add_online_user(&uuid).await;
        }
    }
}

impl ServerHook for ChatRequestHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_query_option("uuid" => uuid_opt)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: &Request = ctx.get_request();
        let req_data: WebSocketReqData = match request.try_get_body_json() {
            Ok(data) => data,
            Err(error) => {
                ctx.get_mut_response().set_body(error.to_string());
                return;
            }
        };
        if ChatService::handle_ping_request(ctx, &req_data).await {
            return;
        }
        let resp_data: WebSocketRespData = req_data.into_resp(ctx).await;
        let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
        ctx.get_mut_response().set_body(&resp_data);
        let uuid: String = uuid_opt.unwrap_or_default();
        let req_msg: String = req_data.get_data().clone();
        if ChatService::is_gpt_mentioned(&req_msg) {
            let mut ctx_clone: Context = ctx.clone();
            spawn(async move {
                let path: String = ctx_clone.get_request().get_path().clone();
                let system_resp_data: WebSocketRespData = WebSocketRespData::from(
                    MessageType::System,
                    &mut ctx_clone,
                    format!("{GPT} is running, please wait ..."),
                )
                .await;
                let websocket: &WebSocket = get_global_websocket();
                let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
                let system_resp_json: ResponseBody =
                    serde_json::to_vec(&system_resp_data).unwrap_or_default();
                let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
                    websocket.try_send(key, system_resp_json);
                ChatService::process_gpt_request(uuid, req_msg, &mut ctx_clone).await;
            });
        }
    }
}

impl ServerHook for ChatSendedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_query_option("uuid" => uuid_opt)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: String = get_request_json(ctx).await;
        let response: String = get_response_json(ctx).await;
        info!("{request}{BR}{response}");
        let response_body: ResponseBody = ctx.get_response().get_body().clone();
        if let Ok(resp_data) = serde_json::from_slice::<WebSocketRespData>(&response_body)
            && matches!(
                resp_data.get_type(),
                MessageType::System | MessageType::OnlineCount
            )
        {
            ctx.set_aborted(true);
            return;
        }
        let uuid: String = uuid_opt.unwrap_or_default();
        ChatService::save_message_from_response(&uuid, &response_body).await;
    }
}

impl ServerHook for ChatClosedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_query_option("uuid" => uuid_opt)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path.clone());
        let receiver_count: ReceiverCount = websocket.receiver_count_after_closed(key);
        let uuid: String = uuid_opt.unwrap_or_default();
        ChatDomain::remove_online_user(&uuid).await;
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(ctx, receiver_count.to_string()).await;
        ctx.get_mut_response().set_body(&resp_data);
    }
}

impl ChatService {
    #[instrument_trace]
    pub async fn pre_ws_upgrade(ctx: &mut Context) {
        let mut socket_addr: String = String::new();
        if let Some(stream) = ctx.try_get_stream().as_ref() {
            socket_addr = stream
                .read()
                .await
                .peer_addr()
                .map(|data| data.to_string())
                .unwrap_or_default();
        }
        let encode_addr: String = Encode::execute(CHARSETS, &socket_addr).unwrap_or_default();
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
        let path: String = ctx.get_request().get_path().clone();
        let websocket: &WebSocket = get_global_websocket();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let mut session: ChatSession = ChatDomain::get_or_create_session(&session_id).await;
        let cleaned_msg: String = Self::remove_mentions(&message);
        session.add_message(ROLE_USER.to_string(), cleaned_msg);
        ChatDomain::update_session(session.clone()).await;
        let continue_prompt: &str = "Continue from where you left off.";
        let mut is_first_iteration: bool = true;
        let mut iteration_count: usize = 0;
        loop {
            iteration_count += 1;
            if iteration_count > MAX_GPT_ITERATIONS {
                let final_msg: String =
                    format!("{MENTION_PREFIX}{session_id}{SPACE}[Maximum response length reached]");
                let final_resp_data: WebSocketRespData =
                    WebSocketRespData::from(MessageType::GptResponse, ctx, &final_msg).await;
                let final_resp_json: ResponseBody = serde_json::to_vec(&final_resp_data).unwrap();
                let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
                    websocket.try_send(key.clone(), final_resp_json);
                break;
            }
            let mut current_session: ChatSession =
                ChatDomain::get_or_create_session(&session_id).await;
            if !is_first_iteration {
                current_session.add_message(ROLE_USER.to_string(), continue_prompt.to_string());
                ChatDomain::update_session(current_session.clone()).await;
                current_session = ChatDomain::get_or_create_session(&session_id).await;
            }
            is_first_iteration = false;
            let api_result: Result<(String, bool), String> =
                Self::call_gpt_api_with_context(&current_session).await;
            let (response_content, should_continue): (String, bool) = match api_result {
                Ok((content, should_continue_flag)) => {
                    if content.is_empty() {
                        ("[No more content]".to_string(), false)
                    } else {
                        let mut updated_session: ChatSession =
                            ChatDomain::get_or_create_session(&session_id).await;
                        updated_session.add_message(ROLE_ASSISTANT.to_string(), content.clone());
                        ChatDomain::update_session(updated_session).await;
                        let formatted_response: String =
                            format!("{MENTION_PREFIX}{session_id}{SPACE}{content}");
                        (formatted_response, should_continue_flag)
                    }
                }
                Err(error) => {
                    let error_msg: String = format!("API call failed {error}");
                    (error_msg, false)
                }
            };
            let gpt_resp_data: WebSocketRespData =
                WebSocketRespData::from(MessageType::GptResponse, ctx, &response_content).await;
            let gpt_resp_json: ResponseBody = serde_json::to_vec(&gpt_resp_data).unwrap();
            let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
                websocket.try_send(key.clone(), gpt_resp_json);
            let save_res: Result<(), String> = Self::save_message(
                &session_id,
                "GPT Assistant",
                "assistant",
                "GptResponse",
                &response_content,
            )
            .await;
            if save_res.is_err() {
                error!(
                    "Failed to save GPT response for session {session_id} {}",
                    save_res.err().unwrap_or_default()
                );
            }
            if !should_continue {
                break;
            }
        }
    }

    #[instrument_trace]
    fn build_gpt_request_body(session: &ChatSession) -> serde_json::Value {
        let json_schema: serde_json::Value = json!({
            "type": "json_schema",
            "json_schema": {
                "name": "chat_response",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        JSON_FIELD_RESPONSE_CONTENT: {
                            "type": "string",
                            "description": "The response content to the user's message"
                        },
                        JSON_FIELD_SHOULD_CONTINUE: {
                            "type": "boolean",
                            "description": "Set to true if you have more content to provide and want to continue in the next iteration. Set to false if your response is complete."
                        }
                    },
                    "required": [JSON_FIELD_RESPONSE_CONTENT, JSON_FIELD_SHOULD_CONTINUE],
                    "additionalProperties": false
                }
            }
        });
        let system_message: serde_json::Value = json!({
            JSON_FIELD_ROLE: SYSTEM,
            JSON_FIELD_CONTENT: SYSTEM_INSTRUCTION
        });
        let mut messages: Vec<serde_json::Value> = vec![system_message];
        let session_messages: Vec<serde_json::Value> = session
            .get_messages()
            .iter()
            .map(|msg| {
                json!({
                    JSON_FIELD_ROLE: msg.get_role(),
                    JSON_FIELD_CONTENT: msg.get_content()
                })
            })
            .collect();
        messages.extend(session_messages);
        json!({
            GPT_MODEL: EnvPlugin::get_or_init().get_gpt_model(),
            JSON_FIELD_MESSAGES: messages,
            RESPONSE_FORMAT: json_schema
        })
    }

    #[instrument_trace]
    fn build_gpt_request_headers(api_key: &str) -> HashMapXxHash3_64<&'static str, String> {
        let mut headers: HashMapXxHash3_64<&'static str, String> = hash_map_xx_hash3_64();
        headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string());
        if !api_key.is_empty() {
            headers.insert(AUTHORIZATION, format!("{BEARER_WITH_SPACE}{api_key}"));
        }
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
    fn handle_gpt_api_response(response_text: &str) -> Result<(String, bool), String> {
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
        let content_str: String = match Self::extract_response_content(&response_json) {
            Some(content) => content,
            None => {
                if let Some(error) = Self::extract_error_message(&response_json) {
                    return Err(error);
                }
                return Err(format!("Incorrect API response format {response_text}"));
            }
        };
        let parsed_content: serde_json::Value =
            serde_json::from_str(&content_str).map_err(|error| {
                format!("Failed to parse inner JSON content {error} (content: {content_str})")
            })?;
        let response_content: String = parsed_content
            .get(JSON_FIELD_RESPONSE_CONTENT)
            .and_then(|v: &serde_json::Value| v.as_str())
            .unwrap_or("")
            .to_string();
        let should_continue: bool = parsed_content
            .get(JSON_FIELD_SHOULD_CONTINUE)
            .and_then(|v: &serde_json::Value| v.as_bool())
            .unwrap_or(false);
        Ok((response_content, should_continue))
    }

    #[instrument_trace]
    async fn call_gpt_api_with_context(session: &ChatSession) -> Result<(String, bool), String> {
        let config: &EnvConfig = EnvPlugin::get_or_init();
        let api_key: &str = config.get_gpt_api_key();
        let body: serde_json::Value = Self::build_gpt_request_body(session);
        let headers: HashMapXxHash3_64<&str, String> = Self::build_gpt_request_headers(api_key);
        let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
            .post(config.get_gpt_api_url())
            .json(body)
            .headers(headers)
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
        ChatHistoryRepository::insert_message(
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
        limit: u64,
    ) -> Result<ChatHistoryResponse, String> {
        let messages: Vec<ChatHistory> =
            ChatHistoryRepository::get_history(before_id, limit).await?;
        let total: i64 = ChatHistoryRepository::count_messages().await?;
        let has_more: bool = messages.len() as u64 == limit;
        let mut response: ChatHistoryResponse = ChatHistoryResponse::default();
        response
            .set_messages(messages)
            .set_total(total as usize)
            .set_has_more(has_more);
        Ok(response)
    }
}
