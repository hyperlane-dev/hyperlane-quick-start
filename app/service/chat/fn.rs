use super::*;

pub async fn pre_ws_upgrade(ctx: Context) {
    let addr: String = ctx.get_socket_addr_string().await;
    let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
    ctx.set_response_header(HEADER_X_CLIENT_ADDR, &encode_addr)
        .await;
}

pub async fn create_online_count_message(ctx: &Context, receiver_count: String) -> ResponseBody {
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    WebSocketRespData::get_json_data(MessageType::OnlineCount, ctx, data)
        .await
        .unwrap()
}

pub fn broadcast_online_count(key: BroadcastType<String>, message: ResponseBody) {
    let websocket: &WebSocket = get_global_websocket();
    let _ = websocket.send(key, message);
}

fn remove_mentions(text: &str) -> String {
    text.split_whitespace()
        .filter(|word| !word.starts_with(MENTION_PREFIX))
        .collect::<Vec<&str>>()
        .join(SPACE)
}

async fn handle_ping_request(ctx: &Context, req_data: &WebSocketReqData) -> bool {
    if req_data.is_ping() {
        let resp_data: WebSocketRespData = WebSocketRespData::new(MessageType::Pang, ctx, "").await;
        let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
        ctx.set_response_body(&resp_data).await;
        ctx.try_get_send_body_hook().await.unwrap()(ctx.clone()).await;
        ctx.set_response_body(&vec![]).await;
        return true;
    }
    false
}

fn is_gpt_mentioned(message: &str) -> bool {
    message.contains(GPT_MENTION_UPPER)
        || message.contains(GPT_MENTION_FULL)
        || message.contains(GPT_MENTION_LOWER)
}

async fn process_gpt_request(session_id: String, message: String, ctx: Context) {
    let mut session: ChatSession = get_or_create_session(&session_id);
    let cleaned_msg: String = remove_mentions(&message);
    session.add_message(ROLE_USER.to_string(), cleaned_msg);
    let api_response: String = match call_gpt_api_with_context(&session).await {
        Ok(gpt_response) => {
            session.add_message(ROLE_ASSISTANT.to_string(), gpt_response.clone());
            update_session(session);
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
    let _ = websocket.send(key, gpt_resp_json.clone());
    ctx.set_response_body(&gpt_resp_json).await;
    ChatSendedHook.handle(&ctx).await;
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
    let mut headers: HashMapXxHash3_64<&str, String> = hash_map_xx_hash3_64();
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
        .or_else(|| Some(format!("API error: Unknown error")))
}

fn handle_gpt_api_response(response_text: &str) -> Result<String, String> {
    if response_text.trim().is_empty() {
        return Err(
            "API response is empty, possible authentication failure or network issue".to_string(),
        );
    }
    let response_json: JsonValue = serde_json::from_str(response_text).map_err(|error| {
        format!("JSON parsing failed: {error} (response content: {response_text})",)
    })?;
    if let Some(content) = extract_response_content(&response_json) {
        return Ok(content);
    }
    if let Some(error) = extract_error_message(&response_json) {
        return Err(error);
    }
    Err(format!("Incorrect API response format: {response_text}"))
}

async fn call_gpt_api_with_context(session: &ChatSession) -> Result<String, String> {
    let config: &EnvConfig = get_global_env_config();
    let gtp_model: &str = &config.gtp_model;
    let messages: Vec<JsonValue> = build_gpt_request_messages(session);
    let body: JsonValue = json_value!({
        GPT_MODEL: gtp_model,
        JSON_FIELD_MESSAGES: messages
    });
    let headers: HashMapXxHash3_64<&str, String> = build_gpt_request_headers();
    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .post(&config.gpt_api_url)
        .json(body)
        .headers(headers)
        .redirect()
        .http1_1_only()
        .build_async();
    match request_builder.send().await {
        Ok(response) => {
            let response_text: String = response.text().get_body();
            handle_gpt_api_response(&response_text)
        }
        Err(error) => Err(format!("Request sending failed: {error}")),
    }
}
