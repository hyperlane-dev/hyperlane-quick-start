use super::*;

#[utoipa::path(
    get,
    path = "/tracking",
    responses(
        (status = 302, description = "Redirect to tracking page")
    )
)]
pub async fn html() {}

impl ServerHook for TrackingViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    async fn handle(self, ctx: &Context) {
        let html: &str = include_str!("../../../resources/static/tracking/index.html");
        ctx.set_response_body(html.as_bytes()).await;
    }
}
