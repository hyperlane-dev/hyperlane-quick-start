use super::*;

/// One row in the `GET /api/euv/playground/projects` listing.
///
/// Fields mirror the on-disk `metadata.json` so the sidebar can render
/// without an extra GET per row.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundProjectListItem {
    /// Stable project id (monotonic per-user counter).
    pub(super) id: String,
    /// Human-readable project name.
    pub(super) name: String,
    /// Last-modified time of the project (ms since unix epoch, UTC).
    /// Frontend can format this with `new Date(...)`.
    pub(super) updated_at_ms: i64,
    /// Code length in bytes (so the sidebar can show e.g. "1.4 KB" without
    /// an extra round-trip to fetch the source).
    pub(super) code_size: u64,
}

/// Response body for `GET /api/euv/playground/projects/{id}` — full project
/// (name + code + metadata). Returned to the editor when the user picks a
/// row from the sidebar.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundProjectDetail {
    /// Stable project id.
    pub(super) id: String,
    /// Human-readable project name.
    pub(super) name: String,
    /// Current Rust source code.
    pub(super) code: String,
    /// Last-modified time (ms since unix epoch, UTC).
    pub(super) updated_at_ms: i64,
}

/// Response body for project mutation routes (create / save / delete) —
/// just returns the (possibly updated) project metadata so the frontend can
/// refresh the sidebar without an extra list call.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundProjectMutationResponse {
    /// Project id of the affected row. For create = the newly-assigned id;
    /// for save / delete = the same id that came in.
    pub(super) id: String,
    /// Updated project name.
    pub(super) name: String,
    /// Updated last-modified time (ms since unix epoch, UTC).
    pub(super) updated_at_ms: i64,
    /// `true` if the project was deleted (only the delete route returns
    /// this in the data; other routes leave it false).
    pub(super) deleted: bool,
}

/// Response body for `GET /api/euv/playground/default-code` — the canonical
/// starter template the server uses both here and when creating a new
/// project on disk. The frontend reads this before opening a brand-new
/// editor so the template lives in one place.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundDefaultCodeResponse {
    /// Default Rust source code pre-filled into a new project.
    pub(super) code: String,
}

/// Response body for `POST /api/euv/playground/run` — produced `index.html`
/// + glue JS + wasm bytes as base64-encoded JSON.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EuvPlaygroundRunResponse {
    /// Whether the build succeeded.
    pub(super) ok: bool,
    /// `index.html` content (base64). Empty when `ok` is false.
    pub(super) html: String,
    /// wasm-bindgen glue JS content (base64). Empty when `ok` is false.
    pub(super) js: String,
    /// Compiled wasm bytes (base64). Empty when `ok` is false.
    pub(super) wasm: String,
    /// Combined compile + linker stderr; non-empty when `ok` is false.
    pub(super) stderr: String,
    /// Absolute path the frontend should load in its preview iframe
    /// once the build succeeds. Empty when `ok` is false. Always
    /// follows `/static/euv-playground/builds/{project_id}/index.html`
    /// (the static-resource route serves the published artefacts
    /// directly), but the server returns the URL so the frontend
    /// doesn't have to reconstruct it.
    pub(super) build_url: String,
}
