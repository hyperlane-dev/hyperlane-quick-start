use super::*;

impl ServerHook for AuthViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let location_target: String = querys
            .get(LOCATION)
            .unwrap_or(&ROOT_PATH.to_string())
            .to_string();
        let is_authenticated: bool = ctx.get_request().try_get_cookie(TOKEN).is_some();
        let location: String = if is_authenticated {
            location_target
        } else {
            let encoded_location: String = urlencoding::encode(&location_target).to_string();
            format!("/static/auth/index.html?{LOCATION}={encoded_location}")
        };
        ctx.get_mut_response()
            .set_status_code(302)
            .set_header(LOCATION, &location);
    }
}
