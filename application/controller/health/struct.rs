use super::*;

#[route("/health")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct HealthCheckRoute;
