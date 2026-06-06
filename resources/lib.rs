//! Hyperlane resources
//!
//! Resource module containing static assets, SQL scripts, Docker configurations, environment files, and templates used by the Hyperlane framework.

#![recursion_limit = "1024"]

pub mod docker;
pub mod env;
pub mod sql;
pub mod r#static;
pub mod templates;

pub use {sql::*, templates::*};
