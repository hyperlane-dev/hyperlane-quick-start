mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

use {super::*, r#static::*};

use hyperlane_config::{application::logger::*, framework::*};

use std::fmt::Arguments;

use hyperlane_utils::once_cell::sync::Lazy;
