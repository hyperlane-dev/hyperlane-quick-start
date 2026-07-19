use super::*;

/// Route — list the current user's playground projects (most-recent first).
#[route("/api/euv/playground/projects")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundProjectsListRoute;

/// Route — create a new playground project for the current user.
#[route("/api/euv/playground/projects/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundProjectsCreateRoute;

/// Route — read a project's full content (name + code + timestamps).
#[route("/api/euv/playground/projects/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundProjectsGetRoute;

/// Route — update an existing project's name and/or code.
#[route("/api/euv/playground/projects/save/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundProjectsSaveRoute;

/// Route — delete an existing project (irreversible).
#[route("/api/euv/playground/projects/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundProjectsDeleteRoute;

/// Route — compile the current code of a project to wasm via `wasm-pack`.
#[route("/api/euv/playground/run")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundRunRoute;

/// Route — read the default source code template that pre-fills brand-new
/// playground projects. The same string is reused on the server when a
/// project is created, so both `/create` and `/default-code` stay in sync.
#[route("/api/euv/playground/default-code")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundDefaultCodeRoute;

/// Zero-sized struct used purely as a namespace for euv-playground
/// controller-side helpers (cookie extraction, request validation, etc.).
/// Methods live in `impl.rs` under `impl EuvPlaygroundHelpers { ... }`.
#[derive(Clone, Copy, Default)]
pub struct EuvPlaygroundHelpers;
