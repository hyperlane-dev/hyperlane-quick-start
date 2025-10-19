use super::*;

impl Hook for ChatRequestHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[request_body_json(req_data_res: WebSocketReqData)]
    async fn handle(self, ctx: &Context) {
        let req_data: WebSocketReqData = req_data_res.unwrap();
        if handle_ping_request(&ctx, &req_data).await {
            return;
        }
        let session_id: String = get_name(ctx).await;
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
        let resp_data: ResponseBody = serde_json::to_vec(&resp_data).unwrap();
        ctx.set_response_body(&resp_data).await;
    }
}

impl Hook for ChatSendedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request_string: String = ctx.get_request().await.get_body_string();
        let response_string: String = ctx.get_response().await.get_body_string();
        let request: String = ctx.get_request().await.get_string();
        let response: String = ctx.get_response().await.get_string();
        if *ctx.get_response().await.get_status_code() == 200 {
            println_success!("{request}{BR}{request_string}{BR}{response}{BR}{response_string}");
        } else {
            println_warning!("{request}{BR}{request_string}{BR}{response}{BR}{response_string}");
        }
        log_info(&format!(
            "{request}{BR}{request_string}{BR}{response}{BR}{response_string}"
        ))
        .await;
    }
}

impl Hook for ChatClosedHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: ReceiverCount = websocket.receiver_count_after_decrement(key);
        let username: String = get_name(ctx).await;
        remove_online_user(&username);
        let resp_data: ResponseBody =
            create_online_count_message(&ctx, receiver_count.to_string()).await;
        ctx.set_response_body(&resp_data).await;
    }
}
