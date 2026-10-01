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
    hyperlane_utils::{log::*, *},
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
    utoipa::ToSchema,
};

// `hyperlane` 21.x and `hyperlane-utils` 33.x are two generations of the same
// crate family and both re-export these attribute macros. Two glob imports of
// one name in the same module is E0659, so every macro invocation in this crate
// (`#[task_panic]`, `#[route]`, `#[request_middleware]`, `#[response_header]`,
// ... including the `_*_data` / `_*_macros` helper attributes they expand to)
// would fail to resolve. An explicit import outranks a glob import in Rust, so
// naming them once here disambiguates them onto the `hyperlane` facade without
// changing which implementation is used.
use hyperlane::{
    epilogue_macros, prologue_macros, request_error, request_error_data, request_middleware,
    response_header, response_middleware, response_status_code, response_version, route,
    task_panic, task_panic_data,
};
