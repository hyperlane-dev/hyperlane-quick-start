//! Hyperlane config
//!
//! Configuration layer containing application-level and framework-level constants for logging, caching, and static assets.

#![recursion_limit = "1024"]

pub mod application;
pub mod framework;

use hyperlane_utils::log::*;
