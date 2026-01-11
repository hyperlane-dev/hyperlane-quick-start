use super::*;

impl ServerHook for HelloRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        route_param_option(NAME_KEY => name_opt),
        request_cookie_option("time" => time_opt),
        response_body(format!("Hello {} ! The time is {}.", name_opt.unwrap_or_default(), time_opt.unwrap_or(time())))
    )]
    #[epilogue_macros(response_header(SET_COOKIE => cookie_value))]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let cookie_value: String = CookieBuilder::new("time", time()).path("/").build();
    }
}
