use super::*;

pub(crate) fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(|| WebSocket::new())
}

pub async fn before_ws_upgrade(ctx: Context) {
    let addr: String = ctx.get_socket_addr_or_default_string().await;
    let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
    ctx.set_response_header("addr", encode_addr).await;
}

pub async fn on_connected(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: String = websocket
        .receiver_count_after_increment(key.clone())
        .to_string();
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    let resp_data: String = WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, data)
        .await
        .unwrap();
    spawn(async move {
        let _ = websocket.send(key, resp_data);
    });
}

pub(crate) async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: ReceiverCount = websocket.receiver_count_after_decrement(key);
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    let resp_data: String = WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, data)
        .await
        .unwrap();
    ctx.set_response_body(resp_data).await;
}

pub(crate) async fn callback(ctx: Context) {
    let req_data: WebSocketReqData = ctx
        .get_request_body_json::<WebSocketReqData>()
        .await
        .unwrap();
    if req_data.is_ping() {
        let resp_data: WebSocketRespData =
            WebSocketRespData::new(MessageType::Pang, &ctx, "").await;
        let resp_data: String = json_stringify_string(&resp_data).unwrap();
        ctx.set_response_body(resp_data)
            .await
            .send_body()
            .await
            .unwrap();
        ctx.set_response_body("").await;
        return;
    }
    clone!(req_data, ctx => {
        spawn(async move {
            let req_msg: &String = req_data.get_data();
            let api_response = match call_gpt_api(req_msg).await {
                Ok(response) => response,
                Err(error) => {
                    let err_msg: String = format!("[API 调用失败: {}]", error);
                    err_msg
                }
            };
            let _ = ctx.set_response_body(api_response)
                .await
                .send_body()
                .await;
            ctx.flush().await.unwrap();
        });
    });
    let resp_data: WebSocketRespData = req_data.into_resp(&ctx).await;
    let resp_data: String = json_stringify_string(&resp_data).unwrap();
    ctx.set_response_body(resp_data).await;
}

async fn call_gpt_api(message: &str) -> Result<String, String> {
    let config = get_global_env_config();
    let gpt_api_url = &config.gpt_api_url;
    let api_key = &config.gpt_api_key;
    let body: JsonValue = json_value!({
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ]
    });
    let mut headers: HashMapXxHash3_64<&str, String> = hash_map_xx_hash3_64();
    headers.insert(HOST, "api.cloudflare.com".to_string());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key));
    headers.insert(CONTENT_TYPE, APPLICATION_JSON.to_string());
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
                return Err("API 响应为空，可能是认证失败或网络问题".to_string());
            }
            let response_json: JsonValue = serde_json::from_str(&response_text)
                .map_err(|e| format!("JSON 解析失败: {} (响应内容: {})", e, response_text))?;
            if let Some(result) = response_json.get("result") {
                if let Some(response_content) = result.get("response") {
                    if let Some(response_str) = response_content.as_str() {
                        if !response_str.is_empty() {
                            return Ok(response_str.to_string());
                        }
                    }
                }
            }
            if let Some(choices) = response_json.get("choices") {
                if let Some(first_choice) = choices.get(0) {
                    if let Some(message) = first_choice.get("message") {
                        if let Some(content) = message.get("content") {
                            if let Some(content_str) = content.as_str() {
                                return Ok(content_str.to_string());
                            }
                        }
                    }
                }
            }
            if let Some(errors) = response_json.get("errors") {
                if let Some(first_error) = errors.get(0) {
                    if let Some(error_message) = first_error.get("message") {
                        return Err(format!(
                            "API 错误: {}",
                            error_message.as_str().unwrap_or("未知错误")
                        ));
                    }
                }
            }
            Err(format!("API 响应格式不正确: {}", response_text))
        }
        Err(e) => Err(format!("请求发送失败: {}", e)),
    }
}

pub(crate) async fn send_callback(_: Context) {}
