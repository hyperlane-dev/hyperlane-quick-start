//! hyperlane_application
//!
//! Application layer containing controllers, services, repositories, mappers, models, middleware, views, and utilities for the Hyperlane framework.

#![recursion_limit = "1024"]

pub mod controller;
pub mod domain;
pub mod exception;
mod header;
pub mod mapper;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod service;
pub mod utils;
pub mod view;

pub(crate) use header::*;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, OnceLock},
    time::Duration,
};

use {
    ::log::*,
    ::redis::RedisError,
    bin_encode_decode::{Decode, DecodeError, Encode, EncodeError},
    chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, Utc},
    chunkify::*,
    color_output::*,
    file_operation::*,
    hyperlane::*,
    hyperlane_broadcast::*,
    hyperlane_plugin_websocket::*,
    instrument_level::*,
    jwt_service::*,
    lombok_macros::*,
    rust_decimal::Decimal,
    sea_orm::{
        ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr,
        DeriveActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
        EntityTrait, EnumIter, PaginatorTrait, PrimaryKeyTrait, QueryFilter, QueryOrder,
        QuerySelect, RelationDef, RelationTrait, Select, prelude::Expr,
    },
    serde::{Deserialize, Serialize},
    serde_json::json,
    serde_with::skip_serializing_none,
    tokio::{spawn, time::sleep},
    utoipa::{OpenApi, ToSchema},
    utoipa_rapidoc::RapiDoc,
    utoipa_swagger_ui::SwaggerUi,
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
