//! hyperlane_application
//!
//! Application layer containing controllers, services, repositories, mappers, models, middleware, views, and utilities for the Hyperlane framework.

#![recursion_limit = "1024"]

pub mod config;
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

pub use view::page::serve_page;

pub(crate) use config::*;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    string::FromUtf8Error,
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
    serde_json::{Error, Value, json},
    serde_with::skip_serializing_none,
    tokio::{spawn, time::sleep},
    urlencoding::decode,
    utoipa::{OpenApi, ToSchema},
    utoipa_rapidoc::RapiDoc,
    utoipa_swagger_ui::SwaggerUi,
};
