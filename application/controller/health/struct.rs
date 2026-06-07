use super::*;

/// health check route.
#[route("/health")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct HealthCheckRoute;
