use super::*;

impl ServerHook for HealthCheckRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(get)]
    async fn handle(self, ctx: &Context) {}
}
