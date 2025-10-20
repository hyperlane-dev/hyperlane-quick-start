use super::*;

#[utoipa::path(
    get,
    path = "/chat",
    responses(
        (status = 302, description = "Redirect to chat page")
    )
)]
pub async fn html() {}

impl ServerHook for ChatViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/chat/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}
