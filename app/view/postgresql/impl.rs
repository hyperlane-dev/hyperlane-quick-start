use super::*;

impl ServerHook for PostgresqlViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("PostgresqlViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/postgresql/index.html")
    )]
    async fn handle(self, ctx: &Context) {
        trace!("PostgresqlViewRoute handle");
    }
}
