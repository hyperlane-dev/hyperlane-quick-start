use super::*;

impl ServerHook for HelloRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        route_param(NAME_KEY => name_opt),
        request_cookie("time" => time_opt),
        response_body(format!("Hello {} ! The time is {}.", name_opt.unwrap_or_default(), time_opt.unwrap_or(time())))
    )]
    #[epilogue_macros(response_header(SET_COOKIE => cookie_value))]
    async fn handle(self, ctx: &Context) {
        let cookie_value: String = CookieBuilder::new("time", time()).path("/").build();
    }
}
