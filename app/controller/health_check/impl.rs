use super::*;

impl ServerHook for HealthCheckRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("HealthCheckRoute new");
        Self
    }

    #[prologue_macros(get)]
    async fn handle(self, ctx: &Context) {
        trace!("HealthCheckRoute handle");
    }
}
