mod r#enum;
mod r#impl;
mod r#struct;

pub use {
    r#enum::*,
    r#struct::{Model as ShortlinkModel, *},
};

use super::*;
