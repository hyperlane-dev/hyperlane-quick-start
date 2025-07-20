use super::*;

pub(crate) fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(|| WebSocket::new())
}

pub async fn pre_ws_upgrade(ctx: Context) {
    let addr: String = ctx.get_socket_addr_or_default_string().await;
    let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
    ctx.set_response_header("X-Client-Addr", encode_addr).await;
}

async fn create_online_count_message(ctx: &Context, receiver_count: String) -> String {
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    WebSocketRespData::get_json_data(MessageType::OnlineCount, ctx, data)
        .await
        .unwrap()
}

fn broadcast_online_count(key: BroadcastType<String>, message: String) {
    let websocket: &'static WebSocket = get_global_websocket();
    spawn(async move {
        let _ = websocket.send(key, message);
    });
}

pub async fn connected_hook(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: String = websocket
        .receiver_count_after_increment(key.clone())
        .to_string();
    let username: String = get_name(&ctx).await;
    add_online_user(&username);
    let resp_data: String = create_online_count_message(&ctx, receiver_count).await;
    broadcast_online_count(key, resp_data);
}

pub(crate) async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: ReceiverCount = websocket.receiver_count_after_decrement(key);
    let username: String = get_name(&ctx).await;
    remove_online_user(&username);
    let resp_data: String = create_online_count_message(&ctx, receiver_count.to_string()).await;
    ctx.set_response_body(resp_data).await;
}

fn remove_mentions(text: &str) -> String {
    let mut result: String = String::new();
    let mut chars: Peekable<Chars<'_>> = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '@' {
            // Skip the @ and the following username
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_whitespace() {
                    break;
                }
                chars.next();
            }
            // Skip any trailing whitespace after the mention
            while let Some(&next_ch) = chars.peek() {
                if !next_ch.is_whitespace() {
                    break;
                }
                chars.next();
            }
        } else {
            result.push(ch);
        }
    }

    // Clean up multiple consecutive spaces and trim
    result.split_whitespace().collect::<Vec<&str>>().join(" ")
}

async fn handle_ping_request(ctx: &Context, req_data: &WebSocketReqData) -> bool {
    if req_data.is_ping() {
        let resp_data: WebSocketRespData = WebSocketRespData::new(MessageType::Pang, ctx, "").await;
        let resp_data: String = serde_json::to_string(&resp_data).unwrap();
        let _ = ctx.set_response_body(resp_data).await.send_body().await;
        ctx.set_response_body("").await;
        return true;
    }
    false
}

fn is_gpt_mentioned(message: &str) -> bool {
    message.contains("@GPT") || message.contains("@GPT Assistant") || message.contains("@gpt")
}

async fn process_gpt_request(session_id: String, message: String, ctx: Context) {
    let mut session = get_or_create_session(&session_id);
    let cleaned_msg = remove_mentions(&message);
    session.add_message("user".to_string(), cleaned_msg);

    let api_response = match call_gpt_api_with_context(&session).await {
        Ok(gpt_response) => {
            session.add_message("assistant".to_string(), gpt_response.clone());
            update_session(session);
            format!("@{} {}", session_id, gpt_response)
        }
        Err(error) => format!("API call failed: {error}"),
    };

    let gpt_resp_data = WebSocketRespData::new(MessageType::GptResponse, &ctx, &api_response).await;
    let gpt_resp_json = serde_json::to_string(&gpt_resp_data).unwrap();
    let websocket = get_global_websocket();
    let path = ctx.get_request_path().await;
    let key = BroadcastType::PointToGroup(path);
    let _ = websocket.send(key, gpt_resp_json.clone());
    ctx.set_response_body(gpt_resp_json).await;
    send_callback(ctx).await;
}

#[request_body_json(req_data_res: WebSocketReqData)]
pub(crate) async fn callback(ctx: Context) {
    let req_data: WebSocketReqData = req_data_res.unwrap();

    if handle_ping_request(&ctx, &req_data).await {
        return;
    }

    let session_id: String = get_name(&ctx).await;
    clone!(req_data, ctx, session_id => {
        let req_msg: &String = req_data.get_data();
        if is_gpt_mentioned(req_msg) {
            let req_msg_clone = req_msg.clone();
            spawn(async move {
                process_gpt_request(session_id, req_msg_clone, ctx).await;
            });
        }
    });

    let resp_data: WebSocketRespData = req_data.into_resp(&ctx).await;
    let resp_data: String = serde_json::to_string(&resp_data).unwrap();
    ctx.set_response_body(resp_data).await;
}

fn build_gpt_request_messages(session: &ChatSession) -> Vec<JsonValue> {
    session
        .messages
        .iter()
        .map(|msg| {
            json_value!({
                "role": msg.role,
                "content": msg.content
            })
        })
        .collect()
}

fn build_gpt_request_headers(api_key: &str) -> HashMapXxHash3_64<&str, String> {
    let mut headers: HashMapXxHash3_64<&str, String> = hash_map_xx_hash3_64();
    headers.insert(HOST, "api.cloudflare.com".to_string());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key));
    headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string());
    headers
}

fn extract_response_content(response_json: &JsonValue) -> Option<String> {
    if let Some(result) = response_json.get("result") {
        if let Some(response_content) = result.get("response") {
            if let Some(response_str) = response_content.as_str() {
                if !response_str.is_empty() {
                    return Some(response_str.to_string());
                }
            }
        }
    }

    if let Some(choices) = response_json.get("choices") {
        if let Some(first_choice) = choices.get(0) {
            if let Some(message) = first_choice.get("message") {
                if let Some(content) = message.get("content") {
                    if let Some(content_str) = content.as_str() {
                        return Some(content_str.to_string());
                    }
                }
            }
        }
    }

    None
}

fn extract_error_message(response_json: &JsonValue) -> Option<String> {
    if let Some(errors) = response_json.get("errors") {
        if let Some(first_error) = errors.get(0) {
            if let Some(error_message) = first_error.get("message") {
                return Some(format!(
                    "API error: {}",
                    error_message.as_str().unwrap_or("Unknown error")
                ));
            }
        }
    }
    None
}

async fn call_gpt_api_with_context(session: &ChatSession) -> Result<String, String> {
    let config: &EnvConfig = get_global_env_config();
    let gpt_api_url: &String = &config.gpt_api_url;
    let api_key: &String = &config.gpt_api_key;

    let messages: Vec<JsonValue> = build_gpt_request_messages(session);
    let body: JsonValue = json_value!({
        "max_tokens": 32000,
        "messages": messages
    });

    let headers: HashMapXxHash3_64<&str, String> = build_gpt_request_headers(api_key);
    let mut request_builder = RequestBuilder::new()
        .post(gpt_api_url)
        .json(body)
        .headers(headers)
        .redirect()
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            let response_text: String = response.text().get_body();
            if response_text.trim().is_empty() {
                return Err(
                    "API response is empty, possible authentication failure or network issue"
                        .to_string(),
                );
            }

            let response_json: JsonValue = serde_json::from_str(&response_text).map_err(|e| {
                format!(
                    "JSON parsing failed: {} (response content: {})",
                    e, response_text
                )
            })?;

            if let Some(content) = extract_response_content(&response_json) {
                return Ok(content);
            }

            if let Some(error) = extract_error_message(&response_json) {
                return Err(error);
            }

            Err(format!("Incorrect API response format: {}", response_text))
        }
        Err(e) => Err(format!("Request sending failed: {}", e)),
    }
}

pub(crate) async fn send_callback(ctx: Context) {
    let request_string: String = ctx.get_request().await.get_body_string();
    let response_string: String = ctx.get_response().await.get_body_string();
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    if *ctx.get_response().await.get_status_code() == 200 {
        println_success!(
            request,
            BR,
            request_string,
            BR,
            response,
            BR,
            response_string
        );
    } else {
        println_warning!(
            request,
            BR,
            request_string,
            BR,
            response,
            BR,
            response_string
        );
    }
    log_info(format!(
        "{request}{BR}{request_string}{BR}{response}{BR}{response_string}"
    ))
    .await;
}
