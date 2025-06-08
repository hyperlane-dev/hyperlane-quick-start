use super::*;

pub async fn get_id(ctx: &Context) -> String {
    ctx.get_socket_addr_or_default_string().await
}

pub fn get_name_from_id(id: &str) -> String {
    id.to_string()
}
