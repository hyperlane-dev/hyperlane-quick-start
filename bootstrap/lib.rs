//! hyperlane_bootstrap
//!
//! Bootstrap layer providing initialization logic for environment, database, logger, server configuration, runtime, and server startup.

#![recursion_limit = "1024"]

pub mod application;
pub mod common;
pub mod framework;

use common::*;

use {hyperlane::*, hyperlane_log::*, log::*, lombok_macros::*};

// `hyperlane-utils` used to re-export this macro too, so globbing both crates made
// every `#[hyperlane(...)]` invocation E0659 and this import had to disambiguate them.
// That crate is gone; the explicit import keeps the macro resolving against the
// `hyperlane` facade no matter what else a glob below brings in.
use hyperlane::hyperlane;
