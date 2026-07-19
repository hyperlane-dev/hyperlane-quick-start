use super::*;

/// Route structure for the OpenAPI specification view endpoints.
#[route("/openapi")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OpenApiViewRoute;
