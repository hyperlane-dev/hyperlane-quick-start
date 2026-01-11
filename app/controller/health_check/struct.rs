use super::*;

#[route("/health")]
#[derive(Clone, Copy, Debug, Default)]
pub struct HealthCheckRoute;
