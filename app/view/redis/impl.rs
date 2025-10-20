use super::*;

#[utoipa::path(
    get,
    path = "/redis",
    responses(
        (status = 302, description = "Redirect to redis page")
    )
)]
pub async fn html() {}

impl ServerHook for RedisViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/redis/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}
