mod r#impl;
mod r#struct;

pub use r#struct::*;

use {
    super::*,
    mapper::tracking::*,
    model::{application::tracking::*, response::tracking::*},
};
