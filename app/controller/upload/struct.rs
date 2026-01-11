use super::*;

#[route("/api/upload/register")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RegisterRoute;

#[route("/api/upload/save")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SaveRoute;

#[route("/api/upload/merge")]
#[derive(Clone, Copy, Debug, Default)]
pub struct MergeRoute;
