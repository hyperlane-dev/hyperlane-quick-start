use super::*;

/// Request body for `POST /api/euv-playground/projects` — create a new
/// playground project for the current user.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundProjectCreateRequest {
    /// Human-readable project name. Trimmed; defaults to "Untitled" if empty.
    pub name: String,
}

/// Request body for `PUT /api/euv-playground/projects/{id}` — update the
/// source code (and optionally the name) of an existing project.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundProjectSaveRequest {
    /// New project name (trimmed; empty = keep existing name).
    pub name: Option<String>,
    /// New Rust source code. Empty = keep existing code (used for rename-only).
    pub code: Option<String>,
}

/// Request body for `POST /api/euv-playground/run` — compile a project.
///
/// If `code` is omitted, the latest saved code for the project is loaded
/// from disk. This lets the frontend run the project after a fresh load
/// without first round-tripping a save.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundRunRequest {
    /// Encoded project id returned by the project APIs. The current user
    /// must own the decoded project.
    pub project_id: String,
    /// Optional override source code. If absent, the project's saved code
    /// is used (so "click Run" right after a list-and-pick works without an
    /// explicit save).
    pub code: Option<String>,
}
