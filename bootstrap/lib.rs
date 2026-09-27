//! Hyperlane bootstrap
//!
//! Bootstrap layer providing initialization logic for environment, database, logger, server configuration, runtime, and server startup.

#![recursion_limit = "1024"]

pub mod application;
pub mod common;
pub mod framework;

use common::*;

use {
    color_log::*, color_output::*, hyperlane::*, hyperlane_application::service::cicd::CicdService,
    hyperlane_macros::*, instrument_level::*, log::*, lombok_macros::*,
};
