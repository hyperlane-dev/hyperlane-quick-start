#![recursion_limit = "1024"]

pub mod application;
pub mod common;
pub mod framework;

use common::*;

use {
    hyperlane::*,
    hyperlane_utils::{log::*, *},
};
