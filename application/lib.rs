//! hyperlane_application
//!
//! Application layer containing controllers, services, repositories, mappers, models, middleware, views, and utilities for the Hyperlane framework.

#![recursion_limit = "1024"]

pub mod controller;
pub mod domain;
pub mod exception;
pub mod mapper;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod service;
pub mod utils;
pub mod view;

use {
    chrono::Utc,
    hyperlane::*,
    hyperlane_time::*,
    instrument_level::*,
    log::*,
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
    utoipa::ToSchema,
};

// These attribute macros (`#[task_panic]`, `#[route]`, `#[request_middleware]`,
// `#[response_header]`, and the `_*_data` / `_*_macros` helpers they expand to)
// used to be re-exported by `hyperlane-utils` as well, so globbing both crates made
// every invocation E0659. That crate is gone; naming them once here keeps them
// resolving against the `hyperlane` facade regardless of the glob above.
use hyperlane::{
    epilogue_macros, prologue_macros, request_error, request_error_data, request_middleware,
    response_header, response_middleware, response_status_code, response_version, route,
    task_panic, task_panic_data,
};
