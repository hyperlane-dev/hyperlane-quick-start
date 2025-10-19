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
        let name: String = get_name(ctx).await;
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
        let name: String = get_name(ctx).await;
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
    ) -> ResultJsonError<ResponseBody> {
        serde_json::to_vec(&WebSocketRespData::new(r#type, ctx, data).await)
    }
}

impl ChatSession {
    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.get_last_activity().elapsed().as_secs() > timeout_minutes * 60
    }
}
