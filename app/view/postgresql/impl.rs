use super::*;

#[utoipa::path(
    get,
    path = "/postgresql",
    responses(
        (status = 302, description = "Redirect to postgresql page")
    )
)]
pub async fn html() {}

impl ServerHook for PostgresqlViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/postgresql/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}
