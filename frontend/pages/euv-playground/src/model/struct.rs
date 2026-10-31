use super::*;

/// One row of the list of the projects.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundProject {
    /// The identifier of the project.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The moment the project was written the last time.
    pub updated_at_ms: i64,
    /// The size of the code of the project in bytes.
    pub code_size: u64,
}

/// The full project the editor opens.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundProjectDetail {
    /// The identifier of the project.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The code of the project.
    pub code: String,
    /// The moment the project was written the last time.
    pub updated_at_ms: i64,
}

/// The answer of the server to a request that changes a project.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundProjectMutation {
    /// The identifier of the project.
    pub id: String,
    /// The name of the project.
    pub name: String,
    /// The moment the project was written the last time.
    pub updated_at_ms: i64,
    /// Whether the project was deleted.
    pub deleted: bool,
}

/// The template the server hands to a project that was never written.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundDefaultCode {
    /// The code of the template.
    pub code: String,
}

/// The answer of the server to a request that starts a build.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundRun {
    /// Whether the server accepted the build.
    pub ok: bool,
    /// The identifier of the build.
    pub job_id: String,
    /// The status of the build.
    pub status: String,
    /// The message of a failure.
    pub message: String,
    /// The page the build produced.
    pub html: String,
    /// The glue the build produced.
    pub js: String,
    /// The module the build produced.
    pub wasm: String,
    /// The output of the compiler.
    pub stderr: String,
    /// The address of the page the build produced.
    pub build_url: String,
}

/// The status of a build.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PlaygroundBuildStatus {
    /// The identifier of the build.
    pub job_id: String,
    /// The identifier of the project the build belongs to.
    pub project_id: String,
    /// The status of the build.
    pub status: String,
    /// The address of the page the build produced.
    pub build_url: String,
    /// The output of the compiler.
    pub stderr: String,
    /// The moment the build was started.
    pub created_at_ms: i64,
    /// The moment the status of the build changed.
    pub updated_at_ms: i64,
}

/// The body of a request that creates a project.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaygroundCreateRequest {
    /// The name of the project.
    pub name: String,
}

/// The body of a request that writes a project.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaygroundSaveRequest {
    /// The name of the project, absent when the name did not change.
    pub name: Option<String>,
    /// The code of the project, absent when the code did not change.
    pub code: Option<String>,
}

/// The body of a request that starts a build.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaygroundRunRequest {
    /// The identifier of the project the build belongs to.
    pub project_id: String,
    /// The code of the build, absent when the saved code is used.
    pub code: Option<String>,
}
