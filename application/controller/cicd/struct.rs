use super::*;

/// create pipeline route.
#[route("/api/cicd/pipeline/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreatePipelineRoute;

/// list pipelines route.
#[route("/api/cicd/pipeline/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListPipelinesRoute;

/// get pipeline route.
#[route("/api/cicd/pipeline/get")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetPipelineRoute;

/// trigger run route.
#[route("/api/cicd/run/trigger")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TriggerRunRoute;

/// list runs route.
#[route("/api/cicd/run/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRunsRoute;

/// get run route.
#[route("/api/cicd/run/get")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetRunRoute;

/// get run detail route.
#[route("/api/cicd/run/detail")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetRunDetailRoute;

/// update job route.
#[route("/api/cicd/job/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateJobRoute;

/// update step route.
#[route("/api/cicd/step/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateStepRoute;

/// get incremental run detail route.
#[route("/api/cicd/run/detail/incremental")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetIncrementalRunDetailRoute;

/// cicd view route.
#[route("/cicd")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdViewRoute;

/// run logs sse route.
#[route("/api/cicd/run/logs/stream")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RunLogsSseRoute;
