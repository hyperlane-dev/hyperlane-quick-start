use super::*;

#[route("/api/cicd/pipeline/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreatePipelineRoute;

#[route("/api/cicd/pipeline/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdatePipelineRoute;

#[route("/api/cicd/pipeline/delete")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeletePipelineRoute;

#[route("/api/cicd/pipeline/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListPipelinesRoute;

#[route("/api/cicd/pipeline/get")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetPipelineRoute;

#[route("/api/cicd/run/trigger")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TriggerRunRoute;

#[route("/api/cicd/run/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRunsRoute;

#[route("/api/cicd/run/get")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetRunRoute;

#[route("/api/cicd/run/detail")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GetRunDetailRoute;

#[route("/api/cicd/job/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateJobRoute;

#[route("/api/cicd/step/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateStepRoute;

#[route("/cicd")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdViewRoute;
