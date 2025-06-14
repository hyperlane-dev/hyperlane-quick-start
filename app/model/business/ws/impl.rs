use super::*;

impl Default for MessageType {
    fn default() -> Self {
        Self::Text
    }
}

impl WebSocketReqData {
    pub fn new<T: ToString>(r#type: MessageType, data: T) -> Self {
        let mut resp_data: Self = Self::default();
        resp_data.set_type(r#type).set_data(data.to_string());
        resp_data
    }

    pub async fn into_resp(&self, ctx: &Context) -> WebSocketRespData {
        let id: String = get_id(ctx).await;
        let name: String = get_name_from_id(&id);
        let mut resp: WebSocketRespData = WebSocketRespData::default();
        resp.set_type(*self.get_type())
            .set_id(id)
            .set_name(name)
            .set_data(self.get_data().clone())
            .set_time(time());
        resp
    }
}

impl WebSocketRespData {
    pub async fn new<T: ToString>(r#type: MessageType, ctx: &Context, data: T) -> Self {
        let id: String = get_id(ctx).await;
        let name: String = get_name_from_id(&id);
        let mut resp_data: Self = Self::default();
        resp_data
            .set_type(r#type)
            .set_id(id.to_string())
            .set_name(name.to_string())
            .set_data(data.to_string())
            .set_time(time());
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
