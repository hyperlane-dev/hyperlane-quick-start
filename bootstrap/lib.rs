//! hyperlane_bootstrap
//!
//! Bootstrap layer providing initialization logic for environment, database, logger, server configuration, runtime, and server startup.

#![recursion_limit = "1024"]

pub mod application;
pub mod common;
pub mod framework;

use common::*;

use {
    color_log::*, hyperlane::*, hyperlane_application::service::cicd::CicdService,
    instrument_level::*, log::*, lombok_macros::*,
};

// `hyperlane` 21.x and `hyperlane-utils` 33.x are two generations of the same
// crate family and both re-export the `hyperlane` attribute macro, so a glob
// import of each makes every `#[hyperlane(...)]` invocation E0659. An explicit
// import outranks a glob import, so naming it once resolves them onto the
// `hyperlane` facade.
use hyperlane::hyperlane;
