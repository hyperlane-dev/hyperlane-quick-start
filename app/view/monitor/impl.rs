use super::*;

#[utoipa::path(
    get,
    path = "/monitor",
    responses(
        (status = 302, description = "Redirect to monitor page")
    )
)]
pub async fn html() {}

impl ServerHook for MonitorViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/monitor/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}
